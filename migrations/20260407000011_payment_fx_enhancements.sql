-- BANKO Payment & FX BC Enhancement
-- Payment consents (PSD3), instant payments, QR payment codes, third-party PIS, FX forwards, FX swaps, FX options, FX position limits, FX regulatory reports

-- Payment Consents (PSD3 Open Banking)
CREATE TABLE IF NOT EXISTS payment.payment_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consent_code VARCHAR(50) NOT NULL UNIQUE,
    psu_id UUID NOT NULL, -- Payment Service User
    psu_ip_address VARCHAR(45),
    psu_user_agent VARCHAR(500),
    consent_type VARCHAR(50) NOT NULL CHECK (consent_type IN ('single_payment', 'standing_order', 'variable_recurrence')),
    payment_service_provider_id UUID, -- Third-party PIS provider
    creditor_account VARCHAR(50),
    creditor_name VARCHAR(255),
    creditor_reference VARCHAR(100),
    instructed_amount BIGINT,
    instructed_currency VARCHAR(3),
    maximum_amount BIGINT, -- For variable/recurrence
    frequency VARCHAR(50), -- For standing orders: daily, weekly, monthly, etc.
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    consent_status VARCHAR(50) NOT NULL DEFAULT 'received' CHECK (consent_status IN ('received', 'valid', 'expired', 'revoked', 'rejected', 'failed')),
    consent_given_at TIMESTAMPTZ NOT NULL,
    consent_valid_until TIMESTAMPTZ,
    number_of_authorizations_used SMALLINT DEFAULT 0,
    maximum_authorizations SMALLINT,
    consent_revoked_at TIMESTAMPTZ,
    revocation_reason TEXT,
    psd3_compliance_status VARCHAR(50), -- Regulatory compliance tracking
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_consents_psu ON payment.payment_consents(psu_id);
CREATE INDEX idx_payment_consents_status ON payment.payment_consents(consent_status);
CREATE INDEX idx_payment_consents_valid_until ON payment.payment_consents(consent_valid_until);

COMMENT ON TABLE payment.payment_consents IS 'PSD3 payment consents for single payments, standing orders, and variable recurrences';

-- Instant Payments (Request for Payment / RfP)
CREATE TABLE IF NOT EXISTS payment.instant_payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instant_payment_ref VARCHAR(50) NOT NULL UNIQUE,
    payer_customer_id UUID NOT NULL,
    payer_account VARCHAR(50) NOT NULL,
    payee_account VARCHAR(50) NOT NULL,
    payee_name VARCHAR(255),
    payee_bank_bic VARCHAR(11),
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    payment_method VARCHAR(50) NOT NULL CHECK (payment_method IN ('sepa_instant', 'domestic_instant', 'cross_border_instant', 'rtgs')),
    invoice_reference VARCHAR(100),
    end_to_end_reference VARCHAR(100),
    purpose_code VARCHAR(10),
    priority VARCHAR(50) DEFAULT 'normal' CHECK (priority IN ('normal', 'high', 'low')),
    execution_request_time TIMESTAMPTZ NOT NULL,
    settlement_time TIMESTAMPTZ,
    settlement_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (settlement_status IN ('pending', 'processing', 'settled', 'returned', 'rejected', 'failed')),
    return_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_instant_payments_payer ON payment.instant_payments(payer_customer_id);
CREATE INDEX idx_instant_payments_status ON payment.instant_payments(settlement_status);
CREATE INDEX idx_instant_payments_time ON payment.instant_payments(execution_request_time);

COMMENT ON TABLE payment.instant_payments IS 'Instant payments (SEPA SCT Instant, domestic real-time transfers)';

-- QR Code Payment Requests
CREATE TABLE IF NOT EXISTS payment.qr_payment_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    qr_code_id VARCHAR(100) NOT NULL UNIQUE,
    qr_code_type VARCHAR(50) NOT NULL CHECK (qr_code_type IN ('static', 'dynamic', 'merchant_presented', 'customer_initiated')),
    qr_code_image BYTEA,
    qr_code_hash VARCHAR(255),
    merchant_id UUID NOT NULL,
    merchant_name VARCHAR(255),
    merchant_account VARCHAR(50),
    amount BIGINT,
    currency VARCHAR(3) DEFAULT 'TND',
    reference_number VARCHAR(100),
    description TEXT,
    validity_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    validity_end TIMESTAMPTZ,
    maximum_amount BIGINT, -- For dynamic codes
    minimum_amount BIGINT,
    scan_count BIGINT DEFAULT 0,
    payment_count BIGINT DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_qr_payment_codes_merchant ON payment.qr_payment_codes(merchant_id);
CREATE INDEX idx_qr_payment_codes_active ON payment.qr_payment_codes(is_active);
CREATE INDEX idx_qr_payment_codes_validity ON payment.qr_payment_codes(validity_start, validity_end);

COMMENT ON TABLE payment.qr_payment_codes IS 'QR code payment initiators for in-store and online payments';

-- Third-Party Payment Initiation Service (PIS) Providers
CREATE TABLE IF NOT EXISTS payment.third_party_pis_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_code VARCHAR(50) NOT NULL UNIQUE,
    provider_name VARCHAR(255) NOT NULL,
    provider_type VARCHAR(50) NOT NULL CHECK (provider_type IN ('payment_processor', 'fintech', 'acquirer', 'aggregator')),
    provider_bic VARCHAR(11),
    country VARCHAR(100),
    regulatory_license_number VARCHAR(100),
    license_expiry_date TIMESTAMPTZ,
    authentication_method VARCHAR(50) NOT NULL CHECK (authentication_method IN ('oauth2', 'api_key', 'mutual_tls', 'jwt')),
    api_endpoint VARCHAR(500),
    is_approved BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    max_transaction_amount BIGINT,
    daily_volume_limit BIGINT,
    monthly_volume_limit BIGINT,
    risk_rating VARCHAR(20), -- Low, Medium, High
    last_audit_date TIMESTAMPTZ,
    next_audit_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pis_providers_type ON payment.third_party_pis_providers(provider_type);
CREATE INDEX idx_pis_providers_active ON payment.third_party_pis_providers(is_active);

COMMENT ON TABLE payment.third_party_pis_providers IS 'Approved third-party payment initiation service providers';

-- Foreign Exchange Forward Contracts
CREATE TABLE IF NOT EXISTS payment.fx_forward_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forward_contract_ref VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    contract_type VARCHAR(50) NOT NULL CHECK (contract_type IN ('fixed_rate_forward', 'flexible_forward', 'window_forward')),
    base_currency VARCHAR(3) NOT NULL,
    quote_currency VARCHAR(3) NOT NULL,
    base_amount BIGINT NOT NULL,
    quote_amount BIGINT NOT NULL,
    forward_rate DECIMAL(15, 8) NOT NULL,
    spot_rate DECIMAL(15, 8),
    forward_premium_discount DECIMAL(10, 6), -- Points or percentage
    contract_date TIMESTAMPTZ NOT NULL,
    settlement_date TIMESTAMPTZ NOT NULL,
    settlement_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (settlement_status IN ('pending', 'partially_settled', 'settled', 'cancelled', 'defaulted')),
    settlement_account_base VARCHAR(50),
    settlement_account_quote VARCHAR(50),
    mtm_value BIGINT, -- Mark-to-market unrealized gain/loss
    realised_gain_loss BIGINT,
    counterparty_bank_bic VARCHAR(11),
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('active', 'awaiting_settlement', 'settled', 'cancelled', 'matured')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fx_forwards_customer ON payment.fx_forward_contracts(customer_id);
CREATE INDEX idx_fx_forwards_settlement_date ON payment.fx_forward_contracts(settlement_date);
CREATE INDEX idx_fx_forwards_status ON payment.fx_forward_contracts(contract_status);

COMMENT ON TABLE payment.fx_forward_contracts IS 'Foreign exchange forward contracts with mark-to-market tracking';

-- Foreign Exchange Swap Contracts
CREATE TABLE IF NOT EXISTS payment.fx_swap_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    swap_contract_ref VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    base_currency VARCHAR(3) NOT NULL,
    quote_currency VARCHAR(3) NOT NULL,
    notional_base_amount BIGINT NOT NULL,
    notional_quote_amount BIGINT NOT NULL,
    near_leg_settlement_date TIMESTAMPTZ NOT NULL,
    far_leg_settlement_date TIMESTAMPTZ NOT NULL,
    near_leg_rate DECIMAL(15, 8) NOT NULL,
    far_leg_rate DECIMAL(15, 8) NOT NULL,
    swap_points DECIMAL(10, 6), -- Difference between near and far legs
    interest_differential DECIMAL(10, 6), -- Interest rate difference between currencies
    contract_date TIMESTAMPTZ NOT NULL,
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('active', 'partially_settled', 'fully_settled', 'cancelled')),
    near_leg_status VARCHAR(50) DEFAULT 'pending',
    far_leg_status VARCHAR(50) DEFAULT 'pending',
    mtm_value BIGINT,
    counterparty_bank_bic VARCHAR(11),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fx_swaps_customer ON payment.fx_swap_contracts(customer_id);
CREATE INDEX idx_fx_swaps_settlement_dates ON payment.fx_swap_contracts(near_leg_settlement_date, far_leg_settlement_date);

COMMENT ON TABLE payment.fx_swap_contracts IS 'Foreign exchange swap contracts (simultaneous buy-sell pairs)';

-- Foreign Exchange Option Contracts
CREATE TABLE IF NOT EXISTS payment.fx_option_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    option_contract_ref VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    option_type VARCHAR(20) NOT NULL CHECK (option_type IN ('call', 'put')),
    option_style VARCHAR(20) NOT NULL CHECK (option_style IN ('european', 'american', 'bermuda')),
    base_currency VARCHAR(3) NOT NULL,
    quote_currency VARCHAR(3) NOT NULL,
    notional_amount BIGINT NOT NULL,
    strike_price DECIMAL(15, 8) NOT NULL,
    current_spot_price DECIMAL(15, 8),
    option_premium BIGINT NOT NULL,
    premium_currency VARCHAR(3),
    valuation_method VARCHAR(50) CHECK (valuation_method IN ('black_scholes', 'binomial', 'monte_carlo')),
    expiration_date TIMESTAMPTZ NOT NULL,
    exercise_date TIMESTAMPTZ,
    contract_date TIMESTAMPTZ NOT NULL,
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('active', 'exercised', 'expired_unexercised', 'cancelled')),
    intrinsic_value BIGINT, -- Current value if exercised
    time_value BIGINT, -- Premium - intrinsic value
    mtm_value BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fx_options_customer ON payment.fx_option_contracts(customer_id);
CREATE INDEX idx_fx_options_expiration ON payment.fx_option_contracts(expiration_date);
CREATE INDEX idx_fx_options_status ON payment.fx_option_contracts(contract_status);

COMMENT ON TABLE payment.fx_option_contracts IS 'Foreign exchange option contracts (calls, puts, European/American)';

-- FX Position Limits and Monitoring
CREATE TABLE IF NOT EXISTS payment.fx_position_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    limit_code VARCHAR(50) NOT NULL UNIQUE,
    currency_pair VARCHAR(6) NOT NULL, -- e.g., 'EURUSD'
    limit_type VARCHAR(50) NOT NULL CHECK (limit_type IN ('overnight', 'intraday', 'deal_size', 'counterparty', 'market_risk')),
    limit_amount BIGINT NOT NULL,
    currency VARCHAR(3),
    current_position BIGINT NOT NULL DEFAULT 0,
    limit_utilization_percentage DECIMAL(5, 2) GENERATED ALWAYS AS (CAST(current_position * 100.0 / limit_amount AS DECIMAL(5, 2))) STORED,
    limit_breached BOOLEAN NOT NULL DEFAULT FALSE,
    breach_notification_threshold DECIMAL(5, 2) DEFAULT 80.0, -- Percentage
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    enforcement_action VARCHAR(50) CHECK (enforcement_action IN ('warning', 'pause_new_deals', 'force_reduction', 'escalate')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fx_position_limits_currency_pair ON payment.fx_position_limits(currency_pair);
CREATE INDEX idx_fx_position_limits_breached ON payment.fx_position_limits(limit_breached);

COMMENT ON TABLE payment.fx_position_limits IS 'FX position limits by currency pair and limit type';

-- Foreign Exchange Regulatory Reports
CREATE TABLE IF NOT EXISTS payment.fx_regulatory_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(50) NOT NULL UNIQUE,
    report_type VARCHAR(50) NOT NULL CHECK (report_type IN ('daily_position', 'monthly_exposure', 'quarterly_concentration', 'annual_activity')),
    reporting_date TIMESTAMPTZ NOT NULL,
    reporting_period VARCHAR(20) NOT NULL,
    total_fx_exposure BIGINT,
    fx_positions_by_currency JSONB, -- {EUR: amount, USD: amount, ...}
    largest_fx_position_currency VARCHAR(3),
    largest_fx_position_amount BIGINT,
    average_daily_volume BIGINT,
    largest_deal_size BIGINT,
    counterparty_concentration JSONB, -- Top 10 counterparties
    vat_exposure BIGINT,
    cad_exposure BIGINT,
    market_risk_var_95 BIGINT, -- Value at Risk 95% confidence
    regulatory_limit_breaches SMALLINT DEFAULT 0,
    report_submitted_to_bcn BOOLEAN NOT NULL DEFAULT FALSE,
    submission_date TIMESTAMPTZ,
    report_status VARCHAR(50) NOT NULL DEFAULT 'draft' CHECK (report_status IN ('draft', 'review', 'approved', 'submitted', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fx_reports_date ON payment.fx_regulatory_reports(reporting_date);
CREATE INDEX idx_fx_reports_status ON payment.fx_regulatory_reports(report_status);

COMMENT ON TABLE payment.fx_regulatory_reports IS 'FX position and exposure regulatory reports for BCT';
