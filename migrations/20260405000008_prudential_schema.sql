-- Prudential (BC6) schema — Circ. 2016-03, 2018-06, 2018-10
CREATE SCHEMA IF NOT EXISTS prudential;

CREATE TABLE prudential.ratios (
    id UUID PRIMARY KEY,
    institution_id UUID NOT NULL,
    capital_tier1 BIGINT NOT NULL,
    capital_tier2 BIGINT NOT NULL,
    risk_weighted_assets BIGINT NOT NULL,
    total_credits BIGINT NOT NULL,
    total_deposits BIGINT NOT NULL,
    solvency_ratio DOUBLE PRECISION NOT NULL,
    tier1_ratio DOUBLE PRECISION NOT NULL,
    credit_deposit_ratio DOUBLE PRECISION NOT NULL,
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE prudential.exposures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ratio_id UUID NOT NULL REFERENCES prudential.ratios(id),
    beneficiary_id UUID NOT NULL,
    amount BIGINT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE prudential.ratio_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ratio_id UUID NOT NULL REFERENCES prudential.ratios(id),
    institution_id UUID NOT NULL,
    snapshot_date DATE NOT NULL,
    solvency_ratio DOUBLE PRECISION NOT NULL,
    tier1_ratio DOUBLE PRECISION NOT NULL,
    credit_deposit_ratio DOUBLE PRECISION NOT NULL,
    breach_type VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE prudential.breach_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ratio_id UUID NOT NULL REFERENCES prudential.ratios(id),
    breach_type VARCHAR(50) NOT NULL,
    current_value DOUBLE PRECISION NOT NULL,
    threshold DOUBLE PRECISION NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'Critical',
    status VARCHAR(20) NOT NULL DEFAULT 'Breach',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

CREATE INDEX idx_prudential_ratios_institution ON prudential.ratios(institution_id);
CREATE INDEX idx_prudential_ratios_calculated ON prudential.ratios(calculated_at);
CREATE INDEX idx_prudential_snapshots_institution ON prudential.ratio_snapshots(institution_id);
CREATE INDEX idx_prudential_snapshots_date ON prudential.ratio_snapshots(snapshot_date);
CREATE INDEX idx_prudential_exposures_ratio ON prudential.exposures(ratio_id);
CREATE INDEX idx_prudential_exposures_beneficiary ON prudential.exposures(beneficiary_id);
CREATE INDEX idx_prudential_alerts_status ON prudential.breach_alerts(status);
CREATE INDEX idx_prudential_alerts_ratio ON prudential.breach_alerts(ratio_id);
