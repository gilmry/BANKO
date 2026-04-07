use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArrangementError {
    #[error("Arrangement not found")]
    ArrangementNotFound,

    #[error("Arrangement bundle not found")]
    BundleNotFound,

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Product not found")]
    ProductNotFound,

    #[error("Invalid arrangement status: {0}")]
    InvalidStatus(String),

    #[error("Invalid arrangement type: {0}")]
    InvalidType(String),

    #[error("Invalid effective date")]
    InvalidEffectiveDate,

    #[error("Invalid maturity date")]
    InvalidMaturityDate,

    #[error("Cannot close arrangement with outstanding balance")]
    OutstandingBalance,

    #[error("Arrangement already exists")]
    DuplicateArrangement,

    #[error("Renewal not available")]
    RenewalNotAvailable,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
