use std::sync::Arc;

use banko_domain::reporting::*;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use super::errors::ReportingServiceError;
use super::ports::*;

// ============================================================
// ScheduledReportService (BMAD: Scheduled report generation)
// ============================================================

pub struct ScheduledReportService {
    scheduled_repo: Arc<dyn IScheduledReportRepository>,
}

impl ScheduledReportService {
    pub fn new(scheduled_repo: Arc<dyn IScheduledReportRepository>) -> Self {
        ScheduledReportService { scheduled_repo }
    }

    /// Create a new scheduled report
    pub async fn create_scheduled_report(
        &self,
        name: String,
        description: Option<String>,
        report_type: String,
        frequency: ScheduleFrequency,
        cron_expression: Option<String>,
        next_run: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<ScheduledReport, ReportingServiceError> {
        let scheduled_report = ScheduledReport::new(
            name,
            description,
            report_type,
            frequency,
            cron_expression,
            next_run,
            created_by,
        )
        .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.scheduled_repo
            .save(&scheduled_report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(scheduled_report)
    }

    /// Get a scheduled report by ID
    pub async fn get_scheduled_report(
        &self,
        id: &ScheduledReportId,
    ) -> Result<ScheduledReport, ReportingServiceError> {
        self.scheduled_repo
            .find_by_id(id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::ReportNotFound)
    }

    /// List active scheduled reports
    pub async fn list_active_scheduled_reports(
        &self,
    ) -> Result<Vec<ScheduledReport>, ReportingServiceError> {
        self.scheduled_repo
            .find_active()
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// List scheduled reports due for execution
    pub async fn list_due_scheduled_reports(
        &self,
    ) -> Result<Vec<ScheduledReport>, ReportingServiceError> {
        self.scheduled_repo
            .find_due_for_execution()
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// Mark a scheduled report as executed
    pub async fn mark_executed(
        &self,
        id: &ScheduledReportId,
    ) -> Result<ScheduledReport, ReportingServiceError> {
        let mut report = self.get_scheduled_report(id).await?;

        report
            .mark_executed()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.scheduled_repo
            .save(&report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(report)
    }

    /// Deactivate a scheduled report
    pub async fn deactivate(
        &self,
        id: &ScheduledReportId,
    ) -> Result<ScheduledReport, ReportingServiceError> {
        let mut report = self.get_scheduled_report(id).await?;

        report
            .deactivate()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.scheduled_repo
            .save(&report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(report)
    }
}

// ============================================================
// ReportDistributionService (BMAD: Report distribution)
// ============================================================

pub struct ReportDistributionService {
    distribution_repo: Arc<dyn IReportDistributionRepository>,
}

impl ReportDistributionService {
    pub fn new(distribution_repo: Arc<dyn IReportDistributionRepository>) -> Self {
        ReportDistributionService { distribution_repo }
    }

    /// Create a distribution channel for a report
    pub async fn create_distribution(
        &self,
        report_id: String,
        channel: DistributionChannel,
        recipients: Vec<String>,
    ) -> Result<ReportDistribution, ReportingServiceError> {
        let distribution = ReportDistribution::new(report_id, channel, recipients)
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.distribution_repo
            .save(&distribution)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(distribution)
    }

    /// Get distribution settings for a report
    pub async fn get_distribution(
        &self,
        id: &ReportDistributionId,
    ) -> Result<ReportDistribution, ReportingServiceError> {
        self.distribution_repo
            .find_by_id(id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::InvalidInput(
                "Distribution not found".to_string(),
            ))
    }

    /// Get all distribution channels for a report
    pub async fn get_report_distributions(
        &self,
        report_id: &str,
    ) -> Result<Vec<ReportDistribution>, ReportingServiceError> {
        self.distribution_repo
            .find_by_report_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// Add a recipient to distribution
    pub async fn add_recipient(
        &self,
        id: &ReportDistributionId,
        recipient: String,
    ) -> Result<ReportDistribution, ReportingServiceError> {
        let mut distribution = self.get_distribution(id).await?;

        distribution
            .add_recipient(recipient)
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.distribution_repo
            .save(&distribution)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(distribution)
    }

    /// Remove a recipient from distribution
    pub async fn remove_recipient(
        &self,
        id: &ReportDistributionId,
        recipient: &str,
    ) -> Result<ReportDistribution, ReportingServiceError> {
        let mut distribution = self.get_distribution(id).await?;

        distribution
            .remove_recipient(recipient)
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.distribution_repo
            .save(&distribution)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(distribution)
    }

    /// Deactivate distribution
    pub async fn deactivate(
        &self,
        id: &ReportDistributionId,
    ) -> Result<ReportDistribution, ReportingServiceError> {
        let mut distribution = self.get_distribution(id).await?;

        distribution
            .deactivate()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.distribution_repo
            .save(&distribution)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(distribution)
    }
}

// ============================================================
// ReportArchiveService (BMAD: Report archival & retention)
// ============================================================

pub struct ReportArchiveService {
    archive_repo: Arc<dyn IReportArchiveRepository>,
}

impl ReportArchiveService {
    pub fn new(archive_repo: Arc<dyn IReportArchiveRepository>) -> Self {
        ReportArchiveService { archive_repo }
    }

    /// Archive a report (BMAD: 7-year retention for BCT reports)
    pub async fn archive_report(
        &self,
        report_id: String,
        storage_path: String,
        content_hash: String,
        format: ReportFormatType,
        size_bytes: i64,
        retention_years: i64,
    ) -> Result<ReportArchive, ReportingServiceError> {
        let archive = ReportArchive::new(
            report_id,
            storage_path,
            content_hash,
            format,
            size_bytes,
            retention_years,
        )
        .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.archive_repo
            .save(&archive)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(archive)
    }

    /// Get archived report
    pub async fn get_archived_report(
        &self,
        id: &ReportArchiveId,
    ) -> Result<ReportArchive, ReportingServiceError> {
        self.archive_repo
            .find_by_id(id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::InvalidInput(
                "Archive not found".to_string(),
            ))
    }

    /// Get archives for a report
    pub async fn get_report_archives(
        &self,
        report_id: &str,
    ) -> Result<Vec<ReportArchive>, ReportingServiceError> {
        self.archive_repo
            .find_by_report_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// List expired archives (ready for deletion)
    pub async fn list_expired_archives(&self) -> Result<Vec<ReportArchive>, ReportingServiceError> {
        self.archive_repo
            .find_expired()
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// Audit trail: get all archives
    pub async fn list_all_archives(&self) -> Result<Vec<ReportArchive>, ReportingServiceError> {
        self.archive_repo
            .find_all()
            .await
            .map_err(ReportingServiceError::Internal)
    }
}

// ============================================================
// AdHocReportService (BMAD: Ad-hoc report builder)
// ============================================================

pub struct AdHocReportService {
    adhoc_repo: Arc<dyn IAdHocReportRepository>,
}

impl AdHocReportService {
    pub fn new(adhoc_repo: Arc<dyn IAdHocReportRepository>) -> Self {
        AdHocReportService { adhoc_repo }
    }

    /// Create a new ad-hoc report definition
    pub async fn create_adhoc_report(
        &self,
        name: String,
        description: Option<String>,
        filters: serde_json::Value,
        columns: Vec<String>,
        format: ReportFormatType,
        created_by: Uuid,
    ) -> Result<AdHocReport, ReportingServiceError> {
        let adhoc = AdHocReport::new(name, description, filters, columns, format, created_by)
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.adhoc_repo
            .save(&adhoc)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(adhoc)
    }

    /// Get ad-hoc report
    pub async fn get_adhoc_report(
        &self,
        id: &AdHocReportId,
    ) -> Result<AdHocReport, ReportingServiceError> {
        self.adhoc_repo
            .find_by_id(id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::InvalidInput(
                "AdHoc report not found".to_string(),
            ))
    }

    /// List all ad-hoc reports
    pub async fn list_adhoc_reports(&self) -> Result<Vec<AdHocReport>, ReportingServiceError> {
        self.adhoc_repo
            .find_all()
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// Execute ad-hoc report (mark as executed)
    pub async fn execute_adhoc_report(
        &self,
        id: &AdHocReportId,
    ) -> Result<AdHocReport, ReportingServiceError> {
        let mut adhoc = self.get_adhoc_report(id).await?;

        adhoc
            .mark_executed()
            .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.adhoc_repo
            .save(&adhoc)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(adhoc)
    }
}

// ============================================================
// TaxReportService (BMAD: Tax reporting)
// ============================================================

pub struct TaxReportService {
    tax_repo: Arc<dyn ITaxReportRepository>,
}

impl TaxReportService {
    pub fn new(tax_repo: Arc<dyn ITaxReportRepository>) -> Self {
        TaxReportService { tax_repo }
    }

    /// Generate tax report
    pub async fn generate_tax_report(
        &self,
        tax_type: TaxReportType,
        period_start: NaiveDate,
        period_end: NaiveDate,
        total_amount: f64,
        tax_amount: f64,
        details: serde_json::Value,
        generated_by: Uuid,
    ) -> Result<TaxReport, ReportingServiceError> {
        let tax_report = TaxReport::new(
            tax_type,
            period_start,
            period_end,
            total_amount,
            tax_amount,
            details,
            generated_by,
        )
        .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.tax_repo
            .save(&tax_report)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(tax_report)
    }

    /// Get tax report
    pub async fn get_tax_report(
        &self,
        id: &TaxReportId,
    ) -> Result<TaxReport, ReportingServiceError> {
        self.tax_repo
            .find_by_id(id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::InvalidInput(
                "Tax report not found".to_string(),
            ))
    }

    /// List tax reports by type and period
    pub async fn list_tax_reports(
        &self,
        tax_type: Option<TaxReportType>,
        period_start: Option<NaiveDate>,
        period_end: Option<NaiveDate>,
    ) -> Result<Vec<TaxReport>, ReportingServiceError> {
        self.tax_repo
            .find_by_criteria(tax_type, period_start, period_end)
            .await
            .map_err(ReportingServiceError::Internal)
    }
}

// ============================================================
// Ifrs9ReportService (BMAD: IFRS 9 reporting)
// ============================================================

pub struct AdvancedIfrs9ReportService {
    ifrs9_repo: Arc<dyn IIfrs9ReportRepository>,
}

impl AdvancedIfrs9ReportService {
    pub fn new(ifrs9_repo: Arc<dyn IIfrs9ReportRepository>) -> Self {
        AdvancedIfrs9ReportService { ifrs9_repo }
    }

    /// Generate IFRS 9 report with staging analysis
    pub async fn generate_ifrs9_report(
        &self,
        as_of: NaiveDate,
        staging_analysis: Vec<StagingAnalysis>,
        transition_matrices: Vec<TransitionMatrix>,
        total_ecl: f64,
        generated_by: Uuid,
    ) -> Result<Ifrs9Report, ReportingServiceError> {
        let ifrs9 = Ifrs9Report::new(
            as_of,
            staging_analysis,
            transition_matrices,
            total_ecl,
            generated_by,
        )
        .map_err(|e| ReportingServiceError::DomainError(e.to_string()))?;

        self.ifrs9_repo
            .save(&ifrs9)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(ifrs9)
    }

    /// Get IFRS 9 report
    pub async fn get_ifrs9_report(
        &self,
        id: &Ifrs9ReportId,
    ) -> Result<Ifrs9Report, ReportingServiceError> {
        self.ifrs9_repo
            .find_by_id(id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::InvalidInput(
                "IFRS 9 report not found".to_string(),
            ))
    }

    /// Get latest IFRS 9 report as of a date
    pub async fn get_latest_ifrs9_report(
        &self,
        as_of: NaiveDate,
    ) -> Result<Option<Ifrs9Report>, ReportingServiceError> {
        self.ifrs9_repo
            .find_latest_by_date(as_of)
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// List IFRS 9 reports by period
    pub async fn list_ifrs9_reports(
        &self,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<Vec<Ifrs9Report>, ReportingServiceError> {
        self.ifrs9_repo
            .find_by_period(from, to)
            .await
            .map_err(ReportingServiceError::Internal)
    }

    /// Get ECL by stage (useful for reconciliation)
    pub async fn get_ecl_by_stage(
        &self,
        id: &Ifrs9ReportId,
        stage: CreditStage,
    ) -> Result<Option<f64>, ReportingServiceError> {
        let report = self.get_ifrs9_report(id).await?;
        Ok(report.get_stage_ecl(stage))
    }
}

// ============================================================
// Comprehensive ReportingAdvancedService (Orchestrator)
// ============================================================

pub struct ReportingAdvancedService {
    scheduled: ScheduledReportService,
    distribution: ReportDistributionService,
    archive: ReportArchiveService,
    adhoc: AdHocReportService,
    tax: TaxReportService,
    ifrs9: AdvancedIfrs9ReportService,
}

impl ReportingAdvancedService {
    pub fn new(
        scheduled_repo: Arc<dyn IScheduledReportRepository>,
        distribution_repo: Arc<dyn IReportDistributionRepository>,
        archive_repo: Arc<dyn IReportArchiveRepository>,
        adhoc_repo: Arc<dyn IAdHocReportRepository>,
        tax_repo: Arc<dyn ITaxReportRepository>,
        ifrs9_repo: Arc<dyn IIfrs9ReportRepository>,
    ) -> Self {
        ReportingAdvancedService {
            scheduled: ScheduledReportService::new(scheduled_repo),
            distribution: ReportDistributionService::new(distribution_repo),
            archive: ReportArchiveService::new(archive_repo),
            adhoc: AdHocReportService::new(adhoc_repo),
            tax: TaxReportService::new(tax_repo),
            ifrs9: AdvancedIfrs9ReportService::new(ifrs9_repo),
        }
    }

    // Accessors to sub-services
    pub fn scheduled(&self) -> &ScheduledReportService {
        &self.scheduled
    }
    pub fn distribution(&self) -> &ReportDistributionService {
        &self.distribution
    }
    pub fn archive(&self) -> &ReportArchiveService {
        &self.archive
    }
    pub fn adhoc(&self) -> &AdHocReportService {
        &self.adhoc
    }
    pub fn tax(&self) -> &TaxReportService {
        &self.tax
    }
    pub fn ifrs9(&self) -> &AdvancedIfrs9ReportService {
        &self.ifrs9
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    // Mock repositories
    struct MockScheduledReportRepository {
        reports: Arc<tokio::sync::RwLock<Vec<ScheduledReport>>>,
    }

    #[async_trait]
    impl IScheduledReportRepository for MockScheduledReportRepository {
        async fn save(&self, report: &ScheduledReport) -> Result<(), String> {
            let mut reports = self.reports.write().await;
            reports.push(report.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &ScheduledReportId) -> Result<Option<ScheduledReport>, String> {
            let reports = self.reports.read().await;
            Ok(reports.iter().find(|r| r.scheduled_report_id() == id).cloned())
        }

        async fn find_active(&self) -> Result<Vec<ScheduledReport>, String> {
            let reports = self.reports.read().await;
            Ok(reports.iter().filter(|r| r.is_active()).cloned().collect())
        }

        async fn find_due_for_execution(&self) -> Result<Vec<ScheduledReport>, String> {
            let reports = self.reports.read().await;
            let now = Utc::now();
            Ok(reports
                .iter()
                .filter(|r| r.is_active() && r.next_run().map(|nr| nr <= now).unwrap_or(false))
                .cloned()
                .collect())
        }
    }

    struct MockReportDistributionRepository;

    #[async_trait]
    impl IReportDistributionRepository for MockReportDistributionRepository {
        async fn save(&self, _dist: &ReportDistribution) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &ReportDistributionId) -> Result<Option<ReportDistribution>, String> {
            Ok(None)
        }

        async fn find_by_report_id(&self, _report_id: &str) -> Result<Vec<ReportDistribution>, String> {
            Ok(vec![])
        }
    }

    struct MockReportArchiveRepository;

    #[async_trait]
    impl IReportArchiveRepository for MockReportArchiveRepository {
        async fn save(&self, _archive: &ReportArchive) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &ReportArchiveId) -> Result<Option<ReportArchive>, String> {
            Ok(None)
        }

        async fn find_by_report_id(&self, _report_id: &str) -> Result<Vec<ReportArchive>, String> {
            Ok(vec![])
        }

        async fn find_expired(&self) -> Result<Vec<ReportArchive>, String> {
            Ok(vec![])
        }

        async fn find_all(&self) -> Result<Vec<ReportArchive>, String> {
            Ok(vec![])
        }
    }

    struct MockAdHocReportRepository;

    #[async_trait]
    impl IAdHocReportRepository for MockAdHocReportRepository {
        async fn save(&self, _report: &AdHocReport) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &AdHocReportId) -> Result<Option<AdHocReport>, String> {
            Ok(None)
        }

        async fn find_all(&self) -> Result<Vec<AdHocReport>, String> {
            Ok(vec![])
        }
    }

    struct MockTaxReportRepository;

    #[async_trait]
    impl ITaxReportRepository for MockTaxReportRepository {
        async fn save(&self, _report: &TaxReport) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &TaxReportId) -> Result<Option<TaxReport>, String> {
            Ok(None)
        }

        async fn find_by_criteria(
            &self,
            _tax_type: Option<TaxReportType>,
            _period_start: Option<NaiveDate>,
            _period_end: Option<NaiveDate>,
        ) -> Result<Vec<TaxReport>, String> {
            Ok(vec![])
        }
    }

    struct MockIfrs9ReportRepository;

    #[async_trait]
    impl IIfrs9ReportRepository for MockIfrs9ReportRepository {
        async fn save(&self, _report: &Ifrs9Report) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: &Ifrs9ReportId) -> Result<Option<Ifrs9Report>, String> {
            Ok(None)
        }

        async fn find_latest_by_date(&self, _as_of: NaiveDate) -> Result<Option<Ifrs9Report>, String> {
            Ok(None)
        }

        async fn find_by_period(
            &self,
            _from: Option<NaiveDate>,
            _to: Option<NaiveDate>,
        ) -> Result<Vec<Ifrs9Report>, String> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_scheduled_report_service_create() {
        let repo = Arc::new(MockScheduledReportRepository {
            reports: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        });
        let service = ScheduledReportService::new(repo);

        let result = service
            .create_scheduled_report(
                "Daily Report".to_string(),
                None,
                "Prudential".to_string(),
                ScheduleFrequency::Daily,
                Some("0 9 * * *".to_string()),
                None,
                Uuid::new_v4(),
            )
            .await;

        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.name(), "Daily Report");
    }

    #[tokio::test]
    async fn test_distribution_service_create() {
        let repo = Arc::new(MockReportDistributionRepository);
        let service = ReportDistributionService::new(repo);

        let result = service
            .create_distribution(
                "report-001".to_string(),
                DistributionChannel::Email,
                vec!["test@bank.tn".to_string()],
            )
            .await;

        assert!(result.is_ok());
        let dist = result.unwrap();
        assert_eq!(dist.report_id(), "report-001");
    }

    #[tokio::test]
    async fn test_archive_service_create() {
        let repo = Arc::new(MockReportArchiveRepository);
        let service = ReportArchiveService::new(repo);

        let result = service
            .archive_report(
                "report-001".to_string(),
                "s3://reports/report-001.xbrl".to_string(),
                "sha256hash".to_string(),
                ReportFormatType::Xbrl,
                102400,
                7,
            )
            .await;

        assert!(result.is_ok());
        let archive = result.unwrap();
        assert_eq!(archive.report_id(), "report-001");
    }

    #[tokio::test]
    async fn test_adhoc_service_create() {
        let repo = Arc::new(MockAdHocReportRepository);
        let service = AdHocReportService::new(repo);

        let result = service
            .create_adhoc_report(
                "Transactions".to_string(),
                None,
                serde_json::json!({}),
                vec!["account_id".to_string()],
                ReportFormatType::Csv,
                Uuid::new_v4(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tax_service_create() {
        let repo = Arc::new(MockTaxReportRepository);
        let service = TaxReportService::new(repo);

        let result = service
            .generate_tax_report(
                TaxReportType::Tva,
                NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
                1000000.0,
                180000.0,
                serde_json::json!({}),
                Uuid::new_v4(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ifrs9_service_create() {
        let repo = Arc::new(MockIfrs9ReportRepository);
        let service = AdvancedIfrs9ReportService::new(repo);

        let staging = vec![StagingAnalysis {
            stage: CreditStage::Stage1,
            loan_count: 100,
            ecl_amount: 50000.0,
            probability_of_default: 0.01,
            loss_given_default: 0.4,
            exposure_at_default: 5000000.0,
        }];

        let result = service
            .generate_ifrs9_report(
                NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
                staging,
                vec![],
                50000.0,
                Uuid::new_v4(),
            )
            .await;

        assert!(result.is_ok());
    }
}
