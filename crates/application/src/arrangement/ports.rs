use async_trait::async_trait;

use banko_domain::arrangement::{
    Arrangement, ArrangementId, ArrangementBundle, ArrangementBundleId, ArrangementStatus,
    ArrangementType,
};
use banko_domain::shared::CustomerId;

/// Port for arrangement persistence
#[async_trait]
pub trait IArrangementRepository: Send + Sync {
    async fn save(&self, arrangement: &Arrangement) -> Result<(), String>;
    async fn find_by_id(&self, id: &ArrangementId) -> Result<Option<Arrangement>, String>;
    async fn find_by_customer_id(&self, customer_id: &CustomerId) -> Result<Vec<Arrangement>, String>;
    async fn find_by_status(&self, status: ArrangementStatus) -> Result<Vec<Arrangement>, String>;
    async fn find_by_type(&self, arrangement_type: ArrangementType) -> Result<Vec<Arrangement>, String>;
    async fn find_active(&self) -> Result<Vec<Arrangement>, String>;
    async fn find_maturing_soon(&self, days: i64) -> Result<Vec<Arrangement>, String>;
    async fn delete(&self, id: &ArrangementId) -> Result<(), String>;
}

/// Port for arrangement bundle persistence
#[async_trait]
pub trait IArrangementBundleRepository: Send + Sync {
    async fn save(&self, bundle: &ArrangementBundle) -> Result<(), String>;
    async fn find_by_id(&self, id: &ArrangementBundleId) -> Result<Option<ArrangementBundle>, String>;
    async fn find_active(&self) -> Result<Vec<ArrangementBundle>, String>;
    async fn find_all(&self) -> Result<Vec<ArrangementBundle>, String>;
    async fn delete(&self, id: &ArrangementBundleId) -> Result<(), String>;
}

/// Port for product validation
#[async_trait]
pub trait IProductValidator: Send + Sync {
    async fn validate_product(&self, product_id: &str) -> Result<bool, String>;
    async fn get_product_details(&self, product_id: &str) -> Result<ProductDetails, String>;
}

pub struct ProductDetails {
    pub product_id: String,
    pub name: String,
    pub arrangement_types: Vec<String>,
    pub is_active: bool,
}

/// Port for customer validation
#[async_trait]
pub trait ICustomerValidator: Send + Sync {
    async fn validate_customer(&self, customer_id: &CustomerId) -> Result<bool, String>;
    async fn get_customer_kyc_status(&self, customer_id: &CustomerId) -> Result<bool, String>;
}

/// Port for arrangement renewal engine
#[async_trait]
pub trait IArrangementRenewalEngine: Send + Sync {
    async fn trigger_renewal_check(&self, arrangement_id: &ArrangementId) -> Result<bool, String>;
    async fn create_renewal(&self, arrangement_id: &ArrangementId) -> Result<ArrangementId, String>;
}

/// Port for notification service
#[async_trait]
pub trait INotificationService: Send + Sync {
    async fn notify_maturity_upcoming(
        &self,
        arrangement_id: &ArrangementId,
        customer_id: &CustomerId,
        days_remaining: i64,
    ) -> Result<(), String>;

    async fn notify_status_change(
        &self,
        arrangement_id: &ArrangementId,
        customer_id: &CustomerId,
        new_status: &str,
    ) -> Result<(), String>;
}
