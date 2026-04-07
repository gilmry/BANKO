use std::sync::Arc;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use banko_domain::accounting::{
    FeeCharge, FeeDefinition, FeeGrid,
};

use super::errors::AccountingServiceError;
use super::ports::{IFeeChargeRepository, IFeeDefinitionRepository, IFeeGridRepository};

// ==================== ChargeResult ====================

#[derive(Debug, Clone)]
pub struct ChargeResult {
    pub total_charged: usize,
    pub total_unpaid: usize,
    pub amount_charged: Decimal,
    pub amount_unpaid: Decimal,
}

// ==================== FeeService ====================

pub struct FeeService {
    fee_def_repo: Arc<dyn IFeeDefinitionRepository>,
    fee_charge_repo: Arc<dyn IFeeChargeRepository>,
    fee_grid_repo: Arc<dyn IFeeGridRepository>,
}

impl FeeService {
    pub fn new(
        fee_def_repo: Arc<dyn IFeeDefinitionRepository>,
        fee_charge_repo: Arc<dyn IFeeChargeRepository>,
        fee_grid_repo: Arc<dyn IFeeGridRepository>,
    ) -> Self {
        FeeService {
            fee_def_repo,
            fee_charge_repo,
            fee_grid_repo,
        }
    }

    /// Calculate monthly fees for an account
    pub async fn calculate_monthly_fees(
        &self,
        account_id: Uuid,
        product_id: Uuid,
        balance: Decimal,
        segment: &str,
        day_of_month: u8,
    ) -> Result<Vec<FeeCharge>, AccountingServiceError> {
        // Get fee definitions for the product
        let mut fee_defs = self
            .fee_def_repo
            .list_by_product(product_id)
            .await
            .map_err(AccountingServiceError::Internal)?;

        // Get active fee grid for segment and date
        let now = Utc::now();
        if let Ok(Some(grid)) = self
            .fee_grid_repo
            .find_active_for_segment(segment, now)
            .await
        {
            // Apply grid overrides
            fee_defs = fee_defs
                .into_iter()
                .map(|mut def| {
                    if let Some(override_amount) = grid.get_fee_for_category(&def.category()) {
                        // Create a new definition with the overridden amount
                        def = FeeDefinition::new(
                            def.name().to_string(),
                            def.category(),
                            Some(override_amount),
                            None,
                            None,
                            None,
                            def.condition().clone(),
                            vec![],
                            def.currency().to_string(),
                        )
                        .unwrap_or(def);
                    }
                    def
                })
                .collect();
        }

        let mut charges = Vec::new();

        for def in fee_defs {
            // Check if fee applies to this segment
            if !def.applies_to_segment(segment) {
                continue;
            }

            // Check if condition is met
            if !def.is_condition_met(balance, Decimal::ZERO, day_of_month) {
                continue;
            }

            // Calculate fee amount
            let amount = def.calculate(balance);

            if amount > Decimal::ZERO {
                let charge = FeeCharge::new(def.id(), account_id, amount, None)
                    .map_err(|e| AccountingServiceError::InvalidEntry(e.to_string()))?;
                charges.push(charge);
            }
        }

        // Save all charges
        for charge in &charges {
            self.fee_charge_repo
                .save(charge)
                .await
                .map_err(AccountingServiceError::Internal)?;
        }

        Ok(charges)
    }

    /// Charge fees to an account
    pub async fn charge_fees(
        &self,
        _account_id: Uuid,
        fees: Vec<FeeCharge>,
    ) -> Result<ChargeResult, AccountingServiceError> {
        let mut result = ChargeResult {
            total_charged: 0,
            total_unpaid: 0,
            amount_charged: Decimal::ZERO,
            amount_unpaid: Decimal::ZERO,
        };

        for mut fee in fees {
            // In a real system, we'd check balance and create accounting entries
            // For now, we'll mark as charged if no special conditions
            fee.mark_charged();
            result.total_charged += 1;
            result.amount_charged += fee.amount();

            self.fee_charge_repo
                .update_status(&fee)
                .await
                .map_err(AccountingServiceError::Internal)?;
        }

        Ok(result)
    }

    /// Waive a fee charge
    pub async fn waive_fee(&self, _fee_charge_id: Uuid) -> Result<(), AccountingServiceError> {
        // In a real implementation, fetch the charge, update it, and save
        Ok(())
    }

    /// List all fees for an account
    pub async fn list_account_fees(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<FeeCharge>, AccountingServiceError> {
        self.fee_charge_repo
            .find_by_account(account_id)
            .await
            .map_err(AccountingServiceError::Internal)
    }

    /// Save a fee definition
    pub async fn create_fee_definition(
        &self,
        definition: FeeDefinition,
    ) -> Result<FeeDefinition, AccountingServiceError> {
        self.fee_def_repo
            .save(&definition)
            .await
            .map_err(AccountingServiceError::Internal)?;
        Ok(definition)
    }

    /// Get a fee definition by ID
    pub async fn get_fee_definition(
        &self,
        id: Uuid,
    ) -> Result<Option<FeeDefinition>, AccountingServiceError> {
        self.fee_def_repo
            .find_by_id(id)
            .await
            .map_err(AccountingServiceError::Internal)
    }

    /// List all fee definitions
    pub async fn list_fee_definitions(
        &self,
    ) -> Result<Vec<FeeDefinition>, AccountingServiceError> {
        self.fee_def_repo
            .list_all()
            .await
            .map_err(AccountingServiceError::Internal)
    }

    /// Create a fee grid
    pub async fn create_fee_grid(
        &self,
        grid: FeeGrid,
    ) -> Result<FeeGrid, AccountingServiceError> {
        self.fee_grid_repo
            .save(&grid)
            .await
            .map_err(AccountingServiceError::Internal)?;
        Ok(grid)
    }

    /// Get fee grid for segment
    pub async fn get_fee_grid_for_segment(
        &self,
        segment: &str,
    ) -> Result<Option<FeeGrid>, AccountingServiceError> {
        self.fee_grid_repo
            .find_by_segment(segment)
            .await
            .map_err(AccountingServiceError::Internal)
    }

    /// List all fee grids
    pub async fn list_fee_grids(&self) -> Result<Vec<FeeGrid>, AccountingServiceError> {
        self.fee_grid_repo
            .list_all()
            .await
            .map_err(AccountingServiceError::Internal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockFeeDefRepo {
        defs: Mutex<Vec<FeeDefinition>>,
    }

    impl MockFeeDefRepo {
        fn new() -> Self {
            MockFeeDefRepo {
                defs: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IFeeDefinitionRepository for MockFeeDefRepo {
        async fn save(&self, def: &FeeDefinition) -> Result<(), String> {
            let mut defs = self.defs.lock().unwrap();
            defs.retain(|d| d.id() != def.id());
            defs.push(def.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<FeeDefinition>, String> {
            Ok(self
                .defs
                .lock()
                .unwrap()
                .iter()
                .find(|d| d.id() == id)
                .cloned())
        }

        async fn list_by_product(
            &self,
            _product_id: Uuid,
        ) -> Result<Vec<FeeDefinition>, String> {
            Ok(self.defs.lock().unwrap().clone())
        }

        async fn list_all(&self) -> Result<Vec<FeeDefinition>, String> {
            Ok(self.defs.lock().unwrap().clone())
        }
    }

    struct MockFeeChargeRepo {
        charges: Mutex<Vec<FeeCharge>>,
    }

    impl MockFeeChargeRepo {
        fn new() -> Self {
            MockFeeChargeRepo {
                charges: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IFeeChargeRepository for MockFeeChargeRepo {
        async fn save(&self, charge: &FeeCharge) -> Result<(), String> {
            let mut charges = self.charges.lock().unwrap();
            charges.retain(|c| c.id() != charge.id());
            charges.push(charge.clone());
            Ok(())
        }

        async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<FeeCharge>, String> {
            Ok(self
                .charges
                .lock()
                .unwrap()
                .iter()
                .filter(|c| c.account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn find_pending(&self, account_id: Uuid) -> Result<Vec<FeeCharge>, String> {
            Ok(self
                .charges
                .lock()
                .unwrap()
                .iter()
                .filter(|c| c.account_id() == account_id && c.status() == FeeStatus::Pending)
                .cloned()
                .collect())
        }

        async fn update_status(&self, charge: &FeeCharge) -> Result<(), String> {
            let mut charges = self.charges.lock().unwrap();
            if let Some(idx) = charges.iter().position(|c| c.id() == charge.id()) {
                charges[idx] = charge.clone();
            }
            Ok(())
        }
    }

    struct MockFeeGridRepo {
        grids: Mutex<Vec<FeeGrid>>,
    }

    impl MockFeeGridRepo {
        fn new() -> Self {
            MockFeeGridRepo {
                grids: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IFeeGridRepository for MockFeeGridRepo {
        async fn save(&self, grid: &FeeGrid) -> Result<(), String> {
            let mut grids = self.grids.lock().unwrap();
            grids.retain(|g| g.id() != grid.id());
            grids.push(grid.clone());
            Ok(())
        }

        async fn find_by_segment(&self, segment: &str) -> Result<Option<FeeGrid>, String> {
            Ok(self
                .grids
                .lock()
                .unwrap()
                .iter()
                .find(|g| g.segment() == segment)
                .cloned())
        }

        async fn find_active_for_segment(
            &self,
            segment: &str,
            date: chrono::DateTime<chrono::Utc>,
        ) -> Result<Option<FeeGrid>, String> {
            Ok(self
                .grids
                .lock()
                .unwrap()
                .iter()
                .find(|g| g.segment() == segment && g.is_effective_at(date))
                .cloned())
        }

        async fn list_all(&self) -> Result<Vec<FeeGrid>, String> {
            Ok(self.grids.lock().unwrap().clone())
        }
    }

    #[tokio::test]
    async fn test_create_fee_definition() {
        let repo = Arc::new(MockFeeDefRepo::new());
        let charge_repo = Arc::new(MockFeeChargeRepo::new());
        let grid_repo = Arc::new(MockFeeGridRepo::new());
        let service = FeeService::new(repo, charge_repo, grid_repo);

        let def = FeeDefinition::new(
            "Test Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(10)),
            None,
            None,
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        let result = service.create_fee_definition(def.clone()).await.unwrap();
        assert_eq!(result.id(), def.id());
    }

    #[tokio::test]
    async fn test_list_account_fees() {
        let def_repo = Arc::new(MockFeeDefRepo::new());
        let charge_repo = Arc::new(MockFeeChargeRepo::new());
        let grid_repo = Arc::new(MockFeeGridRepo::new());
        let service = FeeService::new(def_repo, charge_repo.clone(), grid_repo);

        let account_id = Uuid::new_v4();
        let charge = FeeCharge::new(
            Uuid::new_v4(),
            account_id,
            Decimal::from(10),
            None,
        )
        .unwrap();

        charge_repo.save(&charge).await.unwrap();

        let fees = service.list_account_fees(account_id).await.unwrap();
        assert_eq!(fees.len(), 1);
        assert_eq!(fees[0].account_id(), account_id);
    }
}
