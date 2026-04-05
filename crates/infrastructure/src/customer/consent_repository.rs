use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::customer::IConsentRepository;
use banko_domain::customer::{
    ConsentId, ConsentPurpose, ConsentRecord, ConsentRecordStatus,
};

pub struct PgConsentRepository {
    pool: PgPool,
}

impl PgConsentRepository {
    pub fn new(pool: PgPool) -> Self {
        PgConsentRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ConsentRow {
    id: Uuid,
    customer_id: Uuid,
    purpose: String,
    status: String,
    granted_at: DateTime<Utc>,
    revoked_at: Option<DateTime<Utc>>,
}

fn row_to_domain(row: ConsentRow) -> Result<ConsentRecord, String> {
    let consent_id = ConsentId::from_uuid(row.id);
    let purpose =
        ConsentPurpose::from_str_purpose(&row.purpose).map_err(|e| e.to_string())?;
    let status =
        ConsentRecordStatus::from_str_status(&row.status).map_err(|e| e.to_string())?;

    Ok(ConsentRecord::reconstitute(
        consent_id,
        row.customer_id,
        purpose,
        status,
        row.granted_at,
        row.revoked_at,
    ))
}

#[async_trait]
impl IConsentRepository for PgConsentRepository {
    async fn save(&self, consent: &ConsentRecord) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO customer.consents (id, customer_id, purpose, status, granted_at, revoked_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                revoked_at = EXCLUDED.revoked_at
            "#,
        )
        .bind(consent.consent_id().as_uuid())
        .bind(consent.customer_id())
        .bind(consent.purpose().as_str())
        .bind(consent.status().as_str())
        .bind(consent.granted_at())
        .bind(consent.revoked_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save consent error: {e}"))?;

        Ok(())
    }

    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<ConsentRecord>, String> {
        let rows: Vec<ConsentRow> = sqlx::query_as(
            "SELECT id, customer_id, purpose, status, granted_at, revoked_at FROM customer.consents WHERE customer_id = $1 ORDER BY granted_at DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find consents error: {e}"))?;

        rows.into_iter().map(row_to_domain).collect()
    }

    async fn find_by_customer_and_purpose(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<Option<ConsentRecord>, String> {
        let row: Option<ConsentRow> = sqlx::query_as(
            "SELECT id, customer_id, purpose, status, granted_at, revoked_at FROM customer.consents WHERE customer_id = $1 AND purpose = $2 AND status = 'Active' ORDER BY granted_at DESC LIMIT 1",
        )
        .bind(customer_id)
        .bind(purpose)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find consent by purpose error: {e}"))?;

        row.map(row_to_domain).transpose()
    }

    async fn find_active_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<ConsentRecord>, String> {
        let rows: Vec<ConsentRow> = sqlx::query_as(
            "SELECT id, customer_id, purpose, status, granted_at, revoked_at FROM customer.consents WHERE customer_id = $1 AND status = 'Active' ORDER BY granted_at DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find active consents error: {e}"))?;

        rows.into_iter().map(row_to_domain).collect()
    }
}
