use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::CustomerId;

use super::value_objects::{
    Beneficiary, ConsentStatus, CustomerSegment, CustomerStatus, CustomerType, Document,
    KycProfile, PepStatus, RiskScore,
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
    segment: CustomerSegment,  // FR-006: Customer segmentation
    documents: Vec<Document>,  // FR-008: Document lifecycle management
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
}

impl Customer {
    /// Create a new Customer aggregate. Enforces all domain invariants.
    pub fn new(
        customer_type: CustomerType,
        kyc_profile: KycProfile,
        beneficiaries: Vec<Beneficiary>,
        consent: ConsentStatus,
        segment: CustomerSegment,  // FR-006
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
            segment,  // FR-006
            documents: vec![],  // FR-008
            created_at: now,
            updated_at: now,
            closed_at: None,
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
        segment: CustomerSegment,  // FR-006
        documents: Vec<Document>,  // FR-008
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        closed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Customer {
            id,
            customer_type,
            kyc_profile,
            beneficiaries,
            risk_score,
            status,
            consent,
            segment,  // FR-006
            documents,  // FR-008
            created_at,
            updated_at,
            closed_at,
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

    pub fn closed_at(&self) -> Option<DateTime<Utc>> {
        self.closed_at
    }

    pub fn segment(&self) -> CustomerSegment {
        self.segment
    }

    pub fn documents(&self) -> &[Document] {
        &self.documents
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

    /// Close the customer relationship. Sets status to Closed and records the closure timestamp.
    pub fn close(&mut self) -> Result<(), DomainError> {
        if self.status == CustomerStatus::Closed {
            return Err(DomainError::ValidationError(
                "Customer is already closed".to_string(),
            ));
        }
        if self.status == CustomerStatus::Anonymized {
            return Err(DomainError::CustomerAlreadyAnonymized);
        }
        let now = Utc::now();
        self.status = CustomerStatus::Closed;
        self.closed_at = Some(now);
        self.updated_at = now;
        Ok(())
    }

    /// INV-10: Anonymize personal data after 10-year retention period.
    /// Replaces KYC personal data with "[ANONYMIZED]".
    /// Only allowed if customer is Closed and closed_at is > 10 years ago.
    pub fn anonymize(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        if self.status == CustomerStatus::Anonymized {
            return Err(DomainError::CustomerAlreadyAnonymized);
        }
        if self.status != CustomerStatus::Closed {
            return Err(DomainError::CustomerNotClosed);
        }
        let closed_at = self.closed_at.ok_or(DomainError::CustomerNotClosed)?;

        const RETENTION_YEARS: u32 = 10;
        let years_since_closure = (now - closed_at).num_days() as f64 / 365.25;
        if years_since_closure < RETENTION_YEARS as f64 {
            return Err(DomainError::RetentionPeriodNotMet {
                closed_at: closed_at.to_rfc3339(),
                minimum_years: RETENTION_YEARS,
            });
        }

        self.kyc_profile.anonymize();
        self.beneficiaries.clear();
        self.status = CustomerStatus::Anonymized;
        self.updated_at = now;
        Ok(())
    }

    /// Check if this customer has been anonymized.
    pub fn is_anonymized(&self) -> bool {
        self.status == CustomerStatus::Anonymized
    }

    // --- Document Management (FR-008) ---

    /// Add a document to the customer's profile.
    pub fn add_document(&mut self, document: Document) -> Result<(), DomainError> {
        self.documents.push(document);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a document by ID.
    pub fn remove_document(&mut self, document_id: &std::uuid::Uuid) -> Result<(), DomainError> {
        let initial_len = self.documents.len();
        self.documents.retain(|d| d.id() != document_id);
        if self.documents.len() == initial_len {
            return Err(DomainError::DocumentNotFound);
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get expired documents.
    pub fn expired_documents(&self, now: chrono::NaiveDate) -> Vec<&Document> {
        self.documents.iter().filter(|d| d.is_expired(now)).collect()
    }

    /// Get documents expiring within N days (for renewal alerts).
    pub fn documents_expiring_soon(&self, now: chrono::NaiveDate, days: i64) -> Vec<&Document> {
        self.documents
            .iter()
            .filter(|d| d.expires_within_days(now, days))
            .collect()
    }

    /// Update customer segment (FR-006).
    pub fn update_segment(&mut self, segment: CustomerSegment) {
        self.segment = segment;
        self.updated_at = Utc::now();
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
            CustomerSegment::Retail,  // FR-006
        );
        assert!(customer.is_ok());
        let c = customer.unwrap();
        assert_eq!(c.customer_type(), CustomerType::Individual);
        assert_eq!(c.status(), CustomerStatus::Pending);
        assert!(!c.is_kyc_validated());
        assert_eq!(c.risk_score().value(), 0);
        assert_eq!(c.consent(), ConsentStatus::Given);
        assert_eq!(c.segment(), CustomerSegment::Retail);  // FR-006
    }

    #[test]
    fn test_customer_new_legal_entity_success() {
        let customer = Customer::new(
            CustomerType::LegalEntity,
            valid_legal_entity_kyc(),
            vec![valid_beneficiary()],
            ConsentStatus::Given,
            CustomerSegment::Corporate,  // FR-006
        );
        assert!(customer.is_ok());
        let c = customer.unwrap();
        assert_eq!(c.customer_type(), CustomerType::LegalEntity);
        assert_eq!(c.beneficiaries().len(), 1);
        assert_eq!(c.segment(), CustomerSegment::Corporate);  // FR-006
    }

    #[test]
    fn test_customer_inv13_consent_required() {
        let result = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::NotGiven,
            CustomerSegment::Retail,  // FR-006
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
            CustomerSegment::Corporate,  // FR-006
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
            CustomerSegment::Corporate,  // FR-006
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
            CustomerSegment::Retail,  // FR-006
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
            CustomerSegment::Retail,  // FR-006
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
            CustomerSegment::Retail,  // FR-006
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
            CustomerSegment::Retail,  // FR-006
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
            CustomerSegment::Retail,  // FR-006
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

        let customer =
            Customer::new(CustomerType::Individual, kyc, vec![], ConsentStatus::Given, CustomerSegment::Retail).unwrap();  // FR-006

        assert!(customer.is_pep());
    }

    #[test]
    fn test_customer_individual_can_have_no_beneficiaries() {
        let customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
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
            CustomerSegment::Retail,  // FR-006
            vec![],  // FR-008 (documents)
            now,
            now,
            None,
        );
        assert_eq!(customer.id(), &id);
        assert!(customer.is_kyc_validated());
        assert_eq!(customer.risk_score().value(), 30);
    }

    // --- Close and Anonymize tests (STORY-RET-01, INV-10) ---

    #[test]
    fn test_customer_close_sets_status_and_closed_at() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();
        customer.approve_kyc();

        assert!(customer.closed_at().is_none());
        customer.close().unwrap();
        assert_eq!(customer.status(), CustomerStatus::Closed);
        assert!(customer.closed_at().is_some());
    }

    #[test]
    fn test_customer_close_already_closed_fails() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();
        customer.close().unwrap();
        let result = customer.close();
        assert!(result.is_err());
    }

    #[test]
    fn test_customer_anonymize_succeeds_after_10_years() {
        let closed_at = Utc::now() - chrono::Duration::days(3660); // ~10.02 years
        let customer_id = CustomerId::new();
        let mut customer = Customer::reconstitute(
            customer_id,
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            RiskScore::new(10).unwrap(),
            CustomerStatus::Closed,
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
            vec![],  // FR-008 (documents)
            closed_at - chrono::Duration::days(365),
            closed_at,
            Some(closed_at),
        );

        let now = Utc::now();
        customer.anonymize(now).unwrap();
        assert!(customer.is_anonymized());
        assert_eq!(customer.status(), CustomerStatus::Anonymized);
        assert_eq!(customer.kyc_profile().full_name(), "[ANONYMIZED]");
        assert_eq!(customer.kyc_profile().cin_or_rcs(), "[ANONYMIZED]");
        assert!(customer.kyc_profile().is_anonymized());
        assert!(customer.beneficiaries().is_empty());
    }

    #[test]
    fn test_customer_anonymize_fails_before_10_years() {
        let closed_at = Utc::now() - chrono::Duration::days(365 * 5); // only 5 years
        let customer_id = CustomerId::new();
        let mut customer = Customer::reconstitute(
            customer_id,
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            RiskScore::new(10).unwrap(),
            CustomerStatus::Closed,
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
            vec![],  // FR-008 (documents)
            closed_at - chrono::Duration::days(365),
            closed_at,
            Some(closed_at),
        );

        let now = Utc::now();
        let result = customer.anonymize(now);
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::RetentionPeriodNotMet { minimum_years, .. } => {
                assert_eq!(minimum_years, 10);
            }
            e => panic!("Expected RetentionPeriodNotMet, got {:?}", e),
        }
    }

    #[test]
    fn test_customer_anonymize_fails_if_already_anonymized() {
        let closed_at = Utc::now() - chrono::Duration::days(3660);
        let customer_id = CustomerId::new();
        let mut customer = Customer::reconstitute(
            customer_id,
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            RiskScore::new(10).unwrap(),
            CustomerStatus::Closed,
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
            vec![],  // FR-008 (documents)
            closed_at - chrono::Duration::days(365),
            closed_at,
            Some(closed_at),
        );

        let now = Utc::now();
        customer.anonymize(now).unwrap();
        let result = customer.anonymize(now);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::CustomerAlreadyAnonymized);
    }

    #[test]
    fn test_customer_anonymize_fails_if_not_closed() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();

        let result = customer.anonymize(Utc::now());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::CustomerNotClosed);
    }

    #[test]
    fn test_is_anonymized_false_for_active_customer() {
        let customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();
        assert!(!customer.is_anonymized());
    }

    // --- Document Management tests (FR-008) ---

    #[test]
    fn test_customer_add_document() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();

        let doc = Document::new(
            super::value_objects::DocumentType::NationalId,
            "CIN-12345678",
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        )
        .unwrap();

        assert!(customer.add_document(doc).is_ok());
        assert_eq!(customer.documents().len(), 1);
    }

    #[test]
    fn test_customer_remove_document() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();

        let doc = Document::new(
            super::value_objects::DocumentType::NationalId,
            "CIN-12345678",
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
        )
        .unwrap();

        let doc_id = *doc.id();
        customer.add_document(doc).unwrap();
        assert_eq!(customer.documents().len(), 1);

        assert!(customer.remove_document(&doc_id).is_ok());
        assert_eq!(customer.documents().len(), 0);
    }

    #[test]
    fn test_customer_documents_expiring_soon() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();

        let doc = Document::new(
            super::value_objects::DocumentType::NationalId,
            "CIN-12345678",
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
        )
        .unwrap();

        customer.add_document(doc).unwrap();

        let now = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();
        let expiring = customer.documents_expiring_soon(now, 30);
        assert_eq!(expiring.len(), 1);
    }

    #[test]
    fn test_customer_update_segment() {
        let mut customer = Customer::new(
            CustomerType::Individual,
            valid_individual_kyc(),
            vec![],
            ConsentStatus::Given,
            CustomerSegment::Retail,  // FR-006
        )
        .unwrap();

        assert_eq!(customer.segment(), CustomerSegment::Retail);
        customer.update_segment(CustomerSegment::Vip);
        assert_eq!(customer.segment(), CustomerSegment::Vip);
    }
}
