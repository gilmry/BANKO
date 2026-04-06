-- Create products table
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    product_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    interest_rate_annual DECIMAL(10,4),
    interest_calc_method VARCHAR(20),
    interest_floor_rate DECIMAL(10,4),
    interest_ceiling_rate DECIMAL(10,4),
    min_balance DECIMAL(18,3),
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    eligibility_min_age SMALLINT,
    eligibility_max_age SMALLINT,
    eligibility_min_income DECIMAL(18,3),
    eligibility_min_credit_score INTEGER,
    eligibility_required_segment VARCHAR(20),
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_products_status ON products(status);
CREATE INDEX idx_products_product_type ON products(product_type);
CREATE INDEX idx_products_created_at ON products(created_at DESC);

-- Create product fees table
CREATE TABLE IF NOT EXISTS product_fees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    fee_type VARCHAR(30) NOT NULL,
    fixed_amount DECIMAL(18,3),
    rate DECIMAL(10,6),
    min_amount DECIMAL(18,3),
    max_amount DECIMAL(18,3),
    charged_on SMALLINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_fees_product ON product_fees(product_id);
CREATE INDEX idx_product_fees_fee_type ON product_fees(fee_type);

-- Create product segment pricing table
CREATE TABLE IF NOT EXISTS product_segment_pricing (
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    segment VARCHAR(20) NOT NULL,
    rate_override DECIMAL(10,4) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (product_id, segment)
);

CREATE INDEX idx_product_segment_pricing_product ON product_segment_pricing(product_id);

-- Create pricing grids table
CREATE TABLE IF NOT EXISTS pricing_grids (
    id UUID PRIMARY KEY,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    effective_from TIMESTAMPTZ NOT NULL,
    effective_to TIMESTAMPTZ,
    active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pricing_grids_product ON pricing_grids(product_id);
CREATE INDEX idx_pricing_grids_effective_from ON pricing_grids(effective_from DESC);
CREATE INDEX idx_pricing_grids_active ON pricing_grids(active);

-- Create pricing bands table
CREATE TABLE IF NOT EXISTS pricing_bands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grid_id UUID NOT NULL REFERENCES pricing_grids(id) ON DELETE CASCADE,
    min_amount DECIMAL(18,3) NOT NULL,
    max_amount DECIMAL(18,3),
    rate DECIMAL(10,4) NOT NULL,
    fees_override DECIMAL(18,3),
    sort_order SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pricing_bands_grid ON pricing_bands(grid_id);
CREATE INDEX idx_pricing_bands_sort_order ON pricing_bands(grid_id, sort_order);

-- Create interest accruals table
CREATE TABLE IF NOT EXISTS interest_accruals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL,
    accrual_date DATE NOT NULL,
    principal DECIMAL(18,3) NOT NULL,
    rate DECIMAL(10,6) NOT NULL,
    interest_amount DECIMAL(18,3) NOT NULL,
    calc_method VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(account_id, accrual_date)
);

CREATE INDEX idx_interest_accruals_account ON interest_accruals(account_id);
CREATE INDEX idx_interest_accruals_accrual_date ON interest_accruals(accrual_date DESC);
