use std::sync::Arc;

use chrono::Utc;

use banko_domain::insurance::{
    BancassuranceProduct, BancassuranceProductId, ClaimStatus, CommissionStatus, InsuranceClaim,
    InsuranceClaimId, InsuranceCommission, InsuranceCommissionId, InsurancePolicy,
    InsurancePolicyId, LinkedProductType, PolicyStatus, PolicyType, PremiumFrequency,
};
use banko_domain::shared::{Currency, CustomerId, Money};

use super::dto::*;
use super::errors::InsuranceError;
use super::ports::*;

pub struct InsuranceService {
    policy_repo: Arc<dyn IInsurancePolicyRepository>,
    claim_repo: Arc<dyn IInsuranceClaimRepository>,
    product_repo: Arc<dyn IBancassuranceProductRepository>,
    commission_repo: Arc<dyn IInsuranceCommissionRepository>,
}

impl InsuranceService {
    pub fn new(
        policy_repo: Arc<dyn IInsurancePolicyRepository>,
        claim_repo: Arc<dyn IInsuranceClaimRepository>,
        product_repo: Arc<dyn IBancassuranceProductRepository>,
        commission_repo: Arc<dyn IInsuranceCommissionRepository>,
    ) -> Self {
        InsuranceService {
            policy_repo,
            claim_repo,
            product_repo,
            commission_repo,
        }
    }

    // --- Insurance Policy Operations ---

    pub async fn create_insurance_policy(
        &self,
        request: CreateInsurancePolicyRequest,
    ) -> Result<InsurancePolicyResponse, InsuranceError> {
        let policy_type = PolicyType::from_str(&request.policy_type)
            .map_err(|e| InsuranceError::InvalidPolicyConfiguration(e.to_string()))?;

        let customer_id = CustomerId::parse(&request.customer_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| InsuranceError::InvalidPolicyConfiguration(e.to_string()))?;

        let premium_frequency = PremiumFrequency::from_str(&request.premium_frequency)
            .map_err(|e| InsuranceError::InvalidPolicyConfiguration(e.to_string()))?;

        let premium = Money::from_f64(request.premium_amount, currency.clone())
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let coverage = Money::from_f64(request.coverage_amount, currency)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let policy = InsurancePolicy::new(
            policy_type,
            customer_id,
            &request.provider_name,
            &request.policy_number,
            premium,
            premium_frequency,
            coverage,
            request.start_date,
            request.end_date,
            request.beneficiaries,
        )
        .map_err(|e| InsuranceError::InvalidPolicyConfiguration(e.to_string()))?;

        self.policy_repo
            .save(&policy)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.policy_to_response(&policy))
    }

    pub async fn get_insurance_policy(
        &self,
        id: &str,
    ) -> Result<InsurancePolicyResponse, InsuranceError> {
        let policy_id = InsurancePolicyId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let policy = self
            .policy_repo
            .find_by_id(&policy_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::PolicyNotFound)?;

        if policy.is_expired() {
            return Err(InsuranceError::PolicyExpired);
        }

        Ok(self.policy_to_response(&policy))
    }

    pub async fn activate_policy(
        &self,
        id: &str,
    ) -> Result<InsurancePolicyResponse, InsuranceError> {
        let policy_id = InsurancePolicyId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let mut policy = self
            .policy_repo
            .find_by_id(&policy_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::PolicyNotFound)?;

        policy
            .activate()
            .map_err(|e| InsuranceError::InvalidPolicyTransition(e.to_string()))?;

        self.policy_repo
            .save(&policy)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.policy_to_response(&policy))
    }

    pub async fn list_policies_by_customer(
        &self,
        customer_id: &str,
    ) -> Result<Vec<InsurancePolicyResponse>, InsuranceError> {
        let cust_id = CustomerId::parse(customer_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let policies = self
            .policy_repo
            .find_by_customer(&cust_id)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(policies.iter().map(|p| self.policy_to_response(p)).collect())
    }

    // --- Insurance Claim Operations ---

    pub async fn create_insurance_claim(
        &self,
        request: CreateInsuranceClaimRequest,
    ) -> Result<InsuranceClaimResponse, InsuranceError> {
        let policy_id = InsurancePolicyId::parse(&request.policy_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| InsuranceError::InvalidClaimConfiguration(e.to_string()))?;

        let amount = Money::from_f64(request.claim_amount, currency)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let claim = InsuranceClaim::new(
            policy_id,
            request.claim_date,
            amount,
            &request.description,
            request.documents,
        )
        .map_err(|e| InsuranceError::InvalidClaimConfiguration(e.to_string()))?;

        self.claim_repo
            .save(&claim)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.claim_to_response(&claim))
    }

    pub async fn get_insurance_claim(
        &self,
        id: &str,
    ) -> Result<InsuranceClaimResponse, InsuranceError> {
        let claim_id = InsuranceClaimId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let claim = self
            .claim_repo
            .find_by_id(&claim_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::ClaimNotFound)?;

        Ok(self.claim_to_response(&claim))
    }

    pub async fn submit_claim_for_review(
        &self,
        id: &str,
    ) -> Result<InsuranceClaimResponse, InsuranceError> {
        let claim_id = InsuranceClaimId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let mut claim = self
            .claim_repo
            .find_by_id(&claim_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::ClaimNotFound)?;

        claim
            .submit_for_review()
            .map_err(|e| InsuranceError::InvalidClaimTransition(e.to_string()))?;

        self.claim_repo
            .save(&claim)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.claim_to_response(&claim))
    }

    pub async fn approve_claim(
        &self,
        id: &str,
    ) -> Result<InsuranceClaimResponse, InsuranceError> {
        let claim_id = InsuranceClaimId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let mut claim = self
            .claim_repo
            .find_by_id(&claim_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::ClaimNotFound)?;

        claim
            .approve()
            .map_err(|e| InsuranceError::InvalidClaimTransition(e.to_string()))?;

        self.claim_repo
            .save(&claim)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.claim_to_response(&claim))
    }

    pub async fn pay_claim(
        &self,
        id: &str,
    ) -> Result<InsuranceClaimResponse, InsuranceError> {
        let claim_id = InsuranceClaimId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let mut claim = self
            .claim_repo
            .find_by_id(&claim_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::ClaimNotFound)?;

        claim
            .pay()
            .map_err(|e| InsuranceError::InvalidClaimTransition(e.to_string()))?;

        self.claim_repo
            .save(&claim)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.claim_to_response(&claim))
    }

    pub async fn list_claims_by_policy(
        &self,
        policy_id: &str,
    ) -> Result<Vec<InsuranceClaimResponse>, InsuranceError> {
        let pol_id = InsurancePolicyId::parse(policy_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let claims = self
            .claim_repo
            .find_by_policy(&pol_id)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(claims.iter().map(|c| self.claim_to_response(c)).collect())
    }

    // --- Bancassurance Product Operations ---

    pub async fn create_bancassurance_product(
        &self,
        request: CreateBancassuranceProductRequest,
    ) -> Result<BancassuranceProductResponse, InsuranceError> {
        let product_type = PolicyType::from_str(&request.product_type)
            .map_err(|e| InsuranceError::InvalidProductConfiguration(e.to_string()))?;

        let linked_types: Result<Vec<_>, _> = request
            .linked_product_types
            .iter()
            .map(|s| LinkedProductType::from_str(s))
            .collect::<Result<_, _>>()
            .map_err(|e| InsuranceError::InvalidProductConfiguration(e.to_string()));

        let linked_types = linked_types?;

        let product = BancassuranceProduct::new(
            &request.insurance_provider,
            &request.product_name,
            product_type,
            request.commission_rate,
            request.is_mandatory,
            linked_types,
        )
        .map_err(|e| InsuranceError::InvalidProductConfiguration(e.to_string()))?;

        self.product_repo
            .save(&product)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.product_to_response(&product))
    }

    pub async fn get_bancassurance_product(
        &self,
        id: &str,
    ) -> Result<BancassuranceProductResponse, InsuranceError> {
        let product_id = BancassuranceProductId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let product = self
            .product_repo
            .find_by_id(&product_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::ProductNotFound)?;

        Ok(self.product_to_response(&product))
    }

    pub async fn list_all_products(
        &self,
    ) -> Result<Vec<BancassuranceProductResponse>, InsuranceError> {
        let products = self
            .product_repo
            .find_all()
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(products.iter().map(|p| self.product_to_response(p)).collect())
    }

    // --- Insurance Commission Operations ---

    pub async fn create_insurance_commission(
        &self,
        request: CreateInsuranceCommissionRequest,
    ) -> Result<InsuranceCommissionResponse, InsuranceError> {
        let policy_id = InsurancePolicyId::parse(&request.policy_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let product_id = BancassuranceProductId::parse(&request.product_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| InsuranceError::InvalidCommissionConfiguration(e.to_string()))?;

        let amount = Money::from_f64(request.amount, currency)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let commission = InsuranceCommission::new(policy_id, product_id, amount, request.calculation_date)
            .map_err(|e| InsuranceError::InvalidCommissionConfiguration(e.to_string()))?;

        self.commission_repo
            .save(&commission)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.commission_to_response(&commission))
    }

    pub async fn get_insurance_commission(
        &self,
        id: &str,
    ) -> Result<InsuranceCommissionResponse, InsuranceError> {
        let commission_id = InsuranceCommissionId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let commission = self
            .commission_repo
            .find_by_id(&commission_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::CommissionNotFound)?;

        Ok(self.commission_to_response(&commission))
    }

    pub async fn pay_commission(
        &self,
        id: &str,
    ) -> Result<InsuranceCommissionResponse, InsuranceError> {
        let commission_id = InsuranceCommissionId::parse(id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let mut commission = self
            .commission_repo
            .find_by_id(&commission_id)
            .await
            .map_err(InsuranceError::Internal)?
            .ok_or(InsuranceError::CommissionNotFound)?;

        commission
            .mark_paid()
            .map_err(|e| InsuranceError::InvalidCommissionConfiguration(e.to_string()))?;

        self.commission_repo
            .save(&commission)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(self.commission_to_response(&commission))
    }

    pub async fn list_commissions_by_policy(
        &self,
        policy_id: &str,
    ) -> Result<Vec<InsuranceCommissionResponse>, InsuranceError> {
        let pol_id = InsurancePolicyId::parse(policy_id)
            .map_err(|e| InsuranceError::DomainError(e.to_string()))?;

        let commissions = self
            .commission_repo
            .find_by_policy(&pol_id)
            .await
            .map_err(InsuranceError::Internal)?;

        Ok(commissions
            .iter()
            .map(|c| self.commission_to_response(c))
            .collect())
    }

    // --- Response Mappers ---

    fn policy_to_response(&self, policy: &InsurancePolicy) -> InsurancePolicyResponse {
        InsurancePolicyResponse {
            id: policy.id().to_string(),
            policy_type: policy.policy_type().to_string(),
            customer_id: policy.customer_id().to_string(),
            provider_name: policy.provider_name().to_string(),
            policy_number: policy.policy_number().to_string(),
            premium_amount: policy.premium_amount().as_f64(),
            currency: policy.premium_amount().currency().code().to_string(),
            premium_frequency: policy.premium_frequency().to_string(),
            coverage_amount: policy.coverage_amount().as_f64(),
            start_date: policy.start_date(),
            end_date: policy.end_date(),
            beneficiaries: policy.beneficiaries().to_vec(),
            status: policy.status().to_string(),
            is_expired: policy.is_expired(),
            created_at: policy.created_at(),
            updated_at: policy.updated_at(),
        }
    }

    fn claim_to_response(&self, claim: &InsuranceClaim) -> InsuranceClaimResponse {
        InsuranceClaimResponse {
            id: claim.id().to_string(),
            policy_id: claim.policy_id().to_string(),
            claim_date: claim.claim_date(),
            claim_amount: claim.claim_amount().as_f64(),
            currency: claim.claim_amount().currency().code().to_string(),
            description: claim.description().to_string(),
            documents: claim.documents().to_vec(),
            status: claim.status().to_string(),
            created_at: claim.created_at(),
            updated_at: claim.updated_at(),
        }
    }

    fn product_to_response(&self, product: &BancassuranceProduct) -> BancassuranceProductResponse {
        let linked_types: Vec<String> = product
            .linked_product_types()
            .iter()
            .map(|t| t.to_string())
            .collect();

        BancassuranceProductResponse {
            id: product.id().to_string(),
            insurance_provider: product.insurance_provider().to_string(),
            product_name: product.product_name().to_string(),
            product_type: product.product_type().to_string(),
            commission_rate: product.commission_rate(),
            is_mandatory: product.is_mandatory(),
            linked_product_types: linked_types,
            created_at: product.created_at(),
            updated_at: product.updated_at(),
        }
    }

    fn commission_to_response(&self, commission: &InsuranceCommission) -> InsuranceCommissionResponse {
        InsuranceCommissionResponse {
            id: commission.id().to_string(),
            policy_id: commission.policy_id().to_string(),
            product_id: commission.product_id().to_string(),
            amount: commission.amount().as_f64(),
            currency: commission.amount().currency().code().to_string(),
            calculation_date: commission.calculation_date(),
            status: commission.status().to_string(),
            created_at: commission.created_at(),
            updated_at: commission.updated_at(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPolicyRepository;
    #[async_trait::async_trait]
    impl IInsurancePolicyRepository for MockPolicyRepository {
        async fn save(&self, _policy: &InsurancePolicy) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &InsurancePolicyId) -> Result<Option<InsurancePolicy>, String> {
            Ok(None)
        }
        async fn find_by_customer(
            &self,
            _customer_id: &CustomerId,
        ) -> Result<Vec<InsurancePolicy>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &InsurancePolicyId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockClaimRepository;
    #[async_trait::async_trait]
    impl IInsuranceClaimRepository for MockClaimRepository {
        async fn save(&self, _claim: &InsuranceClaim) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &InsuranceClaimId) -> Result<Option<InsuranceClaim>, String> {
            Ok(None)
        }
        async fn find_by_policy(
            &self,
            _policy_id: &InsurancePolicyId,
        ) -> Result<Vec<InsuranceClaim>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &InsuranceClaimId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockProductRepository;
    #[async_trait::async_trait]
    impl IBancassuranceProductRepository for MockProductRepository {
        async fn save(&self, _product: &BancassuranceProduct) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(
            &self,
            _id: &BancassuranceProductId,
        ) -> Result<Option<BancassuranceProduct>, String> {
            Ok(None)
        }
        async fn find_all(&self) -> Result<Vec<BancassuranceProduct>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &BancassuranceProductId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockCommissionRepository;
    #[async_trait::async_trait]
    impl IInsuranceCommissionRepository for MockCommissionRepository {
        async fn save(&self, _commission: &InsuranceCommission) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(
            &self,
            _id: &InsuranceCommissionId,
        ) -> Result<Option<InsuranceCommission>, String> {
            Ok(None)
        }
        async fn find_by_policy(
            &self,
            _policy_id: &InsurancePolicyId,
        ) -> Result<Vec<InsuranceCommission>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &InsuranceCommissionId) -> Result<(), String> {
            Ok(())
        }
    }

    fn create_service() -> InsuranceService {
        InsuranceService::new(
            Arc::new(MockPolicyRepository),
            Arc::new(MockClaimRepository),
            Arc::new(MockProductRepository),
            Arc::new(MockCommissionRepository),
        )
    }

    #[test]
    fn test_service_creation() {
        let _service = create_service();
    }
}
