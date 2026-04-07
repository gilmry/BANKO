use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::arrangement::ports::{IArrangementRepository, IArrangementBundleRepository};
use banko_domain::arrangement::{
    Arrangement, ArrangementId, ArrangementStatus, ArrangementType, ArrangementBundle,
    ArrangementBundleId, ArrangementTerms, RenewalType,
};
use banko_domain::shared::{CustomerId, AccountId};

// --- PostgreSQL Arrangement Repository ---

pub struct PgArrangementRepository {
    pool: PgPool,
}

impl PgArrangementRepository {
    pub fn new(pool: PgPool) -> Self {
        PgArrangementRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ArrangementRow {
    id: Uuid,
    customer_id: Uuid,
    product_id: String,
    arrangement_type: String,
    status: String,
    effective_date: DateTime<Utc>,
    maturity_date: Option<DateTime<Utc>>,
    interest_rate: Option<f64>,
    fee_schedule_id: Option<String>,
    currency: String,
    minimum_balance: Option<i64>,
    overdraft_limit: Option<i64>,
    renewal_type: String,
    notice_period_days: Option<i32>,
    linked_accounts: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ArrangementRow {
    fn into_domain(self) -> Result<Arrangement, String> {
        let id = ArrangementId::from_uuid(self.id);
        let customer_id = CustomerId::from_uuid(self.customer_id);
        let arrangement_type = ArrangementType::from_str(&self.arrangement_type)
            .map_err(|e| e.to_string())?;
        let status = ArrangementStatus::from_str(&self.status)
            .map_err(|e| e.to_string())?;
        let renewal_type = RenewalType::from_str(&self.renewal_type)
            .map_err(|e| e.to_string())?;

        let linked_accounts: Result<Vec<AccountId>, String> = self
            .linked_accounts
            .into_iter()
            .map(|a| {
                AccountId::parse(&a).map_err(|e| e.to_string())
            })
            .collect();
        let linked_accounts = linked_accounts?;

        let terms = ArrangementTerms::new(
            &self.currency,
            self.interest_rate,
            self.fee_schedule_id,
            self.minimum_balance,
            self.overdraft_limit,
            renewal_type,
            self.notice_period_days.map(|n| n as u32),
        )
        .map_err(|e| e.to_string())?;

        Ok(Arrangement::reconstitute(
            id,
            customer_id,
            self.product_id,
            arrangement_type,
            status,
            self.effective_date,
            self.maturity_date,
            terms,
            linked_accounts,
            vec![], // Events loaded separately
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IArrangementRepository for PgArrangementRepository {
    async fn save(&self, arrangement: &Arrangement) -> Result<(), String> {
        let linked_accounts_strs: Vec<String> = arrangement
            .linked_accounts()
            .iter()
            .map(|a| a.to_string())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO arrangements (id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (id) DO UPDATE SET
                status = $5,
                maturity_date = $7,
                linked_accounts = $15,
                updated_at = $17
            "#
        )
        .bind(arrangement.id().as_uuid())
        .bind(arrangement.customer_id().as_uuid())
        .bind(arrangement.product_id())
        .bind(arrangement.arrangement_type().to_string())
        .bind(arrangement.status().to_string())
        .bind(arrangement.effective_date())
        .bind(arrangement.maturity_date())
        .bind(arrangement.terms().interest_rate())
        .bind(arrangement.terms().fee_schedule_id())
        .bind(arrangement.terms().currency())
        .bind(arrangement.terms().minimum_balance())
        .bind(arrangement.terms().overdraft_limit())
        .bind(arrangement.terms().renewal_type().to_string())
        .bind(arrangement.terms().notice_period_days().map(|n| n as i32))
        .bind(linked_accounts_strs)
        .bind(arrangement.created_at())
        .bind(arrangement.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Save events
        for event in arrangement.events() {
            sqlx::query(
                r#"
                INSERT INTO arrangement_events (id, arrangement_id, event_type, event_data_json, occurred_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT DO NOTHING
                "#
            )
            .bind(event.id().as_uuid())
            .bind(event.arrangement_id().as_uuid())
            .bind(event.event_type().to_string())
            .bind(event.event_data_json())
            .bind(event.occurred_at())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn find_by_id(&self, id: &ArrangementId) -> Result<Option<Arrangement>, String> {
        let row: Option<ArrangementRow> = sqlx::query_as(
            "SELECT id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at FROM arrangements WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer_id(&self, customer_id: &CustomerId) -> Result<Vec<Arrangement>, String> {
        let rows: Vec<ArrangementRow> = sqlx::query_as(
            "SELECT id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at FROM arrangements WHERE customer_id = $1"
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_status(&self, status: ArrangementStatus) -> Result<Vec<Arrangement>, String> {
        let rows: Vec<ArrangementRow> = sqlx::query_as(
            "SELECT id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at FROM arrangements WHERE status = $1"
        )
        .bind(status.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_type(&self, arrangement_type: ArrangementType) -> Result<Vec<Arrangement>, String> {
        let rows: Vec<ArrangementRow> = sqlx::query_as(
            "SELECT id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at FROM arrangements WHERE arrangement_type = $1"
        )
        .bind(arrangement_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_active(&self) -> Result<Vec<Arrangement>, String> {
        let rows: Vec<ArrangementRow> = sqlx::query_as(
            "SELECT id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at FROM arrangements WHERE status = $1"
        )
        .bind("active")
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_maturing_soon(&self, days: i64) -> Result<Vec<Arrangement>, String> {
        let future_date = Utc::now() + Duration::days(days);
        let rows: Vec<ArrangementRow> = sqlx::query_as(
            "SELECT id, customer_id, product_id, arrangement_type, status, effective_date, maturity_date, interest_rate, fee_schedule_id, currency, minimum_balance, overdraft_limit, renewal_type, notice_period_days, linked_accounts, created_at, updated_at FROM arrangements WHERE status = $1 AND maturity_date IS NOT NULL AND maturity_date <= $2"
        )
        .bind("active")
        .bind(future_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &ArrangementId) -> Result<(), String> {
        sqlx::query("DELETE FROM arrangements WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- PostgreSQL Arrangement Bundle Repository ---

pub struct PgArrangementBundleRepository {
    pool: PgPool,
}

impl PgArrangementBundleRepository {
    pub fn new(pool: PgPool) -> Self {
        PgArrangementBundleRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ArrangementBundleRow {
    id: Uuid,
    name: String,
    arrangement_ids: Vec<String>,
    discount_pct: Option<f64>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ArrangementBundleRow {
    fn into_domain(self) -> Result<ArrangementBundle, String> {
        let id = ArrangementBundleId::from_uuid(self.id);
        let arrangements: Result<Vec<ArrangementId>, String> = self
            .arrangement_ids
            .into_iter()
            .map(|a| ArrangementId::parse(&a).map_err(|e| e.to_string()))
            .collect();
        let arrangements = arrangements?;

        Ok(ArrangementBundle::reconstitute(
            id,
            self.name,
            arrangements,
            self.discount_pct,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IArrangementBundleRepository for PgArrangementBundleRepository {
    async fn save(&self, bundle: &ArrangementBundle) -> Result<(), String> {
        let arrangement_ids_strs: Vec<String> = bundle
            .arrangements()
            .iter()
            .map(|a| a.to_string())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO arrangement_bundles (id, name, arrangement_ids, discount_pct, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                arrangement_ids = $3,
                is_active = $5,
                updated_at = $7
            "#
        )
        .bind(bundle.id().as_uuid())
        .bind(bundle.name())
        .bind(arrangement_ids_strs)
        .bind(bundle.discount_pct())
        .bind(bundle.is_active())
        .bind(bundle.created_at())
        .bind(bundle.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ArrangementBundleId) -> Result<Option<ArrangementBundle>, String> {
        let row: Option<ArrangementBundleRow> = sqlx::query_as(
            "SELECT id, name, arrangement_ids, discount_pct, is_active, created_at, updated_at FROM arrangement_bundles WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_active(&self) -> Result<Vec<ArrangementBundle>, String> {
        let rows: Vec<ArrangementBundleRow> = sqlx::query_as(
            "SELECT id, name, arrangement_ids, discount_pct, is_active, created_at, updated_at FROM arrangement_bundles WHERE is_active = true"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_all(&self) -> Result<Vec<ArrangementBundle>, String> {
        let rows: Vec<ArrangementBundleRow> = sqlx::query_as(
            "SELECT id, name, arrangement_ids, discount_pct, is_active, created_at, updated_at FROM arrangement_bundles"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &ArrangementBundleId) -> Result<(), String> {
        sqlx::query("DELETE FROM arrangement_bundles WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
