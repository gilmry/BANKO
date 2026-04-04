use async_trait::async_trait;

use banko_domain::customer::Customer;
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
}

/// Port for PEP checking — implemented by infrastructure layer.
#[async_trait]
pub trait IPepCheckService: Send + Sync {
    async fn is_pep(&self, full_name: &str) -> Result<bool, String>;
}
