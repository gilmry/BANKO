use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Letter of Credit DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLetterOfCreditRequest {
    pub lc_type: String,
    pub applicant_id: String,
    pub beneficiary_name: String,
    pub issuing_bank: String,
    pub advising_bank: String,
    pub amount: f64,
    pub currency: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub terms_description: String,
    pub documents_required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LetterOfCreditResponse {
    pub id: String,
    pub lc_type: String,
    pub applicant_id: String,
    pub beneficiary_name: String,
    pub issuing_bank: String,
    pub advising_bank: String,
    pub amount: f64,
    pub currency: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub terms_description: String,
    pub documents_required: Vec<String>,
    pub status: String,
    pub is_expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueLetterOfCreditRequest {
    pub id: String,
}

// --- Bank Guarantee DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBankGuaranteeRequest {
    pub guarantee_type: String,
    pub principal_id: String,
    pub beneficiary_name: String,
    pub amount: f64,
    pub currency: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub claim_conditions: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankGuaranteeResponse {
    pub id: String,
    pub guarantee_type: String,
    pub principal_id: String,
    pub beneficiary_name: String,
    pub amount: f64,
    pub currency: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub claim_conditions: String,
    pub status: String,
    pub is_expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallBankGuaranteeRequest {
    pub id: String,
}

// --- Documentary Collection DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDocumentaryCollectionRequest {
    pub collection_type: String,
    pub exporter_id: String,
    pub importer_name: String,
    pub amount: f64,
    pub currency: String,
    pub documents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentaryCollectionResponse {
    pub id: String,
    pub collection_type: String,
    pub exporter_id: String,
    pub importer_name: String,
    pub amount: f64,
    pub currency: String,
    pub documents: Vec<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Trade Finance Limit DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTradeFinanceLimitRequest {
    pub customer_id: String,
    pub limit_type: String,
    pub total_limit: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeFinanceLimitResponse {
    pub id: String,
    pub customer_id: String,
    pub limit_type: String,
    pub total_limit: f64,
    pub utilized: f64,
    pub available: f64,
    pub currency: String,
    pub utilization_rate: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UtilizeTradeFinanceLimitRequest {
    pub id: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseTradeFinanceLimitRequest {
    pub id: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncreaseLimitRequest {
    pub id: String,
    pub increase: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecreaseLimitRequest {
    pub id: String,
    pub decrease: f64,
}
