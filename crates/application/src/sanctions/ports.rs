use async_trait::async_trait;

use banko_domain::sanctions::{
    ListSource, MatchDetail, SanctionEntry, SanctionEntryId, SanctionList, SanctionListId,
    ScreeningResult, ScreeningResultId, ScreeningStatus,
};

// --- Sanction List Repository ---

#[async_trait]
pub trait ISanctionListRepository: Send + Sync {
    async fn save(&self, list: &SanctionList) -> Result<(), String>;
    async fn find_by_id(&self, id: &SanctionListId) -> Result<Option<SanctionList>, String>;
    async fn find_by_source(&self, source: ListSource) -> Result<Option<SanctionList>, String>;
    async fn find_all(&self) -> Result<Vec<SanctionList>, String>;
}

// --- Sanction Entry Repository ---

#[async_trait]
pub trait ISanctionEntryRepository: Send + Sync {
    async fn save_entries(&self, entries: &[SanctionEntry]) -> Result<(), String>;
    async fn find_by_id(&self, id: &SanctionEntryId) -> Result<Option<SanctionEntry>, String>;
    async fn find_all_active(&self) -> Result<Vec<SanctionEntry>, String>;
    async fn find_by_source(&self, source: ListSource) -> Result<Vec<SanctionEntry>, String>;
    async fn count_active(&self) -> Result<i64, String>;
}

// --- Screening Result Repository ---

#[async_trait]
pub trait IScreeningResultRepository: Send + Sync {
    async fn save(&self, result: &ScreeningResult) -> Result<(), String>;
    async fn find_by_id(&self, id: &ScreeningResultId) -> Result<Option<ScreeningResult>, String>;
    async fn find_recent(&self, limit: i64, offset: i64) -> Result<Vec<ScreeningResult>, String>;
    async fn count_by_status(&self, status: Option<ScreeningStatus>) -> Result<i64, String>;
}

// --- Matching Strategy (pluggable) ---

pub trait IMatchingStrategy: Send + Sync {
    fn screen(&self, name: &str, entries: &[SanctionEntry], threshold: u8) -> Vec<MatchDetail>;
}

// ============================================================================
// BMAD FR-058: Batch Screening Job Repository
// ============================================================================

use banko_domain::sanctions::{BatchScreeningJob, BatchScreeningJobId, BatchJobStatus};

#[async_trait]
pub trait IBatchScreeningJobRepository: Send + Sync {
    async fn save(&self, job: &BatchScreeningJob) -> Result<(), String>;
    async fn find_by_id(&self, id: &BatchScreeningJobId) -> Result<Option<BatchScreeningJob>, String>;
    async fn find_by_status(&self, status: BatchJobStatus) -> Result<Vec<BatchScreeningJob>, String>;
}

// ============================================================================
// BMAD FR-057: Sanctions Whitelist Repository (False Positive Management)
// ============================================================================

use banko_domain::sanctions::{
    SanctionsWhitelistEntry, SanctionsWhitelistEntryId, WhitelistApprovalStatus,
};

#[async_trait]
pub trait ISanctionsWhitelistRepository: Send + Sync {
    async fn save(&self, entry: &SanctionsWhitelistEntry) -> Result<(), String>;
    async fn find_by_id(
        &self,
        id: &SanctionsWhitelistEntryId,
    ) -> Result<Option<SanctionsWhitelistEntry>, String>;
    async fn find_by_document_number(
        &self,
        document_number: &str,
    ) -> Result<Option<SanctionsWhitelistEntry>, String>;
    async fn find_by_status(
        &self,
        status: WhitelistApprovalStatus,
    ) -> Result<Vec<SanctionsWhitelistEntry>, String>;
    /// Mark all expired entries as Expired status
    async fn mark_expired_entries(&self) -> Result<i64, String>;
}

// ============================================================================
// BMAD FR-060: Escalation Rule Repository
// ============================================================================

use banko_domain::sanctions::{EscalationRule, EscalationRuleId, EscalationTrigger};

#[async_trait]
pub trait IEscalationRuleRepository: Send + Sync {
    async fn save(&self, rule: &EscalationRule) -> Result<(), String>;
    async fn find_by_id(&self, id: &EscalationRuleId) -> Result<Option<EscalationRule>, String>;
    async fn find_by_trigger(&self, trigger: EscalationTrigger) -> Result<Vec<EscalationRule>, String>;
    async fn find_active(&self) -> Result<Vec<EscalationRule>, String>;
}

// ============================================================================
// BMAD FR-062: Sanctions Report Repository
// ============================================================================

use banko_domain::sanctions::{SanctionsReport, SanctionsReportId};

#[async_trait]
pub trait ISanctionsReportRepository: Send + Sync {
    async fn save(&self, report: &SanctionsReport) -> Result<(), String>;
    async fn find_by_id(&self, id: &SanctionsReportId) -> Result<Option<SanctionsReport>, String>;
    async fn find_by_period_month(
        &self,
        year: i32,
        month: u32,
    ) -> Result<Option<SanctionsReport>, String>;
    async fn find_by_period_quarter(
        &self,
        year: i32,
        quarter: u8,
    ) -> Result<Option<SanctionsReport>, String>;
}
