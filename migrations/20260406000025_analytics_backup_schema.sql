-- Sprint J: BI/Analytics + Backup/DR Schema
-- STORY-BI-01 to BI-03, STORY-DR-01 to DR-03

-- ============================================================
-- Report Definitions & Executions (STORY-BI-01, BI-02)
-- ============================================================

CREATE TABLE IF NOT EXISTS report_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    report_type VARCHAR(50) NOT NULL, -- 'portfolio', 'operational', 'regulatory', 'custom'
    query_template TEXT NOT NULL,
    parameters JSONB DEFAULT '{}',
    schedule VARCHAR(50), -- cron expression or NULL for on-demand
    output_format VARCHAR(20) NOT NULL DEFAULT 'pdf', -- 'pdf', 'xlsx', 'csv', 'json'
    created_by UUID,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS report_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_definition_id UUID NOT NULL REFERENCES report_definitions(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'running', 'completed', 'failed'
    parameters JSONB DEFAULT '{}',
    result_path VARCHAR(500),
    result_size_bytes BIGINT,
    error_message TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    requested_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_report_executions_definition ON report_executions(report_definition_id);
CREATE INDEX idx_report_executions_status ON report_executions(status);
CREATE INDEX idx_report_executions_requested_by ON report_executions(requested_by);

-- ============================================================
-- KPI Snapshots (STORY-BI-03)
-- ============================================================

CREATE TABLE IF NOT EXISTS kpi_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    snapshot_date DATE NOT NULL,
    total_customers INTEGER NOT NULL DEFAULT 0,
    active_accounts INTEGER NOT NULL DEFAULT 0,
    total_deposits NUMERIC(20,3) NOT NULL DEFAULT 0,
    total_loans NUMERIC(20,3) NOT NULL DEFAULT 0,
    npl_ratio NUMERIC(8,4) DEFAULT 0,
    liquidity_ratio NUMERIC(8,4) DEFAULT 0,
    capital_adequacy_ratio NUMERIC(8,4) DEFAULT 0,
    daily_transactions INTEGER NOT NULL DEFAULT 0,
    daily_transaction_volume NUMERIC(20,3) NOT NULL DEFAULT 0,
    aml_alerts_open INTEGER NOT NULL DEFAULT 0,
    fraud_blocked_count INTEGER NOT NULL DEFAULT 0,
    avg_response_time_ms NUMERIC(10,2) DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(snapshot_date)
);

CREATE INDEX idx_kpi_snapshots_date ON kpi_snapshots(snapshot_date);

-- ============================================================
-- Backup Records (STORY-DR-01)
-- ============================================================

CREATE TABLE IF NOT EXISTS backup_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_type VARCHAR(20) NOT NULL, -- 'Full', 'Incremental', 'Wal'
    status VARCHAR(20) NOT NULL DEFAULT 'Running', -- 'Running', 'Completed', 'Failed', 'Verified'
    file_path VARCHAR(500) NOT NULL,
    file_size_bytes BIGINT,
    checksum_sha256 VARCHAR(64),
    checksum_verified BOOLEAN NOT NULL DEFAULT FALSE,
    parent_backup_id UUID REFERENCES backup_records(id),
    retention_days INTEGER NOT NULL DEFAULT 90,
    expires_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_backup_records_type ON backup_records(backup_type);
CREATE INDEX idx_backup_records_status ON backup_records(status);
CREATE INDEX idx_backup_records_expires ON backup_records(expires_at);

-- ============================================================
-- Restore Logs (STORY-DR-02)
-- ============================================================

CREATE TABLE IF NOT EXISTS restore_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_id UUID REFERENCES backup_records(id),
    restore_type VARCHAR(20) NOT NULL, -- 'Full', 'PITR'
    target_timestamp TIMESTAMPTZ,
    status VARCHAR(20) NOT NULL DEFAULT 'Running', -- 'Running', 'Completed', 'Failed'
    data_verified BOOLEAN NOT NULL DEFAULT FALSE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    initiated_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_restore_logs_backup ON restore_logs(backup_id);
CREATE INDEX idx_restore_logs_status ON restore_logs(status);

-- ============================================================
-- Disaster Recovery Runs (STORY-DR-03)
-- ============================================================

CREATE TABLE IF NOT EXISTS dr_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_type VARCHAR(20) NOT NULL, -- 'Test', 'Real'
    status VARCHAR(30) NOT NULL DEFAULT 'Initiated', -- 'Initiated', 'BackupPhase', 'RestorePhase', 'ValidationPhase', 'Completed', 'Failed'
    rpo_seconds INTEGER, -- Recovery Point Objective achieved
    rto_seconds INTEGER, -- Recovery Time Objective achieved
    rpo_target_seconds INTEGER NOT NULL DEFAULT 3600,
    rto_target_seconds INTEGER NOT NULL DEFAULT 14400,
    rpo_met BOOLEAN,
    rto_met BOOLEAN,
    steps_completed JSONB DEFAULT '[]',
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    initiated_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dr_runs_status ON dr_runs(status);
CREATE INDEX idx_dr_runs_type ON dr_runs(run_type);
