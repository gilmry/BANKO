use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================
// Audit DTOs
// ============================================================

#[derive(Debug, Default, Deserialize)]
pub struct AuditFilter {
    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AuditEntryResponse {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub changes: Option<String>,
    pub ip_address: Option<String>,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Debug, Serialize)]
pub struct AuditListResponse {
    pub data: Vec<AuditEntryResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct IntegrityCheckResponse {
    pub valid: bool,
    pub entries_checked: usize,
    pub error: Option<String>,
    pub checked_from: DateTime<Utc>,
    pub checked_to: DateTime<Utc>,
}

// ============================================================
// Committee DTOs
// ============================================================

#[derive(Debug, Serialize)]
pub struct CommitteeResponse {
    pub id: String,
    pub name: String,
    pub committee_type: String,
    pub members: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CommitteeDecisionResponse {
    pub id: String,
    pub committee_id: String,
    pub subject: String,
    pub decision: String,
    pub votes: Vec<VoteResponse>,
    pub justification: Option<String>,
    pub decided_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VoteResponse {
    pub member_id: String,
    pub vote: String,
}

// ============================================================
// Control Check DTOs
// ============================================================

#[derive(Debug, Serialize)]
pub struct ControlCheckResponse {
    pub id: String,
    pub operation_type: String,
    pub operation_id: String,
    pub checker_id: Option<String>,
    pub status: String,
    pub comments: Option<String>,
    pub checked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ControlCheckListResponse {
    pub data: Vec<ControlCheckResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// ============================================================
// BCT Audit DTOs (AUD-01)
// ============================================================

#[derive(Debug, Default, Deserialize)]
pub struct BctAuditFilter {
    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BctAuditResponse {
    pub data: Vec<AuditEntryResponse>,
    pub total: i64,
    pub page: i64,
    pub total_pages: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct AuditExportResponse {
    pub format: String,
    pub data: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct BctIntegrityReport {
    pub valid: bool,
    pub entries_checked: usize,
    pub first_entry_id: Option<String>,
    pub last_entry_id: Option<String>,
    pub errors: Vec<String>,
    pub checked_from: DateTime<Utc>,
    pub checked_to: DateTime<Utc>,
}

// ============================================================
// Dashboard DTOs (AUD-02)
// ============================================================

#[derive(Debug, Serialize)]
pub struct DashboardStatsResponse {
    pub total_entries: i64,
    pub entries_today: i64,
    pub entries_this_week: i64,
    pub top_actors: Vec<ActorCount>,
    pub actions_breakdown: Vec<ActionCount>,
}

#[derive(Debug, Serialize)]
pub struct ActorCount {
    pub user_id: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ActionCount {
    pub action: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct DailyCountResponse {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct SuspiciousActivityResponse {
    pub entry_id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub details: Option<String>,
}

// ============================================================
// Compliance Report DTO (GOV-06)
// ============================================================

#[derive(Debug, Serialize)]
pub struct ComplianceReportResponse {
    pub generated_at: DateTime<Utc>,
    pub first_line: FirstLineDefense,
    pub second_line: SecondLineDefense,
    pub third_line: ThirdLineDefense,
}

#[derive(Debug, Serialize)]
pub struct FirstLineDefense {
    pub description: String,
    pub total_controls: i64,
    pub approved: i64,
    pub rejected: i64,
    pub pending: i64,
}

#[derive(Debug, Serialize)]
pub struct SecondLineDefense {
    pub description: String,
    pub total_audit_entries: i64,
    pub integrity_valid: bool,
}

#[derive(Debug, Serialize)]
pub struct ThirdLineDefense {
    pub description: String,
    pub total_committees: usize,
    pub total_decisions: usize,
}

// ============================================================
// Committee Meeting DTOs (GOV-07 extended)
// ============================================================

#[derive(Debug, Serialize)]
pub struct CommitteeMeetingResponse {
    pub id: String,
    pub committee_id: String,
    pub scheduled_date: DateTime<Utc>,
    pub attendees: Vec<String>,
    pub agenda: Vec<String>,
    pub decisions: Vec<String>,
    pub status: String,
    pub minutes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ============================================================
// Control Check Sign-Off DTOs (GOV-08 extended)
// ============================================================

#[derive(Debug, Serialize)]
pub struct ControlCheckSignOffResponse {
    pub id: String,
    pub control_check_id: String,
    pub control_ref: String,
    pub checker_id: String,
    pub check_date: DateTime<Utc>,
    pub result: String,
    pub findings: Option<String>,
    pub signed_off_by: Option<String>,
    pub signed_off_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ControlCheckSignOffListResponse {
    pub data: Vec<ControlCheckSignOffResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
