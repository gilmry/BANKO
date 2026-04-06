use std::sync::Arc;
use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_application::account::{
    MultiCurrencyService, AccountServiceError, ConsolidatedBalance,
};
use banko_domain::account::Currency;

// ==================== DTOs ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertBetweenAccountsRequest {
    pub from_account_id: String,
    pub to_account_id: String,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionHistoryQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedBalanceResponse {
    pub balances: Vec<(String, Decimal)>,
    pub total_tnd: Decimal,
    pub rates_used: Vec<(String, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResultResponse {
    pub original_amount: Decimal,
    pub original_currency: String,
    pub converted_amount: Decimal,
    pub target_currency: String,
    pub market_rate: Decimal,
    pub bank_rate: Decimal,
    pub margin_applied: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyUsageResponse {
    pub customer_id: String,
    pub currency: String,
    pub month: String,
    pub usage: Decimal,
    pub limit: Decimal,
}

// ==================== Handlers ====================

/// Get consolidated balance across all accounts for a customer
pub async fn get_consolidated_balance_handler(
    service: web::Data<Arc<MultiCurrencyService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let customer_id_str = path.into_inner();
    let customer_id = match Uuid::parse_str(&customer_id_str) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid UUID: {}", e)}))
        }
    };

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.get_consolidated_balance(customer_id))
    })
    .await
    {
        Ok(Ok(balance)) => {
            let response = ConsolidatedBalanceResponse {
                balances: balance
                    .balances
                    .iter()
                    .map(|(curr, amt)| (curr.code().to_string(), *amt))
                    .collect(),
                total_tnd: balance.total_tnd,
                rates_used: balance
                    .rates_used
                    .iter()
                    .map(|(curr, rate)| (curr.code().to_string(), *rate))
                    .collect(),
            };
            HttpResponse::Ok().json(response)
        }
        Ok(Err(AccountServiceError::InvalidEntry(msg))) => {
            HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// Convert between two accounts of the same customer
pub async fn convert_between_accounts_handler(
    service: web::Data<Arc<MultiCurrencyService>>,
    path: web::Path<String>,
    body: web::Json<ConvertBetweenAccountsRequest>,
) -> HttpResponse {
    let customer_id_str = path.into_inner();
    let customer_id = match Uuid::parse_str(&customer_id_str) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid UUID: {}", e)}))
        }
    };

    let req = body.into_inner();
    let from_account_id = match Uuid::parse_str(&req.from_account_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid from_account_id: {}", e)}))
        }
    };

    let to_account_id = match Uuid::parse_str(&req.to_account_id) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid to_account_id: {}", e)}))
        }
    };

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.convert_between_accounts(
            from_account_id,
            to_account_id,
            req.amount,
            customer_id,
        ))
    })
    .await
    {
        Ok(Ok(result)) => {
            let response = ConversionResultResponse {
                original_amount: result.original_amount,
                original_currency: result.original_currency.code().to_string(),
                converted_amount: result.converted_amount,
                target_currency: result.target_currency.code().to_string(),
                market_rate: result.market_rate,
                bank_rate: result.bank_rate,
                margin_applied: result.margin_applied,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(Err(AccountServiceError::InvalidEntry(msg))) => {
            HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// Get monthly conversion usage for a customer
pub async fn get_monthly_usage_handler(
    service: web::Data<Arc<MultiCurrencyService>>,
    path: web::Path<(String, String, String)>,
) -> HttpResponse {
    let (customer_id_str, currency_str, month_str) = path.into_inner();

    let customer_id = match Uuid::parse_str(&customer_id_str) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid customer_id: {}", e)}))
        }
    };

    let currency = match Currency::from_code(&currency_str) {
        Some(curr) => curr,
        None => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Invalid currency code"}))
        }
    };

    let month = match month_str.parse::<u32>() {
        Ok(m) if m > 0 && m <= 12 => m,
        _ => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Invalid month"}))
        }
    };

    let service_clone = service.get_ref().clone();
    match tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(service_clone.get_monthly_conversion_usage(customer_id, currency, month))
    })
    .await
    {
        Ok(Ok(usage)) => {
            let response = MonthlyUsageResponse {
                customer_id: customer_id_str,
                currency: currency_str,
                month: month_str,
                usage,
                limit: Decimal::from(100000),
            };
            HttpResponse::Ok().json(response)
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

/// Get conversion history for a customer
pub async fn get_conversion_history_handler(
    path: web::Path<String>,
) -> HttpResponse {
    let customer_id_str = path.into_inner();
    match Uuid::parse_str(&customer_id_str) {
        Ok(_) => {
            // In a real implementation, fetch from database
            HttpResponse::Ok().json(serde_json::json!({
                "customer_id": customer_id_str,
                "conversions": []
            }))
        }
        Err(e) => {
            HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid customer_id: {}", e)}))
        }
    }
}
