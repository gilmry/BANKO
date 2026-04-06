use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::payment::{
    IPaymentRepository, ISwiftMessageRepository, ITransferRepository,
};
use banko_domain::payment::*;

// ============================================================
// PostgreSQL Payment Order Repository
// ============================================================

pub struct PgPaymentRepository {
    pool: PgPool,
}

impl PgPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        PgPaymentRepository { pool }
    }
}

#[async_trait]
impl IPaymentRepository for PgPaymentRepository {
    async fn save(&self, order: &PaymentOrder) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO payment.payment_orders (
                id, sender_account_id, beneficiary_name, beneficiary_rib, beneficiary_bic,
                amount, currency, payment_type, status, screening_status,
                reference, description, rejection_reason,
                created_at, submitted_at, executed_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                screening_status = EXCLUDED.screening_status,
                rejection_reason = EXCLUDED.rejection_reason,
                submitted_at = EXCLUDED.submitted_at,
                executed_at = EXCLUDED.executed_at
            "#,
        )
        .bind(order.order_id().as_uuid())
        .bind(order.sender_account_id())
        .bind(order.beneficiary_name())
        .bind(order.beneficiary_rib())
        .bind(order.beneficiary_bic())
        .bind(order.amount())
        .bind(order.currency())
        .bind(order.payment_type().as_str())
        .bind(order.status().as_str())
        .bind(order.screening_status().as_str())
        .bind(order.reference())
        .bind(order.description())
        .bind(order.rejection_reason())
        .bind(order.created_at())
        .bind(order.submitted_at())
        .bind(order.executed_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &OrderId) -> Result<Option<PaymentOrder>, String> {
        let row = sqlx::query_as::<_, PaymentOrderRow>(
            r#"
            SELECT id, sender_account_id, beneficiary_name, beneficiary_rib, beneficiary_bic,
                   amount, currency, payment_type, status, screening_status,
                   reference, description, rejection_reason,
                   created_at, submitted_at, executed_at
            FROM payment.payment_orders
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<PaymentOrder>, String> {
        let rows = sqlx::query_as::<_, PaymentOrderRow>(
            r#"
            SELECT id, sender_account_id, beneficiary_name, beneficiary_rib, beneficiary_bic,
                   amount, currency, payment_type, status, screening_status,
                   reference, description, rejection_reason,
                   created_at, submitted_at, executed_at
            FROM payment.payment_orders
            WHERE sender_account_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_all(
        &self,
        status: Option<PaymentStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PaymentOrder>, String> {
        let rows = match status {
            Some(s) => {
                sqlx::query_as::<_, PaymentOrderRow>(
                    r#"
                    SELECT id, sender_account_id, beneficiary_name, beneficiary_rib, beneficiary_bic,
                           amount, currency, payment_type, status, screening_status,
                           reference, description, rejection_reason,
                           created_at, submitted_at, executed_at
                    FROM payment.payment_orders
                    WHERE status = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(s.as_str())
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?
            }
            None => {
                sqlx::query_as::<_, PaymentOrderRow>(
                    r#"
                    SELECT id, sender_account_id, beneficiary_name, beneficiary_rib, beneficiary_bic,
                           amount, currency, payment_type, status, screening_status,
                           reference, description, rejection_reason,
                           created_at, submitted_at, executed_at
                    FROM payment.payment_orders
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?
            }
        };

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn count_all(&self, status: Option<PaymentStatus>) -> Result<i64, String> {
        let count: (i64,) = match status {
            Some(s) => {
                sqlx::query_as("SELECT COUNT(*) FROM payment.payment_orders WHERE status = $1")
                    .bind(s.as_str())
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?
            }
            None => sqlx::query_as("SELECT COUNT(*) FROM payment.payment_orders")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?,
        };
        Ok(count.0)
    }
}

// --- Row mapping ---

#[derive(sqlx::FromRow)]
struct PaymentOrderRow {
    id: Uuid,
    sender_account_id: Uuid,
    beneficiary_name: String,
    beneficiary_rib: Option<String>,
    beneficiary_bic: Option<String>,
    amount: i64,
    currency: String,
    payment_type: String,
    status: String,
    screening_status: String,
    reference: String,
    description: Option<String>,
    rejection_reason: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    executed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl PaymentOrderRow {
    fn into_domain(self) -> Result<PaymentOrder, String> {
        let payment_type =
            PaymentType::from_str_type(&self.payment_type).map_err(|e| e.to_string())?;
        let status = PaymentStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;
        let screening_status =
            ScreeningStatus::from_str_type(&self.screening_status).map_err(|e| e.to_string())?;

        Ok(PaymentOrder::from_raw(
            OrderId::from_uuid(self.id),
            self.sender_account_id,
            self.beneficiary_name,
            self.beneficiary_rib,
            self.beneficiary_bic,
            self.amount,
            self.currency,
            payment_type,
            status,
            screening_status,
            self.reference,
            self.description,
            self.created_at,
            self.submitted_at,
            self.executed_at,
            self.rejection_reason,
        ))
    }
}

// ============================================================
// PostgreSQL Transfer Repository
// ============================================================

pub struct PgTransferRepository {
    pool: PgPool,
}

impl PgTransferRepository {
    pub fn new(pool: PgPool) -> Self {
        PgTransferRepository { pool }
    }
}

#[async_trait]
impl ITransferRepository for PgTransferRepository {
    async fn save(&self, transfer: &Transfer) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO payment.transfers (
                id, order_id, counterparty_rib, clearing_ref,
                amount, currency, transfer_date, status, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                clearing_ref = EXCLUDED.clearing_ref,
                status = EXCLUDED.status
            "#,
        )
        .bind(transfer.transfer_id().as_uuid())
        .bind(transfer.order_id().as_uuid())
        .bind(transfer.counterparty_rib())
        .bind(transfer.clearing_ref())
        .bind(transfer.amount())
        .bind(transfer.currency())
        .bind(transfer.transfer_date())
        .bind(transfer.status().as_str())
        .bind(transfer.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_id(&self, id: &TransferId) -> Result<Option<Transfer>, String> {
        let row = sqlx::query_as::<_, TransferRow>(
            r#"
            SELECT id, order_id, counterparty_rib, clearing_ref,
                   amount, currency, transfer_date, status, created_at
            FROM payment.transfers
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Vec<Transfer>, String> {
        let rows = sqlx::query_as::<_, TransferRow>(
            r#"
            SELECT id, order_id, counterparty_rib, clearing_ref,
                   amount, currency, transfer_date, status, created_at
            FROM payment.transfers
            WHERE order_id = $1
            "#,
        )
        .bind(order_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_submitted(&self) -> Result<Vec<Transfer>, String> {
        let rows = sqlx::query_as::<_, TransferRow>(
            r#"
            SELECT id, order_id, counterparty_rib, clearing_ref,
                   amount, currency, transfer_date, status, created_at
            FROM payment.transfers
            WHERE status = 'Submitted'
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}

#[derive(sqlx::FromRow)]
struct TransferRow {
    id: Uuid,
    order_id: Uuid,
    counterparty_rib: String,
    clearing_ref: Option<String>,
    amount: i64,
    currency: String,
    transfer_date: chrono::NaiveDate,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl TransferRow {
    fn into_domain(self) -> Result<Transfer, String> {
        let status = PaymentStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;
        Ok(Transfer::from_raw(
            TransferId::from_uuid(self.id),
            OrderId::from_uuid(self.order_id),
            self.counterparty_rib,
            self.clearing_ref,
            self.amount,
            self.currency,
            self.transfer_date,
            status,
            self.created_at,
        ))
    }
}

// ============================================================
// PostgreSQL SWIFT Message Repository
// ============================================================

pub struct PgSwiftMessageRepository {
    pool: PgPool,
}

impl PgSwiftMessageRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSwiftMessageRepository { pool }
    }
}

#[async_trait]
impl ISwiftMessageRepository for PgSwiftMessageRepository {
    async fn save(&self, message: &SwiftMessage) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO payment.swift_messages (
                id, order_id, message_type, sender_bic, receiver_bic,
                amount, currency, reference, status, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status
            "#,
        )
        .bind(message.message_id())
        .bind(message.order_id().as_uuid())
        .bind(message.message_type())
        .bind(message.sender_bic())
        .bind(message.receiver_bic())
        .bind(message.amount())
        .bind(message.currency())
        .bind(message.reference())
        .bind(message.status().as_str())
        .bind(message.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<SwiftMessage>, String> {
        let row = sqlx::query_as::<_, SwiftMessageRow>(
            r#"
            SELECT id, order_id, message_type, sender_bic, receiver_bic,
                   amount, currency, reference, status, created_at
            FROM payment.swift_messages
            WHERE order_id = $1
            "#,
        )
        .bind(order_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }
}

#[derive(sqlx::FromRow)]
struct SwiftMessageRow {
    id: Uuid,
    order_id: Uuid,
    message_type: String,
    sender_bic: String,
    receiver_bic: String,
    amount: i64,
    currency: String,
    reference: String,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl SwiftMessageRow {
    fn into_domain(self) -> Result<SwiftMessage, String> {
        let status = SwiftMessageStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;
        Ok(SwiftMessage::from_raw(
            self.id,
            OrderId::from_uuid(self.order_id),
            self.message_type,
            self.sender_bic,
            self.receiver_bic,
            self.amount,
            self.currency,
            self.reference,
            status,
            self.created_at,
        ))
    }
}

// ============================================================
// PostgreSQL Card Repository (STORY-CARD-01 through CARD-06)
// ============================================================

use banko_application::payment::{ICardRepository, ICardTransactionRepository};
use chrono::NaiveDate;
use rust_decimal::Decimal;

pub struct PgCardRepository {
    pool: PgPool,
}

impl PgCardRepository {
    pub fn new(pool: PgPool) -> Self {
        PgCardRepository { pool }
    }
}

#[async_trait]
impl ICardRepository for PgCardRepository {
    async fn save(&self, card: &Card) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO cards (
                id, account_id, customer_id, card_type, network,
                pan_hash, masked_pan, cvv_hash, expiry_month, expiry_year,
                status, activation_code_hash, daily_limit, monthly_limit,
                daily_spent, monthly_spent, is_contactless_enabled,
                created_at, activated_at, cancelled_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                activation_code_hash = EXCLUDED.activation_code_hash,
                daily_limit = EXCLUDED.daily_limit,
                monthly_limit = EXCLUDED.monthly_limit,
                daily_spent = EXCLUDED.daily_spent,
                monthly_spent = EXCLUDED.monthly_spent,
                is_contactless_enabled = EXCLUDED.is_contactless_enabled,
                activated_at = EXCLUDED.activated_at,
                cancelled_at = EXCLUDED.cancelled_at
            "#,
        )
        .bind(card.id())
        .bind(card.account_id())
        .bind(card.customer_id())
        .bind(card.card_type().as_str())
        .bind(card.network().as_str())
        .bind(card.pan_hash())
        .bind(card.masked_pan())
        .bind("") // cvv_hash (placeholder)
        .bind(card.expiry_month() as i16)
        .bind(card.expiry_year() as i16)
        .bind(card.status().as_str())
        .bind::<Option<String>>(None) // activation_code_hash
        .bind(card.daily_limit())
        .bind(card.monthly_limit())
        .bind(card.daily_spent())
        .bind(card.monthly_spent())
        .bind(card.is_contactless_enabled())
        .bind(card.created_at())
        .bind(card.activated_at())
        .bind(card.cancelled_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Card>, String> {
        let row = sqlx::query_as::<_, CardRow>(
            r#"
            SELECT id, account_id, customer_id, card_type, network,
                   pan_hash, masked_pan, cvv_hash, expiry_month, expiry_year,
                   status, activation_code_hash, daily_limit, monthly_limit,
                   daily_spent, monthly_spent, is_contactless_enabled,
                   created_at, activated_at, cancelled_at
            FROM cards
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<Card>, String> {
        let rows = sqlx::query_as::<_, CardRow>(
            r#"
            SELECT id, account_id, customer_id, card_type, network,
                   pan_hash, masked_pan, cvv_hash, expiry_month, expiry_year,
                   status, activation_code_hash, daily_limit, monthly_limit,
                   daily_spent, monthly_spent, is_contactless_enabled,
                   created_at, activated_at, cancelled_at
            FROM cards
            WHERE account_id = $1
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<Card>, String> {
        let rows = sqlx::query_as::<_, CardRow>(
            r#"
            SELECT id, account_id, customer_id, card_type, network,
                   pan_hash, masked_pan, cvv_hash, expiry_month, expiry_year,
                   status, activation_code_hash, daily_limit, monthly_limit,
                   daily_spent, monthly_spent, is_contactless_enabled,
                   created_at, activated_at, cancelled_at
            FROM cards
            WHERE customer_id = $1
            "#,
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn update(&self, card: &Card) -> Result<(), String> {
        self.save(card).await
    }

    async fn list_active(&self) -> Result<Vec<Card>, String> {
        let rows = sqlx::query_as::<_, CardRow>(
            r#"
            SELECT id, account_id, customer_id, card_type, network,
                   pan_hash, masked_pan, cvv_hash, expiry_month, expiry_year,
                   status, activation_code_hash, daily_limit, monthly_limit,
                   daily_spent, monthly_spent, is_contactless_enabled,
                   created_at, activated_at, cancelled_at
            FROM cards
            WHERE status = 'Active'
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}

#[derive(sqlx::FromRow)]
struct CardRow {
    id: Uuid,
    account_id: Uuid,
    customer_id: Uuid,
    card_type: String,
    network: String,
    pan_hash: String,
    masked_pan: String,
    cvv_hash: String,
    expiry_month: i16,
    expiry_year: i16,
    status: String,
    activation_code_hash: Option<String>,
    daily_limit: Decimal,
    monthly_limit: Decimal,
    daily_spent: Decimal,
    monthly_spent: Decimal,
    is_contactless_enabled: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    activated_at: Option<chrono::DateTime<chrono::Utc>>,
    cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl CardRow {
    fn into_domain(self) -> Result<Card, String> {
        let card_type = CardType::from_str_type(&self.card_type).map_err(|e| e.to_string())?;
        let network = CardNetwork::from_str_type(&self.network).map_err(|e| e.to_string())?;
        let status = CardStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;

        Ok(Card::from_raw(
            self.id,
            self.account_id,
            self.customer_id,
            card_type,
            network,
            self.pan_hash,
            self.masked_pan,
            self.cvv_hash,
            self.expiry_month as u8,
            self.expiry_year as u16,
            status,
            self.activation_code_hash,
            self.daily_limit,
            self.monthly_limit,
            self.daily_spent,
            self.monthly_spent,
            self.is_contactless_enabled,
            self.created_at,
            self.activated_at,
            self.cancelled_at,
        ))
    }
}

// ============================================================
// PostgreSQL Card Transaction Repository
// ============================================================

pub struct PgCardTransactionRepository {
    pool: PgPool,
}

impl PgCardTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        PgCardTransactionRepository { pool }
    }
}

#[async_trait]
impl ICardTransactionRepository for PgCardTransactionRepository {
    async fn save(&self, transaction: &CardTransaction) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO card_transactions (
                id, card_id, amount, currency, merchant_name, mcc_code,
                status, auth_code, timestamp, is_contactless, is_online
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(transaction.id())
        .bind(transaction.card_id())
        .bind(transaction.amount())
        .bind(transaction.currency())
        .bind(transaction.merchant_name())
        .bind(transaction.mcc_code())
        .bind(transaction.status().as_str())
        .bind(transaction.auth_code())
        .bind(transaction.timestamp())
        .bind(transaction.is_contactless())
        .bind(transaction.is_online())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CardTransaction>, String> {
        let row = sqlx::query_as::<_, CardTransactionRow>(
            r#"
            SELECT id, card_id, amount, currency, merchant_name, mcc_code,
                   status, auth_code, timestamp, is_contactless, is_online
            FROM card_transactions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_card(&self, card_id: Uuid) -> Result<Vec<CardTransaction>, String> {
        let rows = sqlx::query_as::<_, CardTransactionRow>(
            r#"
            SELECT id, card_id, amount, currency, merchant_name, mcc_code,
                   status, auth_code, timestamp, is_contactless, is_online
            FROM card_transactions
            WHERE card_id = $1
            ORDER BY timestamp DESC
            "#,
        )
        .bind(card_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_card_and_period(
        &self,
        card_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<CardTransaction>, String> {
        let rows = sqlx::query_as::<_, CardTransactionRow>(
            r#"
            SELECT id, card_id, amount, currency, merchant_name, mcc_code,
                   status, auth_code, timestamp, is_contactless, is_online
            FROM card_transactions
            WHERE card_id = $1
            AND DATE(timestamp) >= $2
            AND DATE(timestamp) <= $3
            ORDER BY timestamp DESC
            "#,
        )
        .bind(card_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}

#[derive(sqlx::FromRow)]
struct CardTransactionRow {
    id: Uuid,
    card_id: Uuid,
    amount: Decimal,
    currency: String,
    merchant_name: String,
    mcc_code: String,
    status: String,
    auth_code: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    is_contactless: bool,
    is_online: bool,
}

impl CardTransactionRow {
    fn into_domain(self) -> Result<CardTransaction, String> {
        let status =
            TransactionStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;

        Ok(CardTransaction::from_raw(
            self.id,
            self.card_id,
            self.amount,
            self.currency,
            self.merchant_name,
            self.mcc_code,
            status,
            self.auth_code,
            self.timestamp,
            self.is_contactless,
            self.is_online,
        ))
    }
}
