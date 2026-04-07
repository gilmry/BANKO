use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateCollateralRequest {
    pub collateral_type: String,
    pub description: String,
    pub market_value: f64,
    pub haircut_pct: f64,
    pub valuation_date: String, // ISO 8601 date
    pub customer_id: String,
    pub insurance_policy_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RevalueCollateralRequest {
    pub collateral_id: String,
    pub new_market_value: f64,
    pub new_haircut_pct: f64,
    pub valuation_date: String, // ISO 8601 date
    pub appraiser: String,
    pub valuation_method: String,
}

#[derive(Debug, Deserialize)]
pub struct AllocateCollateralRequest {
    pub collateral_id: String,
    pub loan_id: String,
    pub allocated_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CalculateLtvRequest {
    pub loan_id: String,
    pub total_loan_amount: f64,
    pub collateral_ids: Vec<String>,
    pub collateral_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActivateCollateralRequest {
    pub collateral_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseCollateralRequest {
    pub collateral_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInsurancePolicyRequest {
    pub collateral_id: String,
    pub insurance_policy_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ListCollateralsQuery {
    pub customer_id: Option<String>,
    pub status: Option<String>,
    pub collateral_type: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct CollateralResponse {
    pub id: String,
    pub collateral_type: String,
    pub description: String,
    pub market_value: f64,
    pub haircut_pct: f64,
    pub net_value: f64,
    pub valuation_date: String,
    pub next_revaluation_date: String,
    pub status: String,
    pub customer_id: String,
    pub insurance_policy_id: Option<String>,
    pub is_revaluation_due: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CollateralValuationResponse {
    pub valuation_id: String,
    pub collateral_id: String,
    pub valuation_date: String,
    pub market_value: f64,
    pub appraiser: String,
    pub method: String,
    pub next_revaluation_date: String,
    pub is_revaluation_due: bool,
}

#[derive(Debug, Serialize)]
pub struct CollateralAllocationResponse {
    pub collateral_id: String,
    pub loan_id: String,
    pub allocated_amount: f64,
    pub allocation_date: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LtvCalculationResponse {
    pub loan_id: String,
    pub collateral_ids: Vec<String>,
    pub total_loan_amount: f64,
    pub total_collateral_value: f64,
    pub ltv_ratio: f64,
    pub max_ltv_threshold: f64,
    pub is_compliant: bool,
}

#[derive(Debug, Serialize)]
pub struct PaginatedCollateralsResponse {
    pub data: Vec<CollateralResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct CollateralDetailResponse {
    pub collateral: CollateralResponse,
    pub allocations: Vec<CollateralAllocationResponse>,
    pub latest_valuation: Option<CollateralValuationResponse>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_collateral_request_serialization() {
        let json = r#"
        {
            "collateral_type": "real_estate",
            "description": "Residential property",
            "market_value": 100000.0,
            "haircut_pct": 0.0,
            "valuation_date": "2024-01-01",
            "customer_id": "cust-123",
            "insurance_policy_id": "POL-001"
        }
        "#;

        let req: CreateCollateralRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.collateral_type, "real_estate");
        assert_eq!(req.market_value, 100000.0);
        assert!(req.insurance_policy_id.is_some());
    }

    #[test]
    fn test_revalue_collateral_request_serialization() {
        let json = r#"
        {
            "collateral_id": "coll-123",
            "new_market_value": 95000.0,
            "new_haircut_pct": 0.05,
            "valuation_date": "2024-04-01",
            "appraiser": "John Appraiser",
            "valuation_method": "market_comparison"
        }
        "#;

        let req: RevalueCollateralRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.new_market_value, 95000.0);
        assert_eq!(req.appraiser, "John Appraiser");
    }

    #[test]
    fn test_allocate_collateral_request_serialization() {
        let json = r#"
        {
            "collateral_id": "coll-123",
            "loan_id": "loan-456",
            "allocated_amount": 50000.0
        }
        "#;

        let req: AllocateCollateralRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.loan_id, "loan-456");
        assert_eq!(req.allocated_amount, 50000.0);
    }

    #[test]
    fn test_calculate_ltv_request_serialization() {
        let json = r#"
        {
            "loan_id": "loan-456",
            "total_loan_amount": 70000.0,
            "collateral_ids": ["coll-123"],
            "collateral_types": ["real_estate"]
        }
        "#;

        let req: CalculateLtvRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.total_loan_amount, 70000.0);
        assert_eq!(req.collateral_ids.len(), 1);
    }

    #[test]
    fn test_collateral_response_serialization() {
        let response = CollateralResponse {
            id: "coll-123".to_string(),
            collateral_type: "RealEstate".to_string(),
            description: "Residential property".to_string(),
            market_value: 100000.0,
            haircut_pct: 0.0,
            net_value: 100000.0,
            valuation_date: "2024-01-01".to_string(),
            next_revaluation_date: "2025-01-01".to_string(),
            status: "Active".to_string(),
            customer_id: "cust-123".to_string(),
            insurance_policy_id: Some("POL-001".to_string()),
            is_revaluation_due: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("coll-123"));
        assert!(json.contains("RealEstate"));
    }

    #[test]
    fn test_ltv_calculation_response_serialization() {
        let response = LtvCalculationResponse {
            loan_id: "loan-456".to_string(),
            collateral_ids: vec!["coll-123".to_string()],
            total_loan_amount: 70000.0,
            total_collateral_value: 100000.0,
            ltv_ratio: 0.70,
            max_ltv_threshold: 0.70,
            is_compliant: true,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("0.7"));
        assert!(json.contains("true"));
    }
}
