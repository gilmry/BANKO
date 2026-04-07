-- BANKO Data Hub BC Schema
-- Master data management, data quality, lineage, reconciliation, and governance

CREATE SCHEMA IF NOT EXISTS data_hub;

-- Data Entities (Master Data Records)
CREATE TABLE IF NOT EXISTS data_hub.data_entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_code VARCHAR(50) NOT NULL UNIQUE,
    entity_type VARCHAR(100) NOT NULL CHECK (entity_type IN ('customer', 'counterparty', 'product', 'account', 'organization', 'security', 'branch', 'other')),
    entity_name VARCHAR(255) NOT NULL,
    entity_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (entity_status IN ('active', 'inactive', 'suspended', 'merged', 'archived')),
    primary_system_of_record VARCHAR(100) NOT NULL, -- System where master data lives
    integration_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (integration_status IN ('pending', 'integrated', 'partially_integrated', 'failed')),
    golden_record_id UUID, -- Reference to canonical record
    last_golden_record_update TIMESTAMPTZ,
    data_quality_score DECIMAL(10, 8), -- 0.0 - 1.0
    completeness_percentage DECIMAL(10, 8),
    accuracy_percentage DECIMAL(10, 8),
    consistency_percentage DECIMAL(10, 8),
    timeliness_percentage DECIMAL(10, 8),
    last_validation_date TIMESTAMPTZ,
    validation_status VARCHAR(50) CHECK (validation_status IN ('validated', 'invalid', 'pending_review')),
    validation_errors JSONB, -- Array of validation issues
    last_sync_date TIMESTAMPTZ,
    sync_frequency VARCHAR(50) DEFAULT 'daily' CHECK (sync_frequency IN ('real_time', 'hourly', 'daily', 'weekly', 'monthly')),
    source_systems JSONB, -- Array of source systems contributing data
    version_number INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255)
);

CREATE INDEX idx_data_entities_type ON data_hub.data_entities(entity_type);
CREATE INDEX idx_data_entities_status ON data_hub.data_entities(entity_status);
CREATE INDEX idx_data_entities_golden ON data_hub.data_entities(golden_record_id);
CREATE INDEX idx_data_entities_system ON data_hub.data_entities(primary_system_of_record);

COMMENT ON TABLE data_hub.data_entities IS 'Master data records with quality scoring and lineage';

-- Data Quality Rules
CREATE TABLE IF NOT EXISTS data_hub.data_quality_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_code VARCHAR(50) NOT NULL UNIQUE,
    entity_type VARCHAR(100) NOT NULL,
    rule_name VARCHAR(255) NOT NULL,
    rule_description TEXT,
    rule_type VARCHAR(50) NOT NULL CHECK (rule_type IN ('completeness', 'uniqueness', 'validity', 'consistency', 'timeliness', 'accuracy', 'referential_integrity')),
    rule_expression TEXT NOT NULL, -- SQL or domain-specific language
    severity_level VARCHAR(50) NOT NULL DEFAULT 'warning' CHECK (severity_level IN ('critical', 'high', 'medium', 'low', 'warning')),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    rule_owner VARCHAR(255),
    rule_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (rule_status IN ('active', 'inactive', 'suspended', 'under_review')),
    last_execution_date TIMESTAMPTZ,
    last_execution_status VARCHAR(50) CHECK (last_execution_status IN ('passed', 'failed', 'error')),
    violations_count INTEGER DEFAULT 0,
    critical_violations INTEGER DEFAULT 0,
    exemption_allowed BOOLEAN DEFAULT FALSE,
    exemption_approval_required BOOLEAN DEFAULT FALSE,
    remediation_steps TEXT,
    sla_compliance_percentage DECIMAL(10, 8),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_quality_rules_type ON data_hub.data_quality_rules(entity_type);
CREATE INDEX idx_data_quality_rules_status ON data_hub.data_quality_rules(rule_status);
CREATE INDEX idx_data_quality_rules_severity ON data_hub.data_quality_rules(severity_level);

COMMENT ON TABLE data_hub.data_quality_rules IS 'Data quality validation rules and thresholds';

-- Data Lineage
CREATE TABLE IF NOT EXISTS data_hub.data_lineage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lineage_code VARCHAR(50) NOT NULL UNIQUE,
    source_entity_id UUID REFERENCES data_hub.data_entities(id),
    source_entity_type VARCHAR(100),
    source_system VARCHAR(100),
    source_field_name VARCHAR(255),
    target_entity_id UUID REFERENCES data_hub.data_entities(id),
    target_entity_type VARCHAR(100),
    target_system VARCHAR(100),
    target_field_name VARCHAR(255),
    transformation_logic TEXT, -- Description or code
    transformation_type VARCHAR(50) CHECK (transformation_type IN ('mapping', 'enrichment', 'calculation', 'aggregation', 'filtering')),
    data_flow_direction VARCHAR(20) NOT NULL CHECK (data_flow_direction IN ('upstream', 'downstream')),
    flow_frequency VARCHAR(50) DEFAULT 'daily' CHECK (flow_frequency IN ('real_time', 'hourly', 'daily', 'weekly')),
    lineage_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (lineage_status IN ('active', 'deprecated', 'retired')),
    quality_impact_score DECIMAL(10, 8), -- 0.0 - 1.0
    last_data_flow TIMESTAMPTZ,
    flow_count BIGINT DEFAULT 0,
    error_count BIGINT DEFAULT 0,
    average_latency_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_lineage_source ON data_hub.data_lineage(source_entity_id);
CREATE INDEX idx_data_lineage_target ON data_hub.data_lineage(target_entity_id);
CREATE INDEX idx_data_lineage_status ON data_hub.data_lineage(lineage_status);

COMMENT ON TABLE data_hub.data_lineage IS 'Data flow and transformation lineage tracking';

-- Data Reconciliations
CREATE TABLE IF NOT EXISTS data_hub.data_reconciliations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reconciliation_code VARCHAR(50) NOT NULL UNIQUE,
    reconciliation_date DATE NOT NULL,
    reconciliation_type VARCHAR(50) NOT NULL CHECK (reconciliation_type IN ('balance', 'transaction', 'master_data', 'cross_system', 'period_end', 'ad_hoc')),
    source_system VARCHAR(100) NOT NULL,
    target_system VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100),
    records_matched BIGINT NOT NULL,
    records_unmatched_source BIGINT NOT NULL,
    records_unmatched_target BIGINT NOT NULL,
    variance_amount BIGINT, -- In cents
    variance_percentage DECIMAL(10, 8),
    tolerance_limit DECIMAL(10, 8),
    is_within_tolerance BOOLEAN,
    reconciliation_status VARCHAR(50) NOT NULL DEFAULT 'in_progress' CHECK (reconciliation_status IN ('in_progress', 'matched', 'unmatched_review', 'approved', 'rejected')),
    discrepancies_count INTEGER,
    discrepancies JSONB, -- Array of {field, source_value, target_value, variance}
    reconciliation_owner VARCHAR(255),
    approval_status VARCHAR(50) CHECK (approval_status IN ('pending', 'approved', 'rejected')),
    approved_by VARCHAR(255),
    approval_date TIMESTAMPTZ,
    reconciliation_notes TEXT,
    resolution_status VARCHAR(50) CHECK (resolution_status IN ('pending', 'in_progress', 'resolved', 'escalated')),
    resolution_deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_reconciliations_date ON data_hub.data_reconciliations(reconciliation_date);
CREATE INDEX idx_data_reconciliations_type ON data_hub.data_reconciliations(reconciliation_type);
CREATE INDEX idx_data_reconciliations_status ON data_hub.data_reconciliations(reconciliation_status);
CREATE INDEX idx_data_reconciliations_systems ON data_hub.data_reconciliations(source_system, target_system);

COMMENT ON TABLE data_hub.data_reconciliations IS 'System-to-system data reconciliation results and discrepancies';

-- Master Data Records (Golden Records)
CREATE TABLE IF NOT EXISTS data_hub.master_data_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    master_record_code VARCHAR(50) NOT NULL UNIQUE,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    entity_name VARCHAR(255) NOT NULL,
    canonical_data JSONB NOT NULL, -- Complete canonical representation
    data_version INTEGER NOT NULL DEFAULT 1,
    version_date TIMESTAMPTZ NOT NULL,
    previous_version_id UUID REFERENCES data_hub.master_data_records(id),
    record_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (record_status IN ('active', 'superseded', 'archived', 'deleted')),
    effective_from_date DATE NOT NULL,
    effective_to_date DATE,
    steward_name VARCHAR(255),
    steward_department VARCHAR(100),
    governance_status VARCHAR(50) CHECK (governance_status IN ('pending_approval', 'approved', 'rejected', 'under_review')),
    change_reason TEXT,
    change_request_id VARCHAR(50),
    source_records JSONB, -- Array of contributing source records
    validation_status VARCHAR(50) CHECK (validation_status IN ('valid', 'invalid', 'under_review')),
    validation_errors JSONB,
    last_validated_at TIMESTAMPTZ,
    audit_trail JSONB, -- Array of changes with who/when/why
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255)
);

CREATE INDEX idx_master_data_records_entity ON data_hub.master_data_records(entity_type, entity_id);
CREATE INDEX idx_master_data_records_status ON data_hub.master_data_records(record_status);
CREATE INDEX idx_master_data_records_version ON data_hub.master_data_records(entity_type, data_version);

COMMENT ON TABLE data_hub.master_data_records IS 'Golden record versions with full history and governance';

-- Data Governance Policies
CREATE TABLE IF NOT EXISTS data_hub.data_governance_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_code VARCHAR(50) NOT NULL UNIQUE,
    policy_name VARCHAR(255) NOT NULL,
    policy_description TEXT,
    entity_type VARCHAR(100),
    policy_type VARCHAR(50) NOT NULL CHECK (policy_type IN ('ownership', 'stewardship', 'usage', 'retention', 'privacy', 'security', 'access', 'quality')),
    policy_owner VARCHAR(255) NOT NULL,
    policy_owner_department VARCHAR(100),
    effective_date DATE NOT NULL,
    expiry_date DATE,
    policy_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (policy_status IN ('draft', 'active', 'under_review', 'superseded', 'archived')),
    approval_status VARCHAR(50) CHECK (approval_status IN ('pending', 'approved', 'rejected')),
    approved_by VARCHAR(255),
    approval_date TIMESTAMPTZ,
    policy_version INTEGER NOT NULL DEFAULT 1,
    change_log JSONB,
    compliance_scope TEXT, -- Description of who/what this applies to
    enforcement_level VARCHAR(50) CHECK (enforcement_level IN ('mandatory', 'recommended', 'guidance')),
    violation_consequences TEXT,
    audit_frequency VARCHAR(50) CHECK (audit_frequency IN ('monthly', 'quarterly', 'semi_annual', 'annual')),
    last_audit_date DATE,
    last_audit_result VARCHAR(50) CHECK (last_audit_result IN ('passed', 'failed', 'partial')),
    related_policies JSONB, -- Array of related policy IDs
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_governance_policies_type ON data_hub.data_governance_policies(policy_type);
CREATE INDEX idx_data_governance_policies_owner ON data_hub.data_governance_policies(policy_owner);
CREATE INDEX idx_data_governance_policies_status ON data_hub.data_governance_policies(policy_status);

COMMENT ON TABLE data_hub.data_governance_policies IS 'Data governance policies and compliance requirements';
