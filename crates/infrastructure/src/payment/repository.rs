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
