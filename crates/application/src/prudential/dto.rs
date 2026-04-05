use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CalculateRatiosRequest {
    pub institution_id: String,
    pub capital_tier1: i64,
    pub capital_tier2: i64,
    pub risk_weighted_assets: i64,
    pub total_credits: i64,
    pub total_deposits: i64,
    pub exposures: Option<Vec<ExposureRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct ExposureRequest {
    pub beneficiary_id: String,
    pub amount: i64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct RatioHistoryQuery {
    pub institution_id: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct ConcentrationCheckRequest {
    pub institution_id: String,
    pub beneficiary_id: String,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct PrudentialRatioResponse {
    pub id: String,
    pub institution_id: String,
    pub solvency_ratio: f64,
    pub tier1_ratio: f64,
    pub credit_deposit_ratio: f64,
    pub capital_tier1: i64,
    pub capital_tier2: i64,
    pub fonds_propres_nets: i64,
    pub risk_weighted_assets: i64,
    pub total_credits: i64,
    pub total_deposits: i64,
    pub breaches: Vec<String>,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SolvencyCheckResponse {
    pub ratio: f64,
    pub minimum: f64,
    pub compliant: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct Tier1CheckResponse {
    pub ratio: f64,
    pub minimum: f64,
    pub compliant: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct CreditDepositCheckResponse {
    pub ratio: f64,
    pub maximum: f64,
    pub compliant: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ConcentrationCheckResponse {
    pub beneficiary_id: String,
    pub ratio: f64,
    pub maximum: f64,
    pub compliant: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct BreachAlertResponse {
    pub id: String,
    pub breach_type: String,
    pub current_value: f64,
    pub threshold: f64,
    pub severity: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BreachAlertListResponse {
    pub data: Vec<BreachAlertResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct RatioSnapshotResponse {
    pub snapshot_date: NaiveDate,
    pub solvency_ratio: f64,
    pub tier1_ratio: f64,
    pub credit_deposit_ratio: f64,
    pub breach_type: Option<String>,
}
