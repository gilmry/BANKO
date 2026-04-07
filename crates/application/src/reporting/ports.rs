use async_trait::async_trait;
use chrono::NaiveDate;

use banko_domain::reporting::{
    AdHocReport, AdHocReportId, Ifrs9Report, Ifrs9ReportId, ReportArchive, ReportArchiveId,
    ReportDistribution, ReportDistributionId, RegulatoryReport, ReportId, ReportStatus,
    ReportTemplate, ReportType, ScheduledReport, ScheduledReportId, TaxReport, TaxReportId,
    TaxReportType, TemplateId,
};

// --- Report Repository ---

#[async_trait]
pub trait IReportRepository: Send + Sync {
    async fn save(&self, report: &RegulatoryReport) -> Result<(), String>;
    async fn find_by_id(&self, id: &ReportId) -> Result<Option<RegulatoryReport>, String>;
    async fn find_by_type_and_period(
        &self,
        report_type: ReportType,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<RegulatoryReport>, String>;
    async fn find_all(
        &self,
        report_type: Option<ReportType>,
        status: Option<ReportStatus>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RegulatoryReport>, String>;
    async fn count_all(
        &self,
        report_type: Option<ReportType>,
        status: Option<ReportStatus>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<i64, String>;
}

// --- Report Template Repository ---

#[async_trait]
pub trait IReportTemplateRepository: Send + Sync {
    async fn save(&self, template: &ReportTemplate) -> Result<(), String>;
    async fn find_by_id(&self, id: &TemplateId) -> Result<Option<ReportTemplate>, String>;
    async fn find_active_by_type(
        &self,
        report_type: &ReportType,
    ) -> Result<Option<ReportTemplate>, String>;
    async fn find_all(&self) -> Result<Vec<ReportTemplate>, String>;
}

// --- ECL Data Provider Port (REP-08) ---
/// Provides ECL (Expected Credit Loss) data aggregated by IFRS 9 stage
pub struct EclDataPoint {
    pub stage: i32,                // 1, 2, or 3
    pub count: i64,                // Number of loans in this stage
    pub ecl_amount: f64,           // Expected credit loss in currency units
}

#[async_trait]
pub trait IEclDataProvider: Send + Sync {
    /// Get ECL data aggregated by stage as of a specific date
    async fn get_ecl_by_stage(
        &self,
        as_of: NaiveDate,
    ) -> Result<Vec<EclDataPoint>, String>;
}

// ============================================================
// Advanced Repositories (BMAD v4.0.1 Compliance)
// ============================================================

// --- Scheduled Report Repository ---

#[async_trait]
pub trait IScheduledReportRepository: Send + Sync {
    async fn save(&self, report: &ScheduledReport) -> Result<(), String>;
    async fn find_by_id(&self, id: &ScheduledReportId) -> Result<Option<ScheduledReport>, String>;
    async fn find_active(&self) -> Result<Vec<ScheduledReport>, String>;
    async fn find_due_for_execution(&self) -> Result<Vec<ScheduledReport>, String>;
}

// --- Report Distribution Repository ---

#[async_trait]
pub trait IReportDistributionRepository: Send + Sync {
    async fn save(&self, distribution: &ReportDistribution) -> Result<(), String>;
    async fn find_by_id(
        &self,
        id: &ReportDistributionId,
    ) -> Result<Option<ReportDistribution>, String>;
    async fn find_by_report_id(
        &self,
        report_id: &str,
    ) -> Result<Vec<ReportDistribution>, String>;
}

// --- Report Archive Repository ---

#[async_trait]
pub trait IReportArchiveRepository: Send + Sync {
    async fn save(&self, archive: &ReportArchive) -> Result<(), String>;
    async fn find_by_id(&self, id: &ReportArchiveId) -> Result<Option<ReportArchive>, String>;
    async fn find_by_report_id(&self, report_id: &str) -> Result<Vec<ReportArchive>, String>;
    async fn find_expired(&self) -> Result<Vec<ReportArchive>, String>;
    async fn find_all(&self) -> Result<Vec<ReportArchive>, String>;
}

// --- Ad-Hoc Report Repository ---

#[async_trait]
pub trait IAdHocReportRepository: Send + Sync {
    async fn save(&self, report: &AdHocReport) -> Result<(), String>;
    async fn find_by_id(&self, id: &AdHocReportId) -> Result<Option<AdHocReport>, String>;
    async fn find_all(&self) -> Result<Vec<AdHocReport>, String>;
}

// --- Tax Report Repository ---

#[async_trait]
pub trait ITaxReportRepository: Send + Sync {
    async fn save(&self, report: &TaxReport) -> Result<(), String>;
    async fn find_by_id(&self, id: &TaxReportId) -> Result<Option<TaxReport>, String>;
    async fn find_by_criteria(
        &self,
        tax_type: Option<TaxReportType>,
        period_start: Option<NaiveDate>,
        period_end: Option<NaiveDate>,
    ) -> Result<Vec<TaxReport>, String>;
}

// --- IFRS 9 Report Repository ---

#[async_trait]
pub trait IIfrs9ReportRepository: Send + Sync {
    async fn save(&self, report: &Ifrs9Report) -> Result<(), String>;
    async fn find_by_id(&self, id: &Ifrs9ReportId) -> Result<Option<Ifrs9Report>, String>;
    async fn find_latest_by_date(&self, as_of: NaiveDate) -> Result<Option<Ifrs9Report>, String>;
    async fn find_by_period(
        &self,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<Ifrs9Report>, String>;
}
