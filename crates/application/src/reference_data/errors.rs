use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReferenceDataServiceError {
    #[error("Reference data not found")]
    NotFound,

    #[error("Country code not found: {0}")]
    CountryCodeNotFound(String),

    #[error("Currency not found: {0}")]
    CurrencyNotFound(String),

    #[error("Bank code not found: {0}")]
    BankCodeNotFound(String),

    #[error("Branch code not found: {0}")]
    BranchCodeNotFound(String),

    #[error("Holiday not found")]
    HolidayNotFound,

    #[error("System parameter not found: {0}")]
    SystemParameterNotFound(String),

    #[error("Regulatory code not found: {0}")]
    RegulatoryCodeNotFound(String),

    #[error("Fee schedule not found")]
    FeeScheduleNotFound,

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
