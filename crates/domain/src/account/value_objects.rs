use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- AccountId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        AccountId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(AccountId)
            .map_err(|_| DomainError::AccountNotFound)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AccountId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- MovementId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MovementId(Uuid);

impl MovementId {
    pub fn new() -> Self {
        MovementId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        MovementId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for MovementId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MovementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- AccountType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    Current,
    Savings,
    TimeDeposit,
}

impl AccountType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "current" => Ok(AccountType::Current),
            "savings" => Ok(AccountType::Savings),
            "timedeposit" | "time_deposit" => Ok(AccountType::TimeDeposit),
            _ => Err(DomainError::InvalidAccountType(format!(
                "Unknown account type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AccountType::Current => "Current",
            AccountType::Savings => "Savings",
            AccountType::TimeDeposit => "TimeDeposit",
        }
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- AccountStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Closed,
    Suspended,
}

impl AccountStatus {
    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "active" => Ok(AccountStatus::Active),
            "closed" => Ok(AccountStatus::Closed),
            "suspended" => Ok(AccountStatus::Suspended),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown account status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AccountStatus::Active => "Active",
            AccountStatus::Closed => "Closed",
            AccountStatus::Suspended => "Suspended",
        }
    }
}

impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- MovementType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementType {
    Deposit,
    Withdrawal,
}

impl MovementType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "deposit" => Ok(MovementType::Deposit),
            "withdrawal" => Ok(MovementType::Withdrawal),
            _ => Err(DomainError::InvalidMovement(format!(
                "Unknown movement type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MovementType::Deposit => "Deposit",
            MovementType::Withdrawal => "Withdrawal",
        }
    }
}

impl fmt::Display for MovementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_id_new() {
        let id = AccountId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_account_id_parse_valid() {
        let id = AccountId::parse("550e8400-e29b-41d4-a716-446655440000");
        assert!(id.is_ok());
    }

    #[test]
    fn test_account_id_parse_invalid() {
        assert!(AccountId::parse("not-a-uuid").is_err());
    }

    #[test]
    fn test_movement_id_new() {
        let id = MovementId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_account_type_from_str() {
        assert_eq!(AccountType::from_str_type("Current").unwrap(), AccountType::Current);
        assert_eq!(AccountType::from_str_type("savings").unwrap(), AccountType::Savings);
        assert_eq!(AccountType::from_str_type("TimeDeposit").unwrap(), AccountType::TimeDeposit);
        assert_eq!(AccountType::from_str_type("time_deposit").unwrap(), AccountType::TimeDeposit);
    }

    #[test]
    fn test_account_type_from_str_invalid() {
        assert!(AccountType::from_str_type("unknown").is_err());
    }

    #[test]
    fn test_account_status_from_str() {
        assert_eq!(AccountStatus::from_str_status("Active").unwrap(), AccountStatus::Active);
        assert_eq!(AccountStatus::from_str_status("closed").unwrap(), AccountStatus::Closed);
        assert_eq!(AccountStatus::from_str_status("suspended").unwrap(), AccountStatus::Suspended);
    }

    #[test]
    fn test_account_status_from_str_invalid() {
        assert!(AccountStatus::from_str_status("unknown").is_err());
    }

    #[test]
    fn test_movement_type_from_str() {
        assert_eq!(MovementType::from_str_type("Deposit").unwrap(), MovementType::Deposit);
        assert_eq!(MovementType::from_str_type("withdrawal").unwrap(), MovementType::Withdrawal);
    }

    #[test]
    fn test_movement_type_from_str_invalid() {
        assert!(MovementType::from_str_type("transfer").is_err());
    }
}
