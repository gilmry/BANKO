use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- SweepAccountId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SweepAccountId(Uuid);

impl SweepAccountId {
    pub fn new() -> Self {
        SweepAccountId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        SweepAccountId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(SweepAccountId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid sweep account ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SweepAccountId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SweepAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- SweepType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SweepType {
    ZeroBalance,
    TargetBalance,
    Threshold,
}

impl SweepType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "zero_balance" => Ok(SweepType::ZeroBalance),
            "target_balance" => Ok(SweepType::TargetBalance),
            "threshold" => Ok(SweepType::Threshold),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown sweep type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SweepType::ZeroBalance => "ZeroBalance",
            SweepType::TargetBalance => "TargetBalance",
            SweepType::Threshold => "Threshold",
        }
    }
}

impl fmt::Display for SweepType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SweepFrequency ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SweepFrequency {
    Daily,
    Weekly,
    Monthly,
}

impl SweepFrequency {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(SweepFrequency::Daily),
            "weekly" => Ok(SweepFrequency::Weekly),
            "monthly" => Ok(SweepFrequency::Monthly),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown sweep frequency: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SweepFrequency::Daily => "Daily",
            SweepFrequency::Weekly => "Weekly",
            SweepFrequency::Monthly => "Monthly",
        }
    }
}

impl fmt::Display for SweepFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CashPoolId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CashPoolId(Uuid);

impl CashPoolId {
    pub fn new() -> Self {
        CashPoolId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        CashPoolId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(CashPoolId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid cash pool ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CashPoolId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CashPoolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- PoolType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoolType {
    Notional,
    Physical,
}

impl PoolType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "notional" => Ok(PoolType::Notional),
            "physical" => Ok(PoolType::Physical),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown pool type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PoolType::Notional => "Notional",
            PoolType::Physical => "Physical",
        }
    }
}

impl fmt::Display for PoolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CashForecastId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CashForecastId(Uuid);

impl CashForecastId {
    pub fn new() -> Self {
        CashForecastId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        CashForecastId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(CashForecastId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid cash forecast ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CashForecastId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CashForecastId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- ConfidenceLevel ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

impl ConfidenceLevel {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "low" => Ok(ConfidenceLevel::Low),
            "medium" => Ok(ConfidenceLevel::Medium),
            "high" => Ok(ConfidenceLevel::High),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown confidence level: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConfidenceLevel::Low => "Low",
            ConfidenceLevel::Medium => "Medium",
            ConfidenceLevel::High => "High",
        }
    }
}

impl fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- FundingStrategyId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FundingStrategyId(Uuid);

impl FundingStrategyId {
    pub fn new() -> Self {
        FundingStrategyId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        FundingStrategyId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(FundingStrategyId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid funding strategy ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for FundingStrategyId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FundingStrategyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- InstrumentType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstrumentType {
    Deposits,
    Interbank,
    Bonds,
    CentralBank,
}

impl InstrumentType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "deposits" => Ok(InstrumentType::Deposits),
            "interbank" => Ok(InstrumentType::Interbank),
            "bonds" => Ok(InstrumentType::Bonds),
            "central_bank" => Ok(InstrumentType::CentralBank),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown instrument type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            InstrumentType::Deposits => "Deposits",
            InstrumentType::Interbank => "Interbank",
            InstrumentType::Bonds => "Bonds",
            InstrumentType::CentralBank => "CentralBank",
        }
    }
}

impl fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweep_account_id_new() {
        let id = SweepAccountId::new();
        assert!(!id.as_uuid().to_string().is_empty());
    }

    #[test]
    fn test_sweep_account_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = SweepAccountId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_sweep_account_id_parse() {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();
        let parsed = SweepAccountId::parse(&uuid_str).unwrap();
        assert_eq!(parsed.as_uuid(), &uuid);
    }

    #[test]
    fn test_sweep_account_id_parse_invalid() {
        let result = SweepAccountId::parse("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_sweep_type_from_str() {
        assert_eq!(SweepType::from_str("zero_balance").unwrap(), SweepType::ZeroBalance);
        assert_eq!(SweepType::from_str("target_balance").unwrap(), SweepType::TargetBalance);
        assert_eq!(SweepType::from_str("threshold").unwrap(), SweepType::Threshold);
    }

    #[test]
    fn test_sweep_type_from_str_case_insensitive() {
        assert_eq!(SweepType::from_str("ZERO_BALANCE").unwrap(), SweepType::ZeroBalance);
    }

    #[test]
    fn test_sweep_type_from_str_invalid() {
        let result = SweepType::from_str("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_sweep_type_as_str() {
        assert_eq!(SweepType::ZeroBalance.as_str(), "ZeroBalance");
    }

    #[test]
    fn test_sweep_frequency_from_str() {
        assert_eq!(SweepFrequency::from_str("daily").unwrap(), SweepFrequency::Daily);
        assert_eq!(SweepFrequency::from_str("weekly").unwrap(), SweepFrequency::Weekly);
        assert_eq!(SweepFrequency::from_str("monthly").unwrap(), SweepFrequency::Monthly);
    }

    #[test]
    fn test_pool_type_from_str() {
        assert_eq!(PoolType::from_str("notional").unwrap(), PoolType::Notional);
        assert_eq!(PoolType::from_str("physical").unwrap(), PoolType::Physical);
    }

    #[test]
    fn test_confidence_level_from_str() {
        assert_eq!(ConfidenceLevel::from_str("low").unwrap(), ConfidenceLevel::Low);
        assert_eq!(ConfidenceLevel::from_str("medium").unwrap(), ConfidenceLevel::Medium);
        assert_eq!(ConfidenceLevel::from_str("high").unwrap(), ConfidenceLevel::High);
    }

    #[test]
    fn test_instrument_type_from_str() {
        assert_eq!(InstrumentType::from_str("deposits").unwrap(), InstrumentType::Deposits);
        assert_eq!(InstrumentType::from_str("interbank").unwrap(), InstrumentType::Interbank);
        assert_eq!(InstrumentType::from_str("bonds").unwrap(), InstrumentType::Bonds);
        assert_eq!(InstrumentType::from_str("central_bank").unwrap(), InstrumentType::CentralBank);
    }

    #[test]
    fn test_cash_pool_id_default() {
        let id = CashPoolId::default();
        assert!(!id.as_uuid().to_string().is_empty());
    }

    #[test]