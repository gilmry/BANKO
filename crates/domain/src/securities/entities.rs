use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::CustomerId;

use super::value_objects::*;

// --- SecurityHolding entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHolding {
    account_id: SecuritiesAccountId,
    isin_code: IsinCode,
    security_name: String,
    security_type: SecurityType,
    quantity: i64,
    average_cost: f64,
    market_value: f64,
    unrealized_pnl: f64,
    last_valuation_date: DateTime<Utc>,
}

impl SecurityHolding {
    pub fn new(
        account_id: SecuritiesAccountId,
        isin_code: IsinCode,
        security_name: &str,
        security_type: SecurityType,
        quantity: i64,
        average_cost: f64,
        market_value: f64,
    ) -> Result<Self, DomainError> {
        if quantity <= 0 {
            return Err(DomainError::ValidationError(
                "Quantity must be positive".to_string(),
            ));
        }

        if average_cost <= 0.0 {
            return Err(DomainError::ValidationError(
                "Average cost must be positive".to_string(),
            ));
        }

        if market_value < 0.0 {
            return Err(DomainError::ValidationError(
                "Market value cannot be negative".to_string(),
            ));
        }

        let unrealized_pnl = market_value - (average_cost * quantity as f64);

        Ok(SecurityHolding {
            account_id,
            isin_code,
            security_name: security_name.to_string(),
            security_type,
            quantity,
            average_cost,
            market_value,
            unrealized_pnl,
            last_valuation_date: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        account_id: SecuritiesAccountId,
        isin_code: IsinCode,
        security_name: String,
        security_type: SecurityType,
        quantity: i64,
        average_cost: f64,
        market_value: f64,
        unrealized_pnl: f64,
        last_valuation_date: DateTime<Utc>,
    ) -> Self {
        SecurityHolding {
            account_id,
            isin_code,
            security_name,
            security_type,
            quantity,
            average_cost,
            market_value,
            unrealized_pnl,
            last_valuation_date,
        }
    }

    /// Update market value and recalculate unrealized P&L
    pub fn update_market_value(&mut self, new_market_value: f64) -> Result<(), DomainError> {
        if new_market_value < 0.0 {
            return Err(DomainError::ValidationError(
                "Market value cannot be negative".to_string(),
            ));
        }

        self.market_value = new_market_value;
        self.unrealized_pnl = new_market_value - (self.average_cost * self.quantity as f64);
        self.last_valuation_date = Utc::now();
        Ok(())
    }

    /// Check if enough shares can be sold
    pub fn can_sell(&self, quantity: i64) -> bool {
        quantity > 0 && quantity <= self.quantity
    }

    // Getters
    pub fn account_id(&self) -> &SecuritiesAccountId {
        &self.account_id
    }

    pub fn isin_code(&self) -> &IsinCode {
        &self.isin_code
    }

    pub fn security_name(&self) -> &str {
        &self.security_name
    }

    pub fn security_type(&self) -> SecurityType {
        self.security_type
    }

    pub fn quantity(&self) -> i64 {
        self.quantity
    }

    pub fn average_cost(&self) -> f64 {
        self.average_cost
    }

    pub fn market_value(&self) -> f64 {
        self.market_value
    }

    pub fn unrealized_pnl(&self) -> f64 {
        self.unrealized_pnl
    }

    pub fn last_valuation_date(&self) -> DateTime<Utc> {
        self.last_valuation_date
    }

    pub fn total_cost_basis(&self) -> f64 {
        self.average_cost * self.quantity as f64
    }
}

// --- TradeOrder entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    order_id: TradeOrderId,
    account_id: SecuritiesAccountId,
    order_type: OrderType,
    security_isin: IsinCode,
    quantity: i64,
    price_type: PriceType,
    limit_price: Option<f64>,
    status: OrderStatus,
    executed_quantity: i64,
    average_execution_price: Option<f64>,
    placed_at: DateTime<Utc>,
    executed_at: Option<DateTime<Utc>>,
}

impl TradeOrder {
    pub fn new(
        account_id: SecuritiesAccountId,
        order_type: OrderType,
        security_isin: IsinCode,
        quantity: i64,
        price_type: PriceType,
        limit_price: Option<f64>,
    ) -> Result<Self, DomainError> {
        if quantity <= 0 {
            return Err(DomainError::ValidationError(
                "Order quantity must be positive".to_string(),
            ));
        }

        // Validate limit price for Limit orders
        if price_type == PriceType::Limit && limit_price.is_none() {
            return Err(DomainError::ValidationError(
                "Limit price required for Limit orders".to_string(),
            ));
        }

        if let Some(price) = limit_price {
            if price <= 0.0 {
                return Err(DomainError::ValidationError(
                    "Limit price must be positive".to_string(),
                ));
            }
        }

        Ok(TradeOrder {
            order_id: TradeOrderId::new(),
            account_id,
            order_type,
            security_isin,
            quantity,
            price_type,
            limit_price,
            status: OrderStatus::Pending,
            executed_quantity: 0,
            average_execution_price: None,
            placed_at: Utc::now(),
            executed_at: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        order_id: TradeOrderId,
        account_id: SecuritiesAccountId,
        order_type: OrderType,
        security_isin: IsinCode,
        quantity: i64,
        price_type: PriceType,
        limit_price: Option<f64>,
        status: OrderStatus,
        executed_quantity: i64,
        average_execution_price: Option<f64>,
        placed_at: DateTime<Utc>,
        executed_at: Option<DateTime<Utc>>,
    ) -> Self {
        TradeOrder {
            order_id,
            account_id,
            order_type,
            security_isin,
            quantity,
            price_type,
            limit_price,
            status,
            executed_quantity,
            average_execution_price,
            placed_at,
            executed_at,
        }
    }

    /// Execute or partially execute an order
    pub fn execute(
        &mut self,
        exec_quantity: i64,
        exec_price: f64,
    ) -> Result<(), DomainError> {
        if self.status == OrderStatus::Filled || self.status == OrderStatus::Cancelled {
            return Err(DomainError::ValidationError(
                "Cannot execute order with status Filled or Cancelled".to_string(),
            ));
        }

        if exec_quantity <= 0 || exec_quantity > self.quantity - self.executed_quantity {
            return Err(DomainError::ValidationError(
                "Invalid execution quantity".to_string(),
            ));
        }

        if exec_price <= 0.0 {
            return Err(DomainError::ValidationError(
                "Execution price must be positive".to_string(),
            ));
        }

        // Update average execution price
        let new_total = (self.average_execution_price.unwrap_or(0.0) * self.executed_quantity as f64)
            + (exec_price * exec_quantity as f64);
        self.executed_quantity += exec_quantity;
        self.average_execution_price = Some(new_total / self.executed_quantity as f64);

        // Update status
        if self.executed_quantity == self.quantity {
            self.status = OrderStatus::Filled;
            self.executed_at = Some(Utc::now());
        } else if self.executed_quantity > 0 {
            self.status = OrderStatus::PartiallyFilled;
        }

        Ok(())
    }

    /// Cancel the order
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Filled {
            return Err(DomainError::ValidationError(
                "Cannot cancel a filled order".to_string(),
            ));
        }

        self.status = OrderStatus::Cancelled;
        Ok(())
    }

    /// Reject the order (e.g., due to insufficient funds)
    pub fn reject(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Filled || self.status == OrderStatus::Cancelled {
            return Err(DomainError::ValidationError(
                "Cannot reject an already filled or cancelled order".to_string(),
            ));
        }

        self.status = OrderStatus::Rejected;
        Ok(())
    }

    pub fn is_pending(&self) -> bool {
        self.status == OrderStatus::Pending
    }

    pub fn is_fully_executed(&self) -> bool {
        self.status == OrderStatus::Filled
    }

    pub fn remaining_quantity(&self) -> i64 {
        self.quantity - self.executed_quantity
    }

    pub fn fill_percentage(&self) -> f64 {
        (self.executed_quantity as f64 / self.quantity as f64) * 100.0
    }

    // Getters
    pub fn order_id(&self) -> &TradeOrderId {
        &self.order_id
    }

    pub fn account_id(&self) -> &SecuritiesAccountId {
        &self.account_id
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type
    }

    pub fn security_isin(&self) -> &IsinCode {
        &self.security_isin
    }

    pub fn quantity(&self) -> i64 {
        self.quantity
    }

    pub fn price_type(&self) -> PriceType {
        self.price_type
    }

    pub fn limit_price(&self) -> Option<f64> {
        self.limit_price
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn executed_quantity(&self) -> i64 {
        self.executed_quantity
    }

    pub fn average_execution_price(&self) -> Option<f64> {
        self.average_execution_price
    }

    pub fn placed_at(&self) -> DateTime<Utc> {
        self.placed_at
    }

    pub fn executed_at(&self) -> Option<DateTime<Utc>> {
        self.executed_at
    }
}

// --- Settlement entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    settlement_id: SettlementId,
    trade_order_id: TradeOrderId,
    settlement_date: NaiveDate,
    settlement_type: SettlementType,
    status: SettlementStatus,
    counterparty: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Settlement {
    pub fn new(
        trade_order_id: TradeOrderId,
        settlement_date: NaiveDate,
        counterparty: &str,
    ) -> Result<Self, DomainError> {
        if counterparty.is_empty() {
            return Err(DomainError::ValidationError(
                "Counterparty cannot be empty".to_string(),
            ));
        }

        Ok(Settlement {
            settlement_id: SettlementId::new(),
            trade_order_id,
            settlement_date,
            settlement_type: SettlementType::DVP, // BVMT uses DVP
            status: SettlementStatus::Pending,
            counterparty: counterparty.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        settlement_id: SettlementId,
        trade_order_id: TradeOrderId,
        settlement_date: NaiveDate,
        settlement_type: SettlementType,
        status: SettlementStatus,
        counterparty: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Settlement {
            settlement_id,
            trade_order_id,
            settlement_date,
            settlement_type,
            status,
            counterparty,
            created_at,
            updated_at,
        }
    }

    /// Match delivery and payment
    pub fn match_settlement(&mut self) -> Result<(), DomainError> {
        if self.status != SettlementStatus::Pending {
            return Err(DomainError::ValidationError(
                "Cannot match already matched settlement".to_string(),
            ));
        }

        self.status = SettlementStatus::Matched;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Settle (delivery and payment both completed)
    pub fn settle(&mut self) -> Result<(), DomainError> {
        if self.status != SettlementStatus::Matched {
            return Err(DomainError::ValidationError(
                "Settlement must be matched before settlement".to_string(),
            ));
        }

        self.status = SettlementStatus::Settled;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark as failed
    pub fn fail(&mut self) -> Result<(), DomainError> {
        if self.status == SettlementStatus::Settled {
            return Err(DomainError::ValidationError(
                "Cannot fail a settled settlement".to_string(),
            ));
        }

        self.status = SettlementStatus::Failed;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Getters
    pub fn settlement_id(&self) -> &SettlementId {
        &self.settlement_id
    }

    pub fn trade_order_id(&self) -> &TradeOrderId {
        &self.trade_order_id
    }

    pub fn settlement_date(&self) -> NaiveDate {
        self.settlement_date
    }

    pub fn settlement_type(&self) -> SettlementType {
        self.settlement_type
    }

    pub fn status(&self) -> SettlementStatus {
        self.status
    }

    pub fn counterparty(&self) -> &str {
        &self.counterparty
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- CorporateAction entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateAction {
    action_id: CorporateActionId,
    security_isin: IsinCode,
    action_type: CorporateActionType,
    record_date: NaiveDate,
    ex_date: NaiveDate,
    payment_date: NaiveDate,
    ratio_or_amount: f64,
    status: CorporateActionStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CorporateAction {
    pub fn new(
        security_isin: IsinCode,
        action_type: CorporateActionType,
        record_date: NaiveDate,
        ex_date: NaiveDate,
        payment_date: NaiveDate,
        ratio_or_amount: f64,
    ) -> Result<Self, DomainError> {
        if ratio_or_amount <= 0.0 {
            return Err(DomainError::ValidationError(
                "Ratio or amount must be positive".to_string(),
            ));
        }

        if ex_date < record_date {
            return Err(DomainError::ValidationError(
                "Ex-date must be on or after record date".to_string(),
            ));
        }

        if payment_date < ex_date {
            return Err(DomainError::ValidationError(
                "Payment date must be on or after ex-date".to_string(),
            ));
        }

        Ok(CorporateAction {
            action_id: CorporateActionId::new(),
            security_isin,
            action_type,
            record_date,
            ex_date,
            payment_date,
            ratio_or_amount,
            status: CorporateActionStatus::Announced,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        action_id: CorporateActionId,
        security_isin: IsinCode,
        action_type: CorporateActionType,
        record_date: NaiveDate,
        ex_date: NaiveDate,
        payment_date: NaiveDate,
        ratio_or_amount: f64,
        status: CorporateActionStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        CorporateAction {
            action_id,
            security_isin,
            action_type,
            record_date,
            ex_date,
            payment_date,
            ratio_or_amount,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn make_effective(&mut self) -> Result<(), DomainError> {
        if self.status != CorporateActionStatus::Announced {
            return Err(DomainError::ValidationError(
                "Only announced corporate actions can be made effective".to_string(),
            ));
        }

        self.status = CorporateActionStatus::Effective;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), DomainError> {
        if self.status != CorporateActionStatus::Effective {
            return Err(DomainError::ValidationError(
                "Only effective corporate actions can be completed".to_string(),
            ));
        }

        self.status = CorporateActionStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Getters
    pub fn action_id(&self) -> &CorporateActionId {
        &self.action_id
    }

    pub fn security_isin(&self) -> &IsinCode {
        &self.security_isin
    }

    pub fn action_type(&self) -> CorporateActionType {
        self.action_type
    }

    pub fn record_date(&self) -> NaiveDate {
        self.record_date
    }

    pub fn ex_date(&self) -> NaiveDate {
        self.ex_date
    }

    pub fn payment_date(&self) -> NaiveDate {
        self.payment_date
    }

    pub fn ratio_or_amount(&self) -> f64 {
        self.ratio_or_amount
    }

    pub fn status(&self) -> CorporateActionStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- PortfolioValuation value object ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioValuation {
    account_id: SecuritiesAccountId,
    valuation_date: DateTime<Utc>,
    total_market_value: f64,
    total_cost_basis: f64,
    total_unrealized_pnl: f64,
    currency: String,
}

impl PortfolioValuation {
    pub fn new(
        account_id: SecuritiesAccountId,
        total_market_value: f64,
        total_cost_basis: f64,
        currency: &str,
    ) -> Result<Self, DomainError> {
        if total_market_value < 0.0 || total_cost_basis < 0.0 {
            return Err(DomainError::ValidationError(
                "Market value and cost basis cannot be negative".to_string(),
            ));
        }

        if currency.is_empty() {
            return Err(DomainError::ValidationError(
                "Currency cannot be empty".to_string(),
            ));
        }

        let total_unrealized_pnl = total_market_value - total_cost_basis;

        Ok(PortfolioValuation {
            account_id,
            valuation_date: Utc::now(),
            total_market_value,
            total_cost_basis,
            total_unrealized_pnl,
            currency: currency.to_string(),
        })
    }

    pub fn reconstitute(
        account_id: SecuritiesAccountId,
        valuation_date: DateTime<Utc>,
        total_market_value: f64,
        total_cost_basis: f64,
        total_unrealized_pnl: f64,
        currency: String,
    ) -> Self {
        PortfolioValuation {
            account_id,
            valuation_date,
            total_market_value,
            total_cost_basis,
            total_unrealized_pnl,
            currency,
        }
    }

    pub fn pnl_percentage(&self) -> f64 {
        if self.total_cost_basis == 0.0 {
            0.0
        } else {
            (self.total_unrealized_pnl / self.total_cost_basis) * 100.0
        }
    }

    // Getters
    pub fn account_id(&self) -> &SecuritiesAccountId {
        &self.account_id
    }

    pub fn valuation_date(&self) -> DateTime<Utc> {
        self.valuation_date
    }

    pub fn total_market_value(&self) -> f64 {
        self.total_market_value
    }

    pub fn total_cost_basis(&self) -> f64 {
        self.total_cost_basis
    }

    pub fn total_unrealized_pnl(&self) -> f64 {
        self.total_unrealized_pnl
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }
}

// --- SecuritiesAccount aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount {
    id: SecuritiesAccountId,
    customer_id: CustomerId,
    account_number: String,
    account_type: SecuritiesAccountType,
    custodian_bank: String,
    status: SecuritiesAccountStatus,
    holdings: Vec<SecurityHolding>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SecuritiesAccount {
    pub fn new(
        customer_id: CustomerId,
        account_number: &str,
        account_type: SecuritiesAccountType,
        custodian_bank: &str,
    ) -> Result<Self, DomainError> {
        if account_number.is_empty() {
            return Err(DomainError::ValidationError(
                "Account number cannot be empty".to_string(),
            ));
        }

        if custodian_bank.is_empty() {
            return Err(DomainError::ValidationError(
                "Custodian bank cannot be empty".to_string(),
            ));
        }

        Ok(SecuritiesAccount {
            id: SecuritiesAccountId::new(),
            customer_id,
            account_number: account_number.to_string(),
            account_type,
            custodian_bank: custodian_bank.to_string(),
            status: SecuritiesAccountStatus::Active,
            holdings: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: SecuritiesAccountId,
        customer_id: CustomerId,
        account_number: String,
        account_type: SecuritiesAccountType,
        custodian_bank: String,
        status: SecuritiesAccountStatus,
        holdings: Vec<SecurityHolding>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SecuritiesAccount {
            id,
            customer_id,
            account_number,
            account_type,
            custodian_bank,
            status,
            holdings,
            created_at,
            updated_at,
        }
    }

    /// Check if account is operational (not suspended/closed)
    pub fn is_operational(&self) -> bool {
        self.status == SecuritiesAccountStatus::Active
    }

    /// Suspend the account
    pub fn suspend(&mut self) -> Result<(), DomainError> {
        if self.status == SecuritiesAccountStatus::Closed {
            return Err(DomainError::ValidationError(
                "Cannot suspend a closed account".to_string(),
            ));
        }

        self.status = SecuritiesAccountStatus::Suspended;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Close the account
    pub fn close(&mut self) -> Result<(), DomainError> {
        if self.status == SecuritiesAccountStatus::Closed {
            return Err(DomainError::ValidationError(
                "Account is already closed".to_string(),
            ));
        }

        self.status = SecuritiesAccountStatus::Closed;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Reactivate a suspended account
    pub fn reactivate(&mut self) -> Result<(), DomainError> {
        if self.status != SecuritiesAccountStatus::Suspended {
            return Err(DomainError::ValidationError(
                "Only suspended accounts can be reactivated".to_string(),
            ));
        }

        self.status = SecuritiesAccountStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Add or update a holding
    pub fn add_or_update_holding(&mut self, holding: SecurityHolding) -> Result<(), DomainError> {
        if !self.is_operational() {
            return Err(DomainError::ValidationError(
                "Cannot modify holdings on non-operational account".to_string(),
            ));
        }

        if let Some(pos) = self.holdings.iter().position(|h| h.isin_code() == holding.isin_code()) {
            self.holdings[pos] = holding;
        } else {
            self.holdings.push(holding);
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get holding by ISIN
    pub fn get_holding(&self, isin: &IsinCode) -> Option<&SecurityHolding> {
        self.holdings.iter().find(|h| h.isin_code() == isin)
    }

    /// Get mutable holding by ISIN
    pub fn get_holding_mut(&mut self, isin: &IsinCode) -> Option<&mut SecurityHolding> {
        self.holdings.iter_mut().find(|h| h.isin_code() == isin)
    }

    /// Calculate total portfolio market value
    pub fn total_market_value(&self) -> f64 {
        self.holdings.iter().map(|h| h.market_value()).sum()
    }

    /// Calculate total portfolio cost basis
    pub fn total_cost_basis(&self) -> f64 {
        self.holdings.iter().map(|h| h.total_cost_basis()).sum()
    }

    /// Calculate total unrealized P&L
    pub fn total_unrealized_pnl(&self) -> f64 {
        self.holdings.iter().map(|h| h.unrealized_pnl()).sum()
    }

    /// Get number of distinct securities held
    pub fn holdings_count(&self) -> usize {
        self.holdings.len()
    }

    // Getters
    pub fn id(&self) -> &SecuritiesAccountId {
        &self.id
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    pub fn account_type(&self) -> SecuritiesAccountType {
        self.account_type
    }

    pub fn custodian_bank(&self) -> &str {
        &self.custodian_bank
    }

    pub fn status(&self) -> SecuritiesAccountStatus {
        self.status
    }

    pub fn holdings(&self) -> &[SecurityHolding] {
        &self.holdings
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securities_account_new() {
        let customer_id = CustomerId::new();
        let account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "BNA",
        );
        assert!(account.is_ok());
        let acc = account.unwrap();
        assert!(acc.is_operational());
        assert_eq!(acc.holdings_count(), 0);
    }

    #[test]
    fn test_securities_account_empty_account_number() {
        let customer_id = CustomerId::new();
        let account = SecuritiesAccount::new(
            customer_id,
            "",
            SecuritiesAccountType::Individual,
            "BNA",
        );
        assert!(account.is_err());
    }

    #[test]
    fn test_securities_account_empty_custodian() {
        let customer_id = CustomerId::new();
        let account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "",
        );
        assert!(account.is_err());
    }

    #[test]
    fn test_security_holding_new() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let holding = SecurityHolding::new(
            account_id,
            isin,
            "Tunisie Telecom",
            SecurityType::Equity,
            100,
            45.50,
            5000.0,
        );
        assert!(holding.is_ok());
        let h = holding.unwrap();
        assert_eq!(h.quantity(), 100);
        assert_eq!(h.market_value(), 5000.0);
    }

    #[test]
    fn test_security_holding_zero_quantity() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let holding = SecurityHolding::new(
            account_id,
            isin,
            "Tunisie Telecom",
            SecurityType::Equity,
            0,
            45.50,
            5000.0,
        );
        assert!(holding.is_err());
    }

    #[test]
    fn test_security_holding_can_sell() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let holding = SecurityHolding::new(
            account_id,
            isin,
            "Tunisie Telecom",
            SecurityType::Equity,
            100,
            45.50,
            5000.0,
        )
        .unwrap();

        assert!(holding.can_sell(50));
        assert!(holding.can_sell(100));
        assert!(!holding.can_sell(101));
        assert!(!holding.can_sell(0));
    }

    #[test]
    fn test_security_holding_unrealized_pnl() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let holding = SecurityHolding::new(
            account_id,
            isin,
            "Tunisie Telecom",
            SecurityType::Equity,
            100,
            40.0,
            5500.0, // 45.50 per share
        )
        .unwrap();

        let expected_pnl = 5500.0 - (40.0 * 100.0);
        assert_eq!(holding.unrealized_pnl(), expected_pnl);
    }

    #[test]
    fn test_trade_order_new() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let order = TradeOrder::new(
            account_id,
            OrderType::Buy,
            isin,
            100,
            PriceType::Limit,
            Some(45.50),
        );
        assert!(order.is_ok());
        let o = order.unwrap();
        assert!(o.is_pending());
        assert_eq!(o.remaining_quantity(), 100);
    }

    #[test]
    fn test_trade_order_zero_quantity() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let order = TradeOrder::new(
            account_id,
            OrderType::Buy,
            isin,
            0,
            PriceType::Market,
            None,
        );
        assert!(order.is_err());
    }

    #[test]
    fn test_trade_order_limit_without_price() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let order = TradeOrder::new(
            account_id,
            OrderType::Buy,
            isin,
            100,
            PriceType::Limit,
            None,
        );
        assert!(order.is_err());
    }

    #[test]
    fn test_trade_order_execute() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let mut order = TradeOrder::new(
            account_id,
            OrderType::Buy,
            isin,
            100,
            PriceType::Market,
            None,
        )
        .unwrap();

        let exec = order.execute(50, 45.50);
        assert!(exec.is_ok());
        assert_eq!(order.executed_quantity(), 50);
        assert_eq!(order.fill_percentage(), 50.0);
        assert!(!order.is_fully_executed());

        let exec2 = order.execute(50, 46.00);
        assert!(exec2.is_ok());
        assert!(order.is_fully_executed());
        assert_eq!(order.average_execution_price().unwrap(), 45.75);
    }

    #[test]
    fn test_trade_order_cancel() {
        let account_id = SecuritiesAccountId::new();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let mut order = TradeOrder::new(
            account_id,
            OrderType::Buy,
            isin,
            100,
            PriceType::Market,
            None,
        )
        .unwrap();

        assert!(order.cancel().is_ok());
        assert_eq!(order.status(), OrderStatus::Cancelled);
    }

    #[test]
    fn test_settlement_new() {
        let order_id = TradeOrderId::new();
        let settlement = Settlement::new(order_id, chrono::NaiveDate::from_ymd_opt(2026, 4, 9).unwrap(), "BNP Paribas");
        assert!(settlement.is_ok());
        let s = settlement.unwrap();
        assert_eq!(s.status(), SettlementStatus::Pending);
        assert_eq!(s.settlement_type(), SettlementType::DVP);
    }

    #[test]
    fn test_settlement_empty_counterparty() {
        let order_id = TradeOrderId::new();
        let settlement = Settlement::new(order_id, chrono::NaiveDate::from_ymd_opt(2026, 4, 9).unwrap(), "");
        assert!(settlement.is_err());
    }

    #[test]
    fn test_settlement_state_transitions() {
        let order_id = TradeOrderId::new();
        let mut settlement = Settlement::new(order_id, chrono::NaiveDate::from_ymd_opt(2026, 4, 9).unwrap(), "BNP Paribas").unwrap();

        assert!(settlement.match_settlement().is_ok());
        assert_eq!(settlement.status(), SettlementStatus::Matched);

        assert!(settlement.settle().is_ok());
        assert_eq!(settlement.status(), SettlementStatus::Settled);
    }

    #[test]
    fn test_corporate_action_new() {
        let isin = IsinCode::new("TN0123456789").unwrap();
        let record_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
        let ex_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 2).unwrap();
        let pay_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 15).unwrap();

        let action = CorporateAction::new(
            isin,
            CorporateActionType::Dividend,
            record_date,
            ex_date,
            pay_date,
            2.50,
        );
        assert!(action.is_ok());
        let a = action.unwrap();
        assert_eq!(a.status(), CorporateActionStatus::Announced);
    }

    #[test]
    fn test_corporate_action_invalid_dates() {
        let isin = IsinCode::new("TN0123456789").unwrap();
        let record_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 10).unwrap();
        let ex_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 5).unwrap();
        let pay_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 15).unwrap();

        let action = CorporateAction::new(
            isin,
            CorporateActionType::Dividend,
            record_date,
            ex_date,
            pay_date,
            2.50,
        );
        assert!(action.is_err());
    }

    #[test]
    fn test_corporate_action_state_transitions() {
        let isin = IsinCode::new("TN0123456789").unwrap();
        let record_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
        let ex_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 2).unwrap();
        let pay_date = chrono::NaiveDate::from_ymd_opt(2026, 5, 15).unwrap();

        let mut action = CorporateAction::new(
            isin,
            CorporateActionType::Dividend,
            record_date,
            ex_date,
            pay_date,
            2.50,
        )
        .unwrap();

        assert!(action.make_effective().is_ok());
        assert_eq!(action.status(), CorporateActionStatus::Effective);

        assert!(action.complete().is_ok());
        assert_eq!(action.status(), CorporateActionStatus::Completed);
    }

    #[test]
    fn test_portfolio_valuation_new() {
        let account_id = SecuritiesAccountId::new();
        let valuation = PortfolioValuation::new(
            account_id,
            10000.0,
            9000.0,
            "TND",
        );
        assert!(valuation.is_ok());
        let v = valuation.unwrap();
        assert_eq!(v.total_market_value(), 10000.0);
        assert_eq!(v.total_cost_basis(), 9000.0);
        assert_eq!(v.total_unrealized_pnl(), 1000.0);
        assert_eq!(v.pnl_percentage(), (1000.0 / 9000.0) * 100.0);
    }

    #[test]
    fn test_portfolio_valuation_zero_cost_basis() {
        let account_id = SecuritiesAccountId::new();
        let valuation = PortfolioValuation::new(
            account_id,
            5000.0,
            0.0,
            "TND",
        );
        assert!(valuation.is_ok());
        let v = valuation.unwrap();
        assert_eq!(v.pnl_percentage(), 0.0);
    }

    #[test]
    fn test_securities_account_add_holding() {
        let customer_id = CustomerId::new();
        let mut account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "BNA",
        )
        .unwrap();

        let account_id = account.id().clone();
        let isin = IsinCode::new("TN0123456789").unwrap();
        let holding = SecurityHolding::new(
            account_id,
            isin,
            "Tunisie Telecom",
            SecurityType::Equity,
            100,
            45.50,
            5000.0,
        )
        .unwrap();

        assert!(account.add_or_update_holding(holding).is_ok());
        assert_eq!(account.holdings_count(), 1);
    }

    #[test]
    fn test_securities_account_suspend() {
        let customer_id = CustomerId::new();
        let mut account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "BNA",
        )
        .unwrap();

        assert!(account.suspend().is_ok());
        assert_eq!(account.status(), SecuritiesAccountStatus::Suspended);
        assert!(!account.is_operational());
    }

    #[test]
    fn test_securities_account_close() {
        let customer_id = CustomerId::new();
        let mut account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "BNA",
        )
        .unwrap();

        assert!(account.close().is_ok());
        assert_eq!(account.status(), SecuritiesAccountStatus::Closed);
    }

    #[test]
    fn test_securities_account_reactivate() {
        let customer_id = CustomerId::new();
        let mut account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "BNA",
        )
        .unwrap();

        assert!(account.suspend().is_ok());
        assert!(account.reactivate().is_ok());
        assert_eq!(account.status(), SecuritiesAccountStatus::Active);
    }

    #[test]
    fn test_securities_account_total_market_value() {
        let customer_id = CustomerId::new();
        let mut account = SecuritiesAccount::new(
            customer_id,
            "ACC-001",
            SecuritiesAccountType::Individual,
            "BNA",
        )
        .unwrap();

        let account_id = account.id().clone();
        let isin1 = IsinCode::new("TN0123456789").unwrap();
        let isin2 = IsinCode::new("FR0000000000").unwrap();

        let holding1 = SecurityHolding::new(
            account_id.clone(),
            isin1,
            "Tunisie Telecom",
            SecurityType::Equity,
            100,
            45.50,
            5000.0,
        )
        .unwrap();

        let holding2 = SecurityHolding::new(
            account_id.clone(),
            isin2,
            "LVMH",
            SecurityType::Equity,
            50,
            150.0,
            8000.0,
        )
        .unwrap();

        account.add_or_update_holding(holding1).unwrap();
        account.add_or_update_holding(holding2).unwrap();

        assert_eq!(account.total_market_value(), 13000.0);
        assert_eq!(account.total_cost_basis(), 12050.0);
        assert_eq!(account.total_unrealized_pnl(), 950.0);
    }
}
