use std::sync::Arc;

use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use banko_application::payment::{
    ChequeService, IssueChequeRequest, ChequeResponse, OpposeChequeRequest, ClearingResultRequest,
};
use banko_application::payment::PaymentServiceError;

use crate::web::middleware::AuthenticatedUser;

// ============================================================
// Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct CreateChequeRequest {
    pub account_id: String,
    pub drawer_name: String,
    pub beneficiary_name: String,
    pub amount: Decimal,
    pub cheque_type: String,
}

#[derive(Debug, Serialize)]
pub struct ChequeDetailResponse {
    pub id: String,
    pub cheque_number: String,
    pub account_id: String,
    pub drawer_name: String,
    pub beneficiary_name: String,
    pub amount: Decimal,
    pub currency: String,
    pub cheque_type: String,
    pub status: String,
    pub rejection_reason: Option<String>,
    pub opposition_reason: Option<String>,
    pub issue_date: String,
    pub expiry_date: String,
    pub encashed_at: Option<String>,
    pub presented_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ChequeListResponse {
    pub data: Vec<ChequeDetailResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct OppositionRequest {
    pub reason: String,
    pub is_legal: bool,
}

#[derive(Debug, Deserialize)]
pub struct ClearingResultsRequest {
    pub results: Vec<ClearingResultRequest>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

// ============================================================
// Helper Functions
// ============================================================

fn cheque_response_to_detail(cheque: &ChequeResponse) -> ChequeDetailResponse {
    ChequeDetailResponse {
        id: cheque.id.clone(),
        cheque_number: cheque.cheque_number.clone(),
        account_id: cheque.account_id.clone(),
        drawer_name: cheque.drawer_name.clone(),
        beneficiary_name: cheque.beneficiary_name.clone(),
        amount: cheque.amount,
        currency: cheque.currency.clone(),
        cheque_type: cheque.cheque_type.clone(),
        status: cheque.status.clone(),
        rejection_reason: cheque.rejection_reason.clone(),
        opposition_reason: cheque.opposition_reason.clone(),
        issue_date: cheque.issue_date.to_string(),
        expiry_date: cheque.expiry_date.to_string(),
        encashed_at: cheque.encashed_at.map(|dt| dt.to_rfc3339()),
        presented_at: cheque.presented_at.map(|dt| dt.to_rfc3339()),
        created_at: cheque.created_at.to_rfc3339(),
    }
}

fn error_response(error: PaymentServiceError) -> HttpResponse {
    match error {
        PaymentServiceError::OrderNotFound => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Cheque not found".to_string(),
                details: None,
            })
        }
        PaymentServiceError::InvalidInput(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid input".to_string(),
                details: Some(msg),
            })
        }
        PaymentServiceError::Internal(msg) => {
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                details: Some(msg),
            })
        }
        PaymentServiceError::DomainError(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Domain error".to_string(),
                details: Some(msg),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Unknown error".to_string(),
            details: None,
        }),
    }
}

// ============================================================
// HTTP Handlers
// ============================================================

/// POST /api/v1/cheques
/// Issue a new cheque
pub async fn issue_cheque_handler(
    service: web::Data<Arc<ChequeService>>,
    req: web::Json<CreateChequeRequest>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let cheque_req = IssueChequeRequest {
        account_id: req.account_id.clone(),
        drawer_name: req.drawer_name.clone(),
        beneficiary_name: req.beneficiary_name.clone(),
        amount: req.amount,
        cheque_type: req.cheque_type.clone(),
    };

    match service.issue_cheque(cheque_req).await {
        Ok(cheque) => HttpResponse::Created().json(cheque_response_to_detail(&cheque)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/cheques
/// List cheques for an account
pub async fn list_cheques_handler(
    service: web::Data<Arc<ChequeService>>,
    query: web::Query<std::collections::HashMap<String, String>>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let account_id = match query.get("account_id") {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Missing required parameter: account_id".to_string(),
                details: None,
            })
        }
    };

    match service.list_cheques(account_id).await {
        Ok(cheques) => {
            let data: Vec<ChequeDetailResponse> =
                cheques.iter().map(cheque_response_to_detail).collect();
            HttpResponse::Ok().json(ChequeListResponse {
                total: data.len(),
                data,
            })
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/cheques/{id}
/// Get a specific cheque
pub async fn get_cheque_handler(
    _service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let _cheque_id = path.into_inner();

    // Need to retrieve cheque - would require additional method
    // For now, return not implemented
    HttpResponse::NotImplemented().json(ErrorResponse {
        error: "Get cheque endpoint requires repository implementation".to_string(),
        details: None,
    })
}

/// POST /api/v1/cheques/{id}/present
/// Present a cheque for processing
pub async fn present_cheque_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let cheque_id = path.into_inner();

    match service.present_cheque(&cheque_id).await {
        Ok(cheque) => HttpResponse::Ok().json(cheque_response_to_detail(&cheque)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/cheques/{id}/encash
/// Encash a cheque
pub async fn encash_cheque_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let cheque_id = path.into_inner();

    match service.encash_cheque(&cheque_id).await {
        Ok(cheque) => HttpResponse::Ok().json(cheque_response_to_detail(&cheque)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/cheques/{id}/oppose
/// Oppose a cheque
pub async fn oppose_cheque_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    req: web::Json<OppositionRequest>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let cheque_id = path.into_inner();

    let opposition_req = OpposeChequeRequest {
        cheque_id,
        reason: req.reason.clone(),
        is_legal: req.is_legal,
    };

    match service.oppose_cheque(opposition_req).await {
        Ok(cheque) => HttpResponse::Ok().json(cheque_response_to_detail(&cheque)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/cheques/{id}/reject
/// Reject a cheque
pub async fn reject_cheque_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    req: web::Json<serde_json::Value>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let cheque_id = path.into_inner();

    let reason = match req.get("reason").and_then(|v| v.as_str()) {
        Some(r) => r.to_string(),
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Missing required field: reason".to_string(),
                details: None,
            })
        }
    };

    match service.reject_cheque(&cheque_id, reason).await {
        Ok(cheque) => HttpResponse::Ok().json(cheque_response_to_detail(&cheque)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/cheques/blacklist/{customer_id}
/// Check blacklist status for a customer
pub async fn blacklist_status_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let customer_id = path.into_inner();

    match service.check_blacklist_status(&customer_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/cheques/blacklist/{customer_id}/lift
/// Lift a blacklist
pub async fn lift_blacklist_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let customer_id = path.into_inner();

    match service.lift_blacklist(&customer_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/cheques/clearing/generate
/// Generate a clearing batch
pub async fn generate_clearing_handler(
    service: web::Data<Arc<ChequeService>>,
    req: web::Json<serde_json::Value>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let date_str = match req.get("clearing_date").and_then(|v| v.as_str()) {
        Some(d) => d,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Missing required field: clearing_date".to_string(),
                details: None,
            })
        }
    };

    let clearing_date = match chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid date format, expected YYYY-MM-DD".to_string(),
                details: None,
            })
        }
    };

    match service.generate_clearing_batch(clearing_date).await {
        Ok(batch) => HttpResponse::Created().json(batch),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/cheques/clearing/{batch_id}/results
/// Process clearing results
pub async fn clearing_results_handler(
    service: web::Data<Arc<ChequeService>>,
    path: web::Path<String>,
    req: web::Json<ClearingResultsRequest>,
    _user: AuthenticatedUser,
) -> HttpResponse {
    let batch_id = path.into_inner();

    match service.process_clearing_results(&batch_id, req.results.clone()).await {
        Ok(batch) => HttpResponse::Ok().json(batch),
        Err(e) => error_response(e),
    }
}

// ============================================================
// Route Configuration
// ============================================================

pub fn configure_cheque_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cheques")
            .route("", web::post().to(issue_cheque_handler))
            .route("", web::get().to(list_cheques_handler))
            .route("/{id}", web::get().to(get_cheque_handler))
            .route("/{id}/present", web::post().to(present_cheque_handler))
            .route("/{id}/encash", web::post().to(encash_cheque_handler))
            .route("/{id}/oppose", web::post().to(oppose_cheque_handler))
            .route("/{id}/reject", web::post().to(reject_cheque_handler))
            .route("/blacklist/{customer_id}", web::get().to(blacklist_status_handler))
            .route("/blacklist/{customer_id}/lift", web::post().to(lift_blacklist_handler))
            .route("/clearing/generate", web::post().to(generate_clearing_handler))
            .route("/clearing/{batch_id}/results", web::post().to(clearing_results_handler)),
    );
}
