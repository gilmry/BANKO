use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomerServiceError {
    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Email already registered: {0}")]
    EmailAlreadyExists(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Domain error: {0}")]
    Domain(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
