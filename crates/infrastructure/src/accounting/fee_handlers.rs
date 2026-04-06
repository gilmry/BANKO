use std::sync::Arc;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_application::accounting::{
    FeeService, ChargeResult, AccountingServiceError,
};
use banko_domain::accounting::{
    FeeCategory, FeeCharge, FeeCondition, FeeDefinition, FeeGrid,
};
use std::collections::HashMap;

// ==================== DTOs ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeeDefinitionRequest {
    pub name: String,
    pub category: String,
    pub fixed_amount: Option<Decimal>,
    pub rate_percent: Option<Decimal>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub condition_type: String, // "Always", "BalanceBelow", etc.
    pub condition_value: Option<Decimal>,
    pub applicable_segments: Vec<String>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeeGridRequest {
    pub name: String,
    pub segment: String,
    pub fee_overrides: HashMap<String, Decimal>,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateMonthlyFeesRequest {
    pub product_id: String,
    pub balance: Decimal,
    pub segment: String,
    pub day_of_month: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeMonthlyFeesRequest {
    pub fees: Vec<String>, // Fee IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeDefinitionResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub fixed_amount: Option<Decimal>,
    pub rate_percent: Option<Decimal>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeChargeResponse {
    pub id: String,
    pub fee_definition_id: String,
    pub account_id: String,
    pub amount: Decimal,
    pub status: String,
    pub charged_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeGridResponse {
    pub id: String,
    pub name: String,
    pub segment: String,
    pub fee_overrides: HashMap<String, Decimal>,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeResultResponse {
    pub total_charged: usize,
    pub total_unpaid: usize,
    pub amount_charged: Decimal,
    pub amount_unpaid: Decimal,
}

// ==================== Handlers ====================

/// Create a new fee definition
pub async fn create_fee_definition_handler(
    service: web::Data<Arc<FeeService>>,
    body: web::Json<CreateFeeDefinitionRequest>,
) -> HttpResponse {
    let req = body.into_inner();

    // Parse category
    let category = match FeeCategory::from_str(&req.category) {
        Some(cat) => cat,
        None => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Invalid fee category"}))
        }
    };

    // Parse condition
    let condition = parse_fee_condition(&req.condition_type, req.condition_value);

    // Create definition
    match FeeDefinition::new(
        req.name,
        category,
        req.fixed_amount,
        req.rate_percent,
        req.min_amount,
        req.max_amount,
        condition,
        req.applicable_segments,
        req.currency,
    ) {
        Ok(def) => {
            let service_clone = service.get_ref().clone();
            match tokio::task::spawn_blocking(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(service_clone.create_fee_definition(def))
            })
            .await
            {
                Ok(Ok(created)) => HttpResponse::Created().json(fee_def_to_response(&created)),
                Ok(Err(AccountingServiceError::InvalidEntry(msg))) => {
                    HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
                }
                Ok(Err(e)) => {
                    HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": e.to_string()}))
                }
                Err(e) => {
                    HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": e.to_string()}))
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()})),
    }
}

/// List all fee definitions
pub async fn list_fee_definitions_handler(
    service: web::Data<Arc<FeeService>>,
) -> HttpResponse {
    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.list_fee_definitions())
    })
    .await
    {
        Ok(Ok(defs)) => {
            let response: Vec<FeeDefinitionResponse> =
                defs.iter().map(|d| fee_def_to_response(d)).collect();
            HttpResponse::Ok().json(response)
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// Get a fee definition by ID
pub async fn get_fee_definition_handler(
    service: web::Data<Arc<FeeService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id_str = path.into_inner();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid UUID: {}", e)}))
        }
    };

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.get_fee_definition(id))
    })
    .await
    {
        Ok(Ok(Some(def))) => HttpResponse::Ok().json(fee_def_to_response(&def)),
        Ok(Ok(None)) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Fee definition not found"}))
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// List account fees
pub async fn list_account_fees_handler(
    service: web::Data<Arc<FeeService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id_str = path.into_inner();
    let account_id = match Uuid::parse_str(&account_id_str) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid UUID: {}", e)}))
        }
    };

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.list_account_fees(account_id))
    })
    .await
    {
        Ok(Ok(charges)) => {
            let response: Vec<FeeChargeResponse> =
                charges.iter().map(|c| fee_charge_to_response(c)).collect();
            HttpResponse::Ok().json(response)
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// Create a fee grid
pub async fn create_fee_grid_handler(
    service: web::Data<Arc<FeeService>>,
    body: web::Json<CreateFeeGridRequest>,
) -> HttpResponse {
    let req = body.into_inner();

    // Convert fee_overrides to HashMap<FeeCategory, Decimal>
    let mut overrides = HashMap::new();
    for (cat_str, amount) in req.fee_overrides {
        if let Some(cat) = FeeCategory::from_str(&cat_str) {
            overrides.insert(cat, amount);
        }
    }

    let grid = FeeGrid::new(req.name, req.segment, overrides, req.effective_from, req.effective_to);

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.create_fee_grid(grid))
    })
    .await
    {
        Ok(Ok(created)) => HttpResponse::Created().json(fee_grid_to_response(&created)),
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// List fee grids
pub async fn list_fee_grids_handler(
    service: web::Data<Arc<FeeService>>,
) -> HttpResponse {
    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.list_fee_grids())
    })
    .await
    {
        Ok(Ok(grids)) => {
            let response: Vec<FeeGridResponse> =
                grids.iter().map(|g| fee_grid_to_response(g)).collect();
            HttpResponse::Ok().json(response)
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// Get fee grid for segment
pub async fn get_fee_grid_handler(
    service: web::Data<Arc<FeeService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let segment = path.into_inner();

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.get_fee_grid_for_segment(&segment))
    })
    .await
    {
        Ok(Ok(Some(grid))) => HttpResponse::Ok().json(fee_grid_to_response(&grid)),
        Ok(Ok(None)) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Fee grid not found"}))
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

// ==================== Helpers ====================

fn parse_fee_condition(condition_type: &str, value: Option<Decimal>) -> FeeCondition {
    match condition_type {
        "Always" => FeeCondition::Always,
        "BalanceBelow" => {
            FeeCondition::BalanceBelow(value.unwrap_or(Decimal::ZERO))
        }
        "TransactionAbove" => {
            FeeCondition::TransactionAbove(value.unwrap_or(Decimal::ZERO))
        }
        "MonthDay" => {
            let day = value
                .and_then(|v| v.to_u32())
                .unwrap_or(1)
                .min(31) as u8;
            FeeCondition::MonthDay(day)
        }
        "EndOfMonth" => FeeCondition::EndOfMonth,
        _ => FeeCondition::Always,
    }
}

fn fee_def_to_response(def: &FeeDefinition) -> FeeDefinitionResponse {
    FeeDefinitionResponse {
        id: def.id().to_string(),
        name: def.name().to_string(),
        category: def.category().as_str().to_string(),
        fixed_amount: None, // Would need accessor in domain
        rate_percent: None,
        min_amount: None,
        max_amount: None,
        currency: def.currency().to_string(),
    }
}

fn fee_charge_to_response(charge: &FeeCharge) -> FeeChargeResponse {
    FeeChargeResponse {
        id: charge.id().to_string(),
        fee_definition_id: charge.fee_definition_id().to_string(),
        account_id: charge.account_id().to_string(),
        amount: charge.amount(),
        status: charge.status().as_str().to_string(),
        charged_at: charge.charged_at(),
        description: charge.description().map(|s| s.to_string()),
    }
}

fn fee_grid_to_response(grid: &FeeGrid) -> FeeGridResponse {
    let mut overrides = HashMap::new();
    for (cat, amount) in grid.fee_overrides().iter() {
        overrides.insert(cat.as_str().to_string(), *amount);
    }

    FeeGridResponse {
        id: grid.id().to_string(),
        name: grid.name().to_string(),
        segment: grid.segment().to_string(),
        fee_overrides: overrides,
        effective_from: Utc::now(), // Would need accessor in domain
        effective_to: None,
        active: grid.active(),
    }
}
