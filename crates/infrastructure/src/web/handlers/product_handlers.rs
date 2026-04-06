use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_application::product::{
    CreatePricingGridRequest, CreateProductRequest, EligibilityCheckRequest, ProductService,
    ProductServiceError, InterestCalculationService,
};

use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductBody {
    pub name: String,
    pub product_type: String,
    pub interest_rate: Option<banko_application::product::CreateInterestRateDto>,
    pub fees: Vec<banko_application::product::CreateFeeDto>,
    pub eligibility: banko_application::product::CreateEligibilityDto,
    pub segment_pricing: Option<std::collections::HashMap<String, rust_decimal::Decimal>>,
    pub min_balance: Option<rust_decimal::Decimal>,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct CalculatePriceBody {
    pub product_id: String,
    pub customer_segment: String,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize)]
pub struct EligibilityCheckBody {
    pub product_id: Option<String>,
    pub age: u8,
    pub income: rust_decimal::Decimal,
    pub segment: String,
    pub credit_score: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePricingGridBody {
    pub product_id: String,
    pub bands: Vec<banko_application::product::CreatePricingBandDto>,
    pub effective_from: String,
    pub effective_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CalculateDailyInterestBody {
    pub account_balance: rust_decimal::Decimal,
    pub annual_rate: rust_decimal::Decimal,
    pub calc_method: String,
}

#[derive(Debug, Deserialize)]
pub struct CalculateMaturityBody {
    pub principal: rust_decimal::Decimal,
    pub annual_rate: rust_decimal::Decimal,
    pub months: u32,
    pub currency: String,
}

fn map_service_error(err: ProductServiceError) -> HttpResponse {
    match err {
        ProductServiceError::ProductNotFound => HttpResponse::NotFound().json(ErrorResponse {
            error: "Product not found".to_string(),
        }),
        ProductServiceError::InvalidInput(msg) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Invalid input: {}", msg),
        }),
        ProductServiceError::DomainError(msg) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Domain error: {}", msg),
        }),
        ProductServiceError::RepositoryError(msg) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Repository error: {}", msg),
        }),
        ProductServiceError::InvalidStatus(msg) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Invalid status: {}", msg),
        }),
        ProductServiceError::EligibilityCheckFailed(reasons) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Eligibility check failed: {}", reasons.join(", ")),
        }),
        ProductServiceError::PricingGridNotFound => HttpResponse::NotFound().json(ErrorResponse {
            error: "Pricing grid not found".to_string(),
        }),
    }
}

/// Create a new product (STORY-PROD-01)
pub async fn create_product_handler(
    req: web::Json<CreateProductBody>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let request = CreateProductRequest {
        name: req.name.clone(),
        product_type: req.product_type.clone(),
        interest_rate: req.interest_rate.clone(),
        fees: req.fees.clone(),
        eligibility: req.eligibility.clone(),
        segment_pricing: req.segment_pricing.clone(),
        min_balance: req.min_balance,
        currency: req.currency.clone(),
    };

    match service.create_product(request).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => map_service_error(e),
    }
}

/// Get a product by ID
pub async fn get_product_handler(
    path: web::Path<String>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let product_id_str = path.into_inner();
    let product_id = match Uuid::parse_str(&product_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid product ID".to_string(),
            })
        }
    };

    match service.get_product(product_id).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => map_service_error(e),
    }
}

/// List all products
pub async fn list_products_handler(
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    match service.list_products().await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => map_service_error(e),
    }
}

/// Activate a product
pub async fn activate_product_handler(
    path: web::Path<String>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let product_id_str = path.into_inner();
    let product_id = match Uuid::parse_str(&product_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid product ID".to_string(),
            })
        }
    };

    match service.activate_product(product_id).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => map_service_error(e),
    }
}

/// Suspend a product
pub async fn suspend_product_handler(
    path: web::Path<String>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let product_id_str = path.into_inner();
    let product_id = match Uuid::parse_str(&product_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid product ID".to_string(),
            })
        }
    };

    match service.suspend_product(product_id).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => map_service_error(e),
    }
}

/// Calculate price quote for a product
pub async fn calculate_price_handler(
    req: web::Json<CalculatePriceBody>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let product_id = match Uuid::parse_str(&req.product_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid product ID".to_string(),
            })
        }
    };

    match service
        .calculate_price(product_id, &req.customer_segment, req.amount)
        .await
    {
        Ok(quote) => HttpResponse::Ok().json(quote),
        Err(e) => map_service_error(e),
    }
}

/// Check customer eligibility for a product or get eligible products
pub async fn check_eligibility_handler(
    req: web::Json<EligibilityCheckBody>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let request = EligibilityCheckRequest {
        product_id: req.product_id.clone(),
        age: req.age,
        income: req.income,
        segment: req.segment.clone(),
        credit_score: req.credit_score,
    };

    match service.check_eligibility(request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => map_service_error(e),
    }
}

/// Get eligible products for a customer
pub async fn get_eligible_products_handler(
    req: web::Json<EligibilityCheckBody>,
    service: web::Data<ProductService>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let request = EligibilityCheckRequest {
        product_id: None, // Ignore product_id for this endpoint
        age: req.age,
        income: req.income,
        segment: req.segment.clone(),
        credit_score: req.credit_score,
    };

    match service.get_eligible_products(request).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => map_service_error(e),
    }
}

/// Create a pricing grid for a product
pub async fn create_pricing_grid_handler(
    req: web::Json<CreatePricingGridBody>,
    _service: web::Data<Arc<dyn banko_application::product::IPricingGridRepository>>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    // Note: In a real implementation, this would use the pricing grid repository
    // For now, we return a placeholder response
    let request = CreatePricingGridRequest {
        product_id: req.product_id.clone(),
        bands: req.bands.clone(),
        effective_from: req.effective_from.clone(),
        effective_to: req.effective_to.clone(),
    };

    HttpResponse::Created().json(serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "product_id": request.product_id,
        "bands": request.bands,
        "effective_from": request.effective_from,
        "effective_to": request.effective_to,
        "active": true,
        "created_by": "admin",
        "created_at": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Calculate daily interest
pub async fn calculate_daily_interest_handler(
    req: web::Json<CalculateDailyInterestBody>,
) -> HttpResponse {
    match InterestCalculationService::calculate_daily_interest(
        req.account_balance,
        req.annual_rate,
        &req.calc_method,
    ) {
        Ok(interest) => HttpResponse::Ok().json(serde_json::json!({
            "account_balance": req.account_balance,
            "annual_rate": req.annual_rate,
            "calc_method": req.calc_method,
            "daily_interest": interest,
        })),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

/// Calculate term deposit maturity
pub async fn calculate_maturity_handler(
    req: web::Json<CalculateMaturityBody>,
) -> HttpResponse {
    match InterestCalculationService::calculate_term_deposit_maturity(
        req.principal,
        req.annual_rate,
        req.months,
        &req.currency,
    ) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}
