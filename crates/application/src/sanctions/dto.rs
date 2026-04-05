use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Screening DTOs ---

#[derive(Debug, Deserialize)]
pub struct ScreeningRequest {
    pub name: String,
    pub threshold: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct ScreeningMatchResponse {
    pub entry_name: String,
    pub list_source: String,
    pub score: u8,
    pub matched_fields: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ScreeningResponse {
    pub id: String,
    pub screened_name: String,
    pub status: String,
    pub highest_score: u8,
    pub matches: Vec<ScreeningMatchResponse>,
    pub screened_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ScreeningListResponse {
    pub data: Vec<ScreeningResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// --- List Status DTOs ---

#[derive(Debug, Serialize)]
pub struct ListStatusResponse {
    pub source: String,
    pub version: String,
    pub entry_count: usize,
    pub active_entries: usize,
    pub last_updated: DateTime<Utc>,
}

// --- Dashboard DTOs ---

#[derive(Debug, Serialize)]
pub struct ScreeningStatsResponse {
    pub total_screenings: i64,
    pub hits: i64,
    pub potential_matches: i64,
    pub clear: i64,
    pub lists: Vec<ListStatusResponse>,
}

// --- Sync DTOs ---

#[derive(Debug, Deserialize)]
pub struct SyncListRequest {
    pub source: String,
    pub version: String,
    pub entries: Vec<SyncEntryRequest>,
}

#[derive(Debug, Deserialize)]
pub struct SyncEntryRequest {
    pub full_name: String,
    pub aliases: Vec<String>,
    pub country: Option<String>,
}
