use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- SweepAccount DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSweepAccountRequest {
    pub source_account_id: String,
    pub target_account_id: String,
    pub sweep_type: String,
    pub threshold_amount: Option<f64>,
    pub sweep_frequency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SweepAccountResponse {
    pub id: String,
    pub source_account_id: String,
    pub target_account_id: String,
    pub sweep_type: String,
    pub threshold_amount: Option<f64>,
    pub threshold_currency: Option<String>,
    pub sweep_frequency: String,
    pub is_active: bool,
    pub last_sweep_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- CashPool DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCashPoolRequest {
    pub name: String,
    pub header_account_id: String,
    pub participant_account_ids: Vec<String>,
    pub pool_type: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashPoolResponse {
    pub id: String,
    pub name: String,
    pub header_account_id: String,
    pub participant_account_ids: Vec<String>,
    pub pool_type: String,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddParticipantRequest {
    pub account_id: String,
}

// --- CashForecast DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCashForecastRequest {
    pub account_id: String,
    pub forecast_date: DateTime<Utc>,
    pub expected_inflows: f64,
    pub expected_outflows: f64,
    pub confidence_level: String,
    pub horizon_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashForecastResponse {
    pub id: String,
    pub account_id: String,
    pub forecast_date: DateTime<Utc>,
    pub expected_inflows: f64,
    pub expected_outflows: f64,
    pub currency: String,
    pub net_position: f64,
    pub confidence_level: String,
    pub horizon_days: u32,
    pub created_at: DateTime<Utc>,
}

// --- LiquidityPosition DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLiquidityPositionRequest {
    pub date: DateTime<Utc>,
    pub currency: String,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub lcr_eligible_assets: f64,
    pub nsfr_stable_funding: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidityPositionResponse {
    pub date: DateTime<Utc>,
    pub currency: String,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub net_position: f64,
    pub lcr_eligible_assets: f64,
    pub nsfr_stable_funding: f64,
    pub lcr_ratio: f64,
    pub nsfr_ratio: f64,
}

// --- FundingStrategy DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFundingStrategyRequest {
    pub name: String,
    pub target_ratio: f64,
    pub instruments: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingStrategyResponse {
    pub id: String,
    pub name: String,
    pub target_ratio: f64,
    pub instruments: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTargetRatioRequest {
    pub new_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddInstrumentRequest {
    pub instrument: String,
}
