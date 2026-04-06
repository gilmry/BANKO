CREATE SCHEMA IF NOT EXISTS fx;

CREATE TABLE fx.exchange_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_currency VARCHAR(3) NOT NULL,
    target_currency VARCHAR(3) NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE fx.operations (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    operation_type VARCHAR(20) NOT NULL DEFAULT 'Spot',
    source_currency VARCHAR(3) NOT NULL,
    target_currency VARCHAR(3) NOT NULL,
    source_amount BIGINT NOT NULL,
    target_amount BIGINT NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    reference VARCHAR(100) NOT NULL,
    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,
    settled_at TIMESTAMPTZ
);

CREATE TABLE fx.daily_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL,
    currency VARCHAR(3) NOT NULL,
    daily_limit BIGINT NOT NULL DEFAULT 100000000,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(account_id, currency)
);

-- Seed common exchange rates
INSERT INTO fx.exchange_rates (source_currency, target_currency, rate) VALUES
('TND', 'EUR', 0.30), ('EUR', 'TND', 3.35),
('TND', 'USD', 0.32), ('USD', 'TND', 3.12),
('TND', 'GBP', 0.25), ('GBP', 'TND', 3.95),
('EUR', 'USD', 1.08), ('USD', 'EUR', 0.93);

CREATE INDEX idx_fx_ops_account ON fx.operations(account_id);
CREATE INDEX idx_fx_ops_status ON fx.operations(status);
CREATE INDEX idx_fx_rates_pair ON fx.exchange_rates(source_currency, target_currency);
CREATE INDEX idx_fx_daily_limits ON fx.daily_limits(account_id, currency);
