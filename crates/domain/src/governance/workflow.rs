use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Newtypes for Workflow Identifiers
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApprovalWorkflowId(Uuid);

impl ApprovalWorkflowId {
    pub fn new() -> Self {
        ApprovalWorkflowId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ApprovalWorkflowId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ApprovalWorkflowId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ApprovalWorkflowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PowerDelegationId(Uuid);

impl PowerDelegationId {
    pub fn new() -> Self {
        PowerDelegationId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        PowerDelegationId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PowerDelegationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PowerDelegationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccessReviewId(Uuid);

impl AccessReviewId {
    pub fn new() -> Self {
        AccessReviewId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        AccessReviewId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AccessReviewId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AccessReviewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// Approval Workflow — multi-level approvals (FR-150)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalType {
    TwoEyes,   // Requires 2 approvers (FR-150)
    FourEyes,  // Requires 4 approvers
    SixEyes,   // Requires 6 approvers
}

impl ApprovalType {
    pub fn required_approvers(&self) -> usize {
        match self {
            ApprovalType::TwoEyes => 2,
            ApprovalType::FourEyes => 4,
            ApprovalType::SixEyes => 6,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ApprovalType::TwoEyes => "TwoEyes",
            ApprovalType::FourEyes => "FourEyes",
            ApprovalType::SixEyes => "SixEyes",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "TwoEyes" => Ok(ApprovalType::TwoEyes),
            "FourEyes" => Ok(ApprovalType::FourEyes),
            "SixEyes" => Ok(ApprovalType::SixEyes),
            _ => Err(DomainError::InvalidInput(format!("Unknown approval type: {s}"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    InProgress,
    Approved,
    Rejected,
    Cancelled,
}

impl ApprovalStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ApprovalStatus::Pending => "Pending",
            ApprovalStatus::InProgress => "InProgress",
            ApprovalStatus::Approved => "Approved",
            ApprovalStatus::Rejected => "Rejected",
            ApprovalStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Pending" => Ok(ApprovalStatus::Pending),
            "InProgress" => Ok(ApprovalStatus::InProgress),
            "Approved" => Ok(ApprovalStatus::Approved),
            "Rejected" => Ok(ApprovalStatus::Rejected),
            "Cancelled" => Ok(ApprovalStatus::Cancelled),
            _ => Err(DomainError::InvalidInput(format!("Unknown approval status: {s}"))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approver_id: Uuid,
    pub decision: ApprovalDecision,
    pub comments: Option<String>,
    pub approved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    Abstained,
}

impl ApprovalDecision {
    pub fn as_str(&self) -> &str {
        match self {
            ApprovalDecision::Approved => "Approved",
            ApprovalDecision::Rejected => "Rejected",
            ApprovalDecision::Abstained => "Abstained",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    id: ApprovalWorkflowId,
    operation_id: Uuid,
    operation_type: String,     // e.g., "GrantRole", "RevokePermission"
    requested_by: Uuid,
    approval_type: ApprovalType,
    status: ApprovalStatus,
    approvals: Vec<Approval>,
    reason: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
}

impl ApprovalWorkflow {
    pub fn new(
        operation_id: Uuid,
        operation_type: String,
        requested_by: Uuid,
        approval_type: ApprovalType,
        reason: Option<String>,
    ) -> Result<Self, DomainError> {
        if operation_type.is_empty() {
            return Err(DomainError::InvalidInput(
                "Operation type cannot be empty".into(),
            ));
        }

        Ok(ApprovalWorkflow {
            id: ApprovalWorkflowId::new(),
            operation_id,
            operation_type,
            requested_by,
            approval_type,
            status: ApprovalStatus::Pending,
            approvals: Vec::new(),
            reason,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::days(30)), // Default 30-day expiry
        })
    }

    // Accessors
    pub fn id(&self) -> &ApprovalWorkflowId {
        &self.id
    }

    pub fn operation_id(&self) -> Uuid {
        self.operation_id
    }

    pub fn operation_type(&self) -> &str {
        &self.operation_type
    }

    pub fn requested_by(&self) -> Uuid {
        self.requested_by
    }

    pub fn approval_type(&self) -> ApprovalType {
        self.approval_type
    }

    pub fn status(&self) -> ApprovalStatus {
        self.status
    }

    pub fn approvals(&self) -> &[Approval] {
        &self.approvals
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }

    /// Submit an approval decision
    pub fn submit_approval(
        &mut self,
        approver_id: Uuid,
        decision: ApprovalDecision,
        comments: Option<String>,
    ) -> Result<(), DomainError> {
        if self.status == ApprovalStatus::Cancelled {
            return Err(DomainError::InvalidInput(
                "Cannot approve cancelled workflow".into(),
            ));
        }

        // Check if approver already approved
        if self.approvals.iter().any(|a| a.approver_id == approver_id) {
            return Err(DomainError::InvalidInput(
                "Approver has already submitted decision".into(),
            ));
        }

        self.approvals.push(Approval {
            approver_id,
            decision,
            comments,
            approved_at: Utc::now(),
        });

        self.status = ApprovalStatus::InProgress;
        self.updated_at = Utc::now();

        // Auto-finalize if all approvers have submitted
        self.auto_finalize();

        Ok(())
    }

    /// Auto-finalize workflow based on decisions
    fn auto_finalize(&mut self) {
        let required = self.approval_type.required_approvers();
        if self.approvals.len() < required {
            return;
        }

        let approved_count = self
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count();

        let rejected_count = self
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Rejected)
            .count();

        // If unanimous rejection, reject immediately
        if rejected_count > 0 && approved_count == 0 {
            self.status = ApprovalStatus::Rejected;
        } else if approved_count >= required {
            // If majority approved (2/3 or more)
            let approval_threshold = (required * 2) / 3 + 1;
            if approved_count >= approval_threshold {
                self.status = ApprovalStatus::Approved;
            }
        }
    }

    /// Cancel the workflow
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if matches!(
            self.status,
            ApprovalStatus::Approved | ApprovalStatus::Rejected
        ) {
            return Err(DomainError::InvalidInput(
                "Cannot cancel finalized workflow".into(),
            ));
        }
        self.status = ApprovalStatus::Cancelled;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if workflow is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            Utc::now() > expires
        } else {
            false
        }
    }

    /// Get approval percentage
    pub fn approval_percentage(&self) -> u32 {
        if self.approvals.is_empty() {
            return 0;
        }
        let approved = self
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count() as u32;
        ((approved * 100) / self.approvals.len() as u32).min(100)
    }
}

// ============================================================
// Power Delegation — temporary privilege delegation (FR-151)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelegationStatus {
    Active,
    Pending,
    Revoked,
    Expired,
}

impl DelegationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            DelegationStatus::Active => "Active",
            DelegationStatus::Pending => "Pending",
            DelegationStatus::Revoked => "Revoked",
            DelegationStatus::Expired => "Expired",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDelegation {
    id: PowerDelegationId,
    delegated_from: Uuid,      // Original role holder
    delegated_to: Uuid,         // Temporary delegate
    scope: String,              // What permissions are delegated (e.g., "Approver:ALL")
    reason: Option<String>,
    status: DelegationStatus,
    valid_from: DateTime<Utc>,
    valid_until: DateTime<Utc>,
    created_at: DateTime<Utc>,
    revoked_at: Option<DateTime<Utc>>,
    revoked_by: Option<Uuid>,
}

impl PowerDelegation {
    pub fn new(
        delegated_from: Uuid,
        delegated_to: Uuid,
        scope: String,
        reason: Option<String>,
        duration_days: i64,
    ) -> Result<Self, DomainError> {
        if duration_days <= 0 || duration_days > 365 {
            return Err(DomainError::InvalidInput(
                "Duration must be between 1 and 365 days".into(),
            ));
        }

        if scope.is_empty() {
            return Err(DomainError::InvalidInput("Delegation scope cannot be empty".into()));
        }

        let now = Utc::now();
        let valid_until = now + Duration::days(duration_days);

        Ok(PowerDelegation {
            id: PowerDelegationId::new(),
            delegated_from,
            delegated_to,
            scope,
            reason,
            status: DelegationStatus::Pending,
            valid_from: now,
            valid_until,
            created_at: now,
            revoked_at: None,
            revoked_by: None,
        })
    }

    // Accessors
    pub fn id(&self) -> &PowerDelegationId {
        &self.id
    }

    pub fn delegated_from(&self) -> Uuid {
        self.delegated_from
    }

    pub fn delegated_to(&self) -> Uuid {
        self.delegated_to
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn status(&self) -> DelegationStatus {
        self.status
    }

    pub fn valid_from(&self) -> &DateTime<Utc> {
        &self.valid_from
    }

    pub fn valid_until(&self) -> &DateTime<Utc> {
        &self.valid_until
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn revoked_at(&self) -> Option<&DateTime<Utc>> {
        self.revoked_at.as_ref()
    }

    pub fn revoked_by(&self) -> Option<Uuid> {
        self.revoked_by
    }

    /// Activate the delegation
    pub fn activate(&mut self) -> Result<(), DomainError> {
        if self.status != DelegationStatus::Pending {
            return Err(DomainError::InvalidInput(
                "Only pending delegations can be activated".into(),
            ));
        }
        self.status = DelegationStatus::Active;
        Ok(())
    }

    /// Revoke the delegation early
    pub fn revoke(&mut self, revoked_by: Uuid) -> Result<(), DomainError> {
        if self.status == DelegationStatus::Revoked {
            return Err(DomainError::InvalidInput("Already revoked".into()));
        }
        self.status = DelegationStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.revoked_by = Some(revoked_by);
        Ok(())
    }

    /// Check if delegation is currently valid
    pub fn is_valid(&self) -> bool {
        match self.status {
            DelegationStatus::Active => {
                let now = Utc::now();
                now >= self.valid_from && now <= self.valid_until
            }
            DelegationStatus::Pending => {
                let now = Utc::now();
                now >= self.valid_from && now <= self.valid_until
            }
            _ => false,
        }
    }

    /// Refresh status (mark as expired if past expiry)
    pub fn refresh_status(&mut self) {
        if Utc::now() > self.valid_until && self.status != DelegationStatus::Revoked {
            self.status = DelegationStatus::Expired;
        }
    }
}

// ============================================================
// Access Review — periodic access review (FR-152)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessReviewStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

impl AccessReviewStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AccessReviewStatus::Scheduled => "Scheduled",
            AccessReviewStatus::InProgress => "InProgress",
            AccessReviewStatus::Completed => "Completed",
            AccessReviewStatus::Cancelled => "Cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReviewFinding {
    pub user_id: Uuid,
    pub findings: String,  // e.g., "Excessive permissions", "SoD violation"
    pub recommended_action: String,
    pub severity: AccessReviewSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessReviewSeverity {
    Info,
    Warning,
    Critical,
}

impl AccessReviewSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            AccessReviewSeverity::Info => "Info",
            AccessReviewSeverity::Warning => "Warning",
            AccessReviewSeverity::Critical => "Critical",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReview {
    id: AccessReviewId,
    scope: String,              // e.g., "All users", "Department:Finance"
    status: AccessReviewStatus,
    scheduled_date: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    conducted_by: Uuid,
    findings: Vec<AccessReviewFinding>,
    created_at: DateTime<Utc>,
}

impl AccessReview {
    pub fn new(
        scope: String,
        scheduled_date: DateTime<Utc>,
        conducted_by: Uuid,
    ) -> Result<Self, DomainError> {
        if scope.is_empty() {
            return Err(DomainError::InvalidInput("Scope cannot be empty".into()));
        }

        Ok(AccessReview {
            id: AccessReviewId::new(),
            scope,
            status: AccessReviewStatus::Scheduled,
            scheduled_date,
            started_at: None,
            completed_at: None,
            conducted_by,
            findings: Vec::new(),
            created_at: Utc::now(),
        })
    }

    // Accessors
    pub fn id(&self) -> &AccessReviewId {
        &self.id
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn status(&self) -> AccessReviewStatus {
        self.status
    }

    pub fn scheduled_date(&self) -> &DateTime<Utc> {
        &self.scheduled_date
    }

    pub fn started_at(&self) -> Option<&DateTime<Utc>> {
        self.started_at.as_ref()
    }

    pub fn completed_at(&self) -> Option<&DateTime<Utc>> {
        self.completed_at.as_ref()
    }

    pub fn conducted_by(&self) -> Uuid {
        self.conducted_by
    }

    pub fn findings(&self) -> &[AccessReviewFinding] {
        &self.findings
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Start the review
    pub fn start(&mut self) -> Result<(), DomainError> {
        if self.status != AccessReviewStatus::Scheduled {
            return Err(DomainError::InvalidInput(
                "Only scheduled reviews can be started".into(),
            ));
        }
        self.status = AccessReviewStatus::InProgress;
        self.started_at = Some(Utc::now());
        Ok(())
    }

    /// Add finding to the review
    pub fn add_finding(&mut self, finding: AccessReviewFinding) -> Result<(), DomainError> {
        if self.status != AccessReviewStatus::InProgress {
            return Err(DomainError::InvalidInput(
                "Can only add findings to in-progress reviews".into(),
            ));
        }
        self.findings.push(finding);
        Ok(())
    }

    /// Complete the review
    pub fn complete(&mut self) -> Result<(), DomainError> {
        if self.status != AccessReviewStatus::InProgress {
            return Err(DomainError::InvalidInput(
                "Only in-progress reviews can be completed".into(),
            ));
        }
        self.status = AccessReviewStatus::Completed;
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    /// Cancel the review
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == AccessReviewStatus::Completed {
            return Err(DomainError::InvalidInput(
                "Cannot cancel completed review".into(),
            ));
        }
        self.status = AccessReviewStatus::Cancelled;
        Ok(())
    }

    /// Get critical findings count
    pub fn critical_findings_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == AccessReviewSeverity::Critical)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approval_workflow_creation() {
        let workflow = ApprovalWorkflow::new(
            Uuid::new_v4(),
            "GrantRole".to_string(),
            Uuid::new_v4(),
            ApprovalType::FourEyes,
            Some("Operational necessity".to_string()),
        )
        .unwrap();

        assert_eq!(workflow.status(), ApprovalStatus::Pending);
        assert_eq!(workflow.approval_type(), ApprovalType::FourEyes);
        assert!(!workflow.is_expired());
    }

    #[test]
    fn test_approval_workflow_submit_decision() {
        let mut workflow = ApprovalWorkflow::new(
            Uuid::new_v4(),
            "GrantRole".to_string(),
            Uuid::new_v4(),
            ApprovalType::TwoEyes,
            None,
        )
        .unwrap();

        let approver1 = Uuid::new_v4();
        workflow
            .submit_approval(approver1, ApprovalDecision::Approved, None)
            .unwrap();

        assert_eq!(workflow.approvals().len(), 1);
        assert_eq!(workflow.status(), ApprovalStatus::InProgress);
    }

    #[test]
    fn test_approval_workflow_duplicate_approver() {
        let mut workflow = ApprovalWorkflow::new(
            Uuid::new_v4(),
            "GrantRole".to_string(),
            Uuid::new_v4(),
            ApprovalType::TwoEyes,
            None,
        )
        .unwrap();

        let approver = Uuid::new_v4();
        workflow
            .submit_approval(approver, ApprovalDecision::Approved, None)
            .unwrap();

        let result = workflow.submit_approval(approver, ApprovalDecision::Approved, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_approval_workflow_cancel() {
        let mut workflow = ApprovalWorkflow::new(
            Uuid::new_v4(),
            "GrantRole".to_string(),
            Uuid::new_v4(),
            ApprovalType::FourEyes,
            None,
        )
        .unwrap();

        workflow.cancel().unwrap();
        assert_eq!(workflow.status(), ApprovalStatus::Cancelled);
    }

    #[test]
    fn test_approval_workflow_approval_percentage() {
        let mut workflow = ApprovalWorkflow::new(
            Uuid::new_v4(),
            "GrantRole".to_string(),
            Uuid::new_v4(),
            ApprovalType::FourEyes,
            None,
        )
        .unwrap();

        workflow
            .submit_approval(Uuid::new_v4(), ApprovalDecision::Approved, None)
            .unwrap();
        workflow
            .submit_approval(Uuid::new_v4(), ApprovalDecision::Rejected, None)
            .unwrap();

        assert_eq!(workflow.approval_percentage(), 50);
    }

    #[test]
    fn test_power_delegation_creation() {
        let delegation = PowerDelegation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Approver:ALL".to_string(),
            Some("Emergency coverage".to_string()),
            7,
        )
        .unwrap();

        assert_eq!(delegation.status(), DelegationStatus::Pending);
    }

    #[test]
    fn test_power_delegation_activate() {
        let mut delegation = PowerDelegation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Approver:ALL".to_string(),
            None,
            7,
        )
        .unwrap();

        delegation.activate().unwrap();
        assert_eq!(delegation.status(), DelegationStatus::Active);
    }

    #[test]
    fn test_power_delegation_revoke() {
        let mut delegation = PowerDelegation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Approver:ALL".to_string(),
            None,
            7,
        )
        .unwrap();

        delegation.activate().unwrap();
        let revoker = Uuid::new_v4();
        delegation.revoke(revoker).unwrap();

        assert_eq!(delegation.status(), DelegationStatus::Revoked);
        assert_eq!(delegation.revoked_by(), Some(revoker));
    }

    #[test]
    fn test_power_delegation_invalid_duration() {
        let result = PowerDelegation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Approver:ALL".to_string(),
            None,
            0,
        );
        assert!(result.is_err());

        let result = PowerDelegation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Approver:ALL".to_string(),
            None,
            400,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_access_review_creation() {
        let review = AccessReview::new(
            "All users".to_string(),
            Utc::now(),
            Uuid::new_v4(),
        )
        .unwrap();

        assert_eq!(review.status(), AccessReviewStatus::Scheduled);
    }

    #[test]
    fn test_access_review_start_and_complete() {
        let mut review = AccessReview::new(
            "All users".to_string(),
            Utc::now(),
            Uuid::new_v4(),
        )
        .unwrap();

        review.start().unwrap();
        assert_eq!(review.status(), AccessReviewStatus::InProgress);

        review.complete().unwrap();
        assert_eq!(review.status(), AccessReviewStatus::Completed);
    }

    #[test]
    fn test_access_review_add_finding() {
        let mut review = AccessReview::new(
            "All users".to_string(),
            Utc::now(),
            Uuid::new_v4(),
        )
        .unwrap();

        review.start().unwrap();

        let finding = AccessReviewFinding {
            user_id: Uuid::new_v4(),
            findings: "Excessive permissions".to_string(),
            recommended_action: "Revoke unused roles".to_string(),
            severity: AccessReviewSeverity::Warning,
        };

        review.add_finding(finding).unwrap();
        assert_eq!(review.findings().len(), 1);
    }

    #[test]
    fn test_access_review_critical_findings() {
        let mut review = AccessReview::new(
            "All users".to_string(),
            Utc::now(),
            Uuid::new_v4(),
        )
        .unwrap();

        review.start().unwrap();

        for i in 0..3 {
            let severity = if i == 0 {
                AccessReviewSeverity::Critical
            } else {
                AccessReviewSeverity::Warning
            };

            let finding = AccessReviewFinding {
                user_id: Uuid::new_v4(),
                findings: format!("Finding {}", i),
                recommended_action: "Action".to_string(),
                severity,
            };
            review.add_finding(finding).unwrap();
        }

        assert_eq!(review.critical_findings_count(), 1);
    }

    #[test]
    fn test_approval_type_required_approvers() {
        assert_eq!(ApprovalType::TwoEyes.required_approvers(), 2);
        assert_eq!(ApprovalType::FourEyes.required_approvers(), 4);
        assert_eq!(ApprovalType::SixEyes.required_approvers(), 6);
    }
}
