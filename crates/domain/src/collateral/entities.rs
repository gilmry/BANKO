use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{CustomerId, Money};

use super::value_objects::{CollateralId, CollateralStatus, CollateralType, ValuationMethod};

// --- CollateralValuation entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollateralValuation {
    valuation_id: String,
    collateral_id: CollateralId,
    valuation_date: NaiveDate,
    market_value: Money,
    appraiser: String,
    method: ValuationMethod,
    next_revaluation_date: NaiveDate,
}

impl CollateralValuation {
    pub fn new(
        valuation_id: String,
        collateral_id: CollateralId,
        valuation_date: NaiveDate,
        market_value: Money,
        appraiser: String,
        method: ValuationMethod,
        collateral_type: CollateralType,
    ) -> Result<Self, DomainError> {
        if appraiser.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Appraiser name cannot be empty".to_string(),
            ));
        }

        let revaluation_frequency_months = collateral_type.revaluation_frequency_months();
        let next_revaluation_date = valuation_date
            + Duration::days((revaluation_frequency_months as i64) * 30);

        Ok(CollateralValuation {
            valuation_id,
            collateral_id,
            valuation_date,
            market_value,
            appraiser,
            method,
            next_revaluation_date,
        })
    }

    pub fn reconstitute(
        valuation_id: String,
        collateral_id: CollateralId,
        valuation_date: NaiveDate,
        market_value: Money,
        appraiser: String,
        method: ValuationMethod,
        next_revaluation_date: NaiveDate,
    ) -> Self {
        CollateralValuation {
            valuation_id,
            collateral_id,
            valuation_date,
            market_value,
            appraiser,
            method,
            next_revaluation_date,
        }
    }

    pub fn valuation_id(&self) -> &str {
        &self.valuation_id
    }

    pub fn collateral_id(&self) -> &CollateralId {
        &self.collateral_id
    }

    pub fn valuation_date(&self) -> NaiveDate {
        self.valuation_date
    }

    pub fn market_value(&self) -> &Money {
        &self.market_value
    }

    pub fn appraiser(&self) -> &str {
        &self.appraiser
    }

    pub fn method(&self) -> ValuationMethod {
        self.method
    }

    pub fn next_revaluation_date(&self) -> NaiveDate {
        self.next_revaluation_date
    }

    pub fn is_revaluation_due(&self, today: NaiveDate) -> bool {
        today >= self.next_revaluation_date
    }
}

// --- CollateralAllocation entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollateralAllocation {
    collateral_id: CollateralId,
    loan_id: String,
    allocated_amount: Money,
    allocation_date: DateTime<Utc>,
}

impl CollateralAllocation {
    pub fn new(
        collateral_id: CollateralId,
        loan_id: String,
        allocated_amount: Money,
        allocation_date: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if loan_id.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Loan ID cannot be empty".to_string(),
            ));
        }

        if allocated_amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Allocated amount must be positive".to_string(),
            ));
        }

        Ok(CollateralAllocation {
            collateral_id,
            loan_id,
            allocated_amount,
            allocation_date,
        })
    }

    pub fn collateral_id(&self) -> &CollateralId {
        &self.collateral_id
    }

    pub fn loan_id(&self) -> &str {
        &self.loan_id
    }

    pub fn allocated_amount(&self) -> &Money {
        &self.allocated_amount
    }

    pub fn allocation_date(&self) -> DateTime<Utc> {
        self.allocation_date
    }
}

// --- Collateral aggregate root ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collateral {
    id: CollateralId,
    collateral_type: CollateralType,
    description: String,
    market_value: Money,
    haircut_pct: f64,
    net_value: Money, // computed: market_value * (1 - haircut_pct)
    valuation_date: NaiveDate,
    next_revaluation_date: NaiveDate,
    status: CollateralStatus,
    customer_id: CustomerId,
    insurance_policy_id: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Collateral {
    /// Create a new collateral aggregate. Validates all business rules.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        collateral_type: CollateralType,
        description: String,
        market_value: Money,
        haircut_pct: f64,
        valuation_date: NaiveDate,
        customer_id: CustomerId,
        insurance_policy_id: Option<String>,
    ) -> Result<Self, DomainError> {
        // Validate description
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Description cannot be empty".to_string(),
            ));
        }

        if description.len() > 500 {
            return Err(DomainError::ValidationError(
                "Description cannot exceed 500 characters".to_string(),
            ));
        }

        // Validate market value
        if market_value.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Market value must be positive".to_string(),
            ));
        }

        // Validate haircut percentage
        if !(0.0..=1.0).contains(&haircut_pct) {
            return Err(DomainError::ValidationError(
                "Haircut percentage must be between 0 and 1".to_string(),
            ));
        }

        // Validate insurance requirement for real estate
        if collateral_type == CollateralType::RealEstate && insurance_policy_id.is_none() {
            return Err(DomainError::ValidationError(
                "Insurance policy ID is mandatory for real estate collateral".to_string(),
            ));
        }

        // Calculate net value
        let _factor = 10_f64.powi(market_value.currency().decimal_places() as i32);
        let net_value_cents =
            (market_value.amount_cents() as f64 * (1.0 - haircut_pct)).ceil() as i64;
        let net_value = Money::from_cents(net_value_cents, market_value.currency());

        // Calculate next revaluation date
        let revaluation_frequency_months = collateral_type.revaluation_frequency_months();
        let next_revaluation_date = valuation_date
            + chrono::Duration::days((revaluation_frequency_months as i64) * 30);

        let now = Utc::now();

        Ok(Collateral {
            id: CollateralId::new(),
            collateral_type,
            description,
            market_value,
            haircut_pct,
            net_value,
            valuation_date,
            next_revaluation_date,
            status: CollateralStatus::Pending,
            customer_id,
            insurance_policy_id,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: CollateralId,
        collateral_type: CollateralType,
        description: String,
        market_value: Money,
        haircut_pct: f64,
        net_value: Money,
        valuation_date: NaiveDate,
        next_revaluation_date: NaiveDate,
        status: CollateralStatus,
        customer_id: CustomerId,
        insurance_policy_id: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Collateral {
            id,
            collateral_type,
            description,
            market_value,
            haircut_pct,
            net_value,
            valuation_date,
            next_revaluation_date,
            status,
            customer_id,
            insurance_policy_id,
            created_at,
            updated_at,
        }
    }

    // --- Accessors ---

    pub fn id(&self) -> &CollateralId {
        &self.id
    }

    pub fn collateral_type(&self) -> CollateralType {
        self.collateral_type
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn market_value(&self) -> &Money {
        &self.market_value
    }

    pub fn haircut_pct(&self) -> f64 {
        self.haircut_pct
    }

    pub fn net_value(&self) -> &Money {
        &self.net_value
    }

    pub fn valuation_date(&self) -> NaiveDate {
        self.valuation_date
    }

    pub fn next_revaluation_date(&self) -> NaiveDate {
        self.next_revaluation_date
    }

    pub fn status(&self) -> CollateralStatus {
        self.status
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn insurance_policy_id(&self) -> Option<&str> {
        self.insurance_policy_id.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Business operations ---

    /// Activate the collateral (transition from Pending to Active).
    pub fn activate(&mut self) -> Result<(), DomainError> {
        if self.status != CollateralStatus::Pending {
            return Err(DomainError::InvalidCollateralStatus(format!(
                "Cannot activate collateral in {} status",
                self.status
            )));
        }
        self.status = CollateralStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Release the collateral (transition to Released).
    pub fn release(&mut self) -> Result<(), DomainError> {
        if self.status == CollateralStatus::Released {
            return Err(DomainError::ValidationError(
                "Collateral is already released".to_string(),
            ));
        }
        self.status = CollateralStatus::Released;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark collateral as impaired (value has deteriorated).
    pub fn mark_impaired(&mut self) -> Result<(), DomainError> {
        if self.status == CollateralStatus::Released {
            return Err(DomainError::ValidationError(
                "Cannot mark released collateral as impaired".to_string(),
            ));
        }
        self.status = CollateralStatus::Impaired;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update collateral valuation after reappraisal.
    pub fn revalue(
        &mut self,
        new_market_value: Money,
        haircut_pct: f64,
        valuation_date: NaiveDate,
    ) -> Result<(), DomainError> {
        // Validate inputs
        if new_market_value.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Market value must be positive".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&haircut_pct) {
            return Err(DomainError::ValidationError(
                "Haircut percentage must be between 0 and 1".to_string(),
            ));
        }

        // Check currency consistency
        if new_market_value.currency() != self.market_value.currency() {
            return Err(DomainError::ValidationError(
                "New valuation must use the same currency".to_string(),
            ));
        }

        // Update values
        self.market_value = new_market_value;
        self.haircut_pct = haircut_pct;
        self.valuation_date = valuation_date;

        // Recalculate net value
        let net_value_cents =
            (self.market_value.amount_cents() as f64 * (1.0 - haircut_pct)).ceil() as i64;
        self.net_value =
            Money::from_cents(net_value_cents, self.market_value.currency());

        // Calculate next revaluation date
        let revaluation_frequency_months = self.collateral_type.revaluation_frequency_months();
        self.next_revaluation_date = valuation_date
            + chrono::Duration::days((revaluation_frequency_months as i64) * 30);

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if collateral is due for revaluation.
    pub fn is_revaluation_due(&self, today: NaiveDate) -> bool {
        today >= self.next_revaluation_date
    }

    /// Update insurance policy (for real estate).
    pub fn update_insurance_policy(&mut self, policy_id: String) -> Result<(), DomainError> {
        if self.collateral_type != CollateralType::RealEstate {
            return Err(DomainError::ValidationError(
                "Insurance policy can only be set for real estate collateral".to_string(),
            ));
        }

        if policy_id.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Insurance policy ID cannot be empty".to_string(),
            ));
        }

        self.insurance_policy_id = Some(policy_id);
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- LtvCalculation entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LtvCalculation {
    loan_id: String,
    collateral_ids: Vec<CollateralId>,
    total_loan_amount: Money,
    total_collateral_value: Money,
    ltv_ratio: f64,
    is_compliant: bool,
    max_ltv_threshold: f64,
}

impl LtvCalculation {
    /// Calculate LTV for a loan with given collateral.
    /// Enforces BCT Circ. 91-24 LTV limits per collateral type.
    pub fn calculate(
        loan_id: String,
        collateral_ids: Vec<CollateralId>,
        total_loan_amount: Money,
        total_collateral_value: Money,
        collateral_types: &[CollateralType],
    ) -> Result<Self, DomainError> {
        if loan_id.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Loan ID cannot be empty".to_string(),
            ));
        }

        if collateral_ids.is_empty() {
            return Err(DomainError::ValidationError(
                "At least one collateral is required".to_string(),
            ));
        }

        if collateral_types.len() != collateral_ids.len() {
            return Err(DomainError::ValidationError(
                "Collateral types and IDs count mismatch".to_string(),
            ));
        }

        if total_loan_amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Loan amount must be positive".to_string(),
            ));
        }

        if total_collateral_value.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Collateral value must be positive".to_string(),
            ));
        }

        // Calculate LTV ratio
        let ltv_ratio = total_loan_amount.amount_cents() as f64
            / total_collateral_value.amount_cents() as f64;

        // Determine the most restrictive LTV threshold from collateral types
        let max_ltv_threshold = collateral_types
            .iter()
            .map(|ct| ct.max_ltv_pct())
            .fold(f64::INFINITY, f64::min);

        // Check if LTV is compliant
        let is_compliant = ltv_ratio <= max_ltv_threshold;

        Ok(LtvCalculation {
            loan_id,
            collateral_ids,
            total_loan_amount,
            total_collateral_value,
            ltv_ratio,
            is_compliant,
            max_ltv_threshold,
        })
    }

    pub fn loan_id(&self) -> &str {
        &self.loan_id
    }

    pub fn collateral_ids(&self) -> &[CollateralId] {
        &self.collateral_ids
    }

    pub fn total_loan_amount(&self) -> &Money {
        &self.total_loan_amount
    }

    pub fn total_collateral_value(&self) -> &Money {
        &self.total_collateral_value
    }

    pub fn ltv_ratio(&self) -> f64 {
        self.ltv_ratio
    }

    pub fn is_compliant(&self) -> bool {
        self.is_compliant
    }

    pub fn max_ltv_threshold(&self) -> f64 {
        self.max_ltv_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::Currency;

    #[test]
    fn test_collateral_creation() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let collateral = Collateral::new(
            CollateralType::RealEstate,
            "Residential property".to_string(),
            market_value,
            0.0,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            Some("POL-001".to_string()),
        );

        assert!(collateral.is_ok());
        let c = collateral.unwrap();
        assert_eq!(c.collateral_type(), CollateralType::RealEstate);
        assert_eq!(c.status(), CollateralStatus::Pending);
        assert!(c.insurance_policy_id().is_some());
    }

    #[test]
    fn test_collateral_real_estate_requires_insurance() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let collateral = Collateral::new(
            CollateralType::RealEstate,
            "Residential property".to_string(),
            market_value,
            0.0,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None, // No insurance
        );

        assert!(collateral.is_err());
    }

    #[test]
    fn test_collateral_negative_market_value() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(-50_000.0, Currency::TND).unwrap();

        let collateral = Collateral::new(
            CollateralType::Securities,
            "Stock portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        );

        assert!(collateral.is_err());
    }

    #[test]
    fn test_collateral_invalid_haircut() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(50_000.0, Currency::TND).unwrap();

        let collateral = Collateral::new(
            CollateralType::Securities,
            "Stock portfolio".to_string(),
            market_value,
            1.5, // > 1.0
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        );

        assert!(collateral.is_err());
    }

    #[test]
    fn test_collateral_net_value_calculation() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.35, // 35% haircut
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        );

        let c = collateral.unwrap();
        let expected_net_value = 65_000.0;
        let actual_net_value = c.net_value().amount();
        assert!((actual_net_value - expected_net_value).abs() < 0.01);
    }

    #[test]
    fn test_collateral_activate() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(50_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        assert_eq!(collateral.status(), CollateralStatus::Pending);
        collateral.activate().unwrap();
        assert_eq!(collateral.status(), CollateralStatus::Active);
    }

    #[test]
    fn test_collateral_activate_already_active() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(50_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        collateral.activate().unwrap();
        let result = collateral.activate();
        assert!(result.is_err());
    }

    #[test]
    fn test_collateral_release() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(50_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        collateral.activate().unwrap();
        collateral.release().unwrap();
        assert_eq!(collateral.status(), CollateralStatus::Released);
    }

    #[test]
    fn test_collateral_mark_impaired() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(50_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        collateral.activate().unwrap();
        collateral.mark_impaired().unwrap();
        assert_eq!(collateral.status(), CollateralStatus::Impaired);
    }

    #[test]
    fn test_collateral_revalue() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        let new_market_value = Money::new(95_000.0, Currency::TND).unwrap();
        collateral
            .revalue(new_market_value, 0.25, NaiveDate::from_ymd_opt(2024, 4, 1).unwrap())
            .unwrap();

        assert_eq!(collateral.market_value().amount(), 95_000.0);
        assert_eq!(collateral.haircut_pct(), 0.25);
    }

    #[test]
    fn test_collateral_revalue_invalid_currency() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        let new_market_value = Money::new(95_000.0, Currency::EUR).unwrap();
        let result = collateral.revalue(
            new_market_value,
            0.25,
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_collateral_is_revaluation_due() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let collateral = Collateral::new(
            CollateralType::RealEstate,
            "Residential property".to_string(),
            market_value,
            0.0,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            Some("POL-001".to_string()),
        )
        .unwrap();

        // For real estate, revaluation is due after 360 days (12 months * 30 days)
        // next_revaluation_date = 2024-01-01 + 360 days = 2024-12-26
        let after_revaluation = NaiveDate::from_ymd_opt(2024, 12, 27).unwrap();
        assert!(collateral.is_revaluation_due(after_revaluation));

        let before_revaluation = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert!(!collateral.is_revaluation_due(before_revaluation));
    }

    #[test]
    fn test_collateral_allocation_new() {
        let collateral_id = CollateralId::new();
        let amount = Money::new(50_000.0, Currency::TND).unwrap();

        let allocation = CollateralAllocation::new(
            collateral_id,
            "LOAN-001".to_string(),
            amount,
            Utc::now(),
        );

        assert!(allocation.is_ok());
        let a = allocation.unwrap();
        assert_eq!(a.loan_id(), "LOAN-001");
    }

    #[test]
    fn test_collateral_allocation_invalid_loan_id() {
        let collateral_id = CollateralId::new();
        let amount = Money::new(50_000.0, Currency::TND).unwrap();

        let allocation = CollateralAllocation::new(
            collateral_id,
            "".to_string(),
            amount,
            Utc::now(),
        );

        assert!(allocation.is_err());
    }

    #[test]
    fn test_collateral_allocation_negative_amount() {
        let collateral_id = CollateralId::new();
        let amount = Money::new(-10_000.0, Currency::TND).unwrap();

        let allocation = CollateralAllocation::new(
            collateral_id,
            "LOAN-001".to_string(),
            amount,
            Utc::now(),
        );

        assert!(allocation.is_err());
    }

    #[test]
    fn test_ltv_calculation_compliant() {
        let loan_amount = Money::new(70_000.0, Currency::TND).unwrap();
        let collateral_value = Money::new(100_000.0, Currency::TND).unwrap();

        let ltv = LtvCalculation::calculate(
            "LOAN-001".to_string(),
            vec![CollateralId::new()],
            loan_amount,
            collateral_value,
            &[CollateralType::RealEstate],
        );

        assert!(ltv.is_ok());
        let l = ltv.unwrap();
        assert_eq!(l.ltv_ratio(), 0.70);
        assert!(l.is_compliant()); // 70% is at the max for real estate
    }

    #[test]
    fn test_ltv_calculation_non_compliant() {
        let loan_amount = Money::new(80_000.0, Currency::TND).unwrap();
        let collateral_value = Money::new(100_000.0, Currency::TND).unwrap();

        let ltv = LtvCalculation::calculate(
            "LOAN-001".to_string(),
            vec![CollateralId::new()],
            loan_amount,
            collateral_value,
            &[CollateralType::RealEstate],
        );

        assert!(ltv.is_ok());
        let l = ltv.unwrap();
        assert_eq!(l.ltv_ratio(), 0.80);
        assert!(!l.is_compliant()); // 80% exceeds 70% max for real estate
    }

    #[test]
    fn test_ltv_calculation_mixed_collateral() {
        let loan_amount = Money::new(75_000.0, Currency::TND).unwrap();
        let collateral_value = Money::new(100_000.0, Currency::TND).unwrap();

        let ltv = LtvCalculation::calculate(
            "LOAN-001".to_string(),
            vec![CollateralId::new(), CollateralId::new()],
            loan_amount,
            collateral_value,
            &[CollateralType::RealEstate, CollateralType::Securities],
        );

        assert!(ltv.is_ok());
        let l = ltv.unwrap();
        // Most restrictive is RealEstate (70%)
        assert_eq!(l.max_ltv_threshold(), 0.70);
        assert!(!l.is_compliant()); // 75% exceeds 70%
    }

    #[test]
    fn test_ltv_calculation_financial_deposit() {
        let loan_amount = Money::new(90_000.0, Currency::TND).unwrap();
        let collateral_value = Money::new(100_000.0, Currency::TND).unwrap();

        let ltv = LtvCalculation::calculate(
            "LOAN-001".to_string(),
            vec![CollateralId::new()],
            loan_amount,
            collateral_value,
            &[CollateralType::FinancialDeposit],
        );

        assert!(ltv.is_ok());
        let l = ltv.unwrap();
        assert_eq!(l.ltv_ratio(), 0.90);
        assert!(l.is_compliant()); // 90% < 95% max for deposits
    }

    #[test]
    fn test_collateral_valuation_new() {
        let collateral_id = CollateralId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let valuation = CollateralValuation::new(
            "VAL-001".to_string(),
            collateral_id,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            market_value,
            "John Appraiser".to_string(),
            ValuationMethod::MarketComparison,
            CollateralType::RealEstate,
        );

        assert!(valuation.is_ok());
        let v = valuation.unwrap();
        assert_eq!(v.appraiser(), "John Appraiser");
    }

    #[test]
    fn test_collateral_valuation_empty_appraiser() {
        let collateral_id = CollateralId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let valuation = CollateralValuation::new(
            "VAL-001".to_string(),
            collateral_id,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            market_value,
            "".to_string(),
            ValuationMethod::MarketComparison,
            CollateralType::RealEstate,
        );

        assert!(valuation.is_err());
    }

    #[test]
    fn test_collateral_valuation_revaluation_due() {
        let collateral_id = CollateralId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let valuation = CollateralValuation::new(
            "VAL-001".to_string(),
            collateral_id,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            market_value,
            "John Appraiser".to_string(),
            ValuationMethod::MarketComparison,
            CollateralType::RealEstate,
        )
        .unwrap();

        // Next revaluation is 360 days later (12 months * 30 days)
        // next_revaluation_date = 2024-01-01 + 360 days = 2024-12-26
        let after_revaluation = NaiveDate::from_ymd_opt(2024, 12, 27).unwrap();
        assert!(valuation.is_revaluation_due(after_revaluation));

        let before_revaluation = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert!(!valuation.is_revaluation_due(before_revaluation));
    }

    #[test]
    fn test_collateral_update_insurance_policy() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(100_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::RealEstate,
            "Residential property".to_string(),
            market_value,
            0.0,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            Some("POL-001".to_string()),
        )
        .unwrap();

        collateral
            .update_insurance_policy("POL-002".to_string())
            .unwrap();
        assert_eq!(collateral.insurance_policy_id(), Some("POL-002"));
    }

    #[test]
    fn test_collateral_update_insurance_non_real_estate() {
        let customer_id = CustomerId::new();
        let market_value = Money::new(50_000.0, Currency::TND).unwrap();

        let mut collateral = Collateral::new(
            CollateralType::Securities,
            "Bond portfolio".to_string(),
            market_value,
            0.2,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            customer_id,
            None,
        )
        .unwrap();

        let result = collateral.update_insurance_policy("POL-001".to_string());
        assert!(result.is_err());
    }
}
