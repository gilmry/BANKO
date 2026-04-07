-- BANKO Securities BC Schema
-- Securities trading, portfolio management, settlements, and corporate actions

CREATE SCHEMA IF NOT EXISTS securities;

-- Securities Accounts
CREATE TABLE IF NOT EXISTS securities.securities_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    account_type VARCHAR(50) NOT NULL CHECK (account_type IN ('cash', 'margin', 'retirement', 'custody', 'managed')),
    account_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (account_status IN ('active', 'suspended', 'closed', 'restricted')),
    currency_code VARCHAR(3) NOT NULL DEFAULT 'TND',
    cash_balance BIGINT NOT NULL DEFAULT 0, -- Cents
    settled_cash BIGINT NOT NULL DEFAULT 0,
    unsettled_cash BIGINT NOT NULL DEFAULT 0,
    margin_balance BIGINT,
    margin_requirement BIGINT,
    leverage_ratio DECIMAL(10, 4),
    max_leverage DECIMAL(10, 4),
    custodian_name VARCHAR(255),
    custodian_account_number VARCHAR(100),
    nominee_name VARCHAR(255),
    statement_frequency VARCHAR(50) DEFAULT 'monthly' CHECK (statement_frequency IN ('daily', 'weekly', 'monthly', 'quarterly')),
    last_statement_date DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255)
);

CREATE INDEX idx_securities_accounts_customer ON securities.securities_accounts(customer_id);
CREATE INDEX idx_securities_accounts_account ON securities.securities_accounts(account_id);
CREATE INDEX idx_securities_accounts_status ON securities.securities_accounts(account_status);

COMMENT ON TABLE securities.securities_accounts IS 'Securities trading and custody accounts';

-- Security Holdings (Positions)
CREATE TABLE IF NOT EXISTS securities.security_holdings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    securities_account_id UUID NOT NULL REFERENCES securities.securities_accounts(id),
    security_id VARCHAR(50) NOT NULL, -- ISIN or internal code
    isin VARCHAR(12) NOT NULL UNIQUE, -- ISO 6166 ISIN format
    security_name VARCHAR(255) NOT NULL,
    security_type VARCHAR(50) NOT NULL CHECK (security_type IN ('equity', 'bond', 'mutual_fund', 'etf', 'derivative', 'other')),
    issuer_name VARCHAR(255) NOT NULL,
    issuer_country_code VARCHAR(2),
    quantity BIGINT NOT NULL, -- Number of units
    average_acquisition_price DECIMAL(19, 8) NOT NULL, -- Per unit
    current_market_price DECIMAL(19, 8) NOT NULL,
    current_market_value BIGINT NOT NULL, -- In account currency, cents
    cost_basis BIGINT NOT NULL, -- Original acquisition cost in cents
    unrealized_gain_loss BIGINT, -- Market value - cost basis
    currency_code VARCHAR(3) NOT NULL,
    acquisition_date DATE,
    maturity_date DATE,
    coupon_rate DECIMAL(10, 8),
    dividend_yield DECIMAL(10, 8),
    rating VARCHAR(10), -- S&P, Moody's, Fitch rating
    last_price_update TIMESTAMPTZ,
    exchange_code VARCHAR(10),
    sector VARCHAR(50),
    industry VARCHAR(100),
    holding_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (holding_status IN ('active', 'pledged', 'restricted', 'sold', 'expired')),
    pledged_amount BIGINT,
    collateral_allocation_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_security_holdings_account ON securities.security_holdings(securities_account_id);
CREATE INDEX idx_security_holdings_isin ON securities.security_holdings(isin);
CREATE INDEX idx_security_holdings_status ON securities.security_holdings(holding_status);
CREATE INDEX idx_security_holdings_type ON securities.security_holdings(security_type);

COMMENT ON TABLE securities.security_holdings IS 'Current positions and holdings in securities';

-- Trade Orders
CREATE TABLE IF NOT EXISTS securities.trade_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_code VARCHAR(50) NOT NULL UNIQUE,
    securities_account_id UUID NOT NULL REFERENCES securities.securities_accounts(id),
    trade_type VARCHAR(20) NOT NULL CHECK (trade_type IN ('buy', 'sell')),
    security_id VARCHAR(50) NOT NULL,
    isin VARCHAR(12) NOT NULL,
    quantity BIGINT NOT NULL,
    order_type VARCHAR(50) NOT NULL CHECK (order_type IN ('market', 'limit', 'stop_loss', 'stop_limit', 'iceberg')),
    limit_price DECIMAL(19, 8),
    stop_price DECIMAL(19, 8),
    execution_price DECIMAL(19, 8),
    currency_code VARCHAR(3) NOT NULL,
    total_order_value BIGINT, -- Cents (quantity * price)
    commission_amount BIGINT,
    commission_rate DECIMAL(10, 8),
    tax_amount BIGINT,
    net_settlement_amount BIGINT,
    order_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (order_status IN ('pending', 'partially_executed', 'executed', 'cancelled', 'rejected', 'expired')),
    execution_status VARCHAR(50) CHECK (execution_status IN ('pending', 'partial_fill', 'full_fill', 'no_fill')),
    order_date TIMESTAMPTZ NOT NULL,
    execution_date TIMESTAMPTZ,
    settlement_date DATE,
    expiry_date TIMESTAMPTZ,
    counterparty_name VARCHAR(255),
    trading_venue VARCHAR(100),
    broker_code VARCHAR(50),
    order_instructions TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_trade_orders_account ON securities.trade_orders(securities_account_id);
CREATE INDEX idx_trade_orders_isin ON securities.trade_orders(isin);
CREATE INDEX idx_trade_orders_status ON securities.trade_orders(order_status);
CREATE INDEX idx_trade_orders_date ON securities.trade_orders(order_date);

COMMENT ON TABLE securities.trade_orders IS 'Buy and sell orders for securities';

-- Settlements (T+2)
CREATE TABLE IF NOT EXISTS securities.settlements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    settlement_code VARCHAR(50) NOT NULL UNIQUE,
    trade_order_id UUID NOT NULL REFERENCES securities.trade_orders(id),
    settlement_type VARCHAR(20) NOT NULL CHECK (settlement_type IN ('buy', 'sell')),
    settlement_date DATE NOT NULL, -- T+2
    value_date DATE, -- When cash/securities transfer occurs
    quantity BIGINT NOT NULL,
    security_isin VARCHAR(12) NOT NULL,
    settlement_amount BIGINT NOT NULL, -- Cents
    currency_code VARCHAR(3) NOT NULL,
    cash_movement BIGINT, -- Positive = inflow, Negative = outflow
    securities_movement_quantity BIGINT,
    dvp_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (dvp_status IN ('pending', 'matched', 'instructed', 'settled', 'failed', 'reversed')),
    dvp_instruction_reference VARCHAR(100),
    depository_id VARCHAR(50),
    clearing_member VARCHAR(100),
    counterparty_settlement_account VARCHAR(100),
    settlement_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (settlement_status IN ('pending', 'in_progress', 'settled', 'failed', 'exception')),
    settlement_failure_reason TEXT,
    settlement_deadline TIMESTAMPTZ,
    settled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_settlements_trade ON securities.settlements(trade_order_id);
CREATE INDEX idx_settlements_date ON securities.settlements(settlement_date);
CREATE INDEX idx_settlements_status ON securities.settlements(settlement_status);
CREATE INDEX idx_settlements_dvp ON securities.settlements(dvp_status);

COMMENT ON TABLE securities.settlements IS 'T+2 settlement tracking with DVP (Delivery vs Payment) status';

-- Corporate Actions
CREATE TABLE IF NOT EXISTS securities.corporate_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    action_code VARCHAR(50) NOT NULL UNIQUE,
    isin VARCHAR(12) NOT NULL,
    security_name VARCHAR(255) NOT NULL,
    action_type VARCHAR(50) NOT NULL CHECK (action_type IN ('dividend', 'split', 'merger', 'spin_off', 'rights_offering', 'bonus_shares', 'capital_reduction', 'name_change')),
    announcement_date DATE NOT NULL,
    ex_date DATE NOT NULL, -- Last day to own for dividend/rights
    record_date DATE, -- Date of ownership record
    payment_date DATE, -- Actual payment date
    effective_date DATE,
    dividend_per_share DECIMAL(19, 8),
    dividend_currency VARCHAR(3),
    split_ratio DECIMAL(19, 8), -- Old:New ratio
    new_isin VARCHAR(12), -- If applicable
    action_status VARCHAR(50) NOT NULL DEFAULT 'announced' CHECK (action_status IN ('announced', 'ex_date_passed', 'processed', 'completed', 'cancelled')),
    gross_distribution BIGINT, -- Total dividends
    withholding_tax_rate DECIMAL(10, 8),
    net_distribution BIGINT,
    action_details JSONB, -- Additional details
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_corporate_actions_isin ON securities.corporate_actions(isin);
CREATE INDEX idx_corporate_actions_date ON securities.corporate_actions(ex_date);
CREATE INDEX idx_corporate_actions_status ON securities.corporate_actions(action_status);

COMMENT ON TABLE securities.corporate_actions IS 'Corporate actions (dividends, splits, mergers) affecting holdings';

-- Portfolio Valuations
CREATE TABLE IF NOT EXISTS securities.portfolio_valuations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    valuation_date DATE NOT NULL,
    securities_account_id UUID NOT NULL REFERENCES securities.securities_accounts(id),
    total_holdings_value BIGINT NOT NULL, -- Cents
    total_cash BIGINT NOT NULL,
    total_portfolio_value BIGINT NOT NULL, -- Holdings + Cash
    currency_code VARCHAR(3) NOT NULL,
    daily_gain_loss BIGINT,
    daily_gain_loss_percentage DECIMAL(10, 8),
    period_gain_loss BIGINT,
    period_gain_loss_percentage DECIMAL(10, 8),
    dividend_income_ytd BIGINT,
    interest_income_ytd BIGINT,
    commission_paid_ytd BIGINT,
    tax_paid_ytd BIGINT,
    diversification_score DECIMAL(10, 8), -- 0.0 - 1.0
    sector_exposure JSONB, -- {technology: 25.5, healthcare: 15.2, ...}
    geographic_exposure JSONB, -- {usa: 45.0, europe: 30.0, ...}
    largest_position_percentage DECIMAL(10, 8),
    valuation_status VARCHAR(50) NOT NULL DEFAULT 'calculated' CHECK (valuation_status IN ('calculated', 'verified', 'audited')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portfolio_valuations_account ON securities.portfolio_valuations(securities_account_id);
CREATE INDEX idx_portfolio_valuations_date ON securities.portfolio_valuations(valuation_date);

COMMENT ON TABLE securities.portfolio_valuations IS 'Daily portfolio valuations and performance metrics';

-- ISIN Validation Check Constraint
ALTER TABLE securities.security_holdings
ADD CONSTRAINT chk_valid_isin CHECK (isin ~ '^[A-Z]{2}[A-Z0-9]{9}[0-9]{1}$');

ALTER TABLE securities.corporate_actions
ADD CONSTRAINT chk_valid_isin CHECK (isin ~ '^[A-Z]{2}[A-Z0-9]{9}[0-9]{1}$');

ALTER TABLE securities.trade_orders
ADD CONSTRAINT chk_valid_isin CHECK (isin ~ '^[A-Z]{2}[A-Z0-9]{9}[0-9]{1}$');

ALTER TABLE securities.settlements
ADD CONSTRAINT chk_valid_isin CHECK (security_isin ~ '^[A-Z]{2}[A-Z0-9]{9}[0-9]{1}$');

-- Constraint: Settlement date must be 2 business days after trade
ALTER TABLE securities.settlements
ADD CONSTRAINT chk_settlement_t_plus_2 CHECK (
    (settlement_date - trade_date) >= INTERVAL '2 days'
);

-- Settlement T+2 calculation (would need trigger for automatic date calculation)
