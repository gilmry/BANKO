-- BANKO Cash Management BC Schema
-- Sweep accounts, cash pools, forecasting, liquidity management

-- Sweep Accounts
CREATE TABLE IF NOT EXISTS cash_management.sweep_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sweep_account_code VARCHAR(50) NOT NULL UNIQUE,
    parent_account_id UUID NOT NULL, -- Master account
    sweep_subaccount_id UUID, -- Sub-account for sweep operations
    sweep_account_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (sweep_account_status IN ('active', 'inactive', 'suspended', 'closed')),
    sweep_type VARCHAR(50) NOT NULL CHECK (sweep_type IN ('zero_balance', 'target_balance', 'interest_sweep', 'reserve_sweep')),
    sweep_frequency VARCHAR(50) NOT NULL CHECK (sweep_frequency IN ('daily', 'weekly', 'monthly', 'on_demand')),
    target_balance_amount BIGINT, -- Cents (for target balance sweeps)
    sweep_trigger_threshold BIGINT, -- Trigger amount for sweep initiation
    currency_code VARCHAR(3) NOT NULL,
    sweep_direction VARCHAR(50) NOT NULL CHECK (sweep_direction IN ('in', 'out', 'bidirectional')),
    maximum_sweep_limit BIGINT,
    minimum_sweep_amount BIGINT,
    sweep_execution_time TIME,
    sweep_destination_account_id UUID, -- Where sweep amounts go
    sweep_source_account_id UUID, -- Where sweep amounts come from
    interest_treatment VARCHAR(50) CHECK (interest_treatment IN ('accrue_on_parent', 'accrue_on_sweep', 'daily_distribution')),
    fee_structure JSONB, -- {sweep_fee: 500, monthly_fee: 10000}
    auto_sweep_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    next_sweep_date DATE,
    last_sweep_date TIMESTAMPTZ,
    last_sweep_amount BIGINT,
    ytd_sweep_count SMALLINT DEFAULT 0,
    ytd_total_swept BIGINT DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sweep_accounts_parent ON cash_management.sweep_accounts(parent_account_id);
CREATE INDEX idx_sweep_accounts_status ON cash_management.sweep_accounts(sweep_account_status);
CREATE INDEX idx_sweep_accounts_type ON cash_management.sweep_accounts(sweep_type);

COMMENT ON TABLE cash_management.sweep_accounts IS 'Zero-balance and target-balance sweep account configurations';

-- Cash Pools
CREATE TABLE IF NOT EXISTS cash_management.cash_pools (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cash_pool_code VARCHAR(50) NOT NULL UNIQUE,
    cash_pool_name VARCHAR(255) NOT NULL,
    cash_pool_type VARCHAR(50) NOT NULL CHECK (cash_pool_type IN ('notional_pool', 'physical_pool', 'regional_pool', 'currency_pool')),
    pool_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (pool_status IN ('active', 'inactive', 'restructuring', 'closed')),
    pool_administrator_id UUID NOT NULL, -- Master customer
    pool_administrator_name VARCHAR(255) NOT NULL,
    pool_currency_code VARCHAR(3) NOT NULL,
    pool_established_date DATE NOT NULL,
    pool_end_date DATE,
    master_account_id UUID NOT NULL, -- Central pool account
    participant_count SMALLINT NOT NULL DEFAULT 1,
    minimum_participant_balance BIGINT,
    pool_interest_rate DECIMAL(10, 4),
    interest_distribution_method VARCHAR(50) CHECK (interest_distribution_method IN ('pro_rata', 'tiered', 'equal_distribution', 'performance_based')),
    interest_distribution_frequency VARCHAR(50) NOT NULL CHECK (interest_distribution_frequency IN ('daily', 'weekly', 'monthly', 'quarterly')),
    pool_fees_annual BIGINT,
    pool_fee_structure JSONB, -- {setup_fee: 500000, monthly_fee: 50000}
    liquidity_requirement DECIMAL(10, 4), -- Minimum liquidity %
    concentration_limit DECIMAL(10, 4), -- Max % any participant can be
    overdraft_permitted BOOLEAN NOT NULL DEFAULT FALSE,
    overdraft_limit BIGINT,
    overdraft_rate DECIMAL(10, 4),
    sweep_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    settlement_account_id UUID,
    settlement_frequency VARCHAR(50) CHECK (settlement_frequency IN ('daily', 'weekly', 'monthly')),
    documentation_reference VARCHAR(255),
    pool_approval_date DATE,
    regulatory_approval_date DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cash_pools_admin ON cash_management.cash_pools(pool_administrator_id);
CREATE INDEX idx_cash_pools_status ON cash_management.cash_pools(pool_status);
CREATE INDEX idx_cash_pools_type ON cash_management.cash_pools(pool_type);
CREATE INDEX idx_cash_pools_master ON cash_management.cash_pools(master_account_id);

COMMENT ON TABLE cash_management.cash_pools IS 'Cash pool configurations for notional and physical pooling arrangements';

-- Cash Pool Participants
CREATE TABLE IF NOT EXISTS cash_management.cash_pool_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cash_pool_id UUID NOT NULL REFERENCES cash_management.cash_pools(id),
    participant_customer_id UUID NOT NULL,
    participant_account_id UUID NOT NULL,
    participant_name VARCHAR(255) NOT NULL,
    participant_join_date DATE NOT NULL,
    participant_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (participant_status IN ('active', 'suspended', 'exiting', 'exited')),
    participant_exit_date DATE,
    pool_share_percentage DECIMAL(10, 4) NOT NULL, -- Share in pool interests/fees
    account_balance BIGINT NOT NULL DEFAULT 0,
    ytd_interest_earned BIGINT,
    ytd_fees_charged BIGINT,
    account_type_in_pool VARCHAR(50) CHECK (account_type_in_pool IN ('master_account', 'participant_account', 'settlement_account')),
    interest_calculation_method VARCHAR(50) CHECK (interest_calculation_method IN ('weighted_average', 'daily_balance', 'tiered')),
    interest_bearing BOOLEAN NOT NULL DEFAULT TRUE,
    overdraft_limit_participant BIGINT,
    overdraft_authorized BOOLEAN NOT NULL DEFAULT FALSE,
    participation_agreement_signed_date DATE,
    agreement_document_file_id VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pool_participants_pool ON cash_management.cash_pool_participants(cash_pool_id);
CREATE INDEX idx_pool_participants_customer ON cash_management.cash_pool_participants(participant_customer_id);
CREATE INDEX idx_pool_participants_account ON cash_management.cash_pool_participants(participant_account_id);
CREATE INDEX idx_pool_participants_status ON cash_management.cash_pool_participants(participant_status);

COMMENT ON TABLE cash_management.cash_pool_participants IS 'Participants in cash pools with individual balance and interest tracking';

-- Cash Forecasts
CREATE TABLE IF NOT EXISTS cash_management.cash_forecasts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forecast_code VARCHAR(50) NOT NULL UNIQUE,
    account_id UUID NOT NULL,
    forecast_date TIMESTAMPTZ NOT NULL,
    forecast_period_days SMALLINT NOT NULL, -- 7, 30, 90 days
    forecast_start_date DATE NOT NULL,
    forecast_end_date DATE NOT NULL,
    forecast_type VARCHAR(50) NOT NULL CHECK (forecast_type IN ('conservative', 'base_case', 'optimistic')),
    forecasting_method VARCHAR(50) NOT NULL CHECK (forecasting_method IN ('historical_trend', 'statistical_model', 'machine_learning', 'expert_judgment', 'hybrid')),
    currency_code VARCHAR(3) NOT NULL,
    opening_balance BIGINT NOT NULL,
    projected_cash_inflows BIGINT,
    projected_cash_outflows BIGINT,
    projected_net_cash_flow BIGINT GENERATED ALWAYS AS (projected_cash_inflows - projected_cash_outflows) STORED,
    projected_closing_balance BIGINT,
    confidence_interval DECIMAL(10, 4), -- e.g., 95% confidence
    forecast_accuracy_percentage DECIMAL(10, 4), -- Based on historical accuracy
    liquidity_risk_assessment VARCHAR(50) CHECK (liquidity_risk_assessment IN ('low', 'medium', 'high', 'critical')),
    minimum_liquidity_required BIGINT,
    liquidity_buffer_amount BIGINT,
    seasonal_adjustments TEXT,
    key_forecast_assumptions TEXT,
    forecast_approver_name VARCHAR(255),
    forecast_approval_date TIMESTAMPTZ,
    forecast_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (forecast_status IN ('draft', 'approved', 'active', 'completed', 'archived')),
    actual_closing_balance BIGINT, -- After period ends
    forecast_variance BIGINT, -- Actual - Projected
    variance_analysis TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cash_forecasts_account ON cash_management.cash_forecasts(account_id);
CREATE INDEX idx_cash_forecasts_date ON cash_management.cash_forecasts(forecast_date);
CREATE INDEX idx_cash_forecasts_status ON cash_management.cash_forecasts(forecast_status);
CREATE INDEX idx_cash_forecasts_risk ON cash_management.cash_forecasts(liquidity_risk_assessment);

COMMENT ON TABLE cash_management.cash_forecasts IS 'Cash flow forecasts with projections and variance analysis';

-- Liquidity Positions
CREATE TABLE IF NOT EXISTS cash_management.liquidity_positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    liquidity_position_code VARCHAR(50) NOT NULL UNIQUE,
    position_date TIMESTAMPTZ NOT NULL,
    position_date_end TIMESTAMPTZ,
    currency_code VARCHAR(3) NOT NULL,
    position_type VARCHAR(50) NOT NULL CHECK (position_type IN ('aggregate_bank', 'aggregate_customer', 'individual_account', 'corporate_customer', 'cash_pool')),
    account_id UUID,
    customer_id UUID,
    available_liquidity_amount BIGINT NOT NULL, -- Cash and equivalents
    required_liquidity_amount BIGINT NOT NULL, -- Minimum requirements
    excess_liquidity BIGINT GENERATED ALWAYS AS (available_liquidity_amount - required_liquidity_amount) STORED,
    liquidity_coverage_ratio DECIMAL(10, 4) GENERATED ALWAYS AS (
        CASE
            WHEN required_liquidity_amount > 0 THEN (available_liquidity_amount::DECIMAL / required_liquidity_amount * 100)
            ELSE 0
        END
    ) STORED,
    liquidity_status VARCHAR(50) NOT NULL DEFAULT 'normal' CHECK (liquidity_status IN ('surplus', 'normal', 'tight', 'critical')),
    cash_and_equivalents BIGINT,
    unencumbered_securities BIGINT,
    committed_credit_facilities BIGINT,
    pledged_assets BIGINT,
    standby_liquidity_sources BIGINT,
    next_7_days_requirement BIGINT,
    next_30_days_requirement BIGINT,
    stress_tested_liquidity BIGINT, -- Under stress scenario
    stress_scenario_type VARCHAR(50) CHECK (stress_scenario_type IN ('baseline', 'moderate_stress', 'severe_stress')),
    liquidity_gap_analysis TEXT,
    contingency_plan TEXT,
    position_approver_name VARCHAR(255),
    approval_date TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_liquidity_positions_date ON cash_management.liquidity_positions(position_date);
CREATE INDEX idx_liquidity_positions_type ON cash_management.liquidity_positions(position_type);
CREATE INDEX idx_liquidity_positions_status ON cash_management.liquidity_positions(liquidity_status);
CREATE INDEX idx_liquidity_positions_account ON cash_management.liquidity_positions(account_id);
CREATE INDEX idx_liquidity_positions_customer ON cash_management.liquidity_positions(customer_id);

COMMENT ON TABLE cash_management.liquidity_positions IS 'Daily liquidity position monitoring and stress testing';

-- Funding Strategies
CREATE TABLE IF NOT EXISTS cash_management.funding_strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    funding_strategy_code VARCHAR(50) NOT NULL UNIQUE,
    strategy_name VARCHAR(255) NOT NULL,
    strategy_effective_date DATE NOT NULL,
    strategy_end_date DATE,
    strategy_type VARCHAR(50) NOT NULL CHECK (strategy_type IN ('short_term', 'medium_term', 'long_term', 'contingency')),
    strategy_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (strategy_status IN ('draft', 'approved', 'active', 'suspended', 'superseded', 'archived')),
    currency_code VARCHAR(3) NOT NULL,
    strategy_objective TEXT NOT NULL,
    target_funding_amount BIGINT NOT NULL,
    target_funding_sources JSONB, -- {deposits: 60%, bonds: 30%, market_operations: 10%}
    primary_funding_sources TEXT,
    secondary_funding_sources TEXT,
    contingency_funding_sources TEXT,
    interest_rate_risk_hedge VARCHAR(50),
    duration_target DECIMAL(10, 4), -- Target duration in years
    concentration_limits JSONB, -- {single_source_max: 20%, single_currency_max: 30%}
    diversification_requirements TEXT,
    liquidity_buffer_percentage DECIMAL(10, 4),
    stress_test_parameters JSONB,
    board_approval_date DATE,
    board_approver_name VARCHAR(255),
    regulatory_approval_date DATE,
    implementation_start_date DATE,
    implementation_status VARCHAR(50) CHECK (implementation_status IN ('not_started', 'in_progress', 'on_track', 'at_risk', 'completed')),
    key_performance_indicators JSONB,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_funding_strategies_date ON cash_management.funding_strategies(strategy_effective_date);
CREATE INDEX idx_funding_strategies_status ON cash_management.funding_strategies(strategy_status);
CREATE INDEX idx_funding_strategies_type ON cash_management.funding_strategies(strategy_type);

COMMENT ON TABLE cash_management.funding_strategies IS 'Bank-wide funding strategies and sources management';
