use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- ArrangementId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrangementId(Uuid);

impl ArrangementId {
    pub fn new() -> Self {
        ArrangementId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ArrangementId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(ArrangementId)
            .map_err(|_| DomainError::ValidationError("Invalid ArrangementId".to_string()))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ArrangementId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ArrangementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- ArrangementType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrangementType {
    Deposit,
    Loan,
    CurrentAccount,
    SavingsAccount,
    TermDeposit,
    CreditCard,
    TradeFinance,
    Insurance,
}

impl ArrangementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Deposit => "deposit",
            Self::Loan => "loan",
            Self::CurrentAccount => "current_account",
            Self::SavingsAccount => "savings_account",
            Self::TermDeposit => "term_deposit",
            Self::CreditCard => "credit_card",
            Self::TradeFinance => "trade_finance",
            Self::Insurance => "insurance",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "deposit" => Ok(Self::Deposit),
            "loan" => Ok(Self::Loan),
            "current_account" => Ok(Self::CurrentAccount),
            "savings_account" => Ok(Self::SavingsAccount),
            "term_deposit" => Ok(Self::TermDeposit),
            "credit_card" => Ok(Self::CreditCard),
            "trade_finance" => Ok(Self::TradeFinance),
            "insurance" => Ok(Self::Insurance),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown arrangement type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for ArrangementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ArrangementStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrangementStatus {
    Proposed,
    Active,
    Suspended,
    Matured,
    Closed,
}

impl ArrangementStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Matured => "matured",
            Self::Closed => "closed",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "proposed" => Ok(Self::Proposed),
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "matured" => Ok(Self::Matured),
            "closed" => Ok(Self::Closed),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown arrangement status: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for ArrangementStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- RenewalType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RenewalType {
    None,
    Automatic,
    Manual,
}

impl RenewalType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "none" => Ok(Self::None),
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown renewal type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for RenewalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ArrangementEventId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrangementEventId(Uuid);

impl ArrangementEventId {
    pub fn new() -> Self {
        ArrangementEventId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ArrangementEventId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ArrangementEventId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ArrangementEventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- ArrangementEventType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrangementEventType {
    Created,
    Activated,
    Modified,
    Suspended,
    Matured,
    Closed,
    RenewalTriggered,
    InterestApplied,
    FeeCharged,
}

impl ArrangementEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Activated => "activated",
            Self::Modified => "modified",
            Self::Suspended => "suspended",
            Self::Matured => "matured",
            Self::Closed => "closed",
            Self::RenewalTriggered => "renewal_triggered",
            Self::InterestApplied => "interest_applied",
            Self::FeeCharged => "fee_charged",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "created" => Ok(Self::Created),
            "activated" => Ok(Self::Activated),
            "modified" => Ok(Self::Modified),
            "suspended" => Ok(Self::Suspended),
            "matured" => Ok(Self::Matured),
            "closed" => Ok(Self::Closed),
            "renewal_triggered" => Ok(Self::RenewalTriggered),
            "interest_applied" => Ok(Self::InterestApplied),
            "fee_charged" => Ok(Self::FeeCharged),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown event type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for ArrangementEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ArrangementBundleId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrangementBundleId(Uuid);

impl ArrangementBundleId {
    pub fn new() -> Self {
        ArrangementBundleId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ArrangementBundleId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ArrangementBundleId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ArrangementBundleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
