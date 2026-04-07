# BANKO Governance BC11 - BMAD v4.0.1 Compliance Enhancement

## Overview

This document outlines the complete BMAD v4.0.1 compliance enhancements for the Governance bounded context (BC11) of BANKO. The enhancements implement requirements FR-140 through FR-152, focusing on RBAC, segregation of duties, approval workflows, power delegation, and periodic access reviews.

## Architecture

The governance domain layer follows hexagonal architecture with three distinct components:

1. **Domain Layer** (`crates/domain/src/governance/`)
   - Pure business logic, no external dependencies
   - Entities: Role, Permission, ApprovalWorkflow, PowerDelegation, AccessReview
   - Value Objects: RoleId, PermissionId, ApprovalDecision, DelegationStatus

2. **Application Layer** (`crates/application/src/governance/`)
   - Ports (traits): Service contracts
   - DTOs: API data structures
   - Use case orchestration

3. **Infrastructure Layer** (`crates/infrastructure/src/governance/`)
   - Repository implementations (PostgreSQL)
   - HTTP handlers (Actix-web)
   - Adapter implementations

## Requirements Mapping

### FR-140: Granular RBAC with Inheritance

**File**: `crates/domain/src/governance/rbac.rs`

**Implementation**:
- `Role` entity with parent role support (role hierarchy)
- `Permission` entity with resource:action granularity
- Role inheritance allows child roles to inherit parent permissions
- Least privilege principle enforced through explicit permission grants

**Key Classes**:
```rust
pub struct Role {
    id: RoleId,
    name: String,
    parent_role_id: Option<RoleId>,  // Inheritance
    permissions: HashSet<PermissionId>,
    is_active: bool,
}

pub struct Permission {
    id: PermissionId,
    resource: String,  // e.g., "Account"
    action: String,    // e.g., "CREATE"
}
```

**API Endpoints** (to be implemented):
- `POST /api/v1/governance/roles` - Create role with optional parent
- `POST /api/v1/governance/roles/{id}/permissions` - Grant permission
- `DELETE /api/v1/governance/roles/{id}/permissions/{perm_id}` - Revoke permission
- `GET /api/v1/governance/roles/{id}/effective-permissions` - Get inherited + direct permissions

### FR-141: Least Privilege Principle

**Implementation**:
- Default deny approach: users start with zero permissions
- Explicit permission grants via `grant_permission()`
- Inactive roles cannot receive new permissions
- Audit trail of all permission changes

**Validation Rules**:
- Permission names must be non-empty
- Resource and action must be specified
- Only active roles can be assigned permissions

### FR-142: Segregation of Duties (SoD)

**File**: `crates/domain/src/governance/rbac.rs`

**Implementation**:
- `SegregationOfDuties` entity prevents incompatible roles on same user
- Incompatible role pairs defined at governance level
- Incompatible actions prevent sequential execution by same user

**Key Classes**:
```rust
pub struct SegregationOfDuties {
    incompatible_pairs: HashSet<(RoleId, RoleId)>,
    incompatible_actions: HashMap<String, HashSet<String>>,
}
```

**Example**:
```rust
// Approver cannot also be requester
sod.add_incompatible_roles(RoleId::REQUESTER, RoleId::APPROVER)?;

// Account CREATE cannot be approved by same user
sod.add_incompatible_actions(
    "Account:CREATE",
    "Account:APPROVE"
)?;
```

**API Endpoints**:
- `POST /api/v1/governance/sod/incompatible-roles` - Register incompatible role pair
- `POST /api/v1/governance/sod/incompatible-actions` - Register incompatible actions
- `GET /api/v1/governance/sod/audit/{user_id}` - Check user for SoD violations
- `GET /api/v1/governance/sod/conflicts` - Get all defined conflicts

### FR-143: Immutable Audit Trail (Hash Chain)

**File**: `crates/domain/src/governance/hash_chain.rs`

**Existing Implementation** (Circular 2006-19 compliant):
- SHA-256 hash chain for immutability
- Each entry links to previous via hash
- Tamper detection through integrity verification
- 7-year retention policy (FR-147)

**Maintained Features**:
- `HashChain::verify_chain()` - Verify chain integrity
- `HashChain::verify_entry(sequence)` - Verify single entry
- `HashChain::get_proof(operation_id)` - Get proof of execution

### FR-144: Committee Audit & Governance

**File**: `crates/domain/src/governance/entities.rs`

**Existing Implementation**:
- `Committee` entity with meeting scheduling
- `CommitteeDecision` with vote tracking
- `CommitteeMeeting` with minutes recording

**Maintained Operations**:
- Create audit/risk committees
- Record decision outcomes (Approved/Rejected)
- Track individual votes with rationale
- Meeting minutes documentation

**API Endpoints**:
- `POST /api/v1/governance/committees` - Create committee
- `POST /api/v1/governance/committees/{id}/meetings` - Schedule meeting
- `POST /api/v1/governance/committees/{id}/decisions` - Record decision
- `GET /api/v1/governance/committees/{id}/audit` - Get decision audit trail

### FR-145: Automated Internal Controls (SOX-like)

**File**: `crates/domain/src/governance/entities.rs`

**Existing Implementation**:
- `ControlCheck` entity for dual control verification
- `ControlCheckSignOff` for sign-off tracking
- First/second/third line defense model

**Maintained Controls**:
- Operational controls (dual approval on sensitive ops)
- Compliance monitoring (audit trail integrity)
- Internal audit (committee governance)

**API Endpoints**:
- `POST /api/v1/governance/controls` - Create control check
- `POST /api/v1/governance/controls/{id}/approve` - Approve operation
- `POST /api/v1/governance/controls/{id}/signoff` - Sign off control

### FR-146: Real-Time Governance Dashboard

**File**: `crates/application/src/governance/workflow_dto.rs`

**Implementation**:
- `GovernanceComplianceResponse` - Overall compliance score
- `ApprovalDashboardResponse` - Pending approvals, backlog
- `RbacDashboardResponse` - Role/permission overview
- `AccessReviewDashboardResponse` - Review scheduling/findings

**Metrics Provided**:
- Pending approval workflows count
- SoD violations count
- Access review compliance status
- Critical findings count
- Delegation expiry warnings

### FR-147: Log Retention Policy (7 Years BCT)

**Implementation**:
- Audit entries stored in `governance.audit_trail` table
- Archive tables for entries older than 7 years
- Retention policy enforced at database level

**Configuration**:
```sql
-- Example: Archive audits older than 7 years
SELECT * FROM governance.audit_trail
WHERE timestamp < NOW() - INTERVAL '7 years'
ORDER BY timestamp DESC;
```

### FR-148: Audit Trail Export (BCT, JSON, CSV)

**File**: `crates/application/src/governance/service.rs`

**Existing Implementation**:
- `BctAuditService::export_csv()` - CSV export
- `BctAuditService::export_json()` - JSON export
- `AuditExportResponse` - Standardized export format

**Export Formats**:
- CSV: Comma-separated values with headers
- JSON: Structured JSON array with all fields
- BCT format: Compliant with BCT audit standards

**API Endpoints**:
- `GET /api/v1/governance/audit/export?format=csv` - Export as CSV
- `GET /api/v1/governance/audit/export?format=json` - Export as JSON
- `GET /api/v1/governance/audit/export?format=bct` - BCT-compliant format

### FR-149: Anomaly Detection Alerts

**Planned Implementation**:
- Monitor unusual approval workflow patterns
- Detect privilege escalation attempts
- Flag SoD violations
- Alert on excessive delegations

**Example Events**:
- Multiple rejections from same approver
- Excessive approvals from single user
- Rapid role changes
- Delegation scope creep

### FR-150: Multi-Level Approval Workflows (4-Eyes, 6-Eyes)

**File**: `crates/domain/src/governance/workflow.rs`

**Implementation**:
- `ApprovalWorkflow` entity with configurable approval types
- 2-eyes (TwoEyes), 4-eyes (FourEyes), 6-eyes (SixEyes) support
- Approval status tracking per workflow
- Auto-finalization based on decision count

**Key Classes**:
```rust
pub enum ApprovalType {
    TwoEyes,   // 2 approvers required
    FourEyes,  // 4 approvers required
    SixEyes,   // 6 approvers required
}

pub struct ApprovalWorkflow {
    id: ApprovalWorkflowId,
    operation_id: Uuid,
    operation_type: String,  // e.g., "GrantRole"
    approval_type: ApprovalType,
    status: ApprovalStatus,
    approvals: Vec<Approval>,
    expires_at: Option<DateTime<Utc>>,
}
```

**Workflow States**:
1. **Pending** - Initial state, awaiting first approval
2. **InProgress** - At least one approval received
3. **Approved** - Threshold met (2/3 majority rule)
4. **Rejected** - At least one rejection received
5. **Cancelled** - Workflow cancelled by requester
6. **Expired** - 30 days without completion (configurable)

**Approval Logic**:
- Approvers submit: Approved, Rejected, or Abstained
- Minimum unique approvers: Required count
- Approval threshold: 2/3 majority
- Any rejection triggers immediate rejection

**API Endpoints**:
- `POST /api/v1/governance/workflows` - Create approval workflow
- `POST /api/v1/governance/workflows/{id}/approve` - Submit approval
- `GET /api/v1/governance/workflows/pending` - List pending workflows
- `GET /api/v1/governance/workflows/awaiting-approval` - Approver queue
- `GET /api/v1/governance/workflows/{id}/audit` - Approval audit trail

### FR-151: Temporary Power Delegation

**File**: `crates/domain/src/governance/workflow.rs`

**Implementation**:
- `PowerDelegation` entity with time-bound validity
- Scoped delegation (e.g., "Approver:ALL", "Approver:LOANS")
- Automatic expiry after duration
- Early revocation capability

**Key Classes**:
```rust
pub struct PowerDelegation {
    id: PowerDelegationId,
    delegated_from: Uuid,      // Original role holder
    delegated_to: Uuid,         // Temporary delegate
    scope: String,              // "Approver:ALL"
    valid_from: DateTime<Utc>,
    valid_until: DateTime<Utc>,
    status: DelegationStatus,   // Active, Pending, Revoked, Expired
}
```

**Constraints**:
- Duration: 1-365 days (enforced)
- Max concurrent delegations: TBD (to be configured)
- Audit trail: All delegations logged

**State Transitions**:
1. **Pending** - Created, awaiting activation
2. **Active** - Approved/activated, currently valid
3. **Revoked** - Early revocation by delegator/admin
4. **Expired** - Past expiry date (auto-transitioned)

**API Endpoints**:
- `POST /api/v1/governance/delegations` - Create delegation
- `POST /api/v1/governance/delegations/{id}/activate` - Activate delegation
- `POST /api/v1/governance/delegations/{id}/revoke` - Revoke delegation
- `GET /api/v1/governance/delegations/active/{user_id}` - Active delegations for user
- `GET /api/v1/governance/delegations/history/{user_id}` - Delegation history

### FR-152: Periodic Access Review

**File**: `crates/domain/src/governance/workflow.rs`

**Implementation**:
- `AccessReview` entity with comprehensive finding tracking
- Severity levels: Info, Warning, Critical
- Scheduling for periodic execution
- Finding resolution tracking

**Key Classes**:
```rust
pub struct AccessReview {
    id: AccessReviewId,
    scope: String,              // "All users", "Department:Finance"
    status: AccessReviewStatus, // Scheduled, InProgress, Completed, Cancelled
    scheduled_date: DateTime<Utc>,
    findings: Vec<AccessReviewFinding>,
    conducted_by: Uuid,
}

pub struct AccessReviewFinding {
    user_id: Uuid,
    findings: String,           // e.g., "Excessive permissions"
    recommended_action: String,  // e.g., "Revoke unused roles"
    severity: AccessReviewSeverity,  // Info, Warning, Critical
}
```

**Review Workflow**:
1. **Scheduled** - Created with future date
2. **InProgress** - Review started, findings being added
3. **Completed** - Review finished, findings documented
4. **Cancelled** - Review cancelled

**Scheduling**:
- Annual minimum (Circ. 2006-19 requirement)
- Configurable frequency per department/scope
- Automatic reminders for overdue reviews

**API Endpoints**:
- `POST /api/v1/governance/reviews/schedule` - Schedule review
- `POST /api/v1/governance/reviews/{id}/start` - Start review
- `POST /api/v1/governance/reviews/{id}/findings` - Add finding
- `POST /api/v1/governance/reviews/{id}/complete` - Complete review
- `GET /api/v1/governance/reviews/scheduled` - List scheduled reviews
- `GET /api/v1/governance/reviews/critical-findings` - Critical findings across all reviews
- `GET /api/v1/governance/reviews/user/{user_id}` - Reviews involving user

## Data Model

### Tables (PostgreSQL)

#### governance.roles
```sql
CREATE TABLE governance.roles (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_role_id UUID REFERENCES governance.roles(id),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
```

#### governance.permissions
```sql
CREATE TABLE governance.permissions (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    UNIQUE(resource, action)
);
```

#### governance.role_permissions
```sql
CREATE TABLE governance.role_permissions (
    role_id UUID NOT NULL REFERENCES governance.roles(id),
    permission_id UUID NOT NULL REFERENCES governance.permissions(id),
    PRIMARY KEY (role_id, permission_id)
);
```

#### governance.user_roles
```sql
CREATE TABLE governance.user_roles (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES governance.roles(id),
    assigned_at TIMESTAMPTZ NOT NULL,
    assigned_by UUID NOT NULL,
    PRIMARY KEY (user_id, role_id)
);
```

#### governance.approval_workflows
```sql
CREATE TABLE governance.approval_workflows (
    id UUID PRIMARY KEY,
    operation_id UUID NOT NULL,
    operation_type VARCHAR(100) NOT NULL,
    requested_by UUID NOT NULL,
    approval_type VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ
);
```

#### governance.approval_decisions
```sql
CREATE TABLE governance.approval_decisions (
    id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES governance.approval_workflows(id),
    approver_id UUID NOT NULL,
    decision VARCHAR(20) NOT NULL,
    comments TEXT,
    approved_at TIMESTAMPTZ NOT NULL
);
```

#### governance.power_delegations
```sql
CREATE TABLE governance.power_delegations (
    id UUID PRIMARY KEY,
    delegated_from UUID NOT NULL,
    delegated_to UUID NOT NULL,
    scope VARCHAR(255) NOT NULL,
    reason TEXT,
    status VARCHAR(20) NOT NULL,
    valid_from TIMESTAMPTZ NOT NULL,
    valid_until TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID
);
```

#### governance.access_reviews
```sql
CREATE TABLE governance.access_reviews (
    id UUID PRIMARY KEY,
    scope VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL,
    scheduled_date TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    conducted_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
```

#### governance.access_review_findings
```sql
CREATE TABLE governance.access_review_findings (
    id UUID PRIMARY KEY,
    review_id UUID NOT NULL REFERENCES governance.access_reviews(id),
    user_id UUID NOT NULL,
    findings TEXT NOT NULL,
    recommended_action TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
```

## Testing Strategy

### Unit Tests
- Entity creation and validation
- Business rule enforcement
- State transitions
- Error handling

**Example**:
```rust
#[test]
fn test_role_inheritance() {
    let parent = Role::new("Admin".into(), None, None).unwrap();
    let child = Role::new("Manager".into(), None, Some(parent.id().clone())).unwrap();

    assert_eq!(child.parent_role_id(), Some(&parent.id()));
}

#[test]
fn test_sod_violation_detection() {
    let mut sod = SegregationOfDuties::new();
    let requester = RoleId::new();
    let approver = RoleId::new();

    sod.add_incompatible_roles(requester.clone(), approver.clone()).unwrap();
    assert!(sod.are_incompatible(&requester, &approver));
}
```

### Integration Tests
- Repository operations (CRUD)
- Workflow state transitions
- Approval chain logic
- SoD enforcement

### BDD Tests (Cucumber)
See `tests/bdd/features/governance.feature` for acceptance criteria.

## Security Considerations

### Input Validation
- Empty string validation on all names/descriptions
- UUID format validation
- Date range validation (delegation duration 1-365 days)
- SQL injection prevention via parameterized queries

### Access Control
- All endpoints require authentication
- Authorization checks per endpoint
- Role-based permission verification

### Audit Logging
- Every role/permission change logged
- All approvals/rejections logged
- Delegation lifecycle logged
- Review findings logged

### Compliance
- Circular 2006-19: Hash-chain immutability, 7-year retention
- FR-150: Multi-level approvals enforced
- FR-142: SoD rules enforced
- FR-141: Least privilege validated

## Implementation Roadmap

### Phase 1 (Complete)
- RBAC entities (Role, Permission)
- Segregation of Duties rules
- User role assignment

### Phase 2 (To Implement)
- Repository implementations (PostgreSQL)
- HTTP handlers (Actix-web)
- Service layer orchestration
- Integration with auth context

### Phase 3 (To Implement)
- Dashboard UI endpoints
- Approval workflow UI
- Delegation management UI
- Access review UI

### Phase 4 (To Implement)
- Anomaly detection (FR-149)
- Automated compliance reports
- Notification system
- Scheduled review triggers

## Compliance Verification

To verify BMAD v4.0.1 compliance:

```bash
# Run all tests
make test

# Run governance tests specifically
cargo test -p banko-domain governance::
cargo test -p banko-application governance::

# Check coverage
make coverage
```

## References

- **BMAD v4.0.1 Standard**: Moroccan Banking Regulatory Framework
- **Circular 2006-19**: Audit trail and data retention requirements
- **SOX Compliance**: Internal controls and segregation of duties
- **Architecture Pattern**: Hexagonal architecture (Ports & Adapters)
- **DDD Principles**: Domain-Driven Design with bounded contexts

## Contact & Support

For questions regarding BMAD v4.0.1 compliance or governance enhancements:
- Review docs in `docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md`
- Consult BMAD specification documents in `docs/bmad/`
- Contact: compliance@banko.local
