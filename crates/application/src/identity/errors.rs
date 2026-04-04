use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Email already taken")]
    EmailTaken,

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account is locked")]
    AccountLocked,

    #[error("Account is inactive")]
    AccountInactive,

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User not found")]
    UserNotFound,

    #[error("Registration failed: {0}")]
    Register(#[from] RegisterError),

    #[error("Login failed: {0}")]
    Login(#[from] LoginError),

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum TwoFactorError {
    #[error("2FA already enabled")]
    AlreadyEnabled,

    #[error("2FA not enabled")]
    NotEnabled,

    #[error("2FA not pending verification")]
    NotPending,

    #[error("Invalid TOTP code")]
    InvalidCode,

    #[error("User not found")]
    UserNotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}
