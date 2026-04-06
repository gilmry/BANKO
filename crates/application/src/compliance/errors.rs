use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComplianceError {
    #[error("Control not found")]
    ControlNotFound,

    #[error("Risk not found")]
    RiskNotFound,

    #[error("Token not found")]
    TokenNotFound,

    #[error("Invalid control reference: {0}")]
    InvalidControlRef(String),

    #[error("Invalid risk score: {0}")]
    InvalidRiskScore(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
