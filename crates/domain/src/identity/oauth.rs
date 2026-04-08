use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use super::UserId;

/// OAuth2 grant type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
    ClientCredentials,
    Implicit,
}

impl GrantType {
    pub fn as_str(&self) -> &str {
        match self {
            GrantType::AuthorizationCode => "authorization_code",
            GrantType::RefreshToken => "refresh_token",
            GrantType::ClientCredentials => "client_credentials",
            GrantType::Implicit => "implicit",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "authorization_code" => Ok(GrantType::AuthorizationCode),
            "refresh_token" => Ok(GrantType::RefreshToken),
            "client_credentials" => Ok(GrantType::ClientCredentials),
            "implicit" => Ok(GrantType::Implicit),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown grant type: {s}"
            ))),
        }
    }
}

/// OAuth2 client aggregate
/// FR-165: OAuth2 authorization server for Open Banking (PSD3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient {
    id: Uuid,
    client_id: String,
    client_secret_hash: String,
    name: String,
    description: Option<String>,
    redirect_uris: Vec<String>,
    allowed_grant_types: Vec<GrantType>,
    scopes: Vec<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
}

impl OAuthClient {
    /// Create a new OAuth2 client
    pub fn new(
        client_id: String,
        client_secret_hash: String,
        name: String,
        redirect_uris: Vec<String>,
        allowed_grant_types: Vec<GrantType>,
        scopes: Vec<String>,
    ) -> Result<Self, DomainError> {
        if client_id.is_empty() {
            return Err(DomainError::InvalidInput(
                "Client ID cannot be empty".to_string(),
            ));
        }
        if client_secret_hash.is_empty() {
            return Err(DomainError::InvalidInput(
                "Client secret hash cannot be empty".to_string(),
            ));
        }
        if name.is_empty() {
            return Err(DomainError::InvalidInput(
                "Client name cannot be empty".to_string(),
            ));
        }
        if redirect_uris.is_empty() {
            return Err(DomainError::InvalidInput(
                "At least one redirect URI is required".to_string(),
            ));
        }
        if allowed_grant_types.is_empty() {
            return Err(DomainError::InvalidInput(
                "At least one grant type must be allowed".to_string(),
            ));
        }
        if scopes.is_empty() {
            return Err(DomainError::InvalidInput(
                "At least one scope must be assigned".to_string(),
            ));
        }

        // Validate redirect URIs
        for uri in &redirect_uris {
            if !uri.starts_with("https://") && !uri.starts_with("http://localhost") {
                return Err(DomainError::InvalidInput(
                    format!("Redirect URI must be HTTPS (except localhost): {uri}")
                ));
            }
        }

        let now = Utc::now();
        Ok(OAuthClient {
            id: Uuid::new_v4(),
            client_id,
            client_secret_hash,
            name,
            description: None,
            redirect_uris,
            allowed_grant_types,
            scopes,
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
        client_id: String,
        client_secret_hash: String,
        name: String,
        description: Option<String>,
        redirect_uris: Vec<String>,
        allowed_grant_types: Vec<GrantType>,
        scopes: Vec<String>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        last_used_at: Option<DateTime<Utc>>,
    ) -> Self {
        OAuthClient {
            id,
            client_id,
            client_secret_hash,
            name,
            description,
            redirect_uris,
            allowed_grant_types,
            scopes,
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

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret_hash(&self) -> &str {
        &self.client_secret_hash
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn redirect_uris(&self) -> &[String] {
        &self.redirect_uris
    }

    pub fn allowed_grant_types(&self) -> &[GrantType] {
        &self.allowed_grant_types
    }

    pub fn scopes(&self) -> &[String] {
        &self.scopes
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

    /// Check if a redirect URI is allowed for this client
    pub fn is_redirect_uri_allowed(&self, uri: &str) -> bool {
        self.redirect_uris.iter().any(|allowed| allowed == uri)
    }

    /// Check if grant type is allowed
    pub fn allows_grant_type(&self, grant: GrantType) -> bool {
        self.allowed_grant_types.contains(&grant)
    }

    /// Check if scope is allowed
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }

    /// Check if all required scopes are allowed
    pub fn has_all_scopes(&self, required_scopes: &[&str]) -> bool {
        required_scopes.iter().all(|req| self.has_scope(req))
    }

    /// Deactivate the client (suspend it)
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Reactivate the client
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Record a successful use of the client
    pub fn record_use(&mut self) {
        self.last_used_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Add a redirect URI
    pub fn add_redirect_uri(&mut self, uri: String) -> Result<(), DomainError> {
        if !uri.starts_with("https://") && !uri.starts_with("http://localhost") {
            return Err(DomainError::InvalidInput(
                format!("Redirect URI must be HTTPS (except localhost): {uri}")
            ));
        }
        if self.redirect_uris.contains(&uri) {
            return Err(DomainError::InvalidInput(
                format!("Redirect URI already registered: {uri}")
            ));
        }
        self.redirect_uris.push(uri);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a redirect URI (must keep at least one)
    pub fn remove_redirect_uri(&mut self, uri: &str) -> Result<(), DomainError> {
        if self.redirect_uris.len() == 1 {
            return Err(DomainError::InvalidInput(
                "Must keep at least one redirect URI".to_string()
            ));
        }
        if !self.redirect_uris.contains(&uri.to_string()) {
            return Err(DomainError::InvalidInput(
                format!("Redirect URI not found: {uri}")
            ));
        }
        self.redirect_uris.retain(|u| u != uri);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Set description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }
}

/// OAuth2 authorization code (short-lived, one-time use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCode {
    code: String,
    client_id: String,
    user_id: UserId,
    scopes: Vec<String>,
    redirect_uri: String,
    is_used: bool,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl AuthorizationCode {
    /// Create a new authorization code (TTL: 10 minutes)
    pub fn new(
        code: String,
        client_id: String,
        user_id: UserId,
        scopes: Vec<String>,
        redirect_uri: String,
    ) -> Result<Self, DomainError> {
        if code.is_empty() {
            return Err(DomainError::InvalidInput(
                "Authorization code cannot be empty".to_string(),
            ));
        }
        if scopes.is_empty() {
            return Err(DomainError::InvalidInput(
                "At least one scope required".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(AuthorizationCode {
            code,
            client_id,
            user_id,
            scopes,
            redirect_uri,
            is_used: false,
            created_at: now,
            expires_at: now + Duration::minutes(10),
        })
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub fn is_used(&self) -> bool {
        self.is_used
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        !self.is_used && !self.is_expired()
    }

    /// Mark code as used (one-time use)
    pub fn mark_used(&mut self) -> Result<(), DomainError> {
        if self.is_used {
            return Err(DomainError::InvalidInput(
                "Authorization code already used".to_string()
            ));
        }
        if self.is_expired() {
            return Err(DomainError::InvalidInput(
                "Authorization code expired".to_string()
            ));
        }
        self.is_used = true;
        Ok(())
    }
}

/// OAuth2 access token (signed JWT in real implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    token: String,
    client_id: String,
    user_id: UserId,
    scopes: Vec<String>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl AccessToken {
    /// Create a new access token (TTL: 1 hour default, configurable)
    pub fn new(
        token: String,
        client_id: String,
        user_id: UserId,
        scopes: Vec<String>,
        ttl_secs: i64,
    ) -> Result<Self, DomainError> {
        if token.is_empty() {
            return Err(DomainError::InvalidInput(
                "Token cannot be empty".to_string(),
            ));
        }
        if scopes.is_empty() {
            return Err(DomainError::InvalidInput(
                "At least one scope required".to_string(),
            ));
        }
        if ttl_secs <= 0 {
            return Err(DomainError::InvalidInput(
                "TTL must be positive".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(AccessToken {
            token,
            client_id,
            user_id,
            scopes,
            created_at: now,
            expires_at: now + Duration::seconds(ttl_secs),
        })
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }

    pub fn has_all_scopes(&self, required_scopes: &[&str]) -> bool {
        required_scopes.iter().all(|req| self.has_scope(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_client() -> OAuthClient {
        OAuthClient::new(
            "client_123".to_string(),
            "$2b$12$LJ3m4ys3Lg2HEOjdLNRsWuBNRZLJDhG5JQqJK9qJKj3K4hNqXKwu".to_string(),
            "Test Client".to_string(),
            vec!["https://example.com/callback".to_string()],
            vec![GrantType::AuthorizationCode],
            vec!["openid".to_string(), "profile".to_string()],
        )
        .unwrap()
    }

    #[test]
    fn test_oauth_client_new() {
        let client = valid_client();
        assert_eq!(client.client_id(), "client_123");
        assert_eq!(client.name(), "Test Client");
        assert!(client.is_active());
    }

    #[test]
    fn test_oauth_client_empty_client_id_rejected() {
        let result = OAuthClient::new(
            String::new(),
            "$hash".to_string(),
            "Name".to_string(),
            vec!["https://example.com".to_string()],
            vec![GrantType::AuthorizationCode],
            vec!["scope".to_string()],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_oauth_client_no_redirect_uris_rejected() {
        let result = OAuthClient::new(
            "client".to_string(),
            "$hash".to_string(),
            "Name".to_string(),
            vec![],
            vec![GrantType::AuthorizationCode],
            vec!["scope".to_string()],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_oauth_client_http_redirect_uri_rejected() {
        let result = OAuthClient::new(
            "client".to_string(),
            "$hash".to_string(),
            "Name".to_string(),
            vec!["http://example.com".to_string()],
            vec![GrantType::AuthorizationCode],
            vec!["scope".to_string()],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_oauth_client_localhost_http_allowed() {
        let result = OAuthClient::new(
            "client".to_string(),
            "$hash".to_string(),
            "Name".to_string(),
            vec!["http://localhost:3000/callback".to_string()],
            vec![GrantType::AuthorizationCode],
            vec!["scope".to_string()],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_oauth_client_is_redirect_uri_allowed() {
        let client = valid_client();
        assert!(client.is_redirect_uri_allowed("https://example.com/callback"));
        assert!(!client.is_redirect_uri_allowed("https://other.com/callback"));
    }

    #[test]
    fn test_oauth_client_allows_grant_type() {
        let client = valid_client();
        assert!(client.allows_grant_type(GrantType::AuthorizationCode));
        assert!(!client.allows_grant_type(GrantType::ClientCredentials));
    }

    #[test]
    fn test_oauth_client_has_scope() {
        let client = valid_client();
        assert!(client.has_scope("openid"));
        assert!(!client.has_scope("email"));
    }

    #[test]
    fn test_oauth_client_deactivate_and_activate() {
        let mut client = valid_client();
        assert!(client.is_active());
        client.deactivate();
        assert!(!client.is_active());
        client.activate();
        assert!(client.is_active());
    }

    #[test]
    fn test_oauth_client_add_redirect_uri() {
        let mut client = valid_client();
        client
            .add_redirect_uri("https://another.com/callback".to_string())
            .unwrap();
        assert_eq!(client.redirect_uris().len(), 2);
        assert!(client.is_redirect_uri_allowed("https://another.com/callback"));
    }

    #[test]
    fn test_oauth_client_remove_last_redirect_uri_rejected() {
        let mut client = valid_client();
        let uri = client.redirect_uris()[0].clone();
        let result = client.remove_redirect_uri(&uri);
        assert!(result.is_err());
    }

    #[test]
    fn test_authorization_code_new() {
        let user_id = UserId::new();
        let code = AuthorizationCode::new(
            "auth_code_123".to_string(),
            "client_id".to_string(),
            user_id.clone(),
            vec!["openid".to_string()],
            "https://example.com/callback".to_string(),
        )
        .unwrap();

        assert_eq!(code.code(), "auth_code_123");
        assert_eq!(code.user_id(), &user_id);
        assert!(!code.is_used());
        assert!(!code.is_expired());
        assert!(code.is_valid());
    }

    #[test]
    fn test_authorization_code_mark_used() {
        let user_id = UserId::new();
        let mut code = AuthorizationCode::new(
            "code".to_string(),
            "client".to_string(),
            user_id,
            vec!["scope".to_string()],
            "https://example.com".to_string(),
        )
        .unwrap();

        assert!(code.mark_used().is_ok());
        assert!(code.is_used());
        assert!(code.mark_used().is_err()); // Can't use twice
    }

    #[test]
    fn test_access_token_new() {
        let user_id = UserId::new();
        let token = AccessToken::new(
            "jwt_token_xyz".to_string(),
            "client".to_string(),
            user_id.clone(),
            vec!["openid".to_string()],
            3600,
        )
        .unwrap();

        assert_eq!(token.token(), "jwt_token_xyz");
        assert_eq!(token.user_id(), &user_id);
        assert!(!token.is_expired());
        assert!(token.has_scope("openid"));
    }

    #[test]
    fn test_access_token_zero_ttl_rejected() {
        let user_id = UserId::new();
        let result = AccessToken::new(
            "token".to_string(),
            "client".to_string(),
            user_id,
            vec!["scope".to_string()],
            0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_grant_type_from_str() {
        assert_eq!(
            GrantType::from_str("authorization_code").unwrap(),
            GrantType::AuthorizationCode
        );
        assert_eq!(
            GrantType::from_str("refresh_token").unwrap(),
            GrantType::RefreshToken
        );
    }

    #[test]
    fn test_grant_type_invalid() {
        assert!(GrantType::from_str("invalid").is_err());
    }
}
