use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Murabaha DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMurabahaContractRequest {
    pub customer_id: String,
    pub cost_price: f64,
    pub profit_margin: f64,
    pub installments: u32,
    pub asset_description: String,
    pub delivery_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MurabahaContractResponse {
    pub id: String,
    pub customer_id: String,
    pub cost_price: f64,
    pub profit_margin: f64,
    pub selling_price: f64,
    pub installments: u32,
    pub asset_description: String,
    pub delivery_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Ijara DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIjaraContractRequest {
    pub customer_id: String,
    pub asset_id: String,
    pub monthly_rental: f64,
    pub lease_start: DateTime<Utc>,
    pub lease_end: DateTime<Utc>,
    pub purchase_option_price: f64,
    pub maintenance_responsibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IjaraContractResponse {
    pub id: String,
    pub customer_id: String,
    pub asset_id: String,
    pub monthly_rental: f64,
    pub lease_start: DateTime<Utc>,
    pub lease_end: DateTime<Utc>,
    pub purchase_option_price: f64,
    pub maintenance_responsibility: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Musharaka DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMusharakaContractRequest {
    pub customer_id: String,
    pub total_capital: f64,
    pub bank_share_pct: f64,
    pub client_share_pct: f64,
    pub profit_sharing_ratio: f64,
    pub diminishing_schedule: Vec<(u32, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusharakaContractResponse {
    pub id: String,
    pub customer_id: String,
    pub total_capital: f64,
    pub bank_share_pct: f64,
    pub client_share_pct: f64,
    pub profit_sharing_ratio: f64,
    pub diminishing_schedule: Vec<(u32, f64)>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Mudaraba DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMudarabaContractRequest {
    pub customer_id: String,
    pub capital_amount: f64,
    pub profit_sharing_ratio: f64,
    pub investment_type: String,
    pub reporting_period: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MudarabaContractResponse {
    pub id: String,
    pub customer_id: String,
    pub capital_amount: f64,
    pub profit_sharing_ratio: f64,
    pub investment_type: String,
    pub reporting_period: u32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Sukuk DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSukukIssuanceRequest {
    pub denomination: f64,
    pub total_amount: f64,
    pub units_issued: u64,
    pub coupon_rate: f64,
    pub maturity_date: DateTime<Utc>,
    pub underlying_asset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SukukIssuanceResponse {
    pub id: String,
    pub denomination: f64,
    pub total_amount: f64,
    pub units_issued: u64,
    pub coupon_rate: f64,
    pub maturity_date: DateTime<Utc>,
    pub underlying_asset: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Zakat DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateZakatRequest {
    pub customer_id: String,
    pub assessment_year: u32,
    pub nisab_threshold: f64,
    pub eligible_wealth: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZakatCalculationResponse {
    pub id: String,
    pub customer_id: String,
    pub assessment_year: u32,
    pub nisab_threshold: f64,
    pub eligible_wealth: f64,
    pub zakat_amount: f64,
    pub payment_status: String,
    pub created_at: DateTime<Utc>,
}

// --- Sharia Board DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShariaBoardDecisionRequest {
    pub product_type: String,
    pub ruling: String,
    pub conditions: Vec<String>,
    pub board_members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShariaBoardDecisionResponse {
    pub id: String,
    pub product_type: String,
    pub ruling: String,
    pub conditions: Vec<String>,
    pub board_members: Vec<String>,
    pub quorum_met: bool,
    pub decision_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// --- Profit Distribution DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfitDistributionRequest {
    pub period: u32,
    pub total_profit: f64,
    pub depositor_pool_share: f64,
    pub bank_share: f64,
    pub per_account_distributions: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitDistributionResponse {
    pub id: String,
    pub period: u32,
    pub total_profit: f64,
    pub depositor_pool_share: f64,
    pub bank_share: f64,
    pub per_account_distributions: Vec<(String, f64)>,
    pub created_at: DateTime<Utc>,
}

// --- Shared DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveContractRequest {
    pub contract_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateContractRequest {
    pub contract_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murabaha_request_serialization() {
        let req = CreateMurabahaContractRequest {
            customer_id: "cust-001".to_string(),
            cost_price: 1000.0,
            profit_margin: 0.15,
            installments: 12,
            asset_description: "Car".to_string(),
            delivery_date: Utc::now(),
        };
        let json = serde_json::to_string(&req);
        assert!(json.is_ok());
    }

    #[test]
    fn test_murabaha_response_serialization() {
        let resp = MurabahaContractResponse {
            id: "muraba-001".to_string(),
            customer_id: "cust-001".to_string(),
            cost_price: 1000.0,
            profit_margin: 0.15,
            selling_price: 1150.0,
            installments: 12,
            asset_description: "Car".to_string(),
            delivery_date: Utc::now(),
            status: "Proposed".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let json = serde_json::to_string(&resp);
        assert!(json.is_ok());
    }

    #[test]
    fn test_zakat_calculation_request() {
        let req = CalculateZakatRequest {
            customer_id: "cust-001".to_string(),
            assessment_year: 2026,
            nisab_threshold: 1000.0,
            eligible_wealth: 5000.0,
        };
        let json = serde_json::to_string(&req);
        assert!(json.is_ok());
    }

    #[test]
    fn test_sharia_board_decision_request() {
        let req = CreateShariaBoardDecisionRequest {
            product_type: "Murabaha".to_string(),
            ruling: "Halal".to_string(),
            conditions: vec![],
            board_members: vec!["Dr. Ahmed".to_string(), "Dr. Fatima".to_string(), "Dr. Ali".to_string()],
        };
        let json = serde_json::to_string(&req);
        assert!(json.is_ok());
    }

    #[test]
    fn te