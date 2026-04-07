use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================
// Approval Workflow DTOs (FR-150)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApprovalWorkflowRequest {
    pub operation_id: String,
    pub operation_type: String,  // e.g., "GrantRole", "RevokePermission"
    pub approval_type: String,   // "TwoEyes", "FourEyes", "SixEyes"
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecisionRequest {
    pub workflow_id: String,
    pub decision: String,  // "Approved", "Rejected", "Abstained"
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub approver_id: String,
    pub decision: String,
    pub comments: Option<String>,
    pub approved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflowResponse {
    pub id: String,
    pub operation_id: String,
    pub operation_type: String,
    pub requested_by: String,
    pub approval_type: String,
    pub status: String,  // "Pending", "InProgress", "Approved", "Rejected", "Cancelled"
    pub approvals: Vec<ApprovalResponse>,
    pub approvals_received: usize,
    pub approvals_required: usize,
    pub approval_percentage: u32,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_expired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApprovalResponse {
    pub workflow_id: String,
    pub operation_type: String,
    pub requested_by: String,
    pub approval_type: String,
    pub approvals_received: usize,
    pub approvals_required: usize,
    pub created_at: DateTime<Utc>,
    pub days_pending: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDashboardResponse {
    pub pending_count: i64,
    pub pending_workflows: Vec<PendingApprovalResponse>,
    pub awaiting_my_approval_count: i64,
    pub awaiting_my_approval: Vec<PendingApprovalResponse>,
    pub completed_today: i64,
    pub approval_backlog: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalAuditResponse {
    pub workflow_id: String,
    pub operation_type: String,
    pub approvers: Vec<ApprovalAuditEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalAuditEntry {
    pub approver_id: String,
    pub decision: String,
    pub comments: Option<String>,
    pub decided_at: DateTime<Utc>,
}

// ============================================================
// Power Delegation DTOs (FR-151)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDelegationRequest {
    pub delegated_to: String,           // User UUID to delegate to
    pub scope: String,                  // e.g., "Approver:ALL", "Approver:LOANS"
    pub reason: Option<String>,
    pub duration_days: i64,             // 1-365 days
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeDelegationRequest {
    pub delegation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDelegationResponse {
    pub id: String,
    pub delegated_from: String,
    pub delegated_to: String,
    pub scope: String,
    pub status: String,  // "Active", "Pending", "Revoked", "Expired"
    pub reason: Option<String>,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub is_valid: bool,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDelegationsResponse {
    pub user_id: String,
    pub delegations_received: Vec<PowerDelegationResponse>,
    pub delegations_given: Vec<PowerDelegationResponse>,
    pub total_active: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationHistoryResponse {
    pub user_id: String,
    pub history: Vec<DelegationHistoryEntry>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationHistoryEntry {
    pub delegation_id: String,
    pub delegated_from: String,
    pub delegated_to: String,
    pub scope: String,
    pub status: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

// ============================================================
// Access Review DTOs (FR-152)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAccessReviewRequest {
    pub scope: String,               // e.g., "All users", "Department:Finance"
    pub scheduled_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewFindingRequest {
    pub user_id: String,
    pub findings: String,
    pub recommended_action: String,
    pub severity: String,  // "Info", "Warning", "Critical"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewResponse {
    pub id: String,
    pub scope: String,
    pub status: String,  // "Scheduled", "InProgress", "Completed", "Cancelled"
    pub scheduled_date: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub conducted_by: String,
    pub findings_count: usize,
    pub critical_findings: usize,
    pub findings: Vec<AccessReviewFindingResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewFindingResponse {
    pub user_id: String,
    pub findings: String,
    pub recommended_action: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReviewsResponse {
    pub total_scheduled: i64,
    pub reviews: Vec<ScheduledReviewInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReviewInfo {
    pub review_id: String,
    pub scope: String,
    pub scheduled_date: DateTime<Utc>,
    pub days_until_review: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewDashboardResponse {
    pub total_reviews: i64,
    pub completed_reviews: i64,
    pub scheduled_reviews: i64,
    pub in_progress_reviews: i64,
    pub total_findings: i64,
    pub critical_findings: i64,
    pub recent_reviews: Vec<AccessReviewResponse>,
    pub pending_reviews: Vec<ScheduledReviewInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessReviewResponse {
    pub user_id: String,
    pub reviews_involving_user: Vec<UserReviewEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReviewEntry {
    pub review_id: String,
    pub review_scope: String,
    pub findings: Vec<String>,
    pub severity_levels: Vec<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewReportResponse {
    pub report_period: String,
    pub total_reviews_conducted: i64,
    pub total_users_reviewed: i64,
    pub findings_by_severity: FindingsBySeverity,
    pub top_findings: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingsBySeverity {
    pub info: i64,
    pub warning: i64,
    pub critical: i64,
}

// ============================================================
// Compliance Reporting DTOs
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceComplianceResponse {
    pub report_date: DateTime<Utc>,
    pub rbac_compliance: RbacComplianceStatus,
    pub approval_workflow_status: ApprovalWorkflowStatus,
    pub delegation_status: DelegationComplianceStatus,
    pub access_review_status: AccessReviewComplianceStatus,
    pub overall_compliance_score: f64,  // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacComplianceStatus {
    pub sod_violations: i64,
    pub compliant_users: i64,
    pub users_requiring_review: i64,
    pub status: String,  // "COMPLIANT", "AT_RISK", "NON_COMPLIANT"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflowStatus {
    pub pending_workflows: i64,
    pub overdue_workflows: i64,
    pub average_resolution_time_hours: f64,
    pub approval_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationComplianceStatus {
    pub active_delegations: i64,
    pub pending_delegations: i64,
    pub expired_delegations: i64,
    pub max_delegation_duration_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewComplianceStatus {
    pub reviews_scheduled: i64,
    pub reviews_completed: i64,
    pub days_since_last_review: i64,
    pub critical_findings_unresolved: i64,
    pub status: String,  // "ON_SCHEDULE", "OVERDUE", "IN_PROGRESS"
}
