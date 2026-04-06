use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::customer::{CustomerServiceError, DataRightsService};
use banko_domain::customer::DataRequestId;

use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize)]
pub struct RectificationRequest {
    pub details: String,
}

#[derive(Debug, Deserialize)]
pub struct OppositionRequest {
    pub purpose: String,
}

#[derive(Debug, Serialize)]
struct DataRightsRequestResponse {
    request_id: String,
    customer_id: String,
    request_type: String,
    status: String,
    details: Option<String>,
    response: Option<String>,
    requested_at: String,
    completed_at: Option<String>,
    deadline: String,
    is_overdue: bool,
}

fn map_service_error(err: CustomerServiceError) -> HttpResponse {
    match err {
        CustomerServiceError::CustomerNotFound => HttpResponse::NotFound().json(ErrorResponse {
            error: "Customer not found".to_string(),
        }),
        CustomerServiceError::Validation(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        CustomerServiceError::Domain(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        CustomerServiceError::Internal(msg) => {
            tracing::error!("Data rights internal error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

fn request_to_response(
    request: &banko_domain::customer::DataRightsRequest,
) -> DataRightsRequestResponse {
    DataRightsRequestResponse {
        request_id: request.request_id().to_string(),
        customer_id: request.customer_id().to_string(),
        request_type: request.request_type().as_str().to_string(),
        status: request.status().as_str().to_string(),
        details: request.details().map(|s| s.to_string()),
        response: request.response().map(|s| s.to_string()),
        requested_at: request.requested_at().to_rfc3339(),
        completed_at: request.completed_at().map(|d| d.to_rfc3339()),
        deadline: request.deadline().to_rfc3339(),
        is_overdue: request.is_overdue(),
    }
}

/// GET /api/v1/customers/{id}/data-export
pub async fn data_export_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<DataRightsService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.request_data_export(customer_id).await {
        Ok(request) => HttpResponse::Created().json(request_to_response(&request)),
        Err(err) => map_service_error(err),
    }
}

/// PUT /api/v1/customers/{id}/data-rectification
pub async fn data_rectification_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<DataRightsService>>,
    path: web::Path<String>,
    body: web::Json<RectificationRequest>,
) -> HttpResponse {
    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service
        .request_rectification(customer_id, body.details.clone())
        .await
    {
        Ok(request) => HttpResponse::Created().json(request_to_response(&request)),
        Err(err) => map_service_error(err),
    }
}

/// POST /api/v1/customers/{id}/data-opposition
pub async fn data_opposition_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<DataRightsService>>,
    path: web::Path<String>,
    body: web::Json<OppositionRequest>,
) -> HttpResponse {
    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service
        .request_opposition(customer_id, body.purpose.clone())
        .await
    {
        Ok(request) => HttpResponse::Created().json(request_to_response(&request)),
        Err(err) => map_service_error(err),
    }
}

/// GET /api/v1/customers/{id}/data-requests
pub async fn list_data_requests_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<DataRightsService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.list_requests(customer_id).await {
        Ok(requests) => {
            let responses: Vec<DataRightsRequestResponse> =
                requests.iter().map(request_to_response).collect();
            HttpResponse::Ok().json(responses)
        }
        Err(err) => map_service_error(err),
    }
}
