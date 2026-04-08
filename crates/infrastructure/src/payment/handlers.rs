use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use banko_application::payment::*;

use crate::web::middleware::AuthenticatedUser;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct ListPaymentsQuery {
    pub account_id: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RejectBody {
    pub reason: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Handlers ---

/// POST /api/v1/payments/transfers
pub async fn create_payment_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    body: web::Json<CreatePaymentRequest>,
) -> HttpResponse {
    match service.create_payment(body.into_inner()).await {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(PaymentServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/payments
pub async fn list_payments_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    query: web::Query<ListPaymentsQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service
        .list_payments(
            query.account_id.as_deref(),
            query.status.as_deref(),
            page,
            limit,
        )
        .await
    {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/payments/{id}
pub async fn get_payment_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    if Uuid::parse_str(&id).is_err() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid payment ID".to_string(),
        });
    }

    match service.get_payment(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Payment order not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/payments/{id}/status
pub async fn get_payment_status_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();

    match service.get_payment_status(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Payment order not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/payments/{id}/screen
pub async fn screen_payment_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();

    match service.screen_payment(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Payment order not found".to_string(),
        }),
        Err(PaymentServiceError::PaymentBlocked(msg)) => {
            HttpResponse::Forbidden().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/payments/{id}/submit
pub async fn submit_payment_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();

    match service.submit_payment(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Payment order not found".to_string(),
        }),
        Err(PaymentServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/payments/{id}/execute
pub async fn execute_payment_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<PaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();

    match service.execute_payment(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Payment order not found".to_string(),
        }),
        Err(PaymentServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/payments/clearing
pub async fn run_clearing_handler(
    _auth: AuthenticatedUser,
    clearing: web::Data<Arc<ClearingService>>,
) -> HttpResponse {
    match clearing.run_clearing_batch().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- Stats Handlers (mock) ---

/// GET /api/v1/payments/status
pub async fn get_payments_status_handler(_auth: AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "total_payments": 1847,
        "pending_count": 45,
        "screened_count": 12,
        "submitted_count": 8,
        "executed_count": 1723,
        "failed_count": 34,
        "rejected_count": 25,
        "total_volume": 45672890.50,
        "average_amount": 24728.42
    }))
}

/// GET /api/v1/payments/clearing/status
pub async fn get_clearing_status_handler(_auth: AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "operational",
        "last_sync": "2026-04-08T14:30:00Z",
        "pending_submissions": 3,
        "failed_submissions": 0
    }))
}
