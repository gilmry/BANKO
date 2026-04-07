use std::sync::Arc;

use rust_decimal::Decimal;

use banko_domain::account::{
    AccountId, AccountLimit, BalanceNotification, InternalAccount, InternalAccountType,
    InterestCapitalization, NotificationType,
};
use banko_domain::shared::Money;

use super::errors::AccountServiceError;
use super::ports::IAccountRepository;

/// Extended service for advanced account features.
/// Handles FR-023 through FR-028 of BMAD v4.0.1
pub struct AdvancedAccountService {
    account_repo: Arc<dyn IAccountRepository>,
}

impl AdvancedAccountService {
    pub fn new(account_repo: Arc<dyn IAccountRepository>) -> Self {
        AdvancedAccountService { account_repo }
    }

    // ============ FR-017: Account Closure (Enhanced) ============

    /// Check if an account can be closed (zero balance, no pending operations).
    pub async fn can_close_account(&self, account_id: &AccountId) -> Result<bool, AccountServiceError> {
        let account = self
            .account_repo
            .find_by_id(account_id)
            .await
            .map_err(AccountServiceError::Internal)?
            .ok_or(AccountServiceError::AccountNotFound)?;

        Ok(account.can_close())
    }

    // ============ FR-023: Internal Accounts (Suspense, Clearing, P&L, Nostro, Vostro) ============

    /// Create a new internal account (suspense, clearing, nostro, vostro, P&L).
    pub async fn create_internal_account(
        &self,
        internal_type: InternalAccountType,
        currency: banko_domain::shared::Currency,
        correspondent_bank: Option<String>,
    ) -> Result<InternalAccount, AccountServiceError> {
        InternalAccount::new(internal_type, currency, correspondent_bank)
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    // ============ FR-025: Account Limits (Single transaction, Daily debit, Transaction count) ============

    /// Create account limits for transaction controls.
    pub async fn create_account_limits(
        &self,
        account_id: AccountId,
        single_transaction_max: Money,
        daily_debit_max: Money,
        transaction_count_max: i32,
        interest_rate: Option<Decimal>,
    ) -> Result<AccountLimit, AccountServiceError> {
        AccountLimit::new(
            account_id,
            single_transaction_max,
            daily_debit_max,
            transaction_count_max,
            interest_rate,
        )
        .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    /// Validate a transaction against account limits.
    pub fn validate_transaction_against_limits(
        &self,
        limit: &AccountLimit,
        amount: &Money,
    ) -> Result<(), AccountServiceError> {
        limit
            .check_single_transaction_limit(amount)
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    /// Validate daily debit against account limits.
    pub fn validate_daily_debit_against_limits(
        &self,
        limit: &AccountLimit,
        daily_total: &Money,
    ) -> Result<(), AccountServiceError> {
        limit
            .check_daily_debit_limit(daily_total)
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    /// Validate transaction count against daily limit.
    pub fn validate_transaction_count_against_limits(
        &self,
        limit: &AccountLimit,
        count: i32,
    ) -> Result<(), AccountServiceError> {
        limit
            .check_transaction_count_limit(count)
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    // ============ FR-026: Interest Capitalization (Automatic for savings/DAT) ============

    /// Create interest capitalization configuration for a savings account.
    pub async fn create_interest_capitalization(
        &self,
        account_id: AccountId,
        annual_rate: Decimal,
        initial_balance: Money,
    ) -> Result<InterestCapitalization, AccountServiceError> {
        InterestCapitalization::new(account_id, annual_rate, initial_balance)
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    /// Calculate and capitalize interest for an account.
    /// Returns the interest amount capitalized.
    pub async fn capitalize_interest(
        &self,
        account_id: &AccountId,
        mut capitalization: InterestCapitalization,
    ) -> Result<Money, AccountServiceError> {
        let account = self
            .account_repo
            .find_by_id(account_id)
            .await
            .map_err(AccountServiceError::Internal)?
            .ok_or(AccountServiceError::AccountNotFound)?;

        let interest = capitalization
            .capitalize_interest(account.balance())
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))?;

        Ok(interest)
    }

    // ============ FR-028: Balance Notifications (Low balance, Credit/Debit alerts) ============

    /// Create a balance notification (low balance, credit, debit).
    pub async fn create_balance_notification(
        &self,
        account_id: AccountId,
        notification_type: NotificationType,
        threshold: Option<Money>,
    ) -> Result<BalanceNotification, AccountServiceError> {
        BalanceNotification::new(account_id, notification_type, threshold)
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))
    }

    /// Check if a notification should be triggered for the current balance.
    pub fn should_trigger_notification(
        &self,
        notification: &BalanceNotification,
        current_balance: &Money,
    ) -> bool {
        notification.should_trigger(current_balance)
    }

    /// Enable or disable a balance notification.
    pub fn update_notification_status(
        &self,
        notification: &mut BalanceNotification,
        active: bool,
    ) {
        notification.set_active(active);
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use banko_domain::shared::Currency;

    struct MockAccountRepository;

    impl MockAccountRepository {
        fn new() -> Self {
            MockAccountRepository
        }
    }

    #[async_trait::async_trait]
    impl IAccountRepository for MockAccountRepository {
        async fn save(&self, _account: &Account) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &AccountId) -> Result<Option<Account>, String> {
            Ok(None)
        }

        async fn find_by_customer_id(&self, _customer_id: &CustomerId) -> Result<Vec<Account>, String> {
            Ok(vec![])
        }

        async fn find_by_rib(&self, _rib: &banko_domain::shared::Rib) -> Result<Option<Account>, String> {
            Ok(None)
        }

        async fn save_movement(&self, _movement: &banko_domain::account::Movement) -> Result<(), String> {
            Ok(())
        }

        async fn find_movements_by_account(
            &self,
            _account_id: &AccountId,
            _limit: i64,
        ) -> Result<Vec<banko_domain::account::Movement>, String> {
            Ok(vec![])
        }

        async fn find_movements_by_account_and_period(
            &self,
            _account_id: &AccountId,
            _from: Option<chrono::DateTime<chrono::Utc>>,
            _to: Option<chrono::DateTime<chrono::Utc>>,
        ) -> Result<Vec<banko_domain::account::Movement>, String> {
            Ok(vec![])
        }

        async fn delete(&self, _id: &AccountId) -> Result<(), String> {
            Ok(())
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    fn make_service() -> AdvancedAccountService {
        AdvancedAccountService::new(Arc::new(MockAccountRepository::new()))
    }

    // --- Account Limits Tests ---

    #[tokio::test]
    async fn test_create_account_limits() {
        let service = make_service();
        let limit = service
            .create_account_limits(
                AccountId::new(),
                tnd(10000.0),
                tnd(50000.0),
                20,
                Some(Decimal::from(5)),
            )
            .await
            .unwrap();
        assert_eq!(limit.transaction_count_max(), 20);
    }

    #[tokio::test]
    async fn test_validate_transaction_within_limits() {
        let service = make_service();
        let limit = service
            .create_account_limits(
                AccountId::new(),
                tnd(10000.0),
                tnd(50000.0),
                20,
                None,
            )
            .await
            .unwrap();
        assert!(service.validate_transaction_against_limits(&limit, &tnd(5000.0)).is_ok());
    }

    #[tokio::test]
    async fn test_validate_transaction_exceeds_limits() {
        let service = make_service();
        let limit = service
            .create_account_limits(
                AccountId::new(),
                tnd(10000.0),
                tnd(50000.0),
                20,
                None,
            )
            .await
            .unwrap();
        assert!(service
            .validate_transaction_against_limits(&limit, &tnd(15000.0))
            .is_err());
    }

    #[tokio::test]
    async fn test_create_interest_capitalization() {
        let service = make_service();
        let ic = service
            .create_interest_capitalization(
                AccountId::new(),
                Decimal::from(5),
                tnd(10000.0),
            )
            .await
            .unwrap();
        assert_eq!(ic.annual_rate(), Decimal::from(5));
    }

    #[tokio::test]
    async fn test_create_balance_notification_low_balance() {
        let service = make_service();
        let notif = service
            .create_balance_notification(
                AccountId::new(),
                NotificationType::LowBalance,
                Some(tnd(1000.0)),
            )
            .await
            .unwrap();
        assert!(notif.is_active());
    }

    #[tokio::test]
    async fn test_should_trigger_notification() {
        let service = make_service();
        let notif = service
            .create_balance_notification(
                AccountId::new(),
                NotificationType::LowBalance,
                Some(tnd(1000.0)),
            )
            .await
            .unwrap();
        assert!(service.should_trigger_notification(&notif, &tnd(500.0)));
        assert!(!service.should_trigger_notification(&notif, &tnd(1500.0)));
    }

    #[tokio::test]
    async fn test_update_notification_status() {
        let service = make_service();
        let mut notif = service
            .create_balance_notification(
                AccountId::new(),
                NotificationType::CreditTransaction,
                None,
            )
            .await
            .unwrap();
        assert!(notif.is_active());
        service.update_notification_status(&mut notif, false);
        assert!(!notif.is_active());
    }

    #[tokio::test]
    async fn test_create_internal_account_suspense() {
        let service = make_service();
        let internal_acct = service
            .create_internal_account(InternalAccountType::Suspense, Currency::TND, None)
            .await
            .unwrap();
        assert_eq!(internal_acct.internal_type(), InternalAccountType::Suspense);
    }

    #[tokio::test]
    async fn test_create_internal_account_nostro() {
        let service = make_service();
        let internal_acct = service
            .create_internal_account(
                InternalAccountType::Nostro,
                Currency::EUR,
                Some("BNP Paribas".to_string()),
            )
            .await
            .unwrap();
        assert_eq!(internal_acct.internal_type(), InternalAccountType::Nostro);
    }
}
