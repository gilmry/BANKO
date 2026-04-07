use async_trait::async_trait;

use banko_domain::cash_management::{
    CashForecast, CashForecastId, CashPool, CashPoolId, FundingStrategy, FundingStrategyId,
    LiquidityPosition, SweepAccount, SweepAccountId,
};

/// Port for sweep account persistence.
#[async_trait]
pub trait ISweepAccountRepository: Send + Sync {
    async fn save(&self, sweep: &SweepAccount) -> Result<(), String>;
    async fn find_by_id(&self, id: &SweepAccountId) -> Result<Option<SweepAccount>, String>;
    async fn find_by_source_account(
        &self,
        source_account_id: &banko_domain::shared::AccountId,
    ) -> Result<Vec<SweepAccount>, String>;
    async fn delete(&self, id: &SweepAccountId) -> Result<(), String>;
}

/// Port for cash pool persistence.
#[async_trait]
pub trait ICashPoolRepository: Send + Sync {
    async fn save(&self, pool: &CashPool) -> Result<(), String>;
    async fn find_by_id(&self, id: &CashPoolId) -> Result<Option<CashPool>, String>;
    async fn find_all_active(&self) -> Result<Vec<CashPool>, String>;
    async fn find_by_header_account(
        &self,
        header_account_id: &banko_domain::shared::AccountId,
    ) -> Result<Option<CashPool>, String>;
    async fn delete(&self, id: &CashPoolId) -> Result<(), String>;
}

/// Port for cash forecast persistence.
#[async_trait]
pub trait ICashForecastRepository: Send + Sync {
    async fn save(&self, forecast: &CashForecast) -> Result<(), String>;
    async fn find_by_id(&self, id: &CashForecastId) -> Result<Option<CashForecast>, String>;
    async fn find_latest_by_account(
        &self,
        account_id: &banko_domain::shared::AccountId,
    ) -> Result<Option<CashForecast>, String>;
    async fn find_by_date_range(
        &self,
        account_id: &banko_domain::shared::AccountId,
        from_date: chrono::DateTime<chrono::Utc>,
        to_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<CashForecast>, String>;
}

/// Port for liquidity position persistence.
#[async_trait]
pub trait ILiquidityPositionRepository: Send + Sync {
    async fn save(&self, position: &LiquidityPosition) -> Result<(), String>;
    async fn find_latest(
        &self,
        currency: &banko_domain::shared::Currency,
    ) -> Result<Option<LiquidityPosition>, String>;
    async fn find_by_date_range(
        &self,
        currency: &banko_domain::shared::Currency,
        from_date: chrono::DateTime<chrono::Utc>,
        to_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<LiquidityPosition>, String>;
}

/// Port for funding strategy persistence.
#[async_trait]
pub trait IFundingStrategyRepository: Send + Sync {
    async fn save(&self, strategy: &FundingStrategy) -> Result<(), String>;
    async fn find_by_id(&self, id: &FundingStrategyId) -> Result<Option<FundingStrategy>, String>;
    async fn find_all_active(&self) -> Result<Vec<FundingStrategy>, String>;
    a