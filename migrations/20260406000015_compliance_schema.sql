-- BANKO Compliance BC13 (STORY-COMP-04)
-- Creates compliance schema with SMSI ISO 27001, Risk Register, PCI DSS Token Vault,
-- INPDP Consents, DPIA, Breach Notifications, and Data Rights Requests tables

CREATE SCHEMA IF NOT EXISTS compliance;

-- SMSI ISO 27001 Controls
CREATE TABLE compliance.smsi_controls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    control_ref VARCHAR(20) NOT NULL UNIQUE,
    title VARCHAR(500) NOT NULL,
    theme VARCHAR(50) NOT NULL CHECK (theme IN ('Organizational', 'People', 'Physical', 'Technological')),
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'NotImplemented' CHECK (status IN ('NotImplemented', 'Partial', 'Implemented', 'NotApplicable')),
    responsible VARCHAR(255),
    evidence TEXT,
    last_audit_date TIMESTAMPTZ,
    next_audit_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Risk Register
CREATE TABLE compliance.risk_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    risk_ref VARCHAR(20) NOT NULL UNIQUE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL CHECK (category IN ('Operational', 'Cyber', 'Regulatory', 'Financial', 'Reputational')),
    likelihood SMALLINT NOT NULL CHECK (likelihood BETWEEN 1 AND 5),
    impact SMALLINT NOT NULL CHECK (impact BETWEEN 1 AND 5),
    inherent_score SMALLINT GENERATED ALWAYS AS (likelihood * impact) STORED,
    residual_score SMALLINT NOT NULL DEFAULT 0,
    mitigations TEXT[],
    owner VARCHAR(255),
    status VARCHAR(50) NOT NULL DEFAULT 'Open' CHECK (status IN ('Open', 'Mitigated', 'Accepted', 'Closed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- PCI DSS Token Vault
CREATE TABLE compliance.token_vault (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(255) NOT NULL UNIQUE,
    masked_pan VARCHAR(19) NOT NULL,
    card_holder_encrypted BYTEA,
    expiry_month SMALLINT NOT NULL CHECK (expiry_month BETWEEN 1 AND 12),
    expiry_year SMALLINT NOT NULL,
    token_status VARCHAR(50) NOT NULL DEFAULT 'Active' CHECK (token_status IN ('Active', 'Expired', 'Revoked')),
    encryption_key_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- INPDP Consents
CREATE TABLE compliance.inpdp_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    purpose VARCHAR(50) NOT NULL,
    granted BOOLEAN NOT NULL DEFAULT false,
    granted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    expiry_date TIMESTAMPTZ,
    legal_basis VARCHAR(50) NOT NULL,
    data_categories TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- DPIA (Data Protection Impact Assessment)
CREATE TABLE compliance.dpias (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    processing_activity VARCHAR(500) NOT NULL,
    risk_assessment TEXT,
    mitigations TEXT[],
    status VARCHAR(50) NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'UnderReview', 'Approved', 'Rejected')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ
);

-- Breach Notifications
CREATE TABLE compliance.breach_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    breach_type VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    affected_data TEXT[],
    affected_count INTEGER NOT NULL DEFAULT 0,
    detected_at TIMESTAMPTZ NOT NULL,
    notified_authority_at TIMESTAMPTZ,
    notified_subjects_at TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL DEFAULT 'Detected' CHECK (status IN ('Detected', 'AuthorityNotified', 'SubjectsNotified', 'Resolved')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Data Rights Requests (portability + erasure)
CREATE TABLE compliance.data_rights_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    request_type VARCHAR(50) NOT NULL CHECK (request_type IN ('Portability', 'Erasure', 'Access', 'Rectification', 'Opposition')),
    status VARCHAR(50) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Processing', 'Completed', 'Rejected')),
    reason TEXT,
    response TEXT,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Create indexes for performance and foreign key relationships
CREATE INDEX idx_smsi_controls_control_ref ON compliance.smsi_controls(control_ref);
CREATE INDEX idx_smsi_controls_theme ON compliance.smsi_controls(theme);
CREATE INDEX idx_smsi_controls_status ON compliance.smsi_controls(status);

CREATE INDEX idx_risk_entries_risk_ref ON compliance.risk_entries(risk_ref);
CREATE INDEX idx_risk_entries_status ON compliance.risk_entries(status);
CREATE INDEX idx_risk_entries_category ON compliance.risk_entries(category);

CREATE INDEX idx_token_vault_token ON compliance.token_vault(token);
CREATE INDEX idx_token_vault_status ON compliance.token_vault(token_status);
CREATE INDEX idx_token_vault_masked_pan ON compliance.token_vault(masked_pan);

CREATE INDEX idx_inpdp_consents_customer_id ON compliance.inpdp_consents(customer_id);
CREATE INDEX idx_inpdp_consents_status ON compliance.inpdp_consents(granted);
CREATE INDEX idx_inpdp_consents_purpose ON compliance.inpdp_consents(purpose);

CREATE INDEX idx_dpias_status ON compliance.dpias(status);

CREATE INDEX idx_breach_notifications_status ON compliance.breach_notifications(status);
CREATE INDEX idx_breach_notifications_type ON compliance.breach_notifications(breach_type);

CREATE INDEX idx_data_rights_requests_customer_id ON compliance.data_rights_requests(customer_id);
CREATE INDEX idx_data_rights_requests_status ON compliance.data_rights_requests(status);
CREATE INDEX idx_data_rights_requests_type ON compliance.data_rights_requests(request_type);

-- e-KYC Biometric Verification
CREATE TABLE compliance.biometric_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    verification_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'verified', 'failed', 'expired')),
    confidence_score FLOAT NOT NULL DEFAULT 0.0 CHECK (confidence_score BETWEEN 0.0 AND 1.0),
    liveness_check BOOLEAN NOT NULL DEFAULT false,
    document_type VARCHAR(50),
    document_number VARCHAR(255),
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_biometric_verifications_customer_id ON compliance.biometric_verifications(customer_id);
CREATE INDEX idx_biometric_verifications_status ON compliance.biometric_verifications(status);
CREATE INDEX idx_biometric_verifications_verification_type ON compliance.biometric_verifications(verification_type);
