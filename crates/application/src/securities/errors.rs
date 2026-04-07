use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecuritiesServiceError {
    #[error("Securities account not found")]
    SecuritiesAccountNotFound,

    #[error("Trade order not found")]
    TradeOrderNotFound,

    #[error("Settlement not found")]
    SettlementNotFound,

    #[error("Corporate action not found")]
    CorporateActionNotFound,

    #[error("Security holding not found")]
    SecurityHoldingNotFound,

    #[error("Insufficient holdings to sell")]
    InsufficientHoldings,

    #[error("Account not operational")]
    AccountNotOperational,

    #[error("Invalid order state")]
    InvalidOrderState,

    #[error("Settlement failed")]
    SettlementFailed,

    #[error("Short selling not allowed")]
    ShortSellingNotAllowed,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
