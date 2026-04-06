use async_trait::async_trait;
use chrono::NaiveDate;

use banko_domain::reporting::{
    RegulatoryReport, ReportId, ReportStatus, ReportTemplate, ReportType, TemplateId,
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
