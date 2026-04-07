use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================================
// BMAD FR-045: goAML Integration (Electronic Suspicion Declaration)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GoAmlSubmissionId(Uuid);

impl GoAmlSubmissionId {
    pub fn new() -> Self {
        GoAmlSubmissionId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        GoAmlSubmissionId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for GoAmlSubmissionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoAmlSubmissionStatus {
    /// Initial submission created but not yet sent
    Draft,
    /// Submitted to CTAF (Central Tunisian Authority for AML)
    Submitted,
    /// Acknowledged by CTAF system
    Acknowledged,
    /// Rejected by CTAF (requires resubmission)
    Rejected,
    /// Accepted and under CTAF investigation
    Accepted,
}

impl GoAmlSubmissionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            GoAmlSubmissionStatus::Draft => "Draft",
            GoAmlSubmissionStatus::Submitted => "Submitted",
            GoAmlSubmissionStatus::Acknowledged => "Acknowledged",
            GoAmlSubmissionStatus::Rejected => "Rejected",
            GoAmlSubmissionStatus::Accepted => "Accepted",
        }
    }
}

/// Electronic suspicion declaration to CTAF via goAML XML format
/// Implements FR-045: Déclaration de soupçon électronique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoAmlSubmission {
    id: GoAmlSubmissionId,
    investigation_id: Uuid,
    /// Unique identifier assigned by CTAF
    ctaf_reference: Option<String>,
    status: GoAmlSubmissionStatus,
    /// Suspicion reason (summarized from investigation)
    suspicion_reason: String,
    /// Customer identification document type
    customer_document_type: String,
    customer_document_value: String,
    /// Suspect amount in TND
    suspect_amount: rust_decimal::Decimal,
    /// First suspicion date
    suspicion_date: DateTime<Utc>,
    /// When goAML submission was created
    created_at: DateTime<Utc>,
    /// When submission was sent to CTAF
    submitted_at: Option<DateTime<Utc>>,
    /// CTAF response timestamp
    acknowledged_at: Option<DateTime<Utc>>,
    /// Rejection reason if status = Rejected
    rejection_reason: Option<String>,
    /// goAML XML payload (stored for audit trail)
    xml_payload: String,
}

impl GoAmlSubmission {
    pub fn new(
        investigation_id: Uuid,
        suspicion_reason: String,
        customer_document_type: String,
        customer_document_value: String,
        suspect_amount: rust_decimal::Decimal,
        suspicion_date: DateTime<Utc>,
        xml_payload: String,
    ) -> Result<Self, DomainError> {
        if suspicion_reason.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Suspicion reason cannot be empty".to_string(),
            ));
        }
        if customer_document_value.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Customer document value cannot be empty".to_string(),
            ));
        }
        if suspect_amount <= rust_decimal::Decimal::ZERO {
            return Err(DomainError::InvalidAlert(
                "Suspect amount must be positive".to_string(),
            ));
        }

        Ok(Self {
            id: GoAmlSubmissionId::new(),
            investigation_id,
            ctaf_reference: None,
            status: GoAmlSubmissionStatus::Draft,
            suspicion_reason,
            customer_document_type,
            customer_document_value,
            suspect_amount,
            suspicion_date,
            created_at: Utc::now(),
            submitted_at: None,
            acknowledged_at: None,
            rejection_reason: None,
            xml_payload,
        })
    }

    pub fn id(&self) -> &GoAmlSubmissionId {
        &self.id
    }
    pub fn investigation_id(&self) -> Uuid {
        self.investigation_id
    }
    pub fn status(&self) -> GoAmlSubmissionStatus {
        self.status
    }
    pub fn ctaf_reference(&self) -> Option<&str> {
        self.ctaf_reference.as_deref()
    }

    /// Mark as submitted to CTAF (FR-045)
    pub fn mark_submitted(&mut self) -> Result<(), DomainError> {
        if self.status != GoAmlSubmissionStatus::Draft {
            return Err(DomainError::InvalidAlert(format!(
                "Cannot submit goAML in {} status",
                self.status.as_str()
            )));
        }
        self.status = GoAmlSubmissionStatus::Submitted;
        self.submitted_at = Some(Utc::now());
        Ok(())
    }

    /// Mark as acknowledged by CTAF
    pub fn mark_acknowledged(&mut self, ctaf_reference: String) -> Result<(), DomainError> {
        if self.status != GoAmlSubmissionStatus::Submitted {
            return Err(DomainError::InvalidAlert(
                "Can only acknowledge from Submitted status".to_string(),
            ));
        }
        self.status = GoAmlSubmissionStatus::Acknowledged;
        self.ctaf_reference = Some(ctaf_reference);
        self.acknowledged_at = Some(Utc::now());
        Ok(())
    }

    /// Mark as rejected by CTAF
    pub fn mark_rejected(&mut self, reason: String) -> Result<(), DomainError> {
        if !matches!(
            self.status,
            GoAmlSubmissionStatus::Submitted | GoAmlSubmissionStatus::Acknowledged
        ) {
            return Err(DomainError::InvalidAlert(
                "Can only reject from Submitted or Acknowledged status".to_string(),
            ));
        }
        self.status = GoAmlSubmissionStatus::Rejected;
        self.rejection_reason = Some(reason);
        Ok(())
    }

    /// Mark as accepted (investigation in progress at CTAF)
    pub fn mark_accepted(&mut self) -> Result<(), DomainError> {
        if self.status != GoAmlSubmissionStatus::Acknowledged {
            return Err(DomainError::InvalidAlert(
                "Can only mark accepted from Acknowledged status".to_string(),
            ));
        }
        self.status = GoAmlSubmissionStatus::Accepted;
        Ok(())
    }
}

// ============================================================================
// BMAD FR-047: Travel Rule GAFI R.16 (Originator/Beneficiary Data)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TravelRuleMessageId(Uuid);

impl TravelRuleMessageId {
    pub fn new() -> Self {
        TravelRuleMessageId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        TravelRuleMessageId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TravelRuleMessageId {
    fn default() -> Self {
        Self::new()
    }
}

/// Originator information (sender of funds)
/// Per GAFI R.16 (Travel Rule)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginatorInfo {
    /// Full legal name
    pub name: String,
    /// Identity document type (passport, national_id, etc.)
    pub document_type: String,
    /// Document number
    pub document_number: String,
    /// Originating account number (IBAN preferred)
    pub account_number: String,
    /// Customer ID in BANKO system
    pub customer_id: Uuid,
    /// Address
    pub address: String,
}

impl OriginatorInfo {
    pub fn new(
        name: String,
        document_type: String,
        document_number: String,
        account_number: String,
        customer_id: Uuid,
        address: String,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Originator name is required".to_string(),
            ));
        }
        if document_number.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Originator document number is required".to_string(),
            ));
        }
        if account_number.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Originator account number is required".to_string(),
            ));
        }

        Ok(Self {
            name,
            document_type,
            document_number,
            account_number,
            customer_id,
            address,
        })
    }
}

/// Beneficiary information (receiver of funds)
/// Per GAFI R.16 (Travel Rule)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficiaryInfo {
    /// Full legal name
    pub name: String,
    /// Receiving account number (IBAN preferred)
    pub account_number: String,
    /// Beneficiary bank BIC/SWIFT code
    pub bank_bic: String,
    /// Receiving country (ISO 3166-1 alpha-2)
    pub country: String,
}

impl BeneficiaryInfo {
    pub fn new(
        name: String,
        account_number: String,
        bank_bic: String,
        country: String,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Beneficiary name is required".to_string(),
            ));
        }
        if account_number.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Beneficiary account number is required".to_string(),
            ));
        }
        if country.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Beneficiary country is required".to_string(),
            ));
        }

        Ok(Self {
            name,
            account_number,
            bank_bic,
            country,
        })
    }
}

/// Travel Rule message per GAFI Recommendation 16
/// Implements FR-047: Travel Rule GAFI R.16
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelRuleMessage {
    id: TravelRuleMessageId,
    transaction_id: Uuid,
    originator: OriginatorInfo,
    beneficiary: BeneficiaryInfo,
    /// Transfer amount in original currency
    amount: rust_decimal::Decimal,
    currency: String,
    /// Message creation timestamp
    created_at: DateTime<Utc>,
    /// Message expiry (Travel Rule messages have lifetime)
    expires_at: DateTime<Utc>,
}

impl TravelRuleMessage {
    pub fn new(
        transaction_id: Uuid,
        originator: OriginatorInfo,
        beneficiary: BeneficiaryInfo,
        amount: rust_decimal::Decimal,
        currency: String,
    ) -> Result<Self, DomainError> {
        if amount <= rust_decimal::Decimal::ZERO {
            return Err(DomainError::InvalidTransaction(
                "Transfer amount must be positive".to_string(),
            ));
        }
        if currency.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Currency is required".to_string(),
            ));
        }

        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(24); // 24h validity

        Ok(Self {
            id: TravelRuleMessageId::new(),
            transaction_id,
            originator,
            beneficiary,
            amount,
            currency,
            created_at: now,
            expires_at,
        })
    }

    pub fn id(&self) -> &TravelRuleMessageId {
        &self.id
    }
    pub fn transaction_id(&self) -> Uuid {
        self.transaction_id
    }
    pub fn originator(&self) -> &OriginatorInfo {
        &self.originator
    }
    pub fn beneficiary(&self) -> &BeneficiaryInfo {
        &self.beneficiary
    }
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

// ============================================================================
// BMAD FR-048: Enhanced Due Diligence (EDD)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EddProfileId(Uuid);

impl EddProfileId {
    pub fn new() -> Self {
        EddProfileId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        EddProfileId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for EddProfileId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EddStatus {
    /// Not yet subjected to EDD
    NotApplicable,
    /// EDD in progress
    InProgress,
    /// EDD completed, risk acceptable
    Completed,
    /// EDD completed, risk unacceptable - account blocked
    RejectedHighRisk,
}

impl EddStatus {
    pub fn as_str(&self) -> &str {
        match self {
            EddStatus::NotApplicable => "NotApplicable",
            EddStatus::InProgress => "InProgress",
            EddStatus::Completed => "Completed",
            EddStatus::RejectedHighRisk => "RejectedHighRisk",
        }
    }
}

/// Enhanced Due Diligence profile for high-risk customers
/// Implements FR-048: Enhanced Due Diligence (EDD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EddProfile {
    id: EddProfileId,
    customer_id: Uuid,
    status: EddStatus,
    /// Reason EDD was initiated
    trigger_reason: String,
    /// PEP classification (None, Domestic, Foreign, Family)
    pep_status: String,
    /// Adverse media findings
    adverse_media_findings: Option<String>,
    /// Beneficial owner details collected
    beneficial_owner_verified: bool,
    /// Source of funds verified
    source_of_funds_verified: bool,
    /// Business purpose verified
    business_purpose_verified: bool,
    /// Overall risk assessment
    risk_assessment: String,
    /// Approver ID (compliance officer)
    approved_by: Option<Uuid>,
    /// Created timestamp
    created_at: DateTime<Utc>,
    /// Last review timestamp
    last_reviewed_at: DateTime<Utc>,
    /// Next review due date
    next_review_due: Option<DateTime<Utc>>,
}

impl EddProfile {
    pub fn new(
        customer_id: Uuid,
        trigger_reason: String,
    ) -> Result<Self, DomainError> {
        if trigger_reason.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Trigger reason is required".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(Self {
            id: EddProfileId::new(),
            customer_id,
            status: EddStatus::InProgress,
            trigger_reason,
            pep_status: "None".to_string(),
            adverse_media_findings: None,
            beneficial_owner_verified: false,
            source_of_funds_verified: false,
            business_purpose_verified: false,
            risk_assessment: "MEDIUM".to_string(),
            approved_by: None,
            created_at: now,
            last_reviewed_at: now,
            next_review_due: Some(now + chrono::Duration::days(90)),
        })
    }

    pub fn id(&self) -> &EddProfileId {
        &self.id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn status(&self) -> EddStatus {
        self.status
    }

    /// Mark as completed with approval
    pub fn mark_completed(
        &mut self,
        risk_assessment: String,
        approved_by: Uuid,
    ) -> Result<(), DomainError> {
        if self.status != EddStatus::InProgress {
            return Err(DomainError::InvalidAlert(
                "Can only complete from InProgress status".to_string(),
            ));
        }
        self.status = EddStatus::Completed;
        self.risk_assessment = risk_assessment;
        self.approved_by = Some(approved_by);
        self.last_reviewed_at = Utc::now();
        Ok(())
    }

    /// Reject due to high risk
    pub fn reject_high_risk(&mut self, approved_by: Uuid) -> Result<(), DomainError> {
        if self.status != EddStatus::InProgress {
            return Err(DomainError::InvalidAlert(
                "Can only reject from InProgress status".to_string(),
            ));
        }
        self.status = EddStatus::RejectedHighRisk;
        self.risk_assessment = "CRITICAL".to_string();
        self.approved_by = Some(approved_by);
        self.last_reviewed_at = Utc::now();
        Ok(())
    }

    /// Mark PEP status
    pub fn set_pep_status(&mut self, pep_status: String) -> Result<(), DomainError> {
        if pep_status.is_empty() {
            return Err(DomainError::InvalidAlert("PEP status cannot be empty".to_string()));
        }
        self.pep_status = pep_status;
        Ok(())
    }
}

// ============================================================================
// BMAD FR-051: AML Training Record Tracking
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AmlTrainingRecordId(Uuid);

impl AmlTrainingRecordId {
    pub fn new() -> Self {
        AmlTrainingRecordId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        AmlTrainingRecordId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AmlTrainingRecordId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingType {
    Initial,
    Annual,
    Refresher,
    SpecializedThreat,
}

impl TrainingType {
    pub fn as_str(&self) -> &str {
        match self {
            TrainingType::Initial => "Initial",
            TrainingType::Annual => "Annual",
            TrainingType::Refresher => "Refresher",
            TrainingType::SpecializedThreat => "SpecializedThreat",
        }
    }
}

/// AML Training record for staff compliance
/// Implements FR-051: Formation AML tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlTrainingRecord {
    id: AmlTrainingRecordId,
    employee_id: Uuid,
    training_type: TrainingType,
    /// Training completion date
    completion_date: DateTime<Utc>,
    /// Training material version
    training_version: String,
    /// Training provider/instructor
    provider: String,
    /// Score if exam-based
    score: Option<u8>,
    /// Certificate/proof reference
    certificate_reference: Option<String>,
    /// Next training due date
    next_due_date: DateTime<Utc>,
}

impl AmlTrainingRecord {
    pub fn new(
        employee_id: Uuid,
        training_type: TrainingType,
        training_version: String,
        provider: String,
    ) -> Result<Self, DomainError> {
        if training_version.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Training version is required".to_string(),
            ));
        }
        if provider.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Training provider is required".to_string(),
            ));
        }

        let now = Utc::now();
        let next_due = match training_type {
            TrainingType::Initial => now + chrono::Duration::days(365),
            TrainingType::Annual => now + chrono::Duration::days(365),
            TrainingType::Refresher => now + chrono::Duration::days(180),
            TrainingType::SpecializedThreat => now + chrono::Duration::days(90),
        };

        Ok(Self {
            id: AmlTrainingRecordId::new(),
            employee_id,
            training_type,
            completion_date: now,
            training_version,
            provider,
            score: None,
            certificate_reference: None,
            next_due_date: next_due,
        })
    }

    pub fn id(&self) -> &AmlTrainingRecordId {
        &self.id
    }
    pub fn employee_id(&self) -> Uuid {
        self.employee_id
    }
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.next_due_date
    }

    pub fn set_score(&mut self, score: u8) -> Result<(), DomainError> {
        if score > 100 {
            return Err(DomainError::InvalidAlert(
                "Score must be between 0 and 100".to_string(),
            ));
        }
        self.score = Some(score);
        Ok(())
    }

    pub fn set_certificate(&mut self, cert_ref: String) -> Result<(), DomainError> {
        if cert_ref.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Certificate reference cannot be empty".to_string(),
            ));
        }
        self.certificate_reference = Some(cert_ref);
        Ok(())
    }
}

// ============================================================================
// BMAD FR-053: Continuous PEP Screening Schedule
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PepScreeningScheduleId(Uuid);

impl PepScreeningScheduleId {
    pub fn new() -> Self {
        PepScreeningScheduleId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        PepScreeningScheduleId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PepScreeningScheduleId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreeningFrequency {
    /// Every 30 days
    Monthly,
    /// Every 90 days
    Quarterly,
    /// Every 180 days
    SemiAnnual,
    /// Every 365 days
    Annual,
}

impl ScreeningFrequency {
    pub fn days(&self) -> i64 {
        match self {
            ScreeningFrequency::Monthly => 30,
            ScreeningFrequency::Quarterly => 90,
            ScreeningFrequency::SemiAnnual => 180,
            ScreeningFrequency::Annual => 365,
        }
    }
}

/// Scheduled PEP screening task
/// Implements FR-053: Continuous PEP screening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepScreeningSchedule {
    id: PepScreeningScheduleId,
    customer_id: Uuid,
    /// PEP lists to screen against
    pep_list_sources: Vec<String>, // e.g., ["UN", "EU", "OFAC", "BCT"]
    frequency: ScreeningFrequency,
    /// Active flag
    active: bool,
    /// Last screening date
    last_screened_at: Option<DateTime<Utc>>,
    /// Next scheduled screening
    next_screening_due: DateTime<Utc>,
    /// If PEP match found on last screening
    pep_match_found: bool,
}

impl PepScreeningSchedule {
    pub fn new(
        customer_id: Uuid,
        pep_list_sources: Vec<String>,
        frequency: ScreeningFrequency,
    ) -> Result<Self, DomainError> {
        if pep_list_sources.is_empty() {
            return Err(DomainError::InvalidAlert(
                "At least one PEP list source must be specified".to_string(),
            ));
        }

        let now = Utc::now();
        let next_due = now + chrono::Duration::days(frequency.days());

        Ok(Self {
            id: PepScreeningScheduleId::new(),
            customer_id,
            pep_list_sources,
            frequency,
            active: true,
            last_screened_at: None,
            next_screening_due: next_due,
            pep_match_found: false,
        })
    }

    pub fn id(&self) -> &PepScreeningScheduleId {
        &self.id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn is_due(&self) -> bool {
        Utc::now() >= self.next_screening_due && self.active
    }

    pub fn mark_screened(&mut self, match_found: bool) -> Result<(), DomainError> {
        let now = Utc::now();
        self.last_screened_at = Some(now);
        self.next_screening_due = now + chrono::Duration::days(self.frequency.days());
        self.pep_match_found = match_found;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.active = false;
        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_aml_submission_creation() {
        let submission = GoAmlSubmission::new(
            Uuid::new_v4(),
            "Suspicious structuring detected".to_string(),
            "passport".to_string(),
            "TN123456789".to_string(),
            rust_decimal::Decimal::from(15000),
            Utc::now(),
            "<goAML>...</goAML>".to_string(),
        );
        assert!(submission.is_ok());
        let sub = submission.unwrap();
        assert_eq!(sub.status(), GoAmlSubmissionStatus::Draft);
    }

    #[test]
    fn test_go_aml_submission_workflow() {
        let mut submission = GoAmlSubmission::new(
            Uuid::new_v4(),
            "Structuring".to_string(),
            "passport".to_string(),
            "TN123456789".to_string(),
            rust_decimal::Decimal::from(10000),
            Utc::now(),
            "<goAML>...</goAML>".to_string(),
        )
        .unwrap();

        assert_eq!(submission.status(), GoAmlSubmissionStatus::Draft);

        let _ = submission.mark_submitted();
        assert_eq!(submission.status(), GoAmlSubmissionStatus::Submitted);

        let _ = submission.mark_acknowledged("CTAF-2026-001".to_string());
        assert_eq!(submission.status(), GoAmlSubmissionStatus::Acknowledged);
        assert_eq!(submission.ctaf_reference(), Some("CTAF-2026-001"));
    }

    #[test]
    fn test_travel_rule_message_creation() {
        let originator = OriginatorInfo::new(
            "John Doe".to_string(),
            "passport".to_string(),
            "TN999888777".to_string(),
            "TN6402123456789012345678".to_string(),
            Uuid::new_v4(),
            "123 Main St".to_string(),
        )
        .unwrap();

        let beneficiary = BeneficiaryInfo::new(
            "Jane Smith".to_string(),
            "US9876543210".to_string(),
            "CHASUS33".to_string(),
            "US".to_string(),
        )
        .unwrap();

        let message = TravelRuleMessage::new(
            Uuid::new_v4(),
            originator,
            beneficiary,
            rust_decimal::Decimal::from(5000),
            "USD".to_string(),
        );

        assert!(message.is_ok());
        let msg = message.unwrap();
        assert!(!msg.is_expired());
    }

    #[test]
    fn test_edd_profile_creation() {
        let profile =
            EddProfile::new(Uuid::new_v4(), "PEP detected".to_string()).unwrap();
        assert_eq!(profile.status(), EddStatus::InProgress);
    }

    #[test]
    fn test_edd_profile_workflow() {
        let mut profile =
            EddProfile::new(Uuid::new_v4(), "PEP detected".to_string()).unwrap();
        assert_eq!(profile.status(), EddStatus::InProgress);

        let _ = profile.mark_completed("LOW".to_string(), Uuid::new_v4());
        assert_eq!(profile.status(), EddStatus::Completed);
    }

    #[test]
    fn test_aml_training_record() {
        let record = AmlTrainingRecord::new(
            Uuid::new_v4(),
            TrainingType::Annual,
            "AML_2026_v1".to_string(),
            "CTAF".to_string(),
        );
        assert!(record.is_ok());
        let rec = record.unwrap();
        assert!(!rec.is_expired());
    }

    #[test]
    fn test_aml_training_with_score() {
        let mut record = AmlTrainingRecord::new(
            Uuid::new_v4(),
            TrainingType::Annual,
            "AML_2026_v1".to_string(),
            "CTAF".to_string(),
        )
        .unwrap();

        let _ = record.set_score(95);
        assert_eq!(record.score, Some(95));

        let invalid = record.set_score(150);
        assert!(invalid.is_err());
    }

    #[test]
    fn test_pep_screening_schedule() {
        let schedule = PepScreeningSchedule::new(
            Uuid::new_v4(),
            vec!["UN".to_string(), "OFAC".to_string()],
            ScreeningFrequency::Monthly,
        );
        assert!(schedule.is_ok());
        let sched = schedule.unwrap();
        assert!(sched.is_due());
    }

    #[test]
    fn test_pep_screening_after_check() {
        let mut schedule = PepScreeningSchedule::new(
            Uuid::new_v4(),
            vec!["UN".to_string()],
            ScreeningFrequency::Quarterly,
        )
        .unwrap();

        let _ = schedule.mark_screened(false);
        assert_eq!(schedule.last_screened_at, Some(schedule.next_screening_due - chrono::Duration::days(90)));
        assert!(!schedule.is_due());
    }

    #[test]
    fn test_originator_info_validation() {
        let invalid = OriginatorInfo::new(
            "".to_string(),
            "passport".to_string(),
            "TN123".to_string(),
            "IBAN".to_string(),
            Uuid::new_v4(),
            "Address".to_string(),
        );
        assert!(invalid.is_err());
    }

    #[test]
    fn test_beneficiary_info_validation() {
        let invalid = BeneficiaryInfo::new(
            "Name".to_string(),
            "".to_string(),
            "BIC".to_string(),
            "US".to_string(),
        );
        assert!(invalid.is_err());
    }
}
