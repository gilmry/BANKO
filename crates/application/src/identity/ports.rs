use async_trait::async_trait;

use banko_domain::identity::{Session, SessionId, TwoFactorAuth, User, UserId};
use banko_domain::shared::EmailAddress;

/// Port for user persistence — implemented by infrastructure layer.
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), String>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, String>;
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>, String>;
    async fn exists_by_email(&self, email: &EmailAddress) -> Result<bool, String>;
    async fn delete(&self, id: &UserId) -> Result<(), String>;
}

/// Port for password hashing — implemented by infrastructure layer.
#[async_trait]
pub trait IPasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, String>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, String>;
}

/// Port for TOTP operations — implemented by infrastructure layer.
#[async_trait]
pub trait ITotpService: Send + Sync {
    fn generate_secret(&self) -> String;
    fn generate_totp_uri(&self, secret: &str, email: &str) -> String;
    fn verify_code(&self, secret: &str, code: &str) -> bool;
}

/// Port for 2FA persistence.
#[async_trait]
pub trait ITwoFactorRepository: Send + Sync {
    async fn save(&self, tfa: &TwoFactorAuth) -> Result<(), String>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<TwoFactorAuth>, String>;
    async fn delete_by_user_id(&self, user_id: &UserId) -> Result<(), String>;
}

/// Port for session persistence.
#[async_trait]
pub trait ISessionRepository: Send + Sync {
    async fn save(&self, session: &Session) -> Result<(), String>;
    async fn find_by_id(&self, id: &SessionId) -> Result<Option<Session>, String>;
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<Session>, String>;
    async fn find_active_by_user_id(&self, user_id: &UserId) -> Result<Vec<Session>, String>;
    async fn delete_by_id(&self, id: &SessionId) -> Result<(), String>;
    async fn delete_expired(&self) -> Result<u64, String>;
}
