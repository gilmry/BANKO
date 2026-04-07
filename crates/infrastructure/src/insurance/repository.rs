use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::insurance::{
    IBancassuranceProductRepository, IInsuranceClaimRepository, IInsuranceCommissionRepository,
    IInsurancePolicyRepository,
};
use banko_domain::insurance::{
    BancassuranceProduct, BancassuranceProductId, ClaimStatus, CommissionStatus, InsuranceClaim,
    InsuranceClaimId, InsuranceCommission, InsuranceCommissionId, InsurancePolicy,
    InsurancePolicyId, LinkedProductType, PolicyStatus, PolicyType, PremiumFrequency,
};
use banko_domain::shared::{Currency, CustomerId, Money};

// --- InsurancePolicyRepository ---

pub struct PgInsurancePolicyRepository {
    pool: PgPool,
}

impl PgInsurancePolicyRepository {
    pub fn new(pool: PgPool) -> Self {
        PgInsurancePolicyRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InsurancePolicyRow {
    id: Uuid,
    policy_type: String,
    customer_id: Uuid,
    provider_name: String,
    policy_number: String,
    premium_amount: i64,
    currency: String,
    premium_frequency: String,
    coverage_amount: i64,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    beneficiaries: Vec<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InsurancePolicyRow {
    fn into_domain(self) -> Result<InsurancePolicy, String> {
        let id = InsurancePolicyId::from_uuid(self.id);
        let policy_type =
            PolicyType::from_str(&self.policy_type).map_err(|e| e.to_string())?;
        let customer_id = CustomerId::from_uuid(self.customer_id);
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let premium = Money::from_cents(self.premium_amount, currency.clone());
        let coverage = Money::from_cents(self.coverage_amount, currency);
        let premium_frequency =
            PremiumFrequency::from_str(&self.premium_frequency).map_err(|e| e.to_string())?;
        let status = PolicyStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(InsurancePolicy::reconstitute(
            id,
            policy_type,
            customer_id,
            self.provider_name,
            self.policy_number,
            premium,
            premium_frequency,
            coverage,
            self.start_date,
            self.end_date,
            self.beneficiaries,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IInsurancePolicyRepository for PgInsurancePolicyRepository {
    async fn save(&self, policy: &InsurancePolicy) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO insurance.insurance_policies
            (id, policy_type, customer_id, provider_name, policy_number, premium_amount, currency,
             premium_frequency, coverage_amount, start_date, end_date, beneficiaries, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(policy.id().as_uuid())
        .bind(policy.policy_type().to_string())
        .bind(policy.customer_id().as_uuid())
        .bind(policy.provider_name())
        .bind(policy.policy_number())
        .bind(policy.premium_amount().as_cents())
        .bind(policy.premium_amount().currency().code())
        .bind(policy.premium_frequency().to_string())
        .bind(policy.coverage_amount().as_cents())
        .bind(policy.start_date())
        .bind(policy.end_date())
        .bind(policy.beneficiaries())
        .bind(policy.status().to_string())
        .bind(policy.created_at())
        .bind(policy.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save insurance policy: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &InsurancePolicyId) -> Result<Option<InsurancePolicy>, String> {
        let row = sqlx::query_as::<_, InsurancePolicyRow>(
            "SELECT * FROM insurance.insurance_policies WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find insurance policy: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<InsurancePolicy>, String> {
        let rows = sqlx::query_as::<_, InsurancePolicyRow>(
            "SELECT * FROM insurance.insurance_policies WHERE customer_id = $1",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find insurance policies: {e}"))?;

        let mut policies = Vec::new();
        for row in rows {
            policies.push(row.into_domain()?);
        }
        Ok(policies)
    }

    async fn delete(&self, id: &InsurancePolicyId) -> Result<(), String> {
        sqlx::query("DELETE FROM insurance.insurance_policies WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete insurance policy: {e}"))?;

        Ok(())
    }
}

// --- InsuranceClaimRepository ---

pub struct PgInsuranceClaimRepository {
    pool: PgPool,
}

impl PgInsuranceClaimRepository {
    pub fn new(pool: PgPool) -> Self {
        PgInsuranceClaimRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InsuranceClaimRow {
    id: Uuid,
    policy_id: Uuid,
    claim_date: DateTime<Utc>,
    claim_amount: i64,
    currency: String,
    description: String,
    documents: Vec<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InsuranceClaimRow {
    fn into_domain(self) -> Result<InsuranceClaim, String> {
        let id = InsuranceClaimId::from_uuid(self.id);
        let policy_id = InsurancePolicyId::from_uuid(self.policy_id);
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let amount = Money::from_cents(self.claim_amount, currency);
        let status = ClaimStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(InsuranceClaim::reconstitute(
            id,
            policy_id,
            self.claim_date,
            amount,
            self.description,
            self.documents,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IInsuranceClaimRepository for PgInsuranceClaimRepository {
    async fn save(&self, claim: &InsuranceClaim) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO insurance.insurance_claims
            (id, policy_id, claim_date, claim_amount, currency, description, documents, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(claim.id().as_uuid())
        .bind(claim.policy_id().as_uuid())
        .bind(claim.claim_date())
        .bind(claim.claim_amount().as_cents())
        .bind(claim.claim_amount().currency().code())
        .bind(claim.description())
        .bind(claim.documents())
        .bind(claim.status().to_string())
        .bind(claim.created_at())
        .bind(claim.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save insurance claim: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &InsuranceClaimId) -> Result<Option<InsuranceClaim>, String> {
        let row = sqlx::query_as::<_, InsuranceClaimRow>(
            "SELECT * FROM insurance.insurance_claims WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find insurance claim: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_policy(
        &self,
        policy_id: &InsurancePolicyId,
    ) -> Result<Vec<InsuranceClaim>, String> {
        let rows = sqlx::query_as::<_, InsuranceClaimRow>(
            "SELECT * FROM insurance.insurance_claims WHERE policy_id = $1",
        )
        .bind(policy_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find insurance claims: {e}"))?;

        let mut claims = Vec::new();
        for row in rows {
            claims.push(row.into_domain()?);
        }
        Ok(claims)
    }

    async fn delete(&self, id: &InsuranceClaimId) -> Result<(), String> {
        sqlx::query("DELETE FROM insurance.insurance_claims WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete insurance claim: {e}"))?;

        Ok(())
    }
}

// --- BancassuranceProductRepository ---

pub struct PgBancassuranceProductRepository {
    pool: PgPool,
}

impl PgBancassuranceProductRepository {
    pub fn new(pool: PgPool) -> Self {
        PgBancassuranceProductRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BancassuranceProductRow {
    id: Uuid,
    insurance_provider: String,
    product_name: String,
    product_type: String,
    commission_rate: f64,
    is_mandatory: bool,
    linked_product_types: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BancassuranceProductRow {
    fn into_domain(self) -> Result<BancassuranceProduct, String> {
        let id = BancassuranceProductId::from_uuid(self.id);
        let product_type =
            PolicyType::from_str(&self.product_type).map_err(|e| e.to_string())?;
        let linked_types: Result<Vec<_>, _> = self
            .linked_product_types
            .into_iter()
            .map(|s| LinkedProductType::from_str(&s))
            .collect::<Result<_, _>>()
            .map_err(|e| e.to_string());

        let linked_types = linked_types?;

        Ok(BancassuranceProduct::reconstitute(
            id,
            self.insurance_provider,
            self.product_name,
            product_type,
            self.commission_rate,
            self.is_mandatory,
            linked_types,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IBancassuranceProductRepository for PgBancassuranceProductRepository {
    async fn save(&self, product: &BancassuranceProduct) -> Result<(), String> {
        let linked_types: Vec<String> = product
            .linked_product_types()
            .iter()
            .map(|t| t.to_string())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO insurance.bancassurance_products
            (id, insurance_provider, product_name, product_type, commission_rate, is_mandatory, linked_product_types, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                product_name = EXCLUDED.product_name,
                commission_rate = EXCLUDED.commission_rate,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(product.id().as_uuid())
        .bind(product.insurance_provider())
        .bind(product.product_name())
        .bind(product.product_type().to_string())
        .bind(product.commission_rate())
        .bind(product.is_mandatory())
        .bind(linked_types)
        .bind(product.created_at())
        .bind(product.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save bancassurance product: {e}"))?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &BancassuranceProductId,
    ) -> Result<Option<BancassuranceProduct>, String> {
        let row = sqlx::query_as::<_, BancassuranceProductRow>(
            "SELECT * FROM insurance.bancassurance_products WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find bancassurance product: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<BancassuranceProduct>, String> {
        let rows = sqlx::query_as::<_, BancassuranceProductRow>(
            "SELECT * FROM insurance.bancassurance_products",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find bancassurance products: {e}"))?;

        let mut products = Vec::new();
        for row in rows {
            products.push(row.into_domain()?);
        }
        Ok(products)
    }

    async fn delete(&self, id: &BancassuranceProductId) -> Result<(), String> {
        sqlx::query("DELETE FROM insurance.bancassurance_products WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete bancassurance product: {e}"))?;

        Ok(())
    }
}

// --- InsuranceCommissionRepository ---

pub struct PgInsuranceCommissionRepository {
    pool: PgPool,
}

impl PgInsuranceCommissionRepository {
    pub fn new(pool: PgPool) -> Self {
        PgInsuranceCommissionRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InsuranceCommissionRow {
    id: Uuid,
    policy_id: Uuid,
    product_id: Uuid,
    amount: i64,
    currency: String,
    calculation_date: DateTime<Utc>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InsuranceCommissionRow {
    fn into_domain(self) -> Result<InsuranceCommission, String> {
        let id = InsuranceCommissionId::from_uuid(self.id);
        let policy_id = InsurancePolicyId::from_uuid(self.policy_id);
        let product_id = BancassuranceProductId::from_uuid(self.product_id);
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let amount = Money::from_cents(self.amount, currency);
        let status = CommissionStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(InsuranceCommission::reconstitute(
            id,
            policy_id,
            product_id,
            amount,
            self.calculation_date,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IInsuranceCommissionRepository for PgInsuranceCommissionRepository {
    async fn save(&self, commission: &InsuranceCommission) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO insurance.insurance_commissions
            (id, policy_id, product_id, amount, currency, calculation_date, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(commission.id().as_uuid())
        .bind(commission.policy_id().as_uuid())
        .bind(commission.product_id().as_uuid())
        .bind(commission.amount().as_cents())
        .bind(commission.amount().currency().code())
        .bind(commission.calculation_date())
        .bind(commission.status().to_string())
        .bind(commission.created_at())
        .bind(commission.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save insurance commission: {e}"))?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &InsuranceCommissionId,
    ) -> Result<Option<InsuranceCommission>, String> {
        let row = sqlx::query_as::<_, InsuranceCommissionRow>(
            "SELECT * FROM insurance.insurance_commissions WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find insurance commission: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_policy(
        &self,
        policy_id: &InsurancePolicyId,
    ) -> Result<Vec<InsuranceCommission>, String> {
        let rows = sqlx::query_as::<_, InsuranceCommissionRow>(
            "SELECT * FROM insurance.insurance_commissions WHERE policy_id = $1",
        )
        .bind(policy_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find insurance commissions: {e}"))?;

        let mut commissions = Vec::new();
        for row in rows {
            commissions.push(row.into_domain()?);
        }
        Ok(commissions)
    }

    async fn delete(&self, id: &InsuranceCommissionId) -> Result<(), String> {
        sqlx::query("DELETE FROM insurance.insurance_commissions WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete insurance commission: {e}"))?;

        Ok(())
    }
}
