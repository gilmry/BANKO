use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- ConsentId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConsentId(Uuid);

impl ConsentId {
    pub fn new() -> Self {
        ConsentId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ConsentId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for ConsentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- ConsentPurpose ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentPurpose {
    DataProcessing,
    Marketing,
    ThirdPartySharing,
    Profiling,
    Analytics,
}

impl ConsentPurpose {
    pub fn from_str_purpose(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "dataprocessing" | "data_processing" => Ok(ConsentPurpose::DataProcessing),
            "marketing" => Ok(ConsentPurpose::Marketing),
            "thirdpartysharing" | "third_party_sharing" => Ok(ConsentPurpose::ThirdPartySharing),
            "profiling" => Ok(ConsentPurpose::Profiling),
            "analytics" => Ok(ConsentPurpose::Analytics),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown consent purpose: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConsentPurpose::DataProcessing => "DataProcessing",
            ConsentPurpose::Marketing => "Marketing",
            ConsentPurpose::ThirdPartySharing => "ThirdPartySharing",
            ConsentPurpose::Profiling => "Profiling",
            ConsentPurpose::Analytics => "Analytics",
        }
    }
}

impl fmt::Display for ConsentPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ConsentRecordStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentRecordStatus {
    Active,
    Revoked,
}

impl ConsentRecordStatus {
    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "active" => Ok(ConsentRecordStatus::Active),
            "revoked" => Ok(ConsentRecordStatus::Revoked),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown consent record status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConsentRecordStatus::Active => "Active",
            ConsentRecordStatus::Revoked => "Revoked",
        }
    }
}

impl fmt::Display for ConsentRecordStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ConsentRecord ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    consent_id: ConsentId,
    customer_id: Uuid,
    purpose: ConsentPurpose,
    status: ConsentRecordStatus,
    granted_at: DateTime<Utc>,
    revoked_at: Option<DateTime<Utc>>,
}

impl ConsentRecord {
    /// Grant a new consent for a customer and purpose.
    pub fn grant(customer_id: Uuid, purpose: ConsentPurpose) -> Self {
        ConsentRecord {
            consent_id: ConsentId::new(),
            customer_id,
            purpose,
            status: ConsentRecordStatus::Active,
            granted_at: Utc::now(),
            revoked_at: None,
        }
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(
        consent_id: ConsentId,
        customer_id: Uuid,
        purpose: ConsentPurpose,
        status: ConsentRecordStatus,
        granted_at: DateTime<Utc>,
        revoked_at: Option<DateTime<Utc>>,
    ) -> Self {
        ConsentRecord {
            consent_id,
            customer_id,
            purpose,
            status,
            granted_at,
            revoked_at,
        }
    }

    /// Revoke this consent. Only active consents can be revoked.
    pub fn revoke(&mut self) -> Result<(), DomainError> {
        if self.status == ConsentRecordStatus::Revoked {
            return Err(DomainError::ConsentNotFound);
        }
        self.status = ConsentRecordStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        Ok(())
    }

    /// Check if this consent is currently active.
    pub fn is_active(&self) -> bool {
        self.status == ConsentRecordStatus::Active
    }

    // --- Getters ---

    pub fn consent_id(&self) -> &ConsentId {
        &self.consent_id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn purpose(&self) -> ConsentPurpose {
        self.purpose
    }

    pub fn status(&self) -> ConsentRecordStatus {
        self.status
    }

    pub fn granted_at(&self) -> DateTime<Utc> {
        self.granted_at
    }

    pub fn revoked_at(&self) -> Option<DateTime<Utc>> {
        self.revoked_at
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_customer_id() -> Uuid {
        Uuid::new_v4()
    }

    #[test]
    fn test_consent_grant() {
        let cid = sample_customer_id();
        let consent = ConsentRecord::grant(cid, ConsentPurpose::DataProcessing);
        assert!(consent.is_active());
        assert_eq!(consent.customer_id(), cid);
        assert_eq!(consent.purpose(), ConsentPurpose::DataProcessing);
        assert_eq!(consent.status(), ConsentRecordStatus::Active);
        assert!(consent.revoked_at().is_none());
    }

    #[test]
    fn test_consent_revoke() {
        let cid = sample_customer_id();
        let mut consent = ConsentRecord::grant(cid, ConsentPurpose::Marketing);
        assert!(consent.is_active());

        let result = consent.revoke();
        assert!(result.is_ok());
        assert!(!consent.is_active());
        assert_eq!(consent.status(), ConsentRecordStatus::Revoked);
        assert!(consent.revoked_at().is_some());
    }

    #[test]
    fn test_consent_revoke_already_revoked() {
        let cid = sample_customer_id();
        let mut consent = ConsentRecord::grant(cid, ConsentPurpose::Analytics);
        consent.revoke().unwrap();

        let result = consent.revoke();
        assert!(result.is_err());
    }

    #[test]
    fn test_consent_purpose_from_str() {
        assert_eq!(
            ConsentPurpose::from_str_purpose("DataProcessing").unwrap(),
            ConsentPurpose::DataProcessing
        );
        assert_eq!(
            ConsentPurpose::from_str_purpose("marketing").unwrap(),
            ConsentPurpose::Marketing
        );
        assert_eq!(
            ConsentPurpose::from_str_purpose("third_party_sharing").unwrap(),
            ConsentPurpose::ThirdPartySharing
        );
        assert_eq!(
            ConsentPurpose::from_str_purpose("profiling").unwrap(),
            ConsentPurpose::Profiling
        );
        assert_eq!(
            ConsentPurpose::from_str_purpose("analytics").unwrap(),
            ConsentPurpose::Analytics
        );
        assert!(ConsentPurpose::from_str_purpose("invalid").is_err());
    }

    #[test]
    fn test_consent_record_status_from_str() {
        assert_eq!(
            ConsentRecordStatus::from_str_status("Active").unwrap(),
            ConsentRecordStatus::Active
        );
        assert_eq!(
            ConsentRecordStatus::from_str_status("revoked").unwrap(),
            ConsentRecordStatus::Revoked
        );
        assert!(ConsentRecordStatus::from_str_status("unknown").is_err());
    }

    #[test]
    fn test_consent_id_display() {
        let id = ConsentId::new();
        let display = format!("{id}");
        assert!(!display.is_empty());
    }

    #[test]
    fn test_consent_reconstitute() {
        let cid = sample_customer_id();
        let consent_id = ConsentId::new();
        let now = Utc::now();
        let consent = ConsentRecord::reconstitute(
            consent_id.clone(),
            cid,
            ConsentPurpose::Profiling,
            ConsentRecordStatus::Active,
            now,
            None,
        );
        assert_eq!(consent.consent_id(), &consent_id);
        assert!(consent.is_active());
    }
}
