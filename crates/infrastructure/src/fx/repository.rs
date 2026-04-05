use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::fx::{IExchangeRateRepository, IFxRepository};
use banko_domain::fx::*;

// ============================================================
// PostgreSQL FX Operation Repository
// ============================================================

pub struct PgFxRepository {
    pool: PgPool,
}

impl PgFxRepository {
    pub fn new(pool: PgPool) -> Self {
        PgFxRepository { pool }
    }
}

#[async_trait]
impl IFxRepository for PgFxRepository {
    async fn save(&self, op: &FxOperation) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO fx.operations (
                id, account_id, operation_type, source_currency, target_currency,
                source_amount, target_amount, rate, status, reference,
                rejection_reason, created_at, confirmed_at, settled_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                rejection_reason = EXCLUDED.rejection_reason,
                confirmed_at = EXCLUDED.confirmed_at,
                settled_at = EXCLUDED.settled_at
            "#,
        )
        .bind(op.operation_id().as_uuid())
        .bind(op.account_id())
        .bind(op.operation_type().as_str())
        .bind(op.source_currency())
        .bind(op.target_currency())
        .bind(op.source_amount())
        .bind(op.target_amount())
        .bind(op.rate())
        .bind(op.status().as_str())
        .bind(op.reference())
        .bind(op.rejection_reason())
        .bind(op.created_at())
        .bind(op.confirmed_at())
        .bind(op.settled_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save FX operation: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &FxOperationId) -> Result<Option<FxOperation>, String> {
        let row = sqlx::query_as::<_, FxOperationRow>(
            r#"
            SELECT id, account_id, operation_type, source_currency, target_currency,
                   source_amount, target_amount, rate, status, reference,
                   rejection_reason, created_at, confirmed_at, settled_at
            FROM fx.operations WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find FX operation: {e}"))?;

        Ok(row.map(|r| r.into_domain()))
    }

    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<FxOperation>, String> {
        let rows = sqlx::query_as::<_, FxOperationRow>(
            r#"
            SELECT id, account_id, operation_type, source_currency, target_currency,
                   source_amount, target_amount, rate, status, reference,
                   rejection_reason, created_at, confirmed_at, settled_at
            FROM fx.operations WHERE account_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find FX operations by account: {e}"))?;

        Ok(rows.into_iter().map(|r| r.into_domain()).collect())
    }

    async fn find_all(
        &self,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FxOperation>, String> {
        let rows = sqlx::query_as::<_, FxOperationRow>(
            r#"
            SELECT id, account_id, operation_type, source_currency, target_currency,
                   source_amount, target_amount, rate, status, reference,
                   rejection_reason, created_at, confirmed_at, settled_at
            FROM fx.operations
            WHERE ($1::text IS NULL OR status = $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list FX operations: {e}"))?;

        Ok(rows.into_iter().map(|r| r.into_domain()).collect())
    }

    async fn count_all(&self, status: Option<&str>) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM fx.operations
            WHERE ($1::text IS NULL OR status = $1)
            "#,
        )
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to count FX operations: {e}"))?;

        Ok(row.0)
    }

    async fn get_daily_total(
        &self,
        account_id: Uuid,
        currency: &str,
        date: NaiveDate,
    ) -> Result<i64, String> {
        let row: (Option<i64>,) = sqlx::query_as(
            r#"
            SELECT COALESCE(SUM(source_amount), 0)
            FROM fx.operations
            WHERE account_id = $1
              AND source_currency = $2
              AND DATE(created_at) = $3
              AND status NOT IN ('Rejected', 'Cancelled')
            "#,
        )
        .bind(account_id)
        .bind(currency)
        .bind(date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to get daily total: {e}"))?;

        Ok(row.0.unwrap_or(0))
    }
}

// --- Row mapping helper ---

#[derive(sqlx::FromRow)]
struct FxOperationRow {
    id: Uuid,
    account_id: Uuid,
    operation_type: String,
    source_currency: String,
    target_currency: String,
    source_amount: i64,
    target_amount: i64,
    rate: f64,
    status: String,
    reference: String,
    rejection_reason: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    settled_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FxOperationRow {
    fn into_domain(self) -> FxOperation {
        FxOperation::from_raw(
            FxOperationId::from_uuid(self.id),
            self.account_id,
            FxOperationType::from_str_type(&self.operation_type).unwrap_or(FxOperationType::Spot),
            self.source_currency,
            self.target_currency,
            self.source_amount,
            self.target_amount,
            self.rate,
            FxStatus::from_str_type(&self.status).unwrap_or(FxStatus::Draft),
            self.reference,
            self.rejection_reason,
            self.created_at,
            self.confirmed_at,
            self.settled_at,
        )
    }
}

// ============================================================
// PostgreSQL Exchange Rate Repository
// ============================================================

pub struct PgExchangeRateRepository {
    pool: PgPool,
}

impl PgExchangeRateRepository {
    pub fn new(pool: PgPool) -> Self {
        PgExchangeRateRepository { pool }
    }
}

#[async_trait]
impl IExchangeRateRepository for PgExchangeRateRepository {
    async fn save(&self, rate: &ExchangeRate) -> Result<(), String> {
        // Mark previous rate for this pair as expired
        sqlx::query(
            r#"
            UPDATE fx.exchange_rates
            SET valid_to = NOW()
            WHERE source_currency = $1 AND target_currency = $2 AND valid_to IS NULL
            "#,
        )
        .bind(rate.source_currency())
        .bind(rate.target_currency())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to expire old rate: {e}"))?;

        // Insert new rate
        sqlx::query(
            r#"
            INSERT INTO fx.exchange_rates (source_currency, target_currency, rate, valid_from, valid_to, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            "#,
        )
        .bind(rate.source_currency())
        .bind(rate.target_currency())
        .bind(rate.rate())
        .bind(rate.valid_from())
        .bind(rate.valid_to())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save exchange rate: {e}"))?;

        Ok(())
    }

    async fn find_current(
        &self,
        source: &str,
        target: &str,
    ) -> Result<Option<ExchangeRate>, String> {
        let row = sqlx::query_as::<_, ExchangeRateRow>(
            r#"
            SELECT source_currency, target_currency, rate, valid_from, valid_to
            FROM fx.exchange_rates
            WHERE source_currency = $1 AND target_currency = $2 AND valid_to IS NULL
            ORDER BY valid_from DESC
            LIMIT 1
            "#,
        )
        .bind(source)
        .bind(target)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find current rate: {e}"))?;

        Ok(row.map(|r| r.into_domain()))
    }

    async fn find_all_current(&self) -> Result<Vec<ExchangeRate>, String> {
        let rows = sqlx::query_as::<_, ExchangeRateRow>(
            r#"
            SELECT source_currency, target_currency, rate, valid_from, valid_to
            FROM fx.exchange_rates
            WHERE valid_to IS NULL
            ORDER BY source_currency, target_currency
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list current rates: {e}"))?;

        Ok(rows.into_iter().map(|r| r.into_domain()).collect())
    }

    async fn find_history(
        &self,
        source: &str,
        target: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<ExchangeRate>, String> {
        let rows = sqlx::query_as::<_, ExchangeRateRow>(
            r#"
            SELECT source_currency, target_currency, rate, valid_from, valid_to
            FROM fx.exchange_rates
            WHERE source_currency = $1 AND target_currency = $2
              AND DATE(valid_from) >= $3 AND DATE(valid_from) <= $4
            ORDER BY valid_from DESC
            "#,
        )
        .bind(source)
        .bind(target)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find rate history: {e}"))?;

        Ok(rows.into_iter().map(|r| r.into_domain()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ExchangeRateRow {
    source_currency: String,
    target_currency: String,
    rate: f64,
    valid_from: chrono::DateTime<chrono::Utc>,
    valid_to: Option<chrono::DateTime<chrono::Utc>>,
}

impl ExchangeRateRow {
    fn into_domain(self) -> ExchangeRate {
        ExchangeRate::from_raw(
            self.source_currency,
            self.target_currency,
            self.rate,
            self.valid_from,
            self.valid_to,
        )
    }
}
