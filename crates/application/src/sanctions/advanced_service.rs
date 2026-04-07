use std::sync::Arc;
use uuid::Uuid;

use banko_domain::sanctions::*;

use super::errors::SanctionsServiceError;
use super::ports::*;

// ============================================================================
// Batch Screening Service (FR-058)
// ============================================================================

pub struct BatchScreeningService {
    job_repo: Arc<dyn IBatchScreeningJobRepository>,
    _result_repo: Arc<dyn IScreeningResultRepository>,
}

impl BatchScreeningService {
    pub fn new(
        job_repo: Arc<dyn IBatchScreeningJobRepository>,
        result_repo: Arc<dyn IScreeningResultRepository>,
    ) -> Self {
        Self { job_repo, _result_repo: result_repo }
    }

    /// Create a new batch screening job
    pub async fn create_batch_job(
        &self,
        job_type: BatchJobType,
        total_count: i64,
        triggered_by: Uuid,
    ) -> Result<BatchScreeningJob, SanctionsServiceError> {
        let job = BatchScreeningJob::new(job_type, total_count, triggered_by)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.job_repo
            .save(&job)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(job)
    }

    /// Start a batch job
    pub async fn start_batch_job(
        &self,
        job_id: &BatchScreeningJobId,
    ) -> Result<(), SanctionsServiceError> {
        let mut job = self
            .job_repo
            .find_by_id(job_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Job not found".to_string()))?;

        job.mark_started()
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.job_repo
            .save(&job)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Record screening result for a customer in batch
    pub async fn record_batch_result(
        &self,
        job_id: &BatchScreeningJobId,
        hit: bool,
        potential_match: bool,
        error: bool,
    ) -> Result<(), SanctionsServiceError> {
        let mut job = self
            .job_repo
            .find_by_id(job_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Job not found".to_string()))?;

        job.record_result(hit, potential_match, error)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.job_repo
            .save(&job)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Complete a batch job
    pub async fn complete_batch_job(
        &self,
        job_id: &BatchScreeningJobId,
    ) -> Result<(), SanctionsServiceError> {
        let mut job = self
            .job_repo
            .find_by_id(job_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Job not found".to_string()))?;

        job.mark_completed()
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.job_repo
            .save(&job)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Fail a batch job
    pub async fn fail_batch_job(
        &self,
        job_id: &BatchScreeningJobId,
        error_summary: String,
    ) -> Result<(), SanctionsServiceError> {
        let mut job = self
            .job_repo
            .find_by_id(job_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Job not found".to_string()))?;

        job.mark_failed(error_summary)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.job_repo
            .save(&job)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Get pending batch jobs
    pub async fn get_pending_jobs(&self) -> Result<Vec<BatchScreeningJob>, SanctionsServiceError> {
        self.job_repo
            .find_by_status(BatchJobStatus::Pending)
            .await
            .map_err(SanctionsServiceError::Internal)
    }

    /// Get active batch jobs
    pub async fn get_active_jobs(&self) -> Result<Vec<BatchScreeningJob>, SanctionsServiceError> {
        self.job_repo
            .find_by_status(BatchJobStatus::InProgress)
            .await
            .map_err(SanctionsServiceError::Internal)
    }
}

// ============================================================================
// Sanctions Whitelist Service (FR-057)
// ============================================================================

pub struct SanctionsWhitelistService {
    whitelist_repo: Arc<dyn ISanctionsWhitelistRepository>,
}

impl SanctionsWhitelistService {
    pub fn new(whitelist_repo: Arc<dyn ISanctionsWhitelistRepository>) -> Self {
        Self { whitelist_repo }
    }

    /// Create a whitelist entry for confirmed false positive
    pub async fn create_whitelist_entry(
        &self,
        entity_name: String,
        document_type: String,
        document_number: String,
        justification: String,
        valid_days: i64,
    ) -> Result<SanctionsWhitelistEntry, SanctionsServiceError> {
        let entry = SanctionsWhitelistEntry::new(
            entity_name,
            document_type,
            document_number,
            justification,
            valid_days,
        )
        .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.whitelist_repo
            .save(&entry)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(entry)
    }

    /// Approve a whitelist entry
    pub async fn approve_whitelist(
        &self,
        entry_id: &SanctionsWhitelistEntryId,
        approved_by: Uuid,
    ) -> Result<(), SanctionsServiceError> {
        let mut entry = self
            .whitelist_repo
            .find_by_id(entry_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Entry not found".to_string()))?;

        entry
            .approve(approved_by)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.whitelist_repo
            .save(&entry)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Reject a whitelist entry
    pub async fn reject_whitelist(
        &self,
        entry_id: &SanctionsWhitelistEntryId,
    ) -> Result<(), SanctionsServiceError> {
        let mut entry = self
            .whitelist_repo
            .find_by_id(entry_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Entry not found".to_string()))?;

        entry
            .reject()
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.whitelist_repo
            .save(&entry)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Check if entity is whitelisted
    pub async fn is_whitelisted(
        &self,
        document_number: &str,
    ) -> Result<bool, SanctionsServiceError> {
        let entry = self
            .whitelist_repo
            .find_by_document_number(document_number)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(entry.map(|e| e.is_valid()).unwrap_or(false))
    }

    /// Get pending whitelist approvals
    pub async fn get_pending_approvals(
        &self,
    ) -> Result<Vec<SanctionsWhitelistEntry>, SanctionsServiceError> {
        self.whitelist_repo
            .find_by_status(WhitelistApprovalStatus::Pending)
            .await
            .map_err(SanctionsServiceError::Internal)
    }

    /// Clean up expired whitelist entries
    pub async fn clean_expired_entries(&self) -> Result<i64, SanctionsServiceError> {
        self.whitelist_repo
            .mark_expired_entries()
            .await
            .map_err(SanctionsServiceError::Internal)
    }
}

// ============================================================================
// Escalation Rule Service (FR-060)
// ============================================================================

pub struct EscalationRuleService {
    rule_repo: Arc<dyn IEscalationRuleRepository>,
}

impl EscalationRuleService {
    pub fn new(rule_repo: Arc<dyn IEscalationRuleRepository>) -> Self {
        Self { rule_repo }
    }

    /// Create an escalation rule
    pub async fn create_rule(
        &self,
        trigger: EscalationTrigger,
        similarity_threshold: u8,
    ) -> Result<EscalationRule, SanctionsServiceError> {
        let rule = EscalationRule::new(trigger, similarity_threshold)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.rule_repo
            .save(&rule)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(rule)
    }

    /// Get escalation rules for a trigger
    pub async fn get_rules_for_trigger(
        &self,
        trigger: EscalationTrigger,
    ) -> Result<Vec<EscalationRule>, SanctionsServiceError> {
        self.rule_repo
            .find_by_trigger(trigger)
            .await
            .map_err(SanctionsServiceError::Internal)
    }

    /// Check if match should escalate
    pub async fn should_escalate(
        &self,
        trigger: EscalationTrigger,
        match_confidence: u8,
    ) -> Result<bool, SanctionsServiceError> {
        let rules = self.get_rules_for_trigger(trigger).await?;
        Ok(rules.iter().any(|r| r.should_escalate(match_confidence)))
    }

    /// Update rule threshold
    pub async fn update_threshold(
        &self,
        rule_id: &EscalationRuleId,
        new_threshold: u8,
    ) -> Result<(), SanctionsServiceError> {
        let mut rule = self
            .rule_repo
            .find_by_id(rule_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Rule not found".to_string()))?;

        rule.update_threshold(new_threshold)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.rule_repo
            .save(&rule)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }
}

// ============================================================================
// Sanctions Report Service (FR-062)
// ============================================================================

pub struct SanctionsReportService {
    report_repo: Arc<dyn ISanctionsReportRepository>,
}

impl SanctionsReportService {
    pub fn new(report_repo: Arc<dyn ISanctionsReportRepository>) -> Self {
        Self { report_repo }
    }

    /// Create a new report
    pub async fn create_report(
        &self,
        period: ReportPeriod,
        period_start: chrono::DateTime<chrono::Utc>,
        period_end: chrono::DateTime<chrono::Utc>,
    ) -> Result<SanctionsReport, SanctionsServiceError> {
        let report = SanctionsReport::new(period, period_start, period_end)
            .map_err(|e| SanctionsServiceError::Internal(e.to_string()))?;

        self.report_repo
            .save(&report)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(report)
    }

    /// Add screening result to report
    pub async fn record_screening(
        &self,
        report_id: &SanctionsReportId,
        hit: bool,
        potential_match: bool,
        source: Option<String>,
        high_risk: bool,
    ) -> Result<(), SanctionsServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Report not found".to_string()))?;

        report.record_screening(hit, potential_match, source, high_risk);

        self.report_repo
            .save(&report)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Record false positive in report
    pub async fn record_false_positive(
        &self,
        report_id: &SanctionsReportId,
    ) -> Result<(), SanctionsServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Report not found".to_string()))?;

        report.record_false_positive();

        self.report_repo
            .save(&report)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Record escalation in report
    pub async fn record_escalation(
        &self,
        report_id: &SanctionsReportId,
    ) -> Result<(), SanctionsServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Report not found".to_string()))?;

        report.record_escalation();

        self.report_repo
            .save(&report)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Record account freeze in report
    pub async fn record_freeze(
        &self,
        report_id: &SanctionsReportId,
    ) -> Result<(), SanctionsServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::Internal("Report not found".to_string()))?;

        report.record_freeze();

        self.report_repo
            .save(&report)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(())
    }

    /// Get monthly report
    pub async fn get_monthly_report(
        &self,
        year: i32,
        month: u32,
    ) -> Result<Option<SanctionsReport>, SanctionsServiceError> {
        self.report_repo
            .find_by_period_month(year, month)
            .await
            .map_err(SanctionsServiceError::Internal)
    }

    /// Get quarterly report
    pub async fn get_quarterly_report(
        &self,
        year: i32,
        quarter: u8,
    ) -> Result<Option<SanctionsReport>, SanctionsServiceError> {
        self.report_repo
            .find_by_period_quarter(year, quarter)
            .await
            .map_err(SanctionsServiceError::Internal)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock repositories
    struct MockBatchJobRepository;

    #[async_trait::async_trait]
    impl IBatchScreeningJobRepository for MockBatchJobRepository {
        async fn save(&self, _job: &BatchScreeningJob) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &BatchScreeningJobId,
        ) -> Result<Option<BatchScreeningJob>, String> {
            Ok(None)
        }

        async fn find_by_status(
            &self,
            _status: BatchJobStatus,
        ) -> Result<Vec<BatchScreeningJob>, String> {
            Ok(Vec::new())
        }
    }

    struct MockWhitelistRepository;

    #[async_trait::async_trait]
    impl ISanctionsWhitelistRepository for MockWhitelistRepository {
        async fn save(&self, _entry: &SanctionsWhitelistEntry) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &SanctionsWhitelistEntryId,
        ) -> Result<Option<SanctionsWhitelistEntry>, String> {
            Ok(None)
        }

        async fn find_by_document_number(
            &self,
            _document_number: &str,
        ) -> Result<Option<SanctionsWhitelistEntry>, String> {
            Ok(None)
        }

        async fn find_by_status(
            &self,
            _status: WhitelistApprovalStatus,
        ) -> Result<Vec<SanctionsWhitelistEntry>, String> {
            Ok(Vec::new())
        }

        async fn mark_expired_entries(&self) -> Result<i64, String> {
            Ok(0)
        }
    }

    struct MockEscalationRuleRepository;

    #[async_trait::async_trait]
    impl IEscalationRuleRepository for MockEscalationRuleRepository {
        async fn save(&self, _rule: &EscalationRule) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &EscalationRuleId,
        ) -> Result<Option<EscalationRule>, String> {
            Ok(None)
        }

        async fn find_by_trigger(
            &self,
            _trigger: EscalationTrigger,
        ) -> Result<Vec<EscalationRule>, String> {
            Ok(Vec::new())
        }

        async fn find_active(&self) -> Result<Vec<EscalationRule>, String> {
            Ok(Vec::new())
        }
    }

    struct MockSanctionsReportRepository;

    #[async_trait::async_trait]
    impl ISanctionsReportRepository for MockSanctionsReportRepository {
        async fn save(&self, _report: &SanctionsReport) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &SanctionsReportId,
        ) -> Result<Option<SanctionsReport>, String> {
            Ok(None)
        }

        async fn find_by_period_month(
            &self,
            _year: i32,
            _month: u32,
        ) -> Result<Option<SanctionsReport>, String> {
            Ok(None)
        }

        async fn find_by_period_quarter(
            &self,
            _year: i32,
            _quarter: u8,
        ) -> Result<Option<SanctionsReport>, String> {
            Ok(None)
        }
    }

    struct MockScreeningResultRepository;

    #[async_trait::async_trait]
    impl IScreeningResultRepository for MockScreeningResultRepository {
        async fn save(&self, _result: &ScreeningResult) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &ScreeningResultId,
        ) -> Result<Option<ScreeningResult>, String> {
            Ok(None)
        }

        async fn find_recent(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<ScreeningResult>, String> {
            Ok(Vec::new())
        }

        async fn count_by_status(
            &self,
            _status: Option<ScreeningStatus>,
        ) -> Result<i64, String> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_batch_screening_service_creation() {
        let job_repo = Arc::new(MockBatchJobRepository);
        let result_repo = Arc::new(MockScreeningResultRepository);
        let service = BatchScreeningService::new(job_repo, result_repo);

        let result = service
            .create_batch_job(
                BatchJobType::ScreenAllCustomers,
                100,
                Uuid::new_v4(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_whitelist_service() {
        let whitelist_repo = Arc::new(MockWhitelistRepository);
        let service = SanctionsWhitelistService::new(whitelist_repo);

        let result = service
            .create_whitelist_entry(
                "John Doe".to_string(),
                "passport".to_string(),
                "ID123456".to_string(),
                "False positive".to_string(),
                90,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_escalation_rule_service() {
        let rule_repo = Arc::new(MockEscalationRuleRepository);
        let service = EscalationRuleService::new(rule_repo);

        let result = service
            .create_rule(EscalationTrigger::ConfirmedHit, 85)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sanctions_report_service() {
        let report_repo = Arc::new(MockSanctionsReportRepository);
        let service = SanctionsReportService::new(report_repo);

        let now = chrono::Utc::now();
        let result = service
            .create_report(
                ReportPeriod::Monthly,
                now - chrono::Duration::days(30),
                now,
            )
            .await;

        assert!(result.is_ok());
    }
}
