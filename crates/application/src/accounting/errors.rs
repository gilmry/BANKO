use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountingServiceError {
    #[error("Entry not found")]
    EntryNotFound,

    #[error("Invalid entry: {0}")]
    InvalidEntry(String),

    #[error("Period already closed: {0}")]
    PeriodAlreadyClosed(String),

    #[error("Period not closed: {0}")]
    PeriodNotClosed(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
