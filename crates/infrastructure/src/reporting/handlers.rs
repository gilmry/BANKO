use std::sync::Arc;

use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use banko_application::reporting::*;
use banko_domain::reporting::{ReportId, ReportStatus, ReportType};

use crate::web::middleware::AuthenticatedUser;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct GenerateReportBody {
    pub report_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct ListReportsQuery {
    pub report_type: Option<String>,
    pub status: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Ifrs9Query {
    pub as_of: Option<NaiveDate>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Report Handlers ---

/// GET /api/v1/reporting/forms
pub async fn list_reports_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
    query: web::Query<ListReportsQuery>,
) -> HttpResponse {
    let report_type = match &query.report_type {
        Some(rt) => match ReportType::from_str_type(rt) {
            Ok(t) => Some(t),
            Err(_) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid report_type".to_string(),
                })
            }
        },
        None => None,
    };

    let status = match &query.status {
        Some(s) => match ReportStatus::from_str_type(s) {
            Ok(st) => Some(st),
            Err(_) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid status".to_string(),
                })
            }
        },
        None => None,
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service
        .list_reports(report_type, status, query.from, query.to, page, limit)
        .await
    {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/reporting/forms/{id}
pub async fn get_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid report ID".to_string(),
            })
        }
    };

    match service.get_report(&ReportId::from_uuid(id)).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(ReportingServiceError::ReportNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Report not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/reporting/generate
pub async fn generate_report_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
    body: web::Json<GenerateReportBody>,
) -> HttpResponse {
    let report_type = match ReportType::from_str_type(&body.report_type) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid report_type".to_string(),
            })
        }
    };

    let generated_by = match Uuid::parse_str(&auth.user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })
        }
    };

    match service
        .generate_report(report_type, body.period_start, body.period_end, generated_by)
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(ReportingServiceError::NoActiveTemplate) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "No active template for this report type".to_string(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/reporting/forms/{id}/validate
pub async fn validate_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid report ID".to_string(),
            })
        }
    };

    match service.validate_report(&ReportId::from_uuid(id)).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(ReportingServiceError::ReportNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Report not found".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/reporting/forms/{id}/submit
pub async fn submit_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid report ID".to_string(),
            })
        }
    };

    match service.submit_report(&ReportId::from_uuid(id)).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(ReportingServiceError::ReportNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Report not found".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/reporting/forms/{id}/acknowledge
pub async fn acknowledge_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid report ID".to_string(),
            })
        }
    };

    match service.acknowledge_report(&ReportId::from_uuid(id)).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(ReportingServiceError::ReportNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Report not found".to_string(),
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/reporting/templates
pub async fn list_templates_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ReportingService>>,
) -> HttpResponse {
    match service.list_templates().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/reporting/ifrs9
pub async fn ifrs9_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<Ifrs9ReportService>>,
    query: web::Query<Ifrs9Query>,
) -> HttpResponse {
    let as_of = query
        .as_of
        .unwrap_or_else(|| chrono::Utc::now().date_naive());

    let resp = service.generate_ifrs9_report(as_of);
    HttpResponse::Ok().json(resp)
}
