-- BANKO Governance BC Enhancement
-- RBAC roles with inheritance, permissions, SoD rules, approval workflows, power delegations, access reviews

ALTER TABLE governance.roles ADD COLUMN IF NOT EXISTS parent_role_id UUID REFERENCES governance.roles(id) ON DELETE SET NULL;
ALTER TABLE governance.roles ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE governance.roles ADD COLUMN IF NOT EXISTS risk_level VARCHAR(20) DEFAULT 'medium' CHECK (risk_level IN ('low', 'medium', 'high', 'critical'));

-- Role Permissions table
CREATE TABLE IF NOT EXISTS governance.role_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES governance.roles(id) ON DELETE CASCADE,
    permission_code VARCHAR(100) NOT NULL,
    permission_name VARCHAR(255) NOT NULL,
    permission_description TEXT,
    resource_type VARCHAR(100) NOT NULL, -- e.g., 'account', 'customer', 'report', 'config'
    action VARCHAR(50) NOT NULL CHECK (action IN ('create', 'read', 'update', 'delete', 'approve', 'execute', 'export', 'download')),
    conditions JSONB, -- Complex conditions (e.g., {limit: 100000, branch: 'main'})
    is_delegable BOOLEAN NOT NULL DEFAULT FALSE,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(role_id, permission_code)
);

CREATE INDEX idx_role_permissions_role ON governance.role_permissions(role_id);
CREATE INDEX idx_role_permissions_code ON governance.role_permissions(permission_code);
CREATE INDEX idx_role_permissions_resource ON governance.role_permissions(resource_type);
CREATE INDEX idx_role_permissions_action ON governance.role_permissions(action);

COMMENT ON TABLE governance.role_permissions IS 'Granular permission definitions for RBAC with conditions and delegability';

-- Segregation of Duties (SoD) Rules
CREATE TABLE IF NOT EXISTS governance.sod_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_code VARCHAR(50) NOT NULL UNIQUE,
    rule_name VARCHAR(255) NOT NULL,
    rule_description TEXT,
    conflicting_permission_1 VARCHAR(100) NOT NULL,
    conflicting_permission_2 VARCHAR(100) NOT NULL,
    conflict_type VARCHAR(50) NOT NULL CHECK (conflict_type IN ('incompatible', 'restricted', 'requires_approval', 'requires_different_department')),
    enforcement_level VARCHAR(50) NOT NULL DEFAULT 'strict' CHECK (enforcement_level IN ('advisory', 'warning', 'strict', 'critical')),
    exception_allowed BOOLEAN NOT NULL DEFAULT FALSE,
    exception_approval_required BOOLEAN NOT NULL DEFAULT TRUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sod_rules_active ON governance.sod_rules(is_active);
CREATE INDEX idx_sod_rules_enforcement ON governance.sod_rules(enforcement_level);

COMMENT ON TABLE governance.sod_rules IS 'Segregation of duties rules to prevent fraud and enforce controls';

-- User Role Assignments with temporal validity
CREATE TABLE IF NOT EXISTS governance.user_role_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES governance.roles(id) ON DELETE CASCADE,
    assignment_type VARCHAR(50) NOT NULL CHECK (assignment_type IN ('permanent', 'temporary', 'acting', 'delegated')),
    assigned_by UUID NOT NULL,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    assignment_reason TEXT,
    requires_mfa BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    revoked_by UUID,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_role_assignments_user ON governance.user_role_assignments(user_id);
CREATE INDEX idx_user_role_assignments_role ON governance.user_role_assignments(role_id);
CREATE INDEX idx_user_role_assignments_active ON governance.user_role_assignments(is_active, effective_from, effective_to);

COMMENT ON TABLE governance.user_role_assignments IS 'User role assignments with temporal validity and revocation tracking';

-- Approval Workflows
CREATE TABLE IF NOT EXISTS governance.approval_workflows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_code VARCHAR(50) NOT NULL UNIQUE,
    workflow_name VARCHAR(255) NOT NULL,
    workflow_description TEXT,
    workflow_type VARCHAR(50) NOT NULL CHECK (workflow_type IN ('sequential', 'parallel', 'conditional')),
    triggering_action VARCHAR(100) NOT NULL, -- e.g., 'transfer_over_100k', 'customer_creation', 'config_change'
    required_approval_count SMALLINT NOT NULL DEFAULT 1,
    parallel_approval_required BOOLEAN NOT NULL DEFAULT FALSE,
    timeout_hours SMALLINT DEFAULT 48,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_approval_workflows_active ON governance.approval_workflows(is_active);
CREATE INDEX idx_approval_workflows_type ON governance.approval_workflows(workflow_type);

COMMENT ON TABLE governance.approval_workflows IS 'Configurable approval workflow templates';

-- Approval Workflow Steps
CREATE TABLE IF NOT EXISTS governance.approval_workflow_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES governance.approval_workflows(id) ON DELETE CASCADE,
    step_number SMALLINT NOT NULL,
    approval_role_id UUID NOT NULL REFERENCES governance.roles(id),
    escalation_after_hours SMALLINT,
    escalation_role_id UUID REFERENCES governance.roles(id),
    auto_approve_if_no_response BOOLEAN NOT NULL DEFAULT FALSE,
    approval_authority_limit BIGINT, -- e.g., max transaction amount approver can authorize
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_workflow_steps_workflow ON governance.approval_workflow_steps(workflow_id);
CREATE INDEX idx_workflow_steps_role ON governance.approval_workflow_steps(approval_role_id);

COMMENT ON TABLE governance.approval_workflow_steps IS 'Sequential/parallel approval steps for workflows';

-- Power Delegations
CREATE TABLE IF NOT EXISTS governance.power_delegations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    delegating_user_id UUID NOT NULL,
    delegated_to_user_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES governance.roles(id),
    permission_scope JSONB, -- e.g., {department: ['main', 'branch1'], amount_limit: 100000}
    delegation_reason TEXT NOT NULL,
    delegation_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delegation_end TIMESTAMPTZ NOT NULL,
    approval_required BOOLEAN NOT NULL DEFAULT TRUE,
    approved_by UUID,
    approved_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_power_delegations_delegating ON governance.power_delegations(delegating_user_id);
CREATE INDEX idx_power_delegations_delegated_to ON governance.power_delegations(delegated_to_user_id);
CREATE INDEX idx_power_delegations_active ON governance.power_delegations(is_active, delegation_start, delegation_end);

COMMENT ON TABLE governance.power_delegations IS 'Temporary power delegations with approval tracking and scope limitations';

-- Access Reviews
CREATE TABLE IF NOT EXISTS governance.access_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    review_code VARCHAR(50) NOT NULL UNIQUE,
    review_name VARCHAR(255) NOT NULL,
    review_type VARCHAR(50) NOT NULL CHECK (review_type IN ('periodic', 'ad_hoc', 'risk_based', 'sod_verification')),
    review_frequency VARCHAR(50) CHECK (review_frequency IN ('monthly', 'quarterly', 'semi_annual', 'annual', 'on_demand')),
    scope VARCHAR(100) NOT NULL CHECK (scope IN ('all_users', 'department', 'role', 'function', 'risk_users')), -- scope of review
    scheduled_start_date TIMESTAMPTZ,
    scheduled_completion_date TIMESTAMPTZ,
    actual_start_date TIMESTAMPTZ,
    actual_completion_date TIMESTAMPTZ,
    review_status VARCHAR(50) NOT NULL DEFAULT 'scheduled' CHECK (review_status IN ('scheduled', 'in_progress', 'completed', 'cancelled')),
    findings_count SMALLINT DEFAULT 0,
    exceptions_approved SMALLINT DEFAULT 0,
    access_revoked_count SMALLINT DEFAULT 0,
    reviewed_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_access_reviews_status ON governance.access_reviews(review_status);
CREATE INDEX idx_access_reviews_type ON governance.access_reviews(review_type);
CREATE INDEX idx_access_reviews_dates ON governance.access_reviews(scheduled_start_date, scheduled_completion_date);

COMMENT ON TABLE governance.access_reviews IS 'Access review campaign tracking (periodic and ad-hoc)';

-- Access Review Findings
CREATE TABLE IF NOT EXISTS governance.access_review_findings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    access_review_id UUID NOT NULL REFERENCES governance.access_reviews(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    finding_type VARCHAR(50) NOT NULL CHECK (finding_type IN ('excessive_access', 'sod_violation', 'stale_access', 'unauthorized', 'inappropriate_delegation')),
    finding_description TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    remediation_action VARCHAR(100) NOT NULL CHECK (remediation_action IN ('revoke', 'revoke_and_review', 'retrain', 'investigate', 'exception')),
    remediation_deadline TIMESTAMPTZ,
    remediation_completed_at TIMESTAMPTZ,
    finding_status VARCHAR(50) NOT NULL DEFAULT 'open' CHECK (finding_status IN ('open', 'in_progress', 'remediated', 'accepted_risk', 'waived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_access_review_findings_review ON governance.access_review_findings(access_review_id);
CREATE INDEX idx_access_review_findings_user ON governance.access_review_findings(user_id);
CREATE INDEX idx_access_review_findings_status ON governance.access_review_findings(finding_status);

COMMENT ON TABLE governance.access_review_findings IS 'Findings and remediation tracking from access reviews';
