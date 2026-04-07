-- BANKO Collateral BC Schema
-- Collateral management, valuations, allocations, and LTV calculations

-- Collaterals master table
CREATE TABLE IF NOT EXISTS collateral.collaterals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_code VARCHAR(50) NOT NULL UNIQUE,
    collateral_type VARCHAR(50) NOT NULL CHECK (collateral_type IN ('real_estate', 'securities', 'movable_assets', 'cash', 'guarantee', 'other')),
    collateral_description TEXT NOT NULL,
    owner_customer_id UUID NOT NULL,
    owner_name VARCHAR(255) NOT NULL,
    owner_id_number VARCHAR(50),
    collateral_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (collateral_status IN ('active', 'pledged', 'released', 'forfeited', 'disputed', 'expired')),
    initial_valuation_amount BIGINT NOT NULL, -- Cents for currency precision
    currency_code VARCHAR(3) NOT NULL DEFAULT 'TND',
    valuation_date TIMESTAMPTZ NOT NULL,
    valuation_method VARCHAR(50) NOT NULL CHECK (valuation_method IN ('appraisal', 'market_comparable', 'income_approach', 'cost_approach', 'automated_valuation', 'third_party')),
    appraiser_name VARCHAR(255),
    appraiser_license_number VARCHAR(50),
    appraisal_report_file_id VARCHAR(255),
    insurance_requirement BOOLEAN NOT NULL DEFAULT FALSE,
    insurance_policy_id VARCHAR(100),
    insurance_coverage_amount BIGINT,
    insurance_expiry_date DATE,
    pledged_to_account_id UUID,
    pledged_date TIMESTAMPTZ,
    release_date TIMESTAMPTZ,
    registration_number VARCHAR(100),
    registration_authority VARCHAR(255),
    registration_date DATE,
    expiry_date DATE,
    collateral_documentation_complete BOOLEAN NOT NULL DEFAULT FALSE,
    documentation_checklist JSONB, -- {has_title_deed: true, has_appraisal: true, has_insurance: true}
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255)
);

CREATE INDEX idx_collaterals_owner ON collateral.collaterals(owner_customer_id);
CREATE INDEX idx_collaterals_status ON collateral.collaterals(collateral_status);
CREATE INDEX idx_collaterals_type ON collateral.collaterals(collateral_type);
CREATE INDEX idx_collaterals_account ON collateral.collaterals(pledged_to_account_id);
CREATE INDEX idx_collaterals_expiry ON collateral.collaterals(expiry_date);

COMMENT ON TABLE collateral.collaterals IS 'Collateral master data with ownership, status, and insurance requirements';

-- Collateral Valuations (Historical)
CREATE TABLE IF NOT EXISTS collateral.collateral_valuations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_id UUID NOT NULL REFERENCES collateral.collaterals(id),
    valuation_date TIMESTAMPTZ NOT NULL,
    valuation_amount BIGINT NOT NULL, -- Cents
    currency_code VARCHAR(3) NOT NULL,
    valuation_method VARCHAR(50) NOT NULL CHECK (valuation_method IN ('appraisal', 'market_comparable', 'income_approach', 'cost_approach', 'automated_valuation', 'third_party')),
    valuation_source VARCHAR(100),
    appraiser_name VARCHAR(255),
    appraiser_license_number VARCHAR(50),
    appraisal_report_reference VARCHAR(255),
    valuation_confidence_level VARCHAR(50) NOT NULL CHECK (valuation_confidence_level IN ('high', 'medium', 'low')),
    market_conditions TEXT,
    adjustments_applied TEXT,
    valuation_effective_until DATE,
    valuation_status VARCHAR(50) NOT NULL DEFAULT 'current' CHECK (valuation_status IN ('current', 'superseded', 'disputed', 'pending_approval')),
    approver_name VARCHAR(255),
    approval_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_collateral_valuations_collateral ON collateral.collateral_valuations(collateral_id);
CREATE INDEX idx_collateral_valuations_date ON collateral.collateral_valuations(valuation_date);
CREATE INDEX idx_collateral_valuations_status ON collateral.collateral_valuations(valuation_status);

COMMENT ON TABLE collateral.collateral_valuations IS 'Historical collateral valuations and revaluation audits';

-- Collateral Allocations (Pledge to Loans/Facilities)
CREATE TABLE IF NOT EXISTS collateral.collateral_allocations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_id UUID NOT NULL REFERENCES collateral.collaterals(id),
    account_id UUID NOT NULL,
    facility_code VARCHAR(50),
    facility_type VARCHAR(50) NOT NULL CHECK (facility_type IN ('loan', 'overdraft', 'guarantee', 'other')),
    allocation_date TIMESTAMPTZ NOT NULL,
    allocated_amount BIGINT NOT NULL, -- Cents
    currency_code VARCHAR(3) NOT NULL,
    allocation_percentage DECIMAL(10, 4) NOT NULL, -- e.g., 100.00
    allocation_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (allocation_status IN ('active', 'partial_release', 'full_release', 'transferred', 'disputed')),
    release_date TIMESTAMPTZ,
    release_reason TEXT,
    realization_date TIMESTAMPTZ,
    realization_amount BIGINT, -- Amount realized in case of default
    allocation_priority SMALLINT NOT NULL DEFAULT 1, -- 1 = first position, 2 = second, etc.
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_collateral_allocations_collateral ON collateral.collateral_allocations(collateral_id);
CREATE INDEX idx_collateral_allocations_account ON collateral.collateral_allocations(account_id);
CREATE INDEX idx_collateral_allocations_status ON collateral.collateral_allocations(allocation_status);
CREATE INDEX idx_collateral_allocations_facility ON collateral.collateral_allocations(facility_code);

COMMENT ON TABLE collateral.collateral_allocations IS 'Collateral pledges to specific loans, overdrafts, or other facilities';

-- LTV Calculations and Monitoring
CREATE TABLE IF NOT EXISTS collateral.ltv_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_code VARCHAR(50) NOT NULL UNIQUE,
    calculation_date TIMESTAMPTZ NOT NULL,
    account_id UUID NOT NULL,
    facility_code VARCHAR(50),
    loan_principal_amount BIGINT NOT NULL, -- Cents
    total_collateral_value BIGINT NOT NULL, -- Cents (sum of allocated collaterals)
    currency_code VARCHAR(3) NOT NULL,
    ltv_ratio DECIMAL(10, 4) NOT NULL GENERATED ALWAYS AS (
        CASE
            WHEN total_collateral_value > 0 THEN (loan_principal_amount::DECIMAL / total_collateral_value * 100)
            ELSE 0
        END
    ) STORED, -- Loan-to-Value ratio as percentage
    ltv_threshold DECIMAL(10, 4) NOT NULL DEFAULT 80.0, -- Regulatory or policy maximum
    ltv_compliant BOOLEAN NOT NULL GENERATED ALWAYS AS (ltv_ratio <= ltv_threshold) STORED,
    margin_of_safety_percentage DECIMAL(10, 4), -- (Collateral - Loan) / Collateral * 100
    haircut_applied DECIMAL(10, 4) DEFAULT 0, -- Valuation haircut percentage
    haircut_reason TEXT,
    calculation_status VARCHAR(50) NOT NULL DEFAULT 'current' CHECK (calculation_status IN ('current', 'monitoring', 'breached', 'resolved')),
    breach_notification_sent BOOLEAN NOT NULL DEFAULT FALSE,
    breach_notification_date TIMESTAMPTZ,
    remediation_required BOOLEAN NOT NULL DEFAULT FALSE,
    remediation_deadline TIMESTAMPTZ,
    remediation_plan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ltv_calculations_account ON collateral.ltv_calculations(account_id);
CREATE INDEX idx_ltv_calculations_date ON collateral.ltv_calculations(calculation_date);
CREATE INDEX idx_ltv_calculations_status ON collateral.ltv_calculations(calculation_status);
CREATE INDEX idx_ltv_calculations_compliant ON collateral.ltv_