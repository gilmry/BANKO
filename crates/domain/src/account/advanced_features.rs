use chrono::{DateTime, Utc};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{Currency, Money};

use super::value_objects::{AccountId, AccountStatus};

// ==================== InternalAccountType (FR-023, FR-024) ====================

/// Types of internal accounts used by the bank for operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InternalAccountType {
    /// Suspense account (temporary holding of funds)
    Suspense,
    /// Clearing account (interbank settlements)
    Clearing,
    /// Profit & Loss account
    ProfitAndLoss,
    /// Nostro account (bank's account with foreign bank)
    Nostro,
    /// Vostro account (foreign bank's account with this bank)
    Vostro,
}

impl InternalAccountType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "suspense" => Ok(InternalAccountType::Suspense),
            "clearing" => Ok(InternalAccountType::Clearing),
            "profitandloss" | "profit_and_loss" => Ok(InternalAccountType::ProfitAndLoss),
            "nostro" => Ok(InternalAccountType::Nostro),
            "vostro" => Ok(InternalAccountType::Vostro),
            _ => Err(DomainError::InvalidAccountType(format!(
                "Unknown internal account type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            InternalAccountType::Suspense => "Suspense",
            InternalAccountType::Clearing => "Clearing",
            InternalAccountType::ProfitAndLoss => "ProfitAndLoss",
            InternalAccountType::Nostro => "Nostro",
            InternalAccountType::Vostro => "Vostro",
        }
    }
}

impl fmt::Display for InternalAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ==================== InternalAccount Entity (FR-023, FR-024) ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalAccount {
    id: AccountId,
    internal_type: InternalAccountType,
    balance: Money,
    status: AccountStatus,
    correspondent_bank: Option<String>, // For Nostro/Vostro accounts
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InternalAccount {
    /// Create a new internal account.
    pub fn new(
        internal_type: InternalAccountType,
        currency: Currency,
        correspondent_bank: Option<String>,
    ) -> Result<Self, DomainError> {
        // Validate correspondent bank for Nostro/Vostro
        if matches!(
            internal_type,
            InternalAccountType::Nostro | InternalAccountType::Vostro
        )
            && (correspondent_bank.is_none() || correspondent_bank.as_ref().is_none_or(|b| b.is_empty()))
            {
                return Err(DomainError::ValidationError(
                    "Nostro/Vostro accounts require correspondent bank name".to_string(),
                ));
            }

        let now = Utc::now();
        let zero = Money::zero(currency);

        Ok(InternalAccount {
            id: AccountId::new(),
            internal_type,
            balance: zero,
            status: AccountStatus::Active,
            correspondent_bank,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence.
    pub fn reconstitute(
        id: AccountId,
        internal_type: InternalAccountType,
        balance: Money,
        status: AccountStatus,
        correspondent_bank: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        InternalAccount {
            id,
            internal_type,
            balance,
            status,
            correspondent_bank,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn internal_type(&self) -> InternalAccountType {
        self.internal_type
    }

    pub fn balance(&self) -> &Money {
        &self.balance
    }

    pub fn status(&self) -> AccountStatus {
        self.status
    }

    pub fn correspondent_bank(&self) -> Option<&str> {
        self.correspondent_bank.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Deposit into internal account.
    pub fn deposit(&mut self, amount: Money) -> Result<(), DomainError> {
        self.check_active()?;

        if amount.amount_cents() <= 0 {
            return Err(DomainError::InvalidMovement(
                "Deposit amount must be positive".to_string(),
            ));
        }

        self.balance = self.balance.add(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Withdraw from internal account.
    pub fn withdraw(&mut self, amount: Money) -> Result<(), DomainError> {
        self.check_active()?;

        if amount.amount_cents() <= 0 {
            return Err(DomainError::InvalidMovement(
                "Withdrawal amount must be positive".to_string(),
            ));
        }

        if amount.amount_cents() > self.balance.amount_cents() {
            return Err(DomainError::InsufficientFunds);
        }

        self.balance = self.balance.subtract(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Freeze the internal account.
    pub fn freeze(&mut self) {
        self.status = AccountStatus::Suspended;
        self.updated_at = Utc::now();
    }

    /// Unfreeze the internal account.
    pub fn unfreeze(&mut self) {
        self.status = AccountStatus::Active;
        self.updated_at = Utc::now();
    }

    fn check_active(&self) -> Result<(), DomainError> {
        match self.status {
            AccountStatus::Active => Ok(()),
            AccountStatus::Closed => Err(DomainError::AccountClosed),
            AccountStatus::Suspended => Err(DomainError::AccountSuspended),
        }
    }
}

// ==================== AccountLimit Entity (FR-025) ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLimit {
    account_id: AccountId,
    /// Maximum single transaction amount (in the account's currency)
    single_transaction_max: Money,
    /// Maximum daily debit amount
    daily_debit_max: Money,
    /// Maximum number of transactions per day
    transaction_count_max: i32,
    /// For savings accounts: automatic interest capitalization at this rate per annum
    interest_capitalization_rate: Option<Decimal>,
    /// Created/updated timestamps
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl AccountLimit {
    /// Create a new account limit configuration.
    pub fn new(
        account_id: AccountId,
        single_transaction_max: Money,
        daily_debit_max: Money,
        transaction_count_max: i32,
        interest_capitalization_rate: Option<Decimal>,
    ) -> Result<Self, DomainError> {
        // Validate that limits are positive
        if single_transaction_max.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Single transaction max must be positive".to_string(),
            ));
        }

        if daily_debit_max.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Daily debit max must be positive".to_string(),
            ));
        }

        if transaction_count_max <= 0 {
            return Err(DomainError::ValidationError(
                "Transaction count max must be positive".to_string(),
            ));
        }

        // Validate interest rate (if provided)
        if let Some(rate) = interest_capitalization_rate {
            if rate < Decimal::ZERO || rate > Decimal::from(100) {
                return Err(DomainError::ValidationError(
                    "Interest rate must be between 0 and 100 percent".to_string(),
                ));
            }
        }

        let now = Utc::now();

        Ok(AccountLimit {
            account_id,
            single_transaction_max,
            daily_debit_max,
            transaction_count_max,
            interest_capitalization_rate,
            created_at: now,
            updated_at: now,
        })
    }

    // --- Getters ---

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn single_transaction_max(&self) -> &Money {
        &self.single_transaction_max
    }

    pub fn daily_debit_max(&self) -> &Money {
        &self.daily_debit_max
    }

    pub fn transaction_count_max(&self) -> i32 {
        self.transaction_count_max
    }

    pub fn interest_capitalization_rate(&self) -> Option<Decimal> {
        self.interest_capitalization_rate
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Check if a transaction would exceed the single transaction limit.
    pub fn check_single_transaction_limit(&self, amount: &Money) -> Result<(), DomainError> {
        if amount.amount_cents() > self.single_transaction_max.amount_cents() {
            return Err(DomainError::InvalidMovement(format!(
                "Transaction amount exceeds limit of {:.3}",
                self.single_transaction_max.amount()
            )));
        }
        Ok(())
    }

    /// Check if a daily debit amount would exceed the daily limit.
    pub fn check_daily_debit_limit(&self, daily_debit_total: &Money) -> Result<(), DomainError> {
        if daily_debit_total.amount_cents() > self.daily_debit_max.amount_cents() {
            return Err(DomainError::InvalidMovement(format!(
                "Daily debit exceeds limit of {:.3}",
                self.daily_debit_max.amount()
            )));
        }
        Ok(())
    }

    /// Check if transaction count would exceed the daily limit.
    pub fn check_transaction_count_limit(&self, count: i32) -> Result<(), DomainError> {
        if count > self.transaction_count_max {
            return Err(DomainError::InvalidMovement(format!(
                "Transaction count exceeds daily limit of {}",
                self.transaction_count_max
            )));
        }
        Ok(())
    }
}

// ==================== InterestCapitalization (FR-026) ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestCapitalization {
    account_id: AccountId,
    /// Annual interest rate in percent (e.g., 5 for 5%)
    annual_rate: Decimal,
    /// Last capitalization date
    last_capitalization: DateTime<Utc>,
    /// Total interest capitalized to date
    total_interest_capitalized: Money,
}

impl InterestCapitalization {
    /// Create a new interest capitalization configuration.
    pub fn new(
        account_id: AccountId,
        annual_rate: Decimal,
        initial_balance: Money,
    ) -> Result<Self, DomainError> {
        if annual_rate < Decimal::ZERO || annual_rate > Decimal::from(100) {
            return Err(DomainError::ValidationError(
                "Annual interest rate must be between 0 and 100 percent".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(InterestCapitalization {
            account_id,
            annual_rate,
            last_capitalization: now,
            total_interest_capitalized: Money::zero(initial_balance.currency()),
        })
    }

    // --- Getters ---

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn annual_rate(&self) -> Decimal {
        self.annual_rate
    }

    pub fn last_capitalization(&self) -> DateTime<Utc> {
        self.last_capitalization
    }

    pub fn total_interest_capitalized(&self) -> &Money {
        &self.total_interest_capitalized
    }

    // --- Domain behavior ---

    /// Calculate and capitalize interest based on time passed and balance.
    /// Returns the interest amount calculated (in Money).
    pub fn capitalize_interest(
        &mut self,
        current_balance: &Money,
    ) -> Result<Money, DomainError> {
        let now = Utc::now();
        let days_elapsed = (now - self.last_capitalization).num_days() as f64;

        if days_elapsed < 1.0 {
            // No capitalization if less than 1 day has passed
            return Ok(Money::zero(current_balance.currency()));
        }

        // Interest = balance * (annual_rate / 100) * (days_elapsed / 365)
        let _daily_rate = self.annual_rate / Decimal::from(365);
        let interest_decimal = Decimal::from_str_exact(&current_balance.amount().to_string())
            .unwrap_or(Decimal::ZERO)
            * (self.annual_rate / Decimal::from(100))
            * (Decimal::from_f64_retain(days_elapsed / 365.0).unwrap_or(Decimal::ZERO));

        let interest_amount = Money::new(
            interest_decimal.to_f64().unwrap_or(0.0),
            current_balance.currency(),
        )?;

        self.total_interest_capitalized =
            self.total_interest_capitalized.add(&interest_amount)?;
        self.last_capitalization = now;

        Ok(interest_amount)
    }
}

// ==================== BalanceNotification (FR-028) ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    /// Notification when balance falls below threshold
    LowBalance,
    /// Notification on credit (deposit)
    CreditTransaction,
    /// Notification on debit (withdrawal)
    DebitTransaction,
}

impl NotificationType {
    pub fn as_str(&self) -> &str {
        match self {
            NotificationType::LowBalance => "LowBalance",
            NotificationType::CreditTransaction => "CreditTransaction",
            NotificationType::DebitTransaction => "DebitTransaction",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceNotification {
    account_id: AccountId,
    notification_type: NotificationType,
    /// For LowBalance: the threshold below which notification is triggered
    threshold: Option<Money>,
    /// Whether this notification is active
    is_active: bool,
    created_at: DateTime<Utc>,
}

impl BalanceNotification {
    /// Create a new balance notification.
    pub fn new(
        account_id: AccountId,
        notification_type: NotificationType,
        threshold: Option<Money>,
    ) -> Result<Self, DomainError> {
        // LowBalance requires a threshold
        if notification_type == NotificationType::LowBalance && threshold.is_none() {
            return Err(DomainError::ValidationError(
                "LowBalance notification requires a threshold".to_string(),
            ));
        }

        // Validate threshold is positive
        if let Some(t) = &threshold {
            if t.amount_cents() < 0 {
                return Err(DomainError::ValidationError(
                    "Notification threshold must be non-negative".to_string(),
                ));
            }
        }

        Ok(BalanceNotification {
            account_id,
            notification_type,
            threshold,
            is_active: true,
            created_at: Utc::now(),
        })
    }

    // --- Getters ---

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn notification_type(&self) -> NotificationType {
        self.notification_type
    }

    pub fn threshold(&self) -> Option<&Money> {
        self.threshold.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // --- Domain behavior ---

    /// Check if a notification should be triggered for the current balance.
    pub fn should_trigger(&self, current_balance: &Money) -> bool {
        if !self.is_active {
            return false;
        }

        match self.notification_type {
            NotificationType::LowBalance => {
                if let Some(threshold) = &self.threshold {
                    current_balance.amount_cents() < threshold.amount_cents()
                } else {
                    false
                }
            }
            NotificationType::CreditTransaction | NotificationType::DebitTransaction => {
                // These are always triggered (event-based)
                true
            }
        }
    }

    /// Enable or disable this notification.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::{Currency, CustomerId, Money, Rib};

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    fn valid_account_id() -> AccountId {
        AccountId::new()
    }

    // --- InternalAccountType ---

    #[test]
    fn test_internal_account_type_from_str() {
        assert_eq!(
            InternalAccountType::from_str_type("suspense").unwrap(),
            InternalAccountType::Suspense
        );
        assert_eq!(
            InternalAccountType::from_str_type("Clearing").unwrap(),
            InternalAccountType::Clearing
        );
        assert_eq!(
            InternalAccountType::from_str_type("Nostro").unwrap(),
            InternalAccountType::Nostro
        );
    }

    #[test]
    fn test_internal_account_type_invalid() {
        assert!(InternalAccountType::from_str_type("unknown").is_err());
    }

    // --- InternalAccount ---

    #[test]
    fn test_internal_account_new_suspense() {
        let account = InternalAccount::new(
            InternalAccountType::Suspense,
            Currency::TND,
            None,
        )
        .unwrap();
        assert_eq!(account.internal_type(), InternalAccountType::Suspense);
        assert_eq!(account.status(), AccountStatus::Active);
        assert!(account.balance().is_zero());
    }

    #[test]
    fn test_internal_account_nostro_requires_correspondent() {
        let result = InternalAccount::new(
            InternalAccountType::Nostro,
            Currency::TND,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_internal_account_nostro_with_correspondent() {
        let account = InternalAccount::new(
            InternalAccountType::Nostro,
            Currency::EUR,
            Some("BNP Paribas Paris".to_string()),
        )
        .unwrap();
        assert_eq!(account.internal_type(), InternalAccountType::Nostro);
        assert_eq!(account.correspondent_bank(), Some("BNP Paribas Paris"));
    }

    #[test]
    fn test_internal_account_deposit() {
        let mut account = InternalAccount::new(
            InternalAccountType::Clearing,
            Currency::TND,
            None,
        )
        .unwrap();
        account.deposit(tnd(500.0)).unwrap();
        assert_eq!(account.balance().amount(), 500.0);
    }

    #[test]
    fn test_internal_account_withdraw() {
        let mut account = InternalAccount::new(
            InternalAccountType::Suspense,
            Currency::TND,
            None,
        )
        .unwrap();
        account.deposit(tnd(1000.0)).unwrap();
        account.withdraw(tnd(300.0)).unwrap();
        assert_eq!(account.balance().amount(), 700.0);
    }

    #[test]
    fn test_internal_account_freeze() {
        let mut account = InternalAccount::new(
            InternalAccountType::Clearing,
            Currency::TND,
            None,
        )
        .unwrap();
        account.freeze();
        assert_eq!(account.status(), AccountStatus::Suspended);
    }

    // --- AccountLimit ---

    #[test]
    fn test_account_limit_new() {
        let limit = AccountLimit::new(
            valid_account_id(),
            tnd(10000.0),
            tnd(50000.0),
            20,
            Some(Decimal::from(5)),
        )
        .unwrap();
        assert_eq!(limit.transaction_count_max(), 20);
        assert_eq!(limit.interest_capitalization_rate(), Some(Decimal::from(5)));
    }

    #[test]
    fn test_account_limit_invalid_interest_rate() {
        let result = AccountLimit::new(
            valid_account_id(),
            tnd(10000.0),
            tnd(50000.0),
            20,
            Some(Decimal::from(150)), // Invalid: > 100
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_account_limit_check_single_transaction() {
        let limit = AccountLimit::new(
            valid_account_id(),
            tnd(10000.0),
            tnd(50000.0),
            20,
            None,
        )
        .unwrap();
        assert!(limit.check_single_transaction_limit(&tnd(5000.0)).is_ok());
        assert!(limit.check_single_transaction_limit(&tnd(15000.0)).is_err());
    }

    #[test]
    fn test_account_limit_check_daily_debit() {
        let limit = AccountLimit::new(
            valid_account_id(),
            tnd(10000.0),
            tnd(50000.0),
            20,
            None,
        )
        .unwrap();
        assert!(limit.check_daily_debit_limit(&tnd(30000.0)).is_ok());
        assert!(limit.check_daily_debit_limit(&tnd(70000.0)).is_err());
    }

    #[test]
    fn test_account_limit_check_transaction_count() {
        let limit = AccountLimit::new(
            valid_account_id(),
            tnd(10000.0),
            tnd(50000.0),
            20,
            None,
        )
        .unwrap();
        assert!(limit.check_transaction_count_limit(15).is_ok());
        assert!(limit.check_transaction_count_limit(25).is_err());
    }

    // --- InterestCapitalization ---

    #[test]
    fn test_interest_capitalization_new() {
        let ic = InterestCapitalization::new(
            valid_account_id(),
            Decimal::from(5),
            tnd(10000.0),
        )
        .unwrap();
        assert_eq!(ic.annual_rate(), Decimal::from(5));
        assert!(ic.total_interest_capitalized().is_zero());
    }

    #[test]
    fn test_interest_capitalization_invalid_rate() {
        let result = InterestCapitalization::new(
            valid_account_id(),
            Decimal::from(150), // Invalid: > 100
            tnd(10000.0),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_interest_capitalization_calculate() {
        let mut ic = InterestCapitalization::new(
            valid_account_id(),
            Decimal::from(5),
            tnd(10000.0),
        )
        .unwrap();

        // Set last_capitalization to 365 days ago
        ic.last_capitalization = Utc::now() - chrono::Duration::days(365);

        let interest = ic.capitalize_interest(&tnd(10000.0)).unwrap();
        // 5% of 10000 = 500
        assert!(interest.amount() > 490.0 && interest.amount() < 510.0);
    }

    // --- BalanceNotification ---

    #[test]
    fn test_balance_notification_low_balance() {
        let notif = BalanceNotification::new(
            valid_account_id(),
            NotificationType::LowBalance,
            Some(tnd(1000.0)),
        )
        .unwrap();
        assert!(notif.is_active());
        assert_eq!(notif.threshold().map(|m| m.amount()), Some(1000.0));
    }

    #[test]
    fn test_balance_notification_low_balance_without_threshold() {
        let result = BalanceNotification::new(
            valid_account_id(),
            NotificationType::LowBalance,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_balance_notification_should_trigger() {
        let notif = BalanceNotification::new(
            valid_account_id(),
            NotificationType::LowBalance,
            Some(tnd(1000.0)),
        )
        .unwrap();
        assert!(notif.should_trigger(&tnd(500.0))); // Below threshold
        assert!(!notif.should_trigger(&tnd(1500.0))); // Above threshold
    }

    #[test]
    fn test_balance_notification_credit_transaction() {
        let notif = BalanceNotification::new(
            valid_account_id(),
            NotificationType::CreditTransaction,
            None,
        )
        .unwrap();
        assert!(notif.is_active());
        assert!(notif.should_trigger(&tnd(100.0))); // Always triggers
    }

    #[test]
    fn test_balance_notification_disable() {
        let mut notif = BalanceNotification::new(
            valid_account_id(),
            NotificationType::LowBalance,
            Some(tnd(1000.0)),
        )
        .unwrap();
        notif.set_active(false);
        assert!(!notif.should_trigger(&tnd(500.0))); // Doesn't trigger when disabled
    }
}
