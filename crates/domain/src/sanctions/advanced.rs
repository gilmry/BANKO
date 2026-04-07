use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================================
// BMAD FR-058: Batch Screening Job (Mass customer/list updates)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchScreeningJobId(Uuid);

impl BatchScreeningJobId {
    pub fn new() -> Self {
        BatchScreeningJobId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        BatchScreeningJobId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BatchScreeningJobId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BatchJobStatus {
    /// Scheduled but not yet started
    Pending,
    /// Currently executing
    InProgress,
    /// Completed successfully
    Completed,
    /// Completed with errors (partial success)
    CompletedWithErrors,
    /// Failed completely
    Failed,
    /// Cancelled by user
    Cancelled,
}

impl BatchJobStatus {
    pub fn as_str(&self) -> &str {
        match self {
            BatchJobStatus::Pending => "Pending",
            BatchJobStatus::InProgress => "InProgress",
            BatchJobStatus::Completed => "Completed",
            BatchJobStatus::CompletedWithErrors => "CompletedWithErrors",
            BatchJobStatus::Failed => "Failed",
            BatchJobStatus::Cancelled => "Cancelled",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BatchJobType {
    /// Screen all active customers against sanction lists
    ScreenAllCustomers,
    /// Screen newly added customers
    ScreenNewCustomers,
    /// Re-screen following updated sanction list
    ListUpdate,
    /// Periodic compliance check (monthly, quarterly)
    PeriodicCompliance,
}

impl BatchJobType {
    pub fn as_str(&self) -> &str {
        match self {
            BatchJobType::ScreenAllCustomers => "ScreenAllCustomers",
            BatchJobType::ScreenNewCustomers => "ScreenNewCustomers",
            BatchJobType::ListUpdate => "ListUpdate",
            BatchJobType::PeriodicCompliance => "PeriodicCompliance",
        }
    }
}

/// Batch screening job for mass customer screening
/// Implements FR-058: Screening batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchScreeningJob {
    id: BatchScreeningJobId,
    job_type: BatchJobType,
    status: BatchJobStatus,
    /// Total customers to screen in this batch
    total_count: i64,
    /// Customers processed so far
    processed_count: i64,
    /// Number of hits found
    hits_count: i64,
    /// Number of potential matches
    potential_matches_count: i64,
    /// Number of errors during processing
    error_count: i64,
    /// Job creation timestamp
    created_at: DateTime<Utc>,
    /// Job start timestamp
    started_at: Option<DateTime<Utc>>,
    /// Job completion timestamp
    completed_at: Option<DateTime<Utc>>,
    /// Error summary (if any)
    error_summary: Option<String>,
    /// Compliance officer who triggered the job
    triggered_by: Uuid,
}

impl BatchScreeningJob {
    pub fn new(
        job_type: BatchJobType,
        total_count: i64,
        triggered_by: Uuid,
    ) -> Result<Self, DomainError> {
        if total_count <= 0 {
            return Err(DomainError::InvalidAlert(
                "Total count must be positive".to_string(),
            ));
        }

        Ok(Self {
            id: BatchScreeningJobId::new(),
            job_type,
            status: BatchJobStatus::Pending,
            total_count,
            processed_count: 0,
            hits_count: 0,
            potential_matches_count: 0,
            error_count: 0,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error_summary: None,
            triggered_by,
        })
    }

    pub fn id(&self) -> &BatchScreeningJobId {
        &self.id
    }
    pub fn status(&self) -> BatchJobStatus {
        self.status
    }
    pub fn processed_count(&self) -> i64 {
        self.processed_count
    }

    /// Calculate progress percentage
    pub fn progress_percent(&self) -> u8 {
        if self.total_count == 0 {
            return 0;
        }
        ((self.processed_count as f64 / self.total_count as f64) * 100.0).min(100.0) as u8
    }

    /// Mark job as started
    pub fn mark_started(&mut self) -> Result<(), DomainError> {
        if self.status != BatchJobStatus::Pending {
            return Err(DomainError::InvalidAlert(
                "Can only start from Pending status".to_string(),
            ));
        }
        self.status = BatchJobStatus::InProgress;
        self.started_at = Some(Utc::now());
        Ok(())
    }

    /// Record a processing result
    pub fn record_result(
        &mut self,
        hit: bool,
        potential_match: bool,
        error: bool,
    ) -> Result<(), DomainError> {
        if self.processed_count >= self.total_count {
            return Err(DomainError::InvalidAlert(
                "All customers already processed".to_string(),
            ));
        }

        self.processed_count += 1;
        if hit {
            self.hits_count += 1;
        }
        if potential_match {
            self.potential_matches_count += 1;
        }
        if error {
            self.error_count += 1;
        }

        Ok(())
    }

    /// Mark job as completed
    pub fn mark_completed(&mut self) -> Result<(), DomainError> {
        if self.status != BatchJobStatus::InProgress {
            return Err(DomainError::InvalidAlert(
                "Can only complete from InProgress status".to_string(),
            ));
        }

        self.status = if self.error_count > 0 {
            BatchJobStatus::CompletedWithErrors
        } else {
            BatchJobStatus::Completed
        };
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    /// Mark job as failed
    pub fn mark_failed(&mut self, error_summary: String) -> Result<(), DomainError> {
        if !matches!(
            self.status,
            BatchJobStatus::Pending | BatchJobStatus::InProgress
        ) {
            return Err(DomainError::InvalidAlert(
                "Can only fail from Pending or InProgress status".to_string(),
            ));
        }
        self.status = BatchJobStatus::Failed;
        self.error_summary = Some(error_summary);
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    /// Mark job as cancelled
    pub fn mark_cancelled(&mut self) -> Result<(), DomainError> {
        if self.status == BatchJobStatus::Completed
            || self.status == BatchJobStatus::CompletedWithErrors
            || self.status == BatchJobStatus::Failed
        {
            return Err(DomainError::InvalidAlert(
                "Cannot cancel a finished job".to_string(),
            ));
        }
        self.status = BatchJobStatus::Cancelled;
        self.completed_at = Some(Utc::now());
        Ok(())
    }
}

// ============================================================================
// BMAD FR-057: Sanctions Whitelist (False Positive Management)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SanctionsWhitelistEntryId(Uuid);

impl SanctionsWhitelistEntryId {
    pub fn new() -> Self {
        SanctionsWhitelistEntryId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        SanctionsWhitelistEntryId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SanctionsWhitelistEntryId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WhitelistApprovalStatus {
    /// Pending review by compliance officer
    Pending,
    /// Approved - entity is whitelisted
    Approved,
    /// Rejected - entity must be rescreened
    Rejected,
    /// Expired - review needed
    Expired,
}

impl WhitelistApprovalStatus {
    pub fn as_str(&self) -> &str {
        match self {
            WhitelistApprovalStatus::Pending => "Pending",
            WhitelistApprovalStatus::Approved => "Approved",
            WhitelistApprovalStatus::Rejected => "Rejected",
            WhitelistApprovalStatus::Expired => "Expired",
        }
    }
}

/// Whitelist entry for confirmed false positives
/// Implements FR-057: False positive management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsWhitelistEntry {
    id: SanctionsWhitelistEntryId,
    /// Customer or counterparty name
    entity_name: String,
    /// Document type (passport, national_id, etc.)
    document_type: String,
    /// Document number (for unique identification)
    document_number: String,
    /// Sanction list entry that caused false positive match
    matched_sanction_entry_id: Option<Uuid>,
    /// Reason for whitelisting
    justification: String,
    approval_status: WhitelistApprovalStatus,
    /// Officer who approved whitelist entry
    approved_by: Option<Uuid>,
    /// Approval timestamp
    approved_at: Option<DateTime<Utc>>,
    /// Valid until (reviewer can set expiry)
    valid_until: DateTime<Utc>,
    /// Entry creation timestamp
    created_at: DateTime<Utc>,
}

impl SanctionsWhitelistEntry {
    pub fn new(
        entity_name: String,
        document_type: String,
        document_number: String,
        justification: String,
        valid_days: i64,
    ) -> Result<Self, DomainError> {
        if entity_name.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Entity name cannot be empty".to_string(),
            ));
        }
        if document_number.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Document number cannot be empty".to_string(),
            ));
        }
        if justification.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Justification cannot be empty".to_string(),
            ));
        }
        if valid_days <= 0 {
            return Err(DomainError::InvalidAlert(
                "Validity period must be positive".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(Self {
            id: SanctionsWhitelistEntryId::new(),
            entity_name,
            document_type,
            document_number,
            matched_sanction_entry_id: None,
            justification,
            approval_status: WhitelistApprovalStatus::Pending,
            approved_by: None,
            approved_at: None,
            valid_until: now + chrono::Duration::days(valid_days),
            created_at: now,
        })
    }

    pub fn id(&self) -> &SanctionsWhitelistEntryId {
        &self.id
    }
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }
    pub fn approval_status(&self) -> WhitelistApprovalStatus {
        self.approval_status
    }

    /// Check if whitelist entry is valid
    pub fn is_valid(&self) -> bool {
        self.approval_status == WhitelistApprovalStatus::Approved && Utc::now() <= self.valid_until
    }

    /// Approve the whitelist entry
    pub fn approve(&mut self, approved_by: Uuid) -> Result<(), DomainError> {
        if self.approval_status != WhitelistApprovalStatus::Pending {
            return Err(DomainError::InvalidAlert(
                "Can only approve from Pending status".to_string(),
            ));
        }
        self.approval_status = WhitelistApprovalStatus::Approved;
        self.approved_by = Some(approved_by);
        self.approved_at = Some(Utc::now());
        Ok(())
    }

    /// Reject the whitelist request
    pub fn reject(&mut self) -> Result<(), DomainError> {
        if self.approval_status != WhitelistApprovalStatus::Pending {
            return Err(DomainError::InvalidAlert(
                "Can only reject from Pending status".to_string(),
            ));
        }
        self.approval_status = WhitelistApprovalStatus::Rejected;
        Ok(())
    }

    /// Mark as expired when validity period ends
    pub fn mark_expired(&mut self) -> Result<(), DomainError> {
        if self.approval_status != WhitelistApprovalStatus::Approved {
            return Err(DomainError::InvalidAlert(
                "Can only expire approved entries".to_string(),
            ));
        }
        self.approval_status = WhitelistApprovalStatus::Expired;
        Ok(())
    }
}

// ============================================================================
// BMAD FR-060: Escalation Rule (Automatic escalation to CTAF)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EscalationRuleId(Uuid);

impl EscalationRuleId {
    pub fn new() -> Self {
        EscalationRuleId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        EscalationRuleId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for EscalationRuleId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EscalationTrigger {
    /// Confirmed sanctions hit on any list
    ConfirmedHit,
    /// Hit on high-risk list (OFAC SDN, UN)
    HighRiskList,
    /// Multiple potential matches from different lists
    MultipleMatches,
    /// Hit on country-level sanctions
    CountrySanctions,
}

impl EscalationTrigger {
    pub fn as_str(&self) -> &str {
        match self {
            EscalationTrigger::ConfirmedHit => "ConfirmedHit",
            EscalationTrigger::HighRiskList => "HighRiskList",
            EscalationTrigger::MultipleMatches => "MultipleMatches",
            EscalationTrigger::CountrySanctions => "CountrySanctions",
        }
    }
}

/// Escalation rule for automatic CTAF reporting
/// Implements FR-060: Escalation automatique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    id: EscalationRuleId,
    trigger: EscalationTrigger,
    /// Active flag
    active: bool,
    /// Similarity threshold that triggers escalation (e.g., 90 for high confidence)
    similarity_threshold: u8,
    /// Automatically report to CTAF if triggered
    auto_report_to_ctaf: bool,
    /// Automatically freeze account if triggered
    auto_freeze_account: bool,
    /// Notify compliance team immediately
    auto_notify_compliance: bool,
    /// Escalation priority (Critical, High, Medium)
    priority: String,
    /// Created timestamp
    created_at: DateTime<Utc>,
    /// Last updated
    updated_at: DateTime<Utc>,
}

impl EscalationRule {
    pub fn new(
        trigger: EscalationTrigger,
        similarity_threshold: u8,
    ) -> Result<Self, DomainError> {
        if similarity_threshold > 100 {
            return Err(DomainError::InvalidAlert(
                "Similarity threshold must be 0-100".to_string(),
            ));
        }

        let priority = match trigger {
            EscalationTrigger::HighRiskList => "Critical".to_string(),
            EscalationTrigger::ConfirmedHit => "High".to_string(),
            EscalationTrigger::MultipleMatches => "High".to_string(),
            EscalationTrigger::CountrySanctions => "Critical".to_string(),
        };

        let now = Utc::now();

        Ok(Self {
            id: EscalationRuleId::new(),
            trigger,
            active: true,
            similarity_threshold,
            auto_report_to_ctaf: true,
            auto_freeze_account: true,
            auto_notify_compliance: true,
            priority,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn id(&self) -> &EscalationRuleId {
        &self.id
    }
    pub fn trigger(&self) -> EscalationTrigger {
        self.trigger
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if a match should trigger escalation
    pub fn should_escalate(&self, match_confidence: u8) -> bool {
        self.active && match_confidence >= self.similarity_threshold
    }

    /// Disable the rule
    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.active = false;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update threshold
    pub fn update_threshold(&mut self, new_threshold: u8) -> Result<(), DomainError> {
        if new_threshold > 100 {
            return Err(DomainError::InvalidAlert(
                "Threshold must be 0-100".to_string(),
            ));
        }
        self.similarity_threshold = new_threshold;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ============================================================================
// BMAD FR-062: Sanctions Reporting (Metrics & Statistics)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SanctionsReportId(Uuid);

impl SanctionsReportId {
    pub fn new() -> Self {
        SanctionsReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        SanctionsReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SanctionsReportId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportPeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

impl ReportPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            ReportPeriod::Daily => "Daily",
            ReportPeriod::Weekly => "Weekly",
            ReportPeriod::Monthly => "Monthly",
            ReportPeriod::Quarterly => "Quarterly",
        }
    }
}

/// Sanctions screening metrics and reporting
/// Implements FR-062: Reporting sanctions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionsReport {
    id: SanctionsReportId,
    period: ReportPeriod,
    /// Start of reporting period
    period_start: DateTime<Utc>,
    /// End of reporting period
    period_end: DateTime<Utc>,
    /// Total screening operations performed
    total_screenings: i64,
    /// Number of confirmed sanctions hits
    confirmed_hits: i64,
    /// Number of potential matches (below confirmation threshold)
    potential_matches: i64,
    /// Number of false positives identified and whitelisted
    false_positives: i64,
    /// Breakdown by sanction list source
    hits_by_source: std::collections::HashMap<String, i64>, // e.g., {"UN": 5, "OFAC": 2}
    /// High-risk screenings (>90% similarity)
    high_risk_findings: i64,
    /// Number of automatic escalations to CTAF
    escalations_to_ctaf: i64,
    /// Number of accounts frozen due to sanctions
    accounts_frozen: i64,
    /// Generated timestamp
    generated_at: DateTime<Utc>,
}

impl SanctionsReport {
    pub fn new(period: ReportPeriod, period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> Result<Self, DomainError> {
        if period_start >= period_end {
            return Err(DomainError::InvalidAlert(
                "Period start must be before period end".to_string(),
            ));
        }

        Ok(Self {
            id: SanctionsReportId::new(),
            period,
            period_start,
            period_end,
            total_screenings: 0,
            confirmed_hits: 0,
            potential_matches: 0,
            false_positives: 0,
            hits_by_source: std::collections::HashMap::new(),
            high_risk_findings: 0,
            escalations_to_ctaf: 0,
            accounts_frozen: 0,
            generated_at: Utc::now(),
        })
    }

    pub fn id(&self) -> &SanctionsReportId {
        &self.id
    }
    pub fn total_screenings(&self) -> i64 {
        self.total_screenings
    }
    pub fn confirmed_hits(&self) -> i64 {
        self.confirmed_hits
    }

    /// Add screening result to report
    pub fn record_screening(
        &mut self,
        hit: bool,
        potential_match: bool,
        source: Option<String>,
        high_risk: bool,
    ) {
        self.total_screenings += 1;

        if hit {
            self.confirmed_hits += 1;
            if let Some(src) = source {
                *self.hits_by_source.entry(src).or_insert(0) += 1;
            }
        }

        if potential_match {
            self.potential_matches += 1;
        }

        if high_risk {
            self.high_risk_findings += 1;
        }
    }

    /// Record a false positive
    pub fn record_false_positive(&mut self) {
        self.false_positives += 1;
    }

    /// Record CTAF escalation
    pub fn record_escalation(&mut self) {
        self.escalations_to_ctaf += 1;
    }

    /// Record account freeze
    pub fn record_freeze(&mut self) {
        self.accounts_frozen += 1;
    }

    /// Calculate hit rate (%)
    pub fn hit_rate(&self) -> f64 {
        if self.total_screenings == 0 {
            return 0.0;
        }
        (self.confirmed_hits as f64 / self.total_screenings as f64) * 100.0
    }

    /// Calculate false positive rate (%)
    pub fn false_positive_rate(&self) -> f64 {
        if self.confirmed_hits == 0 {
            return 0.0;
        }
        (self.false_positives as f64 / (self.confirmed_hits + self.false_positives) as f64)
            * 100.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_screening_job_creation() {
        let job = BatchScreeningJob::new(
            BatchJobType::ScreenAllCustomers,
            1000,
            Uuid::new_v4(),
        );
        assert!(job.is_ok());
        let j = job.unwrap();
        assert_eq!(j.status(), BatchJobStatus::Pending);
    }

    #[test]
    fn test_batch_job_workflow() {
        let mut job = BatchScreeningJob::new(
            BatchJobType::ListUpdate,
            100,
            Uuid::new_v4(),
        )
        .unwrap();

        let _ = job.mark_started();
        assert_eq!(job.status(), BatchJobStatus::InProgress);

        for _ in 0..50 {
            let _ = job.record_result(false, false, false);
        }
        assert_eq!(job.progress_percent(), 50);

        for _ in 0..50 {
            let _ = job.record_result(true, false, false);
        }

        let _ = job.mark_completed();
        assert_eq!(job.status(), BatchJobStatus::Completed);
        assert_eq!(job.hits_count, 50);
    }

    #[test]
    fn test_batch_job_with_errors() {
        let mut job = BatchScreeningJob::new(
            BatchJobType::ScreenNewCustomers,
            100,
            Uuid::new_v4(),
        )
        .unwrap();

        let _ = job.mark_started();

        for i in 0..100 {
            let has_error = i % 10 == 0;
            let _ = job.record_result(false, false, has_error);
        }

        let _ = job.mark_completed();
        assert_eq!(job.status(), BatchJobStatus::CompletedWithErrors);
        assert_eq!(job.error_count, 10);
    }

    #[test]
    fn test_whitelist_entry_creation() {
        let entry = SanctionsWhitelistEntry::new(
            "John Doe".to_string(),
            "passport".to_string(),
            "TN123456789".to_string(),
            "False positive - common name".to_string(),
            90,
        );
        assert!(entry.is_ok());
        let e = entry.unwrap();
        assert!(!e.is_valid()); // Not approved yet
    }

    #[test]
    fn test_whitelist_entry_workflow() {
        let mut entry = SanctionsWhitelistEntry::new(
            "John Smith".to_string(),
            "passport".to_string(),
            "TN999888777".to_string(),
            "Confirmed false positive".to_string(),
            365,
        )
        .unwrap();

        assert_eq!(entry.approval_status(), WhitelistApprovalStatus::Pending);

        let _ = entry.approve(Uuid::new_v4());
        assert_eq!(entry.approval_status(), WhitelistApprovalStatus::Approved);
        assert!(entry.is_valid());
    }

    #[test]
    fn test_escalation_rule_creation() {
        let rule = EscalationRule::new(
            EscalationTrigger::ConfirmedHit,
            85,
        );
        assert!(rule.is_ok());
        let r = rule.unwrap();
        assert!(r.is_active());
    }

    #[test]
    fn test_escalation_rule_high_risk() {
        let rule = EscalationRule::new(
            EscalationTrigger::HighRiskList,
            80,
        )
        .unwrap();

        assert_eq!(rule.priority, "Critical");
        assert!(rule.should_escalate(95)); // Confidence > threshold
        assert!(!rule.should_escalate(70)); // Confidence < threshold
    }

    #[test]
    fn test_sanctions_report() {
        let mut report = SanctionsReport::new(
            ReportPeriod::Monthly,
            Utc::now() - chrono::Duration::days(30),
            Utc::now(),
        )
        .unwrap();

        report.record_screening(true, false, Some("UN".to_string()), false);
        report.record_screening(false, true, None, false);
        report.record_screening(true, false, Some("OFAC".to_string()), true);

        assert_eq!(report.total_screenings(), 3);
        assert_eq!(report.confirmed_hits(), 2);
        assert_eq!(report.hit_rate(), (2.0_f64 / 3.0_f64) * 100.0);
    }

    #[test]
    fn test_sanctions_report_metrics() {
        let mut report = SanctionsReport::new(
            ReportPeriod::Quarterly,
            Utc::now() - chrono::Duration::days(90),
            Utc::now(),
        )
        .unwrap();

        for _ in 0..100 {
            report.record_screening(false, false, None, false);
        }

        for _ in 0..10 {
            report.record_screening(true, false, Some("UN".to_string()), false);
        }

        report.record_false_positive();

        assert_eq!(report.total_screenings(), 110);
        assert_eq!(report.confirmed_hits(), 10);
        assert!(report.hit_rate() >= 9.0 && report.hit_rate() <= 10.0);
    }

    #[test]
    fn test_batch_job_cancellation() {
        let mut job = BatchScreeningJob::new(
            BatchJobType::PeriodicCompliance,
            200,
            Uuid::new_v4(),
        )
        .unwrap();

        let _ = job.mark_started();
        let _ = job.mark_cancelled();
        assert_eq!(job.status(), BatchJobStatus::Cancelled);
    }

    #[test]
    fn test_batch_job_invalid_total() {
        let invalid = BatchScreeningJob::new(
            BatchJobType::ScreenAllCustomers,
            0,
            Uuid::new_v4(),
        );
        assert!(invalid.is_err());
    }

    #[test]
    fn test_whitelist_rejection() {
        let mut entry = SanctionsWhitelistEntry::new(
            "Test Entity".to_string(),
            "national_id".to_string(),
            "ID123456".to_string(),
            "Request rejected".to_string(),
            30,
        )
        .unwrap();

        let _ = entry.reject();
        assert_eq!(
            entry.approval_status(),
            WhitelistApprovalStatus::Rejected
        );
    }

    #[test]
    fn test_escalation_rule_update_threshold() {
        let mut rule = EscalationRule::new(
            EscalationTrigger::MultipleMatches,
            75,
        )
        .unwrap();

        let _ = rule.update_threshold(90);
        assert_eq!(rule.similarity_threshold, 90);
        assert!(!rule.should_escalate(85)); // Now below new threshold
    }
}
