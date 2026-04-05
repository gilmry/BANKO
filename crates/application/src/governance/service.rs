use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use banko_domain::governance::*;

use super::dto::*;
use super::errors::GovernanceServiceError;
use super::ports::*;

// ============================================================
// AuditService (GOV-01 to GOV-05)
// ============================================================

pub struct AuditService {
    audit_repo: Arc<dyn IAuditRepository>,
}

impl AuditService {
    pub fn new(audit_repo: Arc<dyn IAuditRepository>) -> Self {
        AuditService { audit_repo }
    }

    /// Log an action to the immutable audit trail (INV-12).
    pub async fn log_action(
        &self,
        user_id: Uuid,
        action: AuditAction,
        resource_type: ResourceType,
        resource_id: Uuid,
        changes: Option<String>,
        ip_address: Option<String>,
    ) -> Result<AuditEntryResponse, GovernanceServiceError> {
        // Get previous hash from latest entry (genesis = all zeros)
        let previous_hash = match self
            .audit_repo
            .find_latest()
            .await
            .map_err(GovernanceServiceError::Internal)?
        {
            Some(entry) => entry.hash().to_string(),
            None => "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        };

        let entry = AuditTrailEntry::new(
            user_id,
            action,
            resource_type,
            resource_id,
            changes,
            ip_address,
            previous_hash,
        );

        self.audit_repo
            .append(&entry)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(to_audit_response(&entry))
    }

    /// Retrieve paginated audit trail.
    pub async fn get_audit_trail(
        &self,
        filters: AuditFilter,
        page: i64,
        limit: i64,
    ) -> Result<AuditListResponse, GovernanceServiceError> {
        let limit = limit.clamp(1, 100);
        let page = page.max(1);
        let offset = (page - 1) * limit;

        let entries = self
            .audit_repo
            .find_all(&filters, limit, offset)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let total = self
            .audit_repo
            .count_all(&filters)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let data = entries.iter().map(to_audit_response).collect();

        Ok(AuditListResponse {
            data,
            total,
            page,
            limit,
        })
    }

    /// Verify hash chain integrity for a date range.
    pub async fn verify_integrity(
        &self,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Result<IntegrityCheckResponse, GovernanceServiceError> {
        let entries = self
            .audit_repo
            .find_chain(from, to)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let count = entries.len();
        match HashChain::verify_chain(&entries) {
            Ok(()) => Ok(IntegrityCheckResponse {
                valid: true,
                entries_checked: count,
                error: None,
                checked_from: from,
                checked_to: to,
            }),
            Err(e) => Ok(IntegrityCheckResponse {
                valid: false,
                entries_checked: count,
                error: Some(e.to_string()),
                checked_from: from,
                checked_to: to,
            }),
        }
    }
}

// ============================================================
// ComplianceReportService (GOV-06)
// ============================================================

pub struct ComplianceReportService {
    audit_repo: Arc<dyn IAuditRepository>,
    committee_repo: Arc<dyn ICommitteeRepository>,
    control_repo: Arc<dyn IControlCheckRepository>,
}

impl ComplianceReportService {
    pub fn new(
        audit_repo: Arc<dyn IAuditRepository>,
        committee_repo: Arc<dyn ICommitteeRepository>,
        control_repo: Arc<dyn IControlCheckRepository>,
    ) -> Self {
        ComplianceReportService {
            audit_repo,
            committee_repo,
            control_repo,
        }
    }

    pub async fn generate_report(
        &self,
    ) -> Result<ComplianceReportResponse, GovernanceServiceError> {
        // First line: operational controls
        let approved = self
            .control_repo
            .count_all(Some(ControlStatus::Approved))
            .await
            .map_err(GovernanceServiceError::Internal)?;
        let rejected = self
            .control_repo
            .count_all(Some(ControlStatus::Rejected))
            .await
            .map_err(GovernanceServiceError::Internal)?;
        let pending = self
            .control_repo
            .count_all(Some(ControlStatus::Pending))
            .await
            .map_err(GovernanceServiceError::Internal)?;

        // Second line: audit trail integrity
        let now = Utc::now();
        let from = now - Duration::days(30);
        let filter = AuditFilter::default();
        let total_audit = self
            .audit_repo
            .count_all(&filter)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let chain = self
            .audit_repo
            .find_chain(from, now)
            .await
            .map_err(GovernanceServiceError::Internal)?;
        let integrity_valid = HashChain::verify_chain(&chain).is_ok();

        // Third line: committee governance
        let committees = self
            .committee_repo
            .find_all_committees()
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let mut total_decisions = 0usize;
        for c in &committees {
            let decisions = self
                .committee_repo
                .find_decisions_by_committee(*c.id())
                .await
                .map_err(GovernanceServiceError::Internal)?;
            total_decisions += decisions.len();
        }

        Ok(ComplianceReportResponse {
            generated_at: now,
            first_line: FirstLineDefense {
                description: "Operational controls — dual control checks on sensitive operations"
                    .to_string(),
                total_controls: approved + rejected + pending,
                approved,
                rejected,
                pending,
            },
            second_line: SecondLineDefense {
                description: "Compliance monitoring — immutable audit trail with SHA256 hash chain"
                    .to_string(),
                total_audit_entries: total_audit,
                integrity_valid,
            },
            third_line: ThirdLineDefense {
                description: "Internal audit — committee governance and decision tracking"
                    .to_string(),
                total_committees: committees.len(),
                total_decisions,
            },
        })
    }
}

// ============================================================
// CommitteeService (GOV-07)
// ============================================================

pub struct CommitteeService {
    committee_repo: Arc<dyn ICommitteeRepository>,
}

impl CommitteeService {
    pub fn new(committee_repo: Arc<dyn ICommitteeRepository>) -> Self {
        CommitteeService { committee_repo }
    }

    pub async fn create_committee(
        &self,
        name: String,
        committee_type: String,
        members: Vec<Uuid>,
    ) -> Result<CommitteeResponse, GovernanceServiceError> {
        let ct = CommitteeType::from_str_type(&committee_type)
            .map_err(|e| GovernanceServiceError::InvalidInput(e.to_string()))?;

        let committee = Committee::new(name, ct, members)
            .map_err(|e| GovernanceServiceError::DomainError(e.to_string()))?;

        self.committee_repo
            .save_committee(&committee)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(to_committee_response(&committee))
    }

    pub async fn list_committees(
        &self,
    ) -> Result<Vec<CommitteeResponse>, GovernanceServiceError> {
        let committees = self
            .committee_repo
            .find_all_committees()
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(committees.iter().map(to_committee_response).collect())
    }

    pub async fn record_decision(
        &self,
        committee_id: Uuid,
        subject: String,
        decision: String,
        votes: Vec<(Uuid, String)>,
        justification: Option<String>,
    ) -> Result<CommitteeDecisionResponse, GovernanceServiceError> {
        // Verify committee exists
        let _committee = self
            .committee_repo
            .find_committee_by_id(committee_id)
            .await
            .map_err(GovernanceServiceError::Internal)?
            .ok_or(GovernanceServiceError::CommitteeNotFound)?;

        let outcome = DecisionOutcome::from_str_type(&decision)
            .map_err(|e| GovernanceServiceError::InvalidInput(e.to_string()))?;

        let mut parsed_votes = Vec::new();
        for (member_id, vote_str) in &votes {
            let vc = VoteChoice::from_str_type(vote_str)
                .map_err(|e| GovernanceServiceError::InvalidInput(e.to_string()))?;
            parsed_votes.push(Vote {
                member_id: *member_id,
                vote: vc,
            });
        }

        let decision_entity = CommitteeDecision::new(
            committee_id,
            subject,
            outcome,
            parsed_votes,
            justification,
        )
        .map_err(|e| GovernanceServiceError::DomainError(e.to_string()))?;

        self.committee_repo
            .save_decision(&decision_entity)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(to_decision_response(&decision_entity))
    }
}

// ============================================================
// ControlService (GOV-08)
// ============================================================

pub struct ControlService {
    control_repo: Arc<dyn IControlCheckRepository>,
}

impl ControlService {
    pub fn new(control_repo: Arc<dyn IControlCheckRepository>) -> Self {
        ControlService { control_repo }
    }

    pub async fn create_check(
        &self,
        operation_type: String,
        operation_id: Uuid,
    ) -> Result<ControlCheckResponse, GovernanceServiceError> {
        let check = ControlCheck::new(operation_type, operation_id)
            .map_err(|e| GovernanceServiceError::DomainError(e.to_string()))?;

        self.control_repo
            .save(&check)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(to_control_response(&check))
    }

    pub async fn approve(
        &self,
        id: Uuid,
        checker_id: Uuid,
    ) -> Result<ControlCheckResponse, GovernanceServiceError> {
        let mut check = self
            .control_repo
            .find_by_id(id)
            .await
            .map_err(GovernanceServiceError::Internal)?
            .ok_or(GovernanceServiceError::ControlCheckNotFound)?;

        check
            .approve(checker_id)
            .map_err(|e| GovernanceServiceError::DomainError(e.to_string()))?;

        self.control_repo
            .save(&check)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(to_control_response(&check))
    }

    pub async fn reject(
        &self,
        id: Uuid,
        checker_id: Uuid,
        reason: String,
    ) -> Result<ControlCheckResponse, GovernanceServiceError> {
        let mut check = self
            .control_repo
            .find_by_id(id)
            .await
            .map_err(GovernanceServiceError::Internal)?
            .ok_or(GovernanceServiceError::ControlCheckNotFound)?;

        check
            .reject(checker_id, reason)
            .map_err(|e| GovernanceServiceError::DomainError(e.to_string()))?;

        self.control_repo
            .save(&check)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        Ok(to_control_response(&check))
    }

    pub async fn list_checks(
        &self,
        status: Option<String>,
        page: i64,
        limit: i64,
    ) -> Result<ControlCheckListResponse, GovernanceServiceError> {
        let limit = limit.clamp(1, 100);
        let page = page.max(1);
        let offset = (page - 1) * limit;

        let status_filter = match status {
            Some(s) => Some(
                ControlStatus::from_str_type(&s)
                    .map_err(|e| GovernanceServiceError::InvalidInput(e.to_string()))?,
            ),
            None => None,
        };

        let checks = self
            .control_repo
            .find_all(status_filter, limit, offset)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let total = self
            .control_repo
            .count_all(status_filter)
            .await
            .map_err(GovernanceServiceError::Internal)?;

        let data = checks.iter().map(to_control_response).collect();

        Ok(ControlCheckListResponse {
            data,
            total,
            page,
            limit,
        })
    }
}

// ============================================================
// Mapping helpers
// ============================================================

fn to_audit_response(entry: &AuditTrailEntry) -> AuditEntryResponse {
    AuditEntryResponse {
        id: entry.entry_id().to_string(),
        timestamp: *entry.timestamp(),
        user_id: entry.user_id().to_string(),
        action: entry.action().as_str().to_string(),
        resource_type: entry.resource_type().as_str().to_string(),
        resource_id: entry.resource_id().to_string(),
        changes: entry.changes().map(|s| s.to_string()),
        ip_address: entry.ip_address().map(|s| s.to_string()),
        previous_hash: entry.previous_hash().to_string(),
        hash: entry.hash().to_string(),
    }
}

fn to_committee_response(c: &Committee) -> CommitteeResponse {
    CommitteeResponse {
        id: c.id().to_string(),
        name: c.name().to_string(),
        committee_type: c.committee_type().as_str().to_string(),
        members: c.members().iter().map(|m| m.to_string()).collect(),
        created_at: *c.created_at(),
    }
}

fn to_decision_response(d: &CommitteeDecision) -> CommitteeDecisionResponse {
    CommitteeDecisionResponse {
        id: d.id().to_string(),
        committee_id: d.committee_id().to_string(),
        subject: d.subject().to_string(),
        decision: d.decision().as_str().to_string(),
        votes: d
            .votes()
            .iter()
            .map(|v| VoteResponse {
                member_id: v.member_id.to_string(),
                vote: v.vote.as_str().to_string(),
            })
            .collect(),
        justification: d.justification().map(|s| s.to_string()),
        decided_at: *d.decided_at(),
    }
}

fn to_control_response(c: &ControlCheck) -> ControlCheckResponse {
    ControlCheckResponse {
        id: c.id().to_string(),
        operation_type: c.operation_type().to_string(),
        operation_id: c.operation_id().to_string(),
        checker_id: c.checker_id().map(|u| u.to_string()),
        status: c.status().as_str().to_string(),
        comments: c.comments().map(|s| s.to_string()),
        checked_at: c.checked_at().copied(),
        created_at: *c.created_at(),
    }
}
