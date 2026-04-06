use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use banko_domain::governance::{
    AuditEntryId, AuditTrailEntry, Committee, CommitteeDecision, CommitteeMeeting, ControlCheck,
    ControlCheckSignOff, ControlStatus,
};

use super::dto::AuditFilter;

// --- Audit Repository ---

#[async_trait]
pub trait IAuditRepository: Send + Sync {
    async fn append(&self, entry: &AuditTrailEntry) -> Result<(), String>;
    async fn find_by_id(&self, id: &AuditEntryId) -> Result<Option<AuditTrailEntry>, String>;
    async fn find_latest(&self) -> Result<Option<AuditTrailEntry>, String>;
    async fn find_all(
        &self,
        filters: &AuditFilter,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditTrailEntry>, String>;
    async fn count_all(&self, filters: &AuditFilter) -> Result<i64, String>;
    async fn find_chain(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<AuditTrailEntry>, String>;

    // --- Dashboard methods (AUD-02) ---
    async fn count_by_date_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<i64, String>;
    async fn count_by_action(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<(String, i64)>, String>;
    async fn count_by_actor(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<(Uuid, i64)>, String>;
    async fn count_per_day(&self, days: u32) -> Result<Vec<(NaiveDate, i64)>, String>;
    async fn find_suspicious(
        &self,
        from: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<AuditTrailEntry>, String>;
}

// --- Committee Repository ---

#[async_trait]
pub trait ICommitteeRepository: Send + Sync {
    async fn save_committee(&self, committee: &Committee) -> Result<(), String>;
    async fn find_committee_by_id(&self, id: Uuid) -> Result<Option<Committee>, String>;
    async fn find_all_committees(&self) -> Result<Vec<Committee>, String>;
    async fn save_decision(&self, decision: &CommitteeDecision) -> Result<(), String>;
    async fn find_decisions_by_committee(
        &self,
        committee_id: Uuid,
    ) -> Result<Vec<CommitteeDecision>, String>;
    async fn save_meeting(&self, meeting: &CommitteeMeeting) -> Result<(), String>;
    async fn find_meeting_by_id(&self, id: Uuid) -> Result<Option<CommitteeMeeting>, String>;
}

// --- Control Check Repository ---

#[async_trait]
pub trait IControlCheckRepository: Send + Sync {
    async fn save(&self, check: &ControlCheck) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ControlCheck>, String>;
    async fn find_all(
        &self,
        status: Option<ControlStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ControlCheck>, String>;
    async fn count_all(&self, status: Option<ControlStatus>) -> Result<i64, String>;
    async fn save_signoff(&self, signoff: &ControlCheckSignOff) -> Result<(), String>;
    async fn find_signoff_by_id(&self, id: Uuid) -> Result<Option<ControlCheckSignOff>, String>;
    async fn find_pending_signoffs(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ControlCheckSignOff>, String>;
    async fn count_pending_signoffs(&self) -> Result<i64, String>;
}
