use std::sync::Arc;

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use banko_application::prudential::{
    CalculateRatiosRequest, PrudentialServiceError, RatioCalculationService,
};

pub async fn get_ratios_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    query: web::Query<InstitutionQuery>,
) -> HttpResponse {
    let institution_id = match Uuid::parse_str(&query.institution_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service.get_current_ratios(institution_id).await {
        Ok(ratios) => HttpResponse::Ok().json(ratios),
        Err(PrudentialServiceError::RatioNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Ratios not found"}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn calculate_ratios_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    body: web::Json<CalculateRatiosRequest>,
) -> HttpResponse {
    match service.calculate_and_save(body.into_inner()).await {
        Ok(ratios) => HttpResponse::Created().json(ratios),
        Err(PrudentialServiceError::DomainError(msg)) => {
            HttpResponse::UnprocessableEntity().json(serde_json::json!({"error": msg}))
        }
        Err(PrudentialServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn check_solvency_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    query: web::Query<InstitutionQuery>,
) -> HttpResponse {
    let institution_id = match Uuid::parse_str(&query.institution_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service.check_solvency(institution_id).await {
        Ok(check) => HttpResponse::Ok().json(check),
        Err(PrudentialServiceError::RatioNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Ratios not found"}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn check_tier1_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    query: web::Query<InstitutionQuery>,
) -> HttpResponse {
    let institution_id = match Uuid::parse_str(&query.institution_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service.check_tier1(institution_id).await {
        Ok(check) => HttpResponse::Ok().json(check),
        Err(PrudentialServiceError::RatioNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Ratios not found"}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn check_credit_deposit_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    query: web::Query<InstitutionQuery>,
) -> HttpResponse {
    let institution_id = match Uuid::parse_str(&query.institution_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service.check_credit_deposit(institution_id).await {
        Ok(check) => HttpResponse::Ok().json(check),
        Err(PrudentialServiceError::RatioNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Ratios not found"}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn check_concentration_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    query: web::Query<ConcentrationQuery>,
) -> HttpResponse {
    let institution_id = match Uuid::parse_str(&query.institution_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };
    let beneficiary_id = match Uuid::parse_str(&query.beneficiary_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service
        .check_concentration(institution_id, beneficiary_id)
        .await
    {
        Ok(check) => HttpResponse::Ok().json(check),
        Err(PrudentialServiceError::RatioNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Ratios not found"}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn get_alerts_handler(
    service: web::Data<Arc<RatioCalculationService>>,
    query: web::Query<AlertsQuery>,
) -> HttpResponse {
    let institution_id = query
        .institution_id
        .as_ref()
        .map(|s| Uuid::parse_str(s))
        .transpose();

    let institution_id = match institution_id {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    match service
        .get_breach_alerts(institution_id, limit, offset)
        .await
    {
        Ok(alerts) => HttpResponse::Ok().json(alerts),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

// --- Query types ---

#[derive(serde::Deserialize)]
pub struct InstitutionQuery {
    pub institution_id: String,
}

#[derive(serde::Deserialize)]
pub struct ConcentrationQuery {
    pub institution_id: String,
    pub beneficiary_id: String,
}

#[derive(serde::Deserialize)]
pub struct AlertsQuery {
    pub institution_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
