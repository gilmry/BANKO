use async_trait::async_trait;
use uuid::Uuid;

use banko_domain::governance::{
    ApprovalWorkflow, ApprovalWorkflowId, PowerDelegation, PowerDelegationId, AccessReview,
    AccessReviewId,
};

// ============================================================
// Approval Workflow Repository Port
// ============================================================

#[async_trait]
pub trait IApprovalWorkflowRepository: Send + Sync {
    /// Save or update an approval workflow
    async fn save(&self, workflow: &ApprovalWorkflow) -> Result<(), String>;

    /// Find workflow by ID
    async fn find_by_id(&self, id: &ApprovalWorkflowId) -> Result<Option<ApprovalWorkflow>, String>;

    /// Find workflow by operation ID
    async fn find_by_operation_id(&self, operation_id: Uuid) -> Result<Option<ApprovalWorkflow>, String>;

    /// Find pending workflows (awaiting approvals)
    async fn find_pending(&self, limit: i64, offset: i64) -> Result<Vec<ApprovalWorkflow>, String>;

    /// Count pending workflows
    async fn count_pending(&self) -> Result<i64, String>;

    /// Find workflows by requester
    async fn find_by_requester(
        &self,
        requester_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ApprovalWorkflow>, String>;

    /// Find workflows requiring action from a specific approver
    async fn find_awaiting_approval_from(
        &self,
        approver_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ApprovalWorkflow>, String>;

    /// Count workflows awaiting approval from a user
    async fn count_awaiting_approval_from(&self, approver_id: Uuid) -> Result<i64, String>;

    /// Find workflows by status
    async fn find_by_status(
        &self,
        status: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ApprovalWorkflow>, String>;

    /// Get audit trail of a workflow (decisions, timestamps, approvers)
    async fn get_workflow_audit(
        &self,
        workflow_id: &ApprovalWorkflowId,
    ) -> Result<Vec<WorkflowAuditEntry>, String>;

    /// Delete workflow (archive)
    async fn delete(&self, id: &ApprovalWorkflowId) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct WorkflowAuditEntry {
    pub workflow_id: ApprovalWorkflowId,
    pub approver_id: Uuid,
    pub decision: String,  // "Approved", "Rejected", "Abstained"
    pub comments: Option<String>,
    pub decided_at: chrono::DateTime<chrono::Utc>,
}

// ============================================================
// Power Delegation Repository Port
// ============================================================

#[async_trait]
pub trait IPowerDelegationRepository: Send + Sync {
    /// Save or update a delegation
    async fn save(&self, delegation: &PowerDelegation) -> Result<(), String>;

    /// Find delegation by ID
    async fn find_by_id(&self, id: &PowerDelegationId) -> Result<Option<PowerDelegation>, String>;

    /// Find active delegations for a user (has delegated to them)
    async fn find_active_for_user(&self, user_id: Uuid) -> Result<Vec<PowerDelegation>, String>;

    /// Find delegations from a user (has delegated from them)
    async fn find_delegated_from(&self, user_id: Uuid) -> Result<Vec<PowerDelegation>, String>;

    /// Find delegations by status
    async fn find_by_status(
        &self,
        status: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PowerDelegation>, String>;

    /// Check if user has active delegation for a scope
    async fn has_active_delegation(
        &self,
        user_id: Uuid,
        scope: &str,
    ) -> Result<bool, String>;

    /// Find all active delegations (for cleanup/expiry checks)
    async fn find_all_active(&self) -> Result<Vec<PowerDelegation>, String>;

    /// Count pending delegations (awaiting activation)
    async fn count_pending(&self) -> Result<i64, String>;

    /// Delete delegation (archive)
    async fn delete(&self, id: &PowerDelegationId) -> Result<(), String>;

    /// Get delegation history for a user
    async fn get_delegation_history(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<DelegationHistoryRecord>, String>;
}

#[derive(Debug, Clone)]
pub struct DelegationHistoryRecord {
    pub delegation_id: PowerDelegationId,
    pub delegated_from: Uuid,
    pub delegated_to: Uuid,
    pub scope: String,
    pub status: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

// ============================================================
// Access Review Repository Port
// ============================================================

#[async_trait]
pub trait IAccessReviewRepository: Send + Sync {
    /// Save or update an access review
    async fn save(&self, review: &AccessReview) -> Result<(), String>;

    /// Find review by ID
    async fn find_by_id(&self, id: &AccessReviewId) -> Result<Option<AccessReview>, String>;

    /// Find scheduled reviews
    async fn find_scheduled(&self, limit: i64, offset: i64) -> Result<Vec<AccessReview>, String>;

    /// Count scheduled reviews
    async fn count_scheduled(&self) -> Result<i64, String>;

    /// Find reviews by status
    async fn find_by_status(
        &self,
        status: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AccessReview>, String>;

    /// Find in-progress reviews
    async fn find_in_progress(&self) -> Result<Vec<AccessReview>, String>;

    /// Find completed reviews
    async fn find_completed(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AccessReview>, String>;

    /// Find reviews by scope (e.g., "Department:Finance")
    async fn find_by_scope(&self, scope: &str) -> Result<Vec<AccessReview>, String>;

    /// Find reviews conducted by a user
    async fn find_by_conductor(
        &self,
        conductor_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AccessReview>, String>;

    /// Get all critical findings across reviews
    async fn get_all_critical_findings(&self) -> Result<Vec<CriticalFindingReport>, String>;

    /// Get findings for a specific user
    async fn get_user_findings(&self, user_id: Uuid) -> Result<Vec<UserFindingRecord>, String>;

    /// Delete review (archive)
    async fn delete(&self, id: &AccessReviewId) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct CriticalFindingReport {
    pub review_id: AccessReviewId,
    pub user_id: Uuid,
    pub findings: String,
    pub recommended_action: String,
    pub severity: String,
    pub identified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct UserFindingRecord {
    pub review_id: AccessReviewId,
    pub findings: String,
    pub recommended_action: String,
    pub severity: String,
}
