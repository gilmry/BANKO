use thiserror::Error;

#[derive(Debug, Error)]
pub enum SanctionsServiceError {
    #[error("Screening result not found")]
    ResultNotFound,

    #[error("Sanction list not found")]
    ListNotFound,

    #[error("Payment blocked: sanctions hit detected")]
    PaymentBlocked,

    #[error("Payment held: potential sanctions match requires manual review")]
    PaymentHeld,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
