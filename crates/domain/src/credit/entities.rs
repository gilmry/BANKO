use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{CustomerId, Money};

use super::value_objects::{
    AmortizationType, AssetClass, InstallmentId, LoanId, LoanStatus, PaymentFrequency,
};

// --- Provision value object ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Provision {
    amount: Money,
    rate: f64,
    asset_class: AssetClass,
}

impl Provision {
    /// Create a new provision. Enforces INV-07 & INV-15:
    /// provision amount ≥ min_provision_pct * exposure
    pub fn new(
        amount: Money,
        asset_class: AssetClass,
        exposure: &Money,
    ) -> Result<Self, DomainError> {
        let min_pct = asset_class.min_provision_pct();
        let min_amount_cents = (exposure.amount_cents() as f64 * min_pct).ceil() as i64;

        if amount.amount_cents() < min_amount_cents {
            let required_amount =
                min_amount_cents as f64 / 10_f64.powi(amount.currency().decimal_places() as i32);
            return Err(DomainError::InsufficientProvision(format!(
                "asset class {} requires {:.0}% (min {:.3}), got {:.3}",
                asset_class.as_i32(),
                min_pct * 100.0,
                required_amount,
                amount.amount(),
            )));
        }

        let rate = if exposure.amount_cents() > 0 {
            amount.amount_cents() as f64 / exposure.amount_cents() as f64
        } else {
            0.0
        };

        Ok(Provision {
            amount,
            rate,
            asset_class,
        })
    }

    /// Calculate the regulatory minimum provision for a given exposure and asset class.
    pub fn calculate_regulatory_minimum(exposure: &Money, asset_class: AssetClass) -> Money {
        let min_pct = asset_class.min_provision_pct();
        let min_cents = (exposure.amount_cents() as f64 * min_pct).ceil() as i64;
        Money::from_cents(min_cents, exposure.currency())
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(amount: Money, rate: f64, asset_class: AssetClass) -> Self {
        Provision {
            amount,
            rate,
            asset_class,
        }
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn asset_class(&self) -> AssetClass {
        self.asset_class
    }
}

// --- Installment entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Installment {
    id: InstallmentId,
    loan_id: LoanId,
    installment_number: u32,
    due_date: NaiveDate,
    principal_amount: Money,
    interest_amount: Money,
    total_amount: Money,
    remaining_balance: Money,
    paid: bool,
    paid_date: Option<DateTime<Utc>>,
}

impl Installment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        loan_id: LoanId,
        installment_number: u32,
        due_date: NaiveDate,
        principal_amount: Money,
        interest_amount: Money,
        total_amount: Money,
        remaining_balance: Money,
    ) -> Self {
        Installment {
            id: InstallmentId::new(),
            loan_id,
            installment_number,
            due_date,
            principal_amount,
            interest_amount,
            total_amount,
            remaining_balance,
            paid: false,
            paid_date: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: InstallmentId,
        loan_id: LoanId,
        installment_number: u32,
        due_date: NaiveDate,
        principal_amount: Money,
        interest_amount: Money,
        total_amount: Money,
        remaining_balance: Money,
        paid: bool,
        paid_date: Option<DateTime<Utc>>,
    ) -> Self {
        Installment {
            id,
            loan_id,
            installment_number,
            due_date,
            principal_amount,
            interest_amount,
            total_amount,
            remaining_balance,
            paid,
            paid_date,
        }
    }

    pub fn mark_paid(&mut self) {
        self.paid = true;
        self.paid_date = Some(Utc::now());
    }

    pub fn id(&self) -> &InstallmentId {
        &self.id
    }
    pub fn loan_id(&self) -> &LoanId {
        &self.loan_id
    }
    pub fn installment_number(&self) -> u32 {
        self.installment_number
    }
    pub fn due_date(&self) -> NaiveDate {
        self.due_date
    }
    pub fn principal_amount(&self) -> &Money {
        &self.principal_amount
    }
    pub fn interest_amount(&self) -> &Money {
        &self.interest_amount
    }
    pub fn total_amount(&self) -> &Money {
        &self.total_amount
    }
    pub fn remaining_balance(&self) -> &Money {
        &self.remaining_balance
    }
    pub fn paid(&self) -> bool {
        self.paid
    }
    pub fn paid_date(&self) -> Option<DateTime<Utc>> {
        self.paid_date
    }
}

// --- LoanSchedule entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoanSchedule {
    installments: Vec<Installment>,
    frequency: PaymentFrequency,
    amortization_type: AmortizationType,
    maturity_date: NaiveDate,
}

impl LoanSchedule {
    pub fn new(
        installments: Vec<Installment>,
        frequency: PaymentFrequency,
        amortization_type: AmortizationType,
        maturity_date: NaiveDate,
    ) -> Self {
        LoanSchedule {
            installments,
            frequency,
            amortization_type,
            maturity_date,
        }
    }

    pub fn installments(&self) -> &[Installment] {
        &self.installments
    }

    pub fn installments_mut(&mut self) -> &mut Vec<Installment> {
        &mut self.installments
    }

    pub fn frequency(&self) -> PaymentFrequency {
        self.frequency
    }

    pub fn amortization_type(&self) -> AmortizationType {
        self.amortization_type
    }

    pub fn maturity_date(&self) -> NaiveDate {
        self.maturity_date
    }

    pub fn next_unpaid_installment(&self) -> Option<&Installment> {
        self.installments.iter().find(|i| !i.paid)
    }

    pub fn total_paid(&self) -> usize {
        self.installments.iter().filter(|i| i.paid).count()
    }

    pub fn total_installments(&self) -> usize {
        self.installments.len()
    }
}

// --- Loan aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    id: LoanId,
    customer_id: CustomerId,
    account_id: crate::account::AccountId,
    amount: Money,
    interest_rate: f64,
    term_months: u32,
    asset_class: AssetClass,
    provision: Provision,
    schedule: Option<LoanSchedule>,
    status: LoanStatus,
    days_past_due: u32,
    disbursement_date: Option<NaiveDate>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Loan {
    /// Create a new loan application. Starts as Applied, Class0, zero provision.
    pub fn new(
        customer_id: CustomerId,
        account_id: crate::account::AccountId,
        amount: Money,
        interest_rate: f64,
        term_months: u32,
    ) -> Result<Self, DomainError> {
        if amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Loan amount must be positive".to_string(),
            ));
        }
        if interest_rate <= 0.0 {
            return Err(DomainError::ValidationError(
                "Interest rate must be positive".to_string(),
            ));
        }
        if term_months == 0 {
            return Err(DomainError::ValidationError(
                "Term months must be positive".to_string(),
            ));
        }

        let zero_provision =
            Provision::new(Money::zero(amount.currency()), AssetClass::Class0, &amount)?;

        let now = Utc::now();

        Ok(Loan {
            id: LoanId::new(),
            customer_id,
            account_id,
            amount,
            interest_rate,
            term_months,
            asset_class: AssetClass::Class0,
            provision: zero_provision,
            schedule: None,
            status: LoanStatus::Applied,
            days_past_due: 0,
            disbursement_date: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: LoanId,
        customer_id: CustomerId,
        account_id: crate::account::AccountId,
        amount: Money,
        interest_rate: f64,
        term_months: u32,
        asset_class: AssetClass,
        provision: Provision,
        schedule: Option<LoanSchedule>,
        status: LoanStatus,
        days_past_due: u32,
        disbursement_date: Option<NaiveDate>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Loan {
            id,
            customer_id,
            account_id,
            amount,
            interest_rate,
            term_months,
            asset_class,
            provision,
            schedule,
            status,
            days_past_due,
            disbursement_date,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &LoanId {
        &self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn account_id(&self) -> &crate::account::AccountId {
        &self.account_id
    }
    pub fn amount(&self) -> &Money {
        &self.amount
    }
    pub fn interest_rate(&self) -> f64 {
        self.interest_rate
    }
    pub fn term_months(&self) -> u32 {
        self.term_months
    }
    pub fn asset_class(&self) -> AssetClass {
        self.asset_class
    }
    pub fn provision(&self) -> &Provision {
        &self.provision
    }
    pub fn schedule(&self) -> Option<&LoanSchedule> {
        self.schedule.as_ref()
    }
    pub fn status(&self) -> LoanStatus {
        self.status
    }
    pub fn days_past_due(&self) -> u32 {
        self.days_past_due
    }
    pub fn disbursement_date(&self) -> Option<NaiveDate> {
        self.disbursement_date
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Approve the loan. Only Applied loans can be approved.
    pub fn approve(&mut self) -> Result<(), DomainError> {
        if self.status != LoanStatus::Applied {
            return Err(DomainError::InvalidLoanTransition(format!(
                "Cannot approve loan in status {}",
                self.status
            )));
        }
        self.status = LoanStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Disburse the loan. Only Approved loans can be disbursed.
    /// Generates the amortization schedule.
    pub fn disburse(
        &mut self,
        disbursement_date: NaiveDate,
        frequency: PaymentFrequency,
        amortization_type: AmortizationType,
    ) -> Result<(), DomainError> {
        if self.status != LoanStatus::Approved {
            return Err(DomainError::InvalidLoanTransition(format!(
                "Cannot disburse loan in status {}",
                self.status
            )));
        }

        let schedule = self.generate_schedule(disbursement_date, frequency, amortization_type)?;
        self.schedule = Some(schedule);
        self.status = LoanStatus::Active;
        self.disbursement_date = Some(disbursement_date);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Generate amortization schedule.
    fn generate_schedule(
        &self,
        start_date: NaiveDate,
        frequency: PaymentFrequency,
        amortization_type: AmortizationType,
    ) -> Result<LoanSchedule, DomainError> {
        let n = self.term_months;
        let currency = self.amount.currency();
        let principal_cents = self.amount.amount_cents();
        let monthly_rate = self.interest_rate / 12.0 / 100.0;

        let mut installments = Vec::with_capacity(n as usize);
        let mut remaining = principal_cents;

        let months_per_period = match frequency {
            PaymentFrequency::Monthly => 1u32,
            PaymentFrequency::Quarterly => 3,
        };
        let num_periods = n / months_per_period;

        let period_rate = monthly_rate * months_per_period as f64;

        match amortization_type {
            AmortizationType::Constant => {
                // Annuité constante: PMT = P * r / (1 - (1+r)^-n)
                let pmt_cents = if period_rate > 0.0 {
                    let factor = (1.0 + period_rate).powi(num_periods as i32);
                    (principal_cents as f64 * period_rate * factor / (factor - 1.0)).round() as i64
                } else {
                    principal_cents / num_periods as i64
                };

                for i in 1..=num_periods {
                    let interest_cents = (remaining as f64 * period_rate).round() as i64;
                    let is_last = i == num_periods;

                    let principal_portion = if is_last {
                        remaining
                    } else {
                        pmt_cents - interest_cents
                    };

                    let total = if is_last {
                        remaining + interest_cents
                    } else {
                        pmt_cents
                    };

                    remaining -= principal_portion;

                    let due_date = add_months(start_date, i * months_per_period);

                    installments.push(Installment::new(
                        self.id.clone(),
                        i,
                        due_date,
                        Money::from_cents(principal_portion, currency),
                        Money::from_cents(interest_cents, currency),
                        Money::from_cents(total, currency),
                        Money::from_cents(remaining, currency),
                    ));
                }
            }
            AmortizationType::Linear => {
                // Principal constant par période
                let principal_per_period = principal_cents / num_periods as i64;

                for i in 1..=num_periods {
                    let interest_cents = (remaining as f64 * period_rate).round() as i64;
                    let is_last = i == num_periods;

                    let principal_portion = if is_last {
                        remaining
                    } else {
                        principal_per_period
                    };

                    let total = principal_portion + interest_cents;
                    remaining -= principal_portion;

                    let due_date = add_months(start_date, i * months_per_period);

                    installments.push(Installment::new(
                        self.id.clone(),
                        i,
                        due_date,
                        Money::from_cents(principal_portion, currency),
                        Money::from_cents(interest_cents, currency),
                        Money::from_cents(total, currency),
                        Money::from_cents(remaining, currency),
                    ));
                }
            }
        }

        let maturity_date = add_months(start_date, n);

        Ok(LoanSchedule::new(
            installments,
            frequency,
            amortization_type,
            maturity_date,
        ))
    }

    /// Classify asset based on days past due (INV-06).
    /// Circ. 91-24 art. 8 [REF-14]
    pub fn classify(&mut self, days_past_due: u32) -> AssetClass {
        self.days_past_due = days_past_due;
        let new_class = AssetClass::from_days_past_due(days_past_due);
        self.asset_class = new_class;
        self.updated_at = Utc::now();
        new_class
    }

    /// Update provision to meet regulatory minimum (INV-07, INV-15).
    /// Returns the new provision amount.
    pub fn update_provision(&mut self, provision_amount: Money) -> Result<Provision, DomainError> {
        let provision = Provision::new(provision_amount, self.asset_class, &self.amount)?;
        self.provision = provision.clone();
        self.updated_at = Utc::now();
        Ok(provision)
    }

    /// Calculate and set the regulatory minimum provision for the current asset class.
    pub fn apply_regulatory_provision(&mut self) -> Result<Provision, DomainError> {
        let min_amount = Provision::calculate_regulatory_minimum(&self.amount, self.asset_class);
        self.update_provision(min_amount)
    }

    /// Record a payment on the next unpaid installment.
    pub fn record_payment(&mut self) -> Result<&Installment, DomainError> {
        if self.status != LoanStatus::Active {
            return Err(DomainError::InvalidLoanTransition(
                "Cannot record payment on non-active loan".to_string(),
            ));
        }

        let schedule = self
            .schedule
            .as_mut()
            .ok_or_else(|| DomainError::ValidationError("Loan has no schedule".to_string()))?;

        let installment = schedule
            .installments_mut()
            .iter_mut()
            .find(|i| !i.paid)
            .ok_or_else(|| {
                DomainError::ValidationError("All installments already paid".to_string())
            })?;

        installment.mark_paid();
        self.updated_at = Utc::now();

        // Check if all paid -> close
        let all_paid = schedule.installments().iter().all(|i| i.paid);
        if all_paid {
            self.status = LoanStatus::Closed;
        }

        // Reset days past due on payment
        self.days_past_due = 0;
        self.asset_class = AssetClass::Class0;

        Ok(schedule
            .installments()
            .iter()
            .rev()
            .find(|i| i.paid)
            .unwrap())
    }

    /// Mark loan as defaulted.
    pub fn default_loan(&mut self) -> Result<(), DomainError> {
        if self.status != LoanStatus::Active {
            return Err(DomainError::InvalidLoanTransition(format!(
                "Cannot default loan in status {}",
                self.status
            )));
        }
        self.status = LoanStatus::Defaulted;
        self.updated_at = Utc::now();
        Ok(())
    }
}

/// Add months to a NaiveDate, clamping day to last day of month if needed.
fn add_months(date: NaiveDate, months: u32) -> NaiveDate {
    let total_months = date.month0() + months;
    let year = date.year() + (total_months / 12) as i32;
    let month = (total_months % 12) + 1;
    let day = date.day().min(days_in_month(year, month));
    NaiveDate::from_ymd_opt(year, month, day).unwrap_or(date)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::AccountId;
    use crate::shared::value_objects::{Currency, CustomerId, Money};

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    fn make_loan(amount: f64) -> Loan {
        Loan::new(CustomerId::new(), AccountId::new(), tnd(amount), 8.0, 12).unwrap()
    }

    // --- Loan creation tests ---

    #[test]
    fn test_new_loan_success() {
        let loan = make_loan(50000.0);
        assert_eq!(loan.status(), LoanStatus::Applied);
        assert_eq!(loan.asset_class(), AssetClass::Class0);
        assert_eq!(loan.amount().amount(), 50000.0);
        assert_eq!(loan.interest_rate(), 8.0);
        assert_eq!(loan.term_months(), 12);
        assert!(loan.provision().amount().is_zero());
    }

    #[test]
    fn test_new_loan_zero_amount_fails() {
        let result = Loan::new(CustomerId::new(), AccountId::new(), tnd(0.0), 8.0, 12);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_loan_negative_amount_fails() {
        let result = Loan::new(
            CustomerId::new(),
            AccountId::new(),
            Money::new(-1000.0, Currency::TND).unwrap(),
            8.0,
            12,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_new_loan_zero_interest_fails() {
        let result = Loan::new(CustomerId::new(), AccountId::new(), tnd(50000.0), 0.0, 12);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_loan_zero_term_fails() {
        let result = Loan::new(CustomerId::new(), AccountId::new(), tnd(50000.0), 8.0, 0);
        assert!(result.is_err());
    }

    // --- Status transitions ---

    #[test]
    fn test_approve_loan() {
        let mut loan = make_loan(50000.0);
        assert!(loan.approve().is_ok());
        assert_eq!(loan.status(), LoanStatus::Approved);
    }

    #[test]
    fn test_approve_non_applied_fails() {
        let mut loan = make_loan(50000.0);
        loan.approve().unwrap();
        assert!(loan.approve().is_err()); // Already approved
    }

    #[test]
    fn test_disburse_loan() {
        let mut loan = make_loan(50000.0);
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        assert!(loan
            .disburse(start, PaymentFrequency::Monthly, AmortizationType::Constant)
            .is_ok());
        assert_eq!(loan.status(), LoanStatus::Active);
        assert!(loan.schedule().is_some());
        assert_eq!(loan.schedule().unwrap().total_installments(), 12);
        assert_eq!(loan.disbursement_date(), Some(start));
    }

    #[test]
    fn test_disburse_non_approved_fails() {
        let mut loan = make_loan(50000.0);
        let start = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        assert!(loan
            .disburse(start, PaymentFrequency::Monthly, AmortizationType::Constant)
            .is_err());
    }

    // --- Asset classification (INV-06) ---

    #[test]
    fn test_classify_class0() {
        let mut loan = make_loan(50000.0);
        assert_eq!(loan.classify(0), AssetClass::Class0);
    }

    #[test]
    fn test_classify_class1() {
        let mut loan = make_loan(50000.0);
        assert_eq!(loan.classify(31), AssetClass::Class1);
    }

    #[test]
    fn test_classify_class2() {
        let mut loan = make_loan(50000.0);
        assert_eq!(loan.classify(91), AssetClass::Class2);
    }

    #[test]
    fn test_classify_class3() {
        let mut loan = make_loan(50000.0);
        assert_eq!(loan.classify(181), AssetClass::Class3);
    }

    #[test]
    fn test_classify_class4() {
        let mut loan = make_loan(50000.0);
        assert_eq!(loan.classify(365), AssetClass::Class4);
    }

    // --- Provision tests (INV-07, INV-15) ---

    #[test]
    fn test_provision_class0_zero_ok() {
        let exposure = tnd(100000.0);
        let provision = Provision::new(tnd(0.0), AssetClass::Class0, &exposure);
        assert!(provision.is_ok());
    }

    #[test]
    fn test_provision_class2_below_20pct_fails() {
        let exposure = tnd(100000.0);
        // 20% of 100000 = 20000; 19000 < 20000 → fail
        let result = Provision::new(tnd(19000.0), AssetClass::Class2, &exposure);
        assert!(result.is_err());
    }

    #[test]
    fn test_provision_class2_at_20pct_ok() {
        let exposure = tnd(100000.0);
        let provision = Provision::new(tnd(20000.0), AssetClass::Class2, &exposure);
        assert!(provision.is_ok());
        assert_eq!(provision.unwrap().amount().amount(), 20000.0);
    }

    #[test]
    fn test_provision_class3_below_50pct_fails() {
        let exposure = tnd(100000.0);
        let result = Provision::new(tnd(49000.0), AssetClass::Class3, &exposure);
        assert!(result.is_err());
    }

    #[test]
    fn test_provision_class3_at_50pct_ok() {
        let exposure = tnd(100000.0);
        let provision = Provision::new(tnd(50000.0), AssetClass::Class3, &exposure);
        assert!(provision.is_ok());
    }

    #[test]
    fn test_provision_class4_must_be_100pct() {
        let exposure = tnd(100000.0);
        let result = Provision::new(tnd(99000.0), AssetClass::Class4, &exposure);
        assert!(result.is_err());

        let provision = Provision::new(tnd(100000.0), AssetClass::Class4, &exposure);
        assert!(provision.is_ok());
    }

    #[test]
    fn test_regulatory_minimum_calculation() {
        let exposure = tnd(100000.0);
        let min = Provision::calculate_regulatory_minimum(&exposure, AssetClass::Class2);
        assert_eq!(min.amount(), 20000.0);

        let min3 = Provision::calculate_regulatory_minimum(&exposure, AssetClass::Class3);
        assert_eq!(min3.amount(), 50000.0);

        let min4 = Provision::calculate_regulatory_minimum(&exposure, AssetClass::Class4);
        assert_eq!(min4.amount(), 100000.0);
    }

    // --- Loan provision update ---

    #[test]
    fn test_loan_classify_then_provision() {
        let mut loan = make_loan(100000.0);
        loan.classify(91); // Class 2
        assert_eq!(loan.asset_class(), AssetClass::Class2);

        // Must provision ≥ 20%
        let result = loan.update_provision(tnd(20000.0));
        assert!(result.is_ok());
        assert_eq!(loan.provision().amount().amount(), 20000.0);

        // Below minimum fails
        let mut loan2 = make_loan(100000.0);
        loan2.classify(91);
        let result = loan2.update_provision(tnd(10000.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_regulatory_provision() {
        let mut loan = make_loan(100000.0);
        loan.classify(181); // Class 3 → 50%
        let provision = loan.apply_regulatory_provision().unwrap();
        assert_eq!(provision.amount().amount(), 50000.0);
    }

    // --- Schedule tests ---

    #[test]
    fn test_constant_amortization_schedule() {
        let mut loan =
            Loan::new(CustomerId::new(), AccountId::new(), tnd(100000.0), 8.0, 60).unwrap();
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        loan.disburse(start, PaymentFrequency::Monthly, AmortizationType::Constant)
            .unwrap();

        let schedule = loan.schedule().unwrap();
        assert_eq!(schedule.total_installments(), 60);

        // Check last installment remaining balance is 0
        let last = &schedule.installments()[59];
        assert_eq!(last.remaining_balance().amount_cents(), 0);

        // Check maturity date
        assert_eq!(
            schedule.maturity_date(),
            NaiveDate::from_ymd_opt(2031, 1, 15).unwrap()
        );

        // All installments should have positive amounts
        for inst in schedule.installments() {
            assert!(inst.principal_amount().amount_cents() > 0);
            assert!(inst.interest_amount().amount_cents() >= 0);
            assert!(inst.total_amount().amount_cents() > 0);
        }
    }

    #[test]
    fn test_linear_amortization_schedule() {
        let mut loan =
            Loan::new(CustomerId::new(), AccountId::new(), tnd(120000.0), 6.0, 12).unwrap();
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        loan.disburse(start, PaymentFrequency::Monthly, AmortizationType::Linear)
            .unwrap();

        let schedule = loan.schedule().unwrap();
        assert_eq!(schedule.total_installments(), 12);

        // Linear: principal per period should be ~10000 TND
        let first = &schedule.installments()[0];
        assert_eq!(first.principal_amount().amount(), 10000.0);

        // Interest should decrease over time
        let first_interest = schedule.installments()[0].interest_amount().amount_cents();
        let last_interest = schedule.installments()[11].interest_amount().amount_cents();
        assert!(first_interest > last_interest);

        // Last installment remaining balance should be 0
        let last = &schedule.installments()[11];
        assert_eq!(last.remaining_balance().amount_cents(), 0);
    }

    #[test]
    fn test_quarterly_schedule() {
        let mut loan =
            Loan::new(CustomerId::new(), AccountId::new(), tnd(100000.0), 8.0, 12).unwrap();
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        loan.disburse(start, PaymentFrequency::Quarterly, AmortizationType::Linear)
            .unwrap();

        let schedule = loan.schedule().unwrap();
        // 12 months / 3 months per quarter = 4 installments
        assert_eq!(schedule.total_installments(), 4);
    }

    // --- Payment recording ---

    #[test]
    fn test_record_payment() {
        let mut loan =
            Loan::new(CustomerId::new(), AccountId::new(), tnd(12000.0), 6.0, 3).unwrap();
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        loan.disburse(start, PaymentFrequency::Monthly, AmortizationType::Linear)
            .unwrap();

        // Record first payment
        let inst = loan.record_payment().unwrap();
        assert!(inst.paid());
        assert_eq!(loan.schedule().unwrap().total_paid(), 1);
        assert_eq!(loan.status(), LoanStatus::Active);
    }

    #[test]
    fn test_record_all_payments_closes_loan() {
        let mut loan =
            Loan::new(CustomerId::new(), AccountId::new(), tnd(12000.0), 6.0, 3).unwrap();
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        loan.disburse(start, PaymentFrequency::Monthly, AmortizationType::Linear)
            .unwrap();

        loan.record_payment().unwrap();
        loan.record_payment().unwrap();
        loan.record_payment().unwrap();

        assert_eq!(loan.status(), LoanStatus::Closed);
    }

    #[test]
    fn test_record_payment_non_active_fails() {
        let mut loan = make_loan(50000.0);
        assert!(loan.record_payment().is_err());
    }

    // --- Default ---

    #[test]
    fn test_default_loan() {
        let mut loan = make_loan(50000.0);
        loan.approve().unwrap();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        loan.disburse(start, PaymentFrequency::Monthly, AmortizationType::Constant)
            .unwrap();
        assert!(loan.default_loan().is_ok());
        assert_eq!(loan.status(), LoanStatus::Defaulted);
    }

    // --- Integration: classify → provision ≥ 20% ---

    #[test]
    fn test_classify_class2_provision_20pct() {
        let mut loan = make_loan(100000.0);
        loan.classify(91); // Class 2
        assert_eq!(loan.asset_class(), AssetClass::Class2);

        // Apply regulatory minimum
        let provision = loan.apply_regulatory_provision().unwrap();
        assert!(provision.amount().amount() >= 20000.0);
        assert_eq!(provision.amount().amount(), 20000.0);
    }

    // --- add_months helper ---

    #[test]
    fn test_add_months() {
        let d = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
        let result = add_months(d, 1);
        // Feb 2026 has 28 days
        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 2, 28).unwrap());
    }
}
