use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoanServiceError {
    #[error("Loan not found")]
    LoanNotFound,

    #[error("Account not found")]
    AccountNotFound,

    #[error("KYC not validated for customer")]
    KycNotValidated,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Prudential violation: {0}")]
    PrudentialViolation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
