use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::prudential::{IBreachAlertRepository, IPrudentialRepository};
use banko_domain::prudential::*;

pub struct PgPrudentialRepository {
    pool: PgPool,
}

impl PgPrudentialRepository {
    pub fn new(pool: PgPool) -> Self {
        PgPrudentialRepository { pool }
    }
}

#[async_trait]
impl IPrudentialRepository for PgPrudentialRepository {
    async fn save(&self, ratio: &PrudentialRatio) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO prudential.ratios
               (id, institution_id, capital_tier1, capital_tier2, risk_weighted_assets,
                total_credits, total_deposits, solvency_ratio, tier1_ratio,
                credit_deposit_ratio, calculated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               ON CONFLICT (id) DO UPDATE SET
                capital_tier1 = EXCLUDED.capital_tier1,
                capital_tier2 = EXCLUDED.capital_tier2,
                risk_weighted_assets = EXCLUDED.risk_weighted_assets,
                total_credits = EXCLUDED.total_credits,
                total_deposits = EXCLUDED.total_deposits,
                solvency_ratio = EXCLUDED.solvency_ratio,
                tier1_ratio = EXCLUDED.tier1_ratio,
                credit_deposit_ratio = EXCLUDED.credit_deposit_ratio,
                calculated_at = EXCLUDED.calculated_at"#,
        )
        .bind(ratio.ratio_id().as_uuid())
        .bind(ratio.institution_id().as_uuid())
        .bind(ratio.capital_tier1())
        .bind(ratio.capital_tier2())
        .bind(ratio.risk_weighted_assets())
        .bind(ratio.total_credits())
        .bind(ratio.total_deposits())
        .bind(ratio.solvency_ratio())
        .bind(ratio.tier1_ratio())
        .bind(ratio.credit_deposit_ratio())
        .bind(ratio.calculated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &RatioId) -> Result<Option<PrudentialRatio>, String> {
        let row = sqlx::query_as::<_, PrudentialRatioRow>(
            "SELECT * FROM prudential.ratios WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(row_to_ratio(r, vec![]))),
            None => Ok(None),
        }
    }

    async fn find_by_institution(
        &self,
        institution_id: Uuid,
    ) -> Result<Option<PrudentialRatio>, String> {
        let row = sqlx::query_as::<_, PrudentialRatioRow>(
            "SELECT * FROM prudential.ratios WHERE institution_id = $1 ORDER BY calculated_at DESC LIMIT 1",
        )
        .bind(institution_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                let exposures = self.find_exposures(&RatioId::from_uuid(r.id)).await?;
                Ok(Some(row_to_ratio(r, exposures)))
            }
            None => Ok(None),
        }
    }

    async fn find_latest(&self, institution_id: Uuid) -> Result<Option<PrudentialRatio>, String> {
        self.find_by_institution(institution_id).await
    }

    async fn save_snapshot(&self, snapshot: &RatioSnapshot) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO prudential.ratio_snapshots
               (id, ratio_id, institution_id, snapshot_date, solvency_ratio,
                tier1_ratio, credit_deposit_ratio, breach_type)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        )
        .bind(snapshot.id())
        .bind(snapshot.ratio_id().as_uuid())
        .bind(snapshot.institution_id().as_uuid())
        .bind(snapshot.snapshot_date())
        .bind(snapshot.solvency_ratio())
        .bind(snapshot.tier1_ratio())
        .bind(snapshot.credit_deposit_ratio())
        .bind(snapshot.breach_type().map(|b| b.as_str().to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_snapshots(
        &self,
        institution_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<RatioSnapshot>, String> {
        let rows = sqlx::query_as::<_, RatioSnapshotRow>(
            "SELECT * FROM prudential.ratio_snapshots WHERE institution_id = $1 AND snapshot_date BETWEEN $2 AND $3 ORDER BY snapshot_date",
        )
        .bind(institution_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(row_to_snapshot).collect())
    }

    async fn save_exposure(&self, ratio_id: &RatioId, exposure: &Exposure) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO prudential.exposures (ratio_id, beneficiary_id, amount, description)
               VALUES ($1, $2, $3, $4)"#,
        )
        .bind(ratio_id.as_uuid())
        .bind(exposure.beneficiary_id())
        .bind(exposure.amount())
        .bind(exposure.description())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_exposures(&self, ratio_id: &RatioId) -> Result<Vec<Exposure>, String> {
        let rows = sqlx::query_as::<_, ExposureRow>(
            "SELECT * FROM prudential.exposures WHERE ratio_id = $1",
        )
        .bind(ratio_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| {
                Exposure::new(
                    r.beneficiary_id,
                    r.amount,
                    r.description.unwrap_or_default(),
                )
            })
            .collect())
    }
}

// --- Breach Alert Repository ---

pub struct PgBreachAlertRepository {
    pool: PgPool,
}

impl PgBreachAlertRepository {
    pub fn new(pool: PgPool) -> Self {
        PgBreachAlertRepository { pool }
    }
}

#[async_trait]
impl IBreachAlertRepository for PgBreachAlertRepository {
    async fn save(&self, alert: &BreachAlert) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO prudential.breach_alerts
               (id, ratio_id, breach_type, current_value, threshold, severity, status)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(alert.id())
        .bind(alert.ratio_id().as_uuid())
        .bind(alert.breach_type().as_str())
        .bind(alert.current_value())
        .bind(alert.threshold())
        .bind(alert.severity().as_str())
        .bind(alert.status().as_str())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_active(&self, _institution_id: Uuid) -> Result<Vec<BreachAlert>, String> {
        // Would join with ratios table for institution filtering
        Ok(vec![])
    }

    async fn find_all(
        &self,
        _institution_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BreachAlert>, String> {
        let _rows = sqlx::query(
            "SELECT * FROM prudential.breach_alerts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Simplified: actual implementation would map rows to BreachAlert
        Ok(vec![])
    }

    async fn count_active(&self, _institution_id: Option<Uuid>) -> Result<i64, String> {
        let row: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM prudential.breach_alerts WHERE status = 'Breach'")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(row.0)
    }
}

// --- Row types ---

#[derive(sqlx::FromRow)]
struct PrudentialRatioRow {
    id: Uuid,
    institution_id: Uuid,
    capital_tier1: i64,
    capital_tier2: i64,
    risk_weighted_assets: i64,
    total_credits: i64,
    total_deposits: i64,
    #[allow(dead_code)]
    solvency_ratio: f64,
    #[allow(dead_code)]
    tier1_ratio: f64,
    #[allow(dead_code)]
    credit_deposit_ratio: f64,
    calculated_at: chrono::DateTime<chrono::Utc>,
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct RatioSnapshotRow {
    #[allow(dead_code)]
    id: Uuid,
    ratio_id: Uuid,
    institution_id: Uuid,
    snapshot_date: NaiveDate,
    solvency_ratio: f64,
    tier1_ratio: f64,
    credit_deposit_ratio: f64,
    breach_type: Option<String>,
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct ExposureRow {
    #[allow(dead_code)]
    id: Uuid,
    #[allow(dead_code)]
    ratio_id: Uuid,
    beneficiary_id: Uuid,
    amount: i64,
    description: Option<String>,
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

fn row_to_ratio(row: PrudentialRatioRow, exposures: Vec<Exposure>) -> PrudentialRatio {
    PrudentialRatio::from_raw(
        RatioId::from_uuid(row.id),
        InstitutionId::from_uuid(row.institution_id),
        row.capital_tier1,
        row.capital_tier2,
        row.risk_weighted_assets,
        row.total_credits,
        row.total_deposits,
        exposures,
        row.calculated_at,
    )
}

fn row_to_snapshot(row: RatioSnapshotRow) -> RatioSnapshot {
    RatioSnapshot::new(
        RatioId::from_uuid(row.ratio_id),
        InstitutionId::from_uuid(row.institution_id),
        row.snapshot_date,
        row.solvency_ratio,
        row.tier1_ratio,
        row.credit_deposit_ratio,
        row.breach_type
            .and_then(|s| BreachType::from_str_value(&s).ok()),
    )
}
