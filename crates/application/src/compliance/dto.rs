use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================
// Control Request DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct CreateControlRequest {
    pub control_code: String,
    pub name: String,
    pub description: String,
    pub theme: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateControlRequest {
    pub status: Option<String>, // "NotStarted", "InProgress", "Implemented", "Partial"
    pub evidence: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ControlResponse {
    pub id: String,
    pub control_code: String,
    pub name: String,
    pub description: String,
    pub theme: String,
    pub status: String,
    pub evidence: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================
// Risk Request DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct CreateRiskRequest {
    pub identifier: String,
    pub description: String,
    pub score: i32, // 0-25 scale
}

#[derive(Debug, Deserialize)]
pub struct UpdateRiskRequest {
    pub status: Option<String>, // "Identified", "Mitigated", "Closed"
    pub mitigations: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RiskResponse {
    pub id: String,
    pub identifier: String,
    pub description: String,
    pub score: i32,
    pub status: String,
    pub mitigations: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================
// Token Vault Request DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct TokenizeRequest {
    pub pan: String, // Primary Account Number (e.g., "1234567890123456")
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub masked_pan: String, // e.g., "****5678"
    pub created_at: DateTime<Utc>,
}

// ============================================================
// Compliance Dashboard Response
// ============================================================

#[derive(Debug, Serialize)]
pub struct ComplianceDashboardResponse {
    pub total_controls: i64,
    pub implemented_count: i64,
    pub partial_count: i64,
    pub in_progress_count: i64,
    pub not_started_count: i64,
    pub implementation_percentage: f64, // (implemented + partial) / total * 100
    pub total_risks: i64,
    pub high_risks_count: i64, // score >= 15
    pub identified_risks: i64,
    pub mitigated_risks: i64,
    pub closed_risks: i64,
    pub total_tokens: i64,
    pub active_tokens: i64,
    pub generated_at: DateTime<Utc>,
}

// ============================================================
// List Responses
// ============================================================

#[derive(Debug, Serialize)]
pub struct ControlListResponse {
    pub data: Vec<ControlResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct RiskListResponse {
    pub data: Vec<RiskResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct TokenListResponse {
    pub data: Vec<TokenResponse>,
    pub total: i64,
}

// ============================================================
// INPDP Consent Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct GrantConsentRequest {
    pub customer_id: String,
    pub purpose: String, // "Marketing", "Analytics", "ThirdPartySharing", "Profiling", "CrossBorder"
    pub legal_basis: String, // "Consent", "ContractualNecessity", "LegalObligation", etc.
    pub data_categories: Vec<String>,
    pub expiry_days: Option<i64>, // Optional expiry in days from now
}

#[derive(Debug, Deserialize)]
pub struct RevokeConsentRequest {
    pub customer_id: String,
    pub consent_id: String,
}

#[derive(Debug, Serialize)]
pub struct ConsentResponse {
    pub id: String,
    pub customer_id: String,
    pub purpose: String,
    pub granted: bool,
    pub granted_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub legal_basis: String,
    pub data_categories: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ConsentDashboardResponse {
    pub customer_id: String,
    pub total_consents: i64,
    pub active_consents: i64,
    pub revoked_consents: i64,
    pub consents_by_purpose: std::collections::HashMap<String, i64>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ConsentListResponse {
    pub customer_id: String,
    pub consents: Vec<ConsentResponse>,
    pub total: i64,
}

// ============================================================
// DPIA Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct CreateDpiaRequest {
    pub title: String,
    pub description: String,
    pub processing_activity: String,
    pub risk_assessment: String,
    pub mitigations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDpiaStatusRequest {
    pub status: String, // "Draft", "UnderReview", "Approved", "Rejected"
    pub approved_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DpiaResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub processing_activity: String,
    pub risk_assessment: String,
    pub mitigations: Vec<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct DpiaListResponse {
    pub data: Vec<DpiaResponse>,
    pub total: i64,
}

// ============================================================
// Breach Notification Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct ReportBreachRequest {
    pub breach_type: String,
    pub description: String,
    pub affected_data: Vec<String>,
    pub affected_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct NotifyAuthorityRequest {
    pub breach_id: String,
}

#[derive(Debug, Deserialize)]
pub struct NotifySubjectsRequest {
    pub breach_id: String,
}

#[derive(Debug, Serialize)]
pub struct BreachNotificationResponse {
    pub id: String,
    pub breach_type: String,
    pub description: String,
    pub affected_data: Vec<String>,
    pub affected_count: u32,
    pub detected_at: DateTime<Utc>,
    pub notified_authority_at: Option<DateTime<Utc>>,
    pub notified_subjects_at: Option<DateTime<Utc>>,
    pub status: String,
    pub authority_notified_in_time: bool,
}

#[derive(Debug, Serialize)]
pub struct BreachListResponse {
    pub data: Vec<BreachNotificationResponse>,
    pub total: i64,
}

// ============================================================
// Data Portability & Erasure Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct RequestDataPortabilityRequest {
    pub customer_id: String,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestErasureRequest {
    pub customer_id: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DataPortabilityResponse {
    pub request_id: String,
    pub customer_id: String,
    pub status: String,
    pub requested_at: DateTime<Utc>,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ErasureEligibilityResponse {
    pub customer_id: String,
    pub is_eligible: bool,
    pub reason: String,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ErasureRequestResponse {
    pub request_id: String,
    pub customer_id: String,
    pub status: String,
    pub requested_at: DateTime<Utc>,
    pub scheduled_for: Option<DateTime<Utc>>,
}

// ============================================================
// e-KYC Biometric Verification Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct InitiateEkycRequest {
    pub customer_id: String,
    pub verification_type: String, // "facial_recognition", "fingerprint", "document_scan", "video_call", "fido2_webauthn"
    pub validity_days: Option<i64>, // Default: 90 days
}

#[derive(Debug, Deserialize)]
pub struct CompleteEkycRequest {
    pub verification_id: String,
    pub confidence_score: f64, // 0.0 - 1.0
    pub liveness_passed: bool,
    pub document_type: Option<String>, // "CIN", "Passport", etc.
    pub document_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EkycVerificationResponse {
    pub id: String,
    pub customer_id: String,
    pub verification_type: String,
    pub status: String,
    pub confidence_score: f64,
    pub liveness_check: bool,
    pub document_type: Option<String>,
    pub document_number: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_valid: bool,
}

#[derive(Debug, Serialize)]
pub struct EkycVerificationListResponse {
    pub data: Vec<EkycVerificationResponse>,
    pub total: i64,
    pub customer_id: String,
}

#[derive(Debug, Serialize)]
pub struct EkycStatusResponse {
    pub customer_id: String,
    pub is_ekyc_verified: bool,
    pub latest_verification: Option<EkycVerificationResponse>,
    pub all_verifications: Vec<EkycVerificationResponse>,
}
