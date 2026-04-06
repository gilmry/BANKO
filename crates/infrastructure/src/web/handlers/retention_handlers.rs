use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Serialize;

use banko_application::customer::{RetentionService, RetentionServiceError};

use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

fn map_retention_error(err: RetentionServiceError) -> HttpResponse {
    match err {
        RetentionServiceError::CustomerNotFound => HttpResponse::NotFound().json(ErrorResponse {
            error: "Customer not found".to_string(),
        }),
        RetentionServiceError::Domain(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        RetentionServiceError::Internal(msg) => {
            tracing::error!("Retention internal error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// POST /api/v1/admin/retention/run — triggers anonymization job (admin only)
pub async fn run_anonymization_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<RetentionService>>,
) -> HttpResponse {
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ErrorResponse {
            error: "Admin access required".to_string(),
        });
    }

    match service.run_anonymization_job().await {
        Ok(report) => {
            tracing::info!(
                "INV-10: Anonymization job completed — checked: {}, anonymized: {}, errors: {}",
                report.count_checked,
                report.count_anonymized,
                report.errors.len()
            );
            HttpResponse::Ok().json(report)
        }
        Err(err) => map_retention_error(err),
    }
}

/// GET /api/v1/admin/retention/status/{customer_id} — check retention for specific customer
pub async fn check_retention_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<RetentionService>>,
    path: web::Path<String>,
) -> HttpResponse {
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ErrorResponse {
            error: "Admin access required".to_string(),
        });
    }

    let customer_id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.check_retention(customer_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(err) => map_retention_error(err),
    }
}
