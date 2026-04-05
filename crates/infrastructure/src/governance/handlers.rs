use std::sync::Arc;

use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use banko_application::governance::*;

use crate::web::middleware::AuthenticatedUser;

// --- Request / Query DTOs ---

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct IntegrityQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommitteeBody {
    pub name: String,
    pub committee_type: String,
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordDecisionBody {
    pub subject: String,
    pub decision: String,
    pub votes: Vec<VoteBody>,
    pub justification: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VoteBody {
    pub member_id: String,
    pub vote: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateControlBody {
    pub operation_type: String,
    pub operation_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ApproveBody {
    pub checker_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RejectBody {
    pub checker_id: String,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ControlsQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    error: String,
}

// ============================================================
// Audit Handlers
// ============================================================

/// GET /api/v1/audit
pub async fn list_audit_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<AuditService>>,
    query: web::Query<AuditQuery>,
) -> HttpResponse {
    let filters = AuditFilter {
        user_id: query.user_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        action: query.action.clone(),
        resource_type: query.resource_type.clone(),
        resource_id: query.resource_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        from: query.from.as_deref().and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        to: query.to.as_deref().and_then(|s| s.parse::<DateTime<Utc>>().ok()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service.get_audit_trail(filters, page, limit).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/audit/integrity
pub async fn verify_integrity_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<AuditService>>,
    query: web::Query<IntegrityQuery>,
) -> HttpResponse {
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

    match service.verify_integrity(from, to).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// ============================================================
// Committee Handlers (GOV-07)
// ============================================================

/// GET /api/v1/governance/committees
pub async fn list_committees_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CommitteeService>>,
) -> HttpResponse {
    match service.list_committees().await {
        Ok(committees) => HttpResponse::Ok().json(committees),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/governance/committees
pub async fn create_committee_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CommitteeService>>,
    body: web::Json<CreateCommitteeBody>,
) -> HttpResponse {
    let members: Result<Vec<Uuid>, _> = body
        .members
        .iter()
        .map(|s| Uuid::parse_str(s))
        .collect();

    let members = match members {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid member UUID".to_string(),
            })
        }
    };

    match service
        .create_committee(body.name.clone(), body.committee_type.clone(), members)
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(GovernanceServiceError::InvalidInput(e)) | Err(GovernanceServiceError::DomainError(e)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: e })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/governance/committees/{id}/decisions
pub async fn record_decision_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<CommitteeService>>,
    path: web::Path<String>,
    body: web::Json<RecordDecisionBody>,
) -> HttpResponse {
    let committee_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid committee_id".to_string(),
            })
        }
    };

    let votes: Result<Vec<(Uuid, String)>, _> = body
        .votes
        .iter()
        .map(|v| Uuid::parse_str(&v.member_id).map(|id| (id, v.vote.clone())))
        .collect();

    let votes = match votes {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid member_id in vote".to_string(),
            })
        }
    };

    match service
        .record_decision(
            committee_id,
            body.subject.clone(),
            body.decision.clone(),
            votes,
            body.justification.clone(),
        )
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(GovernanceServiceError::CommitteeNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Committee not found".to_string(),
            })
        }
        Err(GovernanceServiceError::InvalidInput(e)) | Err(GovernanceServiceError::DomainError(e)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: e })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// ============================================================
// Control Check Handlers (GOV-08)
// ============================================================

/// GET /api/v1/governance/controls
pub async fn list_controls_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ControlService>>,
    query: web::Query<ControlsQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service
        .list_checks(query.status.clone(), page, limit)
        .await
    {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(GovernanceServiceError::InvalidInput(e)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: e })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/governance/controls
pub async fn create_control_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ControlService>>,
    body: web::Json<CreateControlBody>,
) -> HttpResponse {
    let operation_id = match Uuid::parse_str(&body.operation_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid operation_id".to_string(),
            })
        }
    };

    match service
        .create_check(body.operation_type.clone(), operation_id)
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(GovernanceServiceError::DomainError(e)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: e })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/governance/controls/{id}/approve
pub async fn approve_control_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ControlService>>,
    path: web::Path<String>,
    body: web::Json<ApproveBody>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid control check id".to_string(),
            })
        }
    };

    let checker_id = match Uuid::parse_str(&body.checker_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid checker_id".to_string(),
            })
        }
    };

    match service.approve(id, checker_id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(GovernanceServiceError::ControlCheckNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Control check not found".to_string(),
            })
        }
        Err(GovernanceServiceError::DomainError(e)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: e })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/governance/controls/{id}/reject
pub async fn reject_control_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ControlService>>,
    path: web::Path<String>,
    body: web::Json<RejectBody>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid control check id".to_string(),
            })
        }
    };

    let checker_id = match Uuid::parse_str(&body.checker_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid checker_id".to_string(),
            })
        }
    };

    match service.reject(id, checker_id, body.reason.clone()).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(GovernanceServiceError::ControlCheckNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Control check not found".to_string(),
            })
        }
        Err(GovernanceServiceError::DomainError(e)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: e })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// ============================================================
// Compliance Report Handler (GOV-06)
// ============================================================

/// GET /api/v1/governance/compliance-report
pub async fn compliance_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<ComplianceReportService>>,
) -> HttpResponse {
    match service.generate_report().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// ============================================================
// Route configuration
// ============================================================

pub fn configure_governance_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/audit")
            .route("", web::get().to(list_audit_handler))
            .route("/integrity", web::get().to(verify_integrity_handler)),
    );
    cfg.service(
        web::scope("/api/v1/governance")
            .route(
                "/committees",
                web::get().to(list_committees_handler),
            )
            .route(
                "/committees",
                web::post().to(create_committee_handler),
            )
            .route(
                "/committees/{id}/decisions",
                web::post().to(record_decision_handler),
            )
            .route(
                "/controls",
                web::get().to(list_controls_handler),
            )
            .route(
                "/controls",
                web::post().to(create_control_handler),
            )
            .route(
                "/controls/{id}/approve",
                web::post().to(approve_control_handler),
            )
            .route(
                "/controls/{id}/reject",
                web::post().to(reject_control_handler),
            )
            .route(
                "/compliance-report",
                web::get().to(compliance_report_handler),
            ),
    );
}
