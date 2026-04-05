use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{Currency, CustomerId, Money, Rib};

use super::value_objects::{AccountId, AccountStatus, AccountType, MovementId, MovementType};

// --- Movement entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    id: MovementId,
    account_id: AccountId,
    movement_type: MovementType,
    amount: Money,
    balance_after: Money,
    description: String,
    created_at: DateTime<Utc>,
}

impl Movement {
    pub fn new(
        account_id: AccountId,
        movement_type: MovementType,
        amount: Money,
        balance_after: Money,
        description: &str,
    ) -> Self {
        Movement {
            id: MovementId::new(),
            account_id,
            movement_type,
            amount,
            balance_after,
            description: description.to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn reconstitute(
        id: MovementId,
        account_id: AccountId,
        movement_type: MovementType,
        amount: Money,
        balance_after: Money,
        description: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Movement {
            id,
            account_id,
            movement_type,
            amount,
            balance_after,
            description,
            created_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &MovementId {
        &self.id
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn movement_type(&self) -> MovementType {
        self.movement_type
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn balance_after(&self) -> &Money {
        &self.balance_after
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- Account aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    id: AccountId,
    customer_id: CustomerId,
    rib: Rib,
    account_type: AccountType,
    balance: Money,
    available_balance: Money,
    status: AccountStatus,
    movements: Vec<Movement>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Account {
    /// Create a new account. Enforces INV-01: KYC must be validated.
    pub fn new(
        customer_id: CustomerId,
        rib: Rib,
        account_type: AccountType,
        kyc_validated: bool,
    ) -> Result<Self, DomainError> {
        if !kyc_validated {
            return Err(DomainError::KycNotValidated);
        }

        let now = Utc::now();
        let zero = Money::zero(Currency::TND);

        Ok(Account {
            id: AccountId::new(),
            customer_id,
            rib,
            account_type,
            balance: zero.clone(),
            available_balance: zero,
            status: AccountStatus::Active,
            movements: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: AccountId,
        customer_id: CustomerId,
        rib: Rib,
        account_type: AccountType,
        balance: Money,
        available_balance: Money,
        status: AccountStatus,
        movements: Vec<Movement>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Account {
            id,
            customer_id,
            rib,
            account_type,
            balance,
            available_balance,
            status,
            movements,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn rib(&self) -> &Rib {
        &self.rib
    }

    pub fn account_type(&self) -> AccountType {
        self.account_type
    }

    pub fn balance(&self) -> &Money {
        &self.balance
    }

    pub fn available_balance(&self) -> &Money {
        &self.available_balance
    }

    pub fn status(&self) -> AccountStatus {
        self.status
    }

    pub fn movements(&self) -> &[Movement] {
        &self.movements
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Deposit money into the account.
    pub fn deposit(&mut self, amount: Money, description: &str) -> Result<Movement, DomainError> {
        self.check_active()?;

        if amount.amount_cents() <= 0 {
            return Err(DomainError::InvalidMovement(
                "Deposit amount must be positive".to_string(),
            ));
        }

        self.balance = self.balance.add(&amount)?;
        self.available_balance = self.available_balance.add(&amount)?;
        self.updated_at = Utc::now();

        let movement = Movement::new(
            self.id.clone(),
            MovementType::Deposit,
            amount,
            self.balance.clone(),
            description,
        );
        self.movements.push(movement.clone());
        Ok(movement)
    }

    /// Withdraw money from the account. Checks available_balance.
    pub fn withdraw(&mut self, amount: Money, description: &str) -> Result<Movement, DomainError> {
        self.check_active()?;

        if amount.amount_cents() <= 0 {
            return Err(DomainError::InvalidMovement(
                "Withdrawal amount must be positive".to_string(),
            ));
        }

        if amount.amount_cents() > self.available_balance.amount_cents() {
            return Err(DomainError::InsufficientFunds);
        }

        self.balance = self.balance.subtract(&amount)?;
        self.available_balance = self.available_balance.subtract(&amount)?;
        self.updated_at = Utc::now();

        let movement = Movement::new(
            self.id.clone(),
            MovementType::Withdrawal,
            amount,
            self.balance.clone(),
            description,
        );
        self.movements.push(movement.clone());
        Ok(movement)
    }

    /// Freeze the account (set status to Suspended, available_balance to 0).
    pub fn freeze(&mut self) {
        self.status = AccountStatus::Suspended;
        self.available_balance = Money::zero(self.balance.currency());
        self.updated_at = Utc::now();
    }

    /// Unfreeze the account (set status back to Active, restore available_balance to balance).
    pub fn unfreeze(&mut self) {
        self.status = AccountStatus::Active;
        self.available_balance = self.balance.clone();
        self.updated_at = Utc::now();
    }

    /// Close the account. Only allowed if balance is zero.
    pub fn close(&mut self) -> Result<(), DomainError> {
        if !self.balance.is_zero() {
            return Err(DomainError::InvalidMovement(
                "Cannot close account with non-zero balance".to_string(),
            ));
        }
        self.status = AccountStatus::Closed;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Reduce available_balance by hold amount (e.g., pending transaction).
    pub fn apply_hold(&mut self, amount: Money) -> Result<(), DomainError> {
        self.check_active()?;

        if amount.amount_cents() > self.available_balance.amount_cents() {
            return Err(DomainError::InsufficientFunds);
        }

        self.available_balance = self.available_balance.subtract(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Release a hold, increasing available_balance (up to balance).
    pub fn release_hold(&mut self, amount: Money) -> Result<(), DomainError> {
        self.check_active()?;

        let new_available = self.available_balance.add(&amount)?;
        // available_balance must not exceed balance
        if new_available.amount_cents() > self.balance.amount_cents() {
            self.available_balance = self.balance.clone();
        } else {
            self.available_balance = new_available;
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    // --- Private helpers ---

    fn check_active(&self) -> Result<(), DomainError> {
        match self.status {
            AccountStatus::Active => Ok(()),
            AccountStatus::Closed => Err(DomainError::AccountClosed),
            AccountStatus::Suspended => Err(DomainError::AccountSuspended),
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::{Currency, CustomerId, Money, Rib};

    fn valid_rib() -> Rib {
        Rib::new("01001234567890123400").unwrap()
    }

    fn valid_customer_id() -> CustomerId {
        CustomerId::new()
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    // --- INV-01: KYC not validated ---

    #[test]
    fn test_new_account_kyc_not_validated_fails() {
        let result = Account::new(
            valid_customer_id(),
            valid_rib(),
            AccountType::Current,
            false,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::KycNotValidated);
    }

    #[test]
    fn test_new_account_kyc_validated_succeeds() {
        let account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        assert_eq!(account.status(), AccountStatus::Active);
        assert!(account.balance().is_zero());
        assert!(account.available_balance().is_zero());
        assert_eq!(account.account_type(), AccountType::Current);
    }

    #[test]
    fn test_new_account_default_currency_tnd() {
        let account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Savings, true).unwrap();
        assert_eq!(account.balance().currency(), Currency::TND);
    }

    // --- Deposit ---

    #[test]
    fn test_deposit_success() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        let movement = account.deposit(tnd(1000.0), "Initial deposit").unwrap();
        assert_eq!(account.balance().amount(), 1000.0);
        assert_eq!(account.available_balance().amount(), 1000.0);
        assert_eq!(movement.movement_type(), MovementType::Deposit);
        assert_eq!(movement.balance_after().amount(), 1000.0);
        assert_eq!(movement.description(), "Initial deposit");
    }

    #[test]
    fn test_deposit_zero_amount_fails() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        let result = account.deposit(tnd(0.0), "Zero deposit");
        assert!(result.is_err());
    }

    #[test]
    fn test_deposit_negative_amount_fails() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        let result = account.deposit(
            Money::new(-100.0, Currency::TND).unwrap(),
            "Negative deposit",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_deposits() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(500.0), "First").unwrap();
        account.deposit(tnd(300.0), "Second").unwrap();
        assert_eq!(account.balance().amount(), 800.0);
        assert_eq!(account.movements().len(), 2);
    }

    // --- Withdrawal ---

    #[test]
    fn test_withdraw_success() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        let movement = account.withdraw(tnd(400.0), "ATM withdrawal").unwrap();
        assert_eq!(account.balance().amount(), 600.0);
        assert_eq!(account.available_balance().amount(), 600.0);
        assert_eq!(movement.movement_type(), MovementType::Withdrawal);
        assert_eq!(movement.balance_after().amount(), 600.0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(100.0), "Deposit").unwrap();
        let result = account.withdraw(tnd(200.0), "Overdraft attempt");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::InsufficientFunds);
        // Balance unchanged
        assert_eq!(account.balance().amount(), 100.0);
    }

    #[test]
    fn test_withdraw_zero_amount_fails() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(100.0), "Deposit").unwrap();
        let result = account.withdraw(tnd(0.0), "Zero withdrawal");
        assert!(result.is_err());
    }

    #[test]
    fn test_withdraw_exact_balance() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(500.0), "Deposit").unwrap();
        account.withdraw(tnd(500.0), "Full withdrawal").unwrap();
        assert!(account.balance().is_zero());
    }

    // --- Freeze / Unfreeze ---

    #[test]
    fn test_freeze_account() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.freeze();
        assert_eq!(account.status(), AccountStatus::Suspended);
        assert!(account.available_balance().is_zero());
        // balance unchanged
        assert_eq!(account.balance().amount(), 1000.0);
    }

    #[test]
    fn test_frozen_account_cannot_deposit() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.freeze();
        let result = account.deposit(tnd(100.0), "Deposit");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::AccountSuspended);
    }

    #[test]
    fn test_frozen_account_cannot_withdraw() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.freeze();
        let result = account.withdraw(tnd(100.0), "Withdrawal");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::AccountSuspended);
    }

    #[test]
    fn test_unfreeze_account() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.freeze();
        account.unfreeze();
        assert_eq!(account.status(), AccountStatus::Active);
        assert_eq!(account.available_balance().amount(), 1000.0);
    }

    // --- Close ---

    #[test]
    fn test_close_account_zero_balance() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.close().unwrap();
        assert_eq!(account.status(), AccountStatus::Closed);
    }

    #[test]
    fn test_close_account_non_zero_balance_fails() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(100.0), "Deposit").unwrap();
        let result = account.close();
        assert!(result.is_err());
    }

    #[test]
    fn test_closed_account_cannot_deposit() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.close().unwrap();
        let result = account.deposit(tnd(100.0), "Deposit");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::AccountClosed);
    }

    // --- Hold ---

    #[test]
    fn test_apply_hold() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.apply_hold(tnd(300.0)).unwrap();
        assert_eq!(account.balance().amount(), 1000.0);
        assert_eq!(account.available_balance().amount(), 700.0);
    }

    #[test]
    fn test_apply_hold_insufficient_available() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(100.0), "Deposit").unwrap();
        let result = account.apply_hold(tnd(200.0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::InsufficientFunds);
    }

    #[test]
    fn test_release_hold() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.apply_hold(tnd(300.0)).unwrap();
        account.release_hold(tnd(200.0)).unwrap();
        assert_eq!(account.available_balance().amount(), 900.0);
    }

    #[test]
    fn test_release_hold_caps_at_balance() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.apply_hold(tnd(300.0)).unwrap();
        // Release more than held
        account.release_hold(tnd(500.0)).unwrap();
        // available_balance should be capped at balance
        assert_eq!(account.available_balance().amount(), 1000.0);
    }

    #[test]
    fn test_hold_then_withdraw_checks_available() {
        let mut account =
            Account::new(valid_customer_id(), valid_rib(), AccountType::Current, true).unwrap();
        account.deposit(tnd(1000.0), "Deposit").unwrap();
        account.apply_hold(tnd(800.0)).unwrap();
        // Only 200 available
        let result = account.withdraw(tnd(300.0), "Withdrawal");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::InsufficientFunds);
        // 200 should work
        account.withdraw(tnd(200.0), "Withdrawal").unwrap();
        assert_eq!(account.balance().amount(), 800.0);
    }

    // --- Reconstitute ---

    #[test]
    fn test_reconstitute() {
        let id = AccountId::new();
        let cid = valid_customer_id();
        let rib = valid_rib();
        let now = Utc::now();
        let balance = tnd(5000.0);
        let available = tnd(4000.0);

        let account = Account::reconstitute(
            id.clone(),
            cid.clone(),
            rib.clone(),
            AccountType::Savings,
            balance.clone(),
            available.clone(),
            AccountStatus::Active,
            vec![],
            now,
            now,
        );

        assert_eq!(account.id(), &id);
        assert_eq!(account.customer_id(), &cid);
        assert_eq!(account.rib(), &rib);
        assert_eq!(account.account_type(), AccountType::Savings);
        assert_eq!(account.balance(), &balance);
        assert_eq!(account.available_balance(), &available);
    }

    // --- Getters ---

    #[test]
    fn test_getters() {
        let account = Account::new(
            valid_customer_id(),
            valid_rib(),
            AccountType::TimeDeposit,
            true,
        )
        .unwrap();
        assert!(!account.id().as_uuid().is_nil());
        assert_eq!(account.account_type(), AccountType::TimeDeposit);
        assert!(account.movements().is_empty());
    }
}
