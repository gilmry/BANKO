use async_trait::async_trait;
use chrono::{DateTime, Utc};

use banko_domain::account::{Account, AccountId, Movement};
use banko_domain::shared::{CustomerId, Rib};

/// Port for account persistence -- implemented by infrastructure layer.
#[async_trait]
pub trait IAccountRepository: Send + Sync {
    async fn save(&self, account: &Account) -> Result<(), String>;
    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, String>;
    async fn find_by_customer_id(&self, customer_id: &CustomerId) -> Result<Vec<Account>, String>;
    async fn find_by_rib(&self, rib: &Rib) -> Result<Option<Account>, String>;
    async fn save_movement(&self, movement: &Movement) -> Result<(), String>;
    async fn find_movements_by_account(
        &self,
        account_id: &AccountId,
        limit: i64,
    ) -> Result<Vec<Movement>, String>;
    async fn find_movements_by_account_and_period(
        &self,
        account_id: &AccountId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<Vec<Movement>, String>;
    async fn delete(&self, id: &AccountId) -> Result<(), String>;
}

/// Port for checking if a customer's KYC is validated.
#[async_trait]
pub trait IKycVerifier: Send + Sync {
    async fn is_kyc_validated(&self, customer_id: &CustomerId) -> Result<bool, String>;
}
