use async_trait::async_trait;

use banko_domain::insurance::{
    BancassuranceProduct, BancassuranceProductId, InsuranceClaim, InsuranceClaimId,
    InsuranceCommission, InsuranceCommissionId, InsurancePolicy, InsurancePolicyId,
};
use banko_domain::shared::CustomerId;

/// Port for insurance policy persistence.
#[async_trait]
pub trait IInsurancePolicyRepository: Send + Sync {
    async fn save(&self, policy: &InsurancePolicy) -> Result<(), String>;
    async fn find_by_id(&self, id: &InsurancePolicyId) -> Result<Option<InsurancePolicy>, String>;
    async fn find_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<InsurancePolicy>, String>;
    async fn delete(&self, id: &InsurancePolicyId) -> Result<(), String>;
}

/// Port for insurance claim persistence.
#[async_trait]
pub trait IInsuranceClaimRepository: Send + Sync {
    async fn save(&self, claim: &InsuranceClaim) -> Result<(), String>;
    async fn find_by_id(&self, id: &InsuranceClaimId) -> Result<Option<InsuranceClaim>, String>;
    async fn find_by_policy(
        &self,
        policy_id: &InsurancePolicyId,
    ) -> Result<Vec<InsuranceClaim>, String>;
    async fn delete(&self, id: &InsuranceClaimId) -> Result<(), String>;
}

/// Port for bancassurance product persistence.
#[async_trait]
pub trait IBancassuranceProductRepository: Send + Sync {
    async fn save(&self, product: &BancassuranceProduct) -> Result<(), String>;
    async fn find_by_id(
        &self,
        id: &BancassuranceProductId,
    ) -> Result<Option<BancassuranceProduct>, String>;
    async fn find_all(&self) -> Result<Vec<BancassuranceProduct>, String>;
    async fn delete(&self, id: &BancassuranceProductId) -> Result<(), String>;
}

/// Port for insurance commission persistence.
#[async_trait]
pub trait IInsuranceCommissionRepository: Send + Sync {
    async fn save(&self, commission: &InsuranceCommission) -> Result<(), String>;
    async fn find_by_id(
        &self,
        id: &InsuranceCommissionId,
    ) -> Result<Option<InsuranceCommission>, String>;
    async fn find_by_policy(
        &self,
        policy_id: &InsurancePolicyId,
    ) -> Result<Vec<InsuranceCommission>, String>;
    async fn delete(&sel