use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use banko_application::payment::*;

use crate::web::middleware::AuthenticatedUser;

// --- Query/Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct GetCardTransactionsQuery {
    pub card_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Handlers ---

/// POST /api/v1/cards
pub async fn request_card_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    body: web::Json<RequestCardRequest>,
) -> HttpResponse {
    match service.request_card(body.into_inner()).await {
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

/// GET /api/v1/cards?customer_id=X
pub async fn list_cards_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    match query.get("customer_id") {
        Some(customer_id) => match service.list_customer_cards(customer_id).await {
            Ok(resp) => HttpResponse::Ok().json(resp),
            Err(PaymentServiceError::InvalidInput(msg)) => {
                HttpResponse::BadRequest().json(ErrorResponse { error: msg })
            }
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            }),
        },
        None => HttpResponse::BadRequest().json(ErrorResponse {
            error: "customer_id query parameter is required".to_string(),
        }),
    }
}

/// GET /api/v1/cards/{id}
pub async fn get_card_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
) -> HttpResponse {
    match service.get_card(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
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

/// POST /api/v1/cards/{id}/activate
pub async fn activate_card_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
    body: web::Json<ActivateCardRequest>,
) -> HttpResponse {
    let mut request = body.into_inner();
    request.card_id = id.into_inner();

    match service.activate_card(request).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
            })
        }
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

/// POST /api/v1/cards/{id}/block
pub async fn block_card_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
) -> HttpResponse {
    match service.block_card(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
            })
        }
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

/// POST /api/v1/cards/{id}/unblock
pub async fn unblock_card_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
) -> HttpResponse {
    match service.unblock_card(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
            })
        }
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

/// DELETE /api/v1/cards/{id}
pub async fn cancel_card_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
) -> HttpResponse {
    match service.cancel_card(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
            })
        }
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

/// PUT /api/v1/cards/{id}/limits
pub async fn set_limits_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
    body: web::Json<SetCardLimitRequest>,
) -> HttpResponse {
    let mut request = body.into_inner();
    request.card_id = id.into_inner();

    match service.set_card_limits(request).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
            })
        }
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

/// GET /api/v1/cards/{id}/transactions
pub async fn card_transactions_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
) -> HttpResponse {
    match service.get_card_transactions(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(PaymentServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/cards/{id}/authorize
pub async fn authorize_transaction_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CardService>>,
    id: web::Path<String>,
    body: web::Json<AuthorizeTransactionRequest>,
) -> HttpResponse {
    let mut request = body.into_inner();
    request.card_id = id.into_inner();

    match service.authorize_transaction(request).await {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(PaymentServiceError::OrderNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Card not found".to_string(),
            })
        }
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
