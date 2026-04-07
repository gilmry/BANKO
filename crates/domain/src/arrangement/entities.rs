use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::CustomerId;
use crate::account::AccountId;

use super::value_objects::{
    ArrangementId, ArrangementType, ArrangementStatus, RenewalType, ArrangementEventId,
    ArrangementEventType, ArrangementBundleId,
};

// --- ArrangementTerms ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrangementTerms {
    interest_rate: Option<f64>,
    fee_schedule_id: Option<String>,
    currency: String,
    minimum_balance: Option<i64>,
    overdraft_limit: Option<i64>,
    renewal_type: RenewalType,
    notice_period_days: Option<u32>,
}

impl ArrangementTerms {
    pub fn new(
        currency: &str,
        interest_rate: Option<f64>,
        fee_schedule_id: Option<String>,
        minimum_balance: Option<i64>,
        overdraft_limit: Option<i64>,
        renewal_type: RenewalType,
        notice_period_days: Option<u32>,
    ) -> Result<Self, DomainError> {
        if currency.is_empty() {
            return Err(DomainError::ValidationError(
                "Currency cannot be empty".to_string(),
            ));
        }

        // Validate interest rate bounds
        if let Some(rate) = interest_rate {
            if rate < 0.0 || rate > 100.0 {
                return Err(DomainError::InvalidPercentage(format!(
                    "Interest rate must be between 0 and 100, got {}",
                    rate
                )));
            }
        }

        Ok(ArrangementTerms {
            interest_rate,
            fee_schedule_id,
            currency: currency.to_string(),
            minimum_balance,
            overdraft_limit,
            renewal_type,
            notice_period_days,
        })
    }

    // Getters
    pub fn interest_rate(&self) -> Option<f64> {
        self.interest_rate
    }

    pub fn fee_schedule_id(&self) -> Option<&str> {
        self.fee_schedule_id.as_deref()
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn minimum_balance(&self) -> Option<i64> {
        self.minimum_balance
    }

    pub fn overdraft_limit(&self) -> Option<i64> {
        self.overdraft_limit
    }

    pub fn renewal_type(&self) -> RenewalType {
        self.renewal_type
    }

    pub fn notice_period_days(&self) -> Option<u32> {
        self.notice_period_days
    }
}

// --- Arrangement Aggregate Root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrangement {
    id: ArrangementId,
    customer_id: CustomerId,
    product_id: String,
    arrangement_type: ArrangementType,
    status: ArrangementStatus,
    effective_date: DateTime<Utc>,
    maturity_date: Option<DateTime<Utc>>,
    terms: ArrangementTerms,
    linked_accounts: Vec<AccountId>,
    events: Vec<ArrangementEvent>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Arrangement {
    pub fn new(
        customer_id: CustomerId,
        product_id: &str,
        arrangement_type: ArrangementType,
        effective_date: DateTime<Utc>,
        maturity_date: Option<DateTime<Utc>>,
        terms: ArrangementTerms,
    ) -> Result<Self, DomainError> {
        // Effective date cannot be in the past
        if effective_date < Utc::now() {
            return Err(DomainError::ValidationError(
                "Effective date cannot be in the past".to_string(),
            ));
        }

        // Maturity date must be after effective date if set
        if let Some(maturity) = maturity_date {
            if maturity <= effective_date {
                return Err(DomainError::ValidationError(
                    "Maturity date must be after effective date".to_string(),
                ));
            }
        }

        let id = ArrangementId::new();
        let now = Utc::now();

        let mut arrangement = Arrangement {
            id: id.clone(),
            customer_id,
            product_id: product_id.to_string(),
            arrangement_type,
            status: ArrangementStatus::Proposed,
            effective_date,
            maturity_date,
            terms,
            linked_accounts: Vec::new(),
            events: Vec::new(),
            created_at: now,
            updated_at: now,
        };

        // Add creation event
        let event = ArrangementEvent::new(
            id,
            ArrangementEventType::Created,
            "Arrangement created",
        )?;
        arrangement.events.push(event);

        Ok(arrangement)
    }

    pub fn reconstitute(
        id: ArrangementId,
        customer_id: CustomerId,
        product_id: String,
        arrangement_type: ArrangementType,
        status: ArrangementStatus,
        effective_date: DateTime<Utc>,
        maturity_date: Option<DateTime<Utc>>,
        terms: ArrangementTerms,
        linked_accounts: Vec<AccountId>,
        events: Vec<ArrangementEvent>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Arrangement {
            id,
            customer_id,
            product_id,
            arrangement_type,
            status,
            effective_date,
            maturity_date,
            terms,
            linked_accounts,
            events,
            created_at,
            updated_at,
        }
    }

    pub fn activate(&mut self) -> Result<(), DomainError> {
        if self.status != ArrangementStatus::Proposed {
            return Err(DomainError::ValidationError(
                "Can only activate proposed arrangements".to_string(),
            ));
        }
        self.status = ArrangementStatus::Active;
        self.updated_at = Utc::now();

        let event = ArrangementEvent::new(
            self.id.clone(),
            ArrangementEventType::Activated,
            "Arrangement activated",
        )?;
        self.events.push(event);

        Ok(())
    }

    pub fn suspend(&mut self) -> Result<(), DomainError> {
        if self.status == ArrangementStatus::Closed {
            return Err(DomainError::ValidationError(
                "Cannot suspend closed arrangement".to_string(),
            ));
        }
        self.status = ArrangementStatus::Suspended;
        self.updated_at = Utc::now();

        let event = ArrangementEvent::new(
            self.id.clone(),
            ArrangementEventType::Suspended,
            "Arrangement suspended",
        )?;
        self.events.push(event);

        Ok(())
    }

    pub fn close(&mut self, loan_balance: i64) -> Result<(), DomainError> {
        if loan_balance > 0 {
            return Err(DomainError::ValidationError(
                "Cannot close arrangement with outstanding loan balance".to_string(),
            ));
        }
        self.status = ArrangementStatus::Closed;
        self.updated_at = Utc::now();

        let event = ArrangementEvent::new(
            self.id.clone(),
            ArrangementEventType::Closed,
            "Arrangement closed",
        )?;
        self.events.push(event);

        Ok(())
    }

    pub fn mark_matured(&mut self) -> Result<(), DomainError> {
        if self.status != ArrangementStatus::Active {
            return Err(DomainError::ValidationError(
                "Can only mature active arrangements".to_string(),
            ));
        }
        self.status = ArrangementStatus::Matured;
        self.updated_at = Utc::now();

        let event = ArrangementEvent::new(
            self.id.clone(),
            ArrangementEventType::Matured,
            "Arrangement matured",
        )?;
        self.events.push(event);

        Ok(())
    }

    pub fn link_account(&mut self, account_id: AccountId) -> Result<(), DomainError> {
        if self.linked_accounts.contains(&account_id) {
            return Err(DomainError::ValidationError(
                "Account already linked to this arrangement".to_string(),
            ));
        }
        self.linked_accounts.push(account_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn unlink_account(&mut self, account_id: &AccountId) -> Result<(), DomainError> {
        if self.linked_accounts.is_empty() {
            return Err(DomainError::ValidationError(
                "No linked accounts to unlink".to_string(),
            ));
        }
        self.linked_accounts.retain(|a| a != account_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn check_renewal_trigger(&self) -> bool {
        if let Some(maturity) = self.maturity_date {
            let now = Utc::now();
            let notice_days = self.terms.notice_period_days().unwrap_or(30);
            let trigger_date = maturity - Duration::days(notice_days as i64);
            now >= trigger_date && now < maturity
        } else {
            false
        }
    }

    pub fn record_interest_application(&mut self, amount: i64) -> Result<(), DomainError> {
        let event = ArrangementEvent::new(
            self.id.clone(),
            ArrangementEventType::InterestApplied,
            &format!("Interest applied: {}", amount),
        )?;
        self.events.push(event);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn record_fee_charge(&mut self, amount: i64) -> Result<(), DomainError> {
        let event = ArrangementEvent::new(
            self.id.clone(),
            ArrangementEventType::FeeCharged,
            &format!("Fee charged: {}", amount),
        )?;
        self.events.push(event);
        self.updated_at = Utc::now();
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &ArrangementId {
        &self.id
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn product_id(&self) -> &str {
        &self.product_id
    }

    pub fn arrangement_type(&self) -> ArrangementType {
        self.arrangement_type
    }

    pub fn status(&self) -> ArrangementStatus {
        self.status
    }

    pub fn effective_date(&self) -> DateTime<Utc> {
        self.effective_date
    }

    pub fn maturity_date(&self) -> Option<DateTime<Utc>> {
        self.maturity_date
    }

    pub fn terms(&self) -> &ArrangementTerms {
        &self.terms
    }

    pub fn linked_accounts(&self) -> &[AccountId] {
        &self.linked_accounts
    }

    pub fn events(&self) -> &[ArrangementEvent] {
        &self.events
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- ArrangementEvent ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrangementEvent {
    id: ArrangementEventId,
    arrangement_id: ArrangementId,
    event_type: ArrangementEventType,
    event_data_json: String,
    occurred_at: DateTime<Utc>,
}

impl ArrangementEvent {
    pub fn new(
        arrangement_id: ArrangementId,
        event_type: ArrangementEventType,
        event_data: &str,
    ) -> Result<Self, DomainError> {
        Ok(ArrangementEvent {
            id: ArrangementEventId::new(),
            arrangement_id,
            event_type,
            event_data_json: event_data.to_string(),
            occurred_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: ArrangementEventId,
        arrangement_id: ArrangementId,
        event_type: ArrangementEventType,
        event_data_json: String,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        ArrangementEvent {
            id,
            arrangement_id,
            event_type,
            event_data_json,
            occurred_at,
        }
    }

    // Getters
    pub fn id(&self) -> &ArrangementEventId {
        &self.id
    }

    pub fn arrangement_id(&self) -> &ArrangementId {
        &self.arrangement_id
    }

    pub fn event_type(&self) -> ArrangementEventType {
        self.event_type
    }

    pub fn event_data_json(&self) -> &str {
        &self.event_data_json
    }

    pub fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

// --- ArrangementBundle ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrangementBundle {
    id: ArrangementBundleId,
    name: String,
    arrangements: Vec<ArrangementId>,
    discount_pct: Option<f64>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ArrangementBundle {
    pub fn new(
        name: &str,
        arrangements: Vec<ArrangementId>,
        discount_pct: Option<f64>,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Bundle name cannot be empty".to_string(),
            ));
        }

        if arrangements.is_empty() {
            return Err(DomainError::ValidationError(
                "Bundle must contain at least one arrangement".to_string(),
            ));
        }

        // Validate discount percentage
        if let Some(discount) = discount_pct {
            if discount < 0.0 || discount > 100.0 {
                return Err(DomainError::InvalidPercentage(format!(
                    "Discount must be between 0 and 100, got {}",
                    discount
                )));
            }
        }

        Ok(ArrangementBundle {
            id: ArrangementBundleId::new(),
            name: name.to_string(),
            arrangements,
            discount_pct,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: ArrangementBundleId,
        name: String,
        arrangements: Vec<ArrangementId>,
        discount_pct: Option<f64>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ArrangementBundle {
            id,
            name,
            arrangements,
            discount_pct,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn add_arrangement(&mut self, arrangement_id: ArrangementId) -> Result<(), DomainError> {
        if self.arrangements.contains(&arrangement_id) {
            return Err(DomainError::ValidationError(
                "Arrangement already in bundle".to_string(),
            ));
        }
        self.arrangements.push(arrangement_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_arrangement(
        &mut self,
        arrangement_id: &ArrangementId,
    ) -> Result<(), DomainError> {
        if !self.arrangements.contains(arrangement_id) {
            return Err(DomainError::ValidationError(
                "Arrangement not in bundle".to_string(),
            ));
        }
        self.arrangements.retain(|a| a != arrangement_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &ArrangementBundleId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arrangements(&self) -> &[ArrangementId] {
        &self.arrangements
    }

    pub fn discount_pct(&self) -> Option<f64> {
        self.discount_pct
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::CustomerId;

    #[test]
    fn test_arrangement_terms_creation() {
        let terms = ArrangementTerms::new(
            "EUR",
            Some(3.5),
            None,
            Some(1000),
            Some(5000),
            RenewalType::Automatic,
            Some(30),
        );
        assert!(terms.is_ok());
        let terms = terms.unwrap();
        assert_eq!(terms.currency(), "EUR");
        assert_eq!(terms.interest_rate(), Some(3.5));
        assert_eq!(terms.renewal_type(), RenewalType::Automatic);
    }

    #[test]
    fn test_arrangement_terms_invalid_interest_rate() {
        let terms = ArrangementTerms::new(
            "EUR",
            Some(150.0),
            None,
            None,
            None,
            RenewalType::None,
            None,
        );
        assert!(terms.is_err());
    }

    #[test]
    fn test_arrangement_creation() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let future = now + Duration::days(30);
        let terms = ArrangementTerms::new(
            "EUR",
            Some(2.5),
            None,
            None,
            None,
            RenewalType::Automatic,
            Some(30),
        ).unwrap();

        let arrangement = Arrangement::new(
            customer_id.clone(),
            "PROD_001",
            ArrangementType::CurrentAccount,
            future,
            None,
            terms,
        );
        assert!(arrangement.is_ok());
        let arrangement = arrangement.unwrap();
        assert_eq!(arrangement.status(), ArrangementStatus::Proposed);
        assert_eq!(arrangement.arrangement_type(), ArrangementType::CurrentAccount);
    }

    #[test]
    fn test_arrangement_past_effective_date() {
        let customer_id = CustomerId::new();
        let past = Utc::now() - Duration::days(1);
        let terms = ArrangementTerms::new(
            "EUR",
            None,
            None,
            None,
            None,
            RenewalType::None,
            None,
        ).unwrap();

        let arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::Deposit,
            past,
            None,
            terms,
        );
        assert!(arrangement.is_err());
    }

    #[test]
    fn test_arrangement_maturity_validation() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let future = now + Duration::days(30);
        let past = now - Duration::days(1);
        let terms = ArrangementTerms::new(
            "EUR",
            None,
            None,
            None,
            None,
            RenewalType::None,
            None,
        ).unwrap();

        // Maturity before effective date should fail
        let arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::TermDeposit,
            future,
            Some(past),
            terms,
        );
        assert!(arrangement.is_err());
    }

    #[test]
    fn test_arrangement_lifecycle() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let future = now + Duration::days(30);
        let terms = ArrangementTerms::new(
            "EUR",
            Some(3.0),
            None,
            None,
            None,
            RenewalType::Manual,
            Some(30),
        ).unwrap();

        let mut arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::Loan,
            future,
            Some(future + Duration::days(365)),
            terms,
        ).unwrap();

        // Activate
        assert!(arrangement.activate().is_ok());
        assert_eq!(arrangement.status(), ArrangementStatus::Active);

        // Suspend
        assert!(arrangement.suspend().is_ok());
        assert_eq!(arrangement.status(), ArrangementStatus::Suspended);

        // Cannot close with outstanding balance
        assert!(arrangement.close(1000).is_err());

        // Can close with zero balance
        assert!(arrangement.close(0).is_ok());
        assert_eq!(arrangement.status(), ArrangementStatus::Closed);
    }

    #[test]
    fn test_arrangement_link_accounts() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let future = now + Duration::days(30);
        let terms = ArrangementTerms::new(
            "EUR",
            None,
            None,
            None,
            None,
            RenewalType::None,
            None,
        ).unwrap();

        let mut arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::CurrentAccount,
            future,
            None,
            terms,
        ).unwrap();

        let account_id_1 = AccountId::new();
        let account_id_2 = AccountId::new();

        assert!(arrangement.link_account(account_id_1.clone()).is_ok());
        assert_eq!(arrangement.linked_accounts().len(), 1);

        assert!(arrangement.link_account(account_id_2.clone()).is_ok());
        assert_eq!(arrangement.linked_accounts().len(), 2);

        // Cannot link same account twice
        assert!(arrangement.link_account(account_id_1.clone()).is_err());

        // Unlink
        assert!(arrangement.unlink_account(&account_id_1).is_ok());
        assert_eq!(arrangement.linked_accounts().len(), 1);
    }

    #[test]
    fn test_arrangement_renewal_trigger() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let maturity = now + Duration::days(30);
        let terms = ArrangementTerms::new(
            "EUR",
            Some(2.5),
            None,
            None,
            None,
            RenewalType::Automatic,
            Some(10),
        ).unwrap();

        let arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::TermDeposit,
            now + Duration::days(1),
            Some(maturity),
            terms,
        ).unwrap();

        // Renewal should not trigger yet (too early)
        assert!(!arrangement.check_renewal_trigger());
    }

    #[test]
    fn test_arrangement_events() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let future = now + Duration::days(30);
        let terms = ArrangementTerms::new(
            "EUR",
            None,
            None,
            None,
            None,
            RenewalType::None,
            None,
        ).unwrap();

        let mut arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::Loan,
            future,
            None,
            terms,
        ).unwrap();

        // Should have creation event
        assert_eq!(arrangement.events().len(), 1);
        assert_eq!(
            arrangement.events()[0].event_type(),
            ArrangementEventType::Created
        );

        // Record interest application
        assert!(arrangement.record_interest_application(500).is_ok());
        assert_eq!(arrangement.events().len(), 2);

        // Record fee charge
        assert!(arrangement.record_fee_charge(25).is_ok());
        assert_eq!(arrangement.events().len(), 3);
    }

    #[test]
    fn test_arrangement_bundle_creation() {
        let arr_id_1 = ArrangementId::new();
        let arr_id_2 = ArrangementId::new();

        let bundle = ArrangementBundle::new(
            "Gold Package",
            vec![arr_id_1.clone(), arr_id_2.clone()],
            Some(5.0),
        );
        assert!(bundle.is_ok());
        let bundle = bundle.unwrap();
        assert_eq!(bundle.name(), "Gold Package");
        assert_eq!(bundle.arrangements().len(), 2);
        assert_eq!(bundle.discount_pct(), Some(5.0));
        assert!(bundle.is_active());
    }

    #[test]
    fn test_arrangement_bundle_empty() {
        let bundle = ArrangementBundle::new("Empty Bundle", vec![], None);
        assert!(bundle.is_err());
    }

    #[test]
    fn test_arrangement_bundle_manage_arrangements() {
        let arr_id_1 = ArrangementId::new();
        let arr_id_2 = ArrangementId::new();
        let arr_id_3 = ArrangementId::new();

        let mut bundle = ArrangementBundle::new(
            "Package",
            vec![arr_id_1.clone(), arr_id_2.clone()],
            None,
        ).unwrap();

        // Add arrangement
        assert!(bundle.add_arrangement(arr_id_3.clone()).is_ok());
        assert_eq!(bundle.arrangements().len(), 3);

        // Cannot add duplicate
        assert!(bundle.add_arrangement(arr_id_1.clone()).is_err());

        // Remove arrangement
        assert!(bundle.remove_arrangement(&arr_id_2).is_ok());
        assert_eq!(bundle.arrangements().len(), 2);

        // Cannot remove non-existent
        assert!(bundle.remove_arrangement(&arr_id_2).is_err());
    }

    #[test]
    fn test_arrangement_bundle_discount_validation() {
        let bundle = ArrangementBundle::new(
            "Bundle",
            vec![ArrangementId::new()],
            Some(150.0),
        );
        assert!(bundle.is_err());
    }

    #[test]
    fn test_arrangement_bundle_lifecycle() {
        let mut bundle = ArrangementBundle::new(
            "Package",
            vec![ArrangementId::new()],
            Some(10.0),
        ).unwrap();

        assert!(bundle.is_active());
        bundle.deactivate();
        assert!(!bundle.is_active());
        bundle.activate();
        assert!(bundle.is_active());
    }

    #[test]
    fn test_arrangement_event_creation() {
        let arr_id = ArrangementId::new();
        let event = ArrangementEvent::new(
            arr_id.clone(),
            ArrangementEventType::Created,
            "Test event",
        );
        assert!(event.is_ok());
        let event = event.unwrap();
        assert_eq!(event.event_type(), ArrangementEventType::Created);
        assert_eq!(event.event_data_json(), "Test event");
    }

    #[test]
    fn test_arrangement_interest_rate_bounds() {
        let terms = ArrangementTerms::new(
            "EUR",
            Some(0.0),
            None,
            None,
            None,
            RenewalType::None,
            None,
        );
        assert!(terms.is_ok());

        let terms = ArrangementTerms::new(
            "EUR",
            Some(100.0),
            None,
            None,
            None,
            RenewalType::None,
            None,
        );
        assert!(terms.is_ok());

        let terms = ArrangementTerms::new(
            "EUR",
            Some(-1.0),
            None,
            None,
            None,
            RenewalType::None,
            None,
        );
        assert!(terms.is_err());

        let terms = ArrangementTerms::new(
            "EUR",
            Some(101.0),
            None,
            None,
            None,
            RenewalType::None,
            None,
        );
        assert!(terms.is_err());
    }

    #[test]
    fn test_arrangement_status_transitions() {
        let customer_id = CustomerId::new();
        let now = Utc::now();
        let future = now + Duration::days(30);
        let terms = ArrangementTerms::new(
            "EUR",
            None,
            None,
            None,
            None,
            RenewalType::None,
            None,
        ).unwrap();

        let mut arrangement = Arrangement::new(
            customer_id,
            "PROD_001",
            ArrangementType::Deposit,
            future,
            None,
            terms,
        ).unwrap();

        // Proposed -> Active
        assert!(arrangement.activate().is_ok());
        assert_eq!(arrangement.status(), ArrangementStatus::Active);

        // Active -> Matured
        assert!(arrangement.mark_matured().is_ok());
        assert_eq!(arrangement.status(), ArrangementStatus::Matured);

        // Matured -> Closed
        assert!(arrangement.close(0).is_ok());
        assert_eq!(arrangement.status(), ArrangementStatus::Closed);
    }
}
