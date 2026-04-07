use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::cash_management::{
    ICashForecastRepository, ICashPoolRepository, IFundingStrategyRepository,
    ILiquidityPositionRepository, ISweepAccountRepository,
};
use banko_domain::cash_management::{
    CashForecast, CashForecastId, CashPool, CashPoolId, ConfidenceLevel, FundingStrategy,
    FundingStrategyId, InstrumentType, LiquidityPosition, PoolType, SweepAccount, SweepAccountId,
    SweepFrequency, SweepType,
};
use banko_domain::shared::{AccountId, Currency, Money};

// --- SweepAccountRepository ---

pub struct PgSweepAccountRepository {
    pool: PgPool,
}

impl PgSweepAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSweepAccountRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SweepAccountRow {
    id: Uuid,
    source_account_id: Uuid,
    target_account_id: Uuid,
    sweep_type: String,
    threshold_amount: Option<i64>,
    threshold_currency: Option<String>,
    sweep_frequency: String,
    is_active: bool,
    last_sweep_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SweepAccountRow {
    fn into_domain(self) -> Result<SweepAccount, String> {
        let id = SweepAccountId::from_uuid(self.id);
        let source_account_id = AccountId::from_uuid(self.source_account_id);
        let target_account_id = AccountId::from_uuid(self.target_account_id);
        let sweep_type =
            SweepType::from_str(&self.sweep_type).map_err(|e| e.to_string())?;
        let sweep_frequency =
            SweepFrequency::from_str(&self.sweep_frequency).map_err(|e| e.to_string())?;

        let threshold_amount = if let Some(amount) = self.threshold_amount {
            let currency = Currency::from_code(&self.threshold_currency.unwrap_or_default())
                .map_err(|e| e.to_string())?;
            Some(Money::from_cents(amount, currency))
        } else {
            None
        };

        Ok(SweepAccount::reconstitute(
            id,
            source_account_id,
            target_account_id,
            sweep_type,
            threshold_amount,
            sweep_frequency,
            self.is_active,
            self.last_sweep_at,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl ISweepAccountRepository for PgSweepAccountRepository {
    async fn save(&self, sweep: &SweepAccount) -> Result<(), String> {
        let threshold_currency = sweep
            .threshold_amount()
            .map(|m| m.currency().code().to_string());
        let threshold_amount = sweep
            .threshold_amount()
            .map(|m| m.as_cents());

        sqlx::query(
            r#"
            INSERT INTO cash_management.sweep_accounts
            (id, source_account_id, target_account_id, sweep_type, threshold_amount,
             threshold_currency, sweep_frequency, is_active, last_sweep_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                sweep_type = EXCLUDED.sweep_type,
                is_active = EXCLUDED.is_active,
                last_sweep_at = EXCLUDED.last_sweep_at,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(sweep.id().as_uuid())
        .bind(sweep.source_account_id().as_uuid())
        .bind(sweep.target_account_id().as_uuid())
        .bind(sweep.sweep_type().to_string())
        .bind(threshold_amount)
        .bind(threshold_currency)
        .bind(sweep.sweep_frequency().to_string())
        .bind(sweep.is_active())
        .bind(sweep.last_sweep_at())
        .bind(sweep.created_at())
        .bind(sweep.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save sweep account: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &SweepAccountId) -> Result<Option<SweepAccount>, String> {
        let row = sqlx::query_as::<_, SweepAccountRow>(
            "SELECT * FROM cash_management.sweep_accounts WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find sweep account: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_source_account(
        &self,
        source_account_id: &AccountId,
    ) -> Result<Vec<SweepAccount>, String> {
        let rows = sqlx::query_as::<_, SweepAccountRow>(
            "SELECT * FROM cash_management.sweep_accounts WHERE source_account_id = $1",
        )
        .bind(source_account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find sweep accounts: {e}"))?;

        let mut sweeps = Vec::new();
        for row in rows {
            sweeps.push(row.into_domain()?);
        }
        Ok(sweeps)
    }

    async fn delete(&self, id: &SweepAccountId) -> Result<(), String> {
        sqlx::query("DELETE FROM cash_management.sweep_accounts WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete sweep account: {e}"))?;

        Ok(())
    }
}

// --- CashPoolRepository ---

pub struct PgCashPoolRepository {
    pool: PgPool,
}

impl PgCashPoolRepository {
    pub fn new(pool: PgPool) -> Self {
        PgCashPoolRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct CashPoolRow {
    id: Uuid,
    name: String,
    header_account_id: Uuid,
    participant_accounts: Vec<Uuid>,
    pool_type: String,
    currency: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CashPoolRow {
    fn into_domain(self) -> Result<CashPool, String> {
        let id = CashPoolId::from_uuid(self.id);
        let header_account_id = AccountId::from_uuid(self.header_account_id);
        let participant_accounts: Vec<AccountId> = self
            .participant_accounts
            .into_iter()
            .map(AccountId::from_uuid)
            .collect();
        let pool_type = PoolType::from_str(&self.pool_type).map_err(|e| e.to_string())?;
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;

        Ok(CashPool::reconstitute(
            id,
            self.name,
            header_account_id,
            participant_accounts,
            pool_type,
            currency,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl ICashPoolRepository for PgCashPoolRepository {
    async fn save(&self, pool: &CashPool) -> Result<(), String> {
        let participant_uuids: Vec<Uuid> = pool
            .participant_accounts()
            .iter()
            .map(|id| *id.as_uuid())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO cash_management.cash_pools
            (id, name, header_account_id, participant_accounts, pool_type, currency, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                participant_accounts = EXCLUDED.participant_accounts,
                pool_type = EXCLUDED.pool_type,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(pool.id().as_uuid())
        .bind(pool.name())
        .bind(pool.header_account_id().as_uuid())
        .bind(participant_uuids)
        .bind(pool.pool_type().to_string())
        .bind(pool.currency().code())
        .bind(pool.is_active())
        .bind(pool.created_at())
        .bind(pool.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save cash pool: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CashPoolId) -> Result<Option<CashPool>, String> {
        let row = sqlx::query_as::<_, CashPoolRow>(
            "SELECT * FROM cash_management.cash_pools WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find cash pool: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_all_active(&self) -> Result<Vec<CashPool>, String> {
        let rows = sqlx::query_as::<_, CashPoolRow>(
            "SELECT * FROM cash_management.cash_pools WHERE is_active = true",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find cash pools: {e}"))?;

        let mut pools = Vec::new();
        for row in rows {
            pools.push(row.into_domain()?);
        }
        Ok(pools)
    }

    async fn find_by_header_account(
        &self,
        header_account_id: &AccountId,
    ) -> Result<Option<CashPool>, String> {
        let row = sqlx::query_as::<_, CashPoolRow>(
            "SELECT * FROM cash_management.cash_pools WHERE header_account_id = $1",
        )
        .bind(header_account_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find cash pool: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, id: &CashPoolId) -> Result<(), String> {
        sqlx::query("DELETE FROM cash_management.cash_pools WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete cash pool: {e}"))?;

        Ok(())
    }
}

// --- CashForecastRepository ---

pub struct PgCashForecastRepository {
    pool: PgPool,
}

impl PgCashForecastRepository {
    pub fn new(pool: PgPool) -> Self {
        PgCashForecastRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct CashForecastRow {
    id: Uuid,
    account_id: Uuid,
    forecast_date: DateTime<Utc>,
    expected_inflows: i64,
    expected_outflows: i64,
    net_position: i64,
    currency: String,
    confidence_level: String,
    horizon_days: i32,
    created_at: DateTime<Utc>,
}

impl CashForecastRow {
    fn into_domain(self) -> Result<CashForecast, String> {
        let id = CashForecastId::from_uuid(self.id);
        let account_id = AccountId::from_uuid(self.account_id);
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let expected_inflows = Money::from_cents(self.expected_inflows, currency.clone());
        let expected_outflows = Money::from_cents(self.expected_outflows, currency.clone());
        let net_position = Money::from_cents(self.net_position, currency);
        let confidence_level =
            ConfidenceLevel::from_str(&self.confidence_level).map_err(|e| e.to_string())?;

        Ok(CashForecast::reconstitute(
            id,
            account_id,
            self.forecast_date,
            expected_inflows,
            expected_outflows,
            net_position,
            confidence_level,
            self.horizon_days as u32,
            self.created_at,
        ))
    }
}

#[async_trait]
impl ICashForecastRepository for PgCashForecastRepository {
    async fn save(&self, forecast: &CashForecast) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO cash_management.cash_forecasts
            (id, account_id, forecast_date, expected_inflows, expected_outflows, net_position,
             currency, confidence_level, horizon_days, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(forecast.id().as_uuid())
        .bind(forecast.account_id().as_uuid())
        .bind(forecast.forecast_date())
        .bind(forecast.expected_inflows().as_cents())
        .bind(forecast.expected_outflows().as_cents())
        .bind(forecast.net_position().as_cents())
        .bind(forecast.expected_inflows().currency().code())
        .bind(forecast.confidence_level().to_string())
        .bind(forecast.horizon_days() as i32)
        .bind(forecast.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save cash forecast: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CashForecastId) -> Result<Option<CashForecast>, String> {
        let row = sqlx::query_as::<_, CashForecastRow>(
            "SELECT * FROM cash_management.cash_forecasts WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find cash forecast: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_latest_by_account(
        &self,
        account_id: &AccountId,
    ) -> Result<Option<CashForecast>, String> {
        let row = sqlx::query_as::<_, CashForecastRow>(
            r#"
            SELECT * FROM cash_management.cash_forecasts
            WHERE account_id = $1
            ORDER BY forecast_date DESC
            LIMIT 1
            "#,
        )
        .bind(account_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find cash forecast: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_date_range(
        &self,
        account_id: &AccountId,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<Vec<CashForecast>, String> {
        let rows = sqlx::query_as::<_, CashForecastRow>(
            r#"
            SELECT * FROM cash_management.cash_forecasts
            WHERE account_id = $1 AND forecast_date >= $2 AND forecast_date <= $3
            ORDER BY forecast_date DESC
            "#,
        )
        .bind(account_id.as_uuid())
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find cash forecasts: {e}"))?;

        let mut forecasts = Vec::new();
        for row in rows {
            forecasts.push(row.into_domain()?);
        }
        Ok(forecasts)
    }
}

// --- LiquidityPositionRepository ---

pub struct PgLiquidityPositionRepository {
    pool: PgPool,
}

impl PgLiquidityPositionRepository {
    pub fn new(pool: PgPool) -> Self {
        PgLiquidityPositionRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct LiquidityPositionRow {
    date: DateTime<Utc>,
    currency: String,
    total_assets: i64,
    total_liabilities: i64,
    net_position: i64,
    lcr_eligible_assets: i64,
    nsfr_stable_funding: i64,
}

impl LiquidityPositionRow {
    fn into_domain(self) -> Result<LiquidityPosition, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let total_assets = Money::from_cents(self.total_assets, currency.clone());
        let total_liabilities = Money::from_cents(self.total_liabilities, currency.clone());
        let net_position = Money::from_cents(self.net_position, currency.clone());
        let lcr_eligible_assets = Money::from_cents(self.lcr_eligible_assets, currency.clone());
        let nsfr_stable_funding = Money::from_cents(self.nsfr_stable_funding, currency);

        Ok(LiquidityPosition::new(
            self.date,
            Currency::from_code(&self.currency).map_err(|e| e.to_string())?,
            total_assets,
            total_liabilities,
            lcr_eligible_assets,
            nsfr_stable_funding,
        )?)
    }
}

#[async_trait]
impl ILiquidityPositionRepository for PgLiquidityPositionRepository {
    async fn save(&self, position: &LiquidityPosition) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO cash_management.liquidity_positions
            (date, currency, total_assets, total_liabilities, net_position, lcr_eligible_assets, nsfr_stable_funding)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (date, currency) DO UPDATE SET
                total_assets = EXCLUDED.total_assets,
                total_liabilities = EXCLUDED.total_liabilities,
                net_position = EXCLUDED.net_position,
                lcr_eligible_assets = EXCLUDED.lcr_eligible_assets,
                nsfr_stable_funding = EXCLUDED.nsfr_stable_funding
            "#,
        )
        .bind(position.date())
        .bind(position.currency().code())
        .bind(position.total_assets().as_cents())
        .bind(position.total_liabilities().as_cents())
        .bind(position.net_position().as_cents())
        .bind(position.lcr_eligible_assets().as_cents())
        .bind(position.nsfr_stable_funding().as_cents())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save liquidity position: {e}"))?;

        Ok(())
    }

    async fn find_latest(
        &self,
        currency: &Currency,
    ) -> Result<Option<LiquidityPosition>, String> {
        let row = sqlx::query_as::<_, LiquidityPositionRow>(
            r#"
            SELECT * FROM cash_management.liquidity_positions
            WHERE currency = $1
            ORDER BY date DESC
            LIMIT 1
            "#,
        )
        .bind(currency.code())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find liquidity position: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_date_range(
        &self,
        currency: &Currency,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<Vec<LiquidityPosition>, String> {
        let rows = sqlx::query_as::<_, LiquidityPositionRow>(
            r#"
            SELECT * FROM cash_management.liquidity_positions
            WHERE currency = $1 AND date >= $2 AND date <= $3
            ORDER BY date DESC
            "#,
        )
        .bind(currency.code())
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find liquidity positions: {e}"))?;

        let mut positions = Vec::new();
        for row in rows {
            positions.push(row.into_domain()?);
        }
        Ok(positions)
    }
}

// --- FundingStrategyRepository ---

pub struct PgFundingStrategyRepository {
    pool: PgPool,
}

impl PgFundingStrategyRepository {
    pub fn new(pool: PgPool) -> Self {
        PgFundingStrategyRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct FundingStrategyRow {
    id: Uuid,
    name: String,
    target_ratio: f64,
    instruments: Vec<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FundingStrategyRow {
    fn into_domain(self) -> Result<FundingStrategy, String> {
        let id = FundingStrategyId::from_uuid(self.id);
        let instruments: Result<Vec<_>, _> = self
            .instruments
            .into_iter()
            .map(|s| InstrumentType::from_str(&s))
            .collect::<Result<_, _>>()
            .map_err(|e| e.to_string());

        let instruments = instruments?;

        Ok(FundingStrategy::reconstitute(
            id,
            self.name,
            self.target_ratio,
            instruments,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IFundingStrategyRepository for PgFundingStrategyRepository {
    async fn save(&self, strategy: &FundingStrategy) -> Result<(), String> {
        let instruments: Vec<String> = strategy
            .instruments()
            .iter()
            .map(|i| i.to_string())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO cash_management.funding_strategies
            (id, name, target_ratio, instruments, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                target_ratio = EXCLUDED.target_ratio,
                instruments = EXCLUDED.instruments,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(strategy.id().as_uuid())
        .bind(strategy.name())
        .bind(strategy.target_ratio())
        .bind(instruments)
        .bind(strategy.is_active())
        .bind(strategy.created_at())
        .bind(strategy.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save funding strategy: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &FundingStrategyId) -> Result<Option<FundingStrategy>, String> {
        let row = sqlx::query_as::<_, FundingStrategyRow>(
            "SELECT * FROM cash_management.funding_strategies WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find funding strategy: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_all_active(&self) -> Result<Vec<FundingStrategy>, String> {
        let rows = sqlx::query_as::<_, FundingStrategyRow>(
            "SELECT * FROM cash_management.funding_strategies WHERE is_active = true",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find funding strategies: {e}"))?;

        let mut strategies = Vec::new();
        for row in rows {
            strategies.push(row.into_domain()?);
        }
        Ok(strategies)
    }

    async fn delete(&self, id: &FundingStrategyId) -> Result<(), String> {
        sqlx::query("DELETE FROM cash_management.funding_strategies WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete funding strategy: {e}"))?;

        Ok(())
    }
}
