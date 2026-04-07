use std::sync::Arc;

use chrono::Utc;

use banko_domain::cash_management::{
    CashForecast, CashForecastId, CashPool, CashPoolId, ConfidenceLevel, FundingStrategy,
    FundingStrategyId, InstrumentType, LiquidityPosition, PoolType, SweepAccount,
    SweepAccountId, SweepFrequency, SweepType,
};
use banko_domain::shared::{AccountId, Currency, Money};

use super::dto::*;
use super::errors::CashManagementError;
use super::ports::*;

pub struct CashManagementService {
    sweep_repo: Arc<dyn ISweepAccountRepository>,
    pool_repo: Arc<dyn ICashPoolRepository>,
    forecast_repo: Arc<dyn ICashForecastRepository>,
    liquidity_repo: Arc<dyn ILiquidityPositionRepository>,
    strategy_repo: Arc<dyn IFundingStrategyRepository>,
}

impl CashManagementService {
    pub fn new(
        sweep_repo: Arc<dyn ISweepAccountRepository>,
        pool_repo: Arc<dyn ICashPoolRepository>,
        forecast_repo: Arc<dyn ICashForecastRepository>,
        liquidity_repo: Arc<dyn ILiquidityPositionRepository>,
        strategy_repo: Arc<dyn IFundingStrategyRepository>,
    ) -> Self {
        CashManagementService {
            sweep_repo,
            pool_repo,
            forecast_repo,
            liquidity_repo,
            strategy_repo,
        }
    }

    // --- Sweep Account Operations ---

    pub async fn create_sweep_account(
        &self,
        request: CreateSweepAccountRequest,
    ) -> Result<SweepAccountResponse, CashManagementError> {
        let source_id = AccountId::parse(&request.source_account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let target_id = AccountId::parse(&request.target_account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let sweep_type = SweepType::from_str(&request.sweep_type)
            .map_err(|e| CashManagementError::InvalidSweepConfiguration(e.to_string()))?;

        let sweep_frequency = SweepFrequency::from_str(&request.sweep_frequency)
            .map_err(|e| CashManagementError::InvalidSweepConfiguration(e.to_string()))?;

        let threshold_amount = if let Some(amount) = request.threshold_amount {
            Some(
                Money::from_f64(amount, Currency::try_from("TND").unwrap())
                    .map_err(|e| CashManagementError::DomainError(e.to_string()))?,
            )
        } else {
            None
        };

        let sweep = SweepAccount::new(
            source_id.clone(),
            target_id.clone(),
            sweep_type,
            threshold_amount.clone(),
            sweep_frequency,
        )
        .map_err(|e| CashManagementError::InvalidSweepConfiguration(e.to_string()))?;

        self.sweep_repo
            .save(&sweep)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.sweep_to_response(&sweep))
    }

    pub async fn get_sweep_account(
        &self,
        id: &str,
    ) -> Result<SweepAccountResponse, CashManagementError> {
        let sweep_id = SweepAccountId::parse(id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let sweep = self
            .sweep_repo
            .find_by_id(&sweep_id)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::SweepAccountNotFound)?;

        Ok(self.sweep_to_response(&sweep))
    }

    pub async fn list_sweeps_by_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<SweepAccountResponse>, CashManagementError> {
        let acc_id = AccountId::parse(account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let sweeps = self
            .sweep_repo
            .find_by_source_account(&acc_id)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(sweeps.iter().map(|s| self.sweep_to_response(s)).collect())
    }

    pub async fn deactivate_sweep_account(
        &self,
        id: &str,
    ) -> Result<SweepAccountResponse, CashManagementError> {
        let sweep_id = SweepAccountId::parse(id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let mut sweep = self
            .sweep_repo
            .find_by_id(&sweep_id)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::SweepAccountNotFound)?;

        sweep
            .deactivate()
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        self.sweep_repo
            .save(&sweep)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.sweep_to_response(&sweep))
    }

    // --- Cash Pool Operations ---

    pub async fn create_cash_pool(
        &self,
        request: CreateCashPoolRequest,
    ) -> Result<CashPoolResponse, CashManagementError> {
        let header_id = AccountId::parse(&request.header_account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let participant_ids: Result<Vec<_>, _> = request
            .participant_account_ids
            .iter()
            .map(|s| AccountId::parse(s))
            .collect::<Result<_, _>>()
            .map_err(|e| CashManagementError::DomainError(e.to_string()));

        let participant_ids = participant_ids?;

        let pool_type = PoolType::from_str(&request.pool_type)
            .map_err(|e| CashManagementError::InvalidPoolConfiguration(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| CashManagementError::InvalidPoolConfiguration(e.to_string()))?;

        let pool = CashPool::new(&request.name, header_id, participant_ids, pool_type, currency)
            .map_err(|e| CashManagementError::InvalidPoolConfiguration(e.to_string()))?;

        self.pool_repo
            .save(&pool)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.pool_to_response(&pool))
    }

    pub async fn get_cash_pool(&self, id: &str) -> Result<CashPoolResponse, CashManagementError> {
        let pool_id = CashPoolId::parse(id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let pool = self
            .pool_repo
            .find_by_id(&pool_id)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::CashPoolNotFound)?;

        Ok(self.pool_to_response(&pool))
    }

    pub async fn list_active_pools(&self) -> Result<Vec<CashPoolResponse>, CashManagementError> {
        let pools = self
            .pool_repo
            .find_all_active()
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(pools.iter().map(|p| self.pool_to_response(p)).collect())
    }

    pub async fn add_pool_participant(
        &self,
        pool_id: &str,
        account_id: &str,
    ) -> Result<CashPoolResponse, CashManagementError> {
        let pool_uuid = CashPoolId::parse(pool_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let acc_id = AccountId::parse(account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let mut pool = self
            .pool_repo
            .find_by_id(&pool_uuid)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::CashPoolNotFound)?;

        pool.add_participant(acc_id)
            .map_err(|e| CashManagementError::InvalidPoolConfiguration(e.to_string()))?;

        self.pool_repo
            .save(&pool)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.pool_to_response(&pool))
    }

    // --- Cash Forecast Operations ---

    pub async fn create_cash_forecast(
        &self,
        request: CreateCashForecastRequest,
    ) -> Result<CashForecastResponse, CashManagementError> {
        let account_id = AccountId::parse(&request.account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let confidence = ConfidenceLevel::from_str(&request.confidence_level)
            .map_err(|e| CashManagementError::InvalidForecastParameters(e.to_string()))?;

        let inflows = Money::from_f64(request.expected_inflows, Currency::try_from("TND").unwrap())
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let outflows = Money::from_f64(request.expected_outflows, Currency::try_from("TND").unwrap())
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let forecast = CashForecast::new(
            account_id,
            request.forecast_date,
            inflows,
            outflows,
            confidence,
            request.horizon_days,
        )
        .map_err(|e| CashManagementError::InvalidForecastParameters(e.to_string()))?;

        self.forecast_repo
            .save(&forecast)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.forecast_to_response(&forecast))
    }

    pub async fn get_latest_forecast(
        &self,
        account_id: &str,
    ) -> Result<CashForecastResponse, CashManagementError> {
        let acc_id = AccountId::parse(account_id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let forecast = self
            .forecast_repo
            .find_latest_by_account(&acc_id)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::CashForecastNotFound)?;

        Ok(self.forecast_to_response(&forecast))
    }

    // --- Liquidity Position Operations ---

    pub async fn record_liquidity_position(
        &self,
        request: CreateLiquidityPositionRequest,
    ) -> Result<LiquidityPositionResponse, CashManagementError> {
        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let total_assets = Money::from_f64(request.total_assets, currency.clone())
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let total_liabilities = Money::from_f64(request.total_liabilities, currency.clone())
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let lcr_assets = Money::from_f64(request.lcr_eligible_assets, currency.clone())
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let nsfr_funding = Money::from_f64(request.nsfr_stable_funding, currency.clone())
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let position = LiquidityPosition::new(
            request.date,
            currency,
            total_assets,
            total_liabilities,
            lcr_assets,
            nsfr_funding,
        )
        .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        self.liquidity_repo
            .save(&position)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.liquidity_to_response(&position)?)
    }

    pub async fn get_current_liquidity_position(
        &self,
        currency: &str,
    ) -> Result<LiquidityPositionResponse, CashManagementError> {
        let curr = Currency::try_from(currency)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let position = self
            .liquidity_repo
            .find_latest(&curr)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::LiquidityPositionNotFound)?;

        self.liquidity_to_response(&position)
    }

    // --- Funding Strategy Operations ---

    pub async fn create_funding_strategy(
        &self,
        request: CreateFundingStrategyRequest,
    ) -> Result<FundingStrategyResponse, CashManagementError> {
        let instruments: Result<Vec<_>, _> = request
            .instruments
            .iter()
            .map(|s| InstrumentType::from_str(s))
            .collect::<Result<_, _>>()
            .map_err(|e| CashManagementError::DomainError(e.to_string()));

        let instruments = instruments?;

        let strategy = FundingStrategy::new(&request.name, request.target_ratio, instruments)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        self.strategy_repo
            .save(&strategy)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.strategy_to_response(&strategy))
    }

    pub async fn get_funding_strategy(
        &self,
        id: &str,
    ) -> Result<FundingStrategyResponse, CashManagementError> {
        let strategy_id = FundingStrategyId::parse(id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let strategy = self
            .strategy_repo
            .find_by_id(&strategy_id)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::FundingStrategyNotFound)?;

        Ok(self.strategy_to_response(&strategy))
    }

    pub async fn list_active_strategies(
        &self,
    ) -> Result<Vec<FundingStrategyResponse>, CashManagementError> {
        let strategies = self
            .strategy_repo
            .find_all_active()
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(strategies
            .iter()
            .map(|s| self.strategy_to_response(s))
            .collect())
    }

    pub async fn update_strategy_target_ratio(
        &self,
        id: &str,
        new_ratio: f64,
    ) -> Result<FundingStrategyResponse, CashManagementError> {
        let strategy_id = FundingStrategyId::parse(id)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let mut strategy = self
            .strategy_repo
            .find_by_id(&strategy_id)
            .await
            .map_err(CashManagementError::Internal)?
            .ok_or(CashManagementError::FundingStrategyNotFound)?;

        strategy
            .update_target_ratio(new_ratio)
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        self.strategy_repo
            .save(&strategy)
            .await
            .map_err(CashManagementError::Internal)?;

        Ok(self.strategy_to_response(&strategy))
    }

    // --- Response Mappers ---

    fn sweep_to_response(&self, sweep: &SweepAccount) -> SweepAccountResponse {
        let (threshold_amount, threshold_currency) = if let Some(amount) = sweep.threshold_amount() {
            (Some(amount.as_f64()), Some(amount.currency().code().to_string()))
        } else {
            (None, None)
        };

        SweepAccountResponse {
            id: sweep.id().to_string(),
            source_account_id: sweep.source_account_id().to_string(),
            target_account_id: sweep.target_account_id().to_string(),
            sweep_type: sweep.sweep_type().to_string(),
            threshold_amount,
            threshold_currency,
            sweep_frequency: sweep.sweep_frequency().to_string(),
            is_active: sweep.is_active(),
            last_sweep_at: sweep.last_sweep_at(),
            created_at: sweep.created_at(),
            updated_at: sweep.updated_at(),
        }
    }

    fn pool_to_response(&self, pool: &CashPool) -> CashPoolResponse {
        let participants: Vec<String> = pool
            .participant_accounts()
            .iter()
            .map(|id| id.to_string())
            .collect();

        CashPoolResponse {
            id: pool.id().to_string(),
            name: pool.name().to_string(),
            header_account_id: pool.header_account_id().to_string(),
            participant_account_ids: participants,
            pool_type: pool.pool_type().to_string(),
            currency: pool.currency().code().to_string(),
            is_active: pool.is_active(),
            created_at: pool.created_at(),
            updated_at: pool.updated_at(),
        }
    }

    fn forecast_to_response(&self, forecast: &CashForecast) -> CashForecastResponse {
        CashForecastResponse {
            id: forecast.id().to_string(),
            account_id: forecast.account_id().to_string(),
            forecast_date: forecast.forecast_date(),
            expected_inflows: forecast.expected_inflows().as_f64(),
            expected_outflows: forecast.expected_outflows().as_f64(),
            currency: forecast.expected_inflows().currency().code().to_string(),
            net_position: forecast.net_position().as_f64(),
            confidence_level: forecast.confidence_level().to_string(),
            horizon_days: forecast.horizon_days(),
            created_at: forecast.created_at(),
        }
    }

    fn liquidity_to_response(
        &self,
        position: &LiquidityPosition,
    ) -> Result<LiquidityPositionResponse, CashManagementError> {
        let lcr_ratio = position
            .lcr_ratio()
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        let nsfr_ratio = position
            .nsfr_ratio()
            .map_err(|e| CashManagementError::DomainError(e.to_string()))?;

        Ok(LiquidityPositionResponse {
            date: position.date(),
            currency: position.currency().code().to_string(),
            total_assets: position.total_assets().as_f64(),
            total_liabilities: position.total_liabilities().as_f64(),
            net_position: position.net_position().as_f64(),
            lcr_eligible_assets: position.lcr_eligible_assets().as_f64(),
            nsfr_stable_funding: position.nsfr_stable_funding().as_f64(),
            lcr_ratio,
            nsfr_ratio,
        })
    }

    fn strategy_to_response(&self, strategy: &FundingStrategy) -> FundingStrategyResponse {
        let instruments: Vec<String> = strategy
            .instruments()
            .iter()
            .map(|i| i.to_string())
            .collect();

        FundingStrategyResponse {
            id: strategy.id().to_string(),
            name: strategy.name().to_string(),
            target_ratio: strategy.target_ratio(),
            instruments,
            is_active: strategy.is_active(),
            created_at: strategy.created_at(),
            updated_at: strategy.updated_at(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Mock implementations for testing
    struct MockSweepRepository;
    #[async_trait::async_trait]
    impl ISweepAccountRepository for MockSweepRepository {
        async fn save(&self, _sweep: &SweepAccount) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &SweepAccountId) -> Result<Option<SweepAccount>, String> {
            Ok(None)
        }
        async fn find_by_source_account(
            &self,
            _source_account_id: &AccountId,
        ) -> Result<Vec<SweepAccount>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &SweepAccountId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockPoolRepository;
    #[async_trait::async_trait]
    impl ICashPoolRepository for MockPoolRepository {
        async fn save(&self, _pool: &CashPool) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &CashPoolId) -> Result<Option<CashPool>, String> {
            Ok(None)
        }
        async fn find_all_active(&self) -> Result<Vec<CashPool>, String> {
            Ok(vec![])
        }
        async fn find_by_header_account(
            &self,
            _header_account_id: &AccountId,
        ) -> Result<Option<CashPool>, String> {
            Ok(None)
        }
        async fn delete(&self, _id: &CashPoolId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockForecastRepository;
    #[async_trait::async_trait]
    impl ICashForecastRepository for MockForecastRepository {
        async fn save(&self, _forecast: &CashForecast) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &CashForecastId) -> Result<Option<CashForecast>, String> {
            Ok(None)
        }
        async fn find_latest_by_account(
            &self,
            _account_id: &AccountId,
        ) -> Result<Option<CashForecast>, String> {
            Ok(None)
        }
        async fn find_by_date_range(
            &self,
            _account_id: &AccountId,
            _from_date: chrono::DateTime<chrono::Utc>,
            _to_date: chrono::DateTime<chrono::Utc>,
        ) -> Result<Vec<CashForecast>, String> {
            Ok(vec![])
        }
    }

    struct MockLiquidityRepository;
    #[async_trait::async_trait]
    impl ILiquidityPositionRepository for MockLiquidityRepository {
        async fn save(&self, _position: &LiquidityPosition) -> Result<(), String> {
            Ok(())
        }
        async fn find_latest(
            &self,
            _currency: &Currency,
        ) -> Result<Option<LiquidityPosition>, String> {
            Ok(None)
        }
        async fn find_by_date_range(
            &self,
            _currency: &Currency,
            _from_date: chrono::DateTime<chrono::Utc>,
            _to_date: chrono::DateTime<chrono::Utc>,
        ) -> Result<Vec<LiquidityPosition>, String> {
            Ok(vec![])
        }
    }

    struct MockStrategyRepository;
    #[async_trait::async_trait]
    impl IFundingStrategyRepository for MockStrategyRepository {
        async fn save(&self, _strategy: &FundingStrategy) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &FundingStrategyId) -> Result<Option<FundingStrategy>, String> {
            Ok(None)
        }
        async fn find_all_active(&self) -> Result<Vec<FundingStrategy>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &FundingStrategyId) -> Result<(), String> {
            Ok(())
        }
    }

    fn create_service() -> CashManagementService {
        CashManagementService::new(
            Arc::new(MockSweepRepository),
            Arc::new(MockPoolRepository),
            Arc::new(MockForecastRepository),
            Arc::new(MockLiquidityRepository),
            Arc::new(MockStrategyRepository),
        )
    }

    #[test]
    fn test_service_creation() {
        let _service = create_service();
    }
}
