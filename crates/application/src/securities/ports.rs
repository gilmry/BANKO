use async_trait::async_trait;
use chrono::NaiveDate;

use banko_domain::securities::{
    CorporateAction, IsinCode, SecuritiesAccount, SecuritiesAccountId, Settlement, SettlementId,
    TradeOrder, TradeOrderId, CorporateActionId,
};
use banko_domain::shared::CustomerId;

/// Port for securities account persistence -- implemented by infrastructure layer.
#[async_trait]
pub trait ISecuritiesAccountRepository: Send + Sync {
    async fn save(&self, account: &SecuritiesAccount) -> Result<(), String>;
    async fn find_by_id(&self, id: &SecuritiesAccountId) -> Result<Option<SecuritiesAccount>, String>;
    async fn find_by_customer_id(&self, customer_id: &CustomerId) -> Result<Vec<SecuritiesAccount>, String>;
    async fn find_by_account_number(&self, account_number: &str) -> Result<Option<SecuritiesAccount>, String>;
    async fn delete(&self, id: &SecuritiesAccountId) -> Result<(), String>;
}

/// Port for trade order persistence -- implemented by infrastructure layer.
#[async_trait]
pub trait ITradeOrderRepository: Send + Sync {
    async fn save(&self, order: &TradeOrder) -> Result<(), String>;
    async fn find_by_id(&self, id: &TradeOrderId) -> Result<Option<TradeOrder>, String>;
    async fn find_by_account_id(&self, account_id: &SecuritiesAccountId) -> Result<Vec<TradeOrder>, String>;
    async fn find_pending_by_account(&self, account_id: &SecuritiesAccountId) -> Result<Vec<TradeOrder>, String>;
    async fn delete(&self, id: &TradeOrderId) -> Result<(), String>;
}

/// Port for settlement persistence -- implemented by infrastructure layer.
#[async_trait]
pub trait ISettlementRepository: Send + Sync {
    async fn save(&self, settlement: &Settlement) -> Result<(), String>;
    async fn find_by_id(&self, id: &SettlementId) -> Result<Option<Settlement>, String>;
    async fn find_by_trade_order_id(&self, order_id: &TradeOrderId) -> Result<Option<Settlement>, String>;
    async fn find_pending_settlements(&self) -> Result<Vec<Settlement>, String>;
    async fn find_by_date_range(&self, from: NaiveDate, to: NaiveDate) -> Result<Vec<Settlement>, String>;
    async fn delete(&self, id: &SettlementId) -> Result<(), String>;
}

/// Port for corporate action persistence -- implemented by infrastructure layer.
#[async_trait]
pub trait ICorporateActionRepository: Send + Sync {
    async fn save(&self, action: &CorporateAction) -> Result<(), String>;
    async fn find_by_id(&self, id: &CorporateActionId) -> Result<Option<CorporateAction>, String>;
    async fn find_by_security_isin(&self, isin: &IsinCode) -> Result<Vec<CorporateAction>, String>;
    async fn find_effective_by_date(&self, date: NaiveDate) -> Result<Vec<CorporateAction>, String>;
    async fn delete(&self, id: &CorporateActionId) -> Result<(), String>;
}

/// Port for market price data -- implemented by infrastructure layer.
#[async_trait]
pub trait IMarketPriceFeed: Send + Sync {
    async fn get_current_price(&self, isin: &IsinCode) -> Result<f64, String>;
    async fn get_historical_price(&self, isin: &IsinCode, date: NaiveDate) -> Result<f64, String>;
}

/// Port for order matching engine -- implemented by infrastructure layer.
#[async_trait]
pub trait IOrderMatchingEngine: Send + Sync {
    async fn match_order(&self, order: &TradeOrder) -> Result<(i64, f64), String>; // (quantity, price)
    async fn get_best_bid_ask(&self, isin: &IsinCode) -> Result<(f64, f64), String>; // (bid, ask)
}

/// Port for settlement processing -- implemented by infrastructure layer.
#[async_trait]
pub trait ISettlementProcessor: Send + Sync {
    async fn process_dvp_settlement(&self, settlement: &Settlement, security_qty: i64, cash_amount: f64) -> Result<(), String>;
    async fn process_dividend_payment(&self, account_id: &SecuritiesAccountId, amount: f64) -> Result<(), String>;
}

/// Port to verify AML/Sanctions compliance
#[async_trait]
pub trait IAmlSanctionsChecker: Send + Sync {
    async fn check_customer_compliance(&self, customer_id: &CustomerId) -> Result<bool, String>;
    async fn check_transaction_compliance(&self, customer_id: &CustomerId, amount: f64) -> Result<bool, String>;
}
