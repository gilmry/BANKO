use std::sync::Arc;

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use banko_application::accounting::{
    AccountingService, AccountingServiceError, ClosePeriodRequest, CreateEntryRequest,
    EclStagingQuery, EntryFilterQuery, LedgerQuery, LedgerService, PeriodService,
    TrialBalanceQuery,
};

// --- Journal Entry handlers ---

pub async fn create_entry_handler(
    service: web::Data<Arc<AccountingService>>,
    body: web::Json<CreateEntryRequest>,
) -> HttpResponse {
    match service.post_entry(body.into_inner()).await {
        Ok(entry) => HttpResponse::Created().json(entry),
        Err(AccountingServiceError::DomainError(msg)) => {
            HttpResponse::UnprocessableEntity().json(serde_json::json!({"error": msg}))
        }
        Err(AccountingServiceError::InvalidEntry(msg)) => {
            HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn get_entry_handler(
    service: web::Data<Arc<AccountingService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let entry_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service.get_entry(entry_id).await {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(AccountingServiceError::EntryNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Entry not found"}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn list_entries_handler(
    service: web::Data<Arc<AccountingService>>,
    query: web::Query<EntryFilterQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);

    match service.list_entries(page, limit).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn reverse_entry_handler(
    service: web::Data<Arc<AccountingService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let entry_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}))
        }
    };

    match service.reverse_entry(entry_id).await {
        Ok(entry) => HttpResponse::Created().json(entry),
        Err(AccountingServiceError::EntryNotFound) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Entry not found"}))
        }
        Err(AccountingServiceError::DomainError(msg)) => {
            HttpResponse::UnprocessableEntity().json(serde_json::json!({"error": msg}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

// --- Ledger handlers ---

pub async fn get_ledger_handler(
    service: web::Data<Arc<LedgerService>>,
    query: web::Query<LedgerQuery>,
) -> HttpResponse {
    let from = query
        .from
        .unwrap_or(chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
    let to = query.to.unwrap_or(chrono::Utc::now().date_naive());

    match service.get_general_ledger(from, to).await {
        Ok(ledger) => HttpResponse::Ok().json(ledger),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn get_trial_balance_handler(
    service: web::Data<Arc<LedgerService>>,
    query: web::Query<TrialBalanceQuery>,
) -> HttpResponse {
    let as_of = query.as_of.unwrap_or(chrono::Utc::now().date_naive());

    match service.get_trial_balance(as_of).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

// --- Period handlers ---

pub async fn close_period_handler(
    service: web::Data<Arc<PeriodService>>,
    body: web::Json<ClosePeriodRequest>,
) -> HttpResponse {
    match service.close_period(&body.period).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(AccountingServiceError::PeriodAlreadyClosed(p)) => HttpResponse::Conflict()
            .json(serde_json::json!({"error": format!("Period {p} already closed")})),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

// --- ECL handlers ---

pub async fn get_ecl_staging_handler(_query: web::Query<EclStagingQuery>) -> HttpResponse {
    // ECL staging is a preparation feature — returns empty staging data
    // Real implementation would query loans and classify them
    HttpResponse::Ok().json(serde_json::json!({
        "stages": {
            "stage1": {"count": 0, "total_ecl": 0},
            "stage2": {"count": 0, "total_ecl": 0},
            "stage3": {"count": 0, "total_ecl": 0}
        },
        "message": "IFRS 9 ECL staging preparation — classify loans via EclService"
    }))
}
