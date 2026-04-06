use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// SMSI Control aggregate state
#[derive(Debug, Clone)]
pub struct SmsiControl {
    pub id: Uuid,
    pub control_code: String,
    pub name: String,
    pub description: String,
    pub theme: String,
    pub status: String, // "NotStarted", "InProgress", "Implemented", "Partial"
    pub evidence: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Risk entry aggregate state
#[derive(Debug, Clone)]
pub struct RiskEntry {
    pub id: Uuid,
    pub identifier: String,
    pub description: String,
    pub score: i32, // 0-25 scale
    pub status: String, // "Identified", "Mitigated", "Closed"
    pub mitigations: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Token Vault entry state
#[derive(Debug, Clone)]
pub struct TokenVaultEntry {
    pub id: Uuid,
    pub token: String,
    pub masked_pan: String, // e.g., "****5678"
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

// ============================================================
// ISmsiRepository - SMSI Control Repository Port
// ============================================================

#[async_trait]
pub trait ISmsiRepository: Send + Sync {
    /// Save or update a control
    async fn save_control(&self, control: &SmsiControl) -> Result<(), String>;

    /// Find a control by ID
    async fn find_control_by_id(&self, id: Uuid) -> Result<Option<SmsiControl>, String>;

    /// Find controls by theme (e.g., "Access Control", "Encryption")
    async fn find_controls_by_theme(&self, theme: &str) -> Result<Vec<SmsiControl>, String>;

    /// List all controls
    async fn list_all_controls(&self) -> Result<Vec<SmsiControl>, String>;

    /// Count controls by status
    async fn count_by_status(&self, status: &str) -> Result<i64, String>;

    /// Save or update a risk entry
    async fn save_risk(&self, risk: &RiskEntry) -> Result<(), String>;

    /// Find a risk by ID
    async fn find_risk_by_id(&self, id: Uuid) -> Result<Option<RiskEntry>, String>;

    /// List all risks
    async fn list_all_risks(&self) -> Result<Vec<RiskEntry>, String>;

    /// List high risks (score >= 15)
    async fn list_high_risks(&self) -> Result<Vec<RiskEntry>, String>;
}

// ============================================================
// ITokenVaultRepository - Token Vault Repository Port
// ============================================================

#[async_trait]
pub trait ITokenVaultRepository: Send + Sync {
    /// Save a tokenized entry
    async fn save_token(&self, entry: &TokenVaultEntry) -> Result<(), String>;

    /// Find token entry by token value
    async fn find_by_token(&self, token: &str) -> Result<Option<TokenVaultEntry>, String>;

    /// Revoke a token
    async fn revoke_token(&self, token_id: Uuid) -> Result<(), String>;

    /// List active tokens
    async fn list_active_tokens(&self) -> Result<Vec<TokenVaultEntry>, String>;

    /// Count total tokens
    async fn count_tokens(&self) -> Result<i64, String>;
}

// ============================================================
// INPDP Consent Repository Port
// ============================================================

#[async_trait]
pub trait IInpdpConsentRepository: Send + Sync {
    /// Save or update a consent
    async fn save_consent(
        &self,
        consent: &banko_domain::compliance::InpdpConsent,
    ) -> Result<(), String>;

    /// Find a consent by ID
    async fn find_consent_by_id(
        &self,
        id: &banko_domain::compliance::InpdpConsentId,
    ) -> Result<Option<banko_domain::compliance::InpdpConsent>, String>;

    /// Find all consents for a customer
    async fn find_consents_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<banko_domain::compliance::InpdpConsent>, String>;

    /// Find active consents for a customer
    async fn find_active_consents_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<banko_domain::compliance::InpdpConsent>, String>;

    /// Count consents by purpose
    async fn count_by_purpose(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<i64, String>;
}

// ============================================================
// DPIA Repository Port
// ============================================================

#[async_trait]
pub trait IDpiaRepository: Send + Sync {
    /// Save or update a DPIA
    async fn save_dpia(&self, dpia: &banko_domain::compliance::Dpia) -> Result<(), String>;

    /// Find a DPIA by ID
    async fn find_dpia_by_id(
        &self,
        id: &banko_domain::compliance::DpiaId,
    ) -> Result<Option<banko_domain::compliance::Dpia>, String>;

    /// List all DPIAs
    async fn list_all_dpias(&self) -> Result<Vec<banko_domain::compliance::Dpia>, String>;

    /// List DPIAs by status
    async fn list_by_status(
        &self,
        status: &str,
    ) -> Result<Vec<banko_domain::compliance::Dpia>, String>;

    /// Count DPIAs by status
    async fn count_by_status(&self, status: &str) -> Result<i64, String>;
}

// ============================================================
// Breach Notification Repository Port
// ============================================================

#[async_trait]
pub trait IBreachNotificationRepository: Send + Sync {
    /// Save or update a breach notification
    async fn save_breach(
        &self,
        breach: &banko_domain::compliance::BreachNotification,
    ) -> Result<(), String>;

    /// Find a breach by ID
    async fn find_breach_by_id(
        &self,
        id: &banko_domain::compliance::BreachNotificationId,
    ) -> Result<Option<banko_domain::compliance::BreachNotification>, String>;

    /// List all breaches
    async fn list_all_breaches(&self)
        -> Result<Vec<banko_domain::compliance::BreachNotification>, String>;

    /// List breaches by status
    async fn list_by_status(
        &self,
        status: &str,
    ) -> Result<Vec<banko_domain::compliance::BreachNotification>, String>;

    /// Count breaches that need 72h authority notification
    async fn count_pending_authority_notification(&self) -> Result<i64, String>;
}

// ============================================================
// Data Portability/Erasure Request Port
// ============================================================

#[derive(Debug, Clone)]
pub struct DataPortabilityRequest {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub status: String, // "Pending", "Processing", "Completed", "Failed"
    pub reason: Option<String>,
    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ErasureRequest {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub status: String,
    pub reason: Option<String>,
    pub requested_at: DateTime<Utc>,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait IDataPortabilityRepository: Send + Sync {
    /// Save a data portability request
    async fn save_request(&self, request: &DataPortabilityRequest) -> Result<(), String>;

    /// Find request by ID
    async fn find_request_by_id(&self, id: Uuid)
        -> Result<Option<DataPortabilityRequest>, String>;

    /// List requests by customer
    async fn find_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<DataPortabilityRequest>, String>;
}

#[async_trait]
pub trait IErasureRepository: Send + Sync {
    /// Save an erasure request
    async fn save_request(&self, request: &ErasureRequest) -> Result<(), String>;

    /// Find request by ID
    async fn find_request_by_id(&self, id: Uuid) -> Result<Option<ErasureRequest>, String>;

    /// List requests by customer
    async fn find_by_customer(&self, customer_id: Uuid)
        -> Result<Vec<ErasureRequest>, String>;
}

// ============================================================
// IBiometricRepository - e-KYC Biometric Verification Port
// ============================================================

#[derive(Debug, Clone)]
pub struct BiometricVerificationDto {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub verification_type: String, // "facial_recognition", "fingerprint", etc.
    pub status: String,            // "pending", "in_progress", "verified", "failed", "expired"
    pub confidence_score: f64,
    pub liveness_check: bool,
    pub document_type: Option<String>,
    pub document_number: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[async_trait]
pub trait IBiometricRepository: Send + Sync {
    /// Save or update a biometric verification
    async fn save_verification(&self, verification: &BiometricVerificationDto)
        -> Result<(), String>;

    /// Find verification by ID
    async fn find_verification_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<BiometricVerificationDto>, String>;

    /// Find latest verification for a customer by type
    async fn find_latest_by_customer_and_type(
        &self,
        customer_id: Uuid,
        verification_type: &str,
    ) -> Result<Option<BiometricVerificationDto>, String>;

    /// List all verifications for a customer
    async fn find_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<BiometricVerificationDto>, String>;

    /// Find verified (not expired) biometric for customer
    async fn find_verified_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Option<BiometricVerificationDto>, String>;

    /// Count verifications by status
    async fn count_by_status(&self, status: &str) -> Result<i64, String>;
}
