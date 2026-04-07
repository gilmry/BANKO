use thiserror::Error;

#[derive(Debug, Error)]
pub enum InsuranceError {
    #[error("Insurance policy not found")]
    PolicyNotFound,

    #[error("Insurance claim not found")]
    ClaimNotFound,

    #[error("Bancassurance product not found")]
    ProductNotFound,

    #[error("Insurance commission not found")]
    CommissionNotFound,

    #[error("Invalid policy configuration: {0}")]
    InvalidPolicyConfiguration(String),

    #[error("Invalid claim configuration: {0}")]
    InvalidClaimConfiguration(String),

    #[error("Invalid product configuration: {0}")]
    InvalidProductConfiguration(String),

    #[error("Invalid commission configuration: {0}")]
    InvalidCommissionConfiguration(String),

    #[error("Policy is expired")]
    PolicyExpired,

    #[error("Policy is not active")]
    PolicyNotActive,

    #[error("Invalid policy transition: {0}")]
    InvalidPolicyTransition(String),

    #[error("Invalid claim transition: {0}")]
    InvalidClaimTransition(String),

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
