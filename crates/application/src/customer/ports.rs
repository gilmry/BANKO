use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use banko_domain::customer::{ConsentRecord, Customer, DataRequestId, DataRightsRequest};
use banko_domain::shared::value_objects::{CustomerId, EmailAddress};

/// Port for customer persistence — implemented by infrastructure layer.
#[async_trait]
pub trait ICustomerRepository: Send + Sync {
    async fn save(&self, customer: &Customer) -> Result<(), String>;
    async fn find_by_id(&self, id: &CustomerId) -> Result<Option<Customer>, String>;
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<Customer>, String>;
    async fn list_all(&self) -> Result<Vec<Customer>, String>;
    async fn list_by_status(&self, status: &str) -> Result<Vec<Customer>, String>;
    async fn delete(&self, id: &CustomerId) -> Result<(), String>;
    /// Find all customers with status Closed and closed_at before the given date.
    async fn find_closed_before(&self, before: DateTime<Utc>) -> Result<Vec<Customer>, String>;
}

/// Port for PEP checking — implemented by infrastructure layer.
#[async_trait]
pub trait IPepCheckService: Send + Sync {
    async fn is_pep(&self, full_name: &str) -> Result<bool, String>;
}

/// Port for consent persistence — implemented by infrastructure layer.
#[async_trait]
pub trait IConsentRepository: Send + Sync {
    async fn save(&self, consent: &ConsentRecord) -> Result<(), String>;
    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<ConsentRecord>, String>;
    async fn find_by_customer_and_purpose(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<Option<ConsentRecord>, String>;
    async fn find_active_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<ConsentRecord>, String>;
}

/// Port for data rights request persistence — implemented by infrastructure layer.
#[async_trait]
pub trait IDataRightsRepository: Send + Sync {
    async fn save(&self, request: &DataRightsRequest) -> Result<(), String>;
    async fn find_by_id(&self, id: &DataRequestId) -> Result<Option<DataRightsRequest>, String>;
    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<DataRightsRequest>, String>;
}

/// Port for accessing account data (for data portability) — implemented by account infrastructure.
#[async_trait]
pub trait IAccountDataProvider: Send + Sync {
    async fn find_accounts_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<serde_json::Value>, String>;
    async fn find_transactions_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<serde_json::Value>, String>;
}
