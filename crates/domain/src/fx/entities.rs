use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FxOperationId(Uuid);

impl FxOperationId {
    pub fn new() -> Self {
        FxOperationId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        FxOperationId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FxOperationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FxOperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FxStatus {
    Draft,
    Confirmed,
    Settled,
    Rejected,
    Cancelled,
}

impl FxStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FxStatus::Draft => "Draft",
            FxStatus::Confirmed => "Confirmed",
            FxStatus::Settled => "Settled",
            FxStatus::Rejected => "Rejected",
            FxStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Draft" => Ok(FxStatus::Draft),
            "Confirmed" => Ok(FxStatus::Confirmed),
            "Settled" => Ok(FxStatus::Settled),
            "Rejected" => Ok(FxStatus::Rejected),
            "Cancelled" => Ok(FxStatus::Cancelled),
            _ => Err(DomainError::InvalidFxTransition(format!(
                "Unknown FX status: {s}"
            ))),
        }
    }
}

impl fmt::Display for FxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FxOperationType {
    Spot,
    Forward,
    Swap,
}

impl FxOperationType {
    pub fn as_str(&self) -> &str {
        match self {
            FxOperationType::Spot => "Spot",
            FxOperationType::Forward => "Forward",
            FxOperationType::Swap => "Swap",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Spot" => Ok(FxOperationType::Spot),
            "Forward" => Ok(FxOperationType::Forward),
            "Swap" => Ok(FxOperationType::Swap),
            _ => Err(DomainError::InvalidFxOperation(format!(
                "Unknown FX operation type: {s}"
            ))),
        }
    }
}

impl fmt::Display for FxOperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ExchangeRate Value Object ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRate {
    source_currency: String,
    target_currency: String,
    rate: f64,
    valid_from: DateTime<Utc>,
    valid_to: Option<DateTime<Utc>>,
}

impl ExchangeRate {
    pub fn new(
        source_currency: String,
        target_currency: String,
        rate: f64,
    ) -> Result<Self, DomainError> {
        if rate <= 0.0 || rate.is_nan() || rate.is_infinite() {
            return Err(DomainError::InvalidExchangeRate(format!(
                "Rate must be a positive finite number, got {rate}"
            )));
        }
        if source_currency == target_currency {
            return Err(DomainError::SameCurrencyExchange);
        }
        if source_currency.trim().is_empty() || target_currency.trim().is_empty() {
            return Err(DomainError::InvalidExchangeRate(
                "Currency codes cannot be empty".to_string(),
            ));
        }
        Ok(ExchangeRate {
            source_currency,
            target_currency,
            rate,
            valid_from: Utc::now(),
            valid_to: None,
        })
    }

    /// Reconstruct from persistence (no validation).
    pub fn from_raw(
        source_currency: String,
        target_currency: String,
        rate: f64,
        valid_from: DateTime<Utc>,
        valid_to: Option<DateTime<Utc>>,
    ) -> Self {
        ExchangeRate {
            source_currency,
            target_currency,
            rate,
            valid_from,
            valid_to,
        }
    }

    pub fn source_currency(&self) -> &str {
        &self.source_currency
    }
    pub fn target_currency(&self) -> &str {
        &self.target_currency
    }
    pub fn rate(&self) -> f64 {
        self.rate
    }
    pub fn valid_from(&self) -> DateTime<Utc> {
        self.valid_from
    }
    pub fn valid_to(&self) -> Option<DateTime<Utc>> {
        self.valid_to
    }

    pub fn set_valid_to(&mut self, valid_to: DateTime<Utc>) {
        self.valid_to = Some(valid_to);
    }
}

impl fmt::Display for ExchangeRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} = {:.6}",
            self.source_currency, self.target_currency, self.rate
        )
    }
}

// --- FxOperation Aggregate ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxOperation {
    operation_id: FxOperationId,
    account_id: Uuid,
    operation_type: FxOperationType,
    source_currency: String,
    target_currency: String,
    source_amount: i64,
    target_amount: i64,
    rate: f64,
    status: FxStatus,
    reference: String,
    rejection_reason: Option<String>,
    created_at: DateTime<Utc>,
    confirmed_at: Option<DateTime<Utc>>,
    settled_at: Option<DateTime<Utc>>,
}

impl FxOperation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: Uuid,
        operation_type: FxOperationType,
        source_currency: String,
        target_currency: String,
        source_amount: i64,
        rate: f64,
        reference: String,
    ) -> Result<Self, DomainError> {
        if source_amount <= 0 {
            return Err(DomainError::InvalidFxOperation(
                "Source amount must be greater than 0".to_string(),
            ));
        }
        if rate <= 0.0 || rate.is_nan() || rate.is_infinite() {
            return Err(DomainError::InvalidExchangeRate(format!(
                "Rate must be a positive finite number, got {rate}"
            )));
        }
        if source_currency == target_currency {
            return Err(DomainError::SameCurrencyExchange);
        }
        if reference.trim().is_empty() {
            return Err(DomainError::InvalidFxOperation(
                "Reference cannot be empty".to_string(),
            ));
        }
        if source_currency.trim().is_empty() || target_currency.trim().is_empty() {
            return Err(DomainError::InvalidFxOperation(
                "Currency codes cannot be empty".to_string(),
            ));
        }

        let target_amount = (source_amount as f64 * rate) as i64;

        Ok(FxOperation {
            operation_id: FxOperationId::new(),
            account_id,
            operation_type,
            source_currency,
            target_currency,
            source_amount,
            target_amount,
            rate,
            status: FxStatus::Draft,
            reference,
            rejection_reason: None,
            created_at: Utc::now(),
            confirmed_at: None,
            settled_at: None,
        })
    }

    /// Reconstruct from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        operation_id: FxOperationId,
        account_id: Uuid,
        operation_type: FxOperationType,
        source_currency: String,
        target_currency: String,
        source_amount: i64,
        target_amount: i64,
        rate: f64,
        status: FxStatus,
        reference: String,
        rejection_reason: Option<String>,
        created_at: DateTime<Utc>,
        confirmed_at: Option<DateTime<Utc>>,
        settled_at: Option<DateTime<Utc>>,
    ) -> Self {
        FxOperation {
            operation_id,
            account_id,
            operation_type,
            source_currency,
            target_currency,
            source_amount,
            target_amount,
            rate,
            status,
            reference,
            rejection_reason,
            created_at,
            confirmed_at,
            settled_at,
        }
    }

    // --- Getters ---

    pub fn operation_id(&self) -> &FxOperationId {
        &self.operation_id
    }
    pub fn account_id(&self) -> Uuid {
        self.account_id
    }
    pub fn operation_type(&self) -> FxOperationType {
        self.operation_type
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
    pub fn target_amount(&self) -> i64 {
        self.target_amount
    }
    pub fn rate(&self) -> f64 {
        self.rate
    }
    pub fn status(&self) -> FxStatus {
        self.status
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }
    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn confirmed_at(&self) -> Option<DateTime<Utc>> {
        self.confirmed_at
    }
    pub fn settled_at(&self) -> Option<DateTime<Utc>> {
        self.settled_at
    }

    // --- State Transitions ---

    /// Confirm: Draft -> Confirmed
    pub fn confirm(&mut self) -> Result<(), DomainError> {
        if self.status != FxStatus::Draft {
            return Err(DomainError::InvalidFxTransition(format!(
                "Cannot confirm from status {}",
                self.status
            )));
        }
        self.status = FxStatus::Confirmed;
        self.confirmed_at = Some(Utc::now());
        Ok(())
    }

    /// Settle: Confirmed -> Settled
    pub fn settle(&mut self) -> Result<(), DomainError> {
        if self.status != FxStatus::Confirmed {
            return Err(DomainError::InvalidFxTransition(format!(
                "Cannot settle from status {}",
                self.status
            )));
        }
        self.status = FxStatus::Settled;
        self.settled_at = Some(Utc::now());
        Ok(())
    }

    /// Reject: Draft|Confirmed -> Rejected
    pub fn reject(&mut self, reason: String) -> Result<(), DomainError> {
        if !matches!(self.status, FxStatus::Draft | FxStatus::Confirmed) {
            return Err(DomainError::InvalidFxTransition(format!(
                "Cannot reject from status {}",
                self.status
            )));
        }
        self.status = FxStatus::Rejected;
        self.rejection_reason = Some(reason);
        Ok(())
    }

    /// Cancel: Draft -> Cancelled
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status != FxStatus::Draft {
            return Err(DomainError::InvalidFxTransition(format!(
                "Cannot cancel from status {}",
                self.status
            )));
        }
        self.status = FxStatus::Cancelled;
        Ok(())
    }
}

// --- FxPosition ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FxPosition {
    pub currency: String,
    pub long_amount: i64,
    pub short_amount: i64,
    pub net_position: i64,
}

impl FxPosition {
    pub fn new(currency: String, long_amount: i64, short_amount: i64) -> Self {
        FxPosition {
            currency,
            long_amount,
            short_amount,
            net_position: long_amount - short_amount,
        }
    }
}

impl fmt::Display for FxPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: long={} short={} net={}",
            self.currency, self.long_amount, self.short_amount, self.net_position
        )
    }
}

// --- DailyLimit ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyLimit {
    pub account_id: Uuid,
    pub currency: String,
    pub daily_limit: i64,
    pub used_today: i64,
}

impl DailyLimit {
    pub fn new(account_id: Uuid, currency: String, daily_limit: i64, used_today: i64) -> Self {
        DailyLimit {
            account_id,
            currency,
            daily_limit,
            used_today,
        }
    }

    pub fn can_execute(&self, amount: i64) -> bool {
        self.used_today + amount <= self.daily_limit
    }

    pub fn remaining(&self) -> i64 {
        self.daily_limit - self.used_today
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // --- ExchangeRate tests ---

    #[test]
    fn test_exchange_rate_valid() {
        let rate = ExchangeRate::new("TND".to_string(), "EUR".to_string(), 0.30).unwrap();
        assert_eq!(rate.source_currency(), "TND");
        assert_eq!(rate.target_currency(), "EUR");
        assert_eq!(rate.rate(), 0.30);
        assert!(rate.valid_to().is_none());
    }

    #[test]
    fn test_exchange_rate_zero_rejected() {
        let result = ExchangeRate::new("TND".to_string(), "EUR".to_string(), 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_exchange_rate_negative_rejected() {
        let result = ExchangeRate::new("TND".to_string(), "EUR".to_string(), -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_exchange_rate_nan_rejected() {
        let result = ExchangeRate::new("TND".to_string(), "EUR".to_string(), f64::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn test_exchange_rate_infinity_rejected() {
        let result = ExchangeRate::new("TND".to_string(), "EUR".to_string(), f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn test_exchange_rate_same_currency_rejected() {
        let result = ExchangeRate::new("EUR".to_string(), "EUR".to_string(), 1.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(DomainError::SameCurrencyExchange)));
    }

    #[test]
    fn test_exchange_rate_empty_currency_rejected() {
        let result = ExchangeRate::new("".to_string(), "EUR".to_string(), 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_exchange_rate_display() {
        let rate = ExchangeRate::new("TND".to_string(), "EUR".to_string(), 0.30).unwrap();
        assert!(format!("{rate}").contains("TND/EUR"));
    }

    // --- FxOperation tests ---

    fn make_fx_operation() -> FxOperation {
        FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            1_000_000, // 1000.000 TND in millimes
            0.30,
            "FX-2026-001".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_create_fx_operation() {
        let op = make_fx_operation();
        assert_eq!(op.status(), FxStatus::Draft);
        assert_eq!(op.source_currency(), "TND");
        assert_eq!(op.target_currency(), "EUR");
        assert_eq!(op.source_amount(), 1_000_000);
        assert_eq!(op.target_amount(), 300_000); // 1_000_000 * 0.30
        assert_eq!(op.rate(), 0.30);
        assert_eq!(op.operation_type(), FxOperationType::Spot);
        assert!(op.confirmed_at().is_none());
        assert!(op.settled_at().is_none());
        assert!(op.rejection_reason().is_none());
    }

    #[test]
    fn test_fx_operation_same_currency_rejected() {
        let result = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "EUR".to_string(),
            "EUR".to_string(),
            1_000_00,
            1.0,
            "FX-001".to_string(),
        );
        assert!(result.is_err());
        assert!(matches!(result, Err(DomainError::SameCurrencyExchange)));
    }

    #[test]
    fn test_fx_operation_zero_amount_rejected() {
        let result = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            0,
            0.30,
            "FX-001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fx_operation_negative_amount_rejected() {
        let result = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            -100,
            0.30,
            "FX-001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fx_operation_invalid_rate_rejected() {
        let result = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            1_000_000,
            -0.5,
            "FX-001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fx_operation_empty_reference_rejected() {
        let result = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            1_000_000,
            0.30,
            "".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fx_rate_calculation() {
        // 500_000 millimes (500 TND) * 0.30 = 150_000
        let op = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            500_000,
            0.30,
            "FX-CALC-001".to_string(),
        )
        .unwrap();
        assert_eq!(op.target_amount(), 150_000);

        // 1_000_00 cents (1000 EUR) * 3.35 = 335_000
        let op2 = FxOperation::new(
            Uuid::new_v4(),
            FxOperationType::Forward,
            "EUR".to_string(),
            "TND".to_string(),
            1_000_00,
            3.35,
            "FX-CALC-002".to_string(),
        )
        .unwrap();
        assert_eq!(op2.target_amount(), 335_000);
    }

    // --- Status Transition Tests ---

    #[test]
    fn test_confirm_from_draft() {
        let mut op = make_fx_operation();
        assert!(op.confirm().is_ok());
        assert_eq!(op.status(), FxStatus::Confirmed);
        assert!(op.confirmed_at().is_some());
    }

    #[test]
    fn test_settle_from_confirmed() {
        let mut op = make_fx_operation();
        op.confirm().unwrap();
        assert!(op.settle().is_ok());
        assert_eq!(op.status(), FxStatus::Settled);
        assert!(op.settled_at().is_some());
    }

    #[test]
    fn test_cannot_settle_from_draft() {
        let mut op = make_fx_operation();
        let result = op.settle();
        assert!(result.is_err());
        assert!(matches!(result, Err(DomainError::InvalidFxTransition(_))));
    }

    #[test]
    fn test_cannot_confirm_from_confirmed() {
        let mut op = make_fx_operation();
        op.confirm().unwrap();
        let result = op.confirm();
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_from_draft() {
        let mut op = make_fx_operation();
        assert!(op.reject("Compliance issue".to_string()).is_ok());
        assert_eq!(op.status(), FxStatus::Rejected);
        assert_eq!(op.rejection_reason(), Some("Compliance issue"));
    }

    #[test]
    fn test_reject_from_confirmed() {
        let mut op = make_fx_operation();
        op.confirm().unwrap();
        assert!(op.reject("Rate expired".to_string()).is_ok());
        assert_eq!(op.status(), FxStatus::Rejected);
    }

    #[test]
    fn test_cannot_reject_from_settled() {
        let mut op = make_fx_operation();
        op.confirm().unwrap();
        op.settle().unwrap();
        let result = op.reject("Too late".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_from_draft() {
        let mut op = make_fx_operation();
        assert!(op.cancel().is_ok());
        assert_eq!(op.status(), FxStatus::Cancelled);
    }

    #[test]
    fn test_cannot_cancel_from_confirmed() {
        let mut op = make_fx_operation();
        op.confirm().unwrap();
        let result = op.cancel();
        assert!(result.is_err());
    }

    // --- FxOperationType from_str ---

    #[test]
    fn test_fx_operation_type_from_str() {
        assert_eq!(
            FxOperationType::from_str_type("Spot").unwrap(),
            FxOperationType::Spot
        );
        assert_eq!(
            FxOperationType::from_str_type("Forward").unwrap(),
            FxOperationType::Forward
        );
        assert_eq!(
            FxOperationType::from_str_type("Swap").unwrap(),
            FxOperationType::Swap
        );
        assert!(FxOperationType::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_fx_status_from_str() {
        assert_eq!(FxStatus::from_str_type("Draft").unwrap(), FxStatus::Draft);
        assert_eq!(
            FxStatus::from_str_type("Confirmed").unwrap(),
            FxStatus::Confirmed
        );
        assert_eq!(
            FxStatus::from_str_type("Settled").unwrap(),
            FxStatus::Settled
        );
        assert_eq!(
            FxStatus::from_str_type("Rejected").unwrap(),
            FxStatus::Rejected
        );
        assert_eq!(
            FxStatus::from_str_type("Cancelled").unwrap(),
            FxStatus::Cancelled
        );
        assert!(FxStatus::from_str_type("Unknown").is_err());
    }

    // --- FxPosition Tests ---

    #[test]
    fn test_fx_position() {
        let pos = FxPosition::new("EUR".to_string(), 500_000, 300_000);
        assert_eq!(pos.currency, "EUR");
        assert_eq!(pos.long_amount, 500_000);
        assert_eq!(pos.short_amount, 300_000);
        assert_eq!(pos.net_position, 200_000);
    }

    #[test]
    fn test_fx_position_negative_net() {
        let pos = FxPosition::new("USD".to_string(), 100_000, 300_000);
        assert_eq!(pos.net_position, -200_000);
    }

    // --- DailyLimit Tests ---

    #[test]
    fn test_daily_limit_can_execute() {
        let limit = DailyLimit::new(Uuid::new_v4(), "EUR".to_string(), 100_000_000, 50_000_000);
        assert!(limit.can_execute(50_000_000));
        assert!(limit.can_execute(49_000_000));
        assert!(!limit.can_execute(51_000_000));
    }

    #[test]
    fn test_daily_limit_remaining() {
        let limit = DailyLimit::new(Uuid::new_v4(), "EUR".to_string(), 100_000_000, 30_000_000);
        assert_eq!(limit.remaining(), 70_000_000);
    }

    #[test]
    fn test_daily_limit_at_max() {
        let limit = DailyLimit::new(Uuid::new_v4(), "EUR".to_string(), 100_000_000, 100_000_000);
        assert!(!limit.can_execute(1));
        assert_eq!(limit.remaining(), 0);
    }

    #[test]
    fn test_daily_limit_exact_remaining() {
        let limit = DailyLimit::new(Uuid::new_v4(), "EUR".to_string(), 100_000_000, 50_000_000);
        assert!(limit.can_execute(50_000_000)); // exactly at limit
    }

    // --- from_raw reconstruction ---

    #[test]
    fn test_fx_operation_from_raw() {
        let id = FxOperationId::new();
        let account_id = Uuid::new_v4();
        let now = Utc::now();
        let op = FxOperation::from_raw(
            id.clone(),
            account_id,
            FxOperationType::Spot,
            "TND".to_string(),
            "EUR".to_string(),
            1_000_000,
            300_000,
            0.30,
            FxStatus::Settled,
            "FX-RAW-001".to_string(),
            None,
            now,
            Some(now),
            Some(now),
        );
        assert_eq!(op.operation_id(), &id);
        assert_eq!(op.status(), FxStatus::Settled);
        assert_eq!(op.source_amount(), 1_000_000);
        assert_eq!(op.target_amount(), 300_000);
    }
}
