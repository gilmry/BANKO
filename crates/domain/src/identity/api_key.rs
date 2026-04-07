use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use super::UserId;

/// API key hash (never store plain keys, only hashes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKeyHash(String);

impl ApiKeyHash {
    pub fn new(hash: String) -> Result<Self, DomainError> {
        if hash.is_empty() {
            return Err(DomainError::InvalidInput(
                "API key hash cannot be empty".to_string(),
            ));
        }
        if hash.len() < 20 {
            return Err(DomainError::InvalidInput(
                "API key hash is too short".to_string(),
            ));
        }
        Ok(ApiKeyHash(hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// API key scope (e.g., "payments:read", "accounts:write")
/// FR-161: API keys management for integrations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiKeyScope(String);

impl ApiKeyScope {
    pub fn new(scope: String) -> Result<Self, DomainError> {
        if scope.is_empty() {
            return Err(DomainError::InvalidInput(
                "Scope cannot be empty".to_string(),
            ));
        }
        // Basic validation: must match pattern "resource:action"
        if !scope.contains(':') {
            return Err(DomainError::InvalidInput(
                "Scope must be in format 'resource:action'".to_string(),
            ));
        }
        Ok(ApiKeyScope(scope))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this scope grants access to a required scope
    pub fn grants(&self, required: &str) -> bool {
        // Exact match
        if self.0 == required {
            return true;
        }
        // Master scope "*:*" grants all access
        if self.0 == "*:*" {
            return true;
        }
        // Wildcard: "resource:*" grants access to all actions under resource
        if self.0.ends_with(":*") {
            let resource = &self.0[..self.0.len() - 2];
            return required.starts_with(&format!("{resource}:"));
        }
        false
    }
}

/// API key aggregate
/// FR-161: API keys management with scopes, expiry, and rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    id: Uuid,
    user_id: UserId,
    key_hash: ApiKeyHash,
    scopes: Vec<ApiKeyScope>,
    name: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    last_used_at: Option<DateTime<Utc>>,
    rotated_from: Option<Uuid>,
}

impl ApiKey {
    /// Create a new API key
    pub fn new(
        user_id: UserId,
        key_hash: ApiKeyHash,
        scopes: Vec<ApiKeyScope>,
        name: String,
    ) -> Result<Self, DomainError> {
        if scopes.is_empty() {
            return Err(DomainError::InvalidInput(
                "API key must have at least one scope".to_string(),
            ));
        }
        if name.is_empty() {
            return Err(DomainError::InvalidInput(
                "API key name cannot be empty".to_string(),
            ));
        }

        Ok(ApiKey {
            id: Uuid::new_v4(),
            user_id,
            key_hash,
            scopes,
            name,
            is_active: true,
            created_at: Utc::now(),
            expires_at: None,
            last_used_at: None,
            rotated_from: None,
        })
    }

    /// Create a new API key with expiry
    pub fn new_with_expiry(
        user_id: UserId,
        key_hash: ApiKeyHash,
        scopes: Vec<ApiKeyScope>,
        name: String,
        ttl_days: u32,
    ) -> Result<Self, DomainError> {
        if ttl_days == 0 {
            return Err(DomainError::InvalidInput(
                "TTL must be at least 1 day".to_string(),
            ));
        }

        let mut key = Self::new(user_id, key_hash, scopes, name)?;
        key.expires_at = Some(Utc::now() + Duration::days(ttl_days as i64));
        Ok(key)
    }

    /// Reconstitute from stored data
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        user_id: UserId,
        key_hash: ApiKeyHash,
        scopes: Vec<ApiKeyScope>,
        name: String,
        is_active: bool,
        created_at: DateTime<Utc>,
        expires_at: Option<DateTime<Utc>>,
        last_used_at: Option<DateTime<Utc>>,
        rotated_from: Option<Uuid>,
    ) -> Self {
        ApiKey {
            id,
            user_id,
            key_hash,
            scopes,
            name,
            is_active,
            created_at,
            expires_at,
            last_used_at,
            rotated_from,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn key_hash(&self) -> &ApiKeyHash {
        &self.key_hash
    }

    pub fn scopes(&self) -> &[ApiKeyScope] {
        &self.scopes
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

    pub fn expires_at(&self) -> Option<DateTime<Utc>> {
        self.expires_at
    }

    pub fn last_used_at(&self) -> Option<DateTime<Utc>> {
        self.last_used_at
    }

    pub fn rotated_from(&self) -> Option<Uuid> {
        self.rotated_from
    }

    // --- Domain behavior ---

    /// Check if the key is still valid (active and not expired)
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }
        if let Some(exp) = self.expires_at {
            if Utc::now() > exp {
                return false;
            }
        }
        true
    }

    /// Check if key has expired
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            Utc::now() > exp
        } else {
            false
        }
    }

    /// Check if key grants the required scope
    pub fn has_scope(&self, required_scope: &str) -> bool {
        self.scopes.iter().any(|scope| scope.grants(required_scope))
    }

    /// Check if key has all required scopes
    pub fn has_all_scopes(&self, required_scopes: &[&str]) -> bool {
        required_scopes
            .iter()
            .all(|req| self.has_scope(req))
    }

    /// Record a successful use of the API key
    pub fn record_use(&mut self) {
        self.last_used_at = Some(Utc::now());
    }

    /// Revoke (deactivate) the API key
    pub fn revoke(&mut self) {
        self.is_active = false;
    }

    /// Re-enable a revoked API key
    pub fn unrevoke(&mut self) {
        self.is_active = true;
    }

    /// Rotate the API key (create a new hash, mark old rotated_from)
    pub fn rotate(&mut self, new_key_hash: ApiKeyHash) {
        let old_id = self.id;
        self.id = Uuid::new_v4();
        self.key_hash = new_key_hash;
        self.rotated_from = Some(old_id);
        self.last_used_at = None;
    }

    /// Extend expiry by additional days
    pub fn extend_expiry(&mut self, additional_days: u32) -> Result<(), DomainError> {
        if additional_days == 0 {
            return Err(DomainError::InvalidInput(
                "Must extend by at least 1 day".to_string(),
            ));
        }

        let current_exp = self.expires_at.unwrap_or(Utc::now());
        self.expires_at = Some(current_exp + Duration::days(additional_days as i64));
        Ok(())
    }

    /// Set expiry to None (make it non-expiring)
    pub fn make_permanent(&mut self) {
        self.expires_at = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_hash() -> ApiKeyHash {
        ApiKeyHash::new(
            "$2b$12$LJ3m4ys3Lg2HEOjdLNRsWuBNRZLJDhG5JQqJK9qJKj3K4hNqXKwu".to_string(),
        )
        .unwrap()
    }

    fn valid_scopes() -> Vec<ApiKeyScope> {
        vec![ApiKeyScope::new("payments:read".to_string()).unwrap()]
    }

    #[test]
    fn test_api_key_scope_valid() {
        let scope = ApiKeyScope::new("accounts:read".to_string()).unwrap();
        assert_eq!(scope.as_str(), "accounts:read");
    }

    #[test]
    fn test_api_key_scope_empty_rejected() {
        assert!(ApiKeyScope::new(String::new()).is_err());
    }

    #[test]
    fn test_api_key_scope_no_colon_rejected() {
        assert!(ApiKeyScope::new("invalidscope".to_string()).is_err());
    }

    #[test]
    fn test_api_key_scope_grants_exact_match() {
        let scope = ApiKeyScope::new("payments:read".to_string()).unwrap();
        assert!(scope.grants("payments:read"));
    }

    #[test]
    fn test_api_key_scope_grants_wildcard() {
        let scope = ApiKeyScope::new("payments:*".to_string()).unwrap();
        assert!(scope.grants("payments:read"));
        assert!(scope.grants("payments:write"));
        assert!(!scope.grants("accounts:read"));
    }

    #[test]
    fn test_api_key_scope_grants_master_wildcard() {
        let scope = ApiKeyScope::new("*:*".to_string()).unwrap();
        assert!(scope.grants("anything:anywhere"));
        assert!(scope.grants("payments:read"));
    }

    #[test]
    fn test_api_key_new() {
        let user_id = UserId::new();
        let api_key = ApiKey::new(
            user_id.clone(),
            valid_hash(),
            valid_scopes(),
            "MyKey".to_string(),
        )
        .unwrap();

        assert_eq!(api_key.user_id(), &user_id);
        assert!(api_key.is_active());
        assert!(!api_key.is_expired());
        assert!(api_key.is_valid());
        assert_eq!(api_key.expires_at(), None);
    }

    #[test]
    fn test_api_key_new_empty_scopes_rejected() {
        let user_id = UserId::new();
        let result = ApiKey::new(
            user_id,
            valid_hash(),
            vec![],
            "MyKey".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_new_empty_name_rejected() {
        let user_id = UserId::new();
        let result = ApiKey::new(
            user_id,
            valid_hash(),
            valid_scopes(),
            String::new(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_with_expiry() {
        let user_id = UserId::new();
        let api_key = ApiKey::new_with_expiry(
            user_id,
            valid_hash(),
            valid_scopes(),
            "ExpiredKey".to_string(),
            30,
        )
        .unwrap();

        assert!(api_key.expires_at().is_some());
        assert!(!api_key.is_expired());
    }

    #[test]
    fn test_api_key_with_zero_ttl_rejected() {
        let user_id = UserId::new();
        let result = ApiKey::new_with_expiry(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Bad".to_string(),
            0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_has_scope() {
        let user_id = UserId::new();
        let api_key = ApiKey::new(
            user_id,
            valid_hash(),
            vec![
                ApiKeyScope::new("payments:read".to_string()).unwrap(),
                ApiKeyScope::new("accounts:*".to_string()).unwrap(),
            ],
            "MultiScope".to_string(),
        )
        .unwrap();

        assert!(api_key.has_scope("payments:read"));
        assert!(api_key.has_scope("accounts:read"));
        assert!(api_key.has_scope("accounts:write"));
        assert!(!api_key.has_scope("sanctions:read"));
    }

    #[test]
    fn test_api_key_has_all_scopes() {
        let user_id = UserId::new();
        let api_key = ApiKey::new(
            user_id,
            valid_hash(),
            vec![ApiKeyScope::new("*:*".to_string()).unwrap()],
            "Master".to_string(),
        )
        .unwrap();

        assert!(api_key.has_all_scopes(&["payments:read", "accounts:write"]));
    }

    #[test]
    fn test_api_key_record_use() {
        let user_id = UserId::new();
        let mut api_key = ApiKey::new(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Key".to_string(),
        )
        .unwrap();

        assert!(api_key.last_used_at().is_none());
        api_key.record_use();
        assert!(api_key.last_used_at().is_some());
    }

    #[test]
    fn test_api_key_revoke_and_unrevoke() {
        let user_id = UserId::new();
        let mut api_key = ApiKey::new(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Key".to_string(),
        )
        .unwrap();

        assert!(api_key.is_valid());
        api_key.revoke();
        assert!(!api_key.is_valid());
        api_key.unrevoke();
        assert!(api_key.is_valid());
    }

    #[test]
    fn test_api_key_rotate() {
        let user_id = UserId::new();
        let mut api_key = ApiKey::new(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Key".to_string(),
        )
        .unwrap();

        let old_id = api_key.id();
        let new_hash = ApiKeyHash::new(
            "$2b$12$NEWHASHNEWHASHNEWHASHNEWHASHNEWHASHNEWHASHNEWHASHNEW".to_string(),
        )
        .unwrap();

        api_key.rotate(new_hash);

        assert_ne!(api_key.id(), old_id);
        assert_eq!(api_key.rotated_from(), Some(old_id));
        assert!(api_key.last_used_at().is_none());
    }

    #[test]
    fn test_api_key_extend_expiry() {
        let user_id = UserId::new();
        let mut api_key = ApiKey::new_with_expiry(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Key".to_string(),
            10,
        )
        .unwrap();

        let original_exp = api_key.expires_at().unwrap();
        api_key.extend_expiry(5).unwrap();
        let new_exp = api_key.expires_at().unwrap();

        assert!(new_exp > original_exp);
    }

    #[test]
    fn test_api_key_extend_expiry_zero_rejected() {
        let user_id = UserId::new();
        let mut api_key = ApiKey::new_with_expiry(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Key".to_string(),
            10,
        )
        .unwrap();

        assert!(api_key.extend_expiry(0).is_err());
    }

    #[test]
    fn test_api_key_make_permanent() {
        let user_id = UserId::new();
        let mut api_key = ApiKey::new_with_expiry(
            user_id,
            valid_hash(),
            valid_scopes(),
            "Key".to_string(),
            30,
        )
        .unwrap();

        assert!(api_key.expires_at().is_some());
        api_key.make_permanent();
        assert!(api_key.expires_at().is_none());
        assert!(!api_key.is_expired());
    }

    #[test]
    fn test_reconstitute() {
        let id = Uuid::new_v4();
        let user_id = UserId::new();
        let now = Utc::now();
        let api_key = ApiKey::reconstitute(
            id,
            user_id.clone(),
            valid_hash(),
            valid_scopes(),
            "Reconstituted".to_string(),
            true,
            now,
            Some(now + Duration::days(30)),
            Some(now),
            None,
        );

        assert_eq!(api_key.id(), id);
        assert_eq!(api_key.user_id(), &user_id);
        assert!(api_key.is_active());
    }
}
