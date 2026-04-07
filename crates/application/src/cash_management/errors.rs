use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashManagementError {
    #[error("Sweep account not found")]
    SweepAccountNotFound,

    #[error("Cash pool not found")]
    CashPoolNotFound,

    #[error("Cash forecast not found")]
    CashForecastNotFound,

    #[error("Liquidity position not found")]
    LiquidityPositionNotFound,

    #[error("Funding strategy not found")]
    FundingStrategyNotFound,

    #[error("Invalid sweep configuration: {0}")]
    InvalidSweepConfiguration(String),

    #[error("Invalid pool configuration: {0}")]
    InvalidPoolConfiguration(String),

    #[error("Invalid forecast parameters: {0}")]
    InvalidForecastParameters(String),

    #[error("Sweep account is not active")]
    SweepAccountInactive,

    #[error("Pool is not active")]
    PoolInactive,

    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    #[error("LCR below minimum threshold")]
    LcrBelowThreshold,

    #[error("NSFR below minimum threshold")]
    NsfrBelowThreshold,

    #[error("Account not found")]
    AccountNotFound,

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error