use std::sync::Arc;
use chrono::Duration;

use banko_domain::arrangement::{
    Arrangement, ArrangementId, ArrangementType, ArrangementStatus, ArrangementTerms,
    RenewalType, ArrangementBundle, ArrangementBundleId,
};
use banko_domain::shared::CustomerId;

use super::dto::{
    CreateArrangementRequest, ArrangementResponse, ArrangementTermsDto, CreateBundleRequest,
    ArrangementBundleResponse,
};
use super::errors::ArrangementError;
use super::ports::{
    IArrangementRepository, IArrangementBundleRepository, IProductValidator,
    ICustomerValidator, IArrangementRenewalEngine, INotificationService,
};

pub struct ArrangementService {
    arrangement_repo: Arc<dyn IArrangementRepository>,
    bundle_repo: Arc<dyn IArrangementBundleRepository>,
    product_validator: Arc<dyn IProductValidator>,
    customer_validator: Arc<dyn ICustomerValidator>,
    renewal_engine: Arc<dyn IArrangementRenewalEngine>,
    notification_service: Arc<dyn INotificationService>,
}

impl ArrangementService {
    pub fn new(
        arrangement_repo: Arc<dyn IArrangementRepository>,
        bundle_repo: Arc<dyn IArrangementBundleRepository>,
        product_validator: Arc<dyn IProductValidator>,
        customer_validator: Arc<dyn ICustomerValidator>,
        renewal_engine: Arc<dyn IArrangementRenewalEngine>,
        notification_service: Arc<dyn INotificationService>,
    ) -> Self {
        ArrangementService {
            arrangement_repo,
            bundle_repo,
            product_validator,
            customer_validator,
            renewal_engine,
            notification_service,
        }
    }

    // --- Arrangement Operations ---

    pub async fn create_arrangement(
        &self,
        request: CreateArrangementRequest,
    ) -> Result<ArrangementResponse, ArrangementError> {
        let customer_id = CustomerId::parse(&request.customer_id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        // Validate customer exists and KYC is done
        let customer_valid = self
            .customer_validator
            .validate_customer(&customer_id)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        if !customer_valid {
            return Err(ArrangementError::CustomerNotFound);
        }

        // Validate product exists
        let product_valid = self
            .product_validator
            .validate_product(&request.product_id)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        if !product_valid {
            return Err(ArrangementError::ProductNotFound);
        }

        let arrangement_type =
            ArrangementType::from_str(&request.arrangement_type)
                .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let renewal_type = RenewalType::from_str(&request.terms.renewal_type)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let terms = ArrangementTerms::new(
            &request.terms.currency,
            request.terms.interest_rate,
            request.terms.fee_schedule_id,
            request.terms.minimum_balance,
            request.terms.overdraft_limit,
            renewal_type,
            request.terms.notice_period_days,
        )
        .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let arrangement = Arrangement::new(
            customer_id,
            &request.product_id,
            arrangement_type,
            request.effective_date,
            request.maturity_date,
            terms,
        )
        .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        self.arrangement_repo
            .save(&arrangement)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(self.arrangement_to_response(&arrangement))
    }

    pub async fn find_arrangement(
        &self,
        id: &str,
    ) -> Result<ArrangementResponse, ArrangementError> {
        let arr_id = ArrangementId::parse(id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let arrangement = self
            .arrangement_repo
            .find_by_id(&arr_id)
            .await
            .map_err(ArrangementError::RepositoryError)?
            .ok_or(ArrangementError::ArrangementNotFound)?;

        Ok(self.arrangement_to_response(&arrangement))
    }

    pub async fn activate_arrangement(
        &self,
        id: &str,
    ) -> Result<ArrangementResponse, ArrangementError> {
        let arr_id = ArrangementId::parse(id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let mut arrangement = self
            .arrangement_repo
            .find_by_id(&arr_id)
            .await
            .map_err(ArrangementError::RepositoryError)?
            .ok_or(ArrangementError::ArrangementNotFound)?;

        arrangement
            .activate()
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        self.notification_service
            .notify_status_change(&arr_id, arrangement.customer_id(), "active")
            .await
            .ok();

        self.arrangement_repo
            .save(&arrangement)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(self.arrangement_to_response(&arrangement))
    }

    pub async fn suspend_arrangement(
        &self,
        id: &str,
    ) -> Result<ArrangementResponse, ArrangementError> {
        let arr_id = ArrangementId::parse(id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let mut arrangement = self
            .arrangement_repo
            .find_by_id(&arr_id)
            .await
            .map_err(ArrangementError::RepositoryError)?
            .ok_or(ArrangementError::ArrangementNotFound)?;

        arrangement
            .suspend()
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        self.notification_service
            .notify_status_change(&arr_id, arrangement.customer_id(), "suspended")
            .await
            .ok();

        self.arrangement_repo
            .save(&arrangement)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(self.arrangement_to_response(&arrangement))
    }

    pub async fn close_arrangement(
        &self,
        id: &str,
        outstanding_loan_balance: i64,
    ) -> Result<ArrangementResponse, ArrangementError> {
        let arr_id = ArrangementId::parse(id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let mut arrangement = self
            .arrangement_repo
            .find_by_id(&arr_id)
            .await
            .map_err(ArrangementError::RepositoryError)?
            .ok_or(ArrangementError::ArrangementNotFound)?;

        arrangement
            .close(outstanding_loan_balance)
            .map_err(|e| match e {
                banko_domain::shared::DomainError::ValidationError(msg)
                    if msg.contains("outstanding") =>
                {
                    ArrangementError::OutstandingBalance
                }
                other => ArrangementError::DomainError(other.to_string()),
            })?;

        self.notification_service
            .notify_status_change(&arr_id, arrangement.customer_id(), "closed")
            .await
            .ok();

        self.arrangement_repo
            .save(&arrangement)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(self.arrangement_to_response(&arrangement))
    }

    pub async fn list_customer_arrangements(
        &self,
        customer_id: &str,
    ) -> Result<Vec<ArrangementResponse>, ArrangementError> {
        let cust_id = CustomerId::parse(customer_id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let arrangements = self
            .arrangement_repo
            .find_by_customer_id(&cust_id)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(arrangements
            .into_iter()
            .map(|a| self.arrangement_to_response(&a))
            .collect())
    }

    pub async fn check_maturing_soon(
        &self,
        days: i64,
    ) -> Result<Vec<ArrangementResponse>, ArrangementError> {
        let arrangements = self
            .arrangement_repo
            .find_maturing_soon(days)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(arrangements
            .into_iter()
            .map(|a| self.arrangement_to_response(&a))
            .collect())
    }

    pub async fn trigger_renewal_checks(&self) -> Result<usize, ArrangementError> {
        let arrangements = self
            .arrangement_repo
            .find_active()
            .await
            .map_err(ArrangementError::RepositoryError)?;

        let mut renewal_count = 0;

        for arrangement in arrangements {
            if arrangement.check_renewal_trigger() {
                match self.renewal_engine.trigger_renewal_check(arrangement.id()).await {
                    Ok(true) => {
                        renewal_count += 1;

                        // Notify customer
                        if let Some(maturity) = arrangement.maturity_date() {
                            let days = (maturity - chrono::Utc::now()).num_days();
                            let _ = self
                                .notification_service
                                .notify_maturity_upcoming(
                                    arrangement.id(),
                                    arrangement.customer_id(),
                                    days,
                                )
                                .await;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(renewal_count)
    }

    // --- Bundle Operations ---

    pub async fn create_bundle(
        &self,
        request: CreateBundleRequest,
    ) -> Result<ArrangementBundleResponse, ArrangementError> {
        let arrangement_ids: Result<Vec<ArrangementId>, _> = request
            .arrangement_ids
            .iter()
            .map(|id| ArrangementId::parse(id))
            .collect();

        let arrangement_ids = arrangement_ids
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let bundle =
            ArrangementBundle::new(&request.name, arrangement_ids, request.discount_pct)
                .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        self.bundle_repo
            .save(&bundle)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(self.bundle_to_response(&bundle))
    }

    pub async fn find_bundle(
        &self,
        id: &str,
    ) -> Result<ArrangementBundleResponse, ArrangementError> {
        let bundle_id = ArrangementBundleId::parse(id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let bundle = self
            .bundle_repo
            .find_by_id(&bundle_id)
            .await
            .map_err(ArrangementError::RepositoryError)?
            .ok_or(ArrangementError::BundleNotFound)?;

        Ok(self.bundle_to_response(&bundle))
    }

    pub async fn add_to_bundle(
        &self,
        bundle_id: &str,
        arrangement_id: &str,
    ) -> Result<ArrangementBundleResponse, ArrangementError> {
        let bundle_id_parsed = ArrangementBundleId::parse(bundle_id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;
        let arr_id = ArrangementId::parse(arrangement_id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        let mut bundle = self
            .bundle_repo
            .find_by_id(&bundle_id_parsed)
            .await
            .map_err(ArrangementError::RepositoryError)?
            .ok_or(ArrangementError::BundleNotFound)?;

        bundle
            .add_arrangement(arr_id)
            .map_err(|e| ArrangementError::DomainError(e.to_string()))?;

        self.bundle_repo
            .save(&bundle)
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(self.bundle_to_response(&bundle))
    }

    pub async fn list_active_bundles(
        &self,
    ) -> Result<Vec<ArrangementBundleResponse>, ArrangementError> {
        let bundles = self
            .bundle_repo
            .find_active()
            .await
            .map_err(ArrangementError::RepositoryError)?;

        Ok(bundles
            .into_iter()
            .map(|b| self.bundle_to_response(&b))
            .collect())
    }

    // --- Helper Methods ---

    fn arrangement_to_response(&self, arrangement: &Arrangement) -> ArrangementResponse {
        ArrangementResponse {
            id: arrangement.id().to_string(),
            customer_id: arrangement.customer_id().to_string(),
            product_id: arrangement.product_id().to_string(),
            arrangement_type: arrangement.arrangement_type().to_string(),
            status: arrangement.status().to_string(),
            effective_date: arrangement.effective_date(),
            maturity_date: arrangement.maturity_date(),
            terms: ArrangementTermsDto {
                currency: arrangement.terms().currency().to_string(),
                interest_rate: arrangement.terms().interest_rate(),
                fee_schedule_id: arrangement.terms().fee_schedule_id().map(|s| s.to_string()),
                minimum_balance: arrangement.terms().minimum_balance(),
                overdraft_limit: arrangement.terms().overdraft_limit(),
                renewal_type: arrangement.terms().renewal_type().to_string(),
                notice_period_days: arrangement.terms().notice_period_days(),
            },
            linked_accounts: arrangement
                .linked_accounts()
                .iter()
                .map(|a| a.to_string())
                .collect(),
            created_at: arrangement.created_at(),
            updated_at: arrangement.updated_at(),
        }
    }

    fn bundle_to_response(&self, bundle: &ArrangementBundle) -> ArrangementBundleResponse {
        ArrangementBundleResponse {
            id: bundle.id().to_string(),
            name: bundle.name().to_string(),
            arrangement_ids: bundle
                .arrangements()
                .iter()
                .map(|a| a.to_string())
                .collect(),
            discount_pct: bundle.discount_pct(),
            is_active: bundle.is_active(),
            created_at: bundle.created_at(),
            updated_at: bundle.updated_at(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full service tests require mock implementations of all ports.
    // These tests demonstrate the expected API contracts.

    #[test]
    fn test_service_instantiation() {
        // Service can be created with Arc trait objects
        // (requires mock implementations)
    }
}
