use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentServiceError {
    #[error("Payment order not found")]
    OrderNotFound,

    #[error("Transfer not found")]
    TransferNotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Screening required: {0}")]
    ScreeningRequired(String),

    #[error("Payment blocked by sanctions: {0}")]
    PaymentBlocked(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
