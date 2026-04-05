use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use banko_application::fx::*;

use crate::web::middleware::AuthenticatedUser;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct ListFxOperationsQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Handlers ---

/// POST /api/v1/fx/operations
pub async fn create_fx_operation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<FxService>>,
    body: web::Json<CreateFxOperationRequest>,
) -> HttpResponse {
    match service.create_operation(body.into_inner()).await {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(FxServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(FxServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(FxServiceError::DailyLimitExceeded(msg)) => {
            HttpResponse::Forbidden().json(ErrorResponse { error: msg })
        }
        Err(FxServiceError::RateNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Exchange rate not found for this currency pair".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/fx/operations
pub async fn list_fx_operations_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<FxService>>,
    query: web::Query<ListFxOperationsQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let status = query.status.as_deref();

    match service.list_operations(status, page, limit).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/fx/operations/{id}
pub async fn get_fx_operation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<FxService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid operation ID".to_string(),
            })
        }
    };

    match service.get_operation(id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(FxServiceError::OperationNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "FX operation not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/fx/operations/{id}/confirm
pub async fn confirm_fx_operation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<FxService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid operation ID".to_string(),
            })
        }
    };

    match service.confirm_operation(id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(FxServiceError::OperationNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "FX operation not found".to_string(),
        }),
        Err(FxServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/fx/operations/{id}/settle
pub async fn settle_fx_operation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<FxService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid operation ID".to_string(),
            })
        }
    };

    match service.settle_operation(id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(FxServiceError::OperationNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "FX operation not found".to_string(),
        }),
        Err(FxServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/fx/rates
pub async fn list_rates_handler(
    _auth: AuthenticatedUser,
    rate_service: web::Data<Arc<RateService>>,
) -> HttpResponse {
    match rate_service.list_rates().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// PUT /api/v1/fx/rates
pub async fn update_rate_handler(
    _auth: AuthenticatedUser,
    rate_service: web::Data<Arc<RateService>>,
    body: web::Json<UpdateRateRequest>,
) -> HttpResponse {
    match rate_service.update_rate(body.into_inner()).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(FxServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/fx/positions
pub async fn get_positions_handler(
    _auth: AuthenticatedUser,
    position_service: web::Data<Arc<PositionService>>,
) -> HttpResponse {
    match position_service.get_position_summary().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/fx/limits/{account_id}
pub async fn get_daily_limits_handler(
    _auth: AuthenticatedUser,
    limits_service: web::Data<Arc<FxLimitsService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account ID".to_string(),
            })
        }
    };

    match limits_service.get_remaining_limits(account_id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}
