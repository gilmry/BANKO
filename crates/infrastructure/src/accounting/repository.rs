use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::accounting::{
    AccountBalanceRow, IJournalRepository, ILedgerRepository, IPeriodRepository,
};
use banko_domain::accounting::*;

// --- Journal Repository ---

pub struct PgJournalRepository {
    pool: PgPool,
}

impl PgJournalRepository {
    pub fn new(pool: PgPool) -> Self {
        PgJournalRepository { pool }
    }
}

#[async_trait]
impl IJournalRepository for PgJournalRepository {
    async fn save(&self, entry: &JournalEntry) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"INSERT INTO accounting.journal_entries
               (id, journal_code, entry_date, description, status, reversal_of, created_at, posted_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, posted_at = EXCLUDED.posted_at"#,
        )
        .bind(entry.entry_id().as_uuid())
        .bind(entry.journal_code().as_str())
        .bind(entry.entry_date())
        .bind(entry.description())
        .bind(entry.status().as_str())
        .bind(entry.reversal_of().map(|id| *id.as_uuid()))
        .bind(entry.created_at())
        .bind(entry.posted_at())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        for line in entry.lines() {
            sqlx::query(
                r#"INSERT INTO accounting.journal_lines
                   (id, entry_id, account_code, debit, credit, description)
                   VALUES ($1, $2, $3, $4, $5, $6)
                   ON CONFLICT (id) DO NOTHING"#,
            )
            .bind(line.line_id())
            .bind(entry.entry_id().as_uuid())
            .bind(line.account_code().as_str())
            .bind(line.debit())
            .bind(line.credit())
            .bind(line.description())
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &EntryId) -> Result<Option<JournalEntry>, String> {
        let row = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT * FROM accounting.journal_entries WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                let lines = self.fetch_lines(r.id).await?;
                Ok(Some(row_to_entry(r, lines)))
            }
            None => Ok(None),
        }
    }

    async fn find_by_period(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<JournalEntry>, String> {
        let rows = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT * FROM accounting.journal_entries WHERE entry_date BETWEEN $1 AND $2 ORDER BY entry_date, created_at",
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut entries = Vec::new();
        for r in rows {
            let lines = self.fetch_lines(r.id).await?;
            entries.push(row_to_entry(r, lines));
        }
        Ok(entries)
    }

    async fn find_by_account(
        &self,
        code: &AccountCode,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<JournalEntry>, String> {
        let rows = sqlx::query_as::<_, JournalEntryRow>(
            r#"SELECT e.* FROM accounting.journal_entries e
               JOIN accounting.journal_lines l ON l.entry_id = e.id
               WHERE l.account_code = $1 AND e.entry_date BETWEEN $2 AND $3
               ORDER BY e.entry_date, e.created_at"#,
        )
        .bind(code.as_str())
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut entries = Vec::new();
        for r in rows {
            let lines = self.fetch_lines(r.id).await?;
            entries.push(row_to_entry(r, lines));
        }
        Ok(entries)
    }

    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<JournalEntry>, String> {
        let rows = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT * FROM accounting.journal_entries ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut entries = Vec::new();
        for r in rows {
            let lines = self.fetch_lines(r.id).await?;
            entries.push(row_to_entry(r, lines));
        }
        Ok(entries)
    }

    async fn count_all(&self) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accounting.journal_entries")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(row.0)
    }
}

impl PgJournalRepository {
    async fn fetch_lines(&self, entry_id: Uuid) -> Result<Vec<JournalLine>, String> {
        let rows = sqlx::query_as::<_, JournalLineRow>(
            "SELECT * FROM accounting.journal_lines WHERE entry_id = $1 ORDER BY id",
        )
        .bind(entry_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(row_to_line).collect())
    }
}

// --- Ledger Repository ---

pub struct PgLedgerRepository {
    pool: PgPool,
}

impl PgLedgerRepository {
    pub fn new(pool: PgPool) -> Self {
        PgLedgerRepository { pool }
    }
}

#[async_trait]
impl ILedgerRepository for PgLedgerRepository {
    async fn get_account_balance(
        &self,
        code: &AccountCode,
        as_of: NaiveDate,
    ) -> Result<(i64, i64), String> {
        let row: (i64, i64) = sqlx::query_as(
            r#"SELECT COALESCE(SUM(l.debit), 0), COALESCE(SUM(l.credit), 0)
               FROM accounting.journal_lines l
               JOIN accounting.journal_entries e ON e.id = l.entry_id
               WHERE l.account_code = $1 AND e.entry_date <= $2 AND e.status = 'Posted'"#,
        )
        .bind(code.as_str())
        .bind(as_of)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row)
    }

    async fn get_all_balances(&self, as_of: NaiveDate) -> Result<Vec<AccountBalanceRow>, String> {
        let rows = sqlx::query_as::<_, BalanceQueryRow>(
            r#"SELECT c.code, c.label, c.account_type,
                      COALESCE(SUM(l.debit), 0) as total_debit,
                      COALESCE(SUM(l.credit), 0) as total_credit
               FROM accounting.chart_of_accounts c
               LEFT JOIN accounting.journal_lines l ON l.account_code = c.code
               LEFT JOIN accounting.journal_entries e ON e.id = l.entry_id AND e.entry_date <= $1 AND e.status = 'Posted'
               GROUP BY c.code, c.label, c.account_type
               ORDER BY c.code"#,
        )
        .bind(as_of)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| AccountBalanceRow {
                code: r.code,
                label: r.label,
                account_type: r.account_type,
                total_debit: r.total_debit,
                total_credit: r.total_credit,
            })
            .collect())
    }

    async fn save_chart_entry(&self, entry: &LedgerAccount) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (code) DO UPDATE SET label = EXCLUDED.label"#,
        )
        .bind(entry.account_code().as_str())
        .bind(entry.label())
        .bind(entry.account_type().as_str())
        .bind(entry.nct_ref())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_chart_entry(&self, code: &AccountCode) -> Result<Option<LedgerAccount>, String> {
        let row = sqlx::query_as::<_, ChartRow>(
            "SELECT * FROM accounting.chart_of_accounts WHERE code = $1",
        )
        .bind(code.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(row_to_ledger_account))
    }

    async fn find_all_chart_entries(&self) -> Result<Vec<LedgerAccount>, String> {
        let rows = sqlx::query_as::<_, ChartRow>(
            "SELECT * FROM accounting.chart_of_accounts ORDER BY code",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(row_to_ledger_account).collect())
    }
}

// --- Period Repository ---

pub struct PgPeriodRepository {
    pool: PgPool,
}

impl PgPeriodRepository {
    pub fn new(pool: PgPool) -> Self {
        PgPeriodRepository { pool }
    }
}

#[async_trait]
impl IPeriodRepository for PgPeriodRepository {
    async fn close_period(&self, period: &str) -> Result<(), String> {
        sqlx::query("INSERT INTO accounting.closed_periods (period) VALUES ($1)")
            .bind(period)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn is_closed(&self, period: &str) -> Result<bool, String> {
        let row: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM accounting.closed_periods WHERE period = $1")
                .bind(period)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(row.0 > 0)
    }

    async fn find_closed_periods(&self) -> Result<Vec<String>, String> {
        let rows: Vec<(String,)> =
            sqlx::query_as("SELECT period FROM accounting.closed_periods ORDER BY period")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }
}

// --- Row types ---

#[derive(sqlx::FromRow)]
struct JournalEntryRow {
    id: Uuid,
    journal_code: String,
    entry_date: NaiveDate,
    description: String,
    status: String,
    reversal_of: Option<Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    posted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow)]
struct JournalLineRow {
    id: Uuid,
    #[allow(dead_code)]
    entry_id: Uuid,
    account_code: String,
    debit: i64,
    credit: i64,
    description: Option<String>,
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct BalanceQueryRow {
    code: String,
    label: String,
    account_type: String,
    total_debit: i64,
    total_credit: i64,
}

#[derive(sqlx::FromRow)]
struct ChartRow {
    code: String,
    label: String,
    account_type: String,
    nct_ref: Option<String>,
    #[allow(dead_code)]
    parent_code: Option<String>,
    #[allow(dead_code)]
    is_active: bool,
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

fn row_to_entry(row: JournalEntryRow, lines: Vec<JournalLine>) -> JournalEntry {
    JournalEntry::from_raw(
        EntryId::from_uuid(row.id),
        JournalCode::from_str_value(&row.journal_code).unwrap_or(JournalCode::OD),
        row.entry_date,
        row.description,
        lines,
        EntryStatus::from_str_value(&row.status).unwrap_or(EntryStatus::Draft),
        row.reversal_of.map(EntryId::from_uuid),
        row.created_at,
        row.posted_at,
    )
}

fn row_to_line(row: JournalLineRow) -> JournalLine {
    JournalLine::from_raw(
        row.id,
        AccountCode::from_raw(row.account_code),
        row.debit,
        row.credit,
        row.description,
    )
}

fn row_to_ledger_account(row: ChartRow) -> LedgerAccount {
    LedgerAccount::new(
        AccountCode::from_raw(row.code),
        row.label,
        AccountType::from_str_value(&row.account_type).unwrap_or(AccountType::Asset),
        row.nct_ref,
    )
}
