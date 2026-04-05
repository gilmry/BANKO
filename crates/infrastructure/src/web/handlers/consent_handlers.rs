use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::customer::{ConsentService, CustomerServiceError};

use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize)]
pub struct GrantConsentRequest {
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct RevokeConsentRequest {
    pub purpose: String,
}

#[derive(Debug, Serialize)]
struct ConsentResponse {
    consent_id: String,
    customer_id: String,
    purpose: String,
    status: String,
    granted_at: String,
    revoked_at: Option<String>,
}

fn map_service_error(err: CustomerServiceError) -> HttpResponse {
    match err {
        CustomerServiceError::CustomerNotFound => HttpResponse::NotFound().json(ErrorResponse {
            error: "Customer not found".to_string(),
        }),
        CustomerServiceError::Validation(msg) => HttpResponse::BadRequest().json(ErrorResponse {
            error: msg,
        }),
        CustomerServiceError::Domain(msg) => HttpResponse::BadRequest().json(ErrorResponse {
            error: msg,
        }),
        CustomerServiceError::Internal(msg) => {
            tracing::error!("Consent internal error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

fn consent_to_response(
    consent: &banko_domain::customer::ConsentRecord,
) -> ConsentResponse {
    ConsentResponse {
        consent_id: consent.consent_id().to_string(),
        customer_id: consent.customer_id().to_string(),
        purpose: consent.purpose().as_str().to_string(),
        status: consent.status().as_str().to_string(),
        granted_at: consent.granted_at().to_rfc3339(),
        revoked_at: consent.revoked_at().map(|d| d.to_rfc3339()),
    }
}

/// POST /api/v1/customers/{id}/consent
pub async fn grant_consent_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<ConsentService>>,
    path: web::Path<String>,
    body: web::Json<GrantConsentRequest>,
) -> HttpResponse {
    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.grant_consent(customer_id, &body.purpose).await {
        Ok(consent) => HttpResponse::Created().json(consent_to_response(&consent)),
        Err(err) => map_service_error(err),
    }
}

/// DELETE /api/v1/customers/{id}/consent
pub async fn revoke_consent_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<ConsentService>>,
    path: web::Path<String>,
    body: web::Json<RevokeConsentRequest>,
) -> HttpResponse {
    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.revoke_consent(customer_id, &body.purpose).await {
        Ok(consent) => HttpResponse::Ok().json(consent_to_response(&consent)),
        Err(err) => map_service_error(err),
    }
}

/// GET /api/v1/customers/{id}/consents
pub async fn list_consents_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<ConsentService>>,
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

    match service.list_consents(customer_id).await {
        Ok(consents) => {
            let responses: Vec<ConsentResponse> =
                consents.iter().map(consent_to_response).collect();
            HttpResponse::Ok().json(responses)
        }
        Err(err) => map_service_error(err),
    }
}
