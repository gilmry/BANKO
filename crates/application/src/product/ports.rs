use async_trait::async_trait;
use uuid::Uuid;

use banko_domain::product::{Product, ProductType, PricingGrid};

// ============================================================
// IProductRepository - Product Repository Port
// ============================================================

#[async_trait]
pub trait IProductRepository: Send + Sync {
    /// Save or update a product
    async fn save(&self, product: &Product) -> Result<(), String>;

    /// Find a product by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, String>;

    /// List all products
    async fn list_all(&self) -> Result<Vec<Product>, String>;

    /// List active products
    async fn list_active(&self) -> Result<Vec<Product>, String>;

    /// Find products by type
    async fn find_by_type(&self, product_type: ProductType) -> Result<Vec<Product>, String>;

    /// Update a product (idempotent save)
    async fn update(&self, product: &Product) -> Result<(), String>;
}

// ============================================================
// IPricingGridRepository - Pricing Grid Repository Port
// ============================================================

#[async_trait]
pub trait IPricingGridRepository: Send + Sync {
    /// Save a pricing grid
    async fn save(&self, grid: &PricingGrid) -> Result<(), String>;

    /// Find all pricing grids for a product
    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<PricingGrid>, String>;

    /// Find active pricing grid(s) for a product at a specific date
    async fn find_active_for_product(
        &self,
        product_id: Uuid,
        as_of_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<PricingGrid>, String>;

    /// List all pricing grids
    async fn list_all(&self) -> Result<Vec<PricingGrid>, String>;
}
