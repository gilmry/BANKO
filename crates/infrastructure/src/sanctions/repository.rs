use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::sanctions::{
    ISanctionEntryRepository, ISanctionListRepository, IScreeningResultRepository,
};
use banko_domain::sanctions::*;

// ============================================================
// PgSanctionListRepository
// ============================================================

pub struct PgSanctionListRepository {
    pool: PgPool,
}

impl PgSanctionListRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSanctionListRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct ListRow {
    id: Uuid,
    source: String,
    version: String,
    entry_count: i32,
    last_updated: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl ListRow {
    fn into_domain(self) -> Result<SanctionList, String> {
        let source = ListSource::from_str_source(&self.source).map_err(|e| e.to_string())?;
        Ok(SanctionList::from_parts(
            SanctionListId::from_uuid(self.id),
            source,
            self.version,
            Vec::new(), // Entries loaded separately
            self.last_updated,
            self.created_at,
        ))
    }
}

#[async_trait]
impl ISanctionListRepository for PgSanctionListRepository {
    async fn save(&self, list: &SanctionList) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO sanctions.lists (id, source, version, entry_count, last_updated, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET version = $3, entry_count = $4, last_updated = $5",
        )
        .bind(list.id().as_uuid())
        .bind(list.source().as_str())
        .bind(list.version())
        .bind(list.entry_count() as i32)
        .bind(list.last_updated())
        .bind(list.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &SanctionListId) -> Result<Option<SanctionList>, String> {
        let row: Option<ListRow> =
            sqlx::query_as("SELECT * FROM sanctions.lists WHERE id = $1")
                .bind(id.as_uuid())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_by_source(&self, source: ListSource) -> Result<Option<SanctionList>, String> {
        let row: Option<ListRow> =
            sqlx::query_as("SELECT * FROM sanctions.lists WHERE source = $1 ORDER BY last_updated DESC LIMIT 1")
                .bind(source.as_str())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_all(&self) -> Result<Vec<SanctionList>, String> {
        let rows: Vec<ListRow> =
            sqlx::query_as("SELECT * FROM sanctions.lists ORDER BY source")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}

// ============================================================
// PgSanctionEntryRepository
// ============================================================

pub struct PgSanctionEntryRepository {
    pool: PgPool,
}

impl PgSanctionEntryRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSanctionEntryRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct EntryRow {
    id: Uuid,
    list_source: String,
    full_name: String,
    aliases: Vec<String>,
    country: Option<String>,
    listing_date: Option<NaiveDate>,
    delisting_date: Option<NaiveDate>,
    additional_info: Option<String>,
    active: bool,
}

impl EntryRow {
    fn into_domain(self) -> Result<SanctionEntry, String> {
        let source = ListSource::from_str_source(&self.list_source).map_err(|e| e.to_string())?;
        Ok(SanctionEntry::from_parts(
            SanctionEntryId::from_uuid(self.id),
            source,
            self.full_name,
            self.aliases,
            self.country,
            self.listing_date,
            self.delisting_date,
            self.additional_info,
            self.active,
        ))
    }
}

#[async_trait]
impl ISanctionEntryRepository for PgSanctionEntryRepository {
    async fn save_entries(&self, entries: &[SanctionEntry]) -> Result<(), String> {
        for entry in entries {
            let normalized = fuzzy_matcher::normalize_name(entry.full_name());
            sqlx::query(
                "INSERT INTO sanctions.entries (id, list_id, list_source, full_name, normalized_name, aliases, country, listing_date, delisting_date, additional_info, active)
                 VALUES ($1, (SELECT id FROM sanctions.lists WHERE source = $2 ORDER BY last_updated DESC LIMIT 1), $2, $3, $4, $5, $6, $7, $8, $9, $10)
                 ON CONFLICT (id) DO UPDATE SET active = $10, delisting_date = $8, updated_at = NOW()",
            )
            .bind(entry.id().as_uuid())
            .bind(entry.list_source().as_str())
            .bind(entry.full_name())
            .bind(&normalized)
            .bind(entry.aliases())
            .bind(entry.country())
            .bind(entry.listing_date())
            .bind(entry.delisting_date())
            .bind(entry.additional_info())
            .bind(entry.active())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &SanctionEntryId) -> Result<Option<SanctionEntry>, String> {
        let row: Option<EntryRow> = sqlx::query_as(
            "SELECT id, list_source, full_name, aliases, country, listing_date, delisting_date, additional_info, active FROM sanctions.entries WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_all_active(&self) -> Result<Vec<SanctionEntry>, String> {
        let rows: Vec<EntryRow> = sqlx::query_as(
            "SELECT id, list_source, full_name, aliases, country, listing_date, delisting_date, additional_info, active FROM sanctions.entries WHERE active = TRUE",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_source(&self, source: ListSource) -> Result<Vec<SanctionEntry>, String> {
        let rows: Vec<EntryRow> = sqlx::query_as(
            "SELECT id, list_source, full_name, aliases, country, listing_date, delisting_date, additional_info, active FROM sanctions.entries WHERE list_source = $1 AND active = TRUE",
        )
        .bind(source.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn count_active(&self) -> Result<i64, String> {
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM sanctions.entries WHERE active = TRUE")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        Ok(count.0)
    }
}

// ============================================================
// PgScreeningResultRepository
// ============================================================

pub struct PgScreeningResultRepository {
    pool: PgPool,
}

impl PgScreeningResultRepository {
    pub fn new(pool: PgPool) -> Self {
        PgScreeningResultRepository { pool }
    }
}

#[async_trait]
impl IScreeningResultRepository for PgScreeningResultRepository {
    async fn save(&self, result: &ScreeningResult) -> Result<(), String> {
        let match_details_json =
            serde_json::to_value(result.matched_entries()).map_err(|e| e.to_string())?;
        let normalized = fuzzy_matcher::normalize_name(result.screened_name());

        sqlx::query(
            "INSERT INTO sanctions.screening_results (id, screened_name, normalized_name, status, highest_score, match_count, match_details, screened_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(result.id().as_uuid())
        .bind(result.screened_name())
        .bind(&normalized)
        .bind(result.status().as_str())
        .bind(result.highest_score() as i32)
        .bind(result.matched_entries().len() as i32)
        .bind(&match_details_json)
        .bind(result.screened_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ScreeningResultId,
    ) -> Result<Option<ScreeningResult>, String> {
        let row: Option<ScreeningResultRow> =
            sqlx::query_as("SELECT * FROM sanctions.screening_results WHERE id = $1")
                .bind(id.as_uuid())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_recent(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ScreeningResult>, String> {
        let rows: Vec<ScreeningResultRow> = sqlx::query_as(
            "SELECT * FROM sanctions.screening_results ORDER BY screened_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn count_by_status(
        &self,
        status: Option<ScreeningStatus>,
    ) -> Result<i64, String> {
        let count: (i64,) = if let Some(s) = status {
            sqlx::query_as("SELECT COUNT(*) FROM sanctions.screening_results WHERE status = $1")
                .bind(s.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query_as("SELECT COUNT(*) FROM sanctions.screening_results")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        };
        Ok(count.0)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ScreeningResultRow {
    id: Uuid,
    screened_name: String,
    #[allow(dead_code)]
    normalized_name: String,
    status: String,
    #[allow(dead_code)]
    highest_score: i32,
    #[allow(dead_code)]
    match_count: i32,
    match_details: serde_json::Value,
    screened_at: DateTime<Utc>,
}

impl ScreeningResultRow {
    fn into_domain(self) -> Result<ScreeningResult, String> {
        let status =
            ScreeningStatus::from_str_status(&self.status).map_err(|e| e.to_string())?;
        let matched_entries: Vec<MatchDetail> =
            serde_json::from_value(self.match_details).map_err(|e| e.to_string())?;

        Ok(ScreeningResult::from_parts(
            ScreeningResultId::from_uuid(self.id),
            self.screened_name,
            matched_entries,
            status,
            self.screened_at,
        ))
    }
}
