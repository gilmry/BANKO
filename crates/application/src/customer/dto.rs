use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub customer_type: String,
    pub full_name: String,
    pub cin: Option<String>,
    pub birth_date: Option<String>,
    pub nationality: Option<String>,
    pub profession: Option<String>,
    pub address: AddressDto,
    pub phone: String,
    pub email: String,
    pub pep_status: Option<String>,
    pub source_of_funds: Option<String>,
    pub consent: bool,
    // LegalEntity specific
    pub registration_number: Option<String>,
    pub sector: Option<String>,
    pub beneficiaries: Option<Vec<BeneficiaryDto>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDto {
    pub street: String,
    pub city: String,
    pub postal_code: String,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeneficiaryDto {
    pub full_name: String,
    pub share_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct CustomerResponse {
    pub customer_id: String,
    pub customer_type: String,
    pub status: String,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub risk_score: u8,
    pub risk_level: String,
    pub consent: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct KycProfileResponse {
    pub full_name: String,
    pub cin_or_rcs: String,
    pub birth_date: Option<String>,
    pub nationality: String,
    pub profession: String,
    pub address: AddressDto,
    pub phone: String,
    pub email: String,
    pub pep_status: String,
    pub source_of_funds: String,
    pub sector: Option<String>,
    pub submission_date: Option<String>,
    pub approval_date: Option<String>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateCustomerResponse {
    pub customer_id: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKycRequest {
    pub full_name: String,
    pub cin: Option<String>,
    pub birth_date: Option<String>,
    pub nationality: Option<String>,
    pub profession: Option<String>,
    pub address: AddressDto,
    pub phone: String,
    pub email: String,
    pub pep_status: Option<String>,
    pub source_of_funds: Option<String>,
    pub registration_number: Option<String>,
    pub sector: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataExportPackage {
    pub customer_id: String,
    pub customer_type: String,
    pub profile: serde_json::Value,
    pub accounts: serde_json::Value,
    pub transactions: serde_json::Value,
    pub consents: serde_json::Value,
    pub export_date: String,
}
