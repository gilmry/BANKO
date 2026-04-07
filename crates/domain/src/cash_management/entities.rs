use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{Currency, Money};
use crate::account::AccountId;

use super::value_objects::{
    CashForecastId, CashPoolId, ConfidenceLevel, FundingStrategyId, InstrumentType,
    PoolType, SweepAccountId, SweepFrequency, SweepType,
};

// --- SweepAccount aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepAccount {
    id: SweepAccountId,
    source_account_id: AccountId,
    target_account_id: AccountId,
    sweep_type: SweepType,
    threshold_amount: Option<Money>,
    sweep_frequency: SweepFrequency,
    is_active: bool,
    last_sweep_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SweepAccount {
    /// Create a new sweep account. Enforces domain invariants.
    pub fn new(
        source_account_id: AccountId,
        target_account_id: AccountId,
        sweep_type: SweepType,
        threshold_amount: Option<Money>,
        sweep_frequency: SweepFrequency,
    ) -> Result<Self, DomainError> {
        // Invariant: source and target must be different
        if source_account_id == target_account_id {
            return Err(DomainError::ValidationError(
                "Source and target accounts must be different".to_string(),
            ));
        }

        // Invariant: threshold-based sweeps require threshold_amount
        if sweep_type == SweepType::Threshold && threshold_amount.is_none() {
            return Err(DomainError::ValidationError(
                "Threshold-based sweeps must specify threshold_amount".to_string(),
            ));
        }

        // Invariant: threshold_amount must be positive if present
        if let Some(ref amount) = threshold_amount {
            if amount.is_negative() || amount.is_zero() {
                return Err(DomainError::InvalidMoney(
                    "Threshold amount must be positive".to_string(),
                ));
            }
        }

        let now = Utc::now();

        Ok(SweepAccount {
            id: SweepAccountId::new(),
            source_account_id,
            target_account_id,
            sweep_type,
            threshold_amount,
            sweep_frequency,
            is_active: true,
            last_sweep_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: SweepAccountId,
        source_account_id: AccountId,
        target_account_id: AccountId,
        sweep_type: SweepType,
        threshold_amount: Option<Money>,
        sweep_frequency: SweepFrequency,
        is_active: bool,
        last_sweep_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SweepAccount {
            id,
            source_account_id,
            target_account_id,
            sweep_type,
            threshold_amount,
            sweep_frequency,
            is_active,
            last_sweep_at,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &SweepAccountId {
        &self.id
    }

    pub fn source_account_id(&self) -> &AccountId {
        &self.source_account_id
    }

    pub fn target_account_id(&self) -> &AccountId {
        &self.target_account_id
    }

    pub fn sweep_type(&self) -> SweepType {
        self.sweep_type
    }

    pub fn threshold_amount(&self) -> Option<&Money> {
        self.threshold_amount.as_ref()
    }

    pub fn sweep_frequency(&self) -> SweepFrequency {
        self.sweep_frequency
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn last_sweep_at(&self) -> Option<DateTime<Utc>> {
        self.last_sweep_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn activate(&mut self) -> Result<(), DomainError> {
        self.is_active = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.is_active = false;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn record_sweep(&mut self) -> Result<(), DomainError> {
        self.last_sweep_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- CashPool aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPool {
    id: CashPoolId,
    name: String,
    header_account_id: AccountId,
    participant_accounts: Vec<AccountId>,
    pool_type: PoolType,
    currency: Currency,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CashPool {
    /// Create a new cash pool. Enforces domain invariants.
    pub fn new(
        name: &str,
        header_account_id: AccountId,
        participant_accounts: Vec<AccountId>,
        pool_type: PoolType,
        currency: Currency,
    ) -> Result<Self, DomainError> {
        // Invariant: name must not be empty
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Pool name cannot be empty".to_string(),
            ));
        }

        // Invariant: name length
        if name.len() > 255 {
            return Err(DomainError::ValidationError(
                "Pool name cannot exceed 255 characters".to_string(),
            ));
        }

        // Invariant: at least one participant (in addition to header)
        if participant_accounts.is_empty() {
            return Err(DomainError::ValidationError(
                "Pool must have at least one participant account".to_string(),
            ));
        }

        // Invariant: header account must not be in participants
        if participant_accounts.contains(&header_account_id) {
            return Err(DomainError::ValidationError(
                "Header account cannot be a participant in its own pool".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(CashPool {
            id: CashPoolId::new(),
            name: name.to_string(),
            header_account_id,
            participant_accounts,
            pool_type,
            currency,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: CashPoolId,
        name: String,
        header_account_id: AccountId,
        participant_accounts: Vec<AccountId>,
        pool_type: PoolType,
        currency: Currency,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        CashPool {
            id,
            name,
            header_account_id,
            participant_accounts,
            pool_type,
            currency,
            is_active,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &CashPoolId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn header_account_id(&self) -> &AccountId {
        &self.header_account_id
    }

    pub fn participant_accounts(&self) -> &[AccountId] {
        &self.participant_accounts
    }

    pub fn pool_type(&self) -> PoolType {
        self.pool_type
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn add_participant(&mut self, account_id: AccountId) -> Result<(), DomainError> {
        if self.participant_accounts.contains(&account_id) {
            return Err(DomainError::ValidationError(
                "Account is already a participant in this pool".to_string(),
            ));
        }

        if account_id == self.header_account_id {
            return Err(DomainError::ValidationError(
                "Header account cannot be added as participant".to_string(),
            ));
        }

        self.participant_accounts.push(account_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_participant(&mut self, account_id: &AccountId) -> Result<(), DomainError> {
        if !self.participant_accounts.contains(account_id) {
            return Err(DomainError::ValidationError(
                "Account is not a participant in this pool".to_string(),
            ));
        }

        self.participant_accounts.retain(|id| id != account_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), DomainError> {
        self.is_active = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.is_active = false;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- CashForecast entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashForecast {
    id: CashForecastId,
    account_id: AccountId,
    forecast_date: DateTime<Utc>,
    expected_inflows: Money,
    expected_outflows: Money,
    net_position: Money,
    confidence_level: ConfidenceLevel,
    horizon_days: u32,
    created_at: DateTime<Utc>,
}

impl CashForecast {
    /// Create a new cash forecast. Enforces domain invariants.
    pub fn new(
        account_id: AccountId,
        forecast_date: DateTime<Utc>,
        expected_inflows: Money,
        expected_outflows: Money,
        confidence_level: ConfidenceLevel,
        horizon_days: u32,
    ) -> Result<Self, DomainError> {
        // Invariant: inflows and outflows must be non-negative
        if expected_inflows.is_negative() {
            return Err(DomainError::InvalidMoney(
                "Expected inflows cannot be negative".to_string(),
            ));
        }

        if expected_outflows.is_negative() {
            return Err(DomainError::InvalidMoney(
                "Expected outflows cannot be negative".to_string(),
            ));
        }

        // Invariant: horizon must be positive
        if horizon_days == 0 {
            return Err(DomainError::ValidationError(
                "Forecast horizon must be at least 1 day".to_string(),
            ));
        }

        // Calculate net position
        let net_position = expected_inflows.subtract(&expected_outflows)?;

        let now = Utc::now();

        Ok(CashForecast {
            id: CashForecastId::new(),
            account_id,
            forecast_date,
            expected_inflows,
            expected_outflows,
            net_position,
            confidence_level,
            horizon_days,
            created_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(
        id: CashForecastId,
        account_id: AccountId,
        forecast_date: DateTime<Utc>,
        expected_inflows: Money,
        expected_outflows: Money,
        net_position: Money,
        confidence_level: ConfidenceLevel,
        horizon_days: u32,
        created_at: DateTime<Utc>,
    ) -> Self {
        CashForecast {
            id,
            account_id,
            forecast_date,
            expected_inflows,
            expected_outflows,
            net_position,
            confidence_level,
            horizon_days,
            created_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &CashForecastId {
        &self.id
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn forecast_date(&self) -> DateTime<Utc> {
        self.forecast_date
    }

    pub fn expected_inflows(&self) -> &Money {
        &self.expected_inflows
    }

    pub fn expected_outflows(&self) -> &Money {
        &self.expected_outflows
    }

    pub fn net_position(&self) -> &Money {
        &self.net_position
    }

    pub fn confidence_level(&self) -> ConfidenceLevel {
        self.confidence_level
    }

    pub fn horizon_days(&self) -> u32 {
        self.horizon_days
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- LiquidityPosition entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPosition {
    date: DateTime<Utc>,
    currency: Currency,
    total_assets: Money,
    total_liabilities: Money,
    net_position: Money,
    lcr_eligible_assets: Money,
    nsfr_stable_funding: Money,
}

impl LiquidityPosition {
    /// Create a new liquidity position. Enforces domain invariants.
    pub fn new(
        date: DateTime<Utc>,
        currency: Currency,
        total_assets: Money,
        total_liabilities: Money,
        lcr_eligible_assets: Money,
        nsfr_stable_funding: Money,
    ) -> Result<Self, DomainError> {
        // Invariant: amounts must be non-negative
        if total_assets.is_negative() {
            return Err(DomainError::InvalidMoney(
                "Total assets cannot be negative".to_string(),
            ));
        }

        if total_liabilities.is_negative() {
            return Err(DomainError::InvalidMoney(
                "Total liabilities cannot be negative".to_string(),
            ));
        }

        if lcr_eligible_assets.is_negative() {
            return Err(DomainError::InvalidMoney(
                "LCR eligible assets cannot be negative".to_string(),
            ));
        }

        if nsfr_stable_funding.is_negative() {
            return Err(DomainError::InvalidMoney(
                "NSFR stable funding cannot be negative".to_string(),
            ));
        }

        // Invariant: LCR eligible assets <= total assets
        if lcr_eligible_assets.amount_cents() > total_assets.amount_cents() {
            return Err(DomainError::ValidationError(
                "LCR eligible assets cannot exceed total assets".to_string(),
            ));
        }

        // Calculate net position
        let net_position = total_assets.subtract(&total_liabilities)?;

        Ok(LiquidityPosition {
            date,
            currency,
            total_assets,
            total_liabilities,
            net_position,
            lcr_eligible_assets,
            nsfr_stable_funding,
        })
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn total_assets(&self) -> &Money {
        &self.total_assets
    }

    pub fn total_liabilities(&self) -> &Money {
        &self.total_liabilities
    }

    pub fn net_position(&self) -> &Money {
        &self.net_position
    }

    pub fn lcr_eligible_assets(&self) -> &Money {
        &self.lcr_eligible_assets
    }

    pub fn nsfr_stable_funding(&self) -> &Money {
        &self.nsfr_stable_funding
    }

    /// Calculate Liquidity Coverage Ratio (LCR).
    pub fn lcr_ratio(&self) -> Result<f64, DomainError> {
        if self.total_liabilities.is_zero() {
            return Ok(1.0);
        }

        let lcr = self.lcr_eligible_assets.amount() / self.total_liabilities.amount();
        Ok(lcr)
    }

    /// Calculate Net Stable Funding Ratio (NSFR).
    pub fn nsfr_ratio(&self) -> Result<f64, DomainError> {
        if self.total_assets.is_zero() {
            return Ok(1.0);
        }

        let nsfr = self.nsfr_stable_funding.amount() / self.total_assets.amount();
        Ok(nsfr)
    }
}

// --- FundingStrategy aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingStrategy {
    id: FundingStrategyId,
    name: String,
    target_ratio: f64,
    instruments: Vec<InstrumentType>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FundingStrategy {
    /// Create a new funding strategy. Enforces domain invariants.
    pub fn new(
        name: &str,
        target_ratio: f64,
        instruments: Vec<InstrumentType>,
    ) -> Result<Self, DomainError> {
        // Invariant: name must not be empty
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Strategy name cannot be empty".to_string(),
            ));
        }

        if name.len() > 255 {
            return Err(DomainError::ValidationError(
                "Strategy name cannot exceed 255 characters".to_string(),
            ));
        }

        // Invariant: target_ratio must be between 0 and 1
        if !(0.0..=1.0).contains(&target_ratio) {
            return Err(DomainError::ValidationError(
                "Target ratio must be between 0 and 1".to_string(),
            ));
        }

        // Invariant: instruments list must not be empty
        if instruments.is_empty() {
            return Err(DomainError::ValidationError(
                "Funding strategy must include at least one instrument type".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(FundingStrategy {
            id: FundingStrategyId::new(),
            name: name.to_string(),
            target_ratio,
            instruments,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    pub fn reconstitute(
        id: FundingStrategyId,
        name: String,
        target_ratio: f64,
        instruments: Vec<InstrumentType>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        FundingStrategy {
            id,
            name,
            target_ratio,
            instruments,
            is_active,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &FundingStrategyId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target_ratio(&self) -> f64 {
        self.target_ratio
    }

    pub fn instruments(&self) -> &[InstrumentType] {
        &self.instruments
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn activate(&mut self) -> Result<(), DomainError> {
        self.is_active = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.is_active = false;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_target_ratio(&mut self, new_ratio: f64) -> Result<(), DomainError> {
        if !(0.0..=1.0).contains(&new_ratio) {
            return Err(DomainError::ValidationError(
                "Target ratio must be between 0 and 1".to_string(),
            ));
        }

        self.target_ratio = new_ratio;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn add_instrument(&mut self, instrument: InstrumentType) -> Result<(), DomainError> {
        if self.instruments.contains(&instrument) {
            return Err(DomainError::ValidationError(
                "This instrument type is already in the strategy".to_string(),
            ));
        }

        self.instruments.push(instrument);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_instrument(&mut self, instrument: InstrumentType) -> Result<(), DomainError> {
        if !self.instruments.contains(&instrument) {
            return Err(DomainError::ValidationError(
                "This instrument type is not in the strategy".to_string(),
            ));
        }

        if self.instruments.len() == 1 {
            return Err(DomainError::ValidationError(
                "Strategy must have at least one instrument type".to_string(),
            ));
        }

        self.instruments.retain(|&i| i != instrument);
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::CustomerId;

    #[test]
    fn test_sweep_account_new() {
        let source = AccountId::from_uuid(uuid::Uuid::new_v4());
        let target = AccountId::from_uuid(uuid::Uuid::new_v4());
        let sweep = SweepAccount::new(
            source.clone(),
            target.clone(),
            SweepType::ZeroBalance,
            None,
            SweepFrequency::Daily,
        );

        assert!(sweep.is_ok());
        let acc = sweep.unwrap();
        assert_eq!(acc.sweep_type(), SweepType::ZeroBalance);
        assert!(acc.is_active());
    }

    #[test]
    fn test_sweep_account_same_accounts_fails() {
        let account = AccountId::from_uuid(uuid::Uuid::new_v4());
        let sweep = SweepAccount::new(
            account.clone(),
            account.clone(),
            SweepType::ZeroBalance,
            None,
            SweepFrequency::Daily,
        );

        assert!(sweep.is_err());
    }

    #[test]
    fn test_sweep_account_threshold_requires_amount() {
        let source = AccountId::from_uuid(uuid::Uuid::new_v4());
        let target = AccountId::from_uuid(uuid::Uuid::new_v4());
        let sweep = SweepAccount::new(
            source.clone(),
            target.clone(),
            SweepType::Threshold,
            None,
            SweepFrequency::Daily,
        );

        assert!(sweep.is_err());
    }

    #[test]
    fn test_sweep_account_threshold_with_valid_amount() {
        let source = AccountId::from_uuid(uuid::Uuid::new_v4());
        let target = AccountId::from_uuid(uuid::Uuid::new_v4());
        let amount = Money::from_cents(100000, Currency::try_from("TND").unwrap());

        let sweep = SweepAccount::new(
            source.clone(),
            target.clone(),
            SweepType::Threshold,
            Some(amount),
            SweepFrequency::Weekly,
        );

        assert!(sweep.is_ok());
    }

    #[test]
    fn test_sweep_account_deactivate() {
        let source = AccountId::from_uuid(uuid::Uuid::new_v4());
        let target = AccountId::from_uuid(uuid::Uuid::new_v4());
        let mut sweep = SweepAccount::new(
            source.clone(),
            target.clone(),
            SweepType::ZeroBalance,
            None,
            SweepFrequency::Daily,
        )
        .unwrap();

        assert!(sweep.is_active());
        sweep.deactivate().unwrap();
        assert!(!sweep.is_active());
    }

    #[test]
    fn test_cash_pool_new() {
        let header = AccountId::from_uuid(uuid::Uuid::new_v4());
        let participant = AccountId::from_uuid(uuid::Uuid::new_v4());

        let pool = CashPool::new(
            "Main Pool",
            header.clone(),
            vec![participant.clone()],
            PoolType::Physical,
            Currency::try_from("TND").unwrap(),
        );

        assert!(pool.is_ok());
        let p = pool.unwrap();
        assert_eq!(p.name(), "Main Pool");
        assert_eq!(p.pool_type(), PoolType::Physical);
    }

    #[test]
    fn test_cash_pool_header_cannot_be_participant() {
        let header = AccountId::from_uuid(uuid::Uuid::new_v4());

        let pool = CashPool::new(
            "Bad Pool",
            header.clone(),
            vec![header.clone()],
            PoolType::Notional,
            Currency::try_from("TND").unwrap(),
        );

        assert!(pool.is_err());
    }

    #[test]
    fn test_cash_pool_empty_participants_fails() {
        let header = AccountId::from_uuid(uuid::Uuid::new_v4());

        let pool = CashPool::new(
            "Bad Pool",
            header.clone(),
            vec![],
            PoolType::Physical,
            Currency::try_from("TND").unwrap(),
        );

        assert!(pool.is_err());
    }

    #[test]
    fn test_cash_pool_add_participant() {
        let header = AccountId::from_uuid(uuid::Uuid::new_v4());
        let p1 = AccountId::from_uuid(uuid::Uuid::new_v4());
        let p2 = AccountId::from_uuid(uuid::Uuid::new_v4());

        let mut pool = CashPool::new(
            "Pool",
            header.clone(),
            vec![p1.clone()],
            PoolType::Physical,
            Currency::try_from("TND").unwrap(),
        )
        .unwrap();

        pool.add_participant(p2.clone()).unwrap();
        assert_eq!(pool.participant_accounts().len(), 2);
    }

    #[test]
    fn test_cash_pool_add_duplicate_participant() {
        let header = AccountId::from_uuid(uuid::Uuid::new_v4());
        let p1 = AccountId::from_uuid(uuid::Uuid::new_v4());

        let mut pool = CashPool::new(
            "Pool",
            header.clone(),
            vec![p1.clone()],
            PoolType::Physical,
            Currency::try_from("TND").unwrap(),
        )
        .unwrap();

        let result = pool.add_participant(p1.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_cash_forecast_new() {
        let account = AccountId::from_uuid(uuid::Uuid::new_v4());
        let inflows = Money::from_cents(500000, Currency::try_from("TND").unwrap());
        let outflows = Money::from_cents(300000, Currency::try_from("TND").unwrap());

        let forecast = CashForecast::new(
            account.clone(),
            Utc::now(),
            inflows,
            outflows,
            ConfidenceLevel::High,
            30,
        );

        assert!(forecast.is_ok());
        let f = forecast.unwrap();
        assert_eq!(f.horizon_days(), 30);
        assert_eq!(f.confidence_level(), ConfidenceLevel::High);
    }

    #[test]
    fn test_cash_forecast_negative_inflows_fails() {
        let account = AccountId::from_uuid(uuid::Uuid::new_v4());
        let inflows = Money::from_cents(-100000, Currency::try_from("TND").unwrap());
        let outflows = Money::from_cents(50000, Currency::try_from("TND").unwrap());

        let forecast = CashForecast::new(
            account.clone(),
            Utc::now(),
            inflows,
            outflows,
            ConfidenceLevel::Low,
            30,
        );

        assert!(forecast.is_err());
    }

    #[test]
    fn test_cash_forecast_zero_horizon_fails() {
        let account = AccountId::from_uuid(uuid::Uuid::new_v4());
        let inflows = Money::from_cents(100000, Currency::try_from("TND").unwrap());
        let outflows = Money::from_cents(50000, Currency::try_from("TND").unwrap());

        let forecast = CashForecast::new(
            account.clone(),
            Utc::now(),
            inflows,
            outflows,
            ConfidenceLevel::Medium,
            0,
        );

        assert!(forecast.is_err());
    }

    #[test]
    fn test_liquidity_position_new() {
        let total_assets = Money::from_cents(1000000, Currency::try_from("TND").unwrap());
        let total_liabilities = Money::from_cents(600000, Currency::try_from("TND").unwrap());
        let lcr_eligible = Money::from_cents(500000, Currency::try_from("TND").unwrap());
        let nsfr_stable = Money::from_cents(700000, Currency::try_from("TND").unwrap());

        let position = LiquidityPosition::new(
            Utc::now(),
            Currency::try_from("TND").unwrap(),
            total_assets,
            total_liabilities,
            lcr_eligible,
            nsfr_stable,
        );

        assert!(position.is_ok());
    }

    #[test]
    fn test_liquidity_position_lcr_exceeds_assets() {
        let total_assets = Money::from_cents(500000, Currency::try_from("TND").unwrap());
        let total_liabilities = Money::from_cents(300000, Currency::try_from("TND").unwrap());
        let lcr_eligible = Money::from_cents(600000, Currency::try_from("TND").unwrap());
        let nsfr_stable = Money::from_cents(400000, Currency::try_from("TND").unwrap());

        let position = LiquidityPosition::new(
            Utc::now(),
            Currency::try_from("TND").unwrap(),
            total_assets,
            total_liabilities,
            lcr_eligible,
            nsfr_stable,
        );

        assert!(position.is_err());
    }

    #[test]
    fn test_funding_strategy_new() {
        let instruments = vec![InstrumentType::Deposits, InstrumentType::Bonds];

        let strategy = FundingStrategy::new("Core Strategy", 0.7, instruments);

        assert!(strategy.is_ok());
        let s = strategy.unwrap();
        assert_eq!(s.name(), "Core Strategy");
        assert_eq!(s.target_ratio(), 0.7);
        assert!(s.is_active());
    }

    #[test]
    fn test_funding_strategy_invalid_ratio() {
        let instruments = vec![InstrumentType::Deposits];

        let strategy = FundingStrategy::new("Bad Strategy", 1.5, instruments);

        assert!(strategy.is_err());
    }

    #[test]
    fn test_funding_strategy_empty_instruments() {
        let strategy = FundingStrategy::new("Bad Strategy", 0.5, vec![]);

        assert!(strategy.is_err());
    }

    #[test]
    fn test_funding_strategy_add_instrument() {
        let instruments = vec![InstrumentType::Deposits];
        let mut strategy = FundingStrategy::new("Strategy", 0.6, instruments).unwrap();

        strategy
            .add_instrument(InstrumentType::CentralBank)
            .unwrap();

        assert_eq!(strategy.instruments().len(), 2);
    }

    #[test]
    fn test_funding_strategy_remove_last_instrument_fails() {
        let instruments = vec![InstrumentType::Deposits];
        let mut strategy = FundingStrategy::new("Strategy", 0.6, instruments).unwrap();

        let result = strategy.remove_instrument(InstrumentType::Deposits);

        assert!(result.is_err());
    }
}
