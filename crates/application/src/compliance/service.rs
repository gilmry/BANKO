use std::sync::Arc;
use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use banko_domain::compliance::{
    BreachNotification, BreachNotificationId, BiometricVerification,
    BiometricType, ConsentPurpose, Dpia, DpiaId, InpdpConsent, InpdpConsentId, LegalBasis,
};

use super::errors::ComplianceError;
use super::ports::{
    BiometricVerificationDto, DataPortabilityRequest, ErasureRequest,
    IBreachNotificationRepository, IBiometricRepository, IDataPortabilityRepository,
    IDpiaRepository, IErasureRepository, IInpdpConsentRepository,
};

// ============================================================
// InpdpConsentService
// ============================================================

pub struct InpdpConsentService {
    consent_repo: Arc<dyn IInpdpConsentRepository>,
}

impl InpdpConsentService {
    pub fn new(consent_repo: Arc<dyn IInpdpConsentRepository>) -> Self {
        InpdpConsentService { consent_repo }
    }

    /// Grant a new INPDP consent for a customer.
    pub async fn grant_consent(
        &self,
        customer_id: Uuid,
        purpose: &str,
        legal_basis: &str,
        data_categories: Vec<String>,
        expiry_days: Option<i64>,
    ) -> Result<InpdpConsent, ComplianceError> {
        let purpose = ConsentPurpose::from_str(purpose)
            .map_err(|e| ComplianceError::InvalidInput(e.to_string()))?;
        let legal_basis = LegalBasis::from_str(legal_basis)
            .map_err(|e| ComplianceError::InvalidInput(e.to_string()))?;

        let expiry_date = expiry_days.map(|days| Utc::now() + Duration::days(days));

        let consent = InpdpConsent::new(customer_id, purpose, legal_basis, data_categories, expiry_date);

        self.consent_repo
            .save_consent(&consent)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(consent)
    }

    /// Revoke a consent by ID.
    pub async fn revoke_consent(
        &self,
        consent_id: &InpdpConsentId,
    ) -> Result<InpdpConsent, ComplianceError> {
        let mut consent = self
            .consent_repo
            .find_consent_by_id(consent_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput(
                "Consent not found".to_string(),
            ))?;

        consent
            .revoke()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.consent_repo
            .save_consent(&consent)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(consent)
    }

    /// Get a consent dashboard for a customer showing consent overview.
    pub async fn get_consent_dashboard(
        &self,
        customer_id: Uuid,
    ) -> Result<(i64, i64, i64, HashMap<String, i64>), ComplianceError> {
        let all_consents = self
            .consent_repo
            .find_consents_by_customer(customer_id)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let active = all_consents.iter().filter(|c| c.is_valid()).count() as i64;
        let revoked = all_consents.iter().filter(|c| !c.is_valid()).count() as i64;
        let total = all_consents.len() as i64;

        let mut by_purpose = HashMap::new();
        for consent in &all_consents {
            let purpose = consent.purpose().as_str().to_string();
            *by_purpose.entry(purpose).or_insert(0) += 1;
        }

        Ok((total, active, revoked, by_purpose))
    }

    /// List all consents for a customer.
    pub async fn list_consents_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<InpdpConsent>, ComplianceError> {
        self.consent_repo
            .find_consents_by_customer(customer_id)
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

// ============================================================
// DpiaService
// ============================================================

pub struct DpiaService {
    dpia_repo: Arc<dyn IDpiaRepository>,
}

impl DpiaService {
    pub fn new(dpia_repo: Arc<dyn IDpiaRepository>) -> Self {
        DpiaService { dpia_repo }
    }

    /// Create a new DPIA in Draft status.
    pub async fn create_dpia(
        &self,
        title: String,
        description: String,
        processing_activity: String,
        risk_assessment: String,
        mitigations: Vec<String>,
    ) -> Result<Dpia, ComplianceError> {
        let dpia = Dpia::new(title, description, processing_activity, risk_assessment, mitigations);

        self.dpia_repo
            .save_dpia(&dpia)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(dpia)
    }

    /// Submit a DPIA for review (Draft -> UnderReview).
    pub async fn submit_dpia_for_review(&self, dpia_id: &DpiaId) -> Result<Dpia, ComplianceError> {
        let mut dpia = self
            .dpia_repo
            .find_dpia_by_id(dpia_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::ControlNotFound)?;

        dpia
            .submit_for_review()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.dpia_repo
            .save_dpia(&dpia)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(dpia)
    }

    /// Approve a DPIA (UnderReview -> Approved).
    pub async fn approve_dpia(
        &self,
        dpia_id: &DpiaId,
        approved_by: String,
    ) -> Result<Dpia, ComplianceError> {
        let mut dpia = self
            .dpia_repo
            .find_dpia_by_id(dpia_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::ControlNotFound)?;

        dpia
            .approve(approved_by)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.dpia_repo
            .save_dpia(&dpia)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(dpia)
    }

    /// Reject a DPIA (UnderReview -> Rejected).
    pub async fn reject_dpia(&self, dpia_id: &DpiaId) -> Result<Dpia, ComplianceError> {
        let mut dpia = self
            .dpia_repo
            .find_dpia_by_id(dpia_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::ControlNotFound)?;

        dpia
            .reject()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.dpia_repo
            .save_dpia(&dpia)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(dpia)
    }

    /// Get a DPIA by ID.
    pub async fn get_dpia(&self, dpia_id: &DpiaId) -> Result<Dpia, ComplianceError> {
        self.dpia_repo
            .find_dpia_by_id(dpia_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::ControlNotFound)
    }

    /// List all DPIAs.
    pub async fn list_dpias(&self) -> Result<Vec<Dpia>, ComplianceError> {
        self.dpia_repo
            .list_all_dpias()
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    /// List DPIAs by status.
    pub async fn list_dpias_by_status(&self, status: &str) -> Result<Vec<Dpia>, ComplianceError> {
        self.dpia_repo
            .list_by_status(status)
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

// ============================================================
// BreachNotificationService
// ============================================================

pub struct BreachNotificationService {
    breach_repo: Arc<dyn IBreachNotificationRepository>,
}

impl BreachNotificationService {
    pub fn new(breach_repo: Arc<dyn IBreachNotificationRepository>) -> Self {
        BreachNotificationService { breach_repo }
    }

    /// Report a new data breach.
    pub async fn report_breach(
        &self,
        breach_type: String,
        description: String,
        affected_data: Vec<String>,
        affected_count: u32,
    ) -> Result<BreachNotification, ComplianceError> {
        let breach = BreachNotification::new(breach_type, description, affected_data, affected_count);

        self.breach_repo
            .save_breach(&breach)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(breach)
    }

    /// Notify authority of a breach (must comply with 72h deadline).
    pub async fn notify_authority(
        &self,
        breach_id: &BreachNotificationId,
    ) -> Result<BreachNotification, ComplianceError> {
        let mut breach = self
            .breach_repo
            .find_breach_by_id(breach_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Breach not found".to_string()))?;

        breach
            .notify_authority()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.breach_repo
            .save_breach(&breach)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(breach)
    }

    /// Notify affected subjects of a breach.
    pub async fn notify_subjects(
        &self,
        breach_id: &BreachNotificationId,
    ) -> Result<BreachNotification, ComplianceError> {
        let mut breach = self
            .breach_repo
            .find_breach_by_id(breach_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Breach not found".to_string()))?;

        breach
            .notify_subjects()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.breach_repo
            .save_breach(&breach)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(breach)
    }

    /// Mark breach as resolved.
    pub async fn resolve_breach(
        &self,
        breach_id: &BreachNotificationId,
    ) -> Result<BreachNotification, ComplianceError> {
        let mut breach = self
            .breach_repo
            .find_breach_by_id(breach_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Breach not found".to_string()))?;

        breach
            .resolve()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.breach_repo
            .save_breach(&breach)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(breach)
    }

    /// Check for breaches that violate the 72h authority notification deadline.
    pub async fn check_72h_compliance(&self) -> Result<i64, ComplianceError> {
        self.breach_repo
            .count_pending_authority_notification()
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    /// Get a breach by ID.
    pub async fn get_breach(&self, breach_id: &BreachNotificationId) -> Result<BreachNotification, ComplianceError> {
        self.breach_repo
            .find_breach_by_id(breach_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Breach not found".to_string()))
    }

    /// List all breaches.
    pub async fn list_breaches(&self) -> Result<Vec<BreachNotification>, ComplianceError> {
        self.breach_repo
            .list_all_breaches()
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

// ============================================================
// DataPortabilityService
// ============================================================

pub struct DataPortabilityService {
    portability_repo: Arc<dyn IDataPortabilityRepository>,
}

impl DataPortabilityService {
    pub fn new(portability_repo: Arc<dyn IDataPortabilityRepository>) -> Self {
        DataPortabilityService { portability_repo }
    }

    /// Request data portability (initiate data export).
    pub async fn request_data_portability(
        &self,
        customer_id: Uuid,
        reason: Option<String>,
    ) -> Result<DataPortabilityRequest, ComplianceError> {
        let request = DataPortabilityRequest {
            id: Uuid::new_v4(),
            customer_id,
            status: "Pending".to_string(),
            reason,
            requested_at: Utc::now(),
            completed_at: None,
        };

        self.portability_repo
            .save_request(&request)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(request)
    }

    /// Get a portability request.
    pub async fn get_request(
        &self,
        request_id: Uuid,
    ) -> Result<DataPortabilityRequest, ComplianceError> {
        self.portability_repo
            .find_request_by_id(request_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput(
                "Data portability request not found".to_string(),
            ))
    }

    /// List portability requests for a customer.
    pub async fn list_requests(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<DataPortabilityRequest>, ComplianceError> {
        self.portability_repo
            .find_by_customer(customer_id)
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

// ============================================================
// ErasureService
// ============================================================

pub struct ErasureService {
    erasure_repo: Arc<dyn IErasureRepository>,
}

impl ErasureService {
    pub fn new(erasure_repo: Arc<dyn IErasureRepository>) -> Self {
        ErasureService { erasure_repo }
    }

    /// Request right to be forgotten (erasure).
    pub async fn request_erasure(
        &self,
        customer_id: Uuid,
        reason: Option<String>,
    ) -> Result<ErasureRequest, ComplianceError> {
        let request = ErasureRequest {
            id: Uuid::new_v4(),
            customer_id,
            status: "Pending".to_string(),
            reason,
            requested_at: Utc::now(),
            scheduled_for: None,
        };

        self.erasure_repo
            .save_request(&request)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(request)
    }

    /// Check if a customer is eligible for erasure based on retention periods.
    /// Regulatory retention typically requires 7+ years for financial records.
    pub fn check_erasure_eligibility(&self, customer_closed_at: Option<DateTime<Utc>>) -> (bool, String) {
        match customer_closed_at {
            None => (
                false,
                "Customer account still active - cannot erase".to_string(),
            ),
            Some(closed_at) => {
                let retention_years = 7;
                let retention_duration = Duration::days(retention_years * 365);
                let eligible_from = closed_at + retention_duration;

                if Utc::now() >= eligible_from {
                    (true, "Customer eligible for erasure after retention period".to_string())
                } else {
                    let days_remaining = (eligible_from - Utc::now()).num_days();
                    (
                        false,
                        format!(
                            "Retention period not yet met. {} days remaining",
                            days_remaining
                        ),
                    )
                }
            }
        }
    }

    /// Get an erasure request.
    pub async fn get_request(&self, request_id: Uuid) -> Result<ErasureRequest, ComplianceError> {
        self.erasure_repo
            .find_request_by_id(request_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput(
                "Erasure request not found".to_string(),
            ))
    }

    /// List erasure requests for a customer.
    pub async fn list_requests(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<ErasureRequest>, ComplianceError> {
        self.erasure_repo
            .find_by_customer(customer_id)
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    // Mock repositories for testing
    struct MockConsentRepository {
        consents: Mutex<Vec<InpdpConsent>>,
    }

    impl MockConsentRepository {
        fn new() -> Self {
            MockConsentRepository {
                consents: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IInpdpConsentRepository for MockConsentRepository {
        async fn save_consent(&self, consent: &InpdpConsent) -> Result<(), String> {
            let mut consents = self.consents.lock().unwrap();
            consents.retain(|c| c.id() != consent.id());
            consents.push(consent.clone());
            Ok(())
        }

        async fn find_consent_by_id(
            &self,
            id: &InpdpConsentId,
        ) -> Result<Option<InpdpConsent>, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents.iter().find(|c| c.id() == id).cloned())
        }

        async fn find_consents_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Vec<InpdpConsent>, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents
                .iter()
                .filter(|c| c.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn find_active_consents_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Vec<InpdpConsent>, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents
                .iter()
                .filter(|c| c.customer_id() == customer_id && c.is_valid())
                .cloned()
                .collect())
        }

        async fn count_by_purpose(
            &self,
            customer_id: Uuid,
            purpose: &str,
        ) -> Result<i64, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents
                .iter()
                .filter(|c| c.customer_id() == customer_id && c.purpose().as_str() == purpose)
                .count() as i64)
        }
    }

    #[tokio::test]
    async fn test_grant_consent() {
        let repo = Arc::new(MockConsentRepository::new());
        let service = InpdpConsentService::new(repo);
        let cid = Uuid::new_v4();

        let consent = service
            .grant_consent(cid, "Marketing", "Consent", vec!["email".to_string()], None)
            .await
            .unwrap();

        assert!(consent.is_valid());
        assert_eq!(consent.customer_id(), cid);
    }

    #[tokio::test]
    async fn test_revoke_consent() {
        let repo = Arc::new(MockConsentRepository::new());
        let service = InpdpConsentService::new(repo);
        let cid = Uuid::new_v4();

        let consent = service
            .grant_consent(cid, "Analytics", "Consent", vec!["behavior".to_string()], None)
            .await
            .unwrap();

        let revoked = service.revoke_consent(consent.id()).await.unwrap();
        assert!(!revoked.is_valid());
    }

    #[tokio::test]
    async fn test_consent_dashboard() {
        let repo = Arc::new(MockConsentRepository::new());
        let service = InpdpConsentService::new(repo);
        let cid = Uuid::new_v4();

        service
            .grant_consent(cid, "Marketing", "Consent", vec![], None)
            .await
            .unwrap();
        service
            .grant_consent(cid, "Analytics", "Consent", vec![], None)
            .await
            .unwrap();

        let (total, active, revoked, by_purpose) = service.get_consent_dashboard(cid).await.unwrap();
        assert_eq!(total, 2);
        assert_eq!(active, 2);
        assert_eq!(revoked, 0);
        assert_eq!(by_purpose.len(), 2);
    }
}

// ============================================================
// EkycService - e-KYC Biometric Verification Service
// ============================================================

pub struct EkycService {
    biometric_repo: Arc<dyn IBiometricRepository>,
}

impl EkycService {
    pub fn new(biometric_repo: Arc<dyn IBiometricRepository>) -> Self {
        EkycService { biometric_repo }
    }

    /// Initiate a new biometric verification for a customer
    pub async fn initiate_verification(
        &self,
        customer_id: Uuid,
        verification_type: &str,
        validity_days: Option<i64>,
    ) -> Result<BiometricVerificationDto, ComplianceError> {
        let biometric_type = BiometricType::from_str(verification_type)
            .map_err(ComplianceError::InvalidInput)?;

        let validity = validity_days.unwrap_or(90);

        let verification = BiometricVerification::new(customer_id, biometric_type, validity)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        let dto = BiometricVerificationDto {
            id: *verification.id().as_uuid(),
            customer_id: verification.customer_id(),
            verification_type: verification.verification_type().to_string(),
            status: verification.status().to_string(),
            confidence_score: verification.confidence_score(),
            liveness_check: verification.liveness_check(),
            document_type: verification.document_type().map(|s| s.to_string()),
            document_number: verification.document_number().map(|s| s.to_string()),
            verified_at: verification.verified_at(),
            created_at: verification.created_at(),
            expires_at: verification.expires_at(),
        };

        self.biometric_repo
            .save_verification(&dto)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(dto)
    }

    /// Complete a biometric verification with results
    pub async fn complete_verification(
        &self,
        verification_id: Uuid,
        confidence_score: f64,
        liveness_passed: bool,
        document_type: Option<String>,
        document_number: Option<String>,
    ) -> Result<BiometricVerificationDto, ComplianceError> {
        let dto = self
            .biometric_repo
            .find_verification_by_id(verification_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput(
                "Biometric verification not found".to_string(),
            ))?;

        // Reconstruct domain entity from DTO
        let mut verification = BiometricVerification::new(
            dto.customer_id,
            BiometricType::from_str(&dto.verification_type)
                .map_err(ComplianceError::InvalidInput)?,
            (dto.expires_at - Utc::now()).num_days(),
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        // Set internal state based on DTO — transition to InProgress if needed
        if dto.status == "pending" || dto.status == "in_progress" {
            verification
                .mark_in_progress()
                .map_err(|e| ComplianceError::DomainError(e.to_string()))?;
        }

        // Complete the verification
        verification
            .complete_verification(
                confidence_score,
                liveness_passed,
                document_type.clone(),
                document_number.clone(),
            )
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        let updated_dto = BiometricVerificationDto {
            id: *verification.id().as_uuid(),
            customer_id: verification.customer_id(),
            verification_type: verification.verification_type().to_string(),
            status: verification.status().to_string(),
            confidence_score: verification.confidence_score(),
            liveness_check: verification.liveness_check(),
            document_type: verification.document_type().map(|s| s.to_string()),
            document_number: verification.document_number().map(|s| s.to_string()),
            verified_at: verification.verified_at(),
            created_at: verification.created_at(),
            expires_at: verification.expires_at(),
        };

        self.biometric_repo
            .save_verification(&updated_dto)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(updated_dto)
    }

    /// Check the status of a biometric verification
    pub async fn check_verification_status(
        &self,
        verification_id: Uuid,
    ) -> Result<String, ComplianceError> {
        let dto = self
            .biometric_repo
            .find_verification_by_id(verification_id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput(
                "Biometric verification not found".to_string(),
            ))?;

        Ok(dto.status)
    }

    /// Check if a customer has valid e-KYC biometric verification
    pub async fn is_customer_ekyc_verified(&self, customer_id: Uuid) -> Result<bool, ComplianceError> {
        let verification = self
            .biometric_repo
            .find_verified_by_customer(customer_id)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(verification.is_some())
    }

    /// Get all biometric verifications for a customer
    pub async fn get_customer_verifications(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<BiometricVerificationDto>, ComplianceError> {
        self.biometric_repo
            .find_by_customer(customer_id)
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    /// Get the latest biometric verification for a customer by type
    pub async fn get_latest_verification_by_type(
        &self,
        customer_id: Uuid,
        verification_type: &str,
    ) -> Result<Option<BiometricVerificationDto>, ComplianceError> {
        self.biometric_repo
            .find_latest_by_customer_and_type(customer_id, verification_type)
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    /// Count verifications by status (for reporting)
    pub async fn count_by_status(&self, status: &str) -> Result<i64, ComplianceError> {
        self.biometric_repo
            .count_by_status(status)
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

#[cfg(test)]
mod ekyc_tests {
    use super::*;

    struct MockBiometricRepository {
        verifications: std::sync::Mutex<Vec<BiometricVerificationDto>>,
    }

    impl MockBiometricRepository {
        fn new() -> Self {
            MockBiometricRepository {
                verifications: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl IBiometricRepository for MockBiometricRepository {
        async fn save_verification(
            &self,
            verification: &BiometricVerificationDto,
        ) -> Result<(), String> {
            let mut verifications = self.verifications.lock().unwrap();
            verifications.retain(|v| v.id != verification.id);
            verifications.push(verification.clone());
            Ok(())
        }

        async fn find_verification_by_id(
            &self,
            id: Uuid,
        ) -> Result<Option<BiometricVerificationDto>, String> {
            let verifications = self.verifications.lock().unwrap();
            Ok(verifications.iter().find(|v| v.id == id).cloned())
        }

        async fn find_latest_by_customer_and_type(
            &self,
            customer_id: Uuid,
            verification_type: &str,
        ) -> Result<Option<BiometricVerificationDto>, String> {
            let verifications = self.verifications.lock().unwrap();
            Ok(verifications
                .iter()
                .filter(|v| v.customer_id == customer_id && v.verification_type == verification_type)
                .max_by_key(|v| v.created_at)
                .cloned())
        }

        async fn find_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Vec<BiometricVerificationDto>, String> {
            let verifications = self.verifications.lock().unwrap();
            Ok(verifications
                .iter()
                .filter(|v| v.customer_id == customer_id)
                .cloned()
                .collect())
        }

        async fn find_verified_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Option<BiometricVerificationDto>, String> {
            let verifications = self.verifications.lock().unwrap();
            Ok(verifications
                .iter()
                .filter(|v| {
                    v.customer_id == customer_id
                        && v.status == "verified"
                        && Utc::now() < v.expires_at
                })
                .max_by_key(|v| v.created_at)
                .cloned())
        }

        async fn count_by_status(&self, status: &str) -> Result<i64, String> {
            let verifications = self.verifications.lock().unwrap();
            Ok(verifications.iter().filter(|v| v.status == status).count() as i64)
        }
    }

    #[tokio::test]
    async fn test_initiate_verification() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo);
        let customer_id = Uuid::new_v4();

        let verification = service
            .initiate_verification(customer_id, "facial_recognition", Some(90))
            .await
            .unwrap();

        assert_eq!(verification.customer_id, customer_id);
        assert_eq!(verification.verification_type, "facial_recognition");
        assert_eq!(verification.status, "pending");
        assert_eq!(verification.confidence_score, 0.0);
        assert!(!verification.liveness_check);
    }

    #[tokio::test]
    async fn test_initiate_verification_invalid_type() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo);
        let customer_id = Uuid::new_v4();

        let result = service
            .initiate_verification(customer_id, "invalid_type", None)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_complete_verification_success() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo.clone());
        let customer_id = Uuid::new_v4();

        let verification = service
            .initiate_verification(customer_id, "facial_recognition", Some(90))
            .await
            .unwrap();

        // Simulate completion
        let verification_id = verification.id;
        let completed = service
            .complete_verification(
                verification_id,
                0.98,
                true,
                Some("CIN".to_string()),
                Some("ABC123".to_string()),
            )
            .await
            .unwrap();

        assert_eq!(completed.status, "verified");
        assert_eq!(completed.confidence_score, 0.98);
        assert!(completed.liveness_check);
    }

    #[tokio::test]
    async fn test_complete_verification_low_confidence() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo);
        let customer_id = Uuid::new_v4();

        let verification = service
            .initiate_verification(customer_id, "fingerprint", Some(60))
            .await
            .unwrap();

        let verification_id = verification.id;
        let completed = service
            .complete_verification(
                verification_id,
                0.85,
                true,
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(completed.status, "failed");
        assert!(!completed.liveness_check || completed.confidence_score < 0.95);
    }

    #[tokio::test]
    async fn test_is_customer_ekyc_verified() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo);
        let customer_id = Uuid::new_v4();

        let not_verified = service.is_customer_ekyc_verified(customer_id).await.unwrap();
        assert!(!not_verified);

        let verification = service
            .initiate_verification(customer_id, "video_call", Some(90))
            .await
            .unwrap();

        let _completed = service
            .complete_verification(
                verification.id,
                0.97,
                true,
                None,
                None,
            )
            .await
            .unwrap();

        let is_verified = service.is_customer_ekyc_verified(customer_id).await.unwrap();
        assert!(is_verified);
    }

    #[tokio::test]
    async fn test_get_customer_verifications() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo);
        let customer_id = Uuid::new_v4();

        service
            .initiate_verification(customer_id, "facial_recognition", Some(90))
            .await
            .unwrap();

        service
            .initiate_verification(customer_id, "fingerprint", Some(60))
            .await
            .unwrap();

        let verifications = service
            .get_customer_verifications(customer_id)
            .await
            .unwrap();

        assert_eq!(verifications.len(), 2);
    }

    #[tokio::test]
    async fn test_check_verification_status() {
        let repo = Arc::new(MockBiometricRepository::new());
        let service = EkycService::new(repo);
        let customer_id = Uuid::new_v4();

        let verification = service
            .initiate_verification(customer_id, "document_scan", Some(90))
            .await
            .unwrap();

        let status = service
            .check_verification_status(verification.id)
            .await
            .unwrap();

        assert_eq!(status, "pending");
    }
}
