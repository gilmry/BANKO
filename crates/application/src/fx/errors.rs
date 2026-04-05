use thiserror::Error;

#[derive(Debug, Error)]
pub enum FxServiceError {
    #[error("FX operation not found")]
    OperationNotFound,

    #[error("Exchange rate not found")]
    RateNotFound,

    #[error("Daily limit exceeded: {0}")]
    DailyLimitExceeded(String),

    #[error("Compliance check failed: {0}")]
    ComplianceFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
