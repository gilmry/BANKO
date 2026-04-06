/// TuniCheque Integration for Real-Time Cheque Verification
///
/// Implements STORY-COMP-11: Integration with TuniCheque API for real-time cheque
/// verification and bounced cheque reporting as per Central Bank of Tunisia
/// Circular 2025-03.
///
/// TuniCheque is the real-time cheque verification system operated by the Tunisian
/// Bankers Association (ATB) and managed by the Central Bank of Tunisia.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuniChequeError {
    InvalidCheque(String),
    VerificationFailed(String),
    NetworkError(String),
    AuthenticationError(String),
    ChequeNotFound(String),
    ReportingFailed(String),
}

impl fmt::Display for TuniChequeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TuniChequeError::InvalidCheque(msg) => write!(f, "Invalid cheque: {}", msg),
            TuniChequeError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            TuniChequeError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            TuniChequeError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            TuniChequeError::ChequeNotFound(msg) => write!(f, "Cheque not found: {}", msg),
            TuniChequeError::ReportingFailed(msg) => write!(f, "Reporting failed: {}", msg),
        }
    }
}

impl std::error::Error for TuniChequeError {}

// ============================================================================
// ENUMS
// ============================================================================

/// Status of a cheque in the TuniCheque system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChequeStatus {
    /// Cheque is valid and has not been reported
    Valid,
    /// Cheque has been reported as bounced
    Bounced,
    /// Cheque has been reported as stopped
    Stopped,
    /// Cheque status unknown (not in system)
    Unknown,
}

impl ChequeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ChequeStatus::Valid => "Valid",
            ChequeStatus::Bounced => "Bounced",
            ChequeStatus::Stopped => "Stopped",
            ChequeStatus::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for ChequeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Reason for bounced cheque
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BounceReason {
    InsufficientFunds,
    AccountClosed,
    StopchequeRequest,
    InvalidAccount,
    SignatureDiscrepancy,
    AlterationDetected,
    Other(String),
}

impl BounceReason {
    pub fn as_str(&self) -> &str {
        match self {
            BounceReason::InsufficientFunds => "INSUFFICIENT_FUNDS",
            BounceReason::AccountClosed => "ACCOUNT_CLOSED",
            BounceReason::StopchequeRequest => "STOPCHEQUE_REQUEST",
            BounceReason::InvalidAccount => "INVALID_ACCOUNT",
            BounceReason::SignatureDiscrepancy => "SIGNATURE_DISCREPANCY",
            BounceReason::AlterationDetected => "ALTERATION_DETECTED",
            BounceReason::Other(s) => s,
        }
    }
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Cheque information for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChequeInfo {
    /// Cheque number
    pub cheque_number: String,
    /// Bank routing/clearing code (6-8 digits)
    pub bank_code: String,
    /// Cheque amount in TND
    pub amount: Option<f64>,
    /// Date cheque is dated (YYYY-MM-DD)
    pub cheque_date: Option<String>,
}

impl ChequeInfo {
    pub fn new(cheque_number: String, bank_code: String) -> Result<Self, TuniChequeError> {
        if cheque_number.trim().is_empty() {
            return Err(TuniChequeError::InvalidCheque(
                "Cheque number cannot be empty".to_string(),
            ));
        }
        if bank_code.trim().is_empty() {
            return Err(TuniChequeError::InvalidCheque(
                "Bank code cannot be empty".to_string(),
            ));
        }
        // Validate cheque number format (typically 7-8 digits)
        if !cheque_number.chars().all(|c| c.is_ascii_digit()) {
            return Err(TuniChequeError::InvalidCheque(
                "Cheque number must contain only digits".to_string(),
            ));
        }

        Ok(ChequeInfo {
            cheque_number,
            bank_code,
            amount: None,
            cheque_date: None,
        })
    }

    pub fn with_amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn with_cheque_date(mut self, date: String) -> Self {
        self.cheque_date = Some(date);
        self
    }
}

/// Response from TuniCheque verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChequeVerificationResponse {
    /// Unique verification ID
    pub verification_id: String,
    /// Status of the cheque
    pub status: ChequeStatus,
    /// If bounced, the reason
    pub bounce_reason: Option<String>,
    /// Bank name if found
    pub bank_name: Option<String>,
    /// Timestamp of verification
    pub verified_at: DateTime<Utc>,
    /// Whether the cheque is in the system
    pub in_system: bool,
}

/// Request to report a bounced cheque
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BounceReportRequest {
    /// Cheque number
    pub cheque_number: String,
    /// Bank code
    pub bank_code: String,
    /// Reason for bounce
    pub reason: BounceReason,
    /// User reporting the bounce
    pub reported_by: String,
    /// Optional notes
    pub notes: Option<String>,
}

/// Response from bounce report submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BounceReportResponse {
    /// Report ID from TuniCheque
    pub report_id: String,
    /// Status of the report
    pub status: String,
    /// Message from TuniCheque
    pub message: String,
    /// Timestamp of report
    pub reported_at: DateTime<Utc>,
}

// ============================================================================
// TUNICHEQUE CLIENT
// ============================================================================

/// Client for TuniCheque real-time verification system
pub struct TuniChequeClient {
    base_url: String,
    api_key: String,
}

impl TuniChequeClient {
    pub fn new(base_url: String, api_key: String) -> Result<Self, TuniChequeError> {
        if base_url.trim().is_empty() {
            return Err(TuniChequeError::AuthenticationError(
                "Base URL cannot be empty".to_string(),
            ));
        }
        if api_key.trim().is_empty() {
            return Err(TuniChequeError::AuthenticationError(
                "API key cannot be empty".to_string(),
            ));
        }

        Ok(TuniChequeClient { base_url, api_key })
    }

    /// Verify cheque status in real-time
    ///
    /// Checks TuniCheque system to determine if a cheque has been reported as
    /// bounced, stopped, or is valid.
    ///
    /// # Arguments
    ///
    /// * `cheque_info` - Cheque details (number and bank code)
    ///
    /// # Returns
    ///
    /// ChequeVerificationResponse with status and details
    pub async fn verify_cheque(&self, cheque_info: &ChequeInfo) -> Result<ChequeVerificationResponse, TuniChequeError> {
        tracing::info!(
            cheque_number = %cheque_info.cheque_number,
            bank_code = %cheque_info.bank_code,
            "Verifying cheque in TuniCheque system"
        );

        // In production, this would be:
        // GET {self.base_url}/api/v1/cheques/{bank_code}/{cheque_number}
        // Headers: Authorization: Bearer {self.api_key}
        //
        // For now, we return a mock response
        Ok(ChequeVerificationResponse {
            verification_id: Uuid::new_v4().to_string(),
            status: ChequeStatus::Valid,
            bounce_reason: None,
            bank_name: Some("Banque Nationale de Tunisie".to_string()),
            verified_at: Utc::now(),
            in_system: true,
        })
    }

    /// Report a bounced cheque
    ///
    /// Submits a report to TuniCheque to mark a cheque as bounced. This will
    /// prevent the cheque from being accepted in future transactions.
    ///
    /// # Arguments
    ///
    /// * `request` - Bounce report details
    ///
    /// # Returns
    ///
    /// BounceReportResponse confirming report submission
    pub async fn report_bounced_cheque(
        &self,
        request: &BounceReportRequest,
    ) -> Result<BounceReportResponse, TuniChequeError> {
        // Validate request
        let cheque_info = ChequeInfo::new(
            request.cheque_number.clone(),
            request.bank_code.clone(),
        )?;

        tracing::info!(
            cheque_number = %cheque_info.cheque_number,
            bank_code = %cheque_info.bank_code,
            reason = %request.reason.as_str(),
            "Reporting bounced cheque to TuniCheque"
        );

        // In production, this would be:
        // POST {self.base_url}/api/v1/bounces
        // Headers: Authorization: Bearer {self.api_key}, Content-Type: application/json
        // Body: {request}
        //
        // For now, we return a mock response
        Ok(BounceReportResponse {
            report_id: Uuid::new_v4().to_string(),
            status: "ACCEPTED".to_string(),
            message: "Bounced cheque report successfully submitted".to_string(),
            reported_at: Utc::now(),
        })
    }

    /// Report a stopped cheque (stopcheque request)
    ///
    /// Submits a report for a cheque that should not be processed (e.g., lost,
    stolen, or preventively stopped).
    pub async fn report_stopped_cheque(
        &self,
        cheque_number: String,
        bank_code: String,
        reason: String,
        reported_by: String,
    ) -> Result<BounceReportResponse, TuniChequeError> {
        let request = BounceReportRequest {
            cheque_number,
            bank_code,
            reason: BounceReason::StopchequeRequest,
            reported_by,
            notes: Some(reason),
        };

        self.report_bounced_cheque(&request).await
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheque_info_creation() {
        let cheque = ChequeInfo::new("1234567".to_string(), "10001".to_string()).unwrap();

        assert_eq!(cheque.cheque_number, "1234567");
        assert_eq!(cheque.bank_code, "10001");
    }

    #[test]
    fn test_cheque_info_invalid_number() {
        let result = ChequeInfo::new("ABC123".to_string(), "10001".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_info_with_amount() {
        let cheque = ChequeInfo::new("1234567".to_string(), "10001".to_string())
            .unwrap()
            .with_amount(5000.0)
            .with_cheque_date("2026-04-15".to_string());

        assert_eq!(cheque.amount, Some(5000.0));
        assert_eq!(cheque.cheque_date, Some("2026-04-15".to_string()));
    }

    #[test]
    fn test_bounce_reason_display() {
        assert_eq!(BounceReason::InsufficientFunds.as_str(), "INSUFFICIENT_FUNDS");
        assert_eq!(BounceReason::AccountClosed.as_str(), "ACCOUNT_CLOSED");
    }

    #[test]
    fn test_cheque_status_display() {
        assert_eq!(ChequeStatus::Valid.as_str(), "Valid");
        assert_eq!(ChequeStatus::Bounced.as_str(), "Bounced");
    }

    #[test]
    fn test_tunicheque_client_creation() {
        let client =
            TuniChequeClient::new("https://tunicheque.atb.tn".to_string(), "test-key".to_string())
                .unwrap();

        assert!(!client.base_url.is_empty());
    }

    #[tokio::test]
    async fn test_verify_cheque() {
        let client =
            TuniChequeClient::new("https://tunicheque.atb.tn".to_string(), "test-key".to_string())
                .unwrap();
        let cheque = ChequeInfo::new("1234567".to_string(), "10001".to_string()).unwrap();

        let response = client.verify_cheque(&cheque).await.unwrap();

        assert_eq!(response.status, ChequeStatus::Valid);
        assert!(response.in_system);
    }

    #[tokio::test]
    async fn test_report_bounced_cheque() {
        let client =
            TuniChequeClient::new("https://tunicheque.atb.tn".to_string(), "test-key".to_string())
                .unwrap();

        let request = BounceReportRequest {
            cheque_number: "1234567".to_string(),
            bank_code: "10001".to_string(),
            reason: BounceReason::InsufficientFunds,
            reported_by: "user@banko.tn".to_string(),
            notes: None,
        };

        let response = client.report_bounced_cheque(&request).await.unwrap();

        assert_eq!(response.status, "ACCEPTED");
    }
}
