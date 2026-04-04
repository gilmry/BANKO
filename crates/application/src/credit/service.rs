use std::sync::Arc;

use chrono::NaiveDate;

use banko_domain::account::AccountId;
use banko_domain::credit::{
    AmortizationType, AssetClass, Loan, LoanId, LoanStatus, PaymentFrequency, Provision,
};
use banko_domain::shared::{CustomerId, Money};

use super::dto::{InstallmentResponse, LoanDetailResponse, LoanResponse, PaginatedLoansResponse};
use super::errors::LoanServiceError;
use super::ports::{ILoanRepository, IScheduleRepository};

pub struct LoanService {
    loan_repo: Arc<dyn ILoanRepository>,
    schedule_repo: Arc<dyn IScheduleRepository>,
}

impl LoanService {
    pub fn new(
        loan_repo: Arc<dyn ILoanRepository>,
        schedule_repo: Arc<dyn IScheduleRepository>,
    ) -> Self {
        LoanService {
            loan_repo,
            schedule_repo,
        }
    }

    /// Apply for a new loan.
    pub async fn apply_for_loan(
        &self,
        account_id: AccountId,
        customer_id: CustomerId,
        amount: Money,
        interest_rate: f64,
        term_months: u32,
    ) -> Result<Loan, LoanServiceError> {
        let loan = Loan::new(customer_id, account_id, amount, interest_rate, term_months)
            .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok(loan)
    }

    /// Approve a loan application.
    pub async fn approve_loan(
        &self,
        loan_id: &LoanId,
    ) -> Result<Loan, LoanServiceError> {
        let mut loan = self.get_loan(loan_id).await?;

        loan.approve()
            .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok(loan)
    }

    /// Disburse an approved loan. Generates amortization schedule.
    pub async fn disburse(
        &self,
        loan_id: &LoanId,
        disbursement_date: NaiveDate,
        frequency: PaymentFrequency,
        amortization_type: AmortizationType,
    ) -> Result<Loan, LoanServiceError> {
        let mut loan = self.get_loan(loan_id).await?;

        loan.disburse(disbursement_date, frequency, amortization_type)
            .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

        // Save schedule installments
        if let Some(schedule) = loan.schedule() {
            self.schedule_repo
                .save_installments(schedule.installments())
                .await
                .map_err(LoanServiceError::Internal)?;
        }

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok(loan)
    }

    /// Classify a loan based on days past due (INV-06).
    pub async fn classify(
        &self,
        loan_id: &LoanId,
        days_past_due: u32,
    ) -> Result<AssetClass, LoanServiceError> {
        let mut loan = self.get_loan(loan_id).await?;

        let new_class = loan.classify(days_past_due);

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok(new_class)
    }

    /// Update provision for a loan (INV-07, INV-15).
    pub async fn provision(
        &self,
        loan_id: &LoanId,
        provision_amount: Money,
    ) -> Result<Provision, LoanServiceError> {
        let mut loan = self.get_loan(loan_id).await?;

        let provision = loan
            .update_provision(provision_amount)
            .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok(provision)
    }

    /// Classify and apply regulatory minimum provision in one step.
    pub async fn classify_and_provision(
        &self,
        loan_id: &LoanId,
        days_past_due: u32,
    ) -> Result<(AssetClass, Provision), LoanServiceError> {
        let mut loan = self.get_loan(loan_id).await?;

        let new_class = loan.classify(days_past_due);
        let provision = loan
            .apply_regulatory_provision()
            .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok((new_class, provision))
    }

    /// Record a payment on the next unpaid installment.
    pub async fn record_payment(
        &self,
        loan_id: &LoanId,
    ) -> Result<Loan, LoanServiceError> {
        let mut loan = self.get_loan(loan_id).await?;

        let installment = loan
            .record_payment()
            .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

        // Update installment in schedule repo
        self.schedule_repo
            .update_installment(installment)
            .await
            .map_err(LoanServiceError::Internal)?;

        self.loan_repo
            .save(&loan)
            .await
            .map_err(LoanServiceError::Internal)?;

        Ok(loan)
    }

    /// Batch: classify all active loans based on days past due.
    pub async fn update_all_classifications(
        &self,
        get_days_past_due: impl Fn(&Loan) -> u32,
    ) -> Result<Vec<(LoanId, AssetClass, Provision)>, LoanServiceError> {
        let active_loans = self
            .loan_repo
            .find_active_loans()
            .await
            .map_err(LoanServiceError::Internal)?;

        let mut results = Vec::new();

        for mut loan in active_loans {
            let dpd = get_days_past_due(&loan);
            let new_class = loan.classify(dpd);
            let provision = loan
                .apply_regulatory_provision()
                .map_err(|e| LoanServiceError::DomainError(e.to_string()))?;

            self.loan_repo
                .save(&loan)
                .await
                .map_err(LoanServiceError::Internal)?;

            results.push((loan.id().clone(), new_class, provision));
        }

        Ok(results)
    }

    /// Find loan by ID.
    pub async fn find_by_id(
        &self,
        loan_id: &LoanId,
    ) -> Result<LoanDetailResponse, LoanServiceError> {
        let loan = self.get_loan(loan_id).await?;
        let schedule_installments = self
            .schedule_repo
            .find_by_loan_id(loan_id)
            .await
            .map_err(LoanServiceError::Internal)?;

        let schedule = if schedule_installments.is_empty() {
            None
        } else {
            Some(
                schedule_installments
                    .iter()
                    .map(Self::installment_to_response)
                    .collect(),
            )
        };

        Ok(LoanDetailResponse {
            loan: Self::loan_to_response(&loan),
            schedule,
        })
    }

    /// List loans with filters and pagination.
    pub async fn list_loans(
        &self,
        status: Option<LoanStatus>,
        asset_class: Option<AssetClass>,
        account_id: Option<&AccountId>,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedLoansResponse, LoanServiceError> {
        let offset = (page - 1) * limit;

        let loans = self
            .loan_repo
            .find_all(status, asset_class, account_id, limit, offset)
            .await
            .map_err(LoanServiceError::Internal)?;

        let total = self
            .loan_repo
            .count_all(status, asset_class, account_id)
            .await
            .map_err(LoanServiceError::Internal)?;

        let data = loans.iter().map(Self::loan_to_response).collect();

        Ok(PaginatedLoansResponse {
            data,
            total,
            page,
            limit,
        })
    }

    // --- Helpers ---

    async fn get_loan(&self, loan_id: &LoanId) -> Result<Loan, LoanServiceError> {
        self.loan_repo
            .find_by_id(loan_id)
            .await
            .map_err(LoanServiceError::Internal)?
            .ok_or(LoanServiceError::LoanNotFound)
    }

    pub fn loan_to_response(loan: &Loan) -> LoanResponse {
        LoanResponse {
            id: loan.id().to_string(),
            customer_id: loan.customer_id().to_string(),
            account_id: loan.account_id().to_string(),
            amount: loan.amount().amount(),
            interest_rate: loan.interest_rate(),
            term_months: loan.term_months(),
            currency: loan.amount().currency().to_string(),
            asset_class: loan.asset_class().as_str().to_string(),
            provision_amount: loan.provision().amount().amount(),
            provision_rate: loan.provision().rate(),
            status: loan.status().as_str().to_string(),
            days_past_due: loan.days_past_due(),
            disbursement_date: loan.disbursement_date().map(|d| d.to_string()),
            created_at: loan.created_at(),
            updated_at: loan.updated_at(),
        }
    }

    pub fn installment_to_response(inst: &banko_domain::credit::Installment) -> InstallmentResponse {
        InstallmentResponse {
            id: inst.id().to_string(),
            installment_number: inst.installment_number(),
            due_date: inst.due_date().to_string(),
            principal_amount: inst.principal_amount().amount(),
            interest_amount: inst.interest_amount().amount(),
            total_amount: inst.total_amount().amount(),
            remaining_balance: inst.remaining_balance().amount(),
            paid: inst.paid(),
            paid_date: inst.paid_date().map(|d| d.to_rfc3339()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use banko_domain::account::AccountId;
    use banko_domain::credit::*;
    use banko_domain::shared::{Currency, CustomerId, Money};

    use super::*;

    // --- Mock Loan Repository ---

    struct MockLoanRepository {
        loans: Mutex<Vec<Loan>>,
    }

    impl MockLoanRepository {
        fn new() -> Self {
            MockLoanRepository {
                loans: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ILoanRepository for MockLoanRepository {
        async fn save(&self, loan: &Loan) -> Result<(), String> {
            let mut loans = self.loans.lock().unwrap();
            loans.retain(|l| l.id() != loan.id());
            loans.push(loan.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &LoanId) -> Result<Option<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.iter().find(|l| l.id() == id).cloned())
        }

        async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.iter().filter(|l| l.account_id() == account_id).cloned().collect())
        }

        async fn find_all(
            &self,
            status: Option<LoanStatus>,
            asset_class: Option<AssetClass>,
            account_id: Option<&AccountId>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans
                .iter()
                .filter(|l| status.is_none() || Some(l.status()) == status)
                .filter(|l| asset_class.is_none() || Some(l.asset_class()) == asset_class)
                .filter(|l| account_id.is_none() || Some(l.account_id()) == account_id)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn count_all(
            &self,
            status: Option<LoanStatus>,
            asset_class: Option<AssetClass>,
            account_id: Option<&AccountId>,
        ) -> Result<i64, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans
                .iter()
                .filter(|l| status.is_none() || Some(l.status()) == status)
                .filter(|l| asset_class.is_none() || Some(l.asset_class()) == asset_class)
                .filter(|l| account_id.is_none() || Some(l.account_id()) == account_id)
                .count() as i64)
        }

        async fn find_active_loans(&self) -> Result<Vec<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.iter().filter(|l| l.status() == LoanStatus::Active).cloned().collect())
        }

        async fn delete(&self, id: &LoanId) -> Result<(), String> {
            let mut loans = self.loans.lock().unwrap();
            loans.retain(|l| l.id() != id);
            Ok(())
        }
    }

    // --- Mock Schedule Repository ---

    struct MockScheduleRepository {
        installments: Mutex<Vec<Installment>>,
    }

    impl MockScheduleRepository {
        fn new() -> Self {
            MockScheduleRepository {
                installments: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IScheduleRepository for MockScheduleRepository {
        async fn save_installments(&self, installments: &[Installment]) -> Result<(), String> {
            let mut all = self.installments.lock().unwrap();
            all.extend(installments.iter().cloned());
            Ok(())
        }

        async fn find_by_loan_id(&self, loan_id: &LoanId) -> Result<Vec<Installment>, String> {
            let all = self.installments.lock().unwrap();
            Ok(all.iter().filter(|i| i.loan_id() == loan_id).cloned().collect())
        }

        async fn update_installment(&self, installment: &Installment) -> Result<(), String> {
            let mut all = self.installments.lock().unwrap();
            if let Some(existing) = all.iter_mut().find(|i| i.id() == installment.id()) {
                *existing = installment.clone();
            }
            Ok(())
        }
    }

    fn make_service() -> LoanService {
        LoanService::new(
            Arc::new(MockLoanRepository::new()),
            Arc::new(MockScheduleRepository::new()),
        )
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    // --- Tests ---

    #[tokio::test]
    async fn test_apply_for_loan() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(50000.0), 8.0, 12)
            .await
            .unwrap();
        assert_eq!(loan.status(), LoanStatus::Applied);
        assert_eq!(loan.amount().amount(), 50000.0);
    }

    #[tokio::test]
    async fn test_apply_invalid_amount() {
        let service = make_service();
        let result = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(0.0), 8.0, 12)
            .await;
        assert!(matches!(result, Err(LoanServiceError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_approve_loan() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(50000.0), 8.0, 12)
            .await
            .unwrap();

        let approved = service.approve_loan(loan.id()).await.unwrap();
        assert_eq!(approved.status(), LoanStatus::Approved);
    }

    #[tokio::test]
    async fn test_approve_not_found() {
        let service = make_service();
        let result = service.approve_loan(&LoanId::new()).await;
        assert!(matches!(result, Err(LoanServiceError::LoanNotFound)));
    }

    #[tokio::test]
    async fn test_disburse_loan() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(100000.0), 8.0, 60)
            .await
            .unwrap();

        service.approve_loan(loan.id()).await.unwrap();

        let start = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let disbursed = service
            .disburse(loan.id(), start, PaymentFrequency::Monthly, AmortizationType::Constant)
            .await
            .unwrap();

        assert_eq!(disbursed.status(), LoanStatus::Active);
        assert!(disbursed.schedule().is_some());
        assert_eq!(disbursed.schedule().unwrap().total_installments(), 60);
    }

    #[tokio::test]
    async fn test_classify_loan() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(50000.0), 8.0, 12)
            .await
            .unwrap();

        let class = service.classify(loan.id(), 91).await.unwrap();
        assert_eq!(class, AssetClass::Class2);
    }

    #[tokio::test]
    async fn test_provision_loan() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(100000.0), 8.0, 12)
            .await
            .unwrap();

        service.classify(loan.id(), 91).await.unwrap(); // Class 2

        // 20% of 100000 = 20000
        let provision = service.provision(loan.id(), tnd(20000.0)).await.unwrap();
        assert_eq!(provision.amount().amount(), 20000.0);
    }

    #[tokio::test]
    async fn test_provision_below_minimum_fails() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(100000.0), 8.0, 12)
            .await
            .unwrap();

        service.classify(loan.id(), 91).await.unwrap(); // Class 2

        let result = service.provision(loan.id(), tnd(10000.0)).await;
        assert!(matches!(result, Err(LoanServiceError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_classify_and_provision() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(100000.0), 8.0, 12)
            .await
            .unwrap();

        let (class, provision) = service.classify_and_provision(loan.id(), 181).await.unwrap();
        assert_eq!(class, AssetClass::Class3);
        assert_eq!(provision.amount().amount(), 50000.0);
    }

    #[tokio::test]
    async fn test_record_payment() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(12000.0), 6.0, 3)
            .await
            .unwrap();
        service.approve_loan(loan.id()).await.unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        service
            .disburse(loan.id(), start, PaymentFrequency::Monthly, AmortizationType::Linear)
            .await
            .unwrap();

        let after_payment = service.record_payment(loan.id()).await.unwrap();
        assert_eq!(after_payment.status(), LoanStatus::Active);
    }

    #[tokio::test]
    async fn test_list_loans() {
        let service = make_service();
        for _ in 0..5 {
            service
                .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(50000.0), 8.0, 12)
                .await
                .unwrap();
        }

        let result = service
            .list_loans(Some(LoanStatus::Applied), None, None, 1, 10)
            .await
            .unwrap();
        assert_eq!(result.total, 5);
        assert_eq!(result.data.len(), 5);
    }

    #[tokio::test]
    async fn test_update_all_classifications() {
        let service = make_service();
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(100000.0), 8.0, 12)
            .await
            .unwrap();
        service.approve_loan(loan.id()).await.unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        service
            .disburse(loan.id(), start, PaymentFrequency::Monthly, AmortizationType::Constant)
            .await
            .unwrap();

        let results = service
            .update_all_classifications(|_| 91) // All loans 91 days late
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, AssetClass::Class2);
        // Provision should be ≥ 20%
        assert!(results[0].2.amount().amount() >= 20000.0);
    }
}
