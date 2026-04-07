use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Insurance Policy DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInsurancePolicyRequest {
    pub policy_type: String,
    pub customer_id: String,
    pub provider_name: String,
    pub policy_number: String,
    pub premium_amount: f64,
    pub currency: String,
    pub premium_frequency: String,
    pub coverage_amount: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub beneficiaries: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsurancePolicyResponse {
    pub id: String,
    pub policy_type: String,
    pub customer_id: String,
    pub provider_name: String,
    pub policy_number: String,
    pub premium_amount: f64,
    pub currency: String,
    pub premium_frequency: String,
    pub coverage_amount: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub beneficiaries: Vec<String>,
    pub status: String,
    pub is_expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivatePolicyRequest {
    pub id: String,
}

// --- Insurance Claim DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInsuranceClaimRequest {
    pub policy_id: String,
    pub claim_date: DateTime<Utc>,
    pub claim_amount: f64,
    pub currency: String,
    pub description: String,
    pub documents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceClaimResponse {
    pub id: String,
    pub policy_id: String,
    pub claim_date: DateTime<Utc>,
    pub claim_amount: f64,
    pub currency: String,
    pub description: String,
    pub documents: Vec<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApproveClaimRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RejectClaimRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayClaimRequest {
    pub id: String,
}

// --- Bancassurance Product DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBancassuranceProductRequest {
    pub insurance_provider: String,
    pub product_name: String,
    pub product_type: String,
    pub commission_rate: f64,
    pub is_mandatory: bool,
    pub linked_product_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BancassuranceProductResponse {
    pub id: String,
    pub insurance_provider: String,
    pub product_name: String,
    pub product_type: String,
    pub commission_rate: f64,
    pub is_mandatory: bool,
    pub linked_product_types: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Insurance Commission DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInsuranceCommissionRequest {
    pub policy_id: String,
    pub product_id: String,
    pub amount: f64,
    pub currency: String,
    pub calculation_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceCommissionResponse {
    pub id: String,
    pub policy_id: String,
    pub product_id: String,
    pub amount: f64,
    pub currency: String,
    pub calculation_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayCommissionRequest {
    pub id: String,
}
