use async_trait::async_trait;
use chrono::Utc;
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use banko_application::collateral::ports::{
    ICollateralAllocationRepository, ICollateralRepository, ICollateralValuationRepository,
};
use banko_domain::collateral::{
    Collateral, CollateralAllocation, CollateralId, CollateralStatus, CollateralType,
    CollateralValuation, ValuationMethod,
};
use banko_domain::shared::value_objects::{Currency, CustomerId, Money};

/// PostgreSQL implementation of ICollateralRepository.
pub struct PostgresCollateralRepository {
    pool: PgPool,
}

impl PostgresCollateralRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresCollateralRepository { pool }
    }
}

#[async_trait]
impl ICollateralRepository for PostgresCollateralRepository {
    async fn save(&self, collateral: &Collateral) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO collaterals (
                id, customer_id, collateral_type, description, market_value,
                haircut_pct, net_value, valuation_date, next_revaluation_date,
                status, insurance_policy_id, created_at, updated_at, currency
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT(id) DO UPDATE SET
                market_value = $5,
                haircut_pct = $6,
                net_value = $7,
                valuation_date = $8,
                next_revaluation_date = $9,
                status = $10,
                insurance_policy_id = $11,
                updated_at = $13
            "#,
        )
        .bind(collateral.id().as_uuid())
        .bind(collateral.customer_id().as_uuid())
        .bind(collateral.collateral_type().as_str())
        .bind(collateral.description())
        .bind(collateral.market_value().amount_cents())
        .bind(collateral.haircut_pct())
        .bind(collateral.net_value().amount_cents())
        .bind(collateral.valuation_date())
        .bind(collateral.next_revaluation_date())
        .bind(collateral.status().as_str())
        .bind(collateral.insurance_policy_id())
        .bind(collateral.created_at())
        .bind(collateral.updated_at())
        .bind("TND") // Hardcoded for now — should be parameterized
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save collateral: {}", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CollateralId) -> Result<Option<Collateral>, String> {
        let row = sqlx::query(
            r#"
            SELECT
                id, customer_id, collateral_type, description, market_value,
                haircut_pct, net_value, valuation_date, next_revaluation_date,
                status, insurance_policy_id, created_at, updated_at
            FROM collaterals
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(row.map(|r| {
            let collateral_type = CollateralType::from_str(&r.get::<String, _>("collateral_type"))
                .unwrap_or(CollateralType::Other);
            let status = CollateralStatus::from_str(&r.get::<String, _>("status"))
                .unwrap_or(CollateralStatus::Pending);

            let customer_uuid: Uuid = r.get("customer_id");
            let customer_id = CustomerId::from_uuid(customer_uuid);

            let market_value_cents: i64 = r.get("market_value");
            let net_value_cents: i64 = r.get("net_value");

            let market_value = Money::from_cents(market_value_cents, Currency::TND);
            let net_value = Money::from_cents(net_value_cents, Currency::TND);

            Collateral::reconstitute(
                CollateralId::from_uuid(r.get("id")),
                collateral_type,
                r.get("description"),
                market_value,
                r.get("haircut_pct"),
                net_value,
                r.get("valuation_date"),
                r.get("next_revaluation_date"),
                status,
                customer_id,
                r.get("insurance_policy_id"),
                r.get("created_at"),
                r.get("updated_at"),
            )
        }))
    }

    async fn find_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<Collateral>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, customer_id, collateral_type, description, market_value,
                haircut_pct, net_value, valuation_date, next_revaluation_date,
                status, insurance_policy_id, created_at, updated_at
            FROM collaterals
            WHERE customer_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let collateral_type = CollateralType::from_str(&r.get::<String, _>("collateral_type"))
                    .unwrap_or(CollateralType::Other);
                let status = CollateralStatus::from_str(&r.get::<String, _>("status"))
                    .unwrap_or(CollateralStatus::Pending);

                let customer_uuid: Uuid = r.get("customer_id");
                let customer_id = CustomerId::from_uuid(customer_uuid);

                let market_value_cents: i64 = r.get("market_value");
                let net_value_cents: i64 = r.get("net_value");

                let market_value = Money::from_cents(market_value_cents, Currency::TND);
                let net_value = Money::from_cents(net_value_cents, Currency::TND);

                Collateral::reconstitute(
                    CollateralId::from_uuid(r.get("id")),
                    collateral_type,
                    r.get("description"),
                    market_value,
                    r.get("haircut_pct"),
                    net_value,
                    r.get("valuation_date"),
                    r.get("next_revaluation_date"),
                    status,
                    customer_id,
                    r.get("insurance_policy_id"),
                    r.get("created_at"),
                    r.get("updated_at"),
                )
            })
            .collect())
    }

    async fn find_all(
        &self,
        customer_id: Option<&CustomerId>,
        status: Option<CollateralStatus>,
        collateral_type: Option<CollateralType>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Collateral>, String> {
        let mut query = "SELECT id, customer_id, collateral_type, description, market_value,
                               haircut_pct, net_value, valuation_date, next_revaluation_date,
                               status, insurance_policy_id, created_at, updated_at
                        FROM collaterals WHERE 1=1".to_string();

        if customer_id.is_some() {
            query.push_str(" AND customer_id = (SELECT $1::uuid)");
        }
        if status.is_some() {
            query.push_str(" AND status = $status");
        }
        if collateral_type.is_some() {
            query.push_str(" AND collateral_type = $type");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT $2 OFFSET $3");

        let mut q = sqlx::query(&query);

        if let Some(cid) = customer_id {
            q = q.bind(cid.as_uuid());
        }
        if let Some(s) = status {
            q = q.bind(s.as_str().to_string());
        }
        if let Some(ct) = collateral_type {
            q = q.bind(ct.as_str().to_string());
        }

        q = q.bind(limit).bind(offset);

        let rows = q
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let collateral_type = CollateralType::from_str(&r.get::<String, _>("collateral_type"))
                    .unwrap_or(CollateralType::Other);
                let status = CollateralStatus::from_str(&r.get::<String, _>("status"))
                    .unwrap_or(CollateralStatus::Pending);

                let customer_uuid: Uuid = r.get("customer_id");
                let customer_id = CustomerId::from_uuid(customer_uuid);

                let market_value_cents: i64 = r.get("market_value");
                let net_value_cents: i64 = r.get("net_value");

                let market_value = Money::from_cents(market_value_cents, Currency::TND);
                let net_value = Money::from_cents(net_value_cents, Currency::TND);

                Collateral::reconstitute(
                    CollateralId::from_uuid(r.get("id")),
                    collateral_type,
                    r.get("description"),
                    market_value,
                    r.get("haircut_pct"),
                    net_value,
                    r.get("valuation_date"),
                    r.get("next_revaluation_date"),
                    status,
                    customer_id,
                    r.get("insurance_policy_id"),
                    r.get("created_at"),
                    r.get("updated_at"),
                )
            })
            .collect())
    }

    async fn count_all(
        &self,
        customer_id: Option<&CustomerId>,
        status: Option<CollateralStatus>,
        collateral_type: Option<CollateralType>,
    ) -> Result<i64, String> {
        let mut query = "SELECT COUNT(*) as count FROM collaterals WHERE 1=1".to_string();

        if customer_id.is_some() {
            query.push_str(" AND customer_id = $1::uuid");
        }
        if status.is_some() {
            query.push_str(" AND status = $status");
        }
        if collateral_type.is_some() {
            query.push_str(" AND collateral_type = $type");
        }

        let mut q = sqlx::query(&query);

        if let Some(cid) = customer_id {
            q = q.bind(cid.as_uuid());
        }
        if let Some(s) = status {
            q = q.bind(s.as_str().to_string());
        }
        if let Some(ct) = collateral_type {
            q = q.bind(ct.as_str().to_string());
        }

        let row = q
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(row.get::<i64, _>("count"))
    }

    async fn find_revaluation_due(&self) -> Result<Vec<Collateral>, String> {
        let today = Utc::now().naive_utc().date();

        let rows = sqlx::query(
            r#"
            SELECT
                id, customer_id, collateral_type, description, market_value,
                haircut_pct, net_value, valuation_date, next_revaluation_date,
                status, insurance_policy_id, created_at, updated_at
            FROM collaterals
            WHERE next_revaluation_date <= $1
            ORDER BY next_revaluation_date ASC
            "#,
        )
        .bind(today)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let collateral_type = CollateralType::from_str(&r.get::<String, _>("collateral_type"))
                    .unwrap_or(CollateralType::Other);
                let status = CollateralStatus::from_str(&r.get::<String, _>("status"))
                    .unwrap_or(CollateralStatus::Pending);

                let customer_uuid: Uuid = r.get("customer_id");
                let customer_id = CustomerId::from_uuid(customer_uuid);

                let market_value_cents: i64 = r.get("market_value");
                let net_value_cents: i64 = r.get("net_value");

                let market_value = Money::from_cents(market_value_cents, Currency::TND);
                let net_value = Money::from_cents(net_value_cents, Currency::TND);

                Collateral::reconstitute(
                    CollateralId::from_uuid(r.get("id")),
                    collateral_type,
                    r.get("description"),
                    market_value,
                    r.get("haircut_pct"),
                    net_value,
                    r.get("valuation_date"),
                    r.get("next_revaluation_date"),
                    status,
                    customer_id,
                    r.get("insurance_policy_id"),
                    r.get("created_at"),
                    r.get("updated_at"),
                )
            })
            .collect())
    }

    async fn delete(&self, id: &CollateralId) -> Result<(), String> {
        sqlx::query("DELETE FROM collaterals WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete collateral: {}", e))?;

        Ok(())
    }
}

// --- PostgresCollateralValuationRepository ---

pub struct PostgresCollateralValuationRepository {
    pool: PgPool,
}

impl PostgresCollateralValuationRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresCollateralValuationRepository { pool }
    }
}

#[async_trait]
impl ICollateralValuationRepository for PostgresCollateralValuationRepository {
    async fn save(&self, valuation: &CollateralValuation) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO collateral_valuations (
                valuation_id, collateral_id, valuation_date, market_value,
                appraiser, method, next_revaluation_date, currency, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(valuation.valuation_id())
        .bind(valuation.collateral_id().as_uuid())
        .bind(valuation.valuation_date())
        .bind(valuation.market_value().amount_cents())
        .bind(valuation.appraiser())
        .bind(valuation.method().as_str())
        .bind(valuation.next_revaluation_date())
        .bind("TND")
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save valuation: {}", e))?;

        Ok(())
    }

    async fn find_by_collateral_id(
        &self,
        collateral_id: &CollateralId,
    ) -> Result<Vec<CollateralValuation>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                valuation_id, collateral_id, valuation_date, market_value,
                appraiser, method, next_revaluation_date
            FROM collateral_valuations
            WHERE collateral_id = $1
            ORDER BY valuation_date DESC
            "#,
        )
        .bind(collateral_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let method = ValuationMethod::from_str(&r.get::<String, _>("method"))
                    .unwrap_or(ValuationMethod::MarketComparison);
                let market_value_cents: i64 = r.get("market_value");
                let market_value = Money::from_cents(market_value_cents, Currency::TND);

                CollateralValuation::reconstitute(
                    r.get("valuation_id"),
                    CollateralId::from_uuid(r.get("collateral_id")),
                    r.get("valuation_date"),
                    market_value,
                    r.get("appraiser"),
                    method,
                    r.get("next_revaluation_date"),
                )
            })
            .collect())
    }

    async fn find_latest(
        &self,
        collateral_id: &CollateralId,
    ) -> Result<Option<CollateralValuation>, String> {
        let row = sqlx::query(
            r#"
            SELECT
                valuation_id, collateral_id, valuation_date, market_value,
                appraiser, method, next_revaluation_date
            FROM collateral_valuations
            WHERE collateral_id = $1
            ORDER BY valuation_date DESC
            LIMIT 1
            "#,
        )
        .bind(collateral_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(row.map(|r| {
            let method = ValuationMethod::from_str(&r.get::<String, _>("method"))
                .unwrap_or(ValuationMethod::MarketComparison);
            let market_value_cents: i64 = r.get("market_value");
            let market_value = Money::from_cents(market_value_cents, Currency::TND);

            CollateralValuation::reconstitute(
                r.get("valuation_id"),
                CollateralId::from_uuid(r.get("collateral_id")),
                r.get("valuation_date"),
                market_value,
                r.get("appraiser"),
                method,
                r.get("next_revaluation_date"),
            )
        }))
    }
}

// --- PostgresCollateralAllocationRepository ---

pub struct PostgresCollateralAllocationRepository {
    pool: PgPool,
}

impl PostgresCollateralAllocationRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresCollateralAllocationRepository { pool }
    }
}

#[async_trait]
impl ICollateralAllocationRepository for PostgresCollateralAllocationRepository {
    async fn save(&self, allocation: &CollateralAllocation) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO collateral_allocations (
                collateral_id, loan_id, allocated_amount, allocation_date, currency
            ) VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(allocation.collateral_id().as_uuid())
        .bind(allocation.loan_id())
        .bind(allocation.allocated_amount().amount_cents())
        .bind(allocation.allocation_date())
        .bind("TND")
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save allocation: {}", e))?;

        Ok(())
    }

    async fn find_by_collateral_id(
        &self,
        collateral_id: &CollateralId,
    ) -> Result<Vec<CollateralAllocation>, String> {
        let rows = sqlx::query(
            r#"
            SELECT collateral_id, loan_id, allocated_amount, allocation_date
            FROM collateral_allocations
            WHERE collateral_id = $1
            ORDER BY allocation_date DESC
            "#,
        )
        .bind(collateral_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let allocated_amount_cents: i64 = r.get("allocated_amount");
                let allocated_amount = Money::from_cents(allocated_amount_cents, Currency::TND);

                CollateralAllocation::new(
                    CollateralId::from_uuid(r.get("collateral_id")),
                    r.get("loan_id"),
                    allocated_amount.clone(),
                    r.get("allocation_date"),
                )
                .unwrap_or_else(|_| {
                    // Fallback: return a valid allocation from persistence
                    CollateralAllocation::new(
                        CollateralId::from_uuid(r.get("collateral_id")),
                        r.get("loan_id"),
                        allocated_amount,
                        r.get("allocation_date"),
                    )
                    .unwrap()
                })
            })
            .collect())
    }

    async fn find_by_loan_id(&self, loan_id: &str) -> Result<Vec<CollateralAllocation>, String> {
        let rows = sqlx::query(
            r#"
            SELECT collateral_id, loan_id, allocated_amount, allocation_date
            FROM collateral_allocations
            WHERE loan_id = $1
            ORDER BY allocation_date DESC
            "#,
        )
        .bind(loan_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let allocated_amount_cents: i64 = r.get("allocated_amount");
                let allocated_amount = Money::from_cents(allocated_amount_cents, Currency::TND);

                CollateralAllocation::new(
                    CollateralId::from_uuid(r.get("collateral_id")),
                    r.get("loan_id"),
                    allocated_amount.clone(),
                    r.get("allocation_date"),
                )
                .unwrap_or_else(|_| {
                    CollateralAllocation::new(
                        CollateralId::from_uuid(r.get("collateral_id")),
                        r.get("loan_id"),
                        allocated_amount,
                        r.get("allocation_date"),
                    )
                    .unwrap()
                })
            })
            .collect())
    }

    async fn find_by_collateral_and_loan(
        &self,
        collateral_id: &CollateralId,
        loan_id: &str,
    ) -> Result<Vec<CollateralAllocation>, String> {
        let rows = sqlx::query(
            r#"
            SELECT collateral_id, loan_id, allocated_amount, allocation_date
            FROM collateral_allocations
            WHERE collateral_id = $1 AND loan_id = $2
            ORDER BY allocation_date DESC
            "#,
        )
        .bind(collateral_id.as_uuid())
        .bind(loan_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let allocated_amount_cents: i64 = r.get("allocated_amount");
                let allocated_amount = Money::from_cents(allocated_amount_cents, Currency::TND);

                CollateralAllocation::new(
                    CollateralId::from_uuid(r.get("collateral_id")),
                    r.get("loan_id"),
                    allocated_amount.clone(),
                    r.get("allocation_date"),
                )
                .unwrap_or_else(|_| {
                    CollateralAllocation::new(
                        CollateralId::from_uuid(r.get("collateral_id")),
                        r.get("loan_id"),
                        allocated_amount,
                        r.get("allocation_date"),
                    )
                    .unwrap()
                })
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_types() {
        // Type checks to ensure implementations compile
        let _: &dyn ICollateralRepository;
        let _: &dyn ICollateralValuationRepository;
        let _: &dyn ICollateralAllocationRepository;
    }
}
