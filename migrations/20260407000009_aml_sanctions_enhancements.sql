-- BANKO AML & Sanctions BC Enhancement
-- goAML submissions, travel rule messages, EDD profiles, AML training, PEP screening schedules, batch screening jobs, sanctions whitelist, escalation rules, sanctions reports

-- goAML Submission Records
CREATE TABLE IF NOT EXISTS aml.goaml_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    submission_code VARCHAR(50) NOT NULL UNIQUE,
    submission_date TIMESTAMPTZ NOT NULL,
    reporting_period_end TIMESTAMPTZ NOT NULL,
    submission_type VARCHAR(50) NOT NULL CHECK (submission_type IN ('ctf', 'str', 'sars', 'initial')), -- CTF=Crypto, STR=Suspicious, SARS=Serious
    total_transactions_reported BIGINT NOT NULL,
    total_amount_reported BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    submitted_by VARCHAR(255) NOT NULL,
    goaml_request_id VARCHAR(100), -- External submission ID from BCT goAML platform
    submission_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (submission_status IN ('draft', 'submitted', 'acknowledged', 'processed', 'failed', 'rejected')),
    rejection_reason TEXT,
    submission_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_goaml_submissions_date ON aml.goaml_submissions(submission_date);
CREATE INDEX idx_goaml_submissions_status ON aml.goaml_submissions(submission_status);
CREATE INDEX idx_goaml_submissions_type ON aml.goaml_submissions(submission_type);

COMMENT ON TABLE aml.goaml_submissions IS 'goAML STR/SARS submission records to BCT';

-- Travel Rule Messages (FATF Recommendation 16)
CREATE TABLE IF NOT EXISTS aml.travel_rule_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    travel_rule_ref VARCHAR(100) NOT NULL UNIQUE,
    originating_customer_id UUID NOT NULL,
    beneficiary_customer_id UUID,
    beneficiary_external_name VARCHAR(255),
    beneficiary_account VARCHAR(50),
    originating_account VARCHAR(50) NOT NULL,
    transaction_amount BIGINT NOT NULL,
    transaction_currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    transaction_date TIMESTAMPTZ NOT NULL,
    destination_country VARCHAR(100),
    message_direction VARCHAR(20) NOT NULL CHECK (message_direction IN ('outbound', 'inbound')),
    travel_rule_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (travel_rule_status IN ('pending', 'sent', 'received', 'confirmed', 'rejected', 'timeout')),
    message_content JSONB, -- IVMS101 or SWIFT gpi message
    message_hash VARCHAR(255),
    sending_timestamp TIMESTAMPTZ,
    confirmation_timestamp TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_travel_rule_messages_ref ON aml.travel_rule_messages(travel_rule_ref);
CREATE INDEX idx_travel_rule_messages_status ON aml.travel_rule_messages(travel_rule_status);
CREATE INDEX idx_travel_rule_messages_customer ON aml.travel_rule_messages(originating_customer_id);

COMMENT ON TABLE aml.travel_rule_messages IS 'Travel rule (FATF R16) message records for crypto and high-value transfers';

-- Enhanced Due Diligence (EDD) Profiles
CREATE TABLE IF NOT EXISTS aml.edd_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    edd_trigger_code VARCHAR(50) NOT NULL,
    edd_trigger_reason TEXT NOT NULL,
    edd_type VARCHAR(50) NOT NULL CHECK (edd_type IN ('high_risk_country', 'pep', 'high_net_worth', 'high_transaction_volume', 'sanctions_related', 'unusual_activity', 'industry_risk')),
    edd_initiated_date TIMESTAMPTZ NOT NULL,
    edd_initiated_by VARCHAR(255),
    beneficial_ownership_verified BOOLEAN NOT NULL DEFAULT FALSE,
    source_of_funds_verified BOOLEAN NOT NULL DEFAULT FALSE,
    politically_exposed_person_check BOOLEAN NOT NULL DEFAULT FALSE,
    complex_structure_analysis BOOLEAN NOT NULL DEFAULT FALSE,
    enhanced_monitoring_required BOOLEAN NOT NULL DEFAULT FALSE,
    edd_completion_date TIMESTAMPTZ,
    edd_completion_notes TEXT,
    edd_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (edd_status IN ('pending', 'in_progress', 'completed', 'pending_remediation', 'rejected')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_edd_profiles_customer ON aml.edd_profiles(customer_id);
CREATE INDEX idx_edd_profiles_status ON aml.edd_profiles(edd_status);
CREATE INDEX idx_edd_profiles_type ON aml.edd_profiles(edd_type);

COMMENT ON TABLE aml.edd_profiles IS 'Enhanced due diligence profiles for high-risk customers';

-- PEP Screening Schedules
CREATE TABLE IF NOT EXISTS aml.pep_screening_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schedule_code VARCHAR(50) NOT NULL UNIQUE,
    screening_frequency VARCHAR(50) NOT NULL DEFAULT 'annual' CHECK (screening_frequency IN ('quarterly', 'semi_annual', 'annual', 'on_demand')),
    next_screening_date TIMESTAMPTZ NOT NULL,
    last_screening_date TIMESTAMPTZ,
    last_screening_results_summary TEXT,
    affected_customer_count BIGINT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pep_screening_schedules_date ON aml.pep_screening_schedules(next_screening_date);
CREATE INDEX idx_pep_screening_schedules_active ON aml.pep_screening_schedules(is_active);

COMMENT ON TABLE aml.pep_screening_schedules IS 'PEP screening campaign schedules and frequencies';

-- Batch Screening Jobs
CREATE TABLE IF NOT EXISTS aml.batch_screening_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_code VARCHAR(50) NOT NULL UNIQUE,
    job_type VARCHAR(50) NOT NULL CHECK (job_type IN ('sanctions', 'pep', 'aml_watchlist', 'high_risk_countries', 'adverse_media')),
    screening_type VARCHAR(50) NOT NULL CHECK (screening_type IN ('name_match', 'fuzzy_match', 'exact_match')),
    batch_start_time TIMESTAMPTZ NOT NULL,
    batch_end_time TIMESTAMPTZ,
    records_screened BIGINT DEFAULT 0,
    matches_found BIGINT DEFAULT 0,
    false_positives BIGINT DEFAULT 0,
    job_status VARCHAR(50) NOT NULL DEFAULT 'queued' CHECK (job_status IN ('queued', 'in_progress', 'completed', 'failed', 'paused')),
    error_message TEXT,
    execution_duration_seconds BIGINT,
    external_screening_provider VARCHAR(100), -- e.g., 'refinitiv', 'lexisnexis'
    provider_job_id VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_batch_screening_jobs_status ON aml.batch_screening_jobs(job_status);
CREATE INDEX idx_batch_screening_jobs_type ON aml.batch_screening_jobs(job_type);
CREATE INDEX idx_batch_screening_jobs_date ON aml.batch_screening_jobs(batch_start_time);

COMMENT ON TABLE aml.batch_screening_jobs IS 'Batch screening job execution records for sanctions and watchlists';

-- Sanctions Whitelist (approved exceptions)
CREATE TABLE IF NOT EXISTS aml.sanctions_whitelist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    whitelist_entry_code VARCHAR(50) NOT NULL UNIQUE,
    entry_type VARCHAR(50) NOT NULL CHECK (entry_type IN ('customer', 'transaction_pattern', 'entity', 'address')),
    matched_entity_name VARCHAR(500) NOT NULL,
    original_screening_hit_id UUID,
    reason_for_whitelisting TEXT NOT NULL,
    whitelist_approved_by VARCHAR(255),
    whitelist_approval_date TIMESTAMPTZ,
    effective_from TIMESTAMPTZ NOT NULL,
    effective_to TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sanctions_whitelist_active ON aml.sanctions_whitelist(is_active);
CREATE INDEX idx_sanctions_whitelist_entry_type ON aml.sanctions_whitelist(entry_type);

COMMENT ON TABLE aml.sanctions_whitelist IS 'Approved whitelist entries to prevent false positive alerts';

-- AML/Sanctions Escalation Rules
CREATE TABLE IF NOT EXISTS aml.escalation_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_code VARCHAR(50) NOT NULL UNIQUE,
    rule_name VARCHAR(255) NOT NULL,
    rule_description TEXT,
    trigger_event VARCHAR(100) NOT NULL CHECK (trigger_event IN ('sanctions_hit', 'pep_match', 'high_risk_country', 'high_transaction_value', 'unusual_pattern', 'watchlist_match')),
    trigger_threshold BIGINT, -- Amount or count threshold
    escalation_level VARCHAR(50) NOT NULL CHECK (escalation_level IN ('level_1_reviewer', 'level_2_supervisor', 'level_3_compliance_officer', 'level_4_board')),
    escalation_sla_hours SMALLINT NOT NULL DEFAULT 24,
    auto_freeze_account BOOLEAN NOT NULL DEFAULT FALSE,
    freeze_duration_hours SMALLINT,
    notifications_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    notification_recipients VARCHAR(255)[],
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_escalation_rules_trigger ON aml.escalation_rules(trigger_event);
CREATE INDEX idx_escalation_rules_active ON aml.escalation_rules(is_active);

COMMENT ON TABLE aml.escalation_rules IS 'Automated escalation rules for sanctions and AML alerts';

-- Sanctions Regulatory Reports
CREATE TABLE IF NOT EXISTS aml.sanctions_regulatory_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(50) NOT NULL UNIQUE,
    report_type VARCHAR(50) NOT NULL CHECK (report_type IN ('monthly_report', 'quarterly_report', 'annual_report', 'incident_report', 'compliance_certification')),
    reporting_period_start TIMESTAMPTZ NOT NULL,
    reporting_period_end TIMESTAMPTZ NOT NULL,
    total_screening_hits BIGINT DEFAULT 0,
    confirmed_matches BIGINT DEFAULT 0,
    false_positives BIGINT DEFAULT 0,
    whitelist_entries BIGINT DEFAULT 0,
    accounts_frozen BIGINT DEFAULT 0,
    transactions_blocked BIGINT DEFAULT 0,
    regulatory_violations_identified BIGINT DEFAULT 0,
    report_submitted_to_bcn BOOLEAN NOT NULL DEFAULT FALSE,
    report_submission_date TIMESTAMPTZ,
    submitted_by VARCHAR(255),
    report_content TEXT,
    report_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (report_status IN ('draft', 'review', 'approved', 'submitted', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sanctions_reports_period ON aml.sanctions_regulatory_reports(reporting_period_start, reporting_period_end);
CREATE INDEX idx_sanctions_reports_status ON aml.sanctions_regulatory_reports(report_status);

COMMENT ON TABLE aml.sanctions_regulatory_reports IS 'Sanctions and AML regulatory report submissions';

-- AML Training Records (complementary to compliance.compliance_training)
CREATE TABLE IF NOT EXISTS aml.aml_training_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    training_date TIMESTAMPTZ NOT NULL,
    training_type VARCHAR(50) NOT NULL CHECK (training_type IN ('initial', 'refresher', 'specialized', 'incident_specific')),
    training_duration_hours DECIMAL(5, 2),
    trainer_name VARCHAR(255),
    assessment_score SMALLINT,
    assessment_passed BOOLEAN,
    certificate_valid_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_aml_training_user ON aml.aml_training_records(user_id);
CREATE INDEX idx_aml_training_date ON aml.aml_training_records(training_date);

COMMENT ON TABLE aml.aml_training_records IS 'AML-specific training records for bank staff';
