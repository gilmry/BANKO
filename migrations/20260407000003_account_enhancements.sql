-- BANKO Account BC Enhancement
-- Internal accounts (suspense, clearing, P&L, nostro, vostro), account limits, interest capitalization, balance notifications

-- Add columns to accounts table
ALTER TABLE account.accounts ADD COLUMN IF NOT EXISTS account_subtype VARCHAR(50) CHECK (account_subtype IN ('suspense', 'clearing', 'pnl', 'nostro', 'vostro', 'loan', 'deposit', 'standard', 'investment'));
ALTER TABLE account.accounts ADD COLUMN IF NOT EXISTS iban VARCHAR(34);
ALTER TABLE account.accounts ADD COLUMN IF NOT EXISTS interest_rate DECIMAL(10, 6);
ALTER TABLE account.accounts ADD COLUMN IF NOT EXISTS interest_capitalization_frequency VARCHAR(20) DEFAULT 'monthly' CHECK (interest_capitalization_frequency IN ('daily', 'weekly', 'monthly', 'quarterly', 'annually', 'manual'));

-- Account Limits table
CREATE TABLE IF NOT EXISTS account.account_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account.accounts(id) ON DELETE CASCADE,
    limit_type VARCHAR(50) NOT NULL CHECK (limit_type IN ('daily_withdrawal', 'daily_transfer', 'transaction_max', 'monthly_total', 'overdraft', 'credit_line')),
    limit_amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    used_amount BIGINT NOT NULL DEFAULT 0,
    period_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    period_end TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_account_limits_account ON account.account_limits(account_id);
CREATE INDEX idx_account_limits_type ON account.account_limits(limit_type);
CREATE INDEX idx_account_limits_active ON account.account_limits(is_active);

COMMENT ON TABLE account.account_limits IS 'Account transaction limits (daily withdrawal, transfer caps, overdraft limits)';

-- Interest Capitalization Ledger table
CREATE TABLE IF NOT EXISTS account.interest_capitalizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account.accounts(id) ON DELETE CASCADE,
    capitalization_date TIMESTAMPTZ NOT NULL,
    interest_amount BIGINT NOT NULL,
    principal_amount BIGINT NOT NULL,
    capitalization_period_start TIMESTAMPTZ NOT NULL,
    capitalization_period_end TIMESTAMPTZ NOT NULL,
    frequency_type VARCHAR(20) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'processed', 'reversed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_interest_capitalizations_account ON account.interest_capitalizations(account_id);
CREATE INDEX idx_interest_capitalizations_date ON account.interest_capitalizations(capitalization_date);
CREATE INDEX idx_interest_capitalizations_status ON account.interest_capitalizations(status);

COMMENT ON TABLE account.interest_capitalizations IS 'Interest capitalization history for accounts with accrual accounting';

-- Balance Notifications table
CREATE TABLE IF NOT EXISTS account.balance_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account.accounts(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL CHECK (notification_type IN ('low_balance', 'high_balance', 'overdraft', 'threshold', 'interest_accrual', 'fee_applied')),
    threshold_amount BIGINT,
    notification_frequency VARCHAR(20) DEFAULT 'immediate' CHECK (notification_frequency IN ('immediate', 'daily', 'weekly', 'monthly', 'never')),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    delivery_channels VARCHAR(50)[] DEFAULT '{"email"}',
    last_notified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_balance_notifications_account ON account.balance_notifications(account_id);
CREATE INDEX idx_balance_notifications_type ON account.balance_notifications(notification_type);
CREATE INDEX idx_balance_notifications_active ON account.balance_notifications(is_active);

COMMENT ON TABLE account.balance_notifications IS 'Configurable balance alert notifications for account monitoring';

-- Internal Account References table
CREATE TABLE IF NOT EXISTS account.internal_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL UNIQUE REFERENCES account.accounts(id) ON DELETE CASCADE,
    internal_code VARCHAR(50) NOT NULL UNIQUE,
    ledger_classification VARCHAR(50) NOT NULL CHECK (ledger_classification IN ('assets', 'liabilities', 'equity', 'revenue', 'expenses')),
    purpose TEXT NOT NULL,
    reconciliation_account_id UUID REFERENCES account.accounts(id),
    is_automated BOOLEAN NOT NULL DEFAULT TRUE,
    automation_rule TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_internal_accounts_code ON account.internal_accounts(internal_code);
CREATE INDEX idx_internal_accounts_classification ON account.internal_accounts(ledger_classification);

COMMENT ON TABLE account.internal_accounts IS 'Internal accounts for suspense, clearing, P&L, nostro/vostro management';
