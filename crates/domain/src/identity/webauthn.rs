use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use super::UserId;

/// WebAuthn credential ID (base64url encoded)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CredentialId(String);

impl CredentialId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::InvalidInput(
                "Credential ID cannot be empty".to_string(),
            ));
        }
        Ok(CredentialId(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// WebAuthn attestation format (typically "none", "direct", or "indirect")
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttestationFormat {
    None,
    Direct,
    Indirect,
    Enterprise,
}

impl AttestationFormat {
    pub fn as_str(&self) -> &str {
        match self {
            AttestationFormat::None => "none",
            AttestationFormat::Direct => "direct",
            AttestationFormat::Indirect => "indirect",
            AttestationFormat::Enterprise => "enterprise",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "none" => Ok(AttestationFormat::None),
            "direct" => Ok(AttestationFormat::Direct),
            "indirect" => Ok(AttestationFormat::Indirect),
            "enterprise" => Ok(AttestationFormat::Enterprise),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown attestation format: {s}"
            ))),
        }
    }
}

/// WebAuthn transports (e.g., usb, ble, nfc, internal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transport {
    Usb,
    Ble,
    Nfc,
    Internal,
    Hybrid,
}

impl Transport {
    pub fn as_str(&self) -> &str {
        match self {
            Transport::Usb => "usb",
            Transport::Ble => "ble",
            Transport::Nfc => "nfc",
            Transport::Internal => "internal",
            Transport::Hybrid => "hybrid",
        }
    }
}

/// WebAuthn credential — represents a registered FIDO2 key or biometric
/// FR-154: FIDO2/WebAuthn for e-KYC biometric authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    id: Uuid,
    user_id: UserId,
    credential_id: CredentialId,
    public_key: Vec<u8>,
    counter: u32,
    transports: Vec<Transport>,
    attestation_format: AttestationFormat,
    aaguid: Option<Vec<u8>>,
    name: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
}

impl WebAuthnCredential {
    /// Register a new WebAuthn credential
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: UserId,
        credential_id: CredentialId,
        public_key: Vec<u8>,
        counter: u32,
        transports: Vec<Transport>,
        attestation_format: AttestationFormat,
        name: String,
    ) -> Result<Self, DomainError> {
        if public_key.is_empty() {
            return Err(DomainError::InvalidInput(
                "Public key cannot be empty".to_string(),
            ));
        }
        if name.is_empty() {
            return Err(DomainError::InvalidInput(
                "Credential name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(WebAuthnCredential {
            id: Uuid::new_v4(),
            user_id,
            credential_id,
            public_key,
            counter,
            transports,
            attestation_format,
            aaguid: None,
            name,
            is_active: true,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        })
    }

    /// Reconstitute from stored data
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        user_id: UserId,
        credential_id: CredentialId,
        public_key: Vec<u8>,
        counter: u32,
        transports: Vec<Transport>,
        attestation_format: AttestationFormat,
        aaguid: Option<Vec<u8>>,
        name: String,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        last_used_at: Option<DateTime<Utc>>,
    ) -> Self {
        WebAuthnCredential {
            id,
            user_id,
            credential_id,
            public_key,
            counter,
            transports,
            attestation_format,
            aaguid,
            name,
            is_active,
            created_at,
            updated_at,
            last_used_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn credential_id(&self) -> &CredentialId {
        &self.credential_id
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn counter(&self) -> u32 {
        self.counter
    }

    pub fn transports(&self) -> &[Transport] {
        &self.transports
    }

    pub fn attestation_format(&self) -> AttestationFormat {
        self.attestation_format
    }

    pub fn aaguid(&self) -> Option<&[u8]> {
        self.aaguid.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn last_used_at(&self) -> Option<DateTime<Utc>> {
        self.last_used_at
    }

    // --- Domain behavior ---

    /// Update counter after successful authentication
    pub fn increment_counter(&mut self, new_counter: u32) -> Result<(), DomainError> {
        if new_counter <= self.counter {
            return Err(DomainError::InvalidInput(
                "Counter must be strictly increasing (cloned authenticator detection)".to_string(),
            ));
        }
        self.counter = new_counter;
        self.last_used_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Deactivate credential (revoke it)
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Reactivate credential
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Rename credential
    pub fn rename(&mut self, new_name: String) -> Result<(), DomainError> {
        if new_name.is_empty() {
            return Err(DomainError::InvalidInput(
                "Credential name cannot be empty".to_string(),
            ));
        }
        self.name = new_name;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Set AAGUID (Authenticator AAGUID)
    pub fn set_aaguid(&mut self, aaguid: Vec<u8>) {
        self.aaguid = Some(aaguid);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_credential_id() -> CredentialId {
        CredentialId::new("dGVzdF9jcmVkZW50aWFsX2lk".to_string()).unwrap()
    }

    fn valid_public_key() -> Vec<u8> {
        vec![0x04, 0xaa, 0xbb, 0xcc, 0xdd]
    }

    #[test]
    fn test_credential_id_new_valid() {
        let cred_id = CredentialId::new("valid_id".to_string()).unwrap();
        assert_eq!(cred_id.as_str(), "valid_id");
    }

    #[test]
    fn test_credential_id_empty_rejected() {
        let result = CredentialId::new(String::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_attestation_format_from_str() {
        assert_eq!(
            AttestationFormat::from_str("none").unwrap(),
            AttestationFormat::None
        );
        assert_eq!(
            AttestationFormat::from_str("direct").unwrap(),
            AttestationFormat::Direct
        );
        assert_eq!(
            AttestationFormat::from_str("indirect").unwrap(),
            AttestationFormat::Indirect
        );
    }

    #[test]
    fn test_attestation_format_invalid() {
        assert!(AttestationFormat::from_str("invalid").is_err());
    }

    #[test]
    fn test_webauthn_credential_new() {
        let user_id = UserId::new();
        let credential = WebAuthnCredential::new(
            user_id.clone(),
            valid_credential_id(),
            valid_public_key(),
            0,
            vec![Transport::Usb, Transport::Ble],
            AttestationFormat::Direct,
            "My Security Key".to_string(),
        )
        .unwrap();

        assert_eq!(credential.user_id(), &user_id);
        assert_eq!(credential.counter(), 0);
        assert!(credential.is_active());
        assert_eq!(credential.name(), "My Security Key");
        assert_eq!(credential.transports().len(), 2);
    }

    #[test]
    fn test_webauthn_credential_empty_public_key_rejected() {
        let user_id = UserId::new();
        let result = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            vec![],
            0,
            vec![],
            AttestationFormat::None,
            "Key".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_webauthn_credential_empty_name_rejected() {
        let user_id = UserId::new();
        let result = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            valid_public_key(),
            0,
            vec![],
            AttestationFormat::None,
            String::new(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_increment_counter_success() {
        let user_id = UserId::new();
        let mut credential = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            valid_public_key(),
            5,
            vec![],
            AttestationFormat::None,
            "Key".to_string(),
        )
        .unwrap();

        assert!(credential.increment_counter(6).is_ok());
        assert_eq!(credential.counter(), 6);
        assert!(credential.last_used_at().is_some());
    }

    #[test]
    fn test_increment_counter_non_increasing_rejected() {
        let user_id = UserId::new();
        let mut credential = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            valid_public_key(),
            10,
            vec![],
            AttestationFormat::None,
            "Key".to_string(),
        )
        .unwrap();

        assert!(credential.increment_counter(10).is_err());
        assert!(credential.increment_counter(5).is_err());
    }

    #[test]
    fn test_deactivate_and_reactivate() {
        let user_id = UserId::new();
        let mut credential = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            valid_public_key(),
            0,
            vec![],
            AttestationFormat::None,
            "Key".to_string(),
        )
        .unwrap();

        assert!(credential.is_active());
        credential.deactivate();
        assert!(!credential.is_active());
        credential.activate();
        assert!(credential.is_active());
    }

    #[test]
    fn test_rename_credential() {
        let user_id = UserId::new();
        let mut credential = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            valid_public_key(),
            0,
            vec![],
            AttestationFormat::None,
            "Old Name".to_string(),
        )
        .unwrap();

        credential.rename("New Name".to_string()).unwrap();
        assert_eq!(credential.name(), "New Name");
    }

    #[test]
    fn test_rename_to_empty_rejected() {
        let user_id = UserId::new();
        let mut credential = WebAuthnCredential::new(
            user_id,
            valid_credential_id(),
            valid_public_key(),
            0,
            vec![],
            AttestationFormat::None,
            "Name".to_string(),
        )
        .unwrap();

        assert!(credential.rename(String::new()).is_err());
    }

    #[test]
    fn test_reconstitute() {
        let id = Uuid::new_v4();
        let user_id = UserId::new();
        let cred_id = valid_credential_id();
        let now = Utc::now();
        let credential = WebAuthnCredential::reconstitute(
            id,
            user_id.clone(),
            cred_id.clone(),
            valid_public_key(),
            42,
            vec![Transport::Internal],
            AttestationFormat::None,
            None,
            "Reconstituted".to_string(),
            true,
            now,
            now,
            Some(now),
        );

        assert_eq!(credential.id(), id);
        assert_eq!(credential.user_id(), &user_id);
        assert_eq!(credential.counter(), 42);
    }
}
