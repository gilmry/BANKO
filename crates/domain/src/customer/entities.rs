use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::CustomerId;

use super::value_objects::{
    Beneficiary, ConsentStatus, CustomerStatus, CustomerType, KycProfile, PepStatus, RiskScore,
};

/// Customer aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    id: CustomerId,
    customer_type: CustomerType,
    kyc_profile: KycProfile,
    beneficiaries: Vec<Beneficiary>,
    risk_score: RiskScore,
    status: CustomerStatus,
    consent: ConsentStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Customer {
    /// Create a new Customer aggregate. Enforces all domain invariants.
    pub fn new(
        customer_type: CustomerType,
        kyc_profile: KycProfile,
        beneficiaries: Vec<Beneficiary>,
        consent: ConsentStatus,
    ) -> Result<Self, DomainError> {
        // INV-13: INPDP consent required
        if consent == ConsentStatus::NotGiven {
            return Err(DomainError::ConsentRequired);
        }

        // LegalEntity must have at least 1 beneficial owner
        if customer_type == CustomerType::LegalEntity && beneficiaries.is_empty() {
            return Err(DomainError::MissingBeneficiaries);
        }

        // Beneficiary shares must not exceed 100%
        if !beneficiaries.is_empty() {
            let total_share: f64 = beneficiaries.iter().map(|b| b.share_percentage()).sum();
            if total_share > 100.0 {
                return Err(DomainError::ValidationError(format!(
                    "Total beneficiary shares exceed 100%: {total_share}"
                )));
            }
        }

        let now = Utc::now();
        Ok(Customer {
            id: CustomerId::new(),
            customer_type,
            kyc_profile,
            beneficiaries,
            risk_score: RiskScore::new(0).unwrap(),
            status: CustomerStatus::Pending,
            consent,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: CustomerId,
        customer_type: CustomerType,
        kyc_profile: KycProfile,
        beneficiaries: Vec<Beneficiary>,
        risk_score: RiskScore,
        status: CustomerStatus,
        consent: ConsentStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Customer {
            id,
            customer_type,
            kyc_profile,
            beneficiaries,
            risk_score,
            status,
            consent,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &CustomerId {
        &self.id
    }

    pub fn customer_type(&self) -> CustomerType {
        self.customer_type
    }

    pub fn kyc_profile(&self) -> &KycProfile {
        &self.kyc_profile
    }

    pub fn beneficiaries(&self) -> &[Beneficiary] {
        &self.beneficiaries
    }

    pub fn risk_score(&self) -> &RiskScore {
        &self.risk_score
    }

    pub fn status(&self) -> CustomerStatus {
        self.status
    }

    pub fn consent(&self) -> ConsentStatus {
        self.consent
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// INV-01: KYC is validated only if status is Approved.
    pub fn is_kyc_validated(&self) -> bool {
        self.status == CustomerStatus::Approved
    }

    /// Approve the KYC profile.
    pub fn approve_kyc(&mut self) {
        self.status = CustomerStatus::Approved;
        self.updated_at = Utc::now();
    }

    /// Reject the KYC profile with a reason.
    pub fn reject_kyc(&mut self, reason: String) {
        self.status = CustomerStatus::Rejected;
        self.updated_at = Utc::now();
        // Store rejection reason — in a full implementation this would be
        // written through a domain event or stored in the KYC profile.
        let _ = reason;
    }

    /// Update the KYC profile.
    pub fn update_kyc(&mut self, profile: KycProfile) {
        self.kyc_profile = profile;
        self.updated_at = Utc::now();
    }

    /// Update the risk score.
    pub fn update_risk_score(&mut self, score: RiskScore) {
        self.risk_score = score;
        self.updated_at = Utc::now();
    }

    /// Suspend the customer.
    pub fn suspend(&mut self) {
        self.status = CustomerStatus::Suspended;
        self.updated_at = Utc::now();
    }

    /// Check if PEP.
    pub fn is_pep(&self) -> bool {
        self.kyc_profile.pep_status() == PepStatus::Yes
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::shared::value_objects::{EmailAddress, PhoneNumber};

    use super::super::value_objects::{Address, Cin, PepStatus, SourceOfFunds};
    use super::*;

    fn valid_address() -> Address {
        Address::new("10 Rue de la Liberte", "Tunis", "1000", "Tunisia").unwrap()
    }

    fn valid_phone() -> PhoneNumber {
        PhoneNumber::new("+21698123456").unwrap()
    }

    fn valid_email() -> EmailAddress {
        EmailAddress::new("ahmed@example.com").unwrap()
    }

    fn valid_individual_kyc() -> KycProfile {
        KycProfile::new_individual(
            "Ahmed Ben Ayed",
            Cin::new("12345678").unwrap(),
            NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            "Tunisia",
            "Banker",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Salary,
        )
        .unwrap()
    }

    fn valid_legal_entity_kyc() -> KycProfile {
        KycProfile::new_legal_entity(
            "Banko SA",
            "RCS-12345",
            "Banking",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Business,
        )
        .unwrap()
    }

    fn valid_beneficiary() -> Beneficiary {
        Beneficiary::new("Ahmed Ben Ayed", 50.0).unwrap()
    }

    // --- Customer::new tests ---

    #[test]
    fn test_customer_new_individual_success() {
        let customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        );
        assert!(customer.is_ok());
        let c = customer.unwrap();
        assert_eq!(c.customer_type(), CustomerType::Individual);
        assert_eq!(c.status(), CustomerStatus::Pending);
        assert!(!c.is_kyc_validated());
        assert_eq!(c.risk_score().value(), 0);
        assert_eq!(c.consent(), ConsentStatus::Given);
    }

    #[test]
    fn test_customer_new_legal_entity_success() {
        let customer = Customer::new(
            CustomerType::LegalEntity,
            valid_legal_entity_kyc(),
            vec![valid_beneficiary()],
            ConsentStatus::Given,
        );
        assert!(customer.is_ok());
        let c = customer.unwrap();
        assert_eq!(c.customer_type(), CustomerType::LegalEntity);
        assert_eq!(c.beneficiaries().len(), 1);
    }

    #[test]
    fn test_customer_inv13_consent_required() {
        let result = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::NotGiven,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::ConsentRequired);
    }

    #[test]
    fn test_customer_legal_entity_missing_beneficiaries() {
        let result = Customer::new(
            CustomerType::LegalEntity,
            valid_legal_entity_kyc(),
            vec![], // no beneficiaries
            ConsentStatus::Given,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::MissingBeneficiaries);
    }

    #[test]
    fn test_customer_beneficiary_shares_exceed_100() {
        let b1 = Beneficiary::new("A", 60.0).unwrap();
        let b2 = Beneficiary::new("B", 50.0).unwrap();
        let result = Customer::new(
            CustomerType::LegalEntity,
            valid_legal_entity_kyc(),
            vec![b1, b2],
            ConsentStatus::Given,
        );
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::ValidationError(msg) => {
                assert!(msg.contains("exceed 100%"));
            }
            e => panic!("Expected ValidationError, got {:?}", e),
        }
    }

    // --- Domain behavior tests ---

    #[test]
    fn test_customer_approve_kyc() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        )
        .unwrap();
        assert!(!customer.is_kyc_validated());

        customer.approve_kyc();
        assert!(customer.is_kyc_validated());
        assert_eq!(customer.status(), CustomerStatus::Approved);
    }

    #[test]
    fn test_customer_reject_kyc() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        )
        .unwrap();

        customer.reject_kyc("Missing documents".to_string());
        assert_eq!(customer.status(), CustomerStatus::Rejected);
        assert!(!customer.is_kyc_validated());
    }

    #[test]
    fn test_customer_suspend() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        )
        .unwrap();

        customer.approve_kyc();
        customer.suspend();
        assert_eq!(customer.status(), CustomerStatus::Suspended);
        assert!(!customer.is_kyc_validated());
    }

    #[test]
    fn test_customer_update_risk_score() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        )
        .unwrap();
        assert_eq!(customer.risk_score().value(), 0);

        let new_score = RiskScore::new(45).unwrap();
        customer.update_risk_score(new_score);
        assert_eq!(customer.risk_score().value(), 45);
    }

    #[test]
    fn test_customer_update_kyc() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        )
        .unwrap();

        let new_kyc = KycProfile::new_individual(
            "Ahmed Updated",
            Cin::new("87654321").unwrap(),
            NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            "Tunisia",
            "Manager",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Business,
        )
        .unwrap();

        customer.update_kyc(new_kyc);
        assert_eq!(customer.kyc_profile().full_name(), "Ahmed Updated");
    }

    #[test]
    fn test_customer_is_pep() {
        let kyc = KycProfile::new_individual(
            "PEP Person",
            Cin::new("12345678").unwrap(),
            NaiveDate::from_ymd_opt(1980, 5, 1).unwrap(),
            "Tunisia",
            "Minister",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::Yes,
            SourceOfFunds::Other,
        )
        .unwrap();

        let customer = Customer::new(
            CustomerType::Individual,
            kyc,
            vec![],
            ConsentStatus::Given,
        )
        .unwrap();

        assert!(customer.is_pep());
    }

    #[test]
    fn test_customer_individual_can_have_no_beneficiaries() {
        let customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
        );
        assert!(customer.is_ok());
    }

    #[test]
    fn test_customer_reconstitute() {
        let id = CustomerId::new();
        let now = Utc::now();
        let customer = Customer::reconstitute(
            id.clone(),
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            RiskScore::new(30).unwrap(),
            CustomerStatus::Approved,
            ConsentStatus::Given,
            now,
            now,
        );
        assert_eq!(customer.id(), &id);
        assert!(customer.is_kyc_validated());
        assert_eq!(customer.risk_score().value(), 30);
    }
}
