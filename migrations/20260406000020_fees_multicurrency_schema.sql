-- Multi-Currency and Fees Migration
-- Created: 2026-04-06

-- ==================== Fee Definitions ====================
CREATE TABLE IF NOT EXISTS fee_definitions (
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    category VARCHAR(30) NOT NULL,
    fixed_amount DECIMAL(18,3),
    rate_percent DECIMAL(10,6),
    min_amount DECIMAL(18,3),
    max_amount DECIMAL(18,3),
    condition_type VARCHAR(30) NOT NULL DEFAULT 'Always',
    condition_value DECIMAL(18,3),
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fee_definitions_category ON fee_definitions(category);
CREATE INDEX idx_fee_definitions_currency ON fee_definitions(currency);

-- ==================== Fee Segment Applicability ====================
CREATE TABLE IF NOT EXISTS fee_segment_applicability (
    fee_definition_id UUID NOT NULL REFERENCES fee_definitions(id) ON DELETE CASCADE,
    segment VARCHAR(20) NOT NULL,
    PRIMARY KEY (fee_definition_id, segment)
);

CREATE INDEX idx_fee_segment_applicability_segment ON fee_segment_applicability(segment);

-- ==================== Fee Charges ====================
CREATE TABLE IF NOT EXISTS fee_charges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fee_definition_id UUID NOT NULL REFERENCES fee_definitions(id),
    account_id UUID NOT NULL,
    amount DECIMAL(18,3) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending','Charged','Unpaid','Waived','Reversed')),
    charged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    description TEXT
);

CREATE INDEX idx_fee_charges_account ON fee_charges(account_id);
CREATE INDEX idx_fee_charges_status ON fee_charges(status);
CREATE INDEX idx_fee_charges_fee_def ON fee_charges(fee_definition_id);

-- ==================== Fee Grids ====================
CREATE TABLE IF NOT EXISTS fee_grids (
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    segment VARCHAR(20) NOT NULL,
    effective_from TIMESTAMPTZ NOT NULL,
    effective_to TIMESTAMPTZ,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fee_grids_segment ON fee_grids(segment);
CREATE INDEX idx_fee_grids_active ON fee_grids(active);
CREATE INDEX idx_fee_grids_effective_from ON fee_grids(effective_from);

-- ==================== Fee Grid Overrides ====================
CREATE TABLE IF NOT EXISTS fee_grid_overrides (
    grid_id UUID NOT NULL REFERENCES fee_grids(id) ON DELETE CASCADE,
    category VARCHAR(30) NOT NULL,
    override_amount DECIMAL(18,3) NOT NULL,
    PRIMARY KEY (grid_id, category)
);

CREATE INDEX idx_fee_grid_overrides_grid ON fee_grid_overrides(grid_id);

-- ==================== Currency Conversions Tracking ====================
CREATE TABLE IF NOT EXISTS currency_conversions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    from_account_id UUID NOT NULL,
    to_account_id UUID NOT NULL,
    original_amount DECIMAL(18,3) NOT NULL,
    original_currency VARCHAR(3) NOT NULL,
    converted_amount DECIMAL(18,3) NOT NULL,
    target_currency VARCHAR(3) NOT NULL,
    market_rate DECIMAL(18,8) NOT NULL,
    bank_rate DECIMAL(18,8) NOT NULL,
    margin_applied DECIMAL(10,4) NOT NULL,
    conversion_date TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_conversions_customer ON currency_conversions(customer_id);
CREATE INDEX idx_conversions_from_account ON currency_conversions(from_account_id);
CREATE INDEX idx_conversions_to_account ON currency_conversions(to_account_id);
CREATE INDEX idx_conversions_date ON currency_conversions(conversion_date);

-- ==================== Monthly Conversion Limits ====================
CREATE TABLE IF NOT EXISTS monthly_conversion_limits (
    customer_id UUID NOT NULL,
    currency VARCHAR(3) NOT NULL,
    month VARCHAR(7) NOT NULL, -- YYYY-MM format
    limit_amount DECIMAL(18,3) NOT NULL,
    used_amount DECIMAL(18,3) NOT NULL DEFAULT 0,
    PRIMARY KEY (customer_id, currency, month)
);

CREATE INDEX idx_monthly_limits_customer ON monthly_conversion_limits(customer_id);
CREATE INDEX idx_monthly_limits_month ON monthly_conversion_limits(month);
