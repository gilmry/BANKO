use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
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

#[derive(Debug, Serialize)]
pub struct ScreeningBatchResponse {
    pub screened: usize,
    pub cleared: usize,
    pub hits: usize,
}

// --- Standing Order DTOs (STORY-RECUR-01) ---

#[derive(Debug, Deserialize)]
pub struct CreateStandingOrderRequest {
    pub account_id: String,
    pub beneficiary_account: String,
    pub beneficiary_name: String,
    pub amount: Decimal,
    pub currency: Option<String>,
    pub frequency: String,
    pub reference: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub max_executions: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct StandingOrderResponse {
    pub id: String,
    pub account_id: String,
    pub beneficiary_account: String,
    pub beneficiary_name: String,
    pub amount: Decimal,
    pub currency: String,
    pub frequency: String,
    pub reference: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub next_execution_date: NaiveDate,
    pub status: String,
    pub execution_count: u32,
    pub max_executions: Option<u32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StandingOrderListResponse {
    pub data: Vec<StandingOrderResponse>,
    pub total: usize,
}

// --- Direct Debit Mandate DTOs (STORY-RECUR-02) ---

#[derive(Debug, Deserialize)]
pub struct CreateMandateRequest {
    pub debtor_account_id: String,
    pub debtor_name: String,
    pub creditor_id: String,
    pub creditor_name: String,
    pub amount_limit: Decimal,
    pub currency: Option<String>,
    pub frequency: String,
    pub reference: String,
    pub expires_at: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct MandateResponse {
    pub id: String,
    pub debtor_account_id: String,
    pub debtor_name: String,
    pub creditor_id: String,
    pub creditor_name: String,
    pub amount_limit: Decimal,
    pub currency: String,
    pub frequency: String,
    pub reference: String,
    pub signed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<NaiveDate>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MandateListResponse {
    pub data: Vec<MandateResponse>,
    pub total: usize,
}

// --- Batch Execution Result (STORY-RECUR-02 + RECUR-03) ---

#[derive(Debug, Serialize, Clone)]
pub struct BatchExecutionResult {
    pub total: usize,
    pub executed: usize,
    pub failed: usize,
    pub skipped: usize,
}
