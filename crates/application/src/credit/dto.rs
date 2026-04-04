use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoanApplicationRequest {
    pub account_id: String,
    pub amount: f64,
    pub interest_rate: Option<f64>,
    pub term_months: u32,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveLoanRequest {
    pub loan_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DisburseLoanRequest {
    pub loan_id: String,
    pub disbursement_date: Option<String>,
    pub frequency: Option<String>,
    pub amortization_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoanResponse {
    pub id: String,
    pub customer_id: String,
    pub account_id: String,
    pub amount: f64,
    pub interest_rate: f64,
    pub term_months: u32,
    pub currency: String,
    pub asset_class: String,
    pub provision_amount: f64,
    pub provision_rate: f64,
    pub status: String,
    pub days_past_due: u32,
    pub disbursement_date: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct InstallmentResponse {
    pub id: String,
    pub installment_number: u32,
    pub due_date: String,
    pub principal_amount: f64,
    pub interest_amount: f64,
    pub total_amount: f64,
    pub remaining_balance: f64,
    pub paid: bool,
    pub paid_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoanDetailResponse {
    pub loan: LoanResponse,
    pub schedule: Option<Vec<InstallmentResponse>>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedLoansResponse {
    pub data: Vec<LoanResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListLoansQuery {
    pub status: Option<String>,
    pub asset_class: Option<String>,
    pub account_id: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
