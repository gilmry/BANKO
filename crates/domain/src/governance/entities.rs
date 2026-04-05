use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Newtypes
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuditEntryId(Uuid);

impl AuditEntryId {
    pub fn new() -> Self {
        AuditEntryId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        AuditEntryId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AuditEntryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AuditEntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditAction {
    Create,
    Read,
    Update,
    Delete,
    Login,
    Logout,
    Approve,
    Reject,
    Submit,
    Export,
}

impl AuditAction {
    pub fn as_str(&self) -> &str {
        match self {
            AuditAction::Create => "Create",
            AuditAction::Read => "Read",
            AuditAction::Update => "Update",
            AuditAction::Delete => "Delete",
            AuditAction::Login => "Login",
            AuditAction::Logout => "Logout",
            AuditAction::Approve => "Approve",
            AuditAction::Reject => "Reject",
            AuditAction::Submit => "Submit",
            AuditAction::Export => "Export",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Create" => Ok(AuditAction::Create),
            "Read" => Ok(AuditAction::Read),
            "Update" => Ok(AuditAction::Update),
            "Delete" => Ok(AuditAction::Delete),
            "Login" => Ok(AuditAction::Login),
            "Logout" => Ok(AuditAction::Logout),
            "Approve" => Ok(AuditAction::Approve),
            "Reject" => Ok(AuditAction::Reject),
            "Submit" => Ok(AuditAction::Submit),
            "Export" => Ok(AuditAction::Export),
            _ => Err(DomainError::InvalidAuditEntry(format!(
                "Unknown audit action: {s}"
            ))),
        }
    }
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Customer,
    Account,
    Loan,
    Transaction,
    Alert,
    Investigation,
    Payment,
    Report,
    User,
    System,
}

impl ResourceType {
    pub fn as_str(&self) -> &str {
        match self {
            ResourceType::Customer => "Customer",
            ResourceType::Account => "Account",
            ResourceType::Loan => "Loan",
            ResourceType::Transaction => "Transaction",
            ResourceType::Alert => "Alert",
            ResourceType::Investigation => "Investigation",
            ResourceType::Payment => "Payment",
            ResourceType::Report => "Report",
            ResourceType::User => "User",
            ResourceType::System => "System",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Customer" => Ok(ResourceType::Customer),
            "Account" => Ok(ResourceType::Account),
            "Loan" => Ok(ResourceType::Loan),
            "Transaction" => Ok(ResourceType::Transaction),
            "Alert" => Ok(ResourceType::Alert),
            "Investigation" => Ok(ResourceType::Investigation),
            "Payment" => Ok(ResourceType::Payment),
            "Report" => Ok(ResourceType::Report),
            "User" => Ok(ResourceType::User),
            "System" => Ok(ResourceType::System),
            _ => Err(DomainError::InvalidAuditEntry(format!(
                "Unknown resource type: {s}"
            ))),
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// AuditTrailEntry — immutable aggregate (INV-12)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditTrailEntry {
    entry_id: AuditEntryId,
    timestamp: DateTime<Utc>,
    user_id: Uuid,
    action: AuditAction,
    resource_type: ResourceType,
    resource_id: Uuid,
    changes: Option<String>,
    ip_address: Option<String>,
    previous_hash: String,
    hash: String,
}

impl AuditTrailEntry {
    pub fn new(
        user_id: Uuid,
        action: AuditAction,
        resource_type: ResourceType,
        resource_id: Uuid,
        changes: Option<String>,
        ip_address: Option<String>,
        previous_hash: String,
    ) -> Self {
        let entry_id = AuditEntryId::new();
        let timestamp = Utc::now();

        let hash = Self::compute_hash(
            &user_id,
            &action,
            &resource_type,
            &resource_id,
            &timestamp,
            &previous_hash,
        );

        AuditTrailEntry {
            entry_id,
            timestamp,
            user_id,
            action,
            resource_type,
            resource_id,
            changes,
            ip_address,
            previous_hash,
            hash,
        }
    }

    /// Reconstruct from persistence (no recomputation).
    pub fn from_raw(
        entry_id: AuditEntryId,
        timestamp: DateTime<Utc>,
        user_id: Uuid,
        action: AuditAction,
        resource_type: ResourceType,
        resource_id: Uuid,
        changes: Option<String>,
        ip_address: Option<String>,
        previous_hash: String,
        hash: String,
    ) -> Self {
        AuditTrailEntry {
            entry_id,
            timestamp,
            user_id,
            action,
            resource_type,
            resource_id,
            changes,
            ip_address,
            previous_hash,
            hash,
        }
    }

    fn compute_hash(
        user_id: &Uuid,
        action: &AuditAction,
        resource_type: &ResourceType,
        resource_id: &Uuid,
        timestamp: &DateTime<Utc>,
        previous_hash: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(user_id.to_string().as_bytes());
        hasher.update(action.as_str().as_bytes());
        hasher.update(resource_type.as_str().as_bytes());
        hasher.update(resource_id.to_string().as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        hasher.update(previous_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify that the stored hash matches recomputation.
    pub fn verify_hash(&self) -> bool {
        let expected = Self::compute_hash(
            &self.user_id,
            &self.action,
            &self.resource_type,
            &self.resource_id,
            &self.timestamp,
            &self.previous_hash,
        );
        self.hash == expected
    }

    // --- Getters (NO setters — immutable) ---

    pub fn entry_id(&self) -> &AuditEntryId {
        &self.entry_id
    }
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
    pub fn action(&self) -> &AuditAction {
        &self.action
    }
    pub fn resource_type(&self) -> &ResourceType {
        &self.resource_type
    }
    pub fn resource_id(&self) -> &Uuid {
        &self.resource_id
    }
    pub fn changes(&self) -> Option<&str> {
        self.changes.as_deref()
    }
    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }
    pub fn previous_hash(&self) -> &str {
        &self.previous_hash
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }
}

// ============================================================
// HashChain — verification helper
// ============================================================

pub struct HashChain;

impl HashChain {
    /// Verify the integrity of a sequence of audit trail entries.
    /// Each entry's previous_hash must match the prior entry's hash.
    pub fn verify_chain(entries: &[AuditTrailEntry]) -> Result<(), DomainError> {
        for (i, entry) in entries.iter().enumerate() {
            // Verify individual hash
            if !entry.verify_hash() {
                return Err(DomainError::HashChainViolation {
                    entry_id: entry.entry_id().to_string(),
                });
            }

            // Verify chain link (skip first entry)
            if i > 0 {
                let prev = &entries[i - 1];
                if entry.previous_hash() != prev.hash() {
                    return Err(DomainError::HashChainViolation {
                        entry_id: entry.entry_id().to_string(),
                    });
                }
            }
        }
        Ok(())
    }
}

// ============================================================
// Committee (GOV-07)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommitteeType {
    Audit,
    Risk,
    Nomination,
    Credit,
}

impl CommitteeType {
    pub fn as_str(&self) -> &str {
        match self {
            CommitteeType::Audit => "Audit",
            CommitteeType::Risk => "Risk",
            CommitteeType::Nomination => "Nomination",
            CommitteeType::Credit => "Credit",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Audit" => Ok(CommitteeType::Audit),
            "Risk" => Ok(CommitteeType::Risk),
            "Nomination" => Ok(CommitteeType::Nomination),
            "Credit" => Ok(CommitteeType::Credit),
            _ => Err(DomainError::InvalidCommittee(format!(
                "Unknown committee type: {s}"
            ))),
        }
    }
}

impl fmt::Display for CommitteeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Committee {
    id: Uuid,
    name: String,
    committee_type: CommitteeType,
    members: Vec<Uuid>,
    created_at: DateTime<Utc>,
}

impl Committee {
    pub fn new(
        name: String,
        committee_type: CommitteeType,
        members: Vec<Uuid>,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidCommittee(
                "Committee name cannot be empty".to_string(),
            ));
        }
        if members.is_empty() {
            return Err(DomainError::InvalidCommittee(
                "Committee must have at least one member".to_string(),
            ));
        }
        Ok(Committee {
            id: Uuid::new_v4(),
            name,
            committee_type,
            members,
            created_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: Uuid,
        name: String,
        committee_type: CommitteeType,
        members: Vec<Uuid>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Committee {
            id,
            name,
            committee_type,
            members,
            created_at,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn committee_type(&self) -> &CommitteeType {
        &self.committee_type
    }
    pub fn members(&self) -> &[Uuid] {
        &self.members
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

// ============================================================
// CommitteeDecision & Vote
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Approved,
    Rejected,
    Deferred,
}

impl DecisionOutcome {
    pub fn as_str(&self) -> &str {
        match self {
            DecisionOutcome::Approved => "Approved",
            DecisionOutcome::Rejected => "Rejected",
            DecisionOutcome::Deferred => "Deferred",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Approved" => Ok(DecisionOutcome::Approved),
            "Rejected" => Ok(DecisionOutcome::Rejected),
            "Deferred" => Ok(DecisionOutcome::Deferred),
            _ => Err(DomainError::InvalidCommittee(format!(
                "Unknown decision outcome: {s}"
            ))),
        }
    }
}

impl fmt::Display for DecisionOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

impl VoteChoice {
    pub fn as_str(&self) -> &str {
        match self {
            VoteChoice::For => "For",
            VoteChoice::Against => "Against",
            VoteChoice::Abstain => "Abstain",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "For" => Ok(VoteChoice::For),
            "Against" => Ok(VoteChoice::Against),
            "Abstain" => Ok(VoteChoice::Abstain),
            _ => Err(DomainError::InvalidCommittee(format!(
                "Unknown vote choice: {s}"
            ))),
        }
    }
}

impl fmt::Display for VoteChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vote {
    pub member_id: Uuid,
    pub vote: VoteChoice,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitteeDecision {
    id: Uuid,
    committee_id: Uuid,
    subject: String,
    decision: DecisionOutcome,
    votes: Vec<Vote>,
    justification: Option<String>,
    decided_at: DateTime<Utc>,
}

impl CommitteeDecision {
    pub fn new(
        committee_id: Uuid,
        subject: String,
        decision: DecisionOutcome,
        votes: Vec<Vote>,
        justification: Option<String>,
    ) -> Result<Self, DomainError> {
        if subject.trim().is_empty() {
            return Err(DomainError::InvalidCommittee(
                "Decision subject cannot be empty".to_string(),
            ));
        }
        Ok(CommitteeDecision {
            id: Uuid::new_v4(),
            committee_id,
            subject,
            decision,
            votes,
            justification,
            decided_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: Uuid,
        committee_id: Uuid,
        subject: String,
        decision: DecisionOutcome,
        votes: Vec<Vote>,
        justification: Option<String>,
        decided_at: DateTime<Utc>,
    ) -> Self {
        CommitteeDecision {
            id,
            committee_id,
            subject,
            decision,
            votes,
            justification,
            decided_at,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn committee_id(&self) -> &Uuid {
        &self.committee_id
    }
    pub fn subject(&self) -> &str {
        &self.subject
    }
    pub fn decision(&self) -> &DecisionOutcome {
        &self.decision
    }
    pub fn votes(&self) -> &[Vote] {
        &self.votes
    }
    pub fn justification(&self) -> Option<&str> {
        self.justification.as_deref()
    }
    pub fn decided_at(&self) -> &DateTime<Utc> {
        &self.decided_at
    }
}

// ============================================================
// ControlCheck (GOV-08)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlStatus {
    Pending,
    Approved,
    Rejected,
}

impl ControlStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ControlStatus::Pending => "Pending",
            ControlStatus::Approved => "Approved",
            ControlStatus::Rejected => "Rejected",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Pending" => Ok(ControlStatus::Pending),
            "Approved" => Ok(ControlStatus::Approved),
            "Rejected" => Ok(ControlStatus::Rejected),
            _ => Err(DomainError::InvalidControlCheck(format!(
                "Unknown control status: {s}"
            ))),
        }
    }
}

impl fmt::Display for ControlStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlCheck {
    id: Uuid,
    operation_type: String,
    operation_id: Uuid,
    checker_id: Option<Uuid>,
    status: ControlStatus,
    comments: Option<String>,
    checked_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl ControlCheck {
    pub fn new(
        operation_type: String,
        operation_id: Uuid,
    ) -> Result<Self, DomainError> {
        if operation_type.trim().is_empty() {
            return Err(DomainError::InvalidControlCheck(
                "Operation type cannot be empty".to_string(),
            ));
        }
        Ok(ControlCheck {
            id: Uuid::new_v4(),
            operation_type,
            operation_id,
            checker_id: None,
            status: ControlStatus::Pending,
            comments: None,
            checked_at: None,
            created_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: Uuid,
        operation_type: String,
        operation_id: Uuid,
        checker_id: Option<Uuid>,
        status: ControlStatus,
        comments: Option<String>,
        checked_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    ) -> Self {
        ControlCheck {
            id,
            operation_type,
            operation_id,
            checker_id,
            status,
            comments,
            checked_at,
            created_at,
        }
    }

    pub fn approve(&mut self, checker_id: Uuid) -> Result<(), DomainError> {
        if self.status != ControlStatus::Pending {
            return Err(DomainError::InvalidControlCheck(
                "Can only approve a pending control check".to_string(),
            ));
        }
        self.checker_id = Some(checker_id);
        self.status = ControlStatus::Approved;
        self.checked_at = Some(Utc::now());
        Ok(())
    }

    pub fn reject(&mut self, checker_id: Uuid, reason: String) -> Result<(), DomainError> {
        if self.status != ControlStatus::Pending {
            return Err(DomainError::InvalidControlCheck(
                "Can only reject a pending control check".to_string(),
            ));
        }
        self.checker_id = Some(checker_id);
        self.status = ControlStatus::Rejected;
        self.comments = Some(reason);
        self.checked_at = Some(Utc::now());
        Ok(())
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn operation_type(&self) -> &str {
        &self.operation_type
    }
    pub fn operation_id(&self) -> &Uuid {
        &self.operation_id
    }
    pub fn checker_id(&self) -> Option<&Uuid> {
        self.checker_id.as_ref()
    }
    pub fn status(&self) -> &ControlStatus {
        &self.status
    }
    pub fn comments(&self) -> Option<&str> {
        self.comments.as_deref()
    }
    pub fn checked_at(&self) -> Option<&DateTime<Utc>> {
        self.checked_at.as_ref()
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entry_hash_computation_and_verification() {
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();
        let entry = AuditTrailEntry::new(
            user_id,
            AuditAction::Create,
            ResourceType::Customer,
            resource_id,
            Some(r#"{"name":"John"}"#.to_string()),
            Some("192.168.1.1".to_string()),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );

        assert!(entry.verify_hash());
        assert!(!entry.hash().is_empty());
        assert_eq!(entry.hash().len(), 64); // SHA256 hex = 64 chars
    }

    #[test]
    fn test_hash_chain_integrity_with_three_entries() {
        let user_id = Uuid::new_v4();
        let genesis_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();

        let e1 = AuditTrailEntry::new(
            user_id,
            AuditAction::Create,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            genesis_hash,
        );

        let e2 = AuditTrailEntry::new(
            user_id,
            AuditAction::Update,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            e1.hash().to_string(),
        );

        let e3 = AuditTrailEntry::new(
            user_id,
            AuditAction::Approve,
            ResourceType::Account,
            Uuid::new_v4(),
            None,
            None,
            e2.hash().to_string(),
        );

        let chain = vec![e1, e2, e3];
        assert!(HashChain::verify_chain(&chain).is_ok());
    }

    #[test]
    fn test_hash_chain_detects_tampering() {
        let user_id = Uuid::new_v4();
        let genesis_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();

        let e1 = AuditTrailEntry::new(
            user_id,
            AuditAction::Create,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            genesis_hash,
        );

        // Create e2 with WRONG previous hash (not linked to e1)
        let e2 = AuditTrailEntry::new(
            user_id,
            AuditAction::Update,
            ResourceType::Customer,
            Uuid::new_v4(),
            None,
            None,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        );

        let chain = vec![e1, e2];
        let result = HashChain::verify_chain(&chain);
        assert!(result.is_err());
        match result {
            Err(DomainError::HashChainViolation { .. }) => {}
            _ => panic!("Expected HashChainViolation"),
        }
    }

    #[test]
    fn test_audit_entry_immutability_no_setters() {
        // This test verifies that AuditTrailEntry only exposes getters.
        // The struct fields are private, so there are no setters.
        let entry = AuditTrailEntry::new(
            Uuid::new_v4(),
            AuditAction::Login,
            ResourceType::User,
            Uuid::new_v4(),
            None,
            Some("10.0.0.1".to_string()),
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        );

        // All we can do is read
        let _ = entry.entry_id();
        let _ = entry.timestamp();
        let _ = entry.user_id();
        let _ = entry.action();
        let _ = entry.resource_type();
        let _ = entry.resource_id();
        let _ = entry.changes();
        let _ = entry.ip_address();
        let _ = entry.previous_hash();
        let _ = entry.hash();
        assert!(entry.verify_hash());
    }

    #[test]
    fn test_committee_creation() {
        let members = vec![Uuid::new_v4(), Uuid::new_v4()];
        let committee = Committee::new("Audit Committee".to_string(), CommitteeType::Audit, members.clone());
        assert!(committee.is_ok());
        let c = committee.unwrap();
        assert_eq!(c.name(), "Audit Committee");
        assert_eq!(*c.committee_type(), CommitteeType::Audit);
        assert_eq!(c.members().len(), 2);
    }

    #[test]
    fn test_committee_empty_name_rejected() {
        let result = Committee::new("".to_string(), CommitteeType::Risk, vec![Uuid::new_v4()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_committee_no_members_rejected() {
        let result = Committee::new("Risk Committee".to_string(), CommitteeType::Risk, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_committee_decision_workflow() {
        let committee_id = Uuid::new_v4();
        let member1 = Uuid::new_v4();
        let member2 = Uuid::new_v4();

        let votes = vec![
            Vote { member_id: member1, vote: VoteChoice::For },
            Vote { member_id: member2, vote: VoteChoice::Against },
        ];

        let decision = CommitteeDecision::new(
            committee_id,
            "Approve loan XYZ".to_string(),
            DecisionOutcome::Approved,
            votes,
            Some("Majority approved".to_string()),
        );
        assert!(decision.is_ok());
        let d = decision.unwrap();
        assert_eq!(d.subject(), "Approve loan XYZ");
        assert_eq!(*d.decision(), DecisionOutcome::Approved);
        assert_eq!(d.votes().len(), 2);
    }

    #[test]
    fn test_control_check_approve() {
        let mut check = ControlCheck::new("LoanDisbursement".to_string(), Uuid::new_v4()).unwrap();
        assert_eq!(*check.status(), ControlStatus::Pending);

        let checker = Uuid::new_v4();
        assert!(check.approve(checker).is_ok());
        assert_eq!(*check.status(), ControlStatus::Approved);
        assert_eq!(*check.checker_id().unwrap(), checker);
        assert!(check.checked_at().is_some());
    }

    #[test]
    fn test_control_check_reject() {
        let mut check = ControlCheck::new("PaymentRelease".to_string(), Uuid::new_v4()).unwrap();
        let checker = Uuid::new_v4();
        assert!(check.reject(checker, "Insufficient documentation".to_string()).is_ok());
        assert_eq!(*check.status(), ControlStatus::Rejected);
        assert_eq!(check.comments().unwrap(), "Insufficient documentation");
    }

    #[test]
    fn test_control_check_cannot_approve_twice() {
        let mut check = ControlCheck::new("Transfer".to_string(), Uuid::new_v4()).unwrap();
        check.approve(Uuid::new_v4()).unwrap();
        let result = check.approve(Uuid::new_v4());
        assert!(result.is_err());
    }

    #[test]
    fn test_control_check_cannot_reject_after_approve() {
        let mut check = ControlCheck::new("Transfer".to_string(), Uuid::new_v4()).unwrap();
        check.approve(Uuid::new_v4()).unwrap();
        let result = check.reject(Uuid::new_v4(), "reason".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_enum_roundtrip_audit_action() {
        for action in [
            AuditAction::Create, AuditAction::Read, AuditAction::Update,
            AuditAction::Delete, AuditAction::Login, AuditAction::Logout,
            AuditAction::Approve, AuditAction::Reject, AuditAction::Submit,
            AuditAction::Export,
        ] {
            let s = action.as_str();
            let parsed = AuditAction::from_str_type(s).unwrap();
            assert_eq!(action, parsed);
        }
    }

    #[test]
    fn test_enum_roundtrip_resource_type() {
        for rt in [
            ResourceType::Customer, ResourceType::Account, ResourceType::Loan,
            ResourceType::Transaction, ResourceType::Alert, ResourceType::Investigation,
            ResourceType::Payment, ResourceType::Report, ResourceType::User,
            ResourceType::System,
        ] {
            let s = rt.as_str();
            let parsed = ResourceType::from_str_type(s).unwrap();
            assert_eq!(rt, parsed);
        }
    }
}
