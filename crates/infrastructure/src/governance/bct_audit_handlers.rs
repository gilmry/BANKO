use std::sync::Arc;

use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use banko_application::governance::*;

use crate::web::middleware::AuthenticatedUser;

// --- Request / Query DTOs ---

#[derive(Debug, Deserialize)]
pub struct BctAuditQuery {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub ip_address: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BctIntegrityQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DailyTrendQuery {
    pub days: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    error: String,
}

// ============================================================
// BCT role check helper
// ============================================================

fn require_bct_inspector(auth: &AuthenticatedUser) -> Result<(), HttpResponse> {
    if auth.has_role("BCT_INSPECTOR") || auth.has_role("admin") || auth.has_role("superadmin") {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().json(ErrorResponse {
            error: "Forbidden: BCT_INSPECTOR role required".to_string(),
        }))
    }
}

// ============================================================
// BCT Audit Handlers (AUD-01)
// ============================================================

/// GET /api/v1/bct/audit/entries
pub async fn list_bct_entries_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<BctAuditService>>,
    query: web::Query<BctAuditQuery>,
) -> HttpResponse {
    if let Err(resp) = require_bct_inspector(&auth) {
        return resp;
    }

    let filter = BctAuditFilter {
        user_id: query
            .user_id
            .as_deref()
            .and_then(|s| Uuid::parse_str(s).ok()),
        action: query.action.clone(),
        resource_type: query.resource_type.clone(),
        resource_id: query
            .resource_id
            .as_deref()
            .and_then(|s| Uuid::parse_str(s).ok()),
        from: query
            .date_from
            .as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        to: query
            .date_to
            .as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        ip_address: query.ip_address.clone(),
        sort_by: query.sort_by.clone(),
        sort_order: query.sort_order.clone(),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service.get_audit_entries(&filter, page, limit).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/bct/audit/entries/export?format=csv|json
pub async fn export_entries_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<BctAuditService>>,
    query: web::Query<ExportQuery>,
) -> HttpResponse {
    if let Err(resp) = require_bct_inspector(&auth) {
        return resp;
    }

    let filter = BctAuditFilter {
        user_id: query
            .user_id
            .as_deref()
            .and_then(|s| Uuid::parse_str(s).ok()),
        action: query.action.clone(),
        resource_type: query.resource_type.clone(),
        resource_id: query
            .resource_id
            .as_deref()
            .and_then(|s| Uuid::parse_str(s).ok()),
        from: query
            .date_from
            .as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        to: query
            .date_to
            .as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        ip_address: None,
        sort_by: None,
        sort_order: None,
    };

    let format = query.format.as_deref().unwrap_or("json");
    match format {
        "csv" => match service.export_csv(&filter).await {
            Ok(resp) => HttpResponse::Ok()
                .content_type("text/csv")
                .insert_header((
                    "Content-Disposition",
                    "attachment; filename=\"audit_export.csv\"",
                ))
                .json(resp),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            }),
        },
        "json" => match service.export_json(&filter).await {
            Ok(resp) => HttpResponse::Ok().json(resp),
            Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            }),
        },
        _ => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid export format. Use 'csv' or 'json'.".to_string(),
        }),
    }
}

/// GET /api/v1/bct/audit/integrity
pub async fn integrity_check_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<BctAuditService>>,
    query: web::Query<BctIntegrityQuery>,
) -> HttpResponse {
    if let Err(resp) = require_bct_inspector(&auth) {
        return resp;
    }

    let now = Utc::now();
    let from = query
        .from
        .as_deref()
        .and_then(|s| s.parse::<DateTime<Utc>>().ok())
        .unwrap_or(now - chrono::Duration::days(30));
    let to = query
        .to
        .as_deref()
        .and_then(|s| s.parse::<DateTime<Utc>>().ok())
        .unwrap_or(now);

    match service.verify_chain_integrity(from, to).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// ============================================================
// Dashboard Handlers (AUD-02)
// ============================================================

/// GET /api/v1/bct/dashboard/stats
pub async fn dashboard_stats_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<AuditDashboardService>>,
) -> HttpResponse {
    if let Err(resp) = require_bct_inspector(&auth) {
        return resp;
    }

    match service.get_dashboard_stats().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/bct/dashboard/daily-trend?days=30
pub async fn daily_trend_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<AuditDashboardService>>,
    query: web::Query<DailyTrendQuery>,
) -> HttpResponse {
    if let Err(resp) = require_bct_inspector(&auth) {
        return resp;
    }

    let days = query.days.unwrap_or(30).clamp(1, 365);

    match service.get_entries_per_day(days).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/bct/dashboard/suspicious
pub async fn suspicious_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<AuditDashboardService>>,
) -> HttpResponse {
    if let Err(resp) = require_bct_inspector(&auth) {
        return resp;
    }

    match service.get_recent_suspicious().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}
