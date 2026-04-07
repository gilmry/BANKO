use thiserror::Error;

/// Service-level errors for Islamic Banking
#[derive(Debug, Error)]
pub enum IslamicBankingServiceError {
    #[error("Contract not found: {0}")]
    ContractNotFound(String),

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Asset not found: {0}")]
    AssetNotFound(String),

    #[error("Invalid contract status")]
    InvalidContractStatus,

    #[error("Sharia board approval required")]
    ShariaApprovalRequired,

    #[error("Sharia board ruled product as haram")]
    HaramProduct,

    #[error("Conditions not met for conditional product")]
    ConditionsNotMet,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Profit calculation error: {0}")]
    ProfitCalculationError(String),

    #[error("Distribution error: {0}")]
    DistributionError(String),

    #[error("Zakat calculation error: {0}")]
    ZakatCalculationError(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = IslamicBankingServiceError::ContractNotFound("MURABA-001".to_string());
        assert!(err.to_string().contains("MURABA-001"));
    }

    #[test]
    fn test_haram_product_error() {
        let err = IslamicBankingServiceError::HaramProduct;