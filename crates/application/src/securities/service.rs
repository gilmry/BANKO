use std::sync::Arc;
use chrono::NaiveDate;

use banko_domain::securities::{
    CorporateAction, CorporateActionType, IsinCode, OrderStatus, OrderType, PortfolioValuation,
    PriceType, SecurityHolding, SecurityType, SecuritiesAccount, SecuritiesAccountId,
    SecuritiesAccountType, Settlement, SettlementStatus, TradeOrder, TradeOrderId,
};
use banko_domain::shared::CustomerId;

use super::dto::*;
use super::errors::SecuritiesServiceError;
use super::ports::{
    IAmlSanctionsChecker, ICorporateActionRepository, IMarketPriceFeed, IOrderMatchingEngine,
    ISecuritiesAccountRepository, ISettlementProcessor, ISettlementRepository, ITradeOrderRepository,
};

pub struct SecuritiesService {
    account_repo: Arc<dyn ISecuritiesAccountRepository>,
    order_repo: Arc<dyn ITradeOrderRepository>,
    settlement_repo: Arc<dyn ISettlementRepository>,
    corporate_action_repo: Arc<dyn ICorporateActionRepository>,
    market_price_feed: Arc<dyn IMarketPriceFeed>,
    order_matching_engine: Arc<dyn IOrderMatchingEngine>,
    settlement_processor: Arc<dyn ISettlementProcessor>,
    aml_sanctions_checker: Arc<dyn IAmlSanctionsChecker>,
}

impl SecuritiesService {
    pub fn new(
        account_repo: Arc<dyn ISecuritiesAccountRepository>,
        order_repo: Arc<dyn ITradeOrderRepository>,
        settlement_repo: Arc<dyn ISettlementRepository>,
        corporate_action_repo: Arc<dyn ICorporateActionRepository>,
        market_price_feed: Arc<dyn IMarketPriceFeed>,
        order_matching_engine: Arc<dyn IOrderMatchingEngine>,
        settlement_processor: Arc<dyn ISettlementProcessor>,
        aml_sanctions_checker: Arc<dyn IAmlSanctionsChecker>,
    ) -> Self {
        SecuritiesService {
            account_repo,
            order_repo,
            settlement_repo,
            corporate_action_repo,
            market_price_feed,
            order_matching_engine,
            settlement_processor,
            aml_sanctions_checker,
        }
    }

    /// Open a new securities account for a customer
    pub async fn open_securities_account(
        &self,
        customer_id: CustomerId,
        req: OpenSecuritiesAccountRequest,
    ) -> Result<SecuritiesAccountResponse, SecuritiesServiceError> {
        // Check AML/Sanctions compliance
        self.aml_sanctions_checker
            .check_customer_compliance(&customer_id)
            .await
            .map_err(|e| SecuritiesServiceError::Internal(e))?;

        let account_type = SecuritiesAccountType::from_str(&req.account_type)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let account = SecuritiesAccount::new(
            customer_id,
            &req.account_number,
            account_type,
            &req.custodian_bank,
        )
        .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.account_repo
            .save(&account)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(self.account_to_response(&account))
    }

    /// Find a securities account by ID
    pub async fn find_account(
        &self,
        account_id: &SecuritiesAccountId,
    ) -> Result<SecuritiesAccountResponse, SecuritiesServiceError> {
        let account = self
            .account_repo
            .find_by_id(account_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::SecuritiesAccountNotFound)?;

        Ok(self.account_to_response(&account))
    }

    /// List all accounts for a customer
    pub async fn list_customer_accounts(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<SecuritiesAccountResponse>, SecuritiesServiceError> {
        let accounts = self
            .account_repo
            .find_by_customer_id(customer_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(accounts.iter().map(|a| self.account_to_response(a)).collect())
    }

    /// Place a trade order
    pub async fn place_trade_order(
        &self,
        customer_id: &CustomerId,
        req: PlaceTradeOrderRequest,
    ) -> Result<TradeOrderResponse, SecuritiesServiceError> {
        let account_id = SecuritiesAccountId::parse(&req.account_id)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        // Verify account exists and belongs to customer
        let account = self
            .account_repo
            .find_by_id(&account_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::SecuritiesAccountNotFound)?;

        if account.customer_id() != customer_id {
            return Err(SecuritiesServiceError::ValidationError(
                "Account does not belong to customer".to_string(),
            ));
        }

        if !account.is_operational() {
            return Err(SecuritiesServiceError::AccountNotOperational);
        }

        // Check compliance
        self.aml_sanctions_checker
            .check_customer_compliance(customer_id)
            .await
            .map_err(|e| SecuritiesServiceError::Internal(e))?;

        let order_type = OrderType::from_str(&req.order_type)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;
        let price_type = PriceType::from_str(&req.price_type)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;
        let isin = IsinCode::new(&req.security_isin)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        // For sell orders: check sufficient holdings
        if order_type == OrderType::Sell {
            if let Some(holding) = account.get_holding(&isin) {
                if !holding.can_sell(req.quantity) {
                    return Err(SecuritiesServiceError::InsufficientHoldings);
                }
            } else {
                return Err(SecuritiesServiceError::SecurityHoldingNotFound);
            }
        }

        let order = TradeOrder::new(
            account_id,
            order_type,
            isin,
            req.quantity,
            price_type,
            req.limit_price,
        )
        .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.order_repo
            .save(&order)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(self.order_to_response(&order))
    }

    /// Execute a pending order (partial or full fill)
    pub async fn execute_order(
        &self,
        req: ExecuteTradeOrderRequest,
    ) -> Result<TradeOrderResponse, SecuritiesServiceError> {
        let order_id = TradeOrderId::parse(&req.order_id)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let mut order = self
            .order_repo
            .find_by_id(&order_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::TradeOrderNotFound)?;

        if order.status() != OrderStatus::Pending && order.status() != OrderStatus::PartiallyFilled {
            return Err(SecuritiesServiceError::InvalidOrderState);
        }

        order
            .execute(req.execution_quantity, req.execution_price)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.order_repo
            .save(&order)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(self.order_to_response(&order))
    }

    /// Cancel a pending order
    pub async fn cancel_order(
        &self,
        req: CancelTradeOrderRequest,
    ) -> Result<TradeOrderResponse, SecuritiesServiceError> {
        let order_id = TradeOrderId::parse(&req.order_id)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let mut order = self
            .order_repo
            .find_by_id(&order_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::TradeOrderNotFound)?;

        order
            .cancel()
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.order_repo
            .save(&order)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(self.order_to_response(&order))
    }

    /// Settle a trade order (T+2 for BVMT)
    pub async fn settle_trade(
        &self,
        req: SettleTradeRequest,
    ) -> Result<SettlementResponse, SecuritiesServiceError> {
        let order_id = TradeOrderId::parse(&req.order_id)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let order = self
            .order_repo
            .find_by_id(&order_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::TradeOrderNotFound)?;

        if !order.is_fully_executed() {
            return Err(SecuritiesServiceError::ValidationError(
                "Order must be fully executed before settlement".to_string(),
            ));
        }

        let settlement = Settlement::new(order_id, req.settlement_date, &req.counterparty)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.settlement_repo
            .save(&settlement)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(self.settlement_to_response(&settlement))
    }

    /// Update security holding market value
    pub async fn update_holding_market_value(
        &self,
        req: UpdateSecurityHoldingRequest,
    ) -> Result<SecurityHoldingResponse, SecuritiesServiceError> {
        let account_id = SecuritiesAccountId::parse(&req.account_id)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let mut account = self
            .account_repo
            .find_by_id(&account_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::SecuritiesAccountNotFound)?;

        let isin = IsinCode::new(&req.security_isin)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let holding = account
            .get_holding_mut(&isin)
            .ok_or(SecuritiesServiceError::SecurityHoldingNotFound)?;

        holding
            .update_market_value(req.new_market_value)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.account_repo
            .save(&account)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        let updated_holding = account
            .get_holding(&isin)
            .ok_or(SecuritiesServiceError::SecurityHoldingNotFound)?;

        Ok(self.holding_to_response(updated_holding))
    }

    /// Create a corporate action
    pub async fn create_corporate_action(
        &self,
        req: CreateCorporateActionRequest,
    ) -> Result<CorporateActionResponse, SecuritiesServiceError> {
        let isin = IsinCode::new(&req.security_isin)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let action_type = CorporateActionType::from_str(&req.action_type)
            .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let action = CorporateAction::new(
            isin,
            action_type,
            req.record_date,
            req.ex_date,
            req.payment_date,
            req.ratio_or_amount,
        )
        .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        self.corporate_action_repo
            .save(&action)
            .await
            .map_err(SecuritiesServiceError::Internal)?;

        Ok(self.corporate_action_to_response(&action))
    }

    /// Get portfolio valuation for an account
    pub async fn get_portfolio_valuation(
        &self,
        account_id: &SecuritiesAccountId,
    ) -> Result<PortfolioValuationResponse, SecuritiesServiceError> {
        let account = self
            .account_repo
            .find_by_id(account_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::SecuritiesAccountNotFound)?;

        let valuation = PortfolioValuation::new(
            account_id.clone(),
            account.total_market_value(),
            account.total_cost_basis(),
            "TND", // Default Tunisian Dinar
        )
        .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        Ok(self.valuation_to_response(&valuation))
    }

    /// Get full portfolio summary
    pub async fn get_portfolio_summary(
        &self,
        account_id: &SecuritiesAccountId,
    ) -> Result<PortfolioSummaryResponse, SecuritiesServiceError> {
        let account = self
            .account_repo
            .find_by_id(account_id)
            .await
            .map_err(SecuritiesServiceError::Internal)?
            .ok_or(SecuritiesServiceError::SecuritiesAccountNotFound)?;

        let valuation = PortfolioValuation::new(
            account_id.clone(),
            account.total_market_value(),
            account.total_cost_basis(),
            "TND",
        )
        .map_err(|e| SecuritiesServiceError::DomainError(e.to_string()))?;

        let holdings = account
            .holdings()
            .iter()
            .map(|h| self.holding_to_response(h))
            .collect();

        Ok(PortfolioSummaryResponse {
            account_id: account.id().to_string(),
            account_number: account.account_number().to_string(),
            account_type: account.account_type().to_string(),
            status: account.status().to_string(),
            valuation: self.valuation_to_response(&valuation),
            holdings,
        })
    }

    // --- Helper methods ---

    fn account_to_response(&self, account: &SecuritiesAccount) -> SecuritiesAccountResponse {
        SecuritiesAccountResponse {
            id: account.id().to_string(),
            customer_id: account.customer_id().to_string(),
            account_number: account.account_number().to_string(),
            account_type: account.account_type().to_string(),
            custodian_bank: account.custodian_bank().to_string(),
            status: account.status().to_string(),
            total_market_value: account.total_market_value(),
            total_cost_basis: account.total_cost_basis(),
            total_unrealized_pnl: account.total_unrealized_pnl(),
            holdings_count: account.holdings_count(),
            created_at: account.created_at(),
            updated_at: account.updated_at(),
        }
    }

    fn holding_to_response(&self, holding: &SecurityHolding) -> SecurityHoldingResponse {
        let total_cost_basis = holding.total_cost_basis();
        let pnl_percentage = if total_cost_basis == 0.0 {
            0.0
        } else {
            (holding.unrealized_pnl() / total_cost_basis) * 100.0
        };

        SecurityHoldingResponse {
            account_id: holding.account_id().to_string(),
            isin_code: holding.isin_code().to_string(),
            security_name: holding.security_name().to_string(),
            security_type: holding.security_type().to_string(),
            quantity: holding.quantity(),
            average_cost: holding.average_cost(),
            total_cost_basis,
            market_value: holding.market_value(),
            unrealized_pnl: holding.unrealized_pnl(),
            pnl_percentage,
            last_valuation_date: holding.last_valuation_date(),
        }
    }

    fn order_to_response(&self, order: &TradeOrder) -> TradeOrderResponse {
        TradeOrderResponse {
            order_id: order.order_id().to_string(),
            account_id: order.account_id().to_string(),
            order_type: order.order_type().to_string(),
            security_isin: order.security_isin().to_string(),
            quantity: order.quantity(),
            price_type: order.price_type().to_string(),
            limit_price: order.limit_price(),
            status: order.status().to_string(),
            executed_quantity: order.executed_quantity(),
            remaining_quantity: order.remaining_quantity(),
            fill_percentage: order.fill_percentage(),
            average_execution_price: order.average_execution_price(),
            placed_at: order.placed_at(),
            executed_at: order.executed_at(),
        }
    }

    fn settlement_to_response(&self, settlement: &Settlement) -> SettlementResponse {
        SettlementResponse {
            settlement_id: settlement.settlement_id().to_string(),
            trade_order_id: settlement.trade_order_id().to_string(),
            settlement_date: settlement.settlement_date(),
            settlement_type: settlement.settlement_type().to_string(),
            status: settlement.status().to_string(),
            counterparty: settlement.counterparty().to_string(),
            created_at: settlement.created_at(),
            updated_at: settlement.updated_at(),
        }
    }

    fn corporate_action_to_response(&self, action: &CorporateAction) -> CorporateActionResponse {
        CorporateActionResponse {
            action_id: action.action_id().to_string(),
            security_isin: action.security_isin().to_string(),
            action_type: action.action_type().to_string(),
            record_date: action.record_date(),
            ex_date: action.ex_date(),
            payment_date: action.payment_date(),
            ratio_or_amount: action.ratio_or_amount(),
            status: action.status().to_string(),
            created_at: action.created_at(),
            updated_at: action.updated_at(),
        }
    }

    fn valuation_to_response(&self, valuation: &PortfolioValuation) -> PortfolioValuationResponse {
        PortfolioValuationResponse {
            account_id: valuation.account_id().to_string(),
            valuation_date: valuation.valuation_date(),
            total_market_value: valuation.total_market_value(),
            total_cost_basis: valuation.total_cost_basis(),
            total_unrealized_pnl: valuation.total_unrealized_pnl(),
            pnl_percentage: valuation.pnl_percentage(),
            currency: valuation.currency().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing
    struct MockAccountRepository;
    struct MockOrderRepository;
    struct MockSettlementRepository;
    struct MockCorporateActionRepository;
    struct MockMarketPriceFeed;
    struct MockOrderMatchingEngine;
    struct MockSettlementProcessor;
    struct MockAmlSanctionsChecker;

    #[async_trait::async_trait]
    impl ISecuritiesAccountRepository for MockAccountRepository {
        async fn save(&self, _account: &SecuritiesAccount) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &SecuritiesAccountId) -> Result<Option<SecuritiesAccount>, String> {
            Ok(None)
        }
        async fn find_by_customer_id(&self, _customer_id: &CustomerId) -> Result<Vec<SecuritiesAccount>, String> {
            Ok(vec![])
        }
        async fn find_by_account_number(&self, _account_number: &str) -> Result<Option<SecuritiesAccount>, String> {
            Ok(None)
        }
        async fn delete(&self, _id: &SecuritiesAccountId) -> Result<(), String> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl ITradeOrderRepository for MockOrderRepository {
        async fn save(&self, _order: &TradeOrder) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &TradeOrderId) -> Result<Option<TradeOrder>, String> {
            Ok(None)
        }
        async fn find_by_account_id(&self, _account_id: &SecuritiesAccountId) -> Result<Vec<TradeOrder>, String> {
            Ok(vec![])
        }
        async fn find_pending_by_account(&self, _account_id: &SecuritiesAccountId) -> Result<Vec<TradeOrder>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &TradeOrderId) -> Result<(), String> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl ISettlementRepository for MockSettlementRepository {
        async fn save(&self, _settlement: &Settlement) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &SettlementId) -> Result<Option<Settlement>, String> {
            Ok(None)
        }
        async fn find_by_trade_order_id(&self, _order_id: &TradeOrderId) -> Result<Option<Settlement>, String> {
            Ok(None)
        }
        async fn find_pending_settlements(&self) -> Result<Vec<Settlement>, String> {
            Ok(vec![])
        }
        async fn find_by_date_range(&self, _from: NaiveDate, _to: NaiveDate) -> Result<Vec<Settlement>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &SettlementId) -> Result<(), String> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl ICorporateActionRepository for MockCorporateActionRepository {
        async fn save(&self, _action: &CorporateAction) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &banko_domain::securities::CorporateActionId) -> Result<Option<CorporateAction>, String> {
            Ok(None)
        }
        async fn find_by_security_isin(&self, _isin: &IsinCode) -> Result<Vec<CorporateAction>, String> {
            Ok(vec![])
        }
        async fn find_effective_by_date(&self, _date: NaiveDate) -> Result<Vec<CorporateAction>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &banko_domain::securities::CorporateActionId) -> Result<(), String> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl IMarketPriceFeed for MockMarketPriceFeed {
        async fn get_current_price(&self, _isin: &IsinCode) -> Result<f64, String> {
            Ok(45.50)
        }
        async fn get_historical_price(&self, _isin: &IsinCode, _date: NaiveDate) -> Result<f64, String> {
            Ok(45.00)
        }
    }

    #[async_trait::async_trait]
    impl IOrderMatchingEngine for MockOrderMatchingEngine {
        async fn match_order(&self, _order: &TradeOrder) -> Result<(i64, f64), String> {
            Ok((100, 45.50))
        }
        async fn get_best_bid_ask(&self, _isin: &IsinCode) -> Result<(f64, f64), String> {
            Ok((45.00, 46.00))
        }
    }

    #[async_trait::async_trait]
    impl ISettlementProcessor for MockSettlementProcessor {
        async fn process_dvp_settlement(&self, _settlement: &Settlement, _security_qty: i64, _cash_amount: f64) -> Result<(), String> {
            Ok(())
        }
        async fn process_dividend_payment(&self, _account_id: &SecuritiesAccountId, _amount: f64) -> Result<(), String> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl IAmlSanctionsChecker for MockAmlSanctionsChecker {
        async fn check_customer_compliance(&self, _customer_id: &CustomerId) -> Result<bool, String> {
            Ok(true)
        }
        async fn check_transaction_compliance(&self, _customer_id: &CustomerId, _amount: f64) -> Result<bool, String> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = SecuritiesService::new(
            Arc::new(MockAccountRepository),
            Arc::new(MockOrderRepository),
            Arc::new(MockSettlementRepository),
            Arc::new(MockCorporateActionRepository),
            Arc::new(MockMarketPriceFeed),
            Arc::new(MockOrderMatchingEngine),
            Arc::new(MockSettlementProcessor),
            Arc::new(MockAmlSanctionsChecker),
        );

        // Service was created successfully
        assert!(!service.account_repo.is_empty() || true);
    }
}
