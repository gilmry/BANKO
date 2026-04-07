use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::securities::{
    ICorporateActionRepository, ISecuritiesAccountRepository, ISettlementRepository,
    ITradeOrderRepository,
};
use banko_domain::securities::{
    CorporateAction, CorporateActionId, CorporateActionStatus, CorporateActionType, IsinCode,
    OrderStatus, OrderType, PriceType, SecurityHolding, SecurityType, SecuritiesAccount,
    SecuritiesAccountId, SecuritiesAccountStatus, SecuritiesAccountType, Settlement,
    SettlementId, SettlementStatus, SettlementType, TradeOrder, TradeOrderId,
};
use banko_domain::shared::CustomerId;

// --- Securities Account Repository ---

pub struct PgSecuritiesAccountRepository {
    pool: PgPool,
}

impl PgSecuritiesAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSecuritiesAccountRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SecuritiesAccountRow {
    id: Uuid,
    customer_id: Uuid,
    account_number: String,
    account_type: String,
    custodian_bank: String,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
struct SecurityHoldingRow {
    account_id: Uuid,
    isin_code: String,
    security_name: String,
    security_type: String,
    quantity: i64,
    average_cost: f64,
    market_value: f64,
    unrealized_pnl: f64,
    last_valuation_date: chrono::DateTime<chrono::Utc>,
}

impl SecurityHoldingRow {
    fn into_domain(self) -> Result<SecurityHolding, String> {
        let isin = IsinCode::new(&self.isin_code).map_err(|e| e.to_string())?;
        let security_type =
            SecurityType::from_str(&self.security_type).map_err(|e| e.to_string())?;

        Ok(SecurityHolding::reconstitute(
            SecuritiesAccountId::from_uuid(self.account_id),
            isin,
            self.security_name,
            security_type,
            self.quantity,
            self.average_cost,
            self.market_value,
            self.unrealized_pnl,
            self.last_valuation_date,
        ))
    }
}

impl SecuritiesAccountRow {
    fn into_domain(self, holdings: Vec<SecurityHolding>) -> Result<SecuritiesAccount, String> {
        let account_type =
            SecuritiesAccountType::from_str(&self.account_type).map_err(|e| e.to_string())?;
        let status =
            SecuritiesAccountStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(SecuritiesAccount::reconstitute(
            SecuritiesAccountId::from_uuid(self.id),
            CustomerId::from_uuid(self.customer_id),
            self.account_number,
            account_type,
            self.custodian_bank,
            status,
            holdings,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl ISecuritiesAccountRepository for PgSecuritiesAccountRepository {
    async fn save(&self, account: &SecuritiesAccount) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // Save account
        sqlx::query(
            r#"
            INSERT INTO securities_accounts (id, customer_id, account_number, account_type, custodian_bank, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                account_type = $4,
                custodian_bank = $5,
                status = $6,
                updated_at = $8
            "#,
        )
        .bind(account.id().as_uuid())
        .bind(account.customer_id().as_uuid())
        .bind(account.account_number())
        .bind(account.account_type().as_str())
        .bind(account.custodian_bank())
        .bind(account.status().as_str())
        .bind(account.created_at())
        .bind(account.updated_at())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Save holdings
        for holding in account.holdings() {
            sqlx::query(
                r#"
                INSERT INTO security_holdings (account_id, isin_code, security_name, security_type, quantity, average_cost, market_value, unrealized_pnl, last_valuation_date)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (account_id, isin_code) DO UPDATE SET
                    security_name = $3,
                    security_type = $4,
                    quantity = $5,
                    average_cost = $6,
                    market_value = $7,
                    unrealized_pnl = $8,
                    last_valuation_date = $9
                "#,
            )
            .bind(holding.account_id().as_uuid())
            .bind(holding.isin_code().as_str())
            .bind(holding.security_name())
            .bind(holding.security_type().as_str())
            .bind(holding.quantity())
            .bind(holding.average_cost())
            .bind(holding.market_value())
            .bind(holding.unrealized_pnl())
            .bind(holding.last_valuation_date())
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &SecuritiesAccountId) -> Result<Option<SecuritiesAccount>, String> {
        let account_row: Option<SecuritiesAccountRow> = sqlx::query_as(
            "SELECT id, customer_id, account_number, account_type, custodian_bank, status, created_at, updated_at FROM securities_accounts WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match account_row {
            None => Ok(None),
            Some(row) => {
                let holdings_rows: Vec<SecurityHoldingRow> = sqlx::query_as(
                    "SELECT account_id, isin_code, security_name, security_type, quantity, average_cost, market_value, unrealized_pnl, last_valuation_date FROM security_holdings WHERE account_id = $1"
                )
                .bind(id.as_uuid())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let holdings: Result<Vec<SecurityHolding>, String> =
                    holdings_rows.into_iter().map(|r| r.into_domain()).collect();

                let account = row.into_domain(holdings?)?;
                Ok(Some(account))
            }
        }
    }

    async fn find_by_customer_id(&self, customer_id: &CustomerId) -> Result<Vec<SecuritiesAccount>, String> {
        let account_rows: Vec<SecuritiesAccountRow> = sqlx::query_as(
            "SELECT id, customer_id, account_number, account_type, custodian_bank, status, created_at, updated_at FROM securities_accounts WHERE customer_id = $1 ORDER BY created_at DESC"
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut accounts = Vec::new();
        for row in account_rows {
            let holdings_rows: Vec<SecurityHoldingRow> = sqlx::query_as(
                "SELECT account_id, isin_code, security_name, security_type, quantity, average_cost, market_value, unrealized_pnl, last_valuation_date FROM security_holdings WHERE account_id = $1"
            )
            .bind(row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let holdings: Result<Vec<SecurityHolding>, String> =
                holdings_rows.into_iter().map(|r| r.into_domain()).collect();

            accounts.push(row.into_domain(holdings?)?);
        }

        Ok(accounts)
    }

    async fn find_by_account_number(&self, account_number: &str) -> Result<Option<SecuritiesAccount>, String> {
        let account_row: Option<SecuritiesAccountRow> = sqlx::query_as(
            "SELECT id, customer_id, account_number, account_type, custodian_bank, status, created_at, updated_at FROM securities_accounts WHERE account_number = $1"
        )
        .bind(account_number)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match account_row {
            None => Ok(None),
            Some(row) => {
                let holdings_rows: Vec<SecurityHoldingRow> = sqlx::query_as(
                    "SELECT account_id, isin_code, security_name, security_type, quantity, average_cost, market_value, unrealized_pnl, last_valuation_date FROM security_holdings WHERE account_id = $1"
                )
                .bind(row.id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let holdings: Result<Vec<SecurityHolding>, String> =
                    holdings_rows.into_iter().map(|r| r.into_domain()).collect();

                let account = row.into_domain(holdings?)?;
                Ok(Some(account))
            }
        }
    }

    async fn delete(&self, id: &SecuritiesAccountId) -> Result<(), String> {
        sqlx::query("DELETE FROM security_holdings WHERE account_id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("DELETE FROM securities_accounts WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Trade Order Repository ---

pub struct PgTradeOrderRepository {
    pool: PgPool,
}

impl PgTradeOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        PgTradeOrderRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct TradeOrderRow {
    order_id: Uuid,
    account_id: Uuid,
    order_type: String,
    security_isin: String,
    quantity: i64,
    price_type: String,
    limit_price: Option<f64>,
    status: String,
    executed_quantity: i64,
    average_execution_price: Option<f64>,
    placed_at: chrono::DateTime<chrono::Utc>,
    executed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TradeOrderRow {
    fn into_domain(self) -> Result<TradeOrder, String> {
        let order_type = OrderType::from_str(&self.order_type).map_err(|e| e.to_string())?;
        let price_type = PriceType::from_str(&self.price_type).map_err(|e| e.to_string())?;
        let security_isin = IsinCode::new(&self.security_isin).map_err(|e| e.to_string())?;
        let status = OrderStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(TradeOrder::reconstitute(
            TradeOrderId::from_uuid(self.order_id),
            SecuritiesAccountId::from_uuid(self.account_id),
            order_type,
            security_isin,
            self.quantity,
            price_type,
            self.limit_price,
            status,
            self.executed_quantity,
            self.average_execution_price,
            self.placed_at,
            self.executed_at,
        ))
    }
}

#[async_trait]
impl ITradeOrderRepository for PgTradeOrderRepository {
    async fn save(&self, order: &TradeOrder) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO trade_orders (order_id, account_id, order_type, security_isin, quantity, price_type, limit_price, status, executed_quantity, average_execution_price, placed_at, executed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (order_id) DO UPDATE SET
                status = $8,
                executed_quantity = $9,
                average_execution_price = $10,
                executed_at = $12
            "#,
        )
        .bind(order.order_id().as_uuid())
        .bind(order.account_id().as_uuid())
        .bind(order.order_type().as_str())
        .bind(order.security_isin().as_str())
        .bind(order.quantity())
        .bind(order.price_type().as_str())
        .bind(order.limit_price())
        .bind(order.status().as_str())
        .bind(order.executed_quantity())
        .bind(order.average_execution_price())
        .bind(order.placed_at())
        .bind(order.executed_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &TradeOrderId) -> Result<Option<TradeOrder>, String> {
        let row: Option<TradeOrderRow> = sqlx::query_as(
            "SELECT order_id, account_id, order_type, security_isin, quantity, price_type, limit_price, status, executed_quantity, average_execution_price, placed_at, executed_at FROM trade_orders WHERE order_id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            None => Ok(None),
            Some(r) => Ok(Some(r.into_domain()?)),
        }
    }

    async fn find_by_account_id(&self, account_id: &SecuritiesAccountId) -> Result<Vec<TradeOrder>, String> {
        let rows: Vec<TradeOrderRow> = sqlx::query_as(
            "SELECT order_id, account_id, order_type, security_isin, quantity, price_type, limit_price, status, executed_quantity, average_execution_price, placed_at, executed_at FROM trade_orders WHERE account_id = $1 ORDER BY placed_at DESC"
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_pending_by_account(&self, account_id: &SecuritiesAccountId) -> Result<Vec<TradeOrder>, String> {
        let rows: Vec<TradeOrderRow> = sqlx::query_as(
            "SELECT order_id, account_id, order_type, security_isin, quantity, price_type, limit_price, status, executed_quantity, average_execution_price, placed_at, executed_at FROM trade_orders WHERE account_id = $1 AND status IN ('Pending', 'PartiallyFilled') ORDER BY placed_at ASC"
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn delete(&self, id: &TradeOrderId) -> Result<(), String> {
        sqlx::query("DELETE FROM trade_orders WHERE order_id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Settlement Repository ---

pub struct PgSettlementRepository {
    pool: PgPool,
}

impl PgSettlementRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSettlementRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SettlementRow {
    settlement_id: Uuid,
    trade_order_id: Uuid,
    settlement_date: NaiveDate,
    settlement_type: String,
    status: String,
    counterparty: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl SettlementRow {
    fn into_domain(self) -> Result<Settlement, String> {
        let settlement_type = SettlementType::from_str(&self.settlement_type).map_err(|e| e.to_string())?;
        let status = SettlementStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(Settlement::reconstitute(
            SettlementId::from_uuid(self.settlement_id),
            TradeOrderId::from_uuid(self.trade_order_id),
            self.settlement_date,
            settlement_type,
            status,
            self.counterparty,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl ISettlementRepository for PgSettlementRepository {
    async fn save(&self, settlement: &Settlement) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO settlements (settlement_id, trade_order_id, settlement_date, settlement_type, status, counterparty, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (settlement_id) DO UPDATE SET
                status = $5,
                updated_at = $8
            "#,
        )
        .bind(settlement.settlement_id().as_uuid())
        .bind(settlement.trade_order_id().as_uuid())
        .bind(settlement.settlement_date())
        .bind(settlement.settlement_type().as_str())
        .bind(settlement.status().as_str())
        .bind(settlement.counterparty())
        .bind(settlement.created_at())
        .bind(settlement.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &SettlementId) -> Result<Option<Settlement>, String> {
        let row: Option<SettlementRow> = sqlx::query_as(
            "SELECT settlement_id, trade_order_id, settlement_date, settlement_type, status, counterparty, created_at, updated_at FROM settlements WHERE settlement_id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            None => Ok(None),
            Some(r) => Ok(Some(r.into_domain()?)),
        }
    }

    async fn find_by_trade_order_id(&self, order_id: &TradeOrderId) -> Result<Option<Settlement>, String> {
        let row: Option<SettlementRow> = sqlx::query_as(
            "SELECT settlement_id, trade_order_id, settlement_date, settlement_type, status, counterparty, created_at, updated_at FROM settlements WHERE trade_order_id = $1"
        )
        .bind(order_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            None => Ok(None),
            Some(r) => Ok(Some(r.into_domain()?)),
        }
    }

    async fn find_pending_settlements(&self) -> Result<Vec<Settlement>, String> {
        let rows: Vec<SettlementRow> = sqlx::query_as(
            "SELECT settlement_id, trade_order_id, settlement_date, settlement_type, status, counterparty, created_at, updated_at FROM settlements WHERE status IN ('Pending', 'Matched') ORDER BY settlement_date ASC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_date_range(&self, from: NaiveDate, to: NaiveDate) -> Result<Vec<Settlement>, String> {
        let rows: Vec<SettlementRow> = sqlx::query_as(
            "SELECT settlement_id, trade_order_id, settlement_date, settlement_type, status, counterparty, created_at, updated_at FROM settlements WHERE settlement_date >= $1 AND settlement_date <= $2 ORDER BY settlement_date ASC"
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn delete(&self, id: &SettlementId) -> Result<(), String> {
        sqlx::query("DELETE FROM settlements WHERE settlement_id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Corporate Action Repository ---

pub struct PgCorporateActionRepository {
    pool: PgPool,
}

impl PgCorporateActionRepository {
    pub fn new(pool: PgPool) -> Self {
        PgCorporateActionRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct CorporateActionRow {
    action_id: Uuid,
    security_isin: String,
    action_type: String,
    record_date: NaiveDate,
    ex_date: NaiveDate,
    payment_date: NaiveDate,
    ratio_or_amount: f64,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl CorporateActionRow {
    fn into_domain(self) -> Result<CorporateAction, String> {
        let isin = IsinCode::new(&self.security_isin).map_err(|e| e.to_string())?;
        let action_type = CorporateActionType::from_str(&self.action_type).map_err(|e| e.to_string())?;
        let status = CorporateActionStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(CorporateAction::reconstitute(
            CorporateActionId::from_uuid(self.action_id),
            isin,
            action_type,
            self.record_date,
            self.ex_date,
            self.payment_date,
            self.ratio_or_amount,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl ICorporateActionRepository for PgCorporateActionRepository {
    async fn save(&self, action: &CorporateAction) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO corporate_actions (action_id, security_isin, action_type, record_date, ex_date, payment_date, ratio_or_amount, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (action_id) DO UPDATE SET
                status = $8,
                updated_at = $10
            "#,
        )
        .bind(action.action_id().as_uuid())
        .bind(action.security_isin().as_str())
        .bind(action.action_type().as_str())
        .bind(action.record_date())
        .bind(action.ex_date())
        .bind(action.payment_date())
        .bind(action.ratio_or_amount())
        .bind(action.status().as_str())
        .bind(action.created_at())
        .bind(action.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CorporateActionId) -> Result<Option<CorporateAction>, String> {
        let row: Option<CorporateActionRow> = sqlx::query_as(
            "SELECT action_id, security_isin, action_type, record_date, ex_date, payment_date, ratio_or_amount, status, created_at, updated_at FROM corporate_actions WHERE action_id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            None => Ok(None),
            Some(r) => Ok(Some(r.into_domain()?)),
        }
    }

    async fn find_by_security_isin(&self, isin: &IsinCode) -> Result<Vec<CorporateAction>, String> {
        let rows: Vec<CorporateActionRow> = sqlx::query_as(
            "SELECT action_id, security_isin, action_type, record_date, ex_date, payment_date, ratio_or_amount, status, created_at, updated_at FROM corporate_actions WHERE security_isin = $1 ORDER BY record_date DESC"
        )
        .bind(isin.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_effective_by_date(&self, date: NaiveDate) -> Result<Vec<CorporateAction>, String> {
        let rows: Vec<CorporateActionRow> = sqlx::query_as(
            "SELECT action_id, security_isin, action_type, record_date, ex_date, payment_date, ratio_or_amount, status, created_at, updated_at FROM corporate_actions WHERE ex_date = $1 AND status IN ('Effective', 'Completed') ORDER BY security_isin ASC"
        )
        .bind(date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn delete(&self, id: &CorporateActionId) -> Result<(), String> {
        sqlx::query("DELETE FROM corporate_actions WHERE action_id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_order_row_conversion() {
        let row = TradeOrderRow {
            order_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            order_type: "Buy".to_string(),
            security_isin: "TN0123456789".to_string(),
            quantity: 100,
            price_type: "Market".to_string(),
            limit_price: None,
            status: "Pending".to_string(),
            executed_quantity: 0,
            average_execution_price: None,
            placed_at: chrono::Utc::now(),
            executed_at: None,
        };

        let result = row.into_domain();
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.quantity(), 100);
    }

    #[test]
    fn test_settlement_row_conversion() {
        let row = SettlementRow {
            settlement_id: Uuid::new_v4(),
            trade_order_id: Uuid::new_v4(),
            settlement_date: chrono::NaiveDate::from_ymd_opt(2026, 4, 9).unwrap(),
            settlement_type: "DVP".to_string(),
            status: "Pending".to_string(),
            counterparty: "BNP Paribas".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = row.into_domain();
        assert!(result.is_ok());
        let settlement = result.unwrap();
        assert_eq!(settlement.counterparty(), "BNP Paribas");
    }

    #[test]
    fn test_corporate_action_row_conversion() {
        let row = CorporateActionRow {
            action_id: Uuid::new_v4(),
            security_isin: "TN0123456789".to_string(),
            action_type: "Dividend".to_string(),
            record_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            ex_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
            payment_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 15).unwrap(),
            ratio_or_amount: 2.50,
            status: "Announced".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = row.into_domain();
        assert!(result.is_ok());
        let action = result.unwrap();
        assert_eq!(action.ratio_or_amount(), 2.50);
    }
}
