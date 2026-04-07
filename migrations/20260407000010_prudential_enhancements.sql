-- BANKO Prudential BC Enhancement
-- Stress scenarios, LCR, NSFR, leverage ratio, capital buffer, RWA calculations, recovery plans, ICAAP assessments

-- Stress Test Scenarios
CREATE TABLE IF NOT EXISTS prudential.stress_scenarios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scenario_code VARCHAR(50) NOT NULL UNIQUE,
    scenario_name VARCHAR(255) NOT NULL,
    scenario_type VARCHAR(50) NOT NULL CHECK (scenario_type IN ('historical', 'hypothetical', 'adverse', 'severe_adverse', 'regulatory')),
    description TEXT,
    severity_level VARCHAR(20) NOT NULL CHECK (severity_level IN ('baseline', 'moderate', 'severe', 'extreme')),
    scenario_date TIMESTAMPTZ NOT NULL,
    assumptions_jsonb JSONB NOT NULL, -- Interest rates, FX, credit spreads, etc.
    created_by VARCHAR(255),
    approved_by VARCHAR(255),
    approved_date TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_stress_scenarios_type ON prudential.stress_scenarios(scenario_type);
CREATE INDEX idx_stress_scenarios_active ON prudential.stress_scenarios(is_active);

COMMENT ON TABLE prudential.stress_scenarios IS 'Stress test scenarios for capital adequacy and liquidity analysis';

-- Liquidity Coverage Ratio (LCR) calculations
CREATE TABLE IF NOT EXISTS prudential.lcr_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_code VARCHAR(50) NOT NULL UNIQUE,
    calculation_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL, -- Daily, Weekly, etc.
    scenario_id UUID REFERENCES prudential.stress_scenarios(id),
    high_quality_liquid_assets_hqla BIGINT NOT NULL,
    cash_outflows_30_days BIGINT NOT NULL,
    lcr_ratio DECIMAL(10, 4) NOT NULL, -- HQLA / Cash Outflows
    lcr_threshold DECIMAL(10, 4) NOT NULL DEFAULT 100.0, -- Regulatory minimum (100%)
    lcr_compliant BOOLEAN NOT NULL GENERATED ALWAYS AS (lcr_ratio >= lcr_threshold) STORED,
    level_1_assets BIGINT,
    level_2a_assets BIGINT,
    level_2b_assets BIGINT,
    retail_deposits_outflow BIGINT,
    wholesale_deposits_outflow BIGINT,
    unsecured_wholesale_funding_outflow BIGINT,
    secured_wholesale_funding_outflow BIGINT,
    calculation_status VARCHAR(50) NOT NULL DEFAULT 'preliminary' CHECK (calculation_status IN ('preliminary', 'draft', 'reviewed', 'approved', 'final')),
    approver_name VARCHAR(255),
    approval_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_lcr_calculations_date ON prudential.lcr_calculations(calculation_date);
CREATE INDEX idx_lcr_calculations_status ON prudential.lcr_calculations(calculation_status);
CREATE INDEX idx_lcr_calculations_compliant ON prudential.lcr_calculations(lcr_compliant);

COMMENT ON TABLE prudential.lcr_calculations IS 'Liquidity Coverage Ratio (LCR) BCBS Basel III Pillar 1 calculations';

-- Net Stable Funding Ratio (NSFR) calculations
CREATE TABLE IF NOT EXISTS prudential.nsfr_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_code VARCHAR(50) NOT NULL UNIQUE,
    calculation_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL, -- Monthly, Quarterly, etc.
    scenario_id UUID REFERENCES prudential.stress_scenarios(id),
    available_stable_funding BIGINT NOT NULL,
    required_stable_funding BIGINT NOT NULL,
    nsfr_ratio DECIMAL(10, 4) NOT NULL, -- ASF / RSF
    nsfr_threshold DECIMAL(10, 4) NOT NULL DEFAULT 100.0, -- Regulatory minimum (100%)
    nsfr_compliant BOOLEAN NOT NULL GENERATED ALWAYS AS (nsfr_ratio >= nsfr_threshold) STORED,
    stable_retail_deposits BIGINT,
    less_stable_retail_deposits BIGINT,
    wholesale_funding_operational_deposits BIGINT,
    wholesale_funding_other BIGINT,
    operational_requirements BIGINT,
    contingent_funding BIGINT,
    calculation_status VARCHAR(50) NOT NULL DEFAULT 'preliminary' CHECK (calculation_status IN ('preliminary', 'draft', 'reviewed', 'approved', 'final')),
    approver_name VARCHAR(255),
    approval_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_nsfr_calculations_date ON prudential.nsfr_calculations(calculation_date);
CREATE INDEX idx_nsfr_calculations_status ON prudential.nsfr_calculations(calculation_status);
CREATE INDEX idx_nsfr_calculations_compliant ON prudential.nsfr_calculations(nsfr_compliant);

COMMENT ON TABLE prudential.nsfr_calculations IS 'Net Stable Funding Ratio (NSFR) BCBS Basel III Pillar 1 calculations';

-- Leverage Ratio (non-risk-weighted) calculations
CREATE TABLE IF NOT EXISTS prudential.leverage_ratio_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_code VARCHAR(50) NOT NULL UNIQUE,
    calculation_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL,
    scenario_id UUID REFERENCES prudential.stress_scenarios(id),
    tier_1_capital BIGINT NOT NULL,
    total_exposure BIGINT NOT NULL, -- Sum of all exposures (non-risk-weighted)
    leverage_ratio DECIMAL(10, 4) NOT NULL, -- Tier 1 / Total Exposure
    leverage_threshold DECIMAL(10, 4) NOT NULL DEFAULT 3.0, -- Regulatory minimum (3%)
    leverage_compliant BOOLEAN NOT NULL GENERATED ALWAYS AS (leverage_ratio >= leverage_threshold) STORED,
    on_balance_sheet_assets BIGINT,
    derivative_exposures BIGINT,
    securities_financing_transactions_exposures BIGINT,
    off_balance_sheet_exposures BIGINT,
    calculation_status VARCHAR(50) NOT NULL DEFAULT 'preliminary' CHECK (calculation_status IN ('preliminary', 'draft', 'reviewed', 'approved', 'final')),
    approver_name VARCHAR(255),
    approval_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_leverage_ratio_date ON prudential.leverage_ratio_calculations(calculation_date);
CREATE INDEX idx_leverage_ratio_status ON prudential.leverage_ratio_calculations(calculation_status);
CREATE INDEX idx_leverage_ratio_compliant ON prudential.leverage_ratio_calculations(leverage_compliant);

COMMENT ON TABLE prudential.leverage_ratio_calculations IS 'Leverage Ratio (non-risk-weighted) calculations';

-- Capital Buffers and Thresholds
CREATE TABLE IF NOT EXISTS prudential.capital_buffers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    buffer_code VARCHAR(50) NOT NULL UNIQUE,
    buffer_type VARCHAR(50) NOT NULL CHECK (buffer_type IN ('capital_conservation', 'countercyclical', 'systemic_risk', 'g_sib', 'capital_adequacy')),
    buffer_name VARCHAR(255) NOT NULL,
    minimum_ratio DECIMAL(10, 4) NOT NULL, -- As percentage of RWA
    current_ratio DECIMAL(10, 4),
    ratio_date TIMESTAMPTZ,
    is_breached BOOLEAN NOT NULL DEFAULT FALSE,
    breach_date TIMESTAMPTZ,
    remediation_plan TEXT,
    remediation_deadline TIMESTAMPTZ,
    buffer_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (buffer_status IN ('active', 'breached', 'under_remediation', 'restored')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_capital_buffers_type ON prudential.capital_buffers(buffer_type);
CREATE INDEX idx_capital_buffers_status ON prudential.capital_buffers(buffer_status);

COMMENT ON TABLE prudential.capital_buffers IS 'Capital buffer requirements and breaches (CET1, Tier 1, Total Capital)';

-- Risk-Weighted Assets (RWA) Calculations by exposure class
CREATE TABLE IF NOT EXISTS prudential.rwa_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    calculation_code VARCHAR(50) NOT NULL UNIQUE,
    calculation_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL,
    exposure_class VARCHAR(50) NOT NULL CHECK (exposure_class IN ('central_governments', 'institutions', 'corporates', 'retail', 'equity', 'securitization', 'trading_book', 'operational_risk')),
    total_exposure_amount BIGINT NOT NULL,
    average_risk_weight DECIMAL(10, 4) NOT NULL,
    rwa BIGINT NOT NULL GENERATED ALWAYS AS (CAST(total_exposure_amount * (average_risk_weight / 100.0) AS BIGINT)) STORED,
    credit_risk_rwa BIGINT,
    market_risk_rwa BIGINT,
    operational_risk_rwa BIGINT,
    calculation_method VARCHAR(50) NOT NULL CHECK (calculation_method IN ('standardized', 'irb_foundation', 'irb_advanced')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_rwa_calculations_date ON prudential.rwa_calculations(calculation_date);
CREATE INDEX idx_rwa_calculations_exposure_class ON prudential.rwa_calculations(exposure_class);

COMMENT ON TABLE prudential.rwa_calculations IS 'Risk-Weighted Assets calculations by exposure class';

-- Recovery Plans
CREATE TABLE IF NOT EXISTS prudential.recovery_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recovery_plan_code VARCHAR(50) NOT NULL UNIQUE,
    recovery_plan_version VARCHAR(20) NOT NULL,
    recovery_plan_type VARCHAR(50) NOT NULL CHECK (recovery_plan_type IN ('stress_recovery', 'liquidity_recovery', 'capital_recovery', 'integrated')),
    plan_effective_date TIMESTAMPTZ NOT NULL,
    plan_review_frequency VARCHAR(50) NOT NULL CHECK (plan_review_frequency IN ('annual', 'bi_annual', 'on_demand')),
    next_review_date TIMESTAMPTZ NOT NULL,
    recovery_triggers JSONB NOT NULL, -- e.g., {capital_ratio_falls_below: 8, lcr_falls_below: 100}
    recovery_measures TEXT NOT NULL, -- Detailed description of recovery actions
    board_approval_date TIMESTAMPTZ,
    board_approved_by VARCHAR(255),
    plan_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (plan_status IN ('draft', 'review', 'approved', 'active', 'superseded')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recovery_plans_type ON prudential.recovery_plans(recovery_plan_type);
CREATE INDEX idx_recovery_plans_status ON prudential.recovery_plans(plan_status);

COMMENT ON TABLE prudential.recovery_plans IS 'Recovery plans for stress scenarios and capital/liquidity restoration';

-- ICAAP (Internal Capital Adequacy Assessment Process) Assessment
CREATE TABLE IF NOT EXISTS prudential.icaap_assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_code VARCHAR(50) NOT NULL UNIQUE,
    assessment_date TIMESTAMPTZ NOT NULL,
    assessment_year SMALLINT NOT NULL,
    assessment_period_start TIMESTAMPTZ NOT NULL,
    assessment_period_end TIMESTAMPTZ NOT NULL,
    capital_adequacy_opinion VARCHAR(50) NOT NULL CHECK (capital_adequacy_opinion IN ('adequate', 'adequate_with_conditions', 'inadequate', 'under_review')),
    total_capital_required_percentage DECIMAL(10, 4) NOT NULL,
    pillar_1_capital_ratio DECIMAL(10, 4),
    pillar_2_capital_requirement DECIMAL(10, 4),
    systemic_risk_buffer DECIMAL(10, 4),
    current_capital_ratio DECIMAL(10, 4) NOT NULL,
    capital_surplus_shortfall BIGINT,
    key_risks_identified TEXT,
    risk_mitigation_strategies TEXT,
    board_approval_date TIMESTAMPTZ,
    board_approved_by VARCHAR(255),
    regulatory_authority_submission_date TIMESTAMPTZ,
    regulatory_feedback TEXT,
    assessment_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (assessment_status IN ('draft', 'review', 'board_approved', 'submitted', 'feedback_received', 'closed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_icaap_assessments_year ON prudential.icaap_assessments(assessment_year);
CREATE INDEX idx_icaap_assessments_status ON prudential.icaap_assessments(assessment_status);

COMMENT ON TABLE prudential.icaap_assessments IS 'ICAAP (Internal Capital Adequacy Assessment Process) annual assessments';

-- Prudential Policy Exceptions and Waivers
CREATE TABLE IF NOT EXISTS prudential.prudential_exceptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exception_code VARCHAR(50) NOT NULL UNIQUE,
    exception_type VARCHAR(50) NOT NULL CHECK (exception_type IN ('lcr_breach', 'nsfr_breach', 'leverage_ratio_breach', 'rwa_concentration', 'limit_breach')),
    exception_reason TEXT NOT NULL,
    initial_detection_date TIMESTAMPTZ NOT NULL,
    exception_start_date TIMESTAMPTZ NOT NULL,
    target_remediation_date TIMESTAMPTZ NOT NULL,
    remediation_plan TEXT,
    remediation_progress SMALLINT, -- Percentage completed
    exception_status VARCHAR(50) NOT NULL DEFAULT 'reported' CHECK (exception_status IN ('reported', 'under_remediation', 'remediated', 'waived', 'escalated')),
    regulatory_notification_required BOOLEAN NOT NULL DEFAULT FALSE,
    regulatory_notification_date TIMESTAMPTZ,
    exception_approved_by VARCHAR(255),
    exception_approval_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_prudential_exceptions_type ON prudential.prudential_exceptions(exception_type);
CREATE INDEX idx_prudential_exceptions_status ON prudential.prudential_exceptions(exception_status);

COMMENT ON TABLE prudential.prudential_exceptions IS 'Prudential policy exceptions and breach remediation tracking';
