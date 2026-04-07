use chrono::{Duration, NaiveDate, Utc};

use banko_domain::collateral::{
    Collateral, CollateralAllocation, CollateralId, CollateralStatus, CollateralType,
    CollateralValuation, LtvCalculation, ValuationMethod,
};
use banko_domain::shared::errors::DomainError;
use banko_domain::shared::value_objects::{Currency, CustomerId, Money};

use super::errors::CollateralApplicationError;
use super::ports::{
    ICollateralAllocationRepository, ICollateralRepository, ICollateralValuationRepository,
    ILtvCalculationPort,
};
use super::dto::*;

/// Collateral service orchestrating use cases.
pub struct CollateralService {
    collateral_repo: std::sync::Arc<dyn ICollateralRepository>,
    valuation_repo: std::sync::Arc<dyn ICollateralValuationRepository>,
    allocation_repo: std::sync::Arc<dyn ICollateralAllocationRepository>,
    ltv_calculator: std::sync::Arc<dyn ILtvCalculationPort>,
}

impl CollateralService {
    pub fn new(
        collateral_repo: std::sync::Arc<dyn ICollateralRepository>,
        valuation_repo: std::sync::Arc<dyn ICollateralValuationRepository>,
        allocation_repo: std::sync::Arc<dyn ICollateralAllocationRepository>,
        ltv_calculator: std::sync::Arc<dyn ILtvCalculationPort>,
    ) -> Self {
        CollateralService {
            collateral_repo,
            valuation_repo,
            allocation_repo,
            ltv_calculator,
        }
    }

    /// Create a new collateral.
    pub async fn create_collateral(
        &self,
        request: CreateCollateralRequest,
    ) -> Result<CollateralResponse, CollateralApplicationError> {
        let collateral_type = CollateralType::from_str(&request.collateral_type)?;
        let customer_id = CustomerId::from_uuid(
            uuid::Uuid::parse_str(&request.customer_id)
                .map_err(|_| CollateralApplicationError::ValidationError(
                    "Invalid customer ID".to_string(),
                ))?,
        );

        let market_value =
            Money::new(request.market_value, Currency::TND).map_err(|e| {
                CollateralApplicationError::ValidationError(e.to_string())
            })?;

        let valuation_date =
            NaiveDate::parse_from_str(&request.valuation_date, "%Y-%m-%d").map_err(|_| {
                CollateralApplicationError::ValidationError(
                    "Invalid valuation date format (use YYYY-MM-DD)".to_string(),
                )
            })?;

        let collateral = Collateral::new(
            collateral_type,
            request.description,
            market_value,
            request.haircut_pct,
            valuation_date,
            customer_id,
            request.insurance_policy_id,
        )
        .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        self.collateral_repo
            .save(&collateral)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(self.collateral_to_response(&collateral))
    }

    /// Get a collateral by ID.
    pub async fn get_collateral(
        &self,
        collateral_id: &str,
    ) -> Result<CollateralDetailResponse, CollateralApplicationError> {
        let id = CollateralId::parse(collateral_id)
            .map_err(|_| CollateralApplicationError::CollateralNotFound)?;

        let collateral = self
            .collateral_repo
            .find_by_id(&id)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?
            .ok_or(CollateralApplicationError::CollateralNotFound)?;

        let allocations = self
            .allocation_repo
            .find_by_collateral_id(&id)
            .await
            .unwrap_or_default();

        let latest_valuation = self
            .valuation_repo
            .find_latest(&id)
            .await
            .unwrap_or(None)
            .map(|v| self.valuation_to_response(&v));

        Ok(CollateralDetailResponse {
            collateral: self.collateral_to_response(&collateral),
            allocations: allocations
                .iter()
                .map(|a| self.allocation_to_response(a))
                .collect(),
            latest_valuation,
        })
    }

    /// List collaterals with pagination.
    pub async fn list_collaterals(
        &self,
        query: ListCollateralsQuery,
    ) -> Result<PaginatedCollateralsResponse, CollateralApplicationError> {
        let customer_id = query
            .customer_id
            .as_ref()
            .map(|id| {
                CustomerId::from_uuid(
                    uuid::Uuid::parse_str(id)
                        .map_err(|_| CollateralApplicationError::ValidationError(
                            "Invalid customer ID".to_string(),
                        ))?,
                )
            })
            .transpose()?;

        let status = query
            .status
            .as_ref()
            .map(|s| CollateralStatus::from_str(s))
            .transpose()?;

        let collateral_type = query
            .collateral_type
            .as_ref()
            .map(|ct| CollateralType::from_str(ct))
            .transpose()?;

        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10).min(100);
        let offset = (page - 1) * limit;

        let collaterals = self
            .collateral_repo
            .find_all(customer_id.as_ref(), status, collateral_type, limit, offset)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        let total = self
            .collateral_repo
            .count_all(customer_id.as_ref(), status, collateral_type)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(PaginatedCollateralsResponse {
            data: collaterals.iter().map(|c| self.collateral_to_response(c)).collect(),
            total,
            page,
            limit,
        })
    }

    /// Revalue a collateral.
    pub async fn revalue_collateral(
        &self,
        request: RevalueCollateralRequest,
    ) -> Result<CollateralResponse, CollateralApplicationError> {
        let id = CollateralId::parse(&request.collateral_id)
            .map_err(|_| CollateralApplicationError::CollateralNotFound)?;

        let mut collateral = self
            .collateral_repo
            .find_by_id(&id)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?
            .ok_or(CollateralApplicationError::CollateralNotFound)?;

        let new_market_value = Money::new(request.new_market_value, Currency::TND)
            .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        let valuation_date = NaiveDate::parse_from_str(&request.valuation_date, "%Y-%m-%d")
            .map_err(|_| {
                CollateralApplicationError::ValidationError(
                    "Invalid valuation date format (use YYYY-MM-DD)".to_string(),
                )
            })?;

        // Revalue in domain
        collateral
            .revalue(
                new_market_value.clone(),
                request.new_haircut_pct,
                valuation_date,
            )
            .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        // Save revaluation record
        let valuation_method = ValuationMethod::from_str(&request.valuation_method)?;
        let valuation = CollateralValuation::new(
            uuid::Uuid::new_v4().to_string(),
            id.clone(),
            valuation_date,
            new_market_value,
            request.appraiser,
            valuation_method,
            collateral.collateral_type(),
        )
        .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        self.valuation_repo
            .save(&valuation)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        // Save updated collateral
        self.collateral_repo
            .save(&collateral)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(self.collateral_to_response(&collateral))
    }

    /// Activate a collateral (transition from Pending to Active).
    pub async fn activate_collateral(
        &self,
        collateral_id: &str,
    ) -> Result<CollateralResponse, CollateralApplicationError> {
        let id = CollateralId::parse(collateral_id)
            .map_err(|_| CollateralApplicationError::CollateralNotFound)?;

        let mut collateral = self
            .collateral_repo
            .find_by_id(&id)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?
            .ok_or(CollateralApplicationError::CollateralNotFound)?;

        collateral
            .activate()
            .map_err(|e| CollateralApplicationError::InvalidStateTransition(e.to_string()))?;

        self.collateral_repo
            .save(&collateral)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(self.collateral_to_response(&collateral))
    }

    /// Release a collateral.
    pub async fn release_collateral(
        &self,
        collateral_id: &str,
    ) -> Result<CollateralResponse, CollateralApplicationError> {
        let id = CollateralId::parse(collateral_id)
            .map_err(|_| CollateralApplicationError::CollateralNotFound)?;

        let mut collateral = self
            .collateral_repo
            .find_by_id(&id)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?
            .ok_or(CollateralApplicationError::CollateralNotFound)?;

        collateral
            .release()
            .map_err(|e| CollateralApplicationError::InvalidStateTransition(e.to_string()))?;

        self.collateral_repo
            .save(&collateral)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(self.collateral_to_response(&collateral))
    }

    /// Allocate collateral to a loan.
    pub async fn allocate_collateral(
        &self,
        request: AllocateCollateralRequest,
    ) -> Result<CollateralAllocationResponse, CollateralApplicationError> {
        let collateral_id = CollateralId::parse(&request.collateral_id)
            .map_err(|_| CollateralApplicationError::CollateralNotFound)?;

        // Verify collateral exists
        let collateral = self
            .collateral_repo
            .find_by_id(&collateral_id)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?
            .ok_or(CollateralApplicationError::CollateralNotFound)?;

        if collateral.status() != CollateralStatus::Active {
            return Err(CollateralApplicationError::InvalidStateTransition(format!(
                "Collateral must be Active to allocate, current status: {}",
                collateral.status()
            )));
        }

        let allocated_amount = Money::new(request.allocated_amount, Currency::TND)
            .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        let allocation = CollateralAllocation::new(
            collateral_id,
            request.loan_id,
            allocated_amount,
            Utc::now(),
        )
        .map_err(|e| CollateralApplicationError::AllocationError(e.to_string()))?;

        self.allocation_repo
            .save(&allocation)
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(self.allocation_to_response(&allocation))
    }

    /// Calculate LTV for a loan.
    pub async fn calculate_ltv(
        &self,
        request: CalculateLtvRequest,
    ) -> Result<LtvCalculationResponse, CollateralApplicationError> {
        let collateral_ids: Result<Vec<_>, _> = request
            .collateral_ids
            .iter()
            .map(|id| CollateralId::parse(id))
            .collect();

        let collateral_ids = collateral_ids
            .map_err(|_| CollateralApplicationError::ValidationError(
                "Invalid collateral ID".to_string(),
            ))?;

        let collateral_types: Result<Vec<_>, _> = request
            .collateral_types
            .iter()
            .map(|ct| CollateralType::from_str(ct))
            .collect();

        let collateral_types = collateral_types?;

        if collateral_ids.len() != collateral_types.len() {
            return Err(CollateralApplicationError::ValidationError(
                "Collateral IDs and types count mismatch".to_string(),
            ));
        }

        // Fetch collaterals and sum their net values
        let mut total_collateral_value = 0i64;
        for id in &collateral_ids {
            let collateral = self
                .collateral_repo
                .find_by_id(id)
                .await
                .map_err(|e| CollateralApplicationError::RepositoryError(e))?
                .ok_or(CollateralApplicationError::CollateralNotFound)?;

            total_collateral_value += collateral.net_value().amount_cents();
        }

        let total_loan_amount = Money::new(request.total_loan_amount, Currency::TND)
            .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        let total_collateral_value =
            Money::from_cents(total_collateral_value, Currency::TND);

        let ltv = LtvCalculation::calculate(
            request.loan_id,
            collateral_ids,
            total_loan_amount,
            total_collateral_value,
            &collateral_types,
        )
        .map_err(|e| CollateralApplicationError::DomainError(e.to_string()))?;

        // Validate LTV compliance
        if !ltv.is_compliant() {
            return Err(CollateralApplicationError::LtvComplianceViolation {
                current_ltv: ltv.ltv_ratio(),
                max_ltv: ltv.max_ltv_threshold(),
            });
        }

        Ok(LtvCalculationResponse {
            loan_id: ltv.loan_id().to_string(),
            collateral_ids: ltv
                .collateral_ids()
                .iter()
                .map(|id| id.to_string())
                .collect(),
            total_loan_amount: total_loan_amount.amount(),
            total_collateral_value: total_collateral_value.amount(),
            ltv_ratio: ltv.ltv_ratio(),
            max_ltv_threshold: ltv.max_ltv_threshold(),
            is_compliant: ltv.is_compliant(),
        })
    }

    /// Find collaterals due for revaluation.
    pub async fn find_revaluation_due(
        &self,
    ) -> Result<Vec<CollateralResponse>, CollateralApplicationError> {
        let collaterals = self
            .collateral_repo
            .find_revaluation_due()
            .await
            .map_err(|e| CollateralApplicationError::RepositoryError(e))?;

        Ok(collaterals.iter().map(|c| self.collateral_to_response(c)).collect())
    }

    // --- Helper methods ---

    fn collateral_to_response(&self, collateral: &Collateral) -> CollateralResponse {
        let today = NaiveDate::from(Utc::now().naive_utc().date());
        CollateralResponse {
            id: collateral.id().to_string(),
            collateral_type: collateral.collateral_type().to_string(),
            description: collateral.description().to_string(),
            market_value: collateral.market_value().amount(),
            haircut_pct: collateral.haircut_pct(),
            net_value: collateral.net_value().amount(),
            valuation_date: collateral.valuation_date().to_string(),
            next_revaluation_date: collateral.next_revaluation_date().to_string(),
            status: collateral.status().to_string(),
            customer_id: collateral.customer_id().to_string(),
            insurance_policy_id: collateral.insurance_policy_id().map(|s| s.to_string()),
            is_revaluation_due: collateral.is_revaluation_due(today),
            created_at: collateral.created_at(),
            updated_at: collateral.updated_at(),
        }
    }

    fn valuation_to_response(&self, valuation: &CollateralValuation) -> CollateralValuationResponse {
        let today = NaiveDate::from(Utc::now().naive_utc().date());
        CollateralValuationResponse {
            valuation_id: valuation.valuation_id().to_string(),
            collateral_id: valuation.collateral_id().to_string(),
            valuation_date: valuation.valuation_date().to_string(),
            market_value: valuation.market_value().amount(),
            appraiser: valuation.appraiser().to_string(),
            method: valuation.method().to_string(),
            next_revaluation_date: valuation.next_revaluation_date().to_string(),
            is_revaluation_due: valuation.is_revaluation_due(today),
        }
    }

    fn allocation_to_response(&self, allocation: &CollateralAllocation) -> CollateralAllocationResponse {
        CollateralAllocationResponse {
            collateral_id: allocation.collateral_id().to_string(),
            loan_id: allocation.loan_id().to_string(),
            allocated_amount: allocation.allocated_amount().amount(),
            allocation_date: allocation.allocation_date(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    // Mock repositories for testing
    struct MockCollateralRepository;
    struct MockValuationRepository;
    struct MockAllocationRepository;
    struct MockLtvCalculator;

    #[async_trait]
    impl ICollateralRepository for MockCollateralRepository {
        async fn save(&self, _collateral: &Collateral) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &CollateralId) -> Result<Option<Collateral>, String> {
            Ok(None)
        }

        async fn find_by_customer_id(&self, _customer_id: &CustomerId) -> Result<Vec<Collateral>, String> {
            Ok(vec![])
        }

        async fn find_all(
            &self,
            _customer_id: Option<&CustomerId>,
            _status: Option<CollateralStatus>,
            _collateral_type: Option<CollateralType>,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<Collateral>, String> {
            Ok(vec![])
        }

        async fn count_all(
            &self,
            _customer_id: Option<&CustomerId>,
            _status: Option<CollateralStatus>,
            _collateral_type: Option<CollateralType>,
        ) -> Result<i64, String> {
            Ok(0)
        }

        async fn find_revaluation_due(&self) -> Result<Vec<Collateral>, String> {
            Ok(vec![])
        }

        async fn delete(&self, _id: &CollateralId) -> Result<(), String> {
            Ok(())
        }
    }

    #[async_trait]
    impl ICollateralValuationRepository for MockValuationRepository {
        async fn save(&self, _valuation: &CollateralValuation) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_collateral_id(&self, _collateral_id: &CollateralId) -> Result<Vec<CollateralValuation>, String> {
            Ok(vec![])
        }

        async fn find_latest(&self, _collateral_id: &CollateralId) -> Result<Option<CollateralValuation>, String> {
            Ok(None)
        }
    }

    #[async_trait]
    impl ICollateralAllocationRepository for MockAllocationRepository {
        async fn save(&self, _allocation: &CollateralAllocation) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_collateral_id(&self, _collateral_id: &CollateralId) -> Result<Vec<CollateralAllocation>, String> {
            Ok(vec![])
        }

        async fn find_by_loan_id(&self, _loan_id: &str) -> Result<Vec<CollateralAllocation>, String> {
            Ok(vec![])
        }

        async fn find_by_collateral_and_loan(&self, _collateral_id: &CollateralId, _loan_id: &str) -> Result<Vec<CollateralAllocation>, String> {
            Ok(vec![])
        }
    }

    #[async_trait]
    impl ILtvCalculationPort for MockLtvCalculator {
        async fn calculate(&self, _calculation: &LtvCalculation) -> Result<bool, String> {
            Ok(true)
        }

        async fn is_compliant(&self, _calculation: &LtvCalcula