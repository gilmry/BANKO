use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::aml::{
    IAlertRepository, IAssetFreezeRepository, IInvestigationRepository,
    ISuspicionReportRepository, ITransactionRepository,
};
use banko_domain::aml::*;
use banko_domain::shared::{Currency, Money};

// ============================================================
// PgTransactionRepository
// ============================================================

pub struct PgTransactionRepository {
    pool: PgPool,
}

impl PgTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        PgTransactionRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct TransactionRow {
    id: Uuid,
    account_id: Uuid,
    customer_id: Uuid,
    counterparty: String,
    amount: i64,
    currency: String,
    transaction_type: String,
    direction: String,
    transaction_timestamp: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl TransactionRow {
    fn into_domain(self) -> Result<Transaction, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let amount = Money::from_cents(self.amount, currency);
        let tx_type =
            TransactionType::from_str_type(&self.transaction_type).map_err(|e| e.to_string())?;
        let direction = Direction::from_str_dir(&self.direction).map_err(|e| e.to_string())?;

        Ok(Transaction::from_parts(
            TransactionId::from_uuid(self.id),
            self.account_id,
            self.customer_id,
            self.counterparty,
            amount,
            tx_type,
            direction,
            self.transaction_timestamp,
            self.created_at,
        ))
    }
}

#[async_trait]
impl ITransactionRepository for PgTransactionRepository {
    async fn save(&self, tx: &Transaction) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO aml.transactions (id, account_id, customer_id, counterparty, amount, currency, transaction_type, direction, transaction_timestamp, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             ON CONFLICT (id) DO NOTHING",
        )
        .bind(tx.id().as_uuid())
        .bind(tx.account_id())
        .bind(tx.customer_id())
        .bind(tx.counterparty())
        .bind(tx.amount().amount_cents())
        .bind(tx.amount().currency().to_string())
        .bind(tx.transaction_type().as_str())
        .bind(tx.direction().as_str())
        .bind(tx.timestamp())
        .bind(tx.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, String> {
        let row: Option<TransactionRow> =
            sqlx::query_as("SELECT * FROM aml.transactions WHERE id = $1")
                .bind(id.as_uuid())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<Transaction>, String> {
        let rows: Vec<TransactionRow> =
            sqlx::query_as("SELECT * FROM aml.transactions WHERE account_id = $1 ORDER BY transaction_timestamp DESC")
                .bind(account_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_date_range(
        &self,
        account_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Transaction>, String> {
        let rows: Vec<TransactionRow> = sqlx::query_as(
            "SELECT * FROM aml.transactions WHERE account_id = $1 AND transaction_timestamp BETWEEN $2 AND $3 ORDER BY transaction_timestamp",
        )
        .bind(account_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_all(
        &self,
        account_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>, String> {
        let rows: Vec<TransactionRow> = if let Some(aid) = account_id {
            sqlx::query_as(
                "SELECT * FROM aml.transactions WHERE account_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(aid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        } else {
            sqlx::query_as(
                "SELECT * FROM aml.transactions ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        };
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn count_all(&self, account_id: Option<Uuid>) -> Result<i64, String> {
        let count: (i64,) = if let Some(aid) = account_id {
            sqlx::query_as("SELECT COUNT(*) FROM aml.transactions WHERE account_id = $1")
                .bind(aid)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query_as("SELECT COUNT(*) FROM aml.transactions")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        };
        Ok(count.0)
    }
}

// ============================================================
// PgAlertRepository (APPEND-ONLY)
// ============================================================

pub struct PgAlertRepository {
    pool: PgPool,
}

impl PgAlertRepository {
    pub fn new(pool: PgPool) -> Self {
        PgAlertRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct AlertRow {
    id: Uuid,
    transaction_id: Uuid,
    risk_level: String,
    reason: String,
    status: String,
    created_at: DateTime<Utc>,
}

impl AlertRow {
    fn into_domain(self) -> Result<Alert, String> {
        let risk_level = RiskLevel::from_str_level(&self.risk_level).map_err(|e| e.to_string())?;
        let status = AlertStatus::from_str_status(&self.status).map_err(|e| e.to_string())?;
        Ok(Alert::from_parts(
            self.id,
            TransactionId::from_uuid(self.transaction_id),
            risk_level,
            self.reason,
            status,
            self.created_at,
        ))
    }
}

#[async_trait]
impl IAlertRepository for PgAlertRepository {
    async fn save(&self, alert: &Alert) -> Result<(), String> {
        // Check if alert exists
        let exists: Option<(String,)> =
            sqlx::query_as("SELECT status FROM aml.alerts WHERE id = $1")
                .bind(alert.id())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        if let Some((old_status,)) = exists {
            let new_status = alert.status().as_str().to_string();
            if old_status != new_status {
                // Append-only: log status change, then update
                sqlx::query(
                    "INSERT INTO aml.alert_status_changes (alert_id, old_status, new_status) VALUES ($1, $2, $3)",
                )
                .bind(alert.id())
                .bind(&old_status)
                .bind(&new_status)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                sqlx::query("UPDATE aml.alerts SET status = $1 WHERE id = $2")
                    .bind(&new_status)
                    .bind(alert.id())
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        } else {
            sqlx::query(
                "INSERT INTO aml.alerts (id, transaction_id, risk_level, reason, status, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(alert.id())
            .bind(alert.transaction_id().as_uuid())
            .bind(alert.risk_level().as_str())
            .bind(alert.reason())
            .bind(alert.status().as_str())
            .bind(alert.created_at())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Alert>, String> {
        let row: Option<AlertRow> = sqlx::query_as("SELECT * FROM aml.alerts WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_by_transaction_id(
        &self,
        tx_id: &TransactionId,
    ) -> Result<Vec<Alert>, String> {
        let rows: Vec<AlertRow> =
            sqlx::query_as("SELECT * FROM aml.alerts WHERE transaction_id = $1 ORDER BY created_at")
                .bind(tx_id.as_uuid())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_status(&self, status: AlertStatus) -> Result<Vec<Alert>, String> {
        let rows: Vec<AlertRow> =
            sqlx::query_as("SELECT * FROM aml.alerts WHERE status = $1 ORDER BY created_at DESC")
                .bind(status.as_str())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_all(
        &self,
        status: Option<AlertStatus>,
        risk_level: Option<RiskLevel>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Alert>, String> {
        let mut query = "SELECT * FROM aml.alerts WHERE 1=1".to_string();
        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s.as_str()));
        }
        if let Some(rl) = risk_level {
            query.push_str(&format!(" AND risk_level = '{}'", rl.as_str()));
        }
        query.push_str(&format!(
            " ORDER BY created_at DESC LIMIT {} OFFSET {}",
            limit, offset
        ));

        let rows: Vec<AlertRow> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn count_by_status(&self, status: Option<AlertStatus>) -> Result<i64, String> {
        let count: (i64,) = if let Some(s) = status {
            sqlx::query_as("SELECT COUNT(*) FROM aml.alerts WHERE status = $1")
                .bind(s.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query_as("SELECT COUNT(*) FROM aml.alerts")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        };
        Ok(count.0)
    }
}

// ============================================================
// PgInvestigationRepository
// ============================================================

pub struct PgInvestigationRepository {
    pool: PgPool,
}

impl PgInvestigationRepository {
    pub fn new(pool: PgPool) -> Self {
        PgInvestigationRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InvestigationRow {
    id: Uuid,
    alert_id: Uuid,
    status: String,
    assigned_to: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
struct NoteRow {
    note: String,
    author: String,
    created_at: DateTime<Utc>,
}

impl PgInvestigationRepository {
    async fn load_notes(&self, investigation_id: Uuid) -> Result<Vec<InvestigationNote>, String> {
        let rows: Vec<NoteRow> = sqlx::query_as(
            "SELECT note, author, created_at FROM aml.investigation_notes WHERE investigation_id = $1 ORDER BY created_at",
        )
        .bind(investigation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| InvestigationNote::from_parts(r.note, r.author, r.created_at))
            .collect())
    }

    fn row_to_domain(
        &self,
        row: InvestigationRow,
        notes: Vec<InvestigationNote>,
    ) -> Result<Investigation, String> {
        let status =
            InvestigationStatus::from_str_status(&row.status).map_err(|e| e.to_string())?;
        Ok(Investigation::from_parts(
            row.id,
            row.alert_id,
            status,
            row.assigned_to,
            notes,
            row.created_at,
            row.updated_at,
        ))
    }
}

#[async_trait]
impl IInvestigationRepository for PgInvestigationRepository {
    async fn save(&self, inv: &Investigation) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO aml.investigations (id, alert_id, status, assigned_to, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET status = $3, assigned_to = $4, updated_at = $6",
        )
        .bind(inv.id())
        .bind(inv.alert_id())
        .bind(inv.status().as_str())
        .bind(inv.assigned_to())
        .bind(inv.created_at())
        .bind(inv.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Save new notes (append-only)
        for note in inv.notes() {
            sqlx::query(
                "INSERT INTO aml.investigation_notes (investigation_id, note, author, created_at)
                 SELECT $1, $2, $3, $4 WHERE NOT EXISTS (
                     SELECT 1 FROM aml.investigation_notes
                     WHERE investigation_id = $1 AND note = $2 AND author = $3 AND created_at = $4
                 )",
            )
            .bind(inv.id())
            .bind(note.note())
            .bind(note.author())
            .bind(note.created_at())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Investigation>, String> {
        let row: Option<InvestigationRow> =
            sqlx::query_as("SELECT * FROM aml.investigations WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        match row {
            Some(r) => {
                let notes = self.load_notes(r.id).await?;
                Ok(Some(self.row_to_domain(r, notes)?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_alert_id(&self, alert_id: Uuid) -> Result<Option<Investigation>, String> {
        let row: Option<InvestigationRow> =
            sqlx::query_as("SELECT * FROM aml.investigations WHERE alert_id = $1")
                .bind(alert_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        match row {
            Some(r) => {
                let notes = self.load_notes(r.id).await?;
                Ok(Some(self.row_to_domain(r, notes)?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_status(
        &self,
        status: InvestigationStatus,
    ) -> Result<Vec<Investigation>, String> {
        let rows: Vec<InvestigationRow> =
            sqlx::query_as("SELECT * FROM aml.investigations WHERE status = $1 ORDER BY created_at DESC")
                .bind(status.as_str())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        let mut result = Vec::new();
        for r in rows {
            let notes = self.load_notes(r.id).await?;
            result.push(self.row_to_domain(r, notes)?);
        }
        Ok(result)
    }

    async fn find_all(
        &self,
        status: Option<InvestigationStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Investigation>, String> {
        let rows: Vec<InvestigationRow> = if let Some(s) = status {
            sqlx::query_as(
                "SELECT * FROM aml.investigations WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(s.as_str())
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        } else {
            sqlx::query_as(
                "SELECT * FROM aml.investigations ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
        };
        let mut result = Vec::new();
        for r in rows {
            let notes = self.load_notes(r.id).await?;
            result.push(self.row_to_domain(r, notes)?);
        }
        Ok(result)
    }

    async fn count_all(&self, status: Option<InvestigationStatus>) -> Result<i64, String> {
        let count: (i64,) = if let Some(s) = status {
            sqlx::query_as("SELECT COUNT(*) FROM aml.investigations WHERE status = $1")
                .bind(s.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query_as("SELECT COUNT(*) FROM aml.investigations")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        };
        Ok(count.0)
    }
}

// ============================================================
// PgSuspicionReportRepository
// ============================================================

pub struct PgSuspicionReportRepository {
    pool: PgPool,
}

impl PgSuspicionReportRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSuspicionReportRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ReportRow {
    id: Uuid,
    investigation_id: Uuid,
    customer_info: String,
    transaction_details: String,
    reasons: String,
    evidence: Option<String>,
    timeline: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
}

impl ReportRow {
    fn into_domain(self) -> Result<SuspicionReport, String> {
        let status = ReportStatus::from_str_status(&self.status).map_err(|e| e.to_string())?;
        Ok(SuspicionReport::from_parts(
            self.id,
            self.investigation_id,
            self.customer_info,
            self.transaction_details,
            self.reasons,
            self.evidence,
            self.timeline,
            status,
            self.created_at,
            self.submitted_at,
        ))
    }
}

#[async_trait]
impl ISuspicionReportRepository for PgSuspicionReportRepository {
    async fn save(&self, report: &SuspicionReport) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO aml.suspicion_reports (id, investigation_id, customer_info, transaction_details, reasons, evidence, timeline, status, created_at, submitted_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             ON CONFLICT (id) DO UPDATE SET status = $8, submitted_at = $10",
        )
        .bind(report.id())
        .bind(report.investigation_id())
        .bind(report.customer_info())
        .bind(report.transaction_details())
        .bind(report.reasons())
        .bind(report.evidence())
        .bind(report.timeline())
        .bind(report.status().as_str())
        .bind(report.created_at())
        .bind(report.submitted_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SuspicionReport>, String> {
        let row: Option<ReportRow> =
            sqlx::query_as("SELECT * FROM aml.suspicion_reports WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_by_investigation_id(
        &self,
        investigation_id: Uuid,
    ) -> Result<Option<SuspicionReport>, String> {
        let row: Option<ReportRow> =
            sqlx::query_as("SELECT * FROM aml.suspicion_reports WHERE investigation_id = $1")
                .bind(investigation_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }
}

// ============================================================
// PgAssetFreezeRepository
// ============================================================

pub struct PgAssetFreezeRepository {
    pool: PgPool,
}

impl PgAssetFreezeRepository {
    pub fn new(pool: PgPool) -> Self {
        PgAssetFreezeRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct FreezeRow {
    id: Uuid,
    account_id: Uuid,
    reason: String,
    ordered_by: String,
    status: String,
    frozen_at: DateTime<Utc>,
    lifted_at: Option<DateTime<Utc>>,
    lifted_by: Option<String>,
}

impl FreezeRow {
    fn into_domain(self) -> Result<AssetFreeze, String> {
        let status = FreezeStatus::from_str_status(&self.status).map_err(|e| e.to_string())?;
        Ok(AssetFreeze::from_parts(
            self.id,
            self.account_id,
            self.reason,
            self.ordered_by,
            status,
            self.frozen_at,
            self.lifted_at,
            self.lifted_by,
        ))
    }
}

#[async_trait]
impl IAssetFreezeRepository for PgAssetFreezeRepository {
    async fn save(&self, freeze: &AssetFreeze) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO aml.asset_freezes (id, account_id, reason, ordered_by, status, frozen_at, lifted_at, lifted_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET status = $5, lifted_at = $7, lifted_by = $8",
        )
        .bind(freeze.id())
        .bind(freeze.account_id())
        .bind(freeze.reason())
        .bind(freeze.ordered_by())
        .bind(freeze.status().as_str())
        .bind(freeze.frozen_at())
        .bind(freeze.lifted_at())
        .bind(freeze.lifted_by())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AssetFreeze>, String> {
        let row: Option<FreezeRow> =
            sqlx::query_as("SELECT * FROM aml.asset_freezes WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<AssetFreeze>, String> {
        let rows: Vec<FreezeRow> =
            sqlx::query_as("SELECT * FROM aml.asset_freezes WHERE account_id = $1 ORDER BY frozen_at DESC")
                .bind(account_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_active_by_account_id(
        &self,
        account_id: Uuid,
    ) -> Result<Option<AssetFreeze>, String> {
        let row: Option<FreezeRow> = sqlx::query_as(
            "SELECT * FROM aml.asset_freezes WHERE account_id = $1 AND status = 'Active' LIMIT 1",
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }
}
