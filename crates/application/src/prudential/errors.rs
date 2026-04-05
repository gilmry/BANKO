use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrudentialServiceError {
    #[error("Institution not found")]
    InstitutionNotFound,

    #[error("Ratio not found")]
    RatioNotFound,

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
