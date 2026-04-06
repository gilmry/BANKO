use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use banko_application::sanctions::*;
use banko_domain::sanctions::{ListSource, ScreeningResultId};

use crate::web::middleware::AuthenticatedUser;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct ScreeningQuery {
    pub name: String,
    pub threshold: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct ListResultsQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Screening Handlers ---

/// GET /api/v1/sanctions/check?name=X&threshold=85
pub async fn screen_name_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<SanctionsScreeningService>>,
    query: web::Query<ScreeningQuery>,
) -> HttpResponse {
    if query.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "name parameter is required".to_string(),
        });
    }

    match service.screen_name(&query.name, query.threshold).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/sanctions/results
pub async fn list_results_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<SanctionsScreeningService>>,
    query: web::Query<ListResultsQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service.list_results(page, limit).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/sanctions/results/{id}
pub async fn get_result_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<SanctionsScreeningService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => ScreeningResultId::from_uuid(id),
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.get_result(&id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(SanctionsServiceError::ResultNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Screening result not found".to_string(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- List Handlers ---

/// GET /api/v1/sanctions/lists
pub async fn list_sanctions_lists_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ListSyncService>>,
) -> HttpResponse {
    match service.get_all_lists_status().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/sanctions/lists/{source}
pub async fn get_list_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ListSyncService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let source = match ListSource::from_str_source(&path.into_inner()) {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid source. Use: UN, EU, OFAC, National".to_string(),
            })
        }
    };

    match service.get_list_status(source).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(SanctionsServiceError::ListNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "List not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- Dashboard Handler ---

/// GET /api/v1/sanctions/dashboard
pub async fn dashboard_handler(
    _auth: AuthenticatedUser,
    screening_service: web::Data<Arc<SanctionsScreeningService>>,
    list_service: web::Data<Arc<ListSyncService>>,
) -> HttpResponse {
    let stats = match screening_service.get_screening_stats().await {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    };

    let lists = match list_service.get_all_lists_status().await {
        Ok(l) => l,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    };

    HttpResponse::Ok().json(ScreeningStatsResponse {
        total_screenings: stats.total_screenings,
        hits: stats.hits,
        potential_matches: stats.potential_matches,
        clear: stats.clear,
        lists,
    })
}
