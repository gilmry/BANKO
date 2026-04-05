use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub sender_account_id: String,
    pub beneficiary_name: String,
    pub beneficiary_rib: Option<String>,
    pub beneficiary_bic: Option<String>,
    pub amount: i64,
    pub currency: Option<String>,
    pub payment_type: String,
    pub reference: String,
    pub description: Option<String>,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: String,
    pub sender_account_id: String,
    pub beneficiary_name: String,
    pub beneficiary_rib: Option<String>,
    pub beneficiary_bic: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub payment_type: String,
    pub status: String,
    pub screening_status: String,
    pub reference: String,
    pub description: Option<String>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PaymentListResponse {
    pub data: Vec<PaymentResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct PaymentStatusResponse {
    pub id: String,
    pub status: String,
    pub screening_status: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SwiftMessageResponse {
    pub message_id: String,
    pub order_id: String,
    pub message_type: String,
    pub sender_bic: String,
    pub receiver_bic: String,
    pub amount: i64,
    pub currency: String,
    pub reference: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ClearingBatchResponse {
    pub processed: usize,
    pub cleared: usize,
    pub failed: usize,
}
