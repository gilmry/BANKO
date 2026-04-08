use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// TASK 3: BC10 FOREIGN EXCHANGE ADVANCED (FR-130 to FR-135)
// FX Hedging, Position Management, Regulatory Reporting
// ============================================================

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FxForwardId(Uuid);

impl FxForwardId {
    pub fn new() -> Self {
        FxForwardId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        FxForwardId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FxForwardId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FxForwardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FxSwapId(Uuid);

impl FxSwapId {
    pub fn new() -> Self {
        FxSwapId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        FxSwapId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FxSwapId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FxSwapId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FxOptionId(Uuid);

impl FxOptionId {
    pub fn new() -> Self {
        FxOptionId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        FxOptionId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FxOptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FxOptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FxPositionLimitId(Uuid);

impl FxPositionLimitId {
    pub fn new() -> Self {
        FxPositionLimitId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        FxPositionLimitId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FxPositionLimitId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FxPositionLimitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FxRegulatoryReportId(Uuid);

impl FxRegulatoryReportId {
    pub fn new() -> Self {
        FxRegulatoryReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        FxRegulatoryReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FxRegulatoryReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FxRegulatoryReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

impl OptionType {
    pub fn as_str(&self) -> &str {
        match self {
            OptionType::Call => "Call",
            OptionType::Put => "Put",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionStyle {
    European,
    American,
}

impl OptionStyle {
    pub fn as_str(&self) -> &str {
        match self {
            OptionStyle::European => "European",
            OptionStyle::American => "American",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HedgeType {
    Economic,
    Accounting,
}

impl HedgeType {
    pub fn as_str(&self) -> &str {
        match self {
            HedgeType::Economic => "Economic",
            HedgeType::Accounting => "Accounting",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionType {
    Long,
    Short,
}

impl PositionType {
    pub fn as_str(&self) -> &str {
        match self {
            PositionType::Long => "Long",
            PositionType::Short => "Short",
        }
    }
}

// --- FR-130: FX Forward Contract ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxForward {
    id: FxForwardId,
    institution_id: Uuid,
    beneficiary_id: Uuid,
    source_currency: String,
    target_currency: String,
    source_amount: i64,
    /// Agreed forward rate (in decimal: 1.23 = 1.23 TND per USD)
    forward_rate: f64,
    /// Settlement date (forward date, typically > 2 days)
    settlement_date: chrono::NaiveDate,
    hedge_type: HedgeType,
    created_at: DateTime<Utc>,
    settled: bool,
    settled_at: Option<DateTime<Utc>>,
}

impl FxForward {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        institution_id: Uuid,
        beneficiary_id: Uuid,
        source_currency: String,
        target_currency: String,
        source_amount: i64,
        forward_rate: f64,
        settlement_date: chrono::NaiveDate,
        hedge_type: HedgeType,
    ) -> Result<Self, DomainError> {
        if source_amount <= 0 {
            return Err(DomainError::InvalidFxTransition(
                "Amount must be positive".to_string(),
            ));
        }
        if forward_rate <= 0.0 {
            return Err(DomainError::InvalidFxTransition(
                "Forward rate must be positive".to_string(),
            ));
        }

        let today = chrono::Utc::now().date_naive();
        if settlement_date <= today {
            return Err(DomainError::InvalidFxTransition(
                "Settlement date must be in future".to_string(),
            ));
        }

        Ok(FxForward {
            id: FxForwardId::new(),
            institution_id,
            beneficiary_id,
            source_currency,
            target_currency,
            source_amount,
            forward_rate,
            settlement_date,
            hedge_type,
            created_at: Utc::now(),
            settled: false,
            settled_at: None,
        })
    }

    /// Calculate target amount = source_amount * forward_rate
    pub fn calculate_target_amount(&self) -> i64 {
        ((self.source_amount as f64) * self.forward_rate) as i64
    }

    pub fn mark_settled(&mut self) {
        self.settled = true;
        self.settled_at = Some(Utc::now());
    }

    // Getters
    pub fn id(&self) -> &FxForwardId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn beneficiary_id(&self) -> Uuid {
        self.beneficiary_id
    }
    pub fn source_currency(&self) -> &str {
        &self.source_currency
    }
    pub fn target_currency(&self) -> &str {
        &self.target_currency
    }
    pub fn source_amount(&self) -> i64 {
        self.source_amount
    }
    pub fn forward_rate(&self) -> f64 {
        self.forward_rate
    }
    pub fn settlement_date(&self) -> chrono::NaiveDate {
        self.settlement_date
    }
    pub fn hedge_type(&self) -> HedgeType {
        self.hedge_type
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn settled(&self) -> bool {
        self.settled
    }
    pub fn settled_at(&self) -> Option<DateTime<Utc>> {
        self.settled_at
    }
}

// --- FR-130: FX Swap (dual forward legs) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxSwap {
    id: FxSwapId,
    institution_id: Uuid,
    beneficiary_id: Uuid,
    currency_pair: String, // e.g., "USD/TND"
    source_currency: String,
    target_currency: String,
    /// Near leg (spot-like) amount
    near_amount: i64,
    /// Far leg (forward) amount
    far_amount: i64,
    near_rate: f64,
    far_rate: f64,
    near_settlement: chrono::NaiveDate,
    far_settlement: chrono::NaiveDate,
    created_at: DateTime<Utc>,
}

impl FxSwap {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        institution_id: Uuid,
        beneficiary_id: Uuid,
        source_currency: String,
        target_currency: String,
        near_amount: i64,
        far_amount: i64,
        near_rate: f64,
        far_rate: f64,
        near_settlement: chrono::NaiveDate,
        far_settlement: chrono::NaiveDate,
    ) -> Result<Self, DomainError> {
        if near_amount <= 0 || far_amount <= 0 {
            return Err(DomainError::InvalidFxTransition(
                "Amounts must be positive".to_string(),
            ));
        }
        if near_rate <= 0.0 || far_rate <= 0.0 {
            return Err(DomainError::InvalidFxTransition(
                "Rates must be positive".to_string(),
            ));
        }

        let today = chrono::Utc::now().date_naive();
        if near_settlement <= today || far_settlement <= today {
            return Err(DomainError::InvalidFxTransition(
                "Settlement dates must be in future".to_string(),
            ));
        }
        if far_settlement <= near_settlement {
            return Err(DomainError::InvalidFxTransition(
                "Far settlement must be after near settlement".to_string(),
            ));
        }

        let currency_pair = format!("{}/{}", source_currency, target_currency);

        Ok(FxSwap {
            id: FxSwapId::new(),
            institution_id,
            beneficiary_id,
            currency_pair,
            source_currency,
            target_currency,
            near_amount,
            far_amount,
            near_rate,
            far_rate,
            near_settlement,
            far_settlement,
            created_at: Utc::now(),
        })
    }

    pub fn swap_points(&self) -> f64 {
        self.far_rate - self.near_rate
    }

    // Getters
    pub fn id(&self) -> &FxSwapId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn beneficiary_id(&self) -> Uuid {
        self.beneficiary_id
    }
    pub fn currency_pair(&self) -> &str {
        &self.currency_pair
    }
    pub fn source_currency(&self) -> &str {
        &self.source_currency
    }
    pub fn target_currency(&self) -> &str {
        &self.target_currency
    }
    pub fn near_amount(&self) -> i64 {
        self.near_amount
    }
    pub fn far_amount(&self) -> i64 {
        self.far_amount
    }
    pub fn near_rate(&self) -> f64 {
        self.near_rate
    }
    pub fn far_rate(&self) -> f64 {
        self.far_rate
    }
    pub fn near_settlement(&self) -> chrono::NaiveDate {
        self.near_settlement
    }
    pub fn far_settlement(&self) -> chrono::NaiveDate {
        self.far_settlement
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- FR-130: FX Option ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxOption {
    id: FxOptionId,
    institution_id: Uuid,
    beneficiary_id: Uuid,
    source_currency: String,
    target_currency: String,
    notional_amount: i64,
    strike_rate: f64,
    option_type: OptionType,
    option_style: OptionStyle,
    premium: i64, // Option premium in base currency
    expiry_date: chrono::NaiveDate,
    created_at: DateTime<Utc>,
    exercised: bool,
}

impl FxOption {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        institution_id: Uuid,
        beneficiary_id: Uuid,
        source_currency: String,
        target_currency: String,
        notional_amount: i64,
        strike_rate: f64,
        option_type: OptionType,
        option_style: OptionStyle,
        premium: i64,
        expiry_date: chrono::NaiveDate,
    ) -> Result<Self, DomainError> {
        if notional_amount <= 0 {
            return Err(DomainError::InvalidFxTransition(
                "Notional amount must be positive".to_string(),
            ));
        }
        if strike_rate <= 0.0 {
            return Err(DomainError::InvalidFxTransition(
                "Strike rate must be positive".to_string(),
            ));
        }
        if premium < 0 {
            return Err(DomainError::InvalidFxTransition(
                "Premium cannot be negative".to_string(),
            ));
        }

        let today = chrono::Utc::now().date_naive();
        if expiry_date <= today {
            return Err(DomainError::InvalidFxTransition(
                "Expiry date must be in future".to_string(),
            ));
        }

        Ok(FxOption {
            id: FxOptionId::new(),
            institution_id,
            beneficiary_id,
            source_currency,
            target_currency,
            notional_amount,
            strike_rate,
            option_type,
            option_style,
            premium,
            expiry_date,
            created_at: Utc::now(),
            exercised: false,
        })
    }

    pub fn is_in_the_money(&self, current_rate: f64) -> bool {
        match self.option_type {
            OptionType::Call => current_rate > self.strike_rate,
            OptionType::Put => current_rate < self.strike_rate,
        }
    }

    pub fn mark_exercised(&mut self) {
        self.exercised = true;
    }

    // Getters
    pub fn id(&self) -> &FxOptionId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn beneficiary_id(&self) -> Uuid {
        self.beneficiary_id
    }
    pub fn source_currency(&self) -> &str {
        &self.source_currency
    }
    pub fn target_currency(&self) -> &str {
        &self.target_currency
    }
    pub fn notional_amount(&self) -> i64 {
        self.notional_amount
    }
    pub fn strike_rate(&self) -> f64 {
        self.strike_rate
    }
    pub fn option_type(&self) -> OptionType {
        self.option_type
    }
    pub fn option_style(&self) -> OptionStyle {
        self.option_style
    }
    pub fn premium(&self) -> i64 {
        self.premium
    }
    pub fn expiry_date(&self) -> chrono::NaiveDate {
        self.expiry_date
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn exercised(&self) -> bool {
        self.exercised
    }
}

// --- FR-131: FX Position Limit Management ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxPositionLimit {
    id: FxPositionLimitId,
    institution_id: Uuid,
    currency_pair: String,
    currency: String,
    /// Maximum long position allowed
    max_long_position: i64,
    /// Maximum short position allowed
    max_short_position: i64,
    /// Current net open position
    current_position: i64,
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}

impl FxPositionLimit {
    pub fn new(
        institution_id: Uuid,
        currency_pair: String,
        currency: String,
        max_long_position: i64,
        max_short_position: i64,
    ) -> Result<Self, DomainError> {
        if max_long_position <= 0 || max_short_position <= 0 {
            return Err(DomainError::InvalidFxTransition(
                "Position limits must be positive".to_string(),
            ));
        }

        Ok(FxPositionLimit {
            id: FxPositionLimitId::new(),
            institution_id,
            currency_pair,
            currency,
            max_long_position,
            max_short_position,
            current_position: 0,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        })
    }

    /// Update position and validate against limits
    pub fn update_position(&mut self, delta: i64) -> Result<(), DomainError> {
        let new_position = self.current_position + delta;

        if new_position > 0 && new_position > self.max_long_position {
            return Err(DomainError::InvalidFxTransition(format!(
                "Long position {} exceeds limit of {}",
                new_position, self.max_long_position
            )));
        }

        if new_position < 0 && new_position.abs() > self.max_short_position {
            return Err(DomainError::InvalidFxTransition(format!(
                "Short position {} exceeds limit of {}",
                new_position.abs(),
                self.max_short_position
            )));
        }

        self.current_position = new_position;
        self.last_updated = Utc::now();
        Ok(())
    }

    pub fn is_within_limits(&self) -> bool {
        if self.current_position > 0 {
            self.current_position <= self.max_long_position
        } else if self.current_position < 0 {
            self.current_position.abs() <= self.max_short_position
        } else {
            true
        }
    }

    // Getters
    pub fn id(&self) -> &FxPositionLimitId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn currency_pair(&self) -> &str {
        &self.currency_pair
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn max_long_position(&self) -> i64 {
        self.max_long_position
    }
    pub fn max_short_position(&self) -> i64 {
        self.max_short_position
    }
    pub fn current_position(&self) -> i64 {
        self.current_position
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }
}

// --- FR-132: FX Regulatory Report (BCT) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxRegulatoryReport {
    id: FxRegulatoryReportId,
    institution_id: Uuid,
    reporting_period: String, // e.g., "2026-Q1"
    /// Total FX exposures by currency pair
    exposures: Vec<(String, i64)>, // (currency_pair, amount)
    /// Net open FX positions
    net_open_positions: Vec<(String, i64)>,
    /// Large exposure count
    large_exposures: i64,
    created_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
}

impl FxRegulatoryReport {
    pub fn new(
        institution_id: Uuid,
        reporting_period: String,
    ) -> Result<Self, DomainError> {
        if reporting_period.is_empty() {
            return Err(DomainError::InvalidFxTransition(
                "Reporting period required".to_string(),
            ));
        }

        Ok(FxRegulatoryReport {
            id: FxRegulatoryReportId::new(),
            institution_id,
            reporting_period,
            exposures: Vec::new(),
            net_open_positions: Vec::new(),
            large_exposures: 0,
            created_at: Utc::now(),
            submitted_at: None,
        })
    }

    pub fn add_exposure(&mut self, currency_pair: String, amount: i64) {
        self.exposures.push((currency_pair, amount));
    }

    pub fn add_position(&mut self, currency_pair: String, net_position: i64) {
        self.net_open_positions.push((currency_pair, net_position));
    }

    pub fn count_large_exposures(&mut self) {
        // Large exposure = > 10% of capital (simplified threshold)
        self.large_exposures = self
            .exposures
            .iter()
            .filter(|(_, amt)| amt.abs() > 100_000_000)
            .count() as i64;
    }

    pub fn mark_submitted(&mut self) {
        self.submitted_at = Some(Utc::now());
    }

    // Getters
    pub fn id(&self) -> &FxRegulatoryReportId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn reporting_period(&self) -> &str {
        &self.reporting_period
    }
    pub fn exposures(&self) -> &[(String, i64)] {
        &self.exposures
    }
    pub fn net_open_positions(&self) -> &[(String, i64)] {
        &self.net_open_positions
    }
    pub fn large_exposures(&self) -> i64 {
        self.large_exposures
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn submitted_at(&self) -> Option<DateTime<Utc>> {
        self.submitted_at
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_institution() -> Uuid {
        Uuid::new_v4()
    }

    fn default_beneficiary() -> Uuid {
        Uuid::new_v4()
    }

    fn future_date(days: i64) -> chrono::NaiveDate {
        chrono::Utc::now().date_naive() + chrono::Duration::days(days)
    }

    // --- FR-130: FX Forward ---

    #[test]
    fn test_fx_forward_creation() {
        let forward = FxForward::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000, // 1M USD
            1.23,      // 1.23 TND/USD
            future_date(30),
            HedgeType::Economic,
        )
        .unwrap();

        assert_eq!(forward.source_currency(), "USD");
        assert_eq!(forward.target_currency(), "TND");
    }

    #[test]
    fn test_fx_forward_calculate_target() {
        let forward = FxForward::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.23,
            future_date(30),
            HedgeType::Economic,
        )
        .unwrap();

        // 1M USD * 1.23 = 1.23M TND
        assert_eq!(forward.calculate_target_amount(), 1_230_000);
    }

    #[test]
    fn test_fx_forward_settlement() {
        let mut forward = FxForward::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.23,
            future_date(30),
            HedgeType::Economic,
        )
        .unwrap();

        assert!(!forward.settled());
        forward.mark_settled();
        assert!(forward.settled());
        assert!(forward.settled_at().is_some());
    }

    #[test]
    fn test_fx_forward_invalid_settlement_date() {
        let result = FxForward::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.23,
            future_date(-1), // In the past
            HedgeType::Economic,
        );
        assert!(result.is_err());
    }

    // --- FR-130: FX Swap ---

    #[test]
    fn test_fx_swap_creation() {
        let swap = FxSwap::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000, // Near: 1M
            1_000_000, // Far: 1M
            1.20,      // Near rate
            1.23,      // Far rate
            future_date(2),
            future_date(30),
        )
        .unwrap();

        assert_eq!(swap.currency_pair(), "USD/TND");
    }

    #[test]
    fn test_fx_swap_swap_points() {
        let swap = FxSwap::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1_000_000,
            1.20,
            1.23,
            future_date(2),
            future_date(30),
        )
        .unwrap();

        // Swap points = far - near = 1.23 - 1.20 = 0.03
        assert!((swap.swap_points() - 0.03).abs() < 0.001);
    }

    #[test]
    fn test_fx_swap_invalid_far_before_near() {
        let result = FxSwap::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1_000_000,
            1.20,
            1.23,
            future_date(30),
            future_date(2), // Before near settlement
        );
        assert!(result.is_err());
    }

    // --- FR-130: FX Option ---

    #[test]
    fn test_fx_option_call_creation() {
        let option = FxOption::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.25,
            OptionType::Call,
            OptionStyle::European,
            10_000, // Premium
            future_date(30),
        )
        .unwrap();

        assert_eq!(option.option_type(), OptionType::Call);
        assert_eq!(option.strike_rate(), 1.25);
    }

    #[test]
    fn test_fx_option_itm_call() {
        let option = FxOption::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.25,
            OptionType::Call,
            OptionStyle::European,
            10_000,
            future_date(30),
        )
        .unwrap();

        // Call ITM if current rate > strike
        assert!(option.is_in_the_money(1.30));
        assert!(!option.is_in_the_money(1.20));
    }

    #[test]
    fn test_fx_option_put_itm() {
        let option = FxOption::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.25,
            OptionType::Put,
            OptionStyle::European,
            10_000,
            future_date(30),
        )
        .unwrap();

        // Put ITM if current rate < strike
        assert!(option.is_in_the_money(1.20));
        assert!(!option.is_in_the_money(1.30));
    }

    #[test]
    fn test_fx_option_exercise() {
        let mut option = FxOption::new(
            default_institution(),
            default_beneficiary(),
            "USD".to_string(),
            "TND".to_string(),
            1_000_000,
            1.25,
            OptionType::Call,
            OptionStyle::European,
            10_000,
            future_date(30),
        )
        .unwrap();

        assert!(!option.exercised());
        option.mark_exercised();
        assert!(option.exercised());
    }

    // --- FR-131: FX Position Limit ---

    #[test]
    fn test_fx_position_limit_creation() {
        let limit = FxPositionLimit::new(
            default_institution(),
            "USD/TND".to_string(),
            "USD".to_string(),
            10_000_000, // Max long: 10M
            5_000_000,  // Max short: 5M
        )
        .unwrap();

        assert_eq!(limit.max_long_position(), 10_000_000);
        assert_eq!(limit.current_position(), 0);
        assert!(limit.is_within_limits());
    }

    #[test]
    fn test_fx_position_limit_long_update() {
        let mut limit = FxPositionLimit::new(
            default_institution(),
            "USD/TND".to_string(),
            "USD".to_string(),
            10_000_000,
            5_000_000,
        )
        .unwrap();

        limit.update_position(5_000_000).unwrap();
        assert_eq!(limit.current_position(), 5_000_000);
        assert!(limit.is_within_limits());
    }

    #[test]
    fn test_fx_position_limit_long_breach() {
        let mut limit = FxPositionLimit::new(
            default_institution(),
            "USD/TND".to_string(),
            "USD".to_string(),
            10_000_000,
            5_000_000,
        )
        .unwrap();

        let result = limit.update_position(15_000_000); // Exceeds limit
        assert!(result.is_err());
    }

    #[test]
    fn test_fx_position_limit_short() {
        let mut limit = FxPositionLimit::new(
            default_institution(),
            "USD/TND".to_string(),
            "USD".to_string(),
            10_000_000,
            5_000_000,
        )
        .unwrap();

        limit.update_position(-3_000_000).unwrap(); // Short 3M
        assert_eq!(limit.current_position(), -3_000_000);
        assert!(limit.is_within_limits());
    }

    #[test]
    fn test_fx_position_limit_short_breach() {
        let mut limit = FxPositionLimit::new(
            default_institution(),
            "USD/TND".to_string(),
            "USD".to_string(),
            10_000_000,
            5_000_000,
        )
        .unwrap();

        let result = limit.update_position(-10_000_000); // Exceeds short limit
        assert!(result.is_err());
    }

    // --- FR-132: FX Regulatory Report ---

    #[test]
    fn test_fx_regulatory_report_creation() {
        let report = FxRegulatoryReport::new(
            default_institution(),
            "2026-Q1".to_string(),
        )
        .unwrap();

        assert_eq!(report.reporting_period(), "2026-Q1");
        assert!(report.exposures().is_empty());
    }

    #[test]
    fn test_fx_regulatory_report_add_exposures() {
        let mut report = FxRegulatoryReport::new(
            default_institution(),
            "2026-Q1".to_string(),
        )
        .unwrap();

        report.add_exposure("USD/TND".to_string(), 50_000_000);
        report.add_exposure("EUR/TND".to_string(), 30_000_000);

        assert_eq!(report.exposures().len(), 2);
    }

    #[test]
    fn test_fx_regulatory_report_count_large_exposures() {
        let mut report = FxRegulatoryReport::new(
            default_institution(),
            "2026-Q1".to_string(),
        )
        .unwrap();

        report.add_exposure("USD/TND".to_string(), 150_000_000); // > 100M threshold
        report.add_exposure("EUR/TND".to_string(), 50_000_000);  // < 100M

        report.count_large_exposures();
        assert_eq!(report.large_exposures(), 1);
    }

    #[test]
    fn test_fx_regulatory_report_submission() {
        let mut report = FxRegulatoryReport::new(
            default_institution(),
            "2026-Q1".to_string(),
        )
        .unwrap();

        assert!(report.submitted_at().is_none());
        report.mark_submitted();
        assert!(report.submitted_at().is_some());
    }

    // --- Enum tests ---

    #[test]
    fn test_option_type_display() {
        assert_eq!(OptionType::Call.as_str(), "Call");
        assert_eq!(OptionType::Put.as_str(), "Put");
    }

    #[test]
    fn test_hedge_type_display() {
        assert_eq!(HedgeType::Economic.as_str(), "Economic");
        assert_eq!(HedgeType::Accounting.as_str(), "Accounting");
    }

    #[test]
    fn test_position_type_display() {
        assert_eq!(PositionType::Long.as_str(), "Long");
        assert_eq!(PositionType::Short.as_str(), "Short");
    }
}
