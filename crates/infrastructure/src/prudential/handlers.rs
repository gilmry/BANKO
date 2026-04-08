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

// --- Trend Handler (mock) ---

/// GET /api/v1/prudential/ratios/{id}/trend
pub async fn get_ratio_trend_handler(
    path: web::Path<String>,
) -> HttpResponse {
    let _ratio_id = path.into_inner();

    // Generate 30 days of realistic trend data
    let mut data_points = Vec::with_capacity(30);
    let base_value: f64 = 18.0;

    for i in 0u32..30 {
        // Simulate a gentle upward trend with small fluctuations
        let variation = match i % 7 {
            0 => 0.0,
            1 => 0.3,
            2 => -0.1,
            3 => 0.2,
            4 => 0.5,
            5 => -0.2,
            6 => 0.1,
            _ => 0.0,
        };
        let trend_offset = f64::from(i) * 0.05;
        let value = ((base_value + trend_offset + variation) * 10.0).round() / 10.0;

        let date = if 10 + i > 31 {
            format!("2026-04-{:02}", 10 + i - 31)
        } else {
            format!("2026-03-{:02}", 10 + i)
        };

        data_points.push(serde_json::json!({
            "date": date,
            "value": value
        }));
    }

    HttpResponse::Ok().json(data_points)
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
