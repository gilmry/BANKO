use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmlServiceError {
    #[error("Transaction not found")]
    TransactionNotFound,

    #[error("Alert not found")]
    AlertNotFound,

    #[error("Investigation not found")]
    InvestigationNotFound,

    #[error("Report not found")]
    ReportNotFound,

    #[error("Freeze not found")]
    FreezeNotFound,

    #[error("Account is frozen")]
    AccountFrozen,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
