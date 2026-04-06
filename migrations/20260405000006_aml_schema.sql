-- AML bounded context schema
-- STORY-AML-03: Transaction, Alert, Investigation tables
-- Loi 2015-26 [REF-28], Circ. 2025-17 [REF-33]

CREATE SCHEMA IF NOT EXISTS aml;

-- Transactions table (AML monitoring context)
CREATE TABLE aml.transactions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    counterparty VARCHAR(255) NOT NULL,
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    transaction_type VARCHAR(20) NOT NULL,
    direction VARCHAR(10) NOT NULL,
    transaction_timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- AML Alerts (append-only: status changes tracked via alert_status_changes)
CREATE TABLE aml.alerts (
    id UUID PRIMARY KEY,
    transaction_id UUID NOT NULL REFERENCES aml.transactions(id),
    risk_level VARCHAR(10) NOT NULL,
    reason TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'New',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Alert status changes (append-only audit trail)
CREATE TABLE aml.alert_status_changes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alert_id UUID NOT NULL REFERENCES aml.alerts(id),
    old_status VARCHAR(20) NOT NULL,
    new_status VARCHAR(20) NOT NULL,
    changed_by VARCHAR(100),
    reason TEXT,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Investigations
CREATE TABLE aml.investigations (
    id UUID PRIMARY KEY,
    alert_id UUID NOT NULL REFERENCES aml.alerts(id),
    status VARCHAR(20) NOT NULL DEFAULT 'Open',
    assigned_to VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Investigation notes (append-only)
CREATE TABLE aml.investigation_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    investigation_id UUID NOT NULL REFERENCES aml.investigations(id),
    note TEXT NOT NULL,
    author VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Suspicion reports (DOS)
CREATE TABLE aml.suspicion_reports (
    id UUID PRIMARY KEY,
    investigation_id UUID NOT NULL REFERENCES aml.investigations(id),
    customer_info TEXT NOT NULL,
    transaction_details TEXT NOT NULL,
    reasons TEXT NOT NULL,
    evidence TEXT,
    timeline TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_at TIMESTAMPTZ
);

-- Asset freezes (INV-09: immediate)
CREATE TABLE aml.asset_freezes (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    reason TEXT NOT NULL,
    ordered_by VARCHAR(100) NOT NULL,
    status VARCHAR(10) NOT NULL DEFAULT 'Active',
    frozen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    lifted_at TIMESTAMPTZ,
    lifted_by VARCHAR(100)
);

-- Indexes
CREATE INDEX idx_aml_transactions_account ON aml.transactions(account_id);
CREATE INDEX idx_aml_transactions_customer ON aml.transactions(customer_id);
CREATE INDEX idx_aml_transactions_timestamp ON aml.transactions(transaction_timestamp);
CREATE INDEX idx_aml_alerts_transaction ON aml.alerts(transaction_id);
CREATE INDEX idx_aml_alerts_status ON aml.alerts(status);
CREATE INDEX idx_aml_alerts_risk_level ON aml.alerts(risk_level);
CREATE INDEX idx_aml_investigations_alert ON aml.investigations(alert_id);
CREATE INDEX idx_aml_investigations_status ON aml.investigations(status);
CREATE INDEX idx_aml_freezes_account ON aml.asset_freezes(account_id);
CREATE INDEX idx_aml_freezes_active ON aml.asset_freezes(account_id) WHERE status = 'Active';
