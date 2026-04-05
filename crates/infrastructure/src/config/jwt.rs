use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct JwtConfig {
    secret: String,
    access_token_expiry_secs: i64,
    refresh_token_expiry_secs: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: String, // user_id
    pub email: String,
    pub roles: Vec<String>,
    pub iat: i64,
    pub exp: i64,
    pub token_type: String, // "access" or "refresh"
}

impl JwtConfig {
    pub fn new(secret: String, access_expiry_secs: i64, refresh_expiry_secs: i64) -> Self {
        JwtConfig {
            secret,
            access_token_expiry_secs: access_expiry_secs,
            refresh_token_expiry_secs: refresh_expiry_secs,
        }
    }

    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "banko-dev-secret-change-in-production-must-be-long".to_string());
        let access_expiry = std::env::var("JWT_ACCESS_EXPIRY_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600); // 1 hour
        let refresh_expiry = std::env::var("JWT_REFRESH_EXPIRY_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(604800); // 7 days

        JwtConfig::new(secret, access_expiry, refresh_expiry)
    }

    pub fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        roles: &[String],
    ) -> Result<String, String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.access_token_expiry_secs);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            roles: roles.to_vec(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            token_type: "access".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| format!("JWT encoding error: {e}"))
    }

    pub fn generate_refresh_token(
        &self,
        user_id: &str,
        email: &str,
        roles: &[String],
    ) -> Result<String, String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.refresh_token_expiry_secs);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            roles: roles.to_vec(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            token_type: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| format!("JWT encoding error: {e}"))
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, String> {
        let mut validation = Validation::default();
        validation.leeway = 0;
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| format!("JWT validation error: {e}"))?;

        Ok(token_data.claims)
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            3600,
            604800,
        )
    }

    #[test]
    fn test_generate_access_token() {
        let config = test_config();
        let token = config
            .generate_access_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();
        assert!(!token.is_empty());
        assert!(token.contains('.'));
    }

    #[test]
    fn test_validate_access_token() {
        let config = test_config();
        let token = config
            .generate_access_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();
        let claims = config.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.email, "test@banko.tn");
        assert_eq!(claims.roles, vec!["user"]);
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_generate_refresh_token() {
        let config = test_config();
        let token = config
            .generate_refresh_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();
        let claims = config.validate_token(&token).unwrap();
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn test_invalid_token_rejected() {
        let config = test_config();
        let result = config.validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_with_wrong_secret_rejected() {
        let config1 = test_config();
        let config2 = JwtConfig::new(
            "different-secret-also-long-enough-for-test".to_string(),
            3600,
            604800,
        );
        let token = config1
            .generate_access_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();
        let result = config2.validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token_rejected() {
        let config = JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            -10,
            -10,
        ); // already expired
        let token = config
            .generate_access_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();
        let result = config.validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_contain_required_fields() {
        let config = test_config();
        let token = config
            .generate_access_token(
                "user-123",
                "test@banko.tn",
                &["user".to_string(), "admin".to_string()],
            )
            .unwrap();
        let claims = config.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.email, "test@banko.tn");
        assert_eq!(claims.roles.len(), 2);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }
}
