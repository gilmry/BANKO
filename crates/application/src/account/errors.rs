use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountServiceError {
    #[error("Account not found")]
    AccountNotFound,

    #[error("KYC not validated for customer")]
    KycNotValidated,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Account is closed")]
    AccountClosed,

    #[error("Account is suspended")]
    AccountSuspended,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
