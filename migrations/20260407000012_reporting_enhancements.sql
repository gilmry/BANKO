-- BANKO Reporting BC Enhancement
-- Scheduled reports, report distributions, report archives, ad-hoc reports, tax reports, IFRS9 reports

-- Scheduled Report Definitions
CREATE TABLE IF NOT EXISTS reporting.scheduled_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(50) NOT NULL UNIQUE,
    report_name VARCHAR(255) NOT NULL,
    report_type VARCHAR(50) NOT NULL CHECK (report_type IN ('regulatory', 'management', 'financial', 'operational', 'compliance', 'risk', 'tax', 'audit', 'statutory')),
    report_category VARCHAR(100) NOT NULL,
    report_description TEXT,
    report_frequency VARCHAR(50) NOT NULL CHECK (report_frequency IN ('daily', 'weekly', 'bi_weekly', 'monthly', 'quarterly', 'semi_annual', 'annual', 'on_demand')),
    schedule_expression VARCHAR(100), -- Cron-like expression: "0 9 * * 1" = Monday 9am
    report_format VARCHAR(50) NOT NULL CHECK (report_format IN ('pdf', 'excel', 'csv', 'html', 'json', 'xml')),
    file_naming_pattern VARCHAR(255), -- e.g., 'report_{date}_v{version}'
    data_source_query VARCHAR(500), -- Reference to data source or SQL
    template_id UUID,
    required_parameters JSONB, -- Parameters passed to report generator
    recipient_distribution_list_id UUID,
    regulatory_requirement VARCHAR(100), -- e.g., 'BCBS_239', 'IFRS9', 'BCN_Circular'
    report_owner VARCHAR(255),
    responsible_department VARCHAR(100),
    sla_hours SMALLINT DEFAULT 24,
    is_automated BOOLEAN NOT NULL DEFAULT TRUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    next_scheduled_run TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_scheduled_reports_type ON reporting.scheduled_reports(report_type);
CREATE INDEX idx_scheduled_reports_frequency ON reporting.scheduled_reports(report_frequency);
CREATE INDEX idx_scheduled_reports_active ON reporting.scheduled_reports(is_active);

COMMENT ON TABLE reporting.scheduled_reports IS 'Scheduled report definitions with frequency and distribution';

-- Report Execution History
CREATE TABLE IF NOT EXISTS reporting.report_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scheduled_report_id UUID NOT NULL REFERENCES reporting.scheduled_reports(id) ON DELETE CASCADE,
    execution_code VARCHAR(50) NOT NULL UNIQUE,
    execution_date TIMESTAMPTZ NOT NULL,
    execution_start_time TIMESTAMPTZ,
    execution_end_time TIMESTAMPTZ,
    execution_duration_seconds BIGINT,
    generated_file_path VARCHAR(500),
    file_format VARCHAR(50),
    file_size_bytes BIGINT,
    record_count BIGINT DEFAULT 0,
    execution_status VARCHAR(50) NOT NULL DEFAULT 'scheduled' CHECK (execution_status IN ('scheduled', 'queued', 'in_progress', 'completed', 'failed', 'partially_failed')),
    error_message TEXT,
    error_details TEXT,
    data_completeness_percentage DECIMAL(5, 2) DEFAULT 100.0,
    data_quality_check_passed BOOLEAN NOT NULL DEFAULT TRUE,
    quality_issues TEXT,
    executed_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_report_executions_scheduled_report ON reporting.report_executions(scheduled_report_id);
CREATE INDEX idx_report_executions_date ON reporting.report_executions(execution_date);
CREATE INDEX idx_report_executions_status ON reporting.report_executions(execution_status);

COMMENT ON TABLE reporting.report_executions IS 'Report execution history with performance and quality metrics';

-- Report Distribution Lists
CREATE TABLE IF NOT EXISTS reporting.report_distribution_lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    distribution_list_code VARCHAR(50) NOT NULL UNIQUE,
    distribution_list_name VARCHAR(255) NOT NULL,
    description TEXT,
    report_id UUID REFERENCES reporting.scheduled_reports(id) ON DELETE SET NULL,
    distribution_method VARCHAR(50) NOT NULL CHECK (distribution_method IN ('email', 'secure_download', 'api', 'sftp', 'portal', 'print')),
    recipient_emails VARCHAR(255)[],
    recipient_departments VARCHAR(100)[],
    cc_recipients VARCHAR(255)[],
    bcc_recipients VARCHAR(255)[],
    report_access_level VARCHAR(50) NOT NULL CHECK (report_access_level IN ('public', 'internal', 'confidential', 'restricted')),
    encryption_required BOOLEAN NOT NULL DEFAULT FALSE,
    digital_signature_required BOOLEAN NOT NULL DEFAULT FALSE,
    delivery_notification_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    read_receipt_requested BOOLEAN NOT NULL DEFAULT FALSE,
    retention_days SMALLINT DEFAULT 365,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_distribution_lists_report ON reporting.report_distribution_lists(report_id);
CREATE INDEX idx_distribution_lists_active ON reporting.report_distribution_lists(is_active);

COMMENT ON TABLE reporting.report_distribution_lists IS 'Report distribution and recipient management';

-- Report Delivery Tracking
CREATE TABLE IF NOT EXISTS reporting.report_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_execution_id UUID NOT NULL REFERENCES reporting.report_executions(id) ON DELETE CASCADE,
    distribution_list_id UUID NOT NULL REFERENCES reporting.report_distribution_lists(id),
    delivery_code VARCHAR(50) NOT NULL UNIQUE,
    recipient_email VARCHAR(255) NOT NULL,
    delivery_method VARCHAR(50) NOT NULL,
    scheduled_delivery_time TIMESTAMPTZ,
    actual_delivery_time TIMESTAMPTZ,
    delivery_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (delivery_status IN ('pending', 'sent', 'delivered', 'read', 'failed', 'bounced', 'rejected')),
    delivery_attempts SMALLINT DEFAULT 1,
    last_attempt_time TIMESTAMPTZ,
    failure_reason TEXT,
    read_at TIMESTAMPTZ,
    download_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_report_deliveries_execution ON reporting.report_deliveries(report_execution_id);
CREATE INDEX idx_report_deliveries_status ON reporting.report_deliveries(delivery_status);
CREATE INDEX idx_report_deliveries_email ON reporting.report_deliveries(recipient_email);

COMMENT ON TABLE reporting.report_deliveries IS 'Report delivery tracking with status and read receipts';

-- Ad-Hoc Report Requests
CREATE TABLE IF NOT EXISTS reporting.adhoc_report_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_code VARCHAR(50) NOT NULL UNIQUE,
    requestor_id UUID NOT NULL,
    requestor_department VARCHAR(100),
    request_date TIMESTAMPTZ NOT NULL,
    report_description TEXT NOT NULL,
    report_criteria JSONB NOT NULL, -- Filters, date ranges, etc.
    report_format VARCHAR(50) NOT NULL,
    urgency_level VARCHAR(50) NOT NULL CHECK (urgency_level IN ('routine', 'urgent', 'critical')),
    requested_delivery_date TIMESTAMPTZ,
    request_status VARCHAR(50) NOT NULL DEFAULT 'submitted' CHECK (request_status IN ('submitted', 'approved', 'in_progress', 'completed', 'rejected', 'cancelled')),
    approval_status VARCHAR(50) DEFAULT 'pending' CHECK (approval_status IN ('pending', 'approved', 'rejected')),
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    rejection_reason TEXT,
    report_execution_id UUID REFERENCES reporting.report_executions(id),
    completion_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_adhoc_reports_requestor ON reporting.adhoc_report_requests(requestor_id);
CREATE INDEX idx_adhoc_reports_status ON reporting.adhoc_report_requests(request_status);
CREATE INDEX idx_adhoc_reports_date ON reporting.adhoc_report_requests(request_date);

COMMENT ON TABLE reporting.adhoc_report_requests IS 'Ad-hoc report request submissions with approval workflow';

-- Report Archive and Retention
CREATE TABLE IF NOT EXISTS reporting.report_archives (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_execution_id UUID NOT NULL REFERENCES reporting.report_executions(id) ON DELETE CASCADE,
    archive_code VARCHAR(50) NOT NULL UNIQUE,
    archive_location VARCHAR(500) NOT NULL, -- S3, MinIO, or file path
    archive_format VARCHAR(50), -- Original format or compressed
    archive_size_bytes BIGINT,
    archive_hash VARCHAR(255), -- SHA256 for integrity
    archive_date TIMESTAMPTZ NOT NULL,
    archive_owner VARCHAR(255),
    retention_end_date TIMESTAMPTZ,
    legal_hold_status VARCHAR(50) CHECK (legal_hold_status IN ('none', 'active', 'pending_review')),
    is_encrypted BOOLEAN NOT NULL DEFAULT FALSE,
    encryption_key_id VARCHAR(100),
    accessible_until TIMESTAMPTZ,
    access_count BIGINT DEFAULT 0,
    last_accessed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_report_archives_execution ON reporting.report_archives(report_execution_id);
CREATE INDEX idx_report_archives_retention ON reporting.report_archives(retention_end_date);

COMMENT ON TABLE reporting.report_archives IS 'Report archival and retention management';

-- Tax Reports
CREATE TABLE IF NOT EXISTS reporting.tax_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tax_report_code VARCHAR(50) NOT NULL UNIQUE,
    tax_report_type VARCHAR(50) NOT NULL CHECK (tax_report_type IN ('annual_return', 'quarterly_return', 'withholding_tax', 'property_tax', 'vat_return', 'corporate_tax')),
    tax_year SMALLINT NOT NULL,
    tax_quarter SMALLINT, -- 1-4 for quarterly returns
    reporting_entity VARCHAR(255) NOT NULL,
    entity_tax_id VARCHAR(50),
    total_income BIGINT,
    total_expenses BIGINT,
    taxable_income BIGINT,
    tax_calculated BIGINT,
    tax_paid BIGINT,
    tax_due BIGINT,
    filing_date TIMESTAMPTZ,
    filing_deadline TIMESTAMPTZ,
    filed_with_authority VARCHAR(100), -- Tax authority name
    filing_reference_number VARCHAR(100),
    filing_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (filing_status IN ('draft', 'prepared', 'filed', 'accepted', 'rejected', 'under_audit')),
    audit_status VARCHAR(50),
    audit_findings TEXT,
    adjustments_made BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tax_reports_type ON reporting.tax_reports(tax_report_type);
CREATE INDEX idx_tax_reports_year ON reporting.tax_reports(tax_year);
CREATE INDEX idx_tax_reports_status ON reporting.tax_reports(filing_status);

COMMENT ON TABLE reporting.tax_reports IS 'Tax reporting and filing records (annual, quarterly, withholding)';

-- IFRS 9 Expected Credit Loss (ECL) Reports
CREATE TABLE IF NOT EXISTS reporting.ifrs9_ecl_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(50) NOT NULL UNIQUE,
    reporting_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL, -- e.g., '2026-Q1'
    ecl_methodology VARCHAR(50) NOT NULL CHECK (ecl_methodology IN ('simplified_approach', 'general_approach_stage_1', 'general_approach_stage_2', 'general_approach_stage_3')),
    total_exposure_at_default BIGINT NOT NULL,
    probability_of_default_percentage DECIMAL(10, 6),
    loss_given_default_percentage DECIMAL(10, 6),
    stage_1_exposure BIGINT,
    stage_1_ecl_amount BIGINT,
    stage_2_exposure BIGINT,
    stage_2_ecl_amount BIGINT,
    stage_3_exposure BIGINT, -- Default
    stage_3_ecl_amount BIGINT,
    total_allowance_for_ecl BIGINT,
    coverage_ratio DECIMAL(10, 4), -- ECL / Total Exposure
    year_over_year_change DECIMAL(10, 4),
    forward_looking_information_applied BOOLEAN NOT NULL DEFAULT FALSE,
    fli_description TEXT,
    macroeconomic_scenarios JSONB, -- Base, downside, upside scenarios
    report_approver VARCHAR(255),
    approval_date TIMESTAMPTZ,
    report_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (report_status IN ('draft', 'review', 'approved', 'submitted', 'audited')),
    audit_findings TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ifrs9_ecl_reports_date ON reporting.ifrs9_ecl_reports(reporting_date);
CREATE INDEX idx_ifrs9_ecl_reports_status ON reporting.ifrs9_ecl_reports(report_status);

COMMENT ON TABLE reporting.ifrs9_ecl_reports IS 'IFRS 9 Expected Credit Loss (ECL) stage 1/2/3 calculations and reporting';

-- IFRS 9 Financial Instrument Classification Reports
CREATE TABLE IF NOT EXISTS reporting.ifrs9_classification_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(50) NOT NULL UNIQUE,
    reporting_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL,
    instrument_type VARCHAR(50) NOT NULL CHECK (instrument_type IN ('loans', 'securities', 'derivatives', 'debt', 'equity')),
    amortized_cost_amount BIGINT,
    fvoci_amount BIGINT, -- Fair value through OCI
    fvpl_amount BIGINT, -- Fair value through P&L
    equity_instruments_amount BIGINT,
    reclassifications BIGINT, -- Amounts reclassified between categories
    reclassification_reason TEXT,
    hedge_accounting_applied BOOLEAN NOT NULL DEFAULT FALSE,
    hedge_effectiveness_percentage DECIMAL(5, 2),
    total_fair_value_adjustments BIGINT,
    impairment_adjustments BIGINT,
    report_preparer VARCHAR(255),
    report_approver VARCHAR(255),
    approval_date TIMESTAMPTZ,
    report_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (report_status IN ('draft', 'review', 'approved', 'finalized')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ifrs9_classification_date ON reporting.ifrs9_classification_reports(reporting_date);
CREATE INDEX idx_ifrs9_classification_instrument ON reporting.ifrs9_classification_reports(instrument_type);

COMMENT ON TABLE reporting.ifrs9_classification_reports IS 'IFRS 9 financial instrument classification and fair value measurement reports';
