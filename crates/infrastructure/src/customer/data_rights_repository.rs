use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::customer::IDataRightsRepository;
use banko_domain::customer::{
    DataRequestId, DataRequestStatus, DataRequestType, DataRightsRequest,
};

pub struct PgDataRightsRepository {
    pool: PgPool,
}

impl PgDataRightsRepository {
    pub fn new(pool: PgPool) -> Self {
        PgDataRightsRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DataRightsRequestRow {
    id: Uuid,
    customer_id: Uuid,
    request_type: String,
    status: String,
    details: Option<String>,
    response: Option<String>,
    requested_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    deadline: DateTime<Utc>,
}

fn row_to_domain(row: DataRightsRequestRow) -> Result<DataRightsRequest, String> {
    let request_id = DataRequestId::from_uuid(row.id);
    let request_type =
        DataRequestType::from_str_type(&row.request_type).map_err(|e| e.to_string())?;
    let status = DataRequestStatus::from_str_status(&row.status).map_err(|e| e.to_string())?;

    Ok(DataRightsRequest::reconstitute(
        request_id,
        row.customer_id,
        request_type,
        status,
        row.details,
        row.response,
        row.requested_at,
        row.completed_at,
        row.deadline,
    ))
}

#[async_trait]
impl IDataRightsRepository for PgDataRightsRepository {
    async fn save(&self, request: &DataRightsRequest) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO customer.data_rights_requests (id, customer_id, request_type, status, details, response, requested_at, completed_at, deadline)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                details = EXCLUDED.details,
                response = EXCLUDED.response,
                completed_at = EXCLUDED.completed_at
            "#,
        )
        .bind(request.request_id().as_uuid())
        .bind(request.customer_id())
        .bind(request.request_type().as_str())
        .bind(request.status().as_str())
        .bind(request.details())
        .bind(request.response())
        .bind(request.requested_at())
        .bind(request.completed_at())
        .bind(request.deadline())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save data rights request error: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DataRequestId) -> Result<Option<DataRightsRequest>, String> {
        let row: Option<DataRightsRequestRow> = sqlx::query_as(
            "SELECT id, customer_id, request_type, status, details, response, requested_at, completed_at, deadline FROM customer.data_rights_requests WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find data rights request error: {e}"))?;

        row.map(row_to_domain).transpose()
    }

    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<DataRightsRequest>, String> {
        let rows: Vec<DataRightsRequestRow> = sqlx::query_as(
            "SELECT id, customer_id, request_type, status, details, response, requested_at, completed_at, deadline FROM customer.data_rights_requests WHERE customer_id = $1 ORDER BY requested_at DESC",
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find data rights requests error: {e}"))?;

        rows.into_iter().map(row_to_domain).collect()
    }
}
