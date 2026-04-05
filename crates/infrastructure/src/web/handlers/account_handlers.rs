use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::account::{AccountService, AccountServiceError};
use banko_domain::account::{AccountId, AccountType};
use banko_domain::shared::{Currency, CustomerId, Money};

use crate::web::middleware::AuthenticatedUser;

// --- Request/Response DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub customer_id: String,
    pub account_type: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAccountResponse {
    pub account_id: String,
    pub rib: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct ListAccountsQuery {
    pub customer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMovementRequest {
    pub movement_type: String,
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MovementResponse {
    pub id: String,
    pub account_id: String,
    pub movement_type: String,
    pub amount: f64,
    pub balance_after: f64,
    pub currency: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AccountDetailResponse {
    pub id: String,
    pub customer_id: String,
    pub rib: String,
    pub account_type: String,
    pub balance: f64,
    pub available_balance: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub recent_movements: Vec<MovementResponse>,
}

#[derive(Debug, Serialize)]
pub struct AccountSummaryResponse {
    pub id: String,
    pub customer_id: String,
    pub rib: String,
    pub account_type: String,
    pub balance: f64,
    pub available_balance: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct StatementQuery {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StatementJsonResponse {
    pub account_id: String,
    pub rib: String,
    pub period_from: Option<String>,
    pub period_to: Option<String>,
    pub opening_balance: f64,
    pub closing_balance: f64,
    pub currency: String,
    pub movements: Vec<MovementResponse>,
}

// --- Handlers ---

/// POST /api/v1/accounts
pub async fn create_account_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<AccountService>>,
    body: web::Json<CreateAccountRequest>,
) -> HttpResponse {
    // Validate customer_id UUID
    let customer_id = match CustomerId::parse(&body.customer_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer_id format".to_string(),
            });
        }
    };

    // Parse account type
    let account_type = match AccountType::from_str_type(&body.account_type) {
        Ok(at) => at,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            });
        }
    };

    match service.open_account(customer_id, account_type).await {
        Ok(account) => HttpResponse::Created().json(CreateAccountResponse {
            account_id: account.id().to_string(),
            rib: account.rib().as_str().to_string(),
            status: account.status().as_str().to_string(),
        }),
        Err(AccountServiceError::KycNotValidated) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "KYC not validated for customer".to_string(),
            })
        }
        Err(AccountServiceError::InvalidInput(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(e) => {
            tracing::error!("Create account error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// GET /api/v1/accounts/{id}
pub async fn get_account_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<AccountService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id = match AccountId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account ID format".to_string(),
            });
        }
    };

    match service.find_by_id(&account_id).await {
        Ok(account) => {
            // Get last 10 movements
            let movements = service
                .list_movements(&account_id, 10)
                .await
                .unwrap_or_default();

            let movement_responses: Vec<MovementResponse> = movements
                .iter()
                .map(|m| MovementResponse {
                    id: m.id().to_string(),
                    account_id: m.account_id().to_string(),
                    movement_type: m.movement_type().as_str().to_string(),
                    amount: m.amount().amount(),
                    balance_after: m.balance_after().amount(),
                    currency: m.amount().currency().to_string(),
                    description: m.description().to_string(),
                    created_at: m.created_at().to_rfc3339(),
                })
                .collect();

            HttpResponse::Ok().json(AccountDetailResponse {
                id: account.id().to_string(),
                customer_id: account.customer_id().to_string(),
                rib: account.rib().as_str().to_string(),
                account_type: account.account_type().as_str().to_string(),
                balance: account.balance().amount(),
                available_balance: account.available_balance().amount(),
                currency: account.balance().currency().to_string(),
                status: account.status().as_str().to_string(),
                created_at: account.created_at().to_rfc3339(),
                updated_at: account.updated_at().to_rfc3339(),
                recent_movements: movement_responses,
            })
        }
        Err(AccountServiceError::AccountNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Account not found".to_string(),
        }),
        Err(e) => {
            tracing::error!("Get account error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// GET /api/v1/accounts
pub async fn list_accounts_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<AccountService>>,
    query: web::Query<ListAccountsQuery>,
) -> HttpResponse {
    if let Some(ref cid) = query.customer_id {
        let customer_id = match CustomerId::parse(cid) {
            Ok(id) => id,
            Err(_) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid customer_id format".to_string(),
                });
            }
        };

        match service.list_by_customer(&customer_id).await {
            Ok(accounts) => {
                let responses: Vec<AccountSummaryResponse> = accounts
                    .iter()
                    .map(|a| AccountSummaryResponse {
                        id: a.id().to_string(),
                        customer_id: a.customer_id().to_string(),
                        rib: a.rib().as_str().to_string(),
                        account_type: a.account_type().as_str().to_string(),
                        balance: a.balance().amount(),
                        available_balance: a.available_balance().amount(),
                        currency: a.balance().currency().to_string(),
                        status: a.status().as_str().to_string(),
                        created_at: a.created_at().to_rfc3339(),
                    })
                    .collect();
                HttpResponse::Ok().json(responses)
            }
            Err(e) => {
                tracing::error!("List accounts error: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                })
            }
        }
    } else {
        // No customer_id filter -- return empty for now (list all would require pagination)
        let empty: Vec<AccountSummaryResponse> = vec![];
        HttpResponse::Ok().json(empty)
    }
}

/// POST /api/v1/accounts/{id}/movements
pub async fn create_movement_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<AccountService>>,
    path: web::Path<String>,
    body: web::Json<CreateMovementRequest>,
) -> HttpResponse {
    let account_id = match AccountId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account ID format".to_string(),
            });
        }
    };

    if body.amount <= 0.0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Amount must be positive".to_string(),
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

    let description = body.description.as_deref().unwrap_or("");

    match body.movement_type.to_lowercase().as_str() {
        "deposit" => match service.deposit(&account_id, amount, description).await {
            Ok(movement) => HttpResponse::Created().json(MovementResponse {
                id: movement.id().to_string(),
                account_id: movement.account_id().to_string(),
                movement_type: movement.movement_type().as_str().to_string(),
                amount: movement.amount().amount(),
                balance_after: movement.balance_after().amount(),
                currency: movement.amount().currency().to_string(),
                description: movement.description().to_string(),
                created_at: movement.created_at().to_rfc3339(),
            }),
            Err(AccountServiceError::AccountNotFound) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "Account not found".to_string(),
                })
            }
            Err(AccountServiceError::AccountClosed) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Account is closed".to_string(),
                })
            }
            Err(AccountServiceError::AccountSuspended) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Account is suspended".to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Deposit error: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                })
            }
        },
        "withdrawal" => match service.withdraw(&account_id, amount, description).await {
            Ok(movement) => HttpResponse::Created().json(MovementResponse {
                id: movement.id().to_string(),
                account_id: movement.account_id().to_string(),
                movement_type: movement.movement_type().as_str().to_string(),
                amount: movement.amount().amount(),
                balance_after: movement.balance_after().amount(),
                currency: movement.amount().currency().to_string(),
                description: movement.description().to_string(),
                created_at: movement.created_at().to_rfc3339(),
            }),
            Err(AccountServiceError::InsufficientFunds) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Insufficient funds".to_string(),
                })
            }
            Err(AccountServiceError::AccountNotFound) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "Account not found".to_string(),
                })
            }
            Err(AccountServiceError::AccountClosed) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Account is closed".to_string(),
                })
            }
            Err(AccountServiceError::AccountSuspended) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Account is suspended".to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Withdrawal error: {e}");
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                })
            }
        },
        _ => HttpResponse::BadRequest().json(ErrorResponse {
            error: "movement_type must be 'Deposit' or 'Withdrawal'".to_string(),
        }),
    }
}

/// GET /api/v1/accounts/{id}/movements
pub async fn list_movements_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<AccountService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id = match AccountId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account ID format".to_string(),
            });
        }
    };

    match service.list_movements(&account_id, 100).await {
        Ok(movements) => {
            let responses: Vec<MovementResponse> = movements
                .iter()
                .map(|m| MovementResponse {
                    id: m.id().to_string(),
                    account_id: m.account_id().to_string(),
                    movement_type: m.movement_type().as_str().to_string(),
                    amount: m.amount().amount(),
                    balance_after: m.balance_after().amount(),
                    currency: m.amount().currency().to_string(),
                    description: m.description().to_string(),
                    created_at: m.created_at().to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(responses)
        }
        Err(AccountServiceError::AccountNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Account not found".to_string(),
        }),
        Err(e) => {
            tracing::error!("List movements error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// GET /api/v1/accounts/{id}/statement
pub async fn get_statement_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<AccountService>>,
    path: web::Path<String>,
    query: web::Query<StatementQuery>,
) -> HttpResponse {
    let account_id = match AccountId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid account ID format".to_string(),
            });
        }
    };

    // Parse date_from and date_to
    let from = query.date_from.as_ref().and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });
    let to = query.date_to.as_ref().and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    let format = query.format.as_deref().unwrap_or("json");

    match service.get_statement(&account_id, from, to).await {
        Ok(statement) => {
            if format == "csv" {
                // Build CSV output
                let mut csv = String::from("date,type,amount,balance,description\n");
                for m in &statement.movements {
                    csv.push_str(&format!(
                        "{},{},{},{},{}\n",
                        m.created_at,
                        m.movement_type,
                        m.amount,
                        m.balance_after,
                        m.description.replace(',', ";"),
                    ));
                }
                HttpResponse::Ok().content_type("text/csv").body(csv)
            } else {
                let movement_responses: Vec<MovementResponse> = statement
                    .movements
                    .iter()
                    .map(|m| MovementResponse {
                        id: String::new(), // Statement DTO doesn't have id from this path
                        account_id: statement.account_id.clone(),
                        movement_type: m.movement_type.clone(),
                        amount: m.amount,
                        balance_after: m.balance_after,
                        currency: m.currency.clone(),
                        description: m.description.clone(),
                        created_at: m.created_at.to_rfc3339(),
                    })
                    .collect();

                HttpResponse::Ok().json(StatementJsonResponse {
                    account_id: statement.account_id,
                    rib: statement.rib,
                    period_from: statement.period_from.map(|d| d.to_rfc3339()),
                    period_to: statement.period_to.map(|d| d.to_rfc3339()),
                    opening_balance: statement.opening_balance,
                    closing_balance: statement.closing_balance,
                    currency: statement.currency,
                    movements: movement_responses,
                })
            }
        }
        Err(AccountServiceError::AccountNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Account not found".to_string(),
        }),
        Err(e) => {
            tracing::error!("Get statement error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}
