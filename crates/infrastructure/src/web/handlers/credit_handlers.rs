use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::credit::{LoanService, LoanServiceError};
use banko_domain::account::AccountId;
use banko_domain::credit::{AmortizationType, AssetClass, LoanId, LoanStatus, PaymentFrequency};
use banko_domain::shared::{Currency, CustomerId, Money};

use crate::web::middleware::AuthenticatedUser;

// --- Request/Response DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateLoanRequest {
    pub account_id: String,
    pub customer_id: String,
    pub amount: f64,
    pub interest_rate: Option<f64>,
    pub term_months: u32,
}

#[derive(Debug, Serialize)]
pub struct CreateLoanResponse {
    pub loan_id: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct ListLoansQuery {
    pub status: Option<String>,
    pub asset_class: Option<String>,
    pub account_id: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveLoanBody {
    // empty for now, can add approved_amount later
}

#[derive(Debug, Deserialize)]
pub struct DisburseLoanBody {
    pub disbursement_date: Option<String>,
    pub frequency: Option<String>,
    pub amortization_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClassifyLoanBody {
    pub days_past_due: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProvisionLoanBody {
    pub amount: f64,
}

// --- Handlers ---

/// POST /api/v1/loans
pub async fn create_loan_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    body: web::Json<CreateLoanRequest>,
) -> HttpResponse {
    let account_id = match AccountId::parse(&body.account_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account_id format".to_string(),
            });
        }
    };

    let customer_id = match CustomerId::parse(&body.customer_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer_id format".to_string(),
            });
        }
    };

    if body.amount <= 0.0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Amount must be positive".to_string(),
        });
    }

    if body.amount > 500000.0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Amount exceeds maximum (500000 TND)".to_string(),
        });
    }

    let amount = match Money::new(body.amount, Currency::TND) {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Invalid amount: {e}"),
            });
        }
    };

    let interest_rate = body.interest_rate.unwrap_or(8.0);

    match service
        .apply_for_loan(
            account_id,
            customer_id,
            amount,
            interest_rate,
            body.term_months,
        )
        .await
    {
        Ok(loan) => HttpResponse::Created().json(CreateLoanResponse {
            loan_id: loan.id().to_string(),
            status: loan.status().as_str().to_string(),
        }),
        Err(LoanServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(LoanServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => {
            tracing::error!("Create loan error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// GET /api/v1/loans
pub async fn list_loans_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    query: web::Query<ListLoansQuery>,
) -> HttpResponse {
    let status = query
        .status
        .as_ref()
        .map(|s| LoanStatus::from_str_status(s));
    if let Some(Err(e)) = &status {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Invalid status filter: {e}"),
        });
    }
    let status = status.and_then(|r| r.ok());

    let asset_class = query
        .asset_class
        .as_ref()
        .map(|s| AssetClass::from_str_class(s));
    if let Some(Err(e)) = &asset_class {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Invalid asset_class filter: {e}"),
        });
    }
    let asset_class = asset_class.and_then(|r| r.ok());

    let account_id = if let Some(ref aid) = query.account_id {
        match AccountId::parse(aid) {
            Ok(id) => Some(id),
            Err(_) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid account_id format".to_string(),
                });
            }
        }
    } else {
        None
    };

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);

    match service
        .list_loans(status, asset_class, account_id.as_ref(), page, limit)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            tracing::error!("List loans error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// GET /api/v1/loans/{id}
pub async fn get_loan_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let loan_id = match LoanId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid loan ID format".to_string(),
            });
        }
    };

    match service.find_by_id(&loan_id).await {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(LoanServiceError::LoanNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Loan not found".to_string(),
        }),
        Err(e) => {
            tracing::error!("Get loan error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// POST /api/v1/loans/{id}/approve
pub async fn approve_loan_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let loan_id = match LoanId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid loan ID format".to_string(),
            });
        }
    };

    match service.approve_loan(&loan_id).await {
        Ok(loan) => HttpResponse::Ok().json(LoanService::loan_to_response(&loan)),
        Err(LoanServiceError::LoanNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Loan not found".to_string(),
        }),
        Err(LoanServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => {
            tracing::error!("Approve loan error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// POST /api/v1/loans/{id}/disburse
pub async fn disburse_loan_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    path: web::Path<String>,
    body: web::Json<DisburseLoanBody>,
) -> HttpResponse {
    let loan_id = match LoanId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid loan ID format".to_string(),
            });
        }
    };

    let disbursement_date = if let Some(ref d) = body.disbursement_date {
        match chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid disbursement_date format (use YYYY-MM-DD)".to_string(),
                });
            }
        }
    } else {
        chrono::Utc::now().date_naive()
    };

    let frequency = if let Some(ref f) = body.frequency {
        match PaymentFrequency::from_str_freq(f) {
            Ok(freq) => freq,
            Err(e) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: e.to_string(),
                });
            }
        }
    } else {
        PaymentFrequency::Monthly
    };

    let amort_type = if let Some(ref a) = body.amortization_type {
        match AmortizationType::from_str_type(a) {
            Ok(at) => at,
            Err(e) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: e.to_string(),
                });
            }
        }
    } else {
        AmortizationType::Constant
    };

    match service
        .disburse(&loan_id, disbursement_date, frequency, amort_type)
        .await
    {
        Ok(loan) => HttpResponse::Ok().json(LoanService::loan_to_response(&loan)),
        Err(LoanServiceError::LoanNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Loan not found".to_string(),
        }),
        Err(LoanServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => {
            tracing::error!("Disburse loan error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// POST /api/v1/loans/{id}/classify
pub async fn classify_loan_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    path: web::Path<String>,
    body: web::Json<ClassifyLoanBody>,
) -> HttpResponse {
    let loan_id = match LoanId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid loan ID format".to_string(),
            });
        }
    };

    match service
        .classify_and_provision(&loan_id, body.days_past_due)
        .await
    {
        Ok((class, provision)) => HttpResponse::Ok().json(serde_json::json!({
            "asset_class": class.as_str(),
            "provision_amount": provision.amount().amount(),
            "provision_rate": provision.rate(),
        })),
        Err(LoanServiceError::LoanNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Loan not found".to_string(),
        }),
        Err(LoanServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => {
            tracing::error!("Classify loan error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// POST /api/v1/loans/{id}/payment
pub async fn record_payment_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<LoanService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let loan_id = match LoanId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid loan ID format".to_string(),
            });
        }
    };

    match service.record_payment(&loan_id).await {
        Ok(loan) => HttpResponse::Ok().json(LoanService::loan_to_response(&loan)),
        Err(LoanServiceError::LoanNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Loan not found".to_string(),
        }),
        Err(LoanServiceError::DomainError(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => {
            tracing::error!("Record payment error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}
