use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::payment::*;

use crate::web::middleware::AuthenticatedUser;

// --- Error Response ---

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Standing Order Handlers ---

/// POST /api/v1/recurring/standing-orders
pub async fn create_standing_order_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    body: web::Json<CreateStandingOrderRequest>,
) -> HttpResponse {
    match service.create_standing_order(body.into_inner()).await {
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

/// GET /api/v1/recurring/standing-orders/{id}
pub async fn get_standing_order_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    match service.get_standing_order(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Standing order not found".to_string(),
            })
        }
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/recurring/standing-orders?account_id={id}
#[derive(Debug, Deserialize)]
pub struct ListStandingOrdersQuery {
    pub account_id: String,
}

pub async fn list_standing_orders_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    query: web::Query<ListStandingOrdersQuery>,
) -> HttpResponse {
    match service
        .list_account_standing_orders(&query.account_id)
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

/// POST /api/v1/recurring/standing-orders/{id}/suspend
pub async fn suspend_standing_order_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    match service.suspend_standing_order(&id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"success": true})),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Standing order not found".to_string(),
            })
        }
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/recurring/standing-orders/{id}/resume
pub async fn resume_standing_order_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    match service.resume_standing_order(&id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"success": true})),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Standing order not found".to_string(),
            })
        }
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/recurring/standing-orders/{id}/cancel
pub async fn cancel_standing_order_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    match service.cancel_standing_order(&id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"success": true})),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Standing order not found".to_string(),
            })
        }
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- Direct Debit Mandate Handlers ---

/// POST /api/v1/recurring/mandates
pub async fn create_mandate_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    body: web::Json<CreateMandateRequest>,
) -> HttpResponse {
    match service.create_mandate(body.into_inner()).await {
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

/// POST /api/v1/recurring/mandates/{id}/sign
pub async fn sign_mandate_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    match service.sign_mandate(&id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"success": true})),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Mandate not found".to_string(),
            })
        }
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/recurring/mandates/{id}/revoke
pub async fn revoke_mandate_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    match service.revoke_mandate(&id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"success": true})),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Mandate not found".to_string(),
            })
        }
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/recurring/mandates?debtor_account_id={id}
#[derive(Debug, Deserialize)]
pub struct ListMandatesQuery {
    pub debtor_account_id: String,
}

pub async fn list_mandates_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<RecurringPaymentService>>,
    query: web::Query<ListMandatesQuery>,
) -> HttpResponse {
    match service
        .list_account_mandates(&query.debtor_account_id)
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
