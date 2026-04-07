use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Arrangement DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArrangementRequest {
    pub customer_id: String,
    pub product_id: String,
    pub arrangement_type: String,
    pub effective_date: DateTime<Utc>,
    pub maturity_date: Option<DateTime<Utc>>,
    pub terms: ArrangementTermsDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArrangementTermsDto {
    pub currency: String,
    pub interest_rate: Option<f64>,
    pub fee_schedule_id: Option<String>,
    pub minimum_balance: Option<i64>,
    pub overdraft_limit: Option<i64>,
    pub renewal_type: String,
    pub notice_period_days: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrangementResponse {
    pub id: String,
    pub customer_id: String,
    pub product_id: String,
    pub arrangement_type: String,
    pub status: String,
    pub effective_date: DateTime<Utc>,
    pub maturity_date: Option<DateTime<Utc>>,
    pub terms: ArrangementTermsDto,
    pub linked_accounts: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkAccountRequest {
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnlinkAccountRequest {
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArrangementStatusRequest {
    pub new_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateArrangementRequest {
    pub arrangement_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspendArrangementRequest {
    pub arrangement_id: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseArrangementRequest {
    pub arrangement_id: String,
    pub outstanding_loan_balance: i64,
}

// --- Arrangement Bundle DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBundleRequest {
    pub name: String,
    pub arrangement_ids: Vec<String>,
    pub discount_pct: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrangementBundleResponse {
    pub id: String,
    pub name: String,
    pub arrangement_ids: Vec<String>,
    pub discount_pct: Option<f64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddToBundleRequest {
    pub arrangement_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveFromBundleRequest {
    pub arrangement_id: String,
}

// --- Analytics DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrangementStatsResponse {
    pub total_arrangements: i64,
    pub active_count: i64,
    pub suspended_count: i64,
    pub matured_count: i64,
    pub closed_count: i64,
    pub maturing_soon_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerArrangementsResponse {
    pub customer_id: String,
    pub arrangements: Vec<ArrangementResponse>,
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaturityAlertResponse {
    pub arrangement_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub maturity_date: DateTime<Utc>,
    pub days_remaining: i64,
    pub renewal_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrangementEventResponse {
    pub id: String,
    pub arrangement_id: String,
    pub event_type: String,
    pub event_data: String,
    pub occurred_at: DateTime<Utc>,
}
