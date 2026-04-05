//! E2E integration tests covering full multi-role workflows.
//!
//! STORY-DOC-E2E-01 to E2E-06 + invariant verification.
//!
//! These tests exercise multiple services together using mock repositories
//! but real domain logic and real service orchestration.

use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

// ==============================================================================
// STORY-DOC-E2E-01: Account Opening Flow
// ==============================================================================

mod e2e_account_opening {
    use super::*;
    use crate::account::*;
    use crate::customer::*;
    use banko_domain::account::{Account, AccountId, AccountStatus, AccountType, Movement};
    use banko_domain::customer::{Customer, CustomerStatus};
    use banko_domain::shared::{Currency, CustomerId, Money, Rib};

    // --- Mock Customer Repository ---

    struct MockCustomerRepo {
        customers: Mutex<Vec<Customer>>,
    }

    impl MockCustomerRepo {
        fn new() -> Self {
            MockCustomerRepo {
                customers: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ICustomerRepository for MockCustomerRepo {
        async fn save(&self, customer: &Customer) -> Result<(), String> {
            let mut customers = self.customers.lock().unwrap();
            customers.retain(|c| c.id() != customer.id());
            customers.push(customer.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &CustomerId) -> Result<Option<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers.iter().find(|c| c.id() == id).cloned())
        }
        async fn find_by_email(
            &self,
            email: &banko_domain::shared::value_objects::EmailAddress,
        ) -> Result<Option<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers
                .iter()
                .find(|c| c.kyc_profile().email() == email)
                .cloned())
        }
        async fn list_all(&self) -> Result<Vec<Customer>, String> {
            Ok(self.customers.lock().unwrap().clone())
        }
        async fn list_by_status(&self, status: &str) -> Result<Vec<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers
                .iter()
                .filter(|c| c.status().as_str() == status)
                .cloned()
                .collect())
        }
        async fn delete(&self, id: &CustomerId) -> Result<(), String> {
            self.customers.lock().unwrap().retain(|c| c.id() != id);
            Ok(())
        }
        async fn find_closed_before(
            &self,
            _before: DateTime<Utc>,
        ) -> Result<Vec<Customer>, String> {
            Ok(vec![])
        }
    }

    struct MockPepChecker;

    #[async_trait]
    impl IPepCheckService for MockPepChecker {
        async fn is_pep(&self, _full_name: &str) -> Result<bool, String> {
            Ok(false)
        }
    }

    // --- Mock Account Repository ---

    struct MockAccountRepo {
        accounts: Mutex<Vec<Account>>,
        movements: Mutex<Vec<Movement>>,
    }

    impl MockAccountRepo {
        fn new() -> Self {
            MockAccountRepo {
                accounts: Mutex::new(Vec::new()),
                movements: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAccountRepository for MockAccountRepo {
        async fn save(&self, account: &Account) -> Result<(), String> {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.retain(|a| a.id() != account.id());
            accounts.push(account.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts.iter().find(|a| a.id() == id).cloned())
        }
        async fn find_by_customer_id(
            &self,
            customer_id: &CustomerId,
        ) -> Result<Vec<Account>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts
                .iter()
                .filter(|a| a.customer_id() == customer_id)
                .cloned()
                .collect())
        }
        async fn find_by_rib(&self, rib: &Rib) -> Result<Option<Account>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts.iter().find(|a| a.rib() == rib).cloned())
        }
        async fn save_movement(&self, movement: &Movement) -> Result<(), String> {
            self.movements.lock().unwrap().push(movement.clone());
            Ok(())
        }
        async fn find_movements_by_account(
            &self,
            account_id: &AccountId,
            limit: i64,
        ) -> Result<Vec<Movement>, String> {
            let movements = self.movements.lock().unwrap();
            Ok(movements
                .iter()
                .filter(|m| m.account_id() == account_id)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn find_movements_by_account_and_period(
            &self,
            account_id: &AccountId,
            from: Option<DateTime<Utc>>,
            to: Option<DateTime<Utc>>,
        ) -> Result<Vec<Movement>, String> {
            let movements = self.movements.lock().unwrap();
            Ok(movements
                .iter()
                .filter(|m| {
                    m.account_id() == account_id
                        && from.is_none_or(|f| m.created_at() >= f)
                        && to.is_none_or(|t| m.created_at() <= t)
                })
                .cloned()
                .collect())
        }
        async fn delete(&self, id: &AccountId) -> Result<(), String> {
            self.accounts.lock().unwrap().retain(|a| a.id() != id);
            Ok(())
        }
    }

    // --- Shared mock KYC verifier that delegates to customer repo ---

    struct CustomerKycVerifier {
        customer_repo: Arc<MockCustomerRepo>,
    }

    #[async_trait]
    impl IKycVerifier for CustomerKycVerifier {
        async fn is_kyc_validated(&self, customer_id: &CustomerId) -> Result<bool, String> {
            let customers = self.customer_repo.customers.lock().unwrap();
            Ok(customers
                .iter()
                .find(|c| c.id() == customer_id)
                .map_or(false, |c| c.is_kyc_validated()))
        }
    }

    fn valid_create_request() -> CreateCustomerRequest {
        CreateCustomerRequest {
            customer_type: "Individual".to_string(),
            full_name: "Karim Ben Ayed".to_string(),
            cin: Some("12345678".to_string()),
            birth_date: Some("1990-01-15".to_string()),
            nationality: Some("Tunisia".to_string()),
            profession: Some("Agent bancaire".to_string()),
            address: AddressDto {
                street: "10 Rue de la Liberte".to_string(),
                city: "Tunis".to_string(),
                postal_code: "1000".to_string(),
                country: Some("Tunisia".to_string()),
            },
            phone: "+21698123456".to_string(),
            email: "karim@example.com".to_string(),
            pep_status: Some("No".to_string()),
            source_of_funds: Some("Salary".to_string()),
            consent: true,
            registration_number: None,
            sector: None,
            beneficiaries: None,
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    #[tokio::test]
    async fn test_e2e_account_opening() {
        // Shared customer repo so account service can verify KYC
        let customer_repo = Arc::new(MockCustomerRepo::new());
        let customer_service = CustomerService::new(
            customer_repo.clone(),
            Arc::new(MockPepChecker),
        );

        let kyc_verifier = Arc::new(CustomerKycVerifier {
            customer_repo: customer_repo.clone(),
        });
        let account_repo = Arc::new(MockAccountRepo::new());
        let account_service = AccountService::new(account_repo.clone(), kyc_verifier);

        // Step 1: Karim (agent) creates customer
        let customer_id = customer_service
            .create_customer(valid_create_request())
            .await
            .unwrap();

        // Verify customer exists in Pending status
        let customer = customer_service.find_by_id(&customer_id).await.unwrap();
        assert_eq!(customer.status(), CustomerStatus::Pending);

        // Step 2: Sonia (compliance officer) validates KYC
        let approved_customer = customer_service.approve_kyc(&customer_id).await.unwrap();
        assert!(approved_customer.is_kyc_validated());
        assert_eq!(approved_customer.status(), CustomerStatus::Approved);

        // Step 3: Open account for the customer
        let account = account_service
            .open_account(customer_id.clone(), AccountType::Current)
            .await
            .unwrap();
        assert_eq!(account.status(), AccountStatus::Active);
        assert_eq!(account.customer_id(), &customer_id);

        // Step 4: Make initial deposit
        let deposit = account_service
            .deposit(account.id(), tnd(50000.0), "Initial deposit")
            .await
            .unwrap();
        assert_eq!(deposit.amount().amount(), 50000.0);

        // Step 5: Verify account balance
        let updated_account = account_service.find_by_id(account.id()).await.unwrap();
        assert_eq!(updated_account.balance().amount(), 50000.0);

        // Step 6: Verify customer is listed with approved status
        let all_customers = customer_service.list_customers().await.unwrap();
        assert_eq!(all_customers.len(), 1);
        assert_eq!(all_customers[0].status(), CustomerStatus::Approved);
    }

    #[tokio::test]
    async fn test_e2e_account_opening_fails_without_kyc() {
        let customer_repo = Arc::new(MockCustomerRepo::new());
        let customer_service = CustomerService::new(
            customer_repo.clone(),
            Arc::new(MockPepChecker),
        );

        let kyc_verifier = Arc::new(CustomerKycVerifier {
            customer_repo: customer_repo.clone(),
        });
        let account_service =
            AccountService::new(Arc::new(MockAccountRepo::new()), kyc_verifier);

        // Create customer but do NOT approve KYC
        let customer_id = customer_service
            .create_customer(valid_create_request())
            .await
            .unwrap();

        // Attempting to open account should fail
        let result = account_service
            .open_account(customer_id, AccountType::Current)
            .await;
        assert!(matches!(result, Err(AccountServiceError::KycNotValidated)));
    }
}

// ==============================================================================
// STORY-DOC-E2E-02: Credit Granting Flow
// ==============================================================================

mod e2e_credit_granting {
    use super::*;
    use crate::accounting::*;
    use crate::credit::*;
    use banko_domain::account::AccountId;
    use banko_domain::credit::*;
    use banko_domain::shared::{Currency, CustomerId, Money};

    // --- Mock Loan Repository ---

    struct MockLoanRepo {
        loans: Mutex<Vec<Loan>>,
    }

    impl MockLoanRepo {
        fn new() -> Self {
            MockLoanRepo {
                loans: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ILoanRepository for MockLoanRepo {
        async fn save(&self, loan: &Loan) -> Result<(), String> {
            let mut loans = self.loans.lock().unwrap();
            loans.retain(|l| l.id() != loan.id());
            loans.push(loan.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &LoanId) -> Result<Option<Loan>, String> {
            Ok(self.loans.lock().unwrap().iter().find(|l| l.id() == id).cloned())
        }
        async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<Loan>, String> {
            Ok(self
                .loans
                .lock()
                .unwrap()
                .iter()
                .filter(|l| l.account_id() == account_id)
                .cloned()
                .collect())
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
            Ok(self
                .loans
                .lock()
                .unwrap()
                .iter()
                .filter(|l| l.status() == LoanStatus::Active)
                .cloned()
                .collect())
        }
        async fn delete(&self, id: &LoanId) -> Result<(), String> {
            self.loans.lock().unwrap().retain(|l| l.id() != id);
            Ok(())
        }
    }

    // --- Mock Schedule Repository ---

    struct MockScheduleRepo {
        installments: Mutex<Vec<Installment>>,
    }

    impl MockScheduleRepo {
        fn new() -> Self {
            MockScheduleRepo {
                installments: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IScheduleRepository for MockScheduleRepo {
        async fn save_installments(&self, installments: &[Installment]) -> Result<(), String> {
            self.installments
                .lock()
                .unwrap()
                .extend(installments.iter().cloned());
            Ok(())
        }
        async fn find_by_loan_id(&self, loan_id: &LoanId) -> Result<Vec<Installment>, String> {
            Ok(self
                .installments
                .lock()
                .unwrap()
                .iter()
                .filter(|i| i.loan_id() == loan_id)
                .cloned()
                .collect())
        }
        async fn update_installment(&self, installment: &Installment) -> Result<(), String> {
            let mut all = self.installments.lock().unwrap();
            if let Some(existing) = all.iter_mut().find(|i| i.id() == installment.id()) {
                *existing = installment.clone();
            }
            Ok(())
        }
    }

    // --- Mock Journal Repository (for accounting verification) ---

    struct MockJournalRepo {
        entries: Mutex<Vec<banko_domain::accounting::JournalEntry>>,
    }

    impl MockJournalRepo {
        fn new() -> Self {
            MockJournalRepo {
                entries: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IJournalRepository for MockJournalRepo {
        async fn save(
            &self,
            entry: &banko_domain::accounting::JournalEntry,
        ) -> Result<(), String> {
            let mut entries = self.entries.lock().unwrap();
            entries.retain(|e| e.entry_id() != entry.entry_id());
            entries.push(entry.clone());
            Ok(())
        }
        async fn find_by_id(
            &self,
            id: &banko_domain::accounting::EntryId,
        ) -> Result<Option<banko_domain::accounting::JournalEntry>, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .find(|e| e.entry_id() == id)
                .cloned())
        }
        async fn find_by_period(
            &self,
            _start: NaiveDate,
            _end: NaiveDate,
        ) -> Result<Vec<banko_domain::accounting::JournalEntry>, String> {
            Ok(vec![])
        }
        async fn find_by_account(
            &self,
            _code: &banko_domain::accounting::AccountCode,
            _start: NaiveDate,
            _end: NaiveDate,
        ) -> Result<Vec<banko_domain::accounting::JournalEntry>, String> {
            Ok(vec![])
        }
        async fn find_all(
            &self,
            offset: i64,
            limit: i64,
        ) -> Result<Vec<banko_domain::accounting::JournalEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self) -> Result<i64, String> {
            Ok(self.entries.lock().unwrap().len() as i64)
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    #[tokio::test]
    async fn test_e2e_credit_granting() {
        let loan_service = LoanService::new(
            Arc::new(MockLoanRepo::new()),
            Arc::new(MockScheduleRepo::new()),
        );
        let auto_entry_service = AutoEntryService::new(Arc::new(MockJournalRepo::new()));

        let account_id = AccountId::new();
        let customer_id = CustomerId::new();

        // Step 1: Karim applies for a loan
        let loan = loan_service
            .apply_for_loan(account_id.clone(), customer_id, tnd(100_000.0), 8.0, 60)
            .await
            .unwrap();
        assert_eq!(loan.status(), LoanStatus::Applied);

        // Step 2: Rachid (risk analyst) classifies loan risk
        let asset_class = loan_service.classify(loan.id(), 0).await.unwrap();
        assert_eq!(asset_class, AssetClass::Class0);

        // Step 3: Committee approves loan
        let approved = loan_service.approve_loan(loan.id()).await.unwrap();
        assert_eq!(approved.status(), LoanStatus::Approved);

        // Step 4: Disburse loan
        let start_date = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let disbursed = loan_service
            .disburse(
                loan.id(),
                start_date,
                PaymentFrequency::Monthly,
                AmortizationType::Constant,
            )
            .await
            .unwrap();
        assert_eq!(disbursed.status(), LoanStatus::Active);
        assert!(disbursed.schedule().is_some());
        assert_eq!(disbursed.schedule().unwrap().total_installments(), 60);

        // Step 5: Verify accounting entry for loan disbursement
        let entry = auto_entry_service
            .on_loan_disbursed(Uuid::new_v4(), 100_000)
            .await
            .unwrap();
        assert_eq!(entry.status, "Posted");
        assert_eq!(entry.total_debit, 100_000);
        assert_eq!(entry.total_credit, 100_000);

        // Step 6: Classify and provision the active loan
        let (class, provision) = loan_service
            .classify_and_provision(loan.id(), 0)
            .await
            .unwrap();
        assert_eq!(class, AssetClass::Class0);
        // Class 0 = 0% provision (performing loan)
        assert_eq!(provision.amount().amount(), 0.0);
    }

    #[tokio::test]
    async fn test_e2e_credit_lifecycle_with_payment() {
        let loan_service = LoanService::new(
            Arc::new(MockLoanRepo::new()),
            Arc::new(MockScheduleRepo::new()),
        );

        let loan = loan_service
            .apply_for_loan(
                AccountId::new(),
                CustomerId::new(),
                tnd(12_000.0),
                6.0,
                3,
            )
            .await
            .unwrap();

        loan_service.approve_loan(loan.id()).await.unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        loan_service
            .disburse(
                loan.id(),
                start,
                PaymentFrequency::Monthly,
                AmortizationType::Linear,
            )
            .await
            .unwrap();

        // Record a payment
        let after_payment = loan_service.record_payment(loan.id()).await.unwrap();
        assert_eq!(after_payment.status(), LoanStatus::Active);
    }
}

// ==============================================================================
// STORY-DOC-E2E-03: Suspicion Declaration (AML)
// ==============================================================================

mod e2e_suspicion_declaration {
    use super::*;
    use crate::aml::*;
    use banko_domain::aml::*;
    use banko_domain::shared::{Currency, Money};

    // --- Mock Repositories ---

    struct MockTransactionRepo {
        txs: Mutex<Vec<Transaction>>,
    }

    impl MockTransactionRepo {
        fn new() -> Self {
            MockTransactionRepo {
                txs: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ITransactionRepository for MockTransactionRepo {
        async fn save(&self, tx: &Transaction) -> Result<(), String> {
            let mut txs = self.txs.lock().unwrap();
            txs.retain(|t| t.id() != tx.id());
            txs.push(tx.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, String> {
            Ok(self.txs.lock().unwrap().iter().find(|t| t.id() == id).cloned())
        }
        async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<Transaction>, String> {
            Ok(self
                .txs
                .lock()
                .unwrap()
                .iter()
                .filter(|t| t.account_id() == account_id)
                .cloned()
                .collect())
        }
        async fn find_by_date_range(
            &self,
            account_id: Uuid,
            from: DateTime<Utc>,
            to: DateTime<Utc>,
        ) -> Result<Vec<Transaction>, String> {
            Ok(self
                .txs
                .lock()
                .unwrap()
                .iter()
                .filter(|t| {
                    t.account_id() == account_id
                        && t.timestamp() >= from
                        && t.timestamp() <= to
                })
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            account_id: Option<Uuid>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Transaction>, String> {
            Ok(self
                .txs
                .lock()
                .unwrap()
                .iter()
                .filter(|t| account_id.is_none() || Some(t.account_id()) == account_id)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self, account_id: Option<Uuid>) -> Result<i64, String> {
            Ok(self
                .txs
                .lock()
                .unwrap()
                .iter()
                .filter(|t| account_id.is_none() || Some(t.account_id()) == account_id)
                .count() as i64)
        }
    }

    struct MockAlertRepo {
        alerts: Mutex<Vec<Alert>>,
    }

    impl MockAlertRepo {
        fn new() -> Self {
            MockAlertRepo {
                alerts: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAlertRepository for MockAlertRepo {
        async fn save(&self, alert: &Alert) -> Result<(), String> {
            let mut alerts = self.alerts.lock().unwrap();
            alerts.retain(|a| a.id() != alert.id());
            alerts.push(alert.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Alert>, String> {
            Ok(self.alerts.lock().unwrap().iter().find(|a| a.id() == id).cloned())
        }
        async fn find_by_transaction_id(
            &self,
            tx_id: &TransactionId,
        ) -> Result<Vec<Alert>, String> {
            Ok(self
                .alerts
                .lock()
                .unwrap()
                .iter()
                .filter(|a| a.transaction_id() == tx_id)
                .cloned()
                .collect())
        }
        async fn find_by_status(&self, status: AlertStatus) -> Result<Vec<Alert>, String> {
            Ok(self
                .alerts
                .lock()
                .unwrap()
                .iter()
                .filter(|a| a.status() == status)
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            status: Option<AlertStatus>,
            _risk_level: Option<RiskLevel>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Alert>, String> {
            Ok(self
                .alerts
                .lock()
                .unwrap()
                .iter()
                .filter(|a| status.is_none() || Some(a.status()) == status)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_by_status(&self, status: Option<AlertStatus>) -> Result<i64, String> {
            Ok(self
                .alerts
                .lock()
                .unwrap()
                .iter()
                .filter(|a| status.is_none() || Some(a.status()) == status)
                .count() as i64)
        }
    }

    struct MockInvestigationRepo {
        investigations: Mutex<Vec<Investigation>>,
    }

    impl MockInvestigationRepo {
        fn new() -> Self {
            MockInvestigationRepo {
                investigations: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IInvestigationRepository for MockInvestigationRepo {
        async fn save(&self, inv: &Investigation) -> Result<(), String> {
            let mut investigations = self.investigations.lock().unwrap();
            investigations.retain(|i| i.id() != inv.id());
            investigations.push(inv.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Investigation>, String> {
            Ok(self
                .investigations
                .lock()
                .unwrap()
                .iter()
                .find(|i| i.id() == id)
                .cloned())
        }
        async fn find_by_alert_id(&self, alert_id: Uuid) -> Result<Option<Investigation>, String> {
            Ok(self
                .investigations
                .lock()
                .unwrap()
                .iter()
                .find(|i| i.alert_id() == alert_id)
                .cloned())
        }
        async fn find_by_status(
            &self,
            status: InvestigationStatus,
        ) -> Result<Vec<Investigation>, String> {
            Ok(self
                .investigations
                .lock()
                .unwrap()
                .iter()
                .filter(|i| i.status() == status)
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            status: Option<InvestigationStatus>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Investigation>, String> {
            Ok(self
                .investigations
                .lock()
                .unwrap()
                .iter()
                .filter(|i| status.is_none() || Some(i.status()) == status)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(
            &self,
            status: Option<InvestigationStatus>,
        ) -> Result<i64, String> {
            Ok(self
                .investigations
                .lock()
                .unwrap()
                .iter()
                .filter(|i| status.is_none() || Some(i.status()) == status)
                .count() as i64)
        }
    }

    struct MockReportRepo {
        reports: Mutex<Vec<SuspicionReport>>,
    }

    impl MockReportRepo {
        fn new() -> Self {
            MockReportRepo {
                reports: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ISuspicionReportRepository for MockReportRepo {
        async fn save(&self, report: &SuspicionReport) -> Result<(), String> {
            let mut reports = self.reports.lock().unwrap();
            reports.retain(|r| r.id() != report.id());
            reports.push(report.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<SuspicionReport>, String> {
            Ok(self.reports.lock().unwrap().iter().find(|r| r.id() == id).cloned())
        }
        async fn find_by_investigation_id(
            &self,
            inv_id: Uuid,
        ) -> Result<Option<SuspicionReport>, String> {
            Ok(self
                .reports
                .lock()
                .unwrap()
                .iter()
                .find(|r| r.investigation_id() == inv_id)
                .cloned())
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    #[tokio::test]
    async fn test_e2e_suspicion_declaration() {
        let alert_repo = Arc::new(MockAlertRepo::new());
        let monitoring_service = TransactionMonitoringService::new(
            Arc::new(MockTransactionRepo::new()),
            alert_repo.clone(),
            vec![Arc::new(ThresholdScenario)],
        );

        let inv_service = InvestigationService::new(
            Arc::new(MockInvestigationRepo::new()),
            alert_repo.clone(),
        );

        let dos_service = DosReportService::new(Arc::new(MockReportRepo::new()));

        // Step 1: Record a transaction > 5000 TND (threshold)
        let account_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let (_tx, alerts) = monitoring_service
            .record_transaction(
                account_id,
                customer_id,
                "Suspicious Corp".to_string(),
                tnd(6000.0),
                TransactionType::Deposit,
                Direction::Inbound,
            )
            .await
            .unwrap();

        // Step 2: Verify alert auto-generated
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].risk_level(), RiskLevel::Medium);
        let alert_id = alerts[0].id();

        // Step 3: Sonia (compliance officer) opens investigation
        let inv = inv_service
            .open_investigation(alert_id, Some("sonia".to_string()))
            .await
            .unwrap();
        assert_eq!(inv.status, "Open");
        let inv_id = Uuid::parse_str(&inv.id).unwrap();

        // Step 4: Add investigation notes
        let inv = inv_service
            .add_note(
                inv_id,
                "Reviewed transfer patterns. Suspicious structuring detected.".to_string(),
                "sonia".to_string(),
            )
            .await
            .unwrap();
        assert_eq!(inv.status, "InProgress");
        assert_eq!(inv.notes.len(), 1);

        // Step 5: Generate DOS (suspicion report)
        let report = dos_service
            .generate_report(
                inv_id,
                "Suspicious Corp".to_string(),
                "6000 TND deposit".to_string(),
                "Suspected structuring".to_string(),
                Some("Bank statements".to_string()),
                None,
            )
            .await
            .unwrap();
        assert_eq!(report.status, "Draft");
        let report_id = Uuid::parse_str(&report.id).unwrap();

        // Step 6: Submit to CTAF
        let submitted = dos_service.submit_to_ctaf(report_id).await.unwrap();
        assert_eq!(submitted.status, "Submitted");
        assert!(submitted.submitted_at.is_some());

        // Step 7: Close investigation as confirmed
        let closed = inv_service.close_confirmed(inv_id).await.unwrap();
        assert_eq!(closed.status, "ClosedConfirmed");
    }
}

// ==============================================================================
// STORY-DOC-E2E-04: Asset Freeze (Sanctions)
// ==============================================================================

mod e2e_asset_freeze {
    use super::*;
    use crate::account::*;
    use crate::aml::*;
    use crate::sanctions::*;
    use banko_domain::account::{Account, AccountId, AccountStatus, AccountType, Movement};
    use banko_domain::aml::AssetFreeze;
    use banko_domain::sanctions::*;
    use banko_domain::shared::{Currency, CustomerId, Money, Rib};

    // --- Mock Account Repository ---

    struct MockAccountRepo {
        accounts: Mutex<Vec<Account>>,
        movements: Mutex<Vec<Movement>>,
    }

    impl MockAccountRepo {
        fn new() -> Self {
            MockAccountRepo {
                accounts: Mutex::new(Vec::new()),
                movements: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAccountRepository for MockAccountRepo {
        async fn save(&self, account: &Account) -> Result<(), String> {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.retain(|a| a.id() != account.id());
            accounts.push(account.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, String> {
            Ok(self
                .accounts
                .lock()
                .unwrap()
                .iter()
                .find(|a| a.id() == id)
                .cloned())
        }
        async fn find_by_customer_id(
            &self,
            customer_id: &CustomerId,
        ) -> Result<Vec<Account>, String> {
            Ok(self
                .accounts
                .lock()
                .unwrap()
                .iter()
                .filter(|a| a.customer_id() == customer_id)
                .cloned()
                .collect())
        }
        async fn find_by_rib(&self, _rib: &Rib) -> Result<Option<Account>, String> {
            Ok(None)
        }
        async fn save_movement(&self, movement: &Movement) -> Result<(), String> {
            self.movements.lock().unwrap().push(movement.clone());
            Ok(())
        }
        async fn find_movements_by_account(
            &self,
            _id: &AccountId,
            _limit: i64,
        ) -> Result<Vec<Movement>, String> {
            Ok(vec![])
        }
        async fn find_movements_by_account_and_period(
            &self,
            _id: &AccountId,
            _from: Option<DateTime<Utc>>,
            _to: Option<DateTime<Utc>>,
        ) -> Result<Vec<Movement>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &AccountId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockKycVerifier;

    #[async_trait]
    impl IKycVerifier for MockKycVerifier {
        async fn is_kyc_validated(&self, _id: &CustomerId) -> Result<bool, String> {
            Ok(true)
        }
    }

    // --- Mock Sanctions Repos ---

    struct MockEntryRepo {
        entries: Mutex<Vec<SanctionEntry>>,
    }

    impl MockEntryRepo {
        fn with_entries(entries: Vec<SanctionEntry>) -> Self {
            MockEntryRepo {
                entries: Mutex::new(entries),
            }
        }
    }

    #[async_trait]
    impl ISanctionEntryRepository for MockEntryRepo {
        async fn save_entries(&self, entries: &[SanctionEntry]) -> Result<(), String> {
            self.entries.lock().unwrap().extend(entries.iter().cloned());
            Ok(())
        }
        async fn find_by_id(
            &self,
            id: &SanctionEntryId,
        ) -> Result<Option<SanctionEntry>, String> {
            Ok(self.entries.lock().unwrap().iter().find(|e| e.id() == id).cloned())
        }
        async fn find_all_active(&self) -> Result<Vec<SanctionEntry>, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .filter(|e| e.active())
                .cloned()
                .collect())
        }
        async fn find_by_source(&self, source: ListSource) -> Result<Vec<SanctionEntry>, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .filter(|e| e.list_source() == source)
                .cloned()
                .collect())
        }
        async fn count_active(&self) -> Result<i64, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .filter(|e| e.active())
                .count() as i64)
        }
    }

    struct MockResultRepo {
        results: Mutex<Vec<ScreeningResult>>,
    }

    impl MockResultRepo {
        fn new() -> Self {
            MockResultRepo {
                results: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IScreeningResultRepository for MockResultRepo {
        async fn save(&self, result: &ScreeningResult) -> Result<(), String> {
            self.results.lock().unwrap().push(result.clone());
            Ok(())
        }
        async fn find_by_id(
            &self,
            id: &ScreeningResultId,
        ) -> Result<Option<ScreeningResult>, String> {
            Ok(self.results.lock().unwrap().iter().find(|r| r.id() == id).cloned())
        }
        async fn find_recent(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<ScreeningResult>, String> {
            Ok(vec![])
        }
        async fn count_by_status(
            &self,
            _status: Option<ScreeningStatus>,
        ) -> Result<i64, String> {
            Ok(0)
        }
    }

    struct TestMatchingStrategy;

    impl IMatchingStrategy for TestMatchingStrategy {
        fn screen(
            &self,
            name: &str,
            entries: &[SanctionEntry],
            threshold: u8,
        ) -> Vec<MatchDetail> {
            screen_name(name, entries, threshold)
                .matched_entries()
                .to_vec()
        }
    }

    // --- Mock Asset Freeze Repo ---

    struct MockFreezeRepo {
        freezes: Mutex<Vec<AssetFreeze>>,
    }

    impl MockFreezeRepo {
        fn new() -> Self {
            MockFreezeRepo {
                freezes: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAssetFreezeRepository for MockFreezeRepo {
        async fn save(&self, freeze: &AssetFreeze) -> Result<(), String> {
            let mut freezes = self.freezes.lock().unwrap();
            freezes.retain(|f| f.id() != freeze.id());
            freezes.push(freeze.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<AssetFreeze>, String> {
            Ok(self.freezes.lock().unwrap().iter().find(|f| f.id() == id).cloned())
        }
        async fn find_by_account_id(
            &self,
            account_id: Uuid,
        ) -> Result<Vec<AssetFreeze>, String> {
            Ok(self
                .freezes
                .lock()
                .unwrap()
                .iter()
                .filter(|f| f.account_id() == account_id)
                .cloned()
                .collect())
        }
        async fn find_active_by_account_id(
            &self,
            account_id: Uuid,
        ) -> Result<Option<AssetFreeze>, String> {
            Ok(self
                .freezes
                .lock()
                .unwrap()
                .iter()
                .find(|f| f.account_id() == account_id && f.is_active())
                .cloned())
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    #[tokio::test]
    async fn test_e2e_asset_freeze() {
        // Setup services
        let account_repo = Arc::new(MockAccountRepo::new());
        let account_service =
            AccountService::new(account_repo.clone(), Arc::new(MockKycVerifier));

        let sanctioned_name = "Mohammed Khalil";
        let sanctions_service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![
                SanctionEntry::new(
                    ListSource::UN,
                    sanctioned_name.to_string(),
                    vec![],
                    None,
                    None,
                )
                .unwrap(),
            ])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        let freeze_service = AssetFreezeService::new(Arc::new(MockFreezeRepo::new()));

        // Step 1: Create customer account (KYC already validated via mock)
        let customer_id = CustomerId::new();
        let account = account_service
            .open_account(customer_id.clone(), AccountType::Current)
            .await
            .unwrap();
        account_service
            .deposit(account.id(), tnd(10_000.0), "Initial deposit")
            .await
            .unwrap();

        // Step 2: Screen name -> sanctions hit detected
        let screening = sanctions_service
            .screen_name(sanctioned_name, None)
            .await
            .unwrap();
        assert_eq!(screening.status, "Hit");
        assert!(screening.highest_score >= 80);

        // Step 3: Freeze account immediately
        let account_uuid = *account.id().as_uuid();
        let freeze = freeze_service
            .freeze_account(
                account_uuid,
                "Sanctions hit - UN list".to_string(),
                "compliance_officer".to_string(),
            )
            .await
            .unwrap();
        assert_eq!(freeze.status, "Active");

        // Also freeze in account service
        account_service.freeze_account(account.id()).await.unwrap();

        // Step 4: Verify account is frozen (cannot transact)
        let frozen_account = account_service.find_by_id(account.id()).await.unwrap();
        assert_eq!(frozen_account.status(), AccountStatus::Suspended);

        // Attempt deposit on frozen account should fail
        let deposit_result = account_service
            .deposit(account.id(), tnd(1000.0), "Should fail")
            .await;
        assert!(deposit_result.is_err());

        // Step 5: Verify freeze is active
        let is_frozen = freeze_service.is_account_frozen(account_uuid).await.unwrap();
        assert!(is_frozen);
    }
}

// ==============================================================================
// STORY-DOC-E2E-05: BCT Audit
// ==============================================================================

mod e2e_bct_audit {
    use super::*;
    use crate::governance::*;
    use banko_domain::governance::*;

    // --- Mock Audit Repository ---

    struct MockAuditRepo {
        entries: Mutex<Vec<AuditTrailEntry>>,
    }

    impl MockAuditRepo {
        fn new() -> Self {
            MockAuditRepo {
                entries: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAuditRepository for MockAuditRepo {
        async fn append(&self, entry: &AuditTrailEntry) -> Result<(), String> {
            self.entries.lock().unwrap().push(entry.clone());
            Ok(())
        }
        async fn find_by_id(
            &self,
            id: &AuditEntryId,
        ) -> Result<Option<AuditTrailEntry>, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .find(|e| e.entry_id() == id)
                .cloned())
        }
        async fn find_latest(&self) -> Result<Option<AuditTrailEntry>, String> {
            Ok(self.entries.lock().unwrap().last().cloned())
        }
        async fn find_all(
            &self,
            filters: &AuditFilter,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<AuditTrailEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| {
                    filters.user_id.is_none() || filters.user_id.as_ref() == Some(e.user_id())
                })
                .filter(|e| {
                    filters.action.is_none()
                        || Some(e.action().as_str().to_string()) == filters.action
                })
                .filter(|e| {
                    filters.resource_type.is_none()
                        || Some(e.resource_type().as_str().to_string()) == filters.resource_type
                })
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self, filters: &AuditFilter) -> Result<i64, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| {
                    filters.user_id.is_none() || filters.user_id.as_ref() == Some(e.user_id())
                })
                .filter(|e| {
                    filters.action.is_none()
                        || Some(e.action().as_str().to_string()) == filters.action
                })
                .count() as i64)
        }
        async fn find_chain(
            &self,
            from: DateTime<Utc>,
            to: DateTime<Utc>,
        ) -> Result<Vec<AuditTrailEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| *e.timestamp() >= from && *e.timestamp() <= to)
                .cloned()
                .collect())
        }
        async fn count_by_date_range(
            &self,
            from: DateTime<Utc>,
            to: DateTime<Utc>,
        ) -> Result<i64, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| *e.timestamp() >= from && *e.timestamp() <= to)
                .count() as i64)
        }
        async fn count_by_action(
            &self,
            _from: DateTime<Utc>,
            _to: DateTime<Utc>,
        ) -> Result<Vec<(String, i64)>, String> {
            Ok(vec![])
        }
        async fn count_by_actor(
            &self,
            _from: DateTime<Utc>,
            _to: DateTime<Utc>,
            _limit: i64,
        ) -> Result<Vec<(Uuid, i64)>, String> {
            Ok(vec![])
        }
        async fn count_per_day(&self, _days: u32) -> Result<Vec<(NaiveDate, i64)>, String> {
            Ok(vec![])
        }
        async fn find_suspicious(
            &self,
            _from: DateTime<Utc>,
            _limit: i64,
        ) -> Result<Vec<AuditTrailEntry>, String> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_e2e_bct_audit() {
        let audit_repo = Arc::new(MockAuditRepo::new());
        let audit_service = AuditService::new(audit_repo.clone());

        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        // Step 1: Log several audit trail entries
        audit_service
            .log_action(
                user_id,
                AuditAction::Create,
                ResourceType::Customer,
                resource_id,
                Some("Created customer".to_string()),
                Some("192.168.1.1".to_string()),
            )
            .await
            .unwrap();

        audit_service
            .log_action(
                user_id,
                AuditAction::Approve,
                ResourceType::Customer,
                resource_id,
                Some("KYC approved".to_string()),
                Some("192.168.1.1".to_string()),
            )
            .await
            .unwrap();

        audit_service
            .log_action(
                user_id,
                AuditAction::Create,
                ResourceType::Account,
                Uuid::new_v4(),
                Some("Account opened".to_string()),
                Some("192.168.1.1".to_string()),
            )
            .await
            .unwrap();

        // Step 2: Query audit trail with filters
        let filter = AuditFilter {
            user_id: Some(user_id),
            action: None,
            resource_type: None,
            resource_id: None,
            from: None,
            to: None,
        };
        let trail = audit_service
            .get_audit_trail(filter, 1, 10)
            .await
            .unwrap();
        assert_eq!(trail.total, 3);
        assert_eq!(trail.data.len(), 3);

        // Filter by action
        let create_filter = AuditFilter {
            user_id: None,
            action: Some("Create".to_string()),
            resource_type: None,
            resource_id: None,
            from: None,
            to: None,
        };
        let creates = audit_service
            .get_audit_trail(create_filter, 1, 10)
            .await
            .unwrap();
        assert_eq!(creates.total, 2);

        // Step 3: Verify hash chain integrity
        let now = Utc::now();
        let from = now - chrono::Duration::hours(1);
        let integrity = audit_service.verify_integrity(from, now).await.unwrap();
        assert!(integrity.valid);
        assert_eq!(integrity.entries_checked, 3);

        // Step 4: Verify pagination works
        let page1 = audit_service
            .get_audit_trail(AuditFilter::default(), 1, 2)
            .await
            .unwrap();
        assert_eq!(page1.data.len(), 2);
        assert_eq!(page1.total, 3);

        let page2 = audit_service
            .get_audit_trail(AuditFilter::default(), 2, 2)
            .await
            .unwrap();
        assert_eq!(page2.data.len(), 1);
    }
}

// ==============================================================================
// STORY-DOC-E2E-06: Monthly Reporting
// ==============================================================================

mod e2e_monthly_reporting {
    use super::*;
    use crate::reporting::*;
    use banko_domain::reporting::*;

    // --- Mock Report Repository ---

    struct MockReportRepo {
        reports: Mutex<Vec<RegulatoryReport>>,
    }

    impl MockReportRepo {
        fn new() -> Self {
            MockReportRepo {
                reports: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IReportRepository for MockReportRepo {
        async fn save(&self, report: &RegulatoryReport) -> Result<(), String> {
            let mut reports = self.reports.lock().unwrap();
            reports.retain(|r| r.report_id() != report.report_id());
            reports.push(report.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &ReportId) -> Result<Option<RegulatoryReport>, String> {
            Ok(self
                .reports
                .lock()
                .unwrap()
                .iter()
                .find(|r| r.report_id() == id)
                .cloned())
        }
        async fn find_by_type_and_period(
            &self,
            report_type: ReportType,
            from: NaiveDate,
            to: NaiveDate,
        ) -> Result<Vec<RegulatoryReport>, String> {
            Ok(self
                .reports
                .lock()
                .unwrap()
                .iter()
                .filter(|r| {
                    r.report_type() == report_type
                        && r.period_start() >= from
                        && r.period_end() <= to
                })
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            report_type: Option<ReportType>,
            status: Option<ReportStatus>,
            _from: Option<NaiveDate>,
            _to: Option<NaiveDate>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<RegulatoryReport>, String> {
            Ok(self
                .reports
                .lock()
                .unwrap()
                .iter()
                .filter(|r| report_type.is_none() || Some(r.report_type()) == report_type)
                .filter(|r| status.is_none() || Some(r.status()) == status)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(
            &self,
            report_type: Option<ReportType>,
            status: Option<ReportStatus>,
            _from: Option<NaiveDate>,
            _to: Option<NaiveDate>,
        ) -> Result<i64, String> {
            Ok(self
                .reports
                .lock()
                .unwrap()
                .iter()
                .filter(|r| report_type.is_none() || Some(r.report_type()) == report_type)
                .filter(|r| status.is_none() || Some(r.status()) == status)
                .count() as i64)
        }
    }

    // --- Mock Template Repository ---

    struct MockTemplateRepo {
        templates: Mutex<Vec<ReportTemplate>>,
    }

    impl MockTemplateRepo {
        fn with_monthly_template() -> Self {
            let template = ReportTemplate::new(
                "Monthly BCT Report".to_string(),
                ReportType::Monthly,
                "1.0".to_string(),
                r#"{"sections":["balance","pnl"]}"#.to_string(),
            )
            .unwrap();
            MockTemplateRepo {
                templates: Mutex::new(vec![template]),
            }
        }
    }

    #[async_trait]
    impl IReportTemplateRepository for MockTemplateRepo {
        async fn save(&self, template: &ReportTemplate) -> Result<(), String> {
            self.templates.lock().unwrap().push(template.clone());
            Ok(())
        }
        async fn find_by_id(
            &self,
            id: &TemplateId,
        ) -> Result<Option<ReportTemplate>, String> {
            Ok(self
                .templates
                .lock()
                .unwrap()
                .iter()
                .find(|t| t.template_id() == id)
                .cloned())
        }
        async fn find_active_by_type(
            &self,
            report_type: &ReportType,
        ) -> Result<Option<ReportTemplate>, String> {
            Ok(self
                .templates
                .lock()
                .unwrap()
                .iter()
                .find(|t| t.report_type() == *report_type && t.is_active())
                .cloned())
        }
        async fn find_all(&self) -> Result<Vec<ReportTemplate>, String> {
            Ok(self.templates.lock().unwrap().clone())
        }
    }

    #[tokio::test]
    async fn test_e2e_monthly_reporting() {
        let reporting_service = ReportingService::new(
            Arc::new(MockReportRepo::new()),
            Arc::new(MockTemplateRepo::with_monthly_template()),
        );

        let generated_by = Uuid::new_v4();

        // Step 1: Generate monthly report
        let report = reporting_service
            .generate_monthly_report(2026, 3, generated_by)
            .await
            .unwrap();
        assert_eq!(report.report_type, "Monthly");
        assert_eq!(report.status, "Generated");
        let report_id = ReportId::from_uuid(Uuid::parse_str(&report.id).unwrap());

        // Step 2: Validate report
        let validated = reporting_service
            .validate_report(&report_id)
            .await
            .unwrap();
        assert_eq!(validated.status, "Validated");

        // Step 3: Submit to BCT
        let submitted = reporting_service
            .submit_report(&report_id)
            .await
            .unwrap();
        assert_eq!(submitted.status, "Submitted");
        assert!(submitted.submitted_at.is_some());

        // Step 4: Acknowledge
        let acknowledged = reporting_service
            .acknowledge_report(&report_id)
            .await
            .unwrap();
        assert_eq!(acknowledged.status, "Acknowledged");
        assert!(acknowledged.acknowledged_at.is_some());

        // Step 5: Verify report lifecycle via list
        let list = reporting_service
            .list_reports(
                Some(ReportType::Monthly),
                Some(ReportStatus::Acknowledged),
                None,
                None,
                1,
                10,
            )
            .await
            .unwrap();
        assert_eq!(list.total, 1);
        assert_eq!(list.data[0].status, "Acknowledged");
    }

    #[tokio::test]
    async fn test_e2e_report_lifecycle_invalid_order() {
        let reporting_service = ReportingService::new(
            Arc::new(MockReportRepo::new()),
            Arc::new(MockTemplateRepo::with_monthly_template()),
        );

        let report = reporting_service
            .generate_monthly_report(2026, 3, Uuid::new_v4())
            .await
            .unwrap();
        let report_id = ReportId::from_uuid(Uuid::parse_str(&report.id).unwrap());

        // Cannot submit before validating
        let submit_result = reporting_service.submit_report(&report_id).await;
        assert!(submit_result.is_err());

        // Cannot acknowledge before submitting
        let ack_result = reporting_service.acknowledge_report(&report_id).await;
        assert!(ack_result.is_err());
    }
}

// ==============================================================================
// Invariant Tests
// ==============================================================================

mod test_invariants {
    use super::*;

    // INV-02: Solvency >= 10%
    #[test]
    fn test_inv02_solvency_minimum() {
        use banko_domain::prudential::*;

        // Solvency = (T1 + T2) / RWA * 100
        // With T1=50_000, T2=20_000, RWA=1_000_000 => 7% < 10% => should fail
        let result = PrudentialRatio::new(
            InstitutionId::new(),
            50_000,
            20_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        );
        assert!(result.is_err());
    }

    // INV-03: Tier1 >= 7%
    #[test]
    fn test_inv03_tier1_minimum() {
        use banko_domain::prudential::*;

        // T1=60_000, T2=60_000, RWA=1_000_000 => Solvency = 12%, Tier1 = 6% < 7% => fail
        let result = PrudentialRatio::new(
            InstitutionId::new(),
            60_000,
            60_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        );
        assert!(result.is_err());
    }

    // INV-04: C/D <= 120%
    #[test]
    fn test_inv04_credit_deposit_ratio() {
        use banko_domain::prudential::*;

        // C/D = credits/deposits * 100
        // credits=1_000_000, deposits=800_000 => 125% > 120% => fail
        let result = PrudentialRatio::new(
            InstitutionId::new(),
            200_000,
            100_000,
            1_000_000,
            1_000_000,
            800_000,
            vec![],
        );
        assert!(result.is_err());
    }

    // INV-05: Concentration <= 25%
    #[test]
    fn test_inv05_concentration_limit() {
        use banko_domain::prudential::*;

        let beneficiary = Uuid::new_v4();
        // FPN = T1 + T2 = 200_000 + 100_000 = 300_000
        // Exposure = 100_000 => concentration = 100_000/300_000 = 33.3% > 25% => fail
        let result = PrudentialRatio::new(
            InstitutionId::new(),
            200_000,
            100_000,
            1_000_000,
            500_000,
            800_000,
            vec![Exposure::new(beneficiary, 100_000, "Large exposure".to_string())],
        );
        assert!(result.is_err());
    }

    // INV-02 to INV-05: Valid ratios should pass
    #[test]
    fn test_inv02_to_inv05_valid() {
        use banko_domain::prudential::*;

        let result = PrudentialRatio::new(
            InstitutionId::new(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        );
        assert!(result.is_ok());
        let ratio = result.unwrap();
        // Solvency = (150+50)/1000 * 100 = 20%
        assert!(ratio.solvency_ratio() >= 10.0);
        // Tier1 = 150/1000 * 100 = 15%
        assert!(ratio.tier1_ratio() >= 7.0);
        // C/D = 500/800 * 100 = 62.5%
        assert!(ratio.credit_deposit_ratio() <= 120.0);
    }

    // INV-11: Unbalanced entry rejected
    #[test]
    fn test_inv11_unbalanced_entry_rejected() {
        use banko_domain::accounting::*;

        let result = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Unbalanced".to_string(),
            vec![
                JournalLine::new(
                    AccountCode::new("31").unwrap(),
                    1000,
                    0,
                    None,
                )
                .unwrap(),
                JournalLine::new(
                    AccountCode::new("42").unwrap(),
                    0,
                    999,
                    None,
                )
                .unwrap(),
            ],
        );
        assert!(result.is_err());
    }

    // INV-11: Balanced entry succeeds
    #[test]
    fn test_inv11_balanced_entry_succeeds() {
        use banko_domain::accounting::*;

        let result = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Balanced".to_string(),
            vec![
                JournalLine::new(
                    AccountCode::new("31").unwrap(),
                    1000,
                    0,
                    None,
                )
                .unwrap(),
                JournalLine::new(
                    AccountCode::new("42").unwrap(),
                    0,
                    1000,
                    None,
                )
                .unwrap(),
            ],
        );
        assert!(result.is_ok());
    }

    // INV-12: Audit trail hash chain integrity
    #[test]
    fn test_inv12_audit_hash_chain_integrity() {
        use banko_domain::governance::*;

        let genesis_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();

        let entry1 = AuditTrailEntry::new(
            Uuid::new_v4(),
            AuditAction::Create,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            genesis_hash,
        );

        let entry2 = AuditTrailEntry::new(
            Uuid::new_v4(),
            AuditAction::Update,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            entry1.hash().to_string(),
        );

        let entry3 = AuditTrailEntry::new(
            Uuid::new_v4(),
            AuditAction::Approve,
            ResourceType::Account,
            Uuid::new_v4(),
            None,
            None,
            entry2.hash().to_string(),
        );

        // Valid chain should pass
        let result = HashChain::verify_chain(&[entry1.clone(), entry2.clone(), entry3.clone()]);
        assert!(result.is_ok());

        // Tampered chain should fail (wrong previous hash)
        let tampered = AuditTrailEntry::new(
            Uuid::new_v4(),
            AuditAction::Delete,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            "wrong_hash".to_string(),
        );

        let result = HashChain::verify_chain(&[entry1, entry2, tampered]);
        assert!(result.is_err());
    }

    // INV-13: Customer requires consent
    #[test]
    fn test_inv13_customer_requires_consent() {
        use banko_domain::customer::*;
        use banko_domain::shared::value_objects::*;

        let address = Address::new("10 Rue", "Tunis", "1000", "Tunisia").unwrap();
        let phone = PhoneNumber::new("+21698123456").unwrap();
        let email = EmailAddress::new("test@example.com").unwrap();
        let cin = Cin::new("12345678").unwrap();
        let birth_date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();

        let kyc = KycProfile::new_individual(
            "Test User",
            cin,
            birth_date,
            "Tunisia",
            "Engineer",
            address,
            phone,
            email,
            PepStatus::No,
            SourceOfFunds::Salary,
        )
        .unwrap();

        // ConsentStatus::NotGiven should cause error
        let result = Customer::new(
            CustomerType::Individual,
            kyc,
            vec![],
            ConsentStatus::NotGiven,
        );
        assert!(result.is_err());
    }

    // INV-14: Payment requires screening
    #[tokio::test]
    async fn test_inv14_payment_requires_screening() {
        use crate::payment::*;

        // --- Minimal mock repos for payment ---

        struct MockPaymentRepo {
            orders: Mutex<Vec<banko_domain::payment::PaymentOrder>>,
        }

        impl MockPaymentRepo {
            fn new() -> Self {
                MockPaymentRepo {
                    orders: Mutex::new(Vec::new()),
                }
            }
        }

        #[async_trait]
        impl IPaymentRepository for MockPaymentRepo {
            async fn save(
                &self,
                order: &banko_domain::payment::PaymentOrder,
            ) -> Result<(), String> {
                let mut orders = self.orders.lock().unwrap();
                orders.retain(|o| o.order_id() != order.order_id());
                orders.push(order.clone());
                Ok(())
            }
            async fn find_by_id(
                &self,
                id: &banko_domain::payment::OrderId,
            ) -> Result<Option<banko_domain::payment::PaymentOrder>, String> {
                Ok(self
                    .orders
                    .lock()
                    .unwrap()
                    .iter()
                    .find(|o| o.order_id() == id)
                    .cloned())
            }
            async fn find_by_account(
                &self,
                _account_id: Uuid,
            ) -> Result<Vec<banko_domain::payment::PaymentOrder>, String> {
                Ok(vec![])
            }
            async fn find_all(
                &self,
                _status: Option<banko_domain::payment::PaymentStatus>,
                _limit: i64,
                _offset: i64,
            ) -> Result<Vec<banko_domain::payment::PaymentOrder>, String> {
                Ok(vec![])
            }
            async fn count_all(
                &self,
                _status: Option<banko_domain::payment::PaymentStatus>,
            ) -> Result<i64, String> {
                Ok(0)
            }
        }

        struct MockClearScreener;

        #[async_trait]
        impl ISanctionsScreener for MockClearScreener {
            async fn screen_beneficiary(
                &self,
                _name: &str,
                _bic: Option<&str>,
            ) -> Result<ScreeningResult, String> {
                Ok(ScreeningResult {
                    is_hit: false,
                    match_details: None,
                })
            }
        }

        let service = PaymentService::new(
            Arc::new(MockPaymentRepo::new()),
            Arc::new(MockClearScreener),
        );

        // Create international payment
        let req = CreatePaymentRequest {
            sender_account_id: Uuid::new_v4().to_string(),
            beneficiary_name: "Pierre Dupont".to_string(),
            beneficiary_rib: None,
            beneficiary_bic: Some("BNPAFRPP".to_string()),
            amount: 100_000,
            currency: Some("EUR".to_string()),
            payment_type: "International".to_string(),
            reference: "REF-INT-001".to_string(),
            description: None,
        };

        let created = service.create_payment(req).await.unwrap();

        // INV-14: Submit international payment WITHOUT screening first -> should fail
        let result = service.submit_payment(&created.id).await;
        assert!(result.is_err());
    }
}
