# Governance BC11 - Implementation Guide for Infrastructure Layer

This guide provides detailed instructions for implementing the infrastructure and HTTP handler layers for the BMAD v4.0.1 governance enhancements.

## Prerequisites

- Rust 1.70+
- PostgreSQL 16+
- SQLx CLI
- Understanding of hexagonal architecture
- Knowledge of Actix-web framework

## Phase 1: Database Schema & Migrations

### Step 1: Create Migration Files

```bash
cd backend
sqlx migrate add -r governance_rbac_tables
sqlx migrate add -r governance_workflow_tables
```

### Step 2: Define RBAC Tables

File: `backend/migrations/TIMESTAMP_governance_rbac_tables.up.sql`

```sql
-- Roles with hierarchy support
CREATE TABLE governance.roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    parent_role_id UUID REFERENCES governance.roles(id) ON DELETE SET NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    CHECK (id != parent_role_id)  -- No self-references
);

CREATE INDEX idx_roles_parent ON governance.roles(parent_role_id);
CREATE INDEX idx_roles_active ON governance.roles(is_active);

-- Permissions with resource:action granularity
CREATE TABLE governance.permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    UNIQUE(resource, action)
);

CREATE INDEX idx_permissions_resource_action ON governance.permissions(resource, action);

-- Role-Permission assignments (many-to-many)
CREATE TABLE governance.role_permissions (
    role_id UUID NOT NULL REFERENCES governance.roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES governance.permissions(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by UUID NOT NULL,
    PRIMARY KEY (role_id, permission_id)
);

CREATE INDEX idx_role_permissions_role ON governance.role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission ON governance.role_permissions(permission_id);

-- User-Role assignments (many-to-many)
CREATE TABLE governance.user_roles (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES governance.roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by UUID NOT NULL,
    PRIMARY KEY (user_id, role_id)
);

CREATE INDEX idx_user_roles_user ON governance.user_roles(user_id);
CREATE INDEX idx_user_roles_role ON governance.user_roles(role_id);

-- Audit: Role change history
CREATE TABLE governance.role_changes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES governance.roles(id),
    operation VARCHAR(20) NOT NULL,  -- 'ASSIGNED' or 'REVOKED'
    changed_by UUID NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reason TEXT
);

CREATE INDEX idx_role_changes_user ON governance.role_changes(user_id);
CREATE INDEX idx_role_changes_timestamp ON governance.role_changes(changed_at);
```

File: `backend/migrations/TIMESTAMP_governance_rbac_tables.down.sql`

```sql
DROP TABLE IF EXISTS governance.role_changes;
DROP TABLE IF EXISTS governance.user_roles;
DROP TABLE IF EXISTS governance.role_permissions;
DROP TABLE IF EXISTS governance.permissions;
DROP TABLE IF EXISTS governance.roles;
```

### Step 3: Define Workflow Tables

File: `backend/migrations/TIMESTAMP_governance_workflow_tables.up.sql`

```sql
-- Approval Workflows (FR-150)
CREATE TABLE governance.approval_workflows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL,
    operation_type VARCHAR(100) NOT NULL,
    requested_by UUID NOT NULL,
    approval_type VARCHAR(20) NOT NULL,  -- 'TwoEyes', 'FourEyes', 'SixEyes'
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',  -- 'Pending', 'InProgress', 'Approved', 'Rejected', 'Cancelled'
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    UNIQUE(operation_id)
);

CREATE INDEX idx_workflows_status ON governance.approval_workflows(status);
CREATE INDEX idx_workflows_requester ON governance.approval_workflows(requested_by);
CREATE INDEX idx_workflows_expires ON governance.approval_workflows(expires_at);

-- Approval Decisions (per workflow)
CREATE TABLE governance.approval_decisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES governance.approval_workflows(id) ON DELETE CASCADE,
    approver_id UUID NOT NULL,
    decision VARCHAR(20) NOT NULL,  -- 'Approved', 'Rejected', 'Abstained'
    comments TEXT,
    approved_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(workflow_id, approver_id)  -- One decision per approver per workflow
);

CREATE INDEX idx_decisions_workflow ON governance.approval_decisions(workflow_id);
CREATE INDEX idx_decisions_approver ON governance.approval_decisions(approver_id);

-- Power Delegations (FR-151)
CREATE TABLE governance.power_delegations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    delegated_from UUID NOT NULL,
    delegated_to UUID NOT NULL,
    scope VARCHAR(255) NOT NULL,  -- e.g., 'Approver:ALL'
    reason TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',  -- 'Active', 'Pending', 'Revoked', 'Expired'
    valid_from TIMESTAMPTZ NOT NULL,
    valid_until TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    revoked_by UUID
);

CREATE INDEX idx_delegations_delegated_to ON governance.power_delegations(delegated_to);
CREATE INDEX idx_delegations_delegated_from ON governance.power_delegations(delegated_from);
CREATE INDEX idx_delegations_status ON governance.power_delegations(status);
CREATE INDEX idx_delegations_valid_until ON governance.power_delegations(valid_until);

-- Access Reviews (FR-152)
CREATE TABLE governance.access_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scope VARCHAR(255) NOT NULL,  -- e.g., 'All users', 'Department:Finance'
    status VARCHAR(20) NOT NULL DEFAULT 'Scheduled',  -- 'Scheduled', 'InProgress', 'Completed', 'Cancelled'
    scheduled_date TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    conducted_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reviews_status ON governance.access_reviews(status);
CREATE INDEX idx_reviews_scheduled_date ON governance.access_reviews(scheduled_date);
CREATE INDEX idx_reviews_conductor ON governance.access_reviews(conducted_by);

-- Access Review Findings
CREATE TABLE governance.access_review_findings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_id UUID NOT NULL REFERENCES governance.access_reviews(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    findings TEXT NOT NULL,
    recommended_action TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL,  -- 'Info', 'Warning', 'Critical'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_findings_review ON governance.access_review_findings(review_id);
CREATE INDEX idx_findings_user ON governance.access_review_findings(user_id);
CREATE INDEX idx_findings_severity ON governance.access_review_findings(severity);
```

File: `backend/migrations/TIMESTAMP_governance_workflow_tables.down.sql`

```sql
DROP TABLE IF EXISTS governance.access_review_findings;
DROP TABLE IF EXISTS governance.access_reviews;
DROP TABLE IF EXISTS governance.power_delegations;
DROP TABLE IF EXISTS governance.approval_decisions;
DROP TABLE IF EXISTS governance.approval_workflows;
```

### Step 4: Run Migrations

```bash
sqlx migrate run
```

## Phase 2: Repository Implementations

### Step 1: Create Repository Module Structure

```bash
# Create repository files
touch crates/infrastructure/src/governance/repositories/role_repository.rs
touch crates/infrastructure/src/governance/repositories/permission_repository.rs
touch crates/infrastructure/src/governance/repositories/user_role_repository.rs
touch crates/infrastructure/src/governance/repositories/approval_workflow_repository.rs
touch crates/infrastructure/src/governance/repositories/power_delegation_repository.rs
touch crates/infrastructure/src/governance/repositories/access_review_repository.rs
```

### Step 2: Implement RoleRepository

File: `crates/infrastructure/src/governance/repositories/role_repository.rs`

```rust
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::governance::IRoleRepository;
use banko_domain::governance::{Role, RoleId};

pub struct RoleRepository {
    pool: PgPool,
}

impl RoleRepository {
    pub fn new(pool: PgPool) -> Self {
        RoleRepository { pool }
    }
}

#[async_trait]
impl IRoleRepository for RoleRepository {
    async fn save(&self, role: &Role) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO governance.roles (id, name, description, parent_role_id, is_active, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                name = $2,
                description = $3,
                parent_role_id = $4,
                is_active = $5,
                updated_at = NOW()
            "#,
        )
        .bind(role.id().as_uuid())
        .bind(role.name())
        .bind(role.description())
        .bind(role.parent_role_id().map(|rid| rid.as_uuid()))
        .bind(role.is_active())
        .bind(Uuid::nil())  // TODO: Get from auth context
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &RoleId) -> Result<Option<Role>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, parent_role_id, is_active, created_at, updated_at
            FROM governance.roles
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(_row) = row {
            // TODO: Reconstruct Role entity from row
            // Need to fetch permissions separately
        }

        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<Role>, String> {
        // TODO: Implement
        Ok(Vec::new())
    }

    async fn find_active(&self) -> Result<Vec<Role>, String> {
        // TODO: Implement with is_active = true filter
        Ok(Vec::new())
    }

    async fn find_by_parent(&self, parent_id: &RoleId) -> Result<Vec<Role>, String> {
        // TODO: Implement with parent_role_id = parent_id
        Ok(Vec::new())
    }

    async fn delete(&self, id: &RoleId) -> Result<(), String> {
        // TODO: Implement soft delete (update is_active = false)
        Ok(())
    }

    async fn count_all(&self) -> Result<i64, String> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM governance.roles"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(count)
    }
}
```

### Step 3: Implement Additional Repositories

Follow similar pattern for:
- `PermissionRepository` - CRUD for permissions
- `UserRoleRepository` - User-role mappings with conflict checking
- `ApprovalWorkflowRepository` - Workflow CRUD and queue queries
- `PowerDelegationRepository` - Delegation lifecycle
- `AccessReviewRepository` - Review scheduling and findings

### Step 4: Create Repository Module

File: `crates/infrastructure/src/governance/repositories/mod.rs`

```rust
pub mod role_repository;
pub mod permission_repository;
pub mod user_role_repository;
pub mod approval_workflow_repository;
pub mod power_delegation_repository;
pub mod access_review_repository;

pub use role_repository::RoleRepository;
pub use permission_repository::PermissionRepository;
pub use user_role_repository::UserRoleRepository;
pub use approval_workflow_repository::ApprovalWorkflowRepository;
pub use power_delegation_repository::PowerDelegationRepository;
pub use access_review_repository::AccessReviewRepository;
```

## Phase 3: Service Layer Implementation

### Step 1: Create Service Files

```bash
touch crates/application/src/governance/rbac_service.rs
touch crates/application/src/governance/workflow_service.rs
```

### Step 2: Implement RbacService

File: `crates/application/src/governance/rbac_service.rs`

```rust
use std::sync::Arc;
use uuid::Uuid;

use banko_application::governance::{
    IRoleRepository, IPermissionRepository, IUserRoleRepository,
    CreateRoleRequest, RoleResponse, CreatePermissionRequest, PermissionResponse,
};
use banko_domain::governance::{Role, Permission, RoleId};

pub struct RbacService {
    role_repo: Arc<dyn IRoleRepository>,
    permission_repo: Arc<dyn IPermissionRepository>,
    user_role_repo: Arc<dyn IUserRoleRepository>,
}

impl RbacService {
    pub fn new(
        role_repo: Arc<dyn IRoleRepository>,
        permission_repo: Arc<dyn IPermissionRepository>,
        user_role_repo: Arc<dyn IUserRoleRepository>,
    ) -> Self {
        RbacService {
            role_repo,
            permission_repo,
            user_role_repo,
        }
    }

    pub async fn create_role(
        &self,
        req: CreateRoleRequest,
    ) -> Result<RoleResponse, String> {
        // Create domain entity
        let parent_id = req.parent_role_id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id).ok())
            .map(RoleId::from_uuid);

        let role = Role::new(req.name, req.description, parent_id)
            .map_err(|e| e.to_string())?;

        // Save to repository
        self.role_repo.save(&role).await?;

        // Convert to response
        Ok(to_role_response(&role))
    }

    pub async fn grant_permission(
        &self,
        role_id: &str,
        permission_id: &str,
    ) -> Result<RoleResponse, String> {
        let role_id = Uuid::parse_str(role_id)
            .ok()
            .map(RoleId::from_uuid)
            .ok_or("Invalid role ID")?;

        let permission_id = Uuid::parse_str(permission_id)
            .ok()
            .map(|id| banko_domain::governance::PermissionId::from_uuid(id))
            .ok_or("Invalid permission ID")?;

        // Load role
        let mut role = self.role_repo
            .find_by_id(&role_id)
            .await?
            .ok_or("Role not found")?;

        // Grant permission
        role.grant_permission(permission_id)
            .map_err(|e| e.to_string())?;

        // Save updated role
        self.role_repo.save(&role).await?;

        Ok(to_role_response(&role))
    }

    // Additional methods for role/permission management...
}

fn to_role_response(role: &Role) -> RoleResponse {
    RoleResponse {
        id: role.id().to_string(),
        name: role.name().to_string(),
        description: role.description().map(|s| s.to_string()),
        parent_role_id: role.parent_role_id().map(|id| id.to_string()),
        permissions: role.permissions()
            .iter()
            .map(|pid| pid.to_string())
            .collect(),
        is_active: role.is_active(),
        created_at: *role.created_at(),
        updated_at: *role.updated_at(),
    }
}
```

### Step 3: Implement WorkflowService

File: `crates/application/src/governance/workflow_service.rs`

```rust
use std::sync::Arc;
use uuid::Uuid;

use banko_application::governance::{
    IApprovalWorkflowRepository, IPowerDelegationRepository, IAccessReviewRepository,
    CreateApprovalWorkflowRequest, ApprovalWorkflowResponse,
};
use banko_domain::governance::{ApprovalWorkflow, ApprovalType};

pub struct WorkflowService {
    workflow_repo: Arc<dyn IApprovalWorkflowRepository>,
    delegation_repo: Arc<dyn IPowerDelegationRepository>,
    review_repo: Arc<dyn IAccessReviewRepository>,
}

impl WorkflowService {
    pub fn new(
        workflow_repo: Arc<dyn IApprovalWorkflowRepository>,
        delegation_repo: Arc<dyn IPowerDelegationRepository>,
        review_repo: Arc<dyn IAccessReviewRepository>,
    ) -> Self {
        WorkflowService {
            workflow_repo,
            delegation_repo,
            review_repo,
        }
    }

    pub async fn create_approval_workflow(
        &self,
        req: CreateApprovalWorkflowRequest,
        requester_id: Uuid,
    ) -> Result<ApprovalWorkflowResponse, String> {
        let operation_id = Uuid::parse_str(&req.operation_id)
            .map_err(|_| "Invalid operation ID")?;

        let approval_type = ApprovalType::from_str_type(&req.approval_type)
            .map_err(|e| e.to_string())?;

        let workflow = ApprovalWorkflow::new(
            operation_id,
            req.operation_type,
            requester_id,
            approval_type,
            req.reason,
        )
        .map_err(|e| e.to_string())?;

        self.workflow_repo.save(&workflow).await?;

        Ok(to_workflow_response(&workflow))
    }

    // Additional methods for approval, delegation, review management...
}

fn to_workflow_response(workflow: &ApprovalWorkflow) -> ApprovalWorkflowResponse {
    ApprovalWorkflowResponse {
        id: workflow.id().to_string(),
        operation_id: workflow.operation_id().to_string(),
        operation_type: workflow.operation_type().to_string(),
        requested_by: workflow.requested_by().to_string(),
        approval_type: workflow.approval_type().as_str().to_string(),
        status: workflow.status().as_str().to_string(),
        approvals: workflow.approvals()
            .iter()
            .map(|a| banko_application::governance::ApprovalResponse {
                approver_id: a.approver_id.to_string(),
                decision: a.decision.as_str().to_string(),
                comments: a.comments.clone(),
                approved_at: a.approved_at,
            })
            .collect(),
        approvals_received: workflow.approvals().len(),
        approvals_required: workflow.approval_type().required_approvers(),
        approval_percentage: workflow.approval_percentage(),
        reason: workflow.reason().map(|s| s.to_string()),
        created_at: *workflow.created_at(),
        updated_at: *workflow.updated_at(),
        expires_at: workflow.expires_at().copied(),
        is_expired: workflow.is_expired(),
    }
}
```

## Phase 4: HTTP Handlers

### Step 1: Create Handler Files

```bash
touch crates/infrastructure/src/governance/handlers/rbac_handlers.rs
touch crates/infrastructure/src/governance/handlers/workflow_handlers.rs
```

### Step 2: Implement RBAC Handlers

File: `crates/infrastructure/src/governance/handlers/rbac_handlers.rs`

```rust
use actix_web::{web, HttpResponse};
use std::sync::Arc;
use uuid::Uuid;

use banko_application::governance::{RbacService, CreateRoleRequest, CreatePermissionRequest};
use crate::web::middleware::AuthenticatedUser;

/// POST /api/v1/governance/roles
pub async fn create_role_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<RbacService>>,
    req: web::Json<CreateRoleRequest>,
) -> HttpResponse {
    match service.create_role(req.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

/// GET /api/v1/governance/roles/{id}
pub async fn get_role_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<RbacService>>,
    role_id: web::Path<String>,
) -> HttpResponse {
    match service.get_role(&role_id).await {
        Ok(Some(response)) => HttpResponse::Ok().json(response),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Role not found" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e })),
    }
}

/// POST /api/v1/governance/roles/{id}/permissions
pub async fn grant_permission_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<RbacService>>,
    role_id: web::Path<String>,
    req: web::Json<serde_json::json!({"permission_id": String})>,
) -> HttpResponse {
    match service.grant_permission(&role_id, &req.permission_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

// Additional handlers for list roles, delete role, etc.
```

### Step 3: Implement Workflow Handlers

File: `crates/infrastructure/src/governance/handlers/workflow_handlers.rs`

```rust
use actix_web::{web, HttpResponse};
use std::sync::Arc;

use banko_application::governance::{WorkflowService, CreateApprovalWorkflowRequest};
use crate::web::middleware::AuthenticatedUser;

/// POST /api/v1/governance/workflows
pub async fn create_workflow_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<WorkflowService>>,
    req: web::Json<CreateApprovalWorkflowRequest>,
) -> HttpResponse {
    match service.create_approval_workflow(req.into_inner(), auth.user_id).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

/// GET /api/v1/governance/workflows/pending
pub async fn list_pending_workflows_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<WorkflowService>>,
) -> HttpResponse {
    match service.list_pending_workflows(10, 0).await {
        Ok(workflows) => HttpResponse::Ok().json(workflows),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e })),
    }
}

/// POST /api/v1/governance/workflows/{id}/approve
pub async fn approve_workflow_handler(
    auth: AuthenticatedUser,
    service: web::Data<Arc<WorkflowService>>,
    workflow_id: web::Path<String>,
    req: web::Json<serde_json::json!({"decision": String, "comments": Option<String>})>,
) -> HttpResponse {
    match service.submit_approval(&workflow_id, auth.user_id, &req.decision, req.comments.clone()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

// Additional handlers for delegation, review management, etc.
```

### Step 4: Register Routes

File: `crates/infrastructure/src/web/routes.rs` (add to existing file)

```rust
// In configure_routes or similar function:

pub fn configure_governance_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // RBAC Routes
        .route("/governance/roles", web::post().to(rbac_handlers::create_role_handler))
        .route("/governance/roles/{id}", web::get().to(rbac_handlers::get_role_handler))
        .route("/governance/roles", web::get().to(rbac_handlers::list_roles_handler))
        .route("/governance/roles/{id}/permissions", web::post().to(rbac_handlers::grant_permission_handler))

        // Approval Workflow Routes
        .route("/governance/workflows", web::post().to(workflow_handlers::create_workflow_handler))
        .route("/governance/workflows/{id}", web::get().to(workflow_handlers::get_workflow_handler))
        .route("/governance/workflows/pending", web::get().to(workflow_handlers::list_pending_workflows_handler))
        .route("/governance/workflows/{id}/approve", web::post().to(workflow_handlers::approve_workflow_handler))

        // Power Delegation Routes
        .route("/governance/delegations", web::post().to(workflow_handlers::create_delegation_handler))
        .route("/governance/delegations/{id}/activate", web::post().to(workflow_handlers::activate_delegation_handler))
        .route("/governance/delegations/{id}/revoke", web::post().to(workflow_handlers::revoke_delegation_handler))

        // Access Review Routes
        .route("/governance/reviews/schedule", web::post().to(workflow_handlers::schedule_review_handler))
        .route("/governance/reviews/{id}/start", web::post().to(workflow_handlers::start_review_handler))
        .route("/governance/reviews/{id}/findings", web::post().to(workflow_handlers::add_finding_handler))

        // Dashboard Routes
        .route("/governance/dashboard/rbac", web::get().to(dashboard_handlers::rbac_dashboard_handler))
        .route("/governance/dashboard/approvals", web::get().to(dashboard_handlers::approval_dashboard_handler))
        .route("/governance/dashboard/compliance", web::get().to(dashboard_handlers::compliance_dashboard_handler));
}
```

## Phase 5: Testing

### Integration Tests

Create `crates/infrastructure/tests/governance_integration_tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use banko_infrastructure::governance::repositories::*;
    use banko_application::governance::*;
    use banko_domain::governance::*;

    #[tokio::test]
    async fn test_create_and_retrieve_role() {
        // Setup
        let pool = setup_test_db().await;
        let role_repo = RoleRepository::new(pool);

        // Create
        let role = Role::new("Manager".into(), None, None).unwrap();
        role_repo.save(&role).await.unwrap();

        // Retrieve
        let retrieved = role_repo.find_by_id(role.id()).await.unwrap();
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_approval_workflow_lifecycle() {
        // Setup
        let pool = setup_test_db().await;
        let workflow_repo = ApprovalWorkflowRepository::new(pool);

        // Create
        let mut workflow = ApprovalWorkflow::new(
            uuid::Uuid::new_v4(),
            "GrantRole".into(),
            uuid::Uuid::new_v4(),
            ApprovalType::FourEyes,
            None,
        ).unwrap();

        workflow_repo.save(&workflow).await.unwrap();

        // Submit approvals
        let approver1 = uuid::Uuid::new_v4();
        workflow.submit_approval(approver1, ApprovalDecision::Approved, None).unwrap();

        // Verify workflow still pending
        assert_eq!(workflow.status(), ApprovalStatus::InProgress);
    }
}
```

### BDD Tests

Update `tests/bdd/features/governance.feature`:

```gherkin
Feature: Governance and RBAC (BC11)
  As a governance administrator
  I want to manage roles, permissions, and approval workflows
  So that the system maintains BMAD v4.0.1 compliance

  Scenario: Create role with parent role
    Given a parent role "Admin" exists
    When I create a child role "Manager" with parent "Admin"
    Then the child role should inherit permissions from parent

  Scenario: Segregation of duties enforcement
    Given roles "Requester" and "Approver" are incompatible
    When I try to assign both roles to user "john"
    Then the assignment should fail with SoD violation

  Scenario: Multi-level approval workflow
    Given an approval workflow with "FourEyes" approval required
    When 3 approvers approve the workflow
    And 1 approver abstains
    Then the workflow should have 75% approval
    And status should be "InProgress" waiting for one more

  Scenario: Power delegation with expiry
    Given user "alice" creates a delegation to "bob" for 7 days
    When the delegation is activated
    Then bob should have temporary access
    And the delegation should auto-expire after 7 days
```

## Deployment & Verification

### Local Testing

```bash
# Start dev environment
make dev

# Run all tests
make test

# Run governance tests
make test-unit -- governance::

# Check code coverage
make coverage
```

### Production Checklist

- [ ] All unit tests passing
- [ ] All integration tests passing
- [ ] BDD scenarios passing
- [ ] Code coverage > 80%
- [ ] Linting passing (cargo clippy)
- [ ] Security audit passing (cargo audit)
- [ ] Database migrations applied
- [ ] API documentation updated
- [ ] BMAD compliance verified
- [ ] SoD rules configured
- [ ] Role hierarchy established
- [ ] Approval workflows defined
- [ ] Access review schedule created

## Common Issues & Solutions

### Issue: Foreign Key Constraint Error on Role Deletion

**Solution**: Implement cascade delete properly:

```sql
CREATE TABLE governance.role_permissions (
    role_id UUID NOT NULL REFERENCES governance.roles(id) ON DELETE CASCADE,
    ...
);
```

### Issue: Approval Workflow Auto-Finalization Not Working

**Solution**: Ensure auto_finalize() is called after every decision submission:

```rust
fn submit_approval(...) {
    // ... add approval to vector ...
    self.auto_finalize();  // Call this!
}
```

### Issue: Permission Inheritance Not Working

**Solution**: When checking effective permissions, traverse parent roles:

```rust
async fn get_effective_permissions(user_id: Uuid) -> Vec<PermissionId> {
    let mut permissions = HashSet::new();
    let mut roles_to_check = vec![primary_role];

    while let Some(role) = roles_to_check.pop() {
        permissions.extend(role.permissions());
        if let Some(parent) = role.parent_role_id() {
            roles_to_check.push(fetch_role(parent).await?);
        }
    }

    permissions.into_iter().collect()
}
```

## Support & Documentation

- See `docs/governance/BMAD_V4_COMPLIANCE.md` for detailed requirements
- See `docs/governance/IMPLEMENTATION_GUIDE.md` (this file) for technical details
- Consult `docs/legal/REFERENTIEL_LEGAL_ET_NORMATIF.md` for legal requirements
- Review `docs/bmad/` for BMAD standard documentation

## Next Steps

1. Implement Phase 1-2 (Database & Repositories)
2. Run integration tests
3. Implement Phase 3-4 (Services & Handlers)
4. Write end-to-end tests
5. Conduct security review
6. Deploy to staging
7. Perform compliance audit
8. Deploy to production
