use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use banko_application::aml::*;
use banko_domain::aml::{AlertStatus, Direction, RiskLevel, TransactionId, TransactionType};
use banko_domain::shared::{Currency, Money};

use crate::web::middleware::AuthenticatedUser;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub account_id: String,
    pub customer_id: String,
    pub counterparty: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub transaction_type: String,
    pub direction: String,
}

#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    pub account_id: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ListAlertsQuery {
    pub status: Option<String>,
    pub risk_level: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ListInvestigationsQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct OpenInvestigationBody {
    pub alert_id: String,
    pub assigned_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddNoteBody {
    pub note: String,
    pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct CloseInvestigationBody {
    pub outcome: String, // "confirmed" or "dismissed"
}

#[derive(Debug, Deserialize)]
pub struct GenerateReportBody {
    pub investigation_id: String,
    pub customer_info: String,
    pub transaction_details: String,
    pub reasons: String,
    pub evidence: Option<String>,
    pub timeline: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FreezeAccountBody {
    pub account_id: String,
    pub reason: String,
    pub ordered_by: String,
}

#[derive(Debug, Deserialize)]
pub struct LiftFreezeBody {
    pub lifted_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ListFreezesQuery {
    pub account_id: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Transaction Handlers ---

/// POST /api/v1/transactions
pub async fn create_transaction_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<TransactionMonitoringService>>,
    body: web::Json<CreateTransactionRequest>,
) -> HttpResponse {
    let account_id = match Uuid::parse_str(&body.account_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account_id".to_string(),
            })
        }
    };
    let customer_id = match Uuid::parse_str(&body.customer_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer_id".to_string(),
            })
        }
    };
    let currency = Currency::from_code(body.currency.as_deref().unwrap_or("TND"));
    let currency = match currency {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid currency".to_string(),
            })
        }
    };
    let amount = match Money::new(body.amount, currency) {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    };
    let tx_type = match TransactionType::from_str_type(&body.transaction_type) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid transaction_type".to_string(),
            })
        }
    };
    let direction = match Direction::from_str_dir(&body.direction) {
        Ok(d) => d,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid direction".to_string(),
            })
        }
    };

    match service
        .record_transaction(
            account_id,
            customer_id,
            body.counterparty.clone(),
            amount,
            tx_type,
            direction,
        )
        .await
    {
        Ok((tx, alerts)) => {
            let alert_responses: Vec<AlertResponse> = alerts
                .iter()
                .map(|a| AlertResponse {
                    id: a.id().to_string(),
                    transaction_id: a.transaction_id().to_string(),
                    risk_level: a.risk_level().as_str().to_string(),
                    reason: a.reason().to_string(),
                    status: a.status().as_str().to_string(),
                    created_at: a.created_at(),
                })
                .collect();

            HttpResponse::Created().json(TransactionResponse {
                id: tx.id().to_string(),
                account_id: tx.account_id().to_string(),
                customer_id: tx.customer_id().to_string(),
                counterparty: tx.counterparty().to_string(),
                amount: tx.amount().amount(),
                currency: tx.amount().currency().to_string(),
                transaction_type: tx.transaction_type().as_str().to_string(),
                direction: tx.direction().as_str().to_string(),
                timestamp: tx.timestamp(),
                created_at: tx.created_at(),
                alerts: alert_responses,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/transactions
pub async fn list_transactions_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<TransactionMonitoringService>>,
    query: web::Query<ListTransactionsQuery>,
) -> HttpResponse {
    let account_id = query
        .account_id
        .as_ref()
        .and_then(|s| Uuid::parse_str(s).ok());
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service.list_transactions(account_id, page, limit).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/transactions/{id}
pub async fn get_transaction_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<TransactionMonitoringService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let tx_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => TransactionId::from_uuid(id),
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.get_transaction(&tx_id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(AmlServiceError::TransactionNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Transaction not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/aml/alerts
pub async fn list_alerts_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<TransactionMonitoringService>>,
    query: web::Query<ListAlertsQuery>,
) -> HttpResponse {
    let status = query
        .status
        .as_ref()
        .and_then(|s| AlertStatus::from_str_status(s).ok());
    let risk_level = query
        .risk_level
        .as_ref()
        .and_then(|s| RiskLevel::from_str_level(s).ok());
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match service.list_alerts(status, risk_level, page, limit).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/aml/alerts/{id}
pub async fn get_alert_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<TransactionMonitoringService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let alert_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.get_alert(alert_id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(AmlServiceError::AlertNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Alert not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- Investigation Handlers ---

/// POST /api/v1/aml/investigations
pub async fn open_investigation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<InvestigationService>>,
    body: web::Json<OpenInvestigationBody>,
) -> HttpResponse {
    let alert_id = match Uuid::parse_str(&body.alert_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid alert_id".to_string(),
            })
        }
    };

    match service
        .open_investigation(alert_id, body.assigned_to.clone())
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(AmlServiceError::AlertNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Alert not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/aml/investigations/{id}
pub async fn get_investigation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<InvestigationService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.get_investigation(id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(AmlServiceError::InvestigationNotFound) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: "Investigation not found".to_string(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/aml/investigations/{id}/notes
pub async fn add_note_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<InvestigationService>>,
    path: web::Path<String>,
    body: web::Json<AddNoteBody>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service
        .add_note(id, body.note.clone(), body.author.clone())
        .await
    {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/aml/investigations/{id}/escalate
pub async fn escalate_investigation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<InvestigationService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.escalate(id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/aml/investigations/{id}/close
pub async fn close_investigation_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<InvestigationService>>,
    path: web::Path<String>,
    body: web::Json<CloseInvestigationBody>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    let result = match body.outcome.as_str() {
        "confirmed" => service.close_confirmed(id).await,
        "dismissed" => service.close_dismissed(id).await,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "outcome must be 'confirmed' or 'dismissed'".to_string(),
            })
        }
    };

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- Report Handlers ---

/// POST /api/v1/aml/reports
pub async fn generate_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<DosReportService>>,
    body: web::Json<GenerateReportBody>,
) -> HttpResponse {
    let investigation_id = match Uuid::parse_str(&body.investigation_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid investigation_id".to_string(),
            })
        }
    };

    match service
        .generate_report(
            investigation_id,
            body.customer_info.clone(),
            body.transaction_details.clone(),
            body.reasons.clone(),
            body.evidence.clone(),
            body.timeline.clone(),
        )
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/aml/reports/{id}/submit
pub async fn submit_report_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<DosReportService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.submit_to_ctaf(id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

// --- Freeze Handlers ---

/// POST /api/v1/aml/freezes
pub async fn freeze_account_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<AssetFreezeService>>,
    body: web::Json<FreezeAccountBody>,
) -> HttpResponse {
    let account_id = match Uuid::parse_str(&body.account_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account_id".to_string(),
            })
        }
    };

    match service
        .freeze_account(account_id, body.reason.clone(), body.ordered_by.clone())
        .await
    {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// POST /api/v1/aml/freezes/{id}/lift
pub async fn lift_freeze_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<AssetFreezeService>>,
    path: web::Path<String>,
    body: web::Json<LiftFreezeBody>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID".to_string(),
            })
        }
    };

    match service.lift_freeze(id, body.lifted_by.clone()).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

/// GET /api/v1/aml/freezes
pub async fn list_freezes_handler(
    _auth: AuthenticatedUser,
    service: web::Data<Arc<AssetFreezeService>>,
    query: web::Query<ListFreezesQuery>,
) -> HttpResponse {
    let account_id = match query.account_id.as_ref() {
        Some(s) => match Uuid::parse_str(s) {
            Ok(id) => id,
            Err(_) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid account_id".to_string(),
                })
            }
        },
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "account_id is required".to_string(),
            })
        }
    };

    match service.list_freezes_for_account(account_id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}
