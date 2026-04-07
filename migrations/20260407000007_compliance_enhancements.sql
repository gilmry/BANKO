-- BANKO Compliance BC Enhancement
-- GAFI recommendations, internal audits, compliance risks, training records, regulatory changes, incidents, whistleblower reports, third-party assessments

-- Internal Audit Program
CREATE TABLE IF NOT EXISTS compliance.internal_audits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    audit_code VARCHAR(50) NOT NULL UNIQUE,
    audit_name VARCHAR(255) NOT NULL,
    audit_description TEXT,
    audit_type VARCHAR(50) NOT NULL CHECK (audit_type IN ('financial', 'operational', 'it', 'governance', 'compliance', 'fraud', 'anti_money_laundering', 'sanctions')),
    audit_scope TEXT NOT NULL,
    planned_start_date TIMESTAMPTZ NOT NULL,
    planned_end_date TIMESTAMPTZ NOT NULL,
    actual_start_date TIMESTAMPTZ,
    actual_end_date TIMESTAMPTZ,
    audit_status VARCHAR(50) NOT NULL DEFAULT 'planned' CHECK (audit_status IN ('planned', 'scheduled', 'in_progress', 'on_hold', 'completed', 'closed')),
    audit_lead VARCHAR(255),
    audit_team VARCHAR(255)[], -- comma-separated auditor names
    internal_rating VARCHAR(20) CHECK (internal_rating IN ('excellent', 'good', 'satisfactory', 'poor', 'unsatisfactory')),
    executive_summary TEXT,
    key_findings_count SMALLINT DEFAULT 0,
    critical_findings SMALLINT DEFAULT 0,
    high_priority_findings SMALLINT DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_internal_audits_status ON compliance.internal_audits(audit_status);
CREATE INDEX idx_internal_audits_type ON compliance.internal_audits(audit_type);
CREATE INDEX idx_internal_audits_dates ON compliance.internal_audits(planned_start_date, planned_end_date);

COMMENT ON TABLE compliance.internal_audits IS 'Internal audit program and engagement tracking';

-- Audit Findings and Recommendations
CREATE TABLE IF NOT EXISTS compliance.audit_findings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    audit_id UUID NOT NULL REFERENCES compliance.internal_audits(id) ON DELETE CASCADE,
    finding_code VARCHAR(50) NOT NULL,
    finding_category VARCHAR(100) NOT NULL CHECK (finding_category IN ('governance', 'control_deficiency', 'process_improvement', 'regulatory_gap', 'fraud_risk', 'it_risk', 'operational_risk')),
    finding_title VARCHAR(255) NOT NULL,
    finding_description TEXT NOT NULL,
    root_cause TEXT,
    potential_impact TEXT,
    severity_rating VARCHAR(20) NOT NULL CHECK (severity_rating IN ('critical', 'high', 'medium', 'low')),
    responsible_department VARCHAR(100),
    management_action_plan TEXT,
    target_remediation_date TIMESTAMPTZ,
    actual_remediation_date TIMESTAMPTZ,
    finding_status VARCHAR(50) NOT NULL DEFAULT 'open' CHECK (finding_status IN ('open', 'in_progress', 'remediated', 'closed', 'not_applicable')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_findings_audit ON compliance.audit_findings(audit_id);
CREATE INDEX idx_audit_findings_status ON compliance.audit_findings(finding_status);
CREATE INDEX idx_audit_findings_severity ON compliance.audit_findings(severity_rating);

COMMENT ON TABLE compliance.audit_findings IS 'Audit findings with management action plans and remediation tracking';

-- Compliance Risk Register
CREATE TABLE IF NOT EXISTS compliance.compliance_risks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    risk_code VARCHAR(50) NOT NULL UNIQUE,
    risk_title VARCHAR(255) NOT NULL,
    risk_description TEXT,
    risk_category VARCHAR(100) NOT NULL CHECK (risk_category IN ('regulatory', 'aml', 'sanctions', 'fraud', 'operational', 'reputational', 'data_protection', 'third_party')),
    gafi_recommendation VARCHAR(50), -- FATF Recommendation e.g., 'R4', 'R10', 'R15'
    likelihood_rating VARCHAR(20) NOT NULL CHECK (likelihood_rating IN ('remote', 'low', 'medium', 'high', 'almost_certain')),
    impact_rating VARCHAR(20) NOT NULL CHECK (impact_rating IN ('negligible', 'low', 'medium', 'high', 'catastrophic')),
    risk_score SMALLINT NOT NULL DEFAULT 0,
    current_mitigation TEXT,
    additional_controls TEXT,
    responsible_department VARCHAR(100),
    risk_owner VARCHAR(255),
    risk_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (risk_status IN ('active', 'mitigated', 'accepted', 'transferred', 'avoided', 'closed')),
    next_review_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_compliance_risks_category ON compliance.compliance_risks(risk_category);
CREATE INDEX idx_compliance_risks_status ON compliance.compliance_risks(risk_status);
CREATE INDEX idx_compliance_risks_gafi ON compliance.compliance_risks(gafi_recommendation);

COMMENT ON TABLE compliance.compliance_risks IS 'Compliance risk register mapped to FATF GAFI recommendations';

-- Regulatory Changes and Tracking
CREATE TABLE IF NOT EXISTS compliance.regulatory_changes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    change_code VARCHAR(50) NOT NULL UNIQUE,
    regulation_name VARCHAR(255) NOT NULL,
    issuing_body VARCHAR(100) NOT NULL, -- BCT, INPDP, CNC, CNCM
    change_type VARCHAR(50) NOT NULL CHECK (change_type IN ('new_law', 'amendment', 'guidance', 'circular', 'instruction', 'requirement')),
    effective_date TIMESTAMPTZ NOT NULL,
    summary_en TEXT NOT NULL,
    summary_fr TEXT,
    summary_ar TEXT,
    implementation_impact TEXT,
    systems_affected VARCHAR(100)[],
    responsible_department VARCHAR(100),
    implementation_status VARCHAR(50) NOT NULL DEFAULT 'not_started' CHECK (implementation_status IN ('not_started', 'in_progress', 'completed', 'on_hold', 'not_applicable')),
    implementation_deadline TIMESTAMPTZ,
    completion_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_regulatory_changes_effective ON compliance.regulatory_changes(effective_date);
CREATE INDEX idx_regulatory_changes_status ON compliance.regulatory_changes(implementation_status);
CREATE INDEX idx_regulatory_changes_issuer ON compliance.regulatory_changes(issuing_body);

COMMENT ON TABLE compliance.regulatory_changes IS 'Regulatory changes from BCT, INPDP, CNC, CNCM with implementation tracking';

-- Compliance Training Records
CREATE TABLE IF NOT EXISTS compliance.compliance_training (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    training_code VARCHAR(50) NOT NULL UNIQUE,
    training_title VARCHAR(255) NOT NULL,
    training_type VARCHAR(50) NOT NULL CHECK (training_type IN ('aml', 'sanctions', 'fraud', 'data_protection', 'governance', 'code_of_conduct', 'it_security', 'mandatory', 'specialized')),
    training_description TEXT,
    required_for_roles VARCHAR(100)[],
    training_provider VARCHAR(255),
    training_duration_hours DECIMAL(5, 2),
    training_format VARCHAR(50) CHECK (training_format IN ('in_person', 'online', 'self_paced', 'hybrid')),
    competency_assessment_required BOOLEAN NOT NULL DEFAULT FALSE,
    passing_score_required SMALLINT,
    annual_recertification_required BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_compliance_training_type ON compliance.compliance_training(training_type);

COMMENT ON TABLE compliance.compliance_training IS 'Compliance training program definitions';

-- Compliance Training Completions
CREATE TABLE IF NOT EXISTS compliance.training_completions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    training_id UUID NOT NULL REFERENCES compliance.compliance_training(id) ON DELETE CASCADE,
    completion_date TIMESTAMPTZ NOT NULL,
    assessment_score SMALLINT,
    assessment_passed BOOLEAN,
    certificate_issued BOOLEAN NOT NULL DEFAULT FALSE,
    certificate_expires_at TIMESTAMPTZ,
    due_for_renewal_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_training_completions_user ON compliance.training_completions(user_id);
CREATE INDEX idx_training_completions_training ON compliance.training_completions(training_id);
CREATE INDEX idx_training_completions_expires ON compliance.training_completions(certificate_expires_at);

COMMENT ON TABLE compliance.training_completions IS 'Training completion and certification records per user';

-- Compliance Incidents and Reports
CREATE TABLE IF NOT EXISTS compliance.compliance_incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_code VARCHAR(50) NOT NULL UNIQUE,
    incident_type VARCHAR(50) NOT NULL CHECK (incident_type IN ('breach', 'violation', 'fraud', 'operational_error', 'third_party_failure', 'system_failure', 'aml_alert', 'sanctions_hit')),
    incident_title VARCHAR(255) NOT NULL,
    incident_description TEXT NOT NULL,
    discovered_date TIMESTAMPTZ NOT NULL,
    reported_date TIMESTAMPTZ NOT NULL,
    incident_status VARCHAR(50) NOT NULL DEFAULT 'open' CHECK (incident_status IN ('open', 'investigating', 'resolved', 'escalated', 'closed')),
    severity_level VARCHAR(20) NOT NULL CHECK (severity_level IN ('low', 'medium', 'high', 'critical')),
    affected_customers BIGINT,
    potential_loss_amount BIGINT,
    actual_loss_amount BIGINT,
    investigation_findings TEXT,
    root_cause TEXT,
    corrective_actions TEXT,
    regulatory_notification_required BOOLEAN NOT NULL DEFAULT FALSE,
    regulatory_notification_date TIMESTAMPTZ,
    public_disclosure_required BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_compliance_incidents_type ON compliance.compliance_incidents(incident_type);
CREATE INDEX idx_compliance_incidents_status ON compliance.compliance_incidents(incident_status);
CREATE INDEX idx_compliance_incidents_severity ON compliance.compliance_incidents(severity_level);
CREATE INDEX idx_compliance_incidents_date ON compliance.compliance_incidents(discovered_date);

COMMENT ON TABLE compliance.compliance_incidents IS 'Compliance incident tracking and escalation management';

-- Whistleblower Reports
CREATE TABLE IF NOT EXISTS compliance.whistleblower_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(50) NOT NULL UNIQUE,
    report_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reporter_type VARCHAR(50) NOT NULL CHECK (reporter_type IN ('employee', 'external', 'anonymous')),
    reporter_name VARCHAR(255), -- NULL if anonymous
    reporter_email VARCHAR(255),
    subject_line VARCHAR(500) NOT NULL,
    report_description TEXT NOT NULL,
    report_category VARCHAR(100) NOT NULL CHECK (report_category IN ('fraud', 'misconduct', 'regulatory_violation', 'harassment', 'discrimination', 'retaliation', 'other')),
    is_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    is_confidential BOOLEAN NOT NULL DEFAULT TRUE,
    report_status VARCHAR(50) NOT NULL DEFAULT 'new' CHECK (report_status IN ('new', 'under_review', 'under_investigation', 'resolved', 'closed', 'no_action')),
    assigned_investigator VARCHAR(255),
    investigation_findings TEXT,
    investigation_complete_date TIMESTAMPTZ,
    disciplinary_action_taken BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_whistleblower_reports_status ON compliance.whistleblower_reports(report_status);
CREATE INDEX idx_whistleblower_reports_category ON compliance.whistleblower_reports(report_category);
CREATE INDEX idx_whistleblower_reports_date ON compliance.whistleblower_reports(report_date);

COMMENT ON TABLE compliance.whistleblower_reports IS 'Anonymous and confidential whistleblower reporting mechanism';

-- Third-Party Compliance Assessments
CREATE TABLE IF NOT EXISTS compliance.third_party_assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL,
    vendor_name VARCHAR(255) NOT NULL,
    vendor_category VARCHAR(100) NOT NULL CHECK (vendor_category IN ('payment_processor', 'technology_provider', 'outsourcing', 'consulting', 'audit', 'insurance', 'other')),
    assessment_type VARCHAR(50) NOT NULL CHECK (assessment_type IN ('aml_kyc', 'security', 'operational', 'compliance', 'financial', 'full_due_diligence')),
    assessment_date TIMESTAMPTZ NOT NULL,
    assessment_scope TEXT,
    assessor_name VARCHAR(255),
    overall_rating VARCHAR(20) CHECK (overall_rating IN ('excellent', 'good', 'satisfactory', 'poor', 'unacceptable')),
    key_findings TEXT,
    critical_issues_count SMALLINT DEFAULT 0,
    remediation_required BOOLEAN NOT NULL DEFAULT FALSE,
    remediation_deadline TIMESTAMPTZ,
    next_assessment_date TIMESTAMPTZ,
    assessment_status VARCHAR(50) NOT NULL DEFAULT 'completed' CHECK (assessment_status IN ('planned', 'in_progress', 'completed', 'on_hold')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_third_party_assessments_vendor ON compliance.third_party_assessments(vendor_id);
CREATE INDEX idx_third_party_assessments_category ON compliance.third_party_assessments(vendor_category);
CREATE INDEX idx_third_party_assessments_date ON compliance.third_party_assessments(assessment_date);

COMMENT ON TABLE compliance.third_party_assessments IS 'Third-party and vendor compliance assessments and due diligence';
