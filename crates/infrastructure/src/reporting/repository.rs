use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::reporting::{IReportRepository, IReportTemplateRepository};
use banko_domain::reporting::*;

// ============================================================
// PgReportRepository
// ============================================================

pub struct PgReportRepository {
    pool: PgPool,
}

impl PgReportRepository {
    pub fn new(pool: PgPool) -> Self {
        PgReportRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ReportRow {
    id: Uuid,
    report_type: String,
    period_start: NaiveDate,
    period_end: NaiveDate,
    template_id: Uuid,
    template_version: String,
    data: serde_json::Value,
    status: String,
    generated_by: Uuid,
    generated_at: chrono::DateTime<chrono::Utc>,
    submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    acknowledged_at: Option<chrono::DateTime<chrono::Utc>>,
    rejection_reason: Option<String>,
}

impl ReportRow {
    fn into_domain(self) -> Result<RegulatoryReport, String> {
        let report_type =
            ReportType::from_str_type(&self.report_type).map_err(|e| e.to_string())?;
        let status = ReportStatus::from_str_type(&self.status).map_err(|e| e.to_string())?;

        Ok(RegulatoryReport::from_raw(
            ReportId::from_uuid(self.id),
            report_type,
            self.period_start,
            self.period_end,
            TemplateId::from_uuid(self.template_id),
            self.template_version,
            self.data.to_string(),
            status,
            self.generated_at,
            self.submitted_at,
            self.acknowledged_at,
            self.rejection_reason,
            self.generated_by,
        ))
    }
}

#[async_trait]
impl IReportRepository for PgReportRepository {
    async fn save(&self, report: &RegulatoryReport) -> Result<(), String> {
        let data: serde_json::Value = serde_json::from_str(report.data())
            .unwrap_or(serde_json::Value::String(report.data().to_string()));

        sqlx::query(
            r#"
            INSERT INTO reporting.regulatory_reports
                (id, report_type, period_start, period_end, template_id, template_version, data, status, generated_by, generated_at, submitted_at, acknowledged_at, rejection_reason)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                submitted_at = EXCLUDED.submitted_at,
                acknowledged_at = EXCLUDED.acknowledged_at,
                rejection_reason = EXCLUDED.rejection_reason
            "#,
        )
        .bind(report.report_id().as_uuid())
        .bind(report.report_type().as_str())
        .bind(report.period_start())
        .bind(report.period_end())
        .bind(report.template_id().as_uuid())
        .bind(report.template_version())
        .bind(data)
        .bind(report.status().as_str())
        .bind(report.generated_by())
        .bind(report.generated_at())
        .bind(report.submitted_at())
        .bind(report.acknowledged_at())
        .bind(report.rejection_reason())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ReportId) -> Result<Option<RegulatoryReport>, String> {
        let row = sqlx::query_as::<_, ReportRow>(
            "SELECT id, report_type, period_start, period_end, template_id, template_version, data, status, generated_by, generated_at, submitted_at, acknowledged_at, rejection_reason FROM reporting.regulatory_reports WHERE id = $1",
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

    async fn find_by_type_and_period(
        &self,
        report_type: ReportType,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<RegulatoryReport>, String> {
        let rows = sqlx::query_as::<_, ReportRow>(
            "SELECT id, report_type, period_start, period_end, template_id, template_version, data, status, generated_by, generated_at, submitted_at, acknowledged_at, rejection_reason FROM reporting.regulatory_reports WHERE report_type = $1 AND period_start >= $2 AND period_end <= $3 ORDER BY period_start DESC",
        )
        .bind(report_type.as_str())
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_all(
        &self,
        report_type: Option<ReportType>,
        status: Option<ReportStatus>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RegulatoryReport>, String> {
        let rows = sqlx::query_as::<_, ReportRow>(
            r#"
            SELECT id, report_type, period_start, period_end, template_id, template_version, data, status, generated_by, generated_at, submitted_at, acknowledged_at, rejection_reason
            FROM reporting.regulatory_reports
            WHERE ($1::text IS NULL OR report_type = $1)
              AND ($2::text IS NULL OR status = $2)
              AND ($3::date IS NULL OR period_start >= $3)
              AND ($4::date IS NULL OR period_end <= $4)
            ORDER BY generated_at DESC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(report_type.map(|rt| rt.as_str().to_string()))
        .bind(status.map(|s| s.as_str().to_string()))
        .bind(from)
        .bind(to)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn count_all(
        &self,
        report_type: Option<ReportType>,
        status: Option<ReportStatus>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM reporting.regulatory_reports
            WHERE ($1::text IS NULL OR report_type = $1)
              AND ($2::text IS NULL OR status = $2)
              AND ($3::date IS NULL OR period_start >= $3)
              AND ($4::date IS NULL OR period_end <= $4)
            "#,
        )
        .bind(report_type.map(|rt| rt.as_str().to_string()))
        .bind(status.map(|s| s.as_str().to_string()))
        .bind(from)
        .bind(to)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.0)
    }
}

// ============================================================
// PgReportTemplateRepository
// ============================================================

pub struct PgReportTemplateRepository {
    pool: PgPool,
}

impl PgReportTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        PgReportTemplateRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct TemplateRow {
    id: Uuid,
    name: String,
    report_type: String,
    version: String,
    definition: serde_json::Value,
    is_active: bool,
}

impl TemplateRow {
    fn into_domain(self) -> Result<ReportTemplate, String> {
        let report_type =
            ReportType::from_str_type(&self.report_type).map_err(|e| e.to_string())?;

        Ok(ReportTemplate::from_raw(
            TemplateId::from_uuid(self.id),
            self.name,
            report_type,
            self.version,
            self.definition.to_string(),
            self.is_active,
        ))
    }
}

#[async_trait]
impl IReportTemplateRepository for PgReportTemplateRepository {
    async fn save(&self, template: &ReportTemplate) -> Result<(), String> {
        let definition: serde_json::Value = serde_json::from_str(template.definition())
            .unwrap_or(serde_json::Value::String(template.definition().to_string()));

        sqlx::query(
            r#"
            INSERT INTO reporting.report_templates (id, name, report_type, version, definition, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                version = EXCLUDED.version,
                definition = EXCLUDED.definition,
                is_active = EXCLUDED.is_active
            "#,
        )
        .bind(template.template_id().as_uuid())
        .bind(template.name())
        .bind(template.report_type().as_str())
        .bind(template.version())
        .bind(definition)
        .bind(template.is_active())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &TemplateId) -> Result<Option<ReportTemplate>, String> {
        let row = sqlx::query_as::<_, TemplateRow>(
            "SELECT id, name, report_type, version, definition, is_active FROM reporting.report_templates WHERE id = $1",
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

    async fn find_active_by_type(
        &self,
        report_type: &ReportType,
    ) -> Result<Option<ReportTemplate>, String> {
        let row = sqlx::query_as::<_, TemplateRow>(
            "SELECT id, name, report_type, version, definition, is_active FROM reporting.report_templates WHERE report_type = $1 AND is_active = true ORDER BY version DESC LIMIT 1",
        )
        .bind(report_type.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<ReportTemplate>, String> {
        let rows = sqlx::query_as::<_, TemplateRow>(
            "SELECT id, name, report_type, version, definition, is_active FROM reporting.report_templates ORDER BY report_type, version DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}
