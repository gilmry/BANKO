use async_trait::async_trait;
use std::sync::Mutex;
use uuid::Uuid;

use banko_application::product::{IPricingGridRepository, IProductRepository};
use banko_domain::product::{Product, ProductType, PricingGrid};

// ============================================================
// ProductRepository - In-Memory Implementation
// ============================================================

/// In-memory implementation of ProductRepository for testing
/// In production, this would use SQLx with PostgreSQL
pub struct ProductRepository {
    products: Mutex<Vec<Product>>,
}

impl ProductRepository {
    pub fn new() -> Self {
        ProductRepository {
            products: Mutex::new(Vec::new()),
        }
    }
}

impl Default for ProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IProductRepository for ProductRepository {
    async fn save(&self, product: &Product) -> Result<(), String> {
        let mut products = self.products.lock().unwrap();
        // Remove if exists (upsert)
        products.retain(|p| p.id() != product.id());
        products.push(product.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, String> {
        let products = self.products.lock().unwrap();
        Ok(products.iter().find(|p| p.id() == id).cloned())
    }

    async fn list_all(&self) -> Result<Vec<Product>, String> {
        let products = self.products.lock().unwrap();
        Ok(products.clone())
    }

    async fn list_active(&self) -> Result<Vec<Product>, String> {
        let products = self.products.lock().unwrap();
        Ok(products
            .iter()
            .filter(|p| p.status().as_str() == "Active")
            .cloned()
            .collect())
    }

    async fn find_by_type(&self, product_type: ProductType) -> Result<Vec<Product>, String> {
        let products = self.products.lock().unwrap();
        Ok(products
            .iter()
            .filter(|p| p.product_type() == product_type)
            .cloned()
            .collect())
    }

    async fn update(&self, product: &Product) -> Result<(), String> {
        let mut products = self.products.lock().unwrap();
        if let Some(pos) = products.iter().position(|p| p.id() == product.id()) {
            products[pos] = product.clone();
            Ok(())
        } else {
            Err("Product not found".to_string())
        }
    }
}

// ============================================================
// PricingGridRepository - In-Memory Implementation
// ============================================================

/// In-memory implementation of PricingGridRepository for testing
/// In production, this would use SQLx with PostgreSQL
pub struct PricingGridRepository {
    grids: Mutex<Vec<PricingGrid>>,
}

impl PricingGridRepository {
    pub fn new() -> Self {
        PricingGridRepository {
            grids: Mutex::new(Vec::new()),
        }
    }
}

impl Default for PricingGridRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IPricingGridRepository for PricingGridRepository {
    async fn save(&self, grid: &PricingGrid) -> Result<(), String> {
        let mut grids = self.grids.lock().unwrap();
        grids.retain(|g| g.id() != grid.id());
        grids.push(grid.clone());
        Ok(())
    }

    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<PricingGrid>, String> {
        let grids = self.grids.lock().unwrap();
        Ok(grids
            .iter()
            .filter(|g| g.product_id() == product_id)
            .cloned()
            .collect())
    }

    async fn find_active_for_product(
        &self,
        product_id: Uuid,
        as_of_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<PricingGrid>, String> {
        let grids = self.grids.lock().unwrap();
        Ok(grids
            .iter()
            .filter(|g| {
                g.product_id() == product_id
                    && g.is_active()
                    && g.is_effective_at(as_of_date)
            })
            .cloned()
            .collect())
    }

    async fn list_all(&self) -> Result<Vec<PricingGrid>, String> {
        let grids = self.grids.lock().unwrap();
        Ok(grids.clone())
    }
}
