use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::account::IAccountLimitRepository;
use banko_domain::account::{AccountId, AccountLimit, BalanceNotification, InternalAccount, InternalAccountType, InterestCapitalization};
use banko_domain::shared::{Currency, Money};

/// Repository for AccountLimit persistence (FR-025)
pub struct PgAccountLimitRepository {
    pool: PgPool,
}

impl PgAccountLimitRepository {
    pub fn new(pool: PgPool) -> Self {
        PgAccountLimitRepository { pool }
    }

    pub async fn save(&self, limit: &AccountLimit) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO account.account_limits (
                account_id,
                single_transaction_max,
                daily_debit_max,
                transaction_count_max,
                interest_capitalization_rate,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (account_id) DO UPDATE SET
                single_transaction_max = EXCLUDED.single_transaction_max,
                daily_debit_max = EXCLUDED.daily_debit_max,
                transaction_count_max = EXCLUDED.transaction_count_max,
                interest_capitalization_rate = EXCLUDED.interest_capitalization_rate,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(limit.account_id().as_uuid())
        .bind(limit.single_transaction_max().amount_cents())
        .bind(limit.daily_debit_max().amount_cents())
        .bind(limit.transaction_count_max())
        .bind(limit.interest_capitalization_rate())
        .bind(limit.created_at())
        .bind(limit.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save account limits error: {}", e))?;

        Ok(())
    }

    pub async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<AccountLimit>, String> {
        // This is a placeholder implementation
        // In production, would fetch from account_limits table and reconstruct entity
        Ok(None)
    }
}

/// Repository for InternalAccount persistence (FR-023, FR-024)
pub struct PgInternalAccountRepository {
    pool: PgPool,
}

impl PgInternalAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        PgInternalAccountRepository { pool }
    }

    pub async fn save(&self, account: &InternalAccount) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO account.internal_accounts (
                id,
                internal_type,
                balance,
                currency,
                status,
                correspondent_bank,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                balance = EXCLUDED.balance,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(account.id().as_uuid())
        .bind(account.internal_type().as_str())
        .bind(account.balance().amount_cents())
        .bind(account.balance().currency().code())
        .bind(account.status().as_str())
        .bind(account.correspondent_bank())
        .bind(account.created_at())
        .bind(account.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save internal account error: {}", e))?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &AccountId) -> Result<Option<InternalAccount>, String> {
        // Placeholder: would fetch from internal_accounts table
        Ok(None)
    }

    pub async fn find_by_type(&self, internal_type: InternalAccountType) -> Result<Vec<InternalAccount>, String> {
        // Placeholder: would filter by internal_type
        Ok(vec![])
    }
}

/// Repository for InterestCapitalization (FR-026)
pub struct PgInterestCapitalizationRepository {
    pool: PgPool,
}

impl PgInterestCapitalizationRepository {
    pub fn new(pool: PgPool) -> Self {
        PgInterestCapitalizationRepository { pool }
    }

    pub async fn save(&self, capitalization: &InterestCapitalization) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO account.interest_capitalizations (
                account_id,
                annual_rate,
                last_capitalization,
                total_interest_capitalized,
                currency
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (account_id) DO UPDATE SET
                last_capitalization = EXCLUDED.last_capitalization,
                total_interest_capitalized = EXCLUDED.total_interest_capitalized
            "#,
        )
        .bind(capitalization.account_id().as_uuid())
        .bind(capitalization.annual_rate())
        .bind(capitalization.last_capitalization())
        .bind(capitalization.total_interest_capitalized().amount_cents())
        .bind(capitalization.total_interest_capitalized().currency().code())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save interest capitalization error: {}", e))?;

        Ok(())
    }

    pub async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<InterestCapitalization>, String> {
        // Placeholder: would fetch and reconstruct entity
        Ok(None)
    }
}

/// Repository for BalanceNotifications (FR-028)
pub struct PgBalanceNotificationRepository {
    pool: PgPool,
}

impl PgBalanceNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        PgBalanceNotificationRepository { pool }
    }

    pub async fn save(&self, notification: &BalanceNotification) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO account.balance_notifications (
                account_id,
                notification_type,
                threshold,
                currency,
                is_active,
                created_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (account_id, notification_type) DO UPDATE SET
                is_active = EXCLUDED.is_active,
                threshold = EXCLUDED.threshold
            "#,
        )
        .bind(notification.account_id().as_uuid())
        .bind(notification.notification_type().as_str())
        .bind(notification.threshold().map(|m| m.amount_cents()))
        .bind(notification.threshold().map(|m| m.currency().code()))
        .bind(notification.is_active())
        .bind(notification.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save balance notification error: {}", e))?;

        Ok(())
    }

    pub async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<BalanceNotification>, String> {
        // Placeholder: would fetch all notifications for account
        Ok(vec![])
    }

    pub async fn find_by_account_id_and_type(
        &self,
        account_id: &AccountId,
        notification_type: &str,
    ) -> Result<Option<BalanceNotification>, String> {
        // Placeholder: would fetch specific notification
        Ok(None)
    }
}

// ==================== Port Trait (for application layer) ====================

/// Port for account limits persistence
#[async_trait]
pub trait IAccountLimitRepository: Send + Sync {
    async fn save(&self, limit: &AccountLimit) -> Result<(), String>;
    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<AccountLimit>, String>;
}

/// Port for internal accounts persistence
#[async_trait]
pub trait IInternalAccountRepository: Send + Sync {
    async fn save(&self, account: &InternalAccount) -> Result<(), String>;
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<InternalAccount>, String>;
    async fn find_by_type(&self, internal_type: InternalAccountType) -> Result<Vec<InternalAccount>, String>;
}

/// Port for interest capitalization persistence
#[async_trait]
pub trait IInterestCapitalizationRepository: Send + Sync {
    async fn save(&self, capitalization: &InterestCapitalization) -> Result<(), String>;
    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<InterestCapitalization>, String>;
}

/// Port for balance notifications persistence
#[async_trait]
pub trait IBalanceNotificationRepository: Send + Sync {
    async fn save(&self, notification: &BalanceNotification) -> Result<(), String>;
    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<BalanceNotification>, String>;
    async fn find_by_account_id_and_type(
        &self,
        account_id: &AccountId,
        notification_type: &str,
    ) -> Result<Option<BalanceNotification>, String>;
}

// Implement port traits for concrete repositories

#[async_trait]
impl IAccountLimitRepository for PgAccountLimitRepository {
    async fn save(&self, limit: &AccountLimit) -> Result<(), String> {
        self.save(limit).await
    }

    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<AccountLimit>, String> {
        self.find_by_account_id(account_id).await
    }
}

#[async_trait]
impl IInternalAccountRepository for PgInternalAccountRepository {
    async fn save(&self, account: &InternalAccount) -> Result<(), String> {
        self.save(account).await
    }

    async fn find_by_id(&self, id: &AccountId) -> Result<Option<InternalAccount>, String> {
        self.find_by_id(id).await
    }

    async fn find_by_type(&self, internal_type: InternalAccountType) -> Result<Vec<InternalAccount>, String> {
        self.find_by_type(internal_type).await
    }
}

#[async_trait]
impl IInterestCapitalizationRepository for PgInterestCapitalizationRepository {
    async fn save(&self, capitalization: &InterestCapitalization) -> Result<(), String> {
        self.save(capitalization).await
    }

    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<InterestCapitalization>, String> {
        self.find_by_account_id(account_id).await
    }
}

#[async_trait]
impl IBalanceNotificationRepository for PgBalanceNotificationRepository {
    async fn save(&self, notification: &BalanceNotification) -> Result<(), String> {
        self.save(notification).await
    }

    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<BalanceNotification>, String> {
        self.find_by_account_id(account_id).await
    }

    async fn find_by_account_id_and_type(
        &self,
        account_id: &AccountId,
        notification_type: &str,
    ) -> Result<Option<BalanceNotification>, String> {
        self.find_by_account_id_and_type(account_id, notification_type)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pg_account_limit_repository_new() {
        // Test would need a test database connection
        // Placeholder for compilation
    }

    #[test]
    fn test_pg_internal_account_repository_new() {
        // Test would need a test database connection
        // Placeholder for compilation
    }

    #[test]
    fn test_pg_interest_capitalization_repository_new() {
        // Test would need a test database connection
        // Placeholder for compilation
    }

    #[test]
    fn test_pg_balance_notification_repository_new() {
        // Test would need a test database connection
        // Placeholder for compilation
    }
}
