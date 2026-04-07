-- BANKO Accounting BC Enhancement
-- Chart of Accounts (NCT Classes 1-7), dual posting (NCT/IFRS), period closings (daily/monthly/annual)

CREATE SCHEMA IF NOT EXISTS accounting;

-- Chart of Accounts (NCT - Normalized Chart of Accounts for Tunisia)
CREATE TABLE IF NOT EXISTS accounting.chart_of_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_code VARCHAR(20) NOT NULL UNIQUE,
    account_name_en VARCHAR(255) NOT NULL,
    account_name_fr VARCHAR(255) NOT NULL,
    account_name_ar VARCHAR(255),
    nct_class SMALLINT NOT NULL CHECK (nct_class BETWEEN 1 AND 7), -- 1:Assets, 2:Liabilities, 3:Equity, 4:Revenue, 5:Expenses, 6:Accounts, 7:Analysis
    account_type VARCHAR(50) NOT NULL CHECK (account_type IN ('asset', 'liability', 'equity', 'revenue', 'expense', 'clearing', 'control')),
    parent_account_code VARCHAR(20) REFERENCES accounting.chart_of_accounts(account_code),
    debit_credit_nature VARCHAR(10) NOT NULL CHECK (debit_credit_nature IN ('D', 'C', 'N')), -- D=Debit, C=Credit, N=Normal (either)
    is_control_account BOOLEAN NOT NULL DEFAULT FALSE,
    is_analytical BOOLEAN NOT NULL DEFAULT FALSE,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chart_of_accounts_code ON accounting.chart_of_accounts(account_code);
CREATE INDEX idx_chart_of_accounts_nct_class ON accounting.chart_of_accounts(nct_class);
CREATE INDEX idx_chart_of_accounts_type ON accounting.chart_of_accounts(account_type);
CREATE INDEX idx_chart_of_accounts_parent ON accounting.chart_of_accounts(parent_account_code);
CREATE INDEX idx_chart_of_accounts_active ON accounting.chart_of_accounts(is_active);

COMMENT ON TABLE accounting.chart_of_accounts IS 'NCT-compliant chart of accounts for Tunisian banking regulations';

-- IFRS/Regulatory Account Mapping
CREATE TABLE IF NOT EXISTS accounting.ifrs_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nct_account_code VARCHAR(20) NOT NULL UNIQUE REFERENCES accounting.chart_of_accounts(account_code),
    ifrs_classification VARCHAR(100) NOT NULL CHECK (ifrs_classification IN ('AC', 'FVOCI', 'FVPL', 'Equity', 'Liability', 'Revenue', 'Expense')), -- IFRS 9 classifications
    bcn_asset_class VARCHAR(50),
    regulatory_risk_weight DECIMAL(5, 2),
    capital_requirement_type VARCHAR(50),
    stress_test_applicable BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ifrs_mappings_nct ON accounting.ifrs_mappings(nct_account_code);
CREATE INDEX idx_ifrs_mappings_classification ON accounting.ifrs_mappings(ifrs_classification);

COMMENT ON TABLE accounting.ifrs_mappings IS 'Mapping between NCT and IFRS/regulatory classifications for reporting';

-- Journal Entries (Dual Posting: NCT + IFRS)
CREATE TABLE IF NOT EXISTS accounting.journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_code VARCHAR(20) NOT NULL,
    entry_date TIMESTAMPTZ NOT NULL,
    posting_reference VARCHAR(100),
    narrative TEXT NOT NULL,
    total_debit BIGINT NOT NULL DEFAULT 0,
    total_credit BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    entry_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (entry_status IN ('draft', 'submitted', 'approved', 'posted', 'reversed', 'corrected')),
    created_by VARCHAR(255) NOT NULL,
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    posted_at TIMESTAMPTZ,
    reversal_reason TEXT,
    original_entry_id UUID REFERENCES accounting.journal_entries(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_journal_entries_date ON accounting.journal_entries(entry_date);
CREATE INDEX idx_journal_entries_status ON accounting.journal_entries(entry_status);
CREATE INDEX idx_journal_entries_journal ON accounting.journal_entries(journal_code);
CREATE INDEX idx_journal_entries_posted ON accounting.journal_entries(posted_at);

COMMENT ON TABLE accounting.journal_entries IS 'Dual-ledger journal entries for NCT and IFRS reporting';

-- Journal Entry Lines (detailed debit/credit)
CREATE TABLE IF NOT EXISTS accounting.journal_entry_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_entry_id UUID NOT NULL REFERENCES accounting.journal_entries(id) ON DELETE CASCADE,
    line_number SMALLINT NOT NULL,
    nct_account_code VARCHAR(20) NOT NULL REFERENCES accounting.chart_of_accounts(account_code),
    ifrs_account_code VARCHAR(20),
    debit_amount BIGINT NOT NULL DEFAULT 0,
    credit_amount BIGINT NOT NULL DEFAULT 0,
    amount_currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    exchange_rate DECIMAL(15, 8),
    narrative TEXT,
    cost_center_code VARCHAR(20),
    analytical_dimension_1 VARCHAR(100),
    analytical_dimension_2 VARCHAR(100),
    analytical_dimension_3 VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_journal_entry_lines_journal ON accounting.journal_entry_lines(journal_entry_id);
CREATE INDEX idx_journal_entry_lines_nct_account ON accounting.journal_entry_lines(nct_account_code);
CREATE INDEX idx_journal_entry_lines_cost_center ON accounting.journal_entry_lines(cost_center_code);

COMMENT ON TABLE accounting.journal_entry_lines IS 'Dual-ledger line items (NCT + IFRS) for each journal entry';

-- Accounting Periods and Closings
CREATE TABLE IF NOT EXISTS accounting.accounting_periods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_code VARCHAR(20) NOT NULL UNIQUE, -- e.g., 2026-04 for April 2026, 2026 for annual
    period_type VARCHAR(20) NOT NULL CHECK (period_type IN ('daily', 'monthly', 'quarterly', 'annual')),
    period_start_date TIMESTAMPTZ NOT NULL,
    period_end_date TIMESTAMPTZ NOT NULL,
    is_closed BOOLEAN NOT NULL DEFAULT FALSE,
    closed_by VARCHAR(255),
    closed_at TIMESTAMPTZ,
    period_status VARCHAR(50) NOT NULL DEFAULT 'open' CHECK (period_status IN ('open', 'in_progress', 'locked', 'closed', 'archived')),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_accounting_periods_code ON accounting.accounting_periods(period_code);
CREATE INDEX idx_accounting_periods_type ON accounting.accounting_periods(period_type);
CREATE INDEX idx_accounting_periods_status ON accounting.accounting_periods(period_status);
CREATE INDEX idx_accounting_periods_dates ON accounting.accounting_periods(period_start_date, period_end_date);

COMMENT ON TABLE accounting.accounting_periods IS 'Accounting period definitions for daily, monthly, quarterly, and annual closings';

-- Period Closing Checklist
CREATE TABLE IF NOT EXISTS accounting.period_closing_checklists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    accounting_period_id UUID NOT NULL REFERENCES accounting.accounting_periods(id) ON DELETE CASCADE,
    task_code VARCHAR(50) NOT NULL,
    task_description VARCHAR(500) NOT NULL,
    responsible_department VARCHAR(100),
    scheduled_completion_date TIMESTAMPTZ,
    actual_completion_date TIMESTAMPTZ,
    task_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (task_status IN ('pending', 'in_progress', 'completed', 'blocked', 'waived')),
    completion_evidence TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_closing_checklists_period ON accounting.period_closing_checklists(accounting_period_id);
CREATE INDEX idx_closing_checklists_status ON accounting.period_closing_checklists(task_status);

COMMENT ON TABLE accounting.period_closing_checklists IS 'Closing task checklist for each accounting period';

-- Trial Balance Snapshots
CREATE TABLE IF NOT EXISTS accounting.trial_balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    accounting_period_id UUID NOT NULL REFERENCES accounting.accounting_periods(id) ON DELETE CASCADE,
    account_code VARCHAR(20) NOT NULL REFERENCES accounting.chart_of_accounts(account_code),
    opening_balance BIGINT NOT NULL DEFAULT 0,
    debit_movements BIGINT NOT NULL DEFAULT 0,
    credit_movements BIGINT NOT NULL DEFAULT 0,
    closing_balance BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    is_balanced BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_trial_balances_period_account ON accounting.trial_balances(accounting_period_id, account_code);
CREATE INDEX idx_trial_balances_period ON accounting.trial_balances(accounting_period_id);

COMMENT ON TABLE accounting.trial_balances IS 'Trial balance snapshots at period end for audit and reconciliation';
