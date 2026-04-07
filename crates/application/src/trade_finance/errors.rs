use thiserror::Error;

#[derive(Debug, Error)]
pub enum TradeFinanceError {
    #[error("Letter of credit not found")]
    LetterOfCreditNotFound,

    #[error("Bank guarantee not found")]
    BankGuaranteeNotFound,

    #[error("Documentary collection not found")]
    DocumentaryCollectionNotFound,

    #[error("Trade finance limit not found")]
    TradeFinanceLimitNotFound,

    #[error("Invalid LC configuration: {0}")]
    InvalidLcConfiguration(String),

    #[error("Invalid guarantee configuration: {0}")]
    InvalidGuaranteeConfiguration(String),

    #[error("Invalid collection configuration: {0}")]
    InvalidCollectionConfiguration(String),

    #[error("Invalid limit configuration: {0}")]
    InvalidLimitConfiguration(String),

    #[error("LC is expired")]
    LcExpired,

    #[error("Guarantee is expired")]
    GuaranteeExpired,

    #[error("Limit exceeded")]
    LimitExceeded,

    #[error("Invalid transition: {0}")]
    InvalidTransition(String),

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
