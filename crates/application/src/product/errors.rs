use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductServiceError {
    ProductNotFound,
    InvalidInput(String),
    DomainError(String),
    RepositoryError(String),
    InvalidStatus(String),
    EligibilityCheckFailed(Vec<String>),
    PricingGridNotFound,
}

impl fmt::Display for ProductServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductServiceError::ProductNotFound => write!(f, "Product not found"),
            ProductServiceError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ProductServiceError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            ProductServiceError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            ProductServiceError::InvalidStatus(msg) => write!(f, "Invalid status: {}", msg),
            ProductServiceError::EligibilityCheckFailed(reasons) => {
                write!(f, "Eligibility check failed: {}", reasons.join(", "))
            }
            ProductServiceError::PricingGridNotFound => write!(f, "Pricing grid not found"),
        }
    }
}

impl std::error::Error for ProductServiceError {}
