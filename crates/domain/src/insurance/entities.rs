use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{CustomerId, Money};

use super::value_objects::{
    BancassuranceProductId, ClaimStatus, CommissionStatus, InsuranceClaimId, InsuranceCommissionId,
    InsurancePolicyId, LinkedProductType, PolicyStatus, PolicyType, PremiumFrequency,
};

// --- InsurancePolicy aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePolicy {
    id: InsurancePolicyId,
    policy_type: PolicyType,
    customer_id: CustomerId,
    provider_name: String,
    policy_number: String,
    premium_amount: Money,
    premium_frequency: PremiumFrequency,
    coverage_amount: Money,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    beneficiaries: Vec<String>,
    status: PolicyStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InsurancePolicy {
    /// Create a new insurance policy. Enforces domain invariants.
    pub fn new(
        policy_type: PolicyType,
        customer_id: CustomerId,
        provider_name: &str,
        policy_number: &str,
        premium_amount: Money,
        premium_frequency: PremiumFrequency,
        coverage_amount: Money,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        beneficiaries: Vec<String>,
    ) -> Result<Self, DomainError> {
        // Invariant: premium amount must be positive
        if premium_amount.is_negative() || premium_amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Premium amount must be positive".to_string(),
            ));
        }

        // Invariant: coverage amount must be positive
        if coverage_amount.is_negative() || coverage_amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Coverage amount must be positive".to_string(),
            ));
        }

        // Invariant: end date must be after start date
        if end_date <= start_date {
            return Err(DomainError::ValidationError(
                "Policy end date must be after start date".to_string(),
            ));
        }

        // Invariant: beneficiaries required for life insurance
        if policy_type == PolicyType::Life && beneficiaries.is_empty() {
            return Err(DomainError::ValidationError(
                "Life insurance policies must have at least one beneficiary".to_string(),
            ));
        }

        // Invariant: provider name must not be empty
        if provider_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Provider name cannot be empty".to_string(),
            ));
        }

        // Invariant: policy number must not be empty
        if policy_number.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Policy number cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(InsurancePolicy {
            id: InsurancePolicyId::new(),
            policy_type,
            customer_id,
            provider_name: provider_name.to_string(),
            policy_number: policy_number.to_string(),
            premium_amount,
            premium_frequency,
            coverage_amount,
            start_date,
            end_date,
            beneficiaries,
            status: PolicyStatus::Proposal,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: InsurancePolicyId,
        policy_type: PolicyType,
        customer_id: CustomerId,
        provider_name: String,
        policy_number: String,
        premium_amount: Money,
        premium_frequency: PremiumFrequency,
        coverage_amount: Money,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        beneficiaries: Vec<String>,
        status: PolicyStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        InsurancePolicy {
            id,
            policy_type,
            customer_id,
            provider_name,
            policy_number,
            premium_amount,
            premium_frequency,
            coverage_amount,
            start_date,
            end_date,
            beneficiaries,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &InsurancePolicyId {
        &self.id
    }

    pub fn policy_type(&self) -> PolicyType {
        self.policy_type
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    pub fn policy_number(&self) -> &str {
        &self.policy_number
    }

    pub fn premium_amount(&self) -> &Money {
        &self.premium_amount
    }

    pub fn premium_frequency(&self) -> PremiumFrequency {
        self.premium_frequency
    }

    pub fn coverage_amount(&self) -> &Money {
        &self.coverage_amount
    }

    pub fn start_date(&self) -> DateTime<Utc> {
        self.start_date
    }

    pub fn end_date(&self) -> DateTime<Utc> {
        self.end_date
    }

    pub fn beneficiaries(&self) -> &[String] {
        &self.beneficiaries
    }

    pub fn status(&self) -> PolicyStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn activate(&mut self) -> Result<(), DomainError> {
        if self.status != PolicyStatus::Proposal {
            return Err(DomainError::ValidationError(
                "Can only activate a proposal policy".to_string(),
            ));
        }

        self.status = PolicyStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<(), DomainError> {
        if self.status != PolicyStatus::Active {
            return Err(DomainError::ValidationError(
                "Can only suspend an active policy".to_string(),
            ));
        }

        self.status = PolicyStatus::Suspended;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), DomainError> {
        if self.status != PolicyStatus::Suspended {
            return Err(DomainError::ValidationError(
                "Can only resume a suspended policy".to_string(),
            ));
        }

        self.status = PolicyStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn lapse(&mut self) -> Result<(), DomainError> {
        if self.status != PolicyStatus::Active && self.status != PolicyStatus::Suspended {
            return Err(DomainError::ValidationError(
                "Can only lapse an active or suspended policy".to_string(),
            ));
        }

        self.status = PolicyStatus::Lapsed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), DomainError> {
        if self.status == PolicyStatus::Terminated {
            return Err(DomainError::ValidationError(
                "Policy is already terminated".to_string(),
            ));
        }

        self.status = PolicyStatus::Terminated;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.end_date
    }
}

// --- InsuranceClaim entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceClaim {
    id: InsuranceClaimId,
    policy_id: InsurancePolicyId,
    claim_date: DateTime<Utc>,
    claim_amount: Money,
    description: String,
    documents: Vec<String>,
    status: ClaimStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InsuranceClaim {
    /// Create a new insurance claim. Enforces domain invariants.
    pub fn new(
        policy_id: InsurancePolicyId,
        claim_date: DateTime<Utc>,
        claim_amount: Money,
        description: &str,
        documents: Vec<String>,
    ) -> Result<Self, DomainError> {
        // Invariant: claim amount must be positive
        if claim_amount.is_negative() || claim_amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Claim amount must be positive".to_string(),
            ));
        }

        // Invariant: description must not be empty
        if description.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Claim description cannot be empty".to_string(),
            ));
        }

        // Invariant: documents must not be empty
        if documents.is_empty() {
            return Err(DomainError::ValidationError(
                "Claim must have at least one supporting document".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(InsuranceClaim {
            id: InsuranceClaimId::new(),
            policy_id,
            claim_date,
            claim_amount,
            description: description.to_string(),
            documents,
            status: ClaimStatus::Filed,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(
        id: InsuranceClaimId,
        policy_id: InsurancePolicyId,
        claim_date: DateTime<Utc>,
        claim_amount: Money,
        description: String,
        documents: Vec<String>,
        status: ClaimStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        InsuranceClaim {
            id,
            policy_id,
            claim_date,
            claim_amount,
            description,
            documents,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &InsuranceClaimId {
        &self.id
    }

    pub fn policy_id(&self) -> &InsurancePolicyId {
        &self.policy_id
    }

    pub fn claim_date(&self) -> DateTime<Utc> {
        self.claim_date
    }

    pub fn claim_amount(&self) -> &Money {
        &self.claim_amount
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn documents(&self) -> &[String] {
        &self.documents
    }

    pub fn status(&self) -> ClaimStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn submit_for_review(&mut self) -> Result<(), DomainError> {
        if self.status != ClaimStatus::Filed {
            return Err(DomainError::ValidationError(
                "Can only submit filed claims for review".to_string(),
            ));
        }

        self.status = ClaimStatus::UnderReview;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), DomainError> {
        if self.status != ClaimStatus::UnderReview {
            return Err(DomainError::ValidationError(
                "Can only approve claims under review".to_string(),
            ));
        }

        self.status = ClaimStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reject(&mut self) -> Result<(), DomainError> {
        if self.status != ClaimStatus::UnderReview {
            return Err(DomainError::ValidationError(
                "Can only reject claims under review".to_string(),
            ));
        }

        self.status = ClaimStatus::Rejected;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn pay(&mut self) -> Result<(), DomainError> {
        if self.status != ClaimStatus::Approved {
            return Err(DomainError::ValidationError(
                "Can only pay approved claims".to_string(),
            ));
        }

        self.status = ClaimStatus::Paid;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- BancassuranceProduct entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BancassuranceProduct {
    id: BancassuranceProductId,
    insurance_provider: String,
    product_name: String,
    product_type: PolicyType,
    commission_rate: f64,
    is_mandatory: bool,
    linked_product_types: Vec<LinkedProductType>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BancassuranceProduct {
    /// Create a new bancassurance product. Enforces domain invariants.
    pub fn new(
        insurance_provider: &str,
        product_name: &str,
        product_type: PolicyType,
        commission_rate: f64,
        is_mandatory: bool,
        linked_product_types: Vec<LinkedProductType>,
    ) -> Result<Self, DomainError> {
        // Invariant: provider name must not be empty
        if insurance_provider.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Insurance provider name cannot be empty".to_string(),
            ));
        }

        // Invariant: product name must not be empty
        if product_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Product name cannot be empty".to_string(),
            ));
        }

        // Invariant: commission rate must be between 0 and 100%
        if !(0.0..=1.0).contains(&commission_rate) {
            return Err(DomainError::ValidationError(
                "Commission rate must be between 0 and 1".to_string(),
            ));
        }

        // Invariant: mandatory products must have linked product types
        if is_mandatory && linked_product_types.is_empty() {
            return Err(DomainError::ValidationError(
                "Mandatory products must be linked to at least one product type".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(BancassuranceProduct {
            id: BancassuranceProductId::new(),
            insurance_provider: insurance_provider.to_string(),
            product_name: product_name.to_string(),
            product_type,
            commission_rate,
            is_mandatory,
            linked_product_types,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(
        id: BancassuranceProductId,
        insurance_provider: String,
        product_name: String,
        product_type: PolicyType,
        commission_rate: f64,
        is_mandatory: bool,
        linked_product_types: Vec<LinkedProductType>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        BancassuranceProduct {
            id,
            insurance_provider,
            product_name,
            product_type,
            commission_rate,
            is_mandatory,
            linked_product_types,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &BancassuranceProductId {
        &self.id
    }

    pub fn insurance_provider(&self) -> &str {
        &self.insurance_provider
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn product_type(&self) -> PolicyType {
        self.product_type
    }

    pub fn commission_rate(&self) -> f64 {
        self.commission_rate
    }

    pub fn is_mandatory(&self) -> bool {
        self.is_mandatory
    }

    pub fn linked_product_types(&self) -> &[LinkedProductType] {
        &self.linked_product_types
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- InsuranceCommission entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceCommission {
    id: InsuranceCommissionId,
    policy_id: InsurancePolicyId,
    product_id: BancassuranceProductId,
    amount: Money,
    calculation_date: DateTime<Utc>,
    status: CommissionStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InsuranceCommission {
    /// Create a new insurance commission. Enforces domain invariants.
    pub fn new(
        policy_id: InsurancePolicyId,
        product_id: BancassuranceProductId,
        amount: Money,
        calculation_date: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        // Invariant: amount must be positive
        if amount.is_negative() || amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Commission amount must be positive".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(InsuranceCommission {
            id: InsuranceCommissionId::new(),
            policy_id,
            product_id,
            amount,
            calculation_date,
            status: CommissionStatus::Pending,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(
        id: InsuranceCommissionId,
        policy_id: InsurancePolicyId,
        product_id: BancassuranceProductId,
        amount: Money,
        calculation_date: DateTime<Utc>,
        status: CommissionStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        InsuranceCommission {
            id,
            policy_id,
            product_id,
            amount,
            calculation_date,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &InsuranceCommissionId {
        &self.id
    }

    pub fn policy_id(&self) -> &InsurancePolicyId {
        &self.policy_id
    }

    pub fn product_id(&self) -> &BancassuranceProductId {
        &self.product_id
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn calculation_date(&self) -> DateTime<Utc> {
        self.calculation_date
    }

    pub fn status(&self) -> CommissionStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn mark_paid(&mut self) -> Result<(), DomainError> {
        if self.status != CommissionStatus::Pending {
            return Err(DomainError::ValidationError(
                "Can only pay pending commissions".to_string(),
            ));
        }

        self.status = CommissionStatus::Paid;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::Currency;

    #[test]
    fn test_insurance_policy_new() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let end = now + chrono::Duration::days(365);
        let premium = Money::from_cents(50000, Currency::try_from("TND").unwrap());
        let coverage = Money::from_cents(500000, Currency::try_from("TND").unwrap());

        let policy = InsurancePolicy::new(
            PolicyType::Life,
            customer,
            "Insurance Corp",
            "POL-2024-001",
            premium,
            PremiumFrequency::Monthly,
            coverage,
            now,
            end,
            vec!["Jane Doe".to_string()],
        );

        assert!(policy.is_ok());
        let p = policy.unwrap();
        assert_eq!(p.policy_type(), PolicyType::Life);
        assert_eq!(p.status(), PolicyStatus::Proposal);
    }

    #[test]
    fn test_insurance_policy_life_without_beneficiaries() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let end = now + chrono::Duration::days(365);
        let premium = Money::from_cents(50000, Currency::try_from("TND").unwrap());
        let coverage = Money::from_cents(500000, Currency::try_from("TND").unwrap());

        let policy = InsurancePolicy::new(
            PolicyType::Life,
            customer,
            "Insurance Corp",
            "POL-2024-002",
            premium,
            PremiumFrequency::Monthly,
            coverage,
            now,
            end,
            vec![],
        );

        assert!(policy.is_err());
    }

    #[test]
    fn test_insurance_policy_activate() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let end = now + chrono::Duration::days(365);
        let premium = Money::from_cents(50000, Currency::try_from("TND").unwrap());
        let coverage = Money::from_cents(500000, Currency::try_from("TND").unwrap());

        let mut policy = InsurancePolicy::new(
            PolicyType::Property,
            customer,
            "Insurance Corp",
            "POL-2024-003",
            premium,
            PremiumFrequency::Annual,
            coverage,
            now,
            end,
            vec![],
        )
        .unwrap();

        policy.activate().unwrap();
        assert_eq!(policy.status(), PolicyStatus::Active);
    }

    #[test]
    fn test_insurance_claim_new() {
        let policy_id = InsurancePolicyId::new();
        let now = Utc::now();
        let amount = Money::from_cents(100000, Currency::try_from("TND").unwrap());

        let claim = InsuranceClaim::new(
            policy_id,
            now,
            amount,
            "Car accident",
            vec!["Police Report".to_string(), "Photos".to_string()],
        );

        assert!(claim.is_ok());
        let c = claim.unwrap();
        assert_eq!(c.status(), ClaimStatus::Filed);
    }

    #[test]
    fn test_insurance_claim_workflow() {
        let policy_id = InsurancePolicyId::new();
        let now = Utc::now();
        let amount = Money::from_cents(50000, Currency::try_from("TND").unwrap());

        let mut claim = InsuranceClaim::new(
            policy_id,
            now,
            amount,
            "Medical claim",
            vec!["Hospital Receipt".to_string()],
        )
        .unwrap();

        claim.submit_for_review().unwrap();
        assert_eq!(claim.status(), ClaimStatus::UnderReview);

        claim.approve().unwrap();
        assert_eq!(claim.status(), ClaimStatus::Approved);

        claim.pay().unwrap();
        assert_eq!(claim.status(), ClaimStatus::Paid);
    }

    #[test]
    fn test_bancassurance_product_new() {
        let product = BancassuranceProduct::new(
            "Global Insurance",
            "Mortgage Protection",
            PolicyType::CreditInsurance,
            0.02,
            true,
            vec![LinkedProductType::Mortgage],
        );

        assert!(product.is_ok());
        let p = product.unwrap();
        assert!(p.is_mandatory());
    }

    #[test]
    fn test_bancassurance_product_invalid_commission_rate() {
        let product = BancassuranceProduct::new(
            "Insurance Co",
            "Product",
            PolicyType::Credit,
            1.5,
            false,
            vec![],
        );

        assert!(product.is_err());
    }

    #[test]
    fn test_insurance_commission_new() {
        let policy_id = InsurancePolicyId::new();
        let product_id = BancassuranceProductId::new();
        let amount = Money::from_cents(10000, Currency::try_from("TND").unwrap());

        let commission = InsuranceCommission::new(
            policy_id,
            product_id,
            amount,
            Utc::now(),
        );

        assert!(commission.is_ok());
        let c = commission.unwrap();
        assert_eq!(c.status(), CommissionStatus::Pending);
    }

    #[test]
    fn test_insurance_commission_mark_paid() {
        let policy_id = InsurancePolicyId::new();
        let product_id = BancassuranceProductId::new();
        let amount = Money::from_cents(10000, Currency::try_from("TND").unwrap());

        let mut commission =
            InsuranceCommission::new(policy_id, product_id, amount, Utc::now()).unwrap();

        commission.mark_paid().unwrap();
        assert_eq!(commission.status(), CommissionStatus::Paid);
    }
}
