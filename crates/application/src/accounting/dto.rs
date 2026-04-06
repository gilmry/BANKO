use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub journal_code: String,
    pub entry_date: NaiveDate,
    pub description: String,
    pub lines: Vec<CreateEntryLineRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryLineRequest {
    pub account_code: String,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EntryFilterQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub journal_code: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ClosePeriodRequest {
    pub period: String,
}

#[derive(Debug, Deserialize)]
pub struct TrialBalanceQuery {
    pub as_of: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct LedgerQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub account_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EclStagingQuery {
    pub as_of: Option<NaiveDate>,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct JournalEntryResponse {
    pub id: String,
    pub journal_code: String,
    pub entry_date: NaiveDate,
    pub description: String,
    pub status: String,
    pub reversal_of: Option<String>,
    pub total_debit: i64,
    pub total_credit: i64,
    pub lines: Vec<JournalLineResponse>,
    pub created_at: DateTime<Utc>,
    pub posted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct JournalLineResponse {
    pub line_id: String,
    pub account_code: String,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JournalEntryListResponse {
    pub data: Vec<JournalEntryResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct TrialBalanceResponse {
    pub as_of: NaiveDate,
    pub lines: Vec<TrialBalanceLineResponse>,
    pub total_debit: i64,
    pub total_credit: i64,
    pub is_balanced: bool,
}

#[derive(Debug, Serialize)]
pub struct TrialBalanceLineResponse {
    pub account_code: String,
    pub label: String,
    pub account_type: String,
    pub debit: i64,
    pub credit: i64,
}

#[derive(Debug, Serialize)]
pub struct GeneralLedgerResponse {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub entries: Vec<JournalEntryResponse>,
}

#[derive(Debug, Serialize)]
pub struct PeriodClosingResponse {
    pub period: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct EclStagingResponse {
    pub loan_id: String,
    pub stage: String,
    pub probability_of_default: f64,
    pub loss_given_default: f64,
    pub exposure_at_default: i64,
    pub ecl_amount: i64,
}
