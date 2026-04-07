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
    pub segment: Option<String>,  // FR-006
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

// --- FR-014: Multi-criteria Search DTO ---

#[derive(Debug, Deserialize, Clone)]
pub struct CustomerSearchQuery {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub cin_or_rcs: Option<String>,
    pub customer_type: Option<String>,
    pub status: Option<String>,
    pub segment: Option<String>,  // FR-006
    pub pep_status: Option<String>,
    pub risk_level: Option<String>,
    pub phone: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub min_risk_score: Option<u8>,
    pub max_risk_score: Option<u8>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl CustomerSearchQuery {
    pub fn new() -> Self {
        CustomerSearchQuery {
            full_name: None,
            email: None,
            cin_or_rcs: None,
            customer_type: None,
            status: None,
            segment: None,
            pep_status: None,
            risk_level: None,
            phone: None,
            city: None,
            country: None,
            min_risk_score: None,
            max_risk_score: None,
            limit: Some(100),
            offset: Some(0),
        }
    }

    pub fn with_full_name(mut self, name: String) -> Self {
        self.full_name = Some(name);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_segment(mut self, segment: String) -> Self {
        self.segment = Some(segment);
        self
    }

    pub fn with_risk_score_range(mut self, min: u8, max: u8) -> Self {
        self.min_risk_score = Some(min);
        self.max_risk_score = Some(max);
        self
    }

    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(100)
    }

    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }
}

impl Default for CustomerSearchQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize)]
pub struct CustomerSearchResult {
    pub customer_id: String,
    pub customer_type: String,
    pub full_name: String,
    pub email: String,
    pub status: String,
    pub segment: String,  // FR-006
    pub risk_score: u8,
    pub risk_level: String,
}

#[derive(Debug, Serialize)]
pub struct CustomerSearchResponse {
    pub total_count: i64,
    pub results: Vec<CustomerSearchResult>,
    pub limit: i64,
    pub offset: i64,
}
