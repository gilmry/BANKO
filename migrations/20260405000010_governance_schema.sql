CREATE SCHEMA IF NOT EXISTS governance;

CREATE TABLE governance.audit_trail (
    id UUID PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    user_id UUID NOT NULL,
    action VARCHAR(20) NOT NULL,
    resource_type VARCHAR(30) NOT NULL,
    resource_id UUID NOT NULL,
    changes JSONB,
    ip_address VARCHAR(45),
    previous_hash VARCHAR(64) NOT NULL,
    hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- APPEND-ONLY: application-level enforcement (no DELETE/UPDATE in code)

CREATE TABLE governance.committees (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    committee_type VARCHAR(20) NOT NULL,
    members UUID[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE governance.committee_decisions (
    id UUID PRIMARY KEY,
    committee_id UUID NOT NULL REFERENCES governance.committees(id),
    subject TEXT NOT NULL,
    decision VARCHAR(20) NOT NULL,
    justification TEXT,
    decided_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE governance.decision_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    decision_id UUID NOT NULL REFERENCES governance.committee_decisions(id),
    member_id UUID NOT NULL,
    vote VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE governance.control_checks (
    id UUID PRIMARY KEY,
    operation_type VARCHAR(30) NOT NULL,
    operation_id UUID NOT NULL,
    checker_id UUID,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    comments TEXT,
    checked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_trail_timestamp ON governance.audit_trail(timestamp);
CREATE INDEX idx_audit_trail_user ON governance.audit_trail(user_id);
CREATE INDEX idx_audit_trail_action ON governance.audit_trail(action);
CREATE INDEX idx_audit_trail_resource ON governance.audit_trail(resource_type, resource_id);
CREATE INDEX idx_control_checks_status ON governance.control_checks(status);
CREATE INDEX idx_control_checks_operation ON governance.control_checks(operation_type, operation_id);
