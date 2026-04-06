use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Transaction DTOs ---

#[derive(Debug, Deserialize)]
pub struct RecordTransactionRequest {
    pub account_id: String,
    pub customer_id: String,
    pub counterparty: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub transaction_type: String,
    pub direction: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: String,
    pub account_id: String,
    pub customer_id: String,
    pub counterparty: String,
    pub amount: f64,
    pub currency: String,
    pub transaction_type: String,
    pub direction: String,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub alerts: Vec<AlertResponse>,
}

#[derive(Debug, Serialize)]
pub struct TransactionListResponse {
    pub data: Vec<TransactionResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// --- Alert DTOs ---

#[derive(Debug, Serialize, Clone)]
pub struct AlertResponse {
    pub id: String,
    pub transaction_id: String,
    pub risk_level: String,
    pub reason: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AlertListResponse {
    pub data: Vec<AlertResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// --- Investigation DTOs ---

#[derive(Debug, Deserialize)]
pub struct OpenInvestigationRequest {
    pub alert_id: String,
    pub assigned_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddNoteRequest {
    pub note: String,
    pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct CloseInvestigationRequest {
    pub outcome: String, // "confirmed" or "dismissed"
}

#[derive(Debug, Serialize)]
pub struct InvestigationNoteResponse {
    pub note: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct InvestigationResponse {
    pub id: String,
    pub alert_id: String,
    pub status: String,
    pub assigned_to: Option<String>,
    pub notes: Vec<InvestigationNoteResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct InvestigationListResponse {
    pub data: Vec<InvestigationResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// --- Suspicion Report DTOs ---

#[derive(Debug, Deserialize)]
pub struct GenerateReportRequest {
    pub investigation_id: String,
    pub customer_info: String,
    pub transaction_details: String,
    pub reasons: String,
    pub evidence: Option<String>,
    pub timeline: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SuspicionReportResponse {
    pub id: String,
    pub investigation_id: String,
    pub customer_info: String,
    pub transaction_details: String,
    pub reasons: String,
    pub evidence: Option<String>,
    pub timeline: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
}

// --- Asset Freeze DTOs ---

#[derive(Debug, Deserialize)]
pub struct FreezeAccountRequest {
    pub account_id: String,
    pub reason: String,
    pub ordered_by: String,
}

#[derive(Debug, Deserialize)]
pub struct LiftFreezeRequest {
    pub lifted_by: String,
}

#[derive(Debug, Serialize)]
pub struct AssetFreezeResponse {
    pub id: String,
    pub account_id: String,
    pub reason: String,
    pub ordered_by: String,
    pub status: String,
    pub frozen_at: DateTime<Utc>,
    pub lifted_at: Option<DateTime<Utc>>,
    pub lifted_by: Option<String>,
}
