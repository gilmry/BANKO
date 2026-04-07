use async_trait::async_trait;

use banko_domain::collateral::{
    Collateral, CollateralAllocation, CollateralId, CollateralStatus, CollateralType,
    CollateralValuation, LtvCalculation,
};
use banko_domain::shared::value_objects::CustomerId;

/// Port for collateral persistence — implemented by infrastructure layer.
#[async_trait]
pub trait ICollateralRepository: Send + Sync {
    /// Save a new collateral or update an existing one.
    async fn save(&self, collateral: &Collateral) -> Result<(), String>;

    /// Find a collateral by ID.
    async fn find_by_id(&self, id: &CollateralId) -> Result<Option<Collateral>, String>;

    /// Find all collaterals for a customer.
    async fn find_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<Collateral>, String>;

    /// Find all collaterals with optional filtering and pagination.
    async fn find_all(
        &self,
        customer_id: Option<&CustomerId>,
        status: Option<CollateralStatus>,
        collateral_type: Option<CollateralType>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Collateral>, String>;

    /// Count all collaterals with optional filtering.
    async fn count_all(
        &self,
        customer_id: Option<&CustomerId>,
        status: Option<CollateralStatus>,
        collateral_type: Option<CollateralType>,
    ) -> Result<i64, String>;

    /// Find collaterals due for revaluation.
    async fn find_revaluation_due(&self) -> Result<Vec<Collateral>, String>;

    /// Delete a collateral by ID.
    async fn delete(&self, id: &CollateralId) -> Result<(), String>;
}

/// Port for collateral valuation persistence.
#[async_trait]
pub trait ICollateralValuationRepository: Send + Sync {
    /// Save a new valuation.
    async fn save(&self, valuation: &CollateralValuation) -> Result<(), String>;

    /// Find valuations by collateral ID.
    async fn find_by_collateral_id(
        &self,
        collateral_id: &CollateralId,
    ) -> Result<Vec<CollateralValuation>, String>;

    /// Find the latest valuation for a collateral.
    async fn find_latest(
        &self,
        collateral_id: &CollateralId,
    ) -> Result<Option<CollateralValuation>, String>;
}

/// Port for collateral allocation persistence.
#[async_trait]
pub trait ICollateralAllocationRepository: Send + Sync {
    /// Save a new allocation.
    async fn save(&self, allocation: &CollateralAllocation) -> Result<(), String>;

    /// Find allocations by collateral ID.
    async fn find_by_collateral_id(
        &self,
        collateral_id: &CollateralId,
    ) -> Result<Vec<CollateralAllocation>, String>;

    /// Find allocations by loan ID.
    async fn find_by_loan_id(&self, loan_id: &str) -> Result<Vec<CollateralAllocation>, String>;

    /// Find all allocations for a collateral and loan pair.
    async fn find_by_collateral_and_loan(
        &self,
        collateral_id: &CollateralId,
        loan_id: &str,
    ) -> Result<Vec<CollateralAllocation>, String>;
}

/// Port for LTV calculation and validation.
#[async_trait]
pub trait ILtvCalculationPort: Send + Sync {
    /// Calculate and validate LTV for a loan.
    async fn calculate(
        &self,
        calculation: &LtvCalculation,
    ) -> Result<bool, String>;

    /// Check if LTV is compliant.
    async fn is_compliant(&self, calculation: &LtvCalculation) -> Result<bool, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collateral_repository_trait() {
        // This test ensures the trait is properly defined
        // Actual implementations will be in the infrastructure layer
    }

    #[tokio::test]
    async fn test_collateral_valuation_repository_trait() {
        // This test ensures the trait is properly defined
    }

    #[tokio::test]
    async