use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenSecuritiesAccountRequest {
    pub account_number: String,
    pub account_type: String, // Individual, Joint, Corporate, Nominee
    pub custodian_bank: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaceTradeOrderRequest {
    pub account_id: String,
    pub order_type: String,      // Buy, Sell
    pub security_isin: String,
    pub quantity: i64,
    pub price_type: String,      // Market, Limit, StopLoss
    pub limit_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecuteTradeOrderRequest {
    pub order_id: String,
    pub execution_quantity: i64,
    pub execution_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelTradeOrderRequest {
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettleTradeRequest {
    pub order_id: String,
    pub settlement_date: NaiveDate,
    pub counterparty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSecurityHoldingRequest {
    pub account_id: String,
    pub security_isin: String,
    pub new_market_value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCorporateActionRequest {
    pub security_isin: String,
    pub action_type: String, // Dividend, Coupon, Split, RightsIssue, Merger, Redemption
    pub record_date: NaiveDate,
    pub ex_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub ratio_or_amount: f64,
}

// --- Response DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct SecuritiesAccountResponse {
    pub id: String,
    pub customer_id: String,
    pub account_number: String,
    pub account_type: String,
    pub custodian_bank: String,
    pub status: String,
    pub total_market_value: f64,
    pub total_cost_basis: f64,
    pub total_unrealized_pnl: f64,
    pub holdings_count: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityHoldingResponse {
    pub account_id: String,
    pub isin_code: String,
    pub security_name: String,
    pub security_type: String,
    pub quantity: i64,
    pub average_cost: f64,
    pub total_cost_basis: f64,
    pub market_value: f64,
    pub unrealized_pnl: f64,
    pub pnl_percentage: f64,
    pub last_valuation_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeOrderResponse {
    pub order_id: String,
    pub account_id: String,
    pub order_type: String,
    pub security_isin: String,
    pub quantity: i64,
    pub price_type: String,
    pub limit_price: Option<f64>,
    pub status: String,
    pub executed_quantity: i64,
    pub remaining_quantity: i64,
    pub fill_percentage: f64,
    pub average_execution_price: Option<f64>,
    pub placed_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementResponse {
    pub settlement_id: String,
    pub trade_order_id: String,
    pub settlement_date: NaiveDate,
    pub settlement_type: String, // DVP
    pub status: String,
    pub counterparty: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorporateActionResponse {
    pub action_id: String,
    pub security_isin: String,
    pub action_type: String,
    pub record_date: NaiveDate,
    pub ex_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub ratio_or_amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioValuationResponse {
    pub account_id: String,
    pub valuation_date: DateTime<Utc>,
    pub total_market_value: f64,
    pub total_cost_basis: f64,
    pub total_unrealized_pnl: f64,
    pub pnl_percentage: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioSummaryResponse {
    pub account_id: String,
    pub account_number: String,
    pub account_type: String,
    pub status: String,
    pub valuation: PortfolioValuationResponse,
    pub holdings: Vec<SecurityHoldingResponse>,
}
