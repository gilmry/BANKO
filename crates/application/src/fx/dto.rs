use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// --- FX Operation DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateFxOperationRequest {
    pub account_id: String,
    pub operation_type: Option<String>,
    pub source_currency: String,
    pub target_currency: String,
    pub source_amount: i64,
    pub rate: Option<f64>,
    pub reference: String,
}

#[derive(Debug, Serialize)]
pub struct FxOperationResponse {
    pub id: String,
    pub account_id: String,
    pub operation_type: String,
    pub source_currency: String,
    pub target_currency: String,
    pub source_amount: i64,
    pub target_amount: i64,
    pub rate: f64,
    pub status: String,
    pub reference: String,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub settled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct FxOperationListResponse {
    pub data: Vec<FxOperationResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// --- Exchange Rate DTOs ---

#[derive(Debug, Deserialize)]
pub struct UpdateRateRequest {
    pub source_currency: String,
    pub target_currency: String,
    pub rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ExchangeRateResponse {
    pub source_currency: String,
    pub target_currency: String,
    pub rate: f64,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct RateListResponse {
    pub data: Vec<ExchangeRateResponse>,
}

#[derive(Debug, Deserialize)]
pub struct RateHistoryQuery {
    pub source: String,
    pub target: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
}

// --- Position DTOs ---

#[derive(Debug, Serialize)]
pub struct FxPositionResponse {
    pub currency: String,
    pub long_amount: i64,
    pub short_amount: i64,
    pub net_position: i64,
}

#[derive(Debug, Serialize)]
pub struct PositionSummaryResponse {
    pub positions: Vec<FxPositionResponse>,
}

// --- Daily Limit DTOs ---

#[derive(Debug, Serialize)]
pub struct DailyLimitResponse {
    pub account_id: String,
    pub currency: String,
    pub daily_limit: i64,
    pub used_today: i64,
    pub remaining: i64,
}

#[derive(Debug, Serialize)]
pub struct DailyLimitsResponse {
    pub limits: Vec<DailyLimitResponse>,
}

// --- Reject DTO ---

#[derive(Debug, Deserialize)]
pub struct RejectFxRequest {
    pub reason: String,
}
