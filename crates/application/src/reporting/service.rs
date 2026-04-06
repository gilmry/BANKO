use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use banko_domain::reporting::*;

use super::dto::*;
use super::errors::ReportingServiceError;
use super::ports::*;

// ============================================================
// ReportingService (REP-01 to REP-06)
// ============================================================

pub struct ReportingService {
    report_repo: Arc<dyn IReportRepository>,
    template_repo: Arc<dyn IReportTemplateRepository>,
}

impl ReportingService {
    pub fn new(
        report_repo: Arc<dyn IReportRepository>,
        template_repo: Arc<dyn IReportTemplateRepository>,
    ) -> Self {
        ReportingService {
            report_repo,
            template_repo,
        }
    }

    /// Generate a new regulatory report.
    pub async fn generate_report(
        &self,
        report_type: ReportType,
        period_start: NaiveDate,
        period_end: NaiveDate,
        generated_by: Uuid,
    ) -> Result<ReportResponse, ReportingServiceError> {
        // Find active template for this report type
        let template = self
            .template_repo
            .find_active_by_type(&report_type)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::NoActiveTemplate)?;

        // Generate stub report data (in production, aggregate from other BCs)
        let data = format!(
            r#"{{"report_type":"{}","period_start":"{}","period_end":"{}","generated":"stub"}}"#,
            report_type.as_str(),
            period_start,
            period_end
        );

        let report = RegulatoryReport::new(
            report_type,
            period_start,
            period_end,
            template.template_id().clone(),
            template.version().to_string(),
            data,
            generated_by,
        )
        .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.report_repo
            .save(&report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(to_report_response(&report))
    }

    /// Validate a report (Generated -> Validated).
    pub async fn validate_report(
        &self,
        report_id: &ReportId,
    ) -> Result<ReportResponse, ReportingServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::ReportNotFound)?;

        report
            .validate()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.report_repo
            .save(&report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(to_report_response(&report))
    }

    /// Submit a report to BCT (Validated -> Submitted). Stub for actual BCT integration.
    pub async fn submit_report(
        &self,
        report_id: &ReportId,
    ) -> Result<ReportResponse, ReportingServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::ReportNotFound)?;

        report
            .submit()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.report_repo
            .save(&report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        // TODO: actual BCT submission via API/SFTP

        Ok(to_report_response(&report))
    }

    /// Acknowledge a submitted report (Submitted -> Acknowledged).
    pub async fn acknowledge_report(
        &self,
        report_id: &ReportId,
    ) -> Result<ReportResponse, ReportingServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::ReportNotFound)?;

        report
            .acknowledge()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.report_repo
            .save(&report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(to_report_response(&report))
    }

    /// Get a single report by ID.
    pub async fn get_report(
        &self,
        report_id: &ReportId,
    ) -> Result<ReportResponse, ReportingServiceError> {
        let report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::ReportNotFound)?;

        Ok(to_report_response(&report))
    }

    /// List reports with optional filters and pagination.
    pub async fn list_reports(
        &self,
        report_type: Option<ReportType>,
        status: Option<ReportStatus>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        page: i64,
        limit: i64,
    ) -> Result<ReportListResponse, ReportingServiceError> {
        let offset = (page - 1).max(0) * limit;

        let reports = self
            .report_repo
            .find_all(report_type, status, from, to, limit, offset)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let total = self
            .report_repo
            .count_all(report_type, status, from, to)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(ReportListResponse {
            data: reports.iter().map(to_report_response).collect(),
            total,
            page,
            limit,
        })
    }

    /// Convenience: generate weekly report.
    pub async fn generate_weekly_report(
        &self,
        week_start: NaiveDate,
        generated_by: Uuid,
    ) -> Result<ReportResponse, ReportingServiceError> {
        let week_end = week_start + chrono::Duration::days(6);
        self.generate_report(ReportType::Weekly, week_start, week_end, generated_by)
            .await
    }

    /// Convenience: generate monthly report.
    pub async fn generate_monthly_report(
        &self,
        year: i32,
        month: u32,
        generated_by: Uuid,
    ) -> Result<ReportResponse, ReportingServiceError> {
        let start = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| ReportingServiceError::InvalidInput("Invalid month".to_string()))?;
        let end = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - chrono::Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - chrono::Duration::days(1)
        };
        self.generate_report(ReportType::Monthly, start, end, generated_by)
            .await
    }

    /// List all templates.
    pub async fn list_templates(&self) -> Result<TemplateListResponse, ReportingServiceError> {
        let templates = self
            .template_repo
            .find_all()
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(TemplateListResponse {
            data: templates.iter().map(to_template_response).collect(),
        })
    }
}

// ============================================================
// Ifrs9ReportService (REP-07/REP-08 — GOV-08 prep)
// ============================================================

pub struct Ifrs9ReportService {
    ecl_provider: Arc<dyn IEclDataProvider>,
}

impl Ifrs9ReportService {
    pub fn new(ecl_provider: Arc<dyn IEclDataProvider>) -> Self {
        Ifrs9ReportService { ecl_provider }
    }

    /// Generate IFRS 9 ECL staging summary with real data aggregation (REP-08)
    pub async fn generate_ifrs9_report(
        &self,
        as_of: NaiveDate,
    ) -> Result<Ifrs9Response, ReportingServiceError> {
        // Get ECL data from accounting/credit BC grouped by stage
        let ecl_data = self
            .ecl_provider
            .get_ecl_by_stage(as_of)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let mut stage1_count = 0i64;
        let mut stage1_ecl = 0.0f64;
        let mut stage2_count = 0i64;
        let mut stage2_ecl = 0.0f64;
        let mut stage3_count = 0i64;
        let mut stage3_ecl = 0.0f64;

        // Aggregate by stage
        for point in ecl_data {
            match point.stage {
                1 => {
                    stage1_count += point.count;
                    stage1_ecl += point.ecl_amount;
                }
                2 => {
                    stage2_count += point.count;
                    stage2_ecl += point.ecl_amount;
                }
                3 => {
                    stage3_count += point.count;
                    stage3_ecl += point.ecl_amount;
                }
                _ => {} // Ignore invalid stages
            }
        }

        let total_ecl = stage1_ecl + stage2_ecl + stage3_ecl;

        Ok(Ifrs9Response {
            as_of,
            stage1_count,
            stage1_ecl,
            stage2_count,
            stage2_ecl,
            stage3_count,
            stage3_ecl,
            total_ecl,
        })
    }
}

impl Default for Ifrs9ReportService {
    fn default() -> Self {
        // Default constructor requires an ECL provider — should be injected via DI
        // This is a fallback for tests only
        panic!("Ifrs9ReportService requires an IEclDataProvider")
    }
}

// --- Helpers ---

fn to_report_response(r: &RegulatoryReport) -> ReportResponse {
    ReportResponse {
        id: r.report_id().to_string(),
        report_type: r.report_type().as_str().to_string(),
        period_start: r.period_start(),
        period_end: r.period_end(),
        template_id: r.template_id().to_string(),
        template_version: r.template_version().to_string(),
        data: r.data().to_string(),
        status: r.status().as_str().to_string(),
        generated_at: r.generated_at(),
        submitted_at: r.submitted_at(),
        acknowledged_at: r.acknowledged_at(),
        rejection_reason: r.rejection_reason().map(|s| s.to_string()),
        generated_by: r.generated_by().to_string(),
    }
}

fn to_template_response(t: &ReportTemplate) -> TemplateResponse {
    TemplateResponse {
        id: t.template_id().to_string(),
        name: t.name().to_string(),
        report_type: t.report_type().as_str().to_string(),
        version: t.version().to_string(),
        definition: t.definition().to_string(),
        is_active: t.is_active(),
    }
}
