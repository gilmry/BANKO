-- End-of-Day Processing Schema (EPIC-25)
CREATE SCHEMA IF NOT EXISTS eod;

-- EOD run metadata and overall status
CREATE TABLE eod.runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_date DATE NOT NULL UNIQUE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    overall_status VARCHAR(30) NOT NULL DEFAULT 'Running'
        CHECK (overall_status IN ('Running', 'Completed', 'PartiallyCompleted', 'Failed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_eod_runs_date ON eod.runs(run_date);
CREATE INDEX idx_eod_runs_status ON eod.runs(overall_status);

-- Individual step execution results within an EOD run
CREATE TABLE eod.step_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_id UUID NOT NULL REFERENCES eod.runs(id) ON DELETE CASCADE,
    step_name VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL
        CHECK (status IN ('Pending', 'Running', 'Completed', 'Failed', 'Skipped')),
    records_processed INTEGER NOT NULL DEFAULT 0,
    duration_ms BIGINT NOT NULL DEFAULT 0,
    details TEXT,
    error_message TEXT,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_eod_step_results_run ON eod.step_results(run_id);
CREATE INDEX idx_eod_step_results_step ON eod.step_results(step_name);
CREATE INDEX idx_eod_step_results_status ON eod.step_results(status);

-- Daily interest accrual records
CREATE TABLE eod.interest_accruals_daily (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL,
    accrual_date DATE NOT NULL,
    principal DECIMAL(18, 3) NOT NULL,
    annual_rate DECIMAL(10, 6) NOT NULL,
    daily_interest DECIMAL(18, 6) NOT NULL,
    accrual_method VARCHAR(20) NOT NULL
        CHECK (accrual_method IN ('Simple', 'Compound')),
    accrual_type VARCHAR(10) NOT NULL
        CHECK (accrual_type IN ('Credit', 'Debit')),
    is_capitalized BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(account_id, accrual_date)
);

CREATE INDEX idx_interest_accruals_account ON eod.interest_accruals_daily(account_id);
CREATE INDEX idx_interest_accruals_date ON eod.interest_accruals_daily(accrual_date);
CREATE INDEX idx_interest_accruals_capitalized ON eod.interest_accruals_daily(is_capitalized);

-- Reconciliation reports
CREATE TABLE eod.reconciliation_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reconciliation_date DATE NOT NULL UNIQUE,
    total_debits DECIMAL(18, 3) NOT NULL,
    total_credits DECIMAL(18, 3) NOT NULL,
    total_variance DECIMAL(18, 3) NOT NULL,
    overall_status VARCHAR(30) NOT NULL
        CHECK (overall_status IN ('Balanced', 'Variance', 'AutoResolved', 'ManualReviewRequired')),
    auto_resolutions JSONB NOT NULL DEFAULT '[]',
    account_details JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reconciliation_date ON eod.reconciliation_reports(reconciliation_date);
CREATE INDEX idx_reconciliation_status ON eod.reconciliation_reports(overall_status);

-- Grant permissions (if needed for role-based access)
-- GRANT SELECT ON ALL TABLES IN SCHEMA eod TO banking_read_role;
-- GRANT SELECT, INSERT, UPDATE ON ALL TABLES IN SCHEMA eod TO banking_write_role;
