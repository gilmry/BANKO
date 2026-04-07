-- BANKO Islamic Banking BC Schema
-- Murabaha, Ijara, Musharaka, Mudaraba, Sukuk, Zakat, Sharia Board

-- Murabaha (Cost Plus) Contracts
CREATE TABLE IF NOT EXISTS islamic_banking.murabaha_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    murabaha_contract_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    account_id UUID NOT NULL,
    contract_date TIMESTAMPTZ NOT NULL,
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('pending_approval', 'active', 'completed', 'defaulted', 'cancelled')),
    asset_description TEXT NOT NULL,
    asset_cost_price BIGINT NOT NULL, -- Cents
    murabaha_markup_amount BIGINT NOT NULL, -- Cents
    murabaha_markup_percentage DECIMAL(10, 4) NOT NULL,
    total_contract_price BIGINT NOT NULL GENERATED ALWAYS AS (asset_cost_price + murabaha_markup_amount) STORED,
    currency_code VARCHAR(3) NOT NULL,
    payment_terms JSONB NOT NULL, -- {installment_amount: 50000, frequency: 'monthly', num_installments: 24}
    number_of_installments SMALLINT NOT NULL,
    installment_amount BIGINT NOT NULL,
    installment_frequency VARCHAR(50) NOT NULL CHECK (installment_frequency IN ('daily', 'weekly', 'monthly', 'quarterly', 'annually')),
    next_installment_date DATE,
    contract_end_date DATE NOT NULL,
    asset_ownership_transfer_date DATE,
    asset_delivery_date DATE,
    late_payment_penalty_rate DECIMAL(10, 4), -- As percentage
    late_payment_penalty_type VARCHAR(50) CHECK (late_payment_penalty_type IN ('deferral_penalty', 'fixed_rate', 'percentage')),
    penalty_distribution TEXT, -- e.g., 'charity', 'bank', 'reserve'
    sharia_board_approval_date DATE,
    approver_name VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_murabaha_customer ON islamic_banking.murabaha_contracts(customer_id);
CREATE INDEX idx_murabaha_account ON islamic_banking.murabaha_contracts(account_id);
CREATE INDEX idx_murabaha_status ON islamic_banking.murabaha_contracts(contract_status);
CREATE INDEX idx_murabaha_end_date ON islamic_banking.murabaha_contracts(contract_end_date);

COMMENT ON TABLE islamic_banking.murabaha_contracts IS 'Murabaha (cost-plus) financing contracts with Islamic financing terms';

-- Ijara (Lease) Contracts
CREATE TABLE IF NOT EXISTS islamic_banking.ijara_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ijara_contract_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    account_id UUID NOT NULL,
    contract_date TIMESTAMPTZ NOT NULL,
    contract_type VARCHAR(50) NOT NULL CHECK (contract_type IN ('ijara_muntahia_bittamleek', 'ijara_muntahia_bighair', 'operational_lease')),
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('pending_approval', 'active', 'completed', 'defaulted', 'cancelled')),
    leased_asset_description TEXT NOT NULL,
    asset_cost_price BIGINT NOT NULL, -- Cents
    ijara_rental_amount BIGINT NOT NULL, -- Per period
    rental_frequency VARCHAR(50) NOT NULL CHECK (rental_frequency IN ('monthly', 'quarterly', 'semi_annually', 'annually')),
    lease_term_months SMALLINT NOT NULL,
    total_rental_payments BIGINT NOT NULL GENERATED ALWAYS AS (ijara_rental_amount * CAST(lease_term_months / 12.0 * (CASE WHEN rental_frequency = 'monthly' THEN 12 WHEN rental_frequency = 'quarterly' THEN 4 ELSE 1 END) AS BIGINT)) STORED,
    currency_code VARCHAR(3) NOT NULL,
    lessor_name VARCHAR(255) NOT NULL,
    lessee_name VARCHAR(255) NOT NULL,
    lease_start_date DATE NOT NULL,
    lease_end_date DATE NOT NULL,
    ownership_transfer_date DATE, -- For ijara muntahia bittamleek
    next_payment_date DATE,
    residual_value_amount BIGINT, -- For ijara muntahia bittamleek
    residual_value_guarantee BOOLEAN,
    asset_maintenance_responsibility VARCHAR(50) CHECK (asset_maintenance_responsibility IN ('lessor', 'lessee', 'shared')),
    insurance_responsibility VARCHAR(50) CHECK (insurance_responsibility IN ('lessor', 'lessee', 'shared')),
    late_payment_penalty_rate DECIMAL(10, 4),
    penalty_distribution TEXT,
    sharia_board_approval_date DATE,
    approver_name VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ijara_customer ON islamic_banking.ijara_contracts(customer_id);
CREATE INDEX idx_ijara_account ON islamic_banking.ijara_contracts(account_id);
CREATE INDEX idx_ijara_status ON islamic_banking.ijara_contracts(contract_status);
CREATE INDEX idx_ijara_end_date ON islamic_banking.ijara_contracts(lease_end_date);

COMMENT ON TABLE islamic_banking.ijara_contracts IS 'Ijara (lease) contracts with Islamic terms and ownership transfer options';

-- Musharaka (Profit Sharing Partnership) Contracts
CREATE TABLE IF NOT EXISTS islamic_banking.musharaka_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    musharaka_contract_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    account_id UUID NOT NULL,
    contract_date TIMESTAMPTZ NOT NULL,
    contract_type VARCHAR(50) NOT NULL CHECK (contract_type IN ('permanent_musharaka', 'declining_musharaka')),
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('pending_approval', 'active', 'completed', 'defaulted', 'cancelled')),
    business_description TEXT NOT NULL,
    project_name VARCHAR(255) NOT NULL,
    bank_capital_contribution BIGINT NOT NULL, -- Cents
    customer_capital_contribution BIGINT NOT NULL,
    total_capital BIGINT NOT NULL GENERATED ALWAYS AS (bank_capital_contribution + customer_capital_contribution) STORED,
    currency_code VARCHAR(3) NOT NULL,
    bank_profit_sharing_percentage DECIMAL(10, 4) NOT NULL, -- Bank's share of profits
    customer_profit_sharing_percentage DECIMAL(10, 4) NOT NULL,
    expected_monthly_revenue BIGINT,
    expected_profit_margin_percentage DECIMAL(10, 4),
    contract_duration_months SMALLINT NOT NULL,
    contract_start_date DATE NOT NULL,
    contract_end_date DATE NOT NULL,
    profit_distribution_frequency VARCHAR(50) NOT NULL CHECK (profit_distribution_frequency IN ('monthly', 'quarterly', 'semi_annually', 'annually')),
    next_distribution_date DATE,
    musharaka_exit_clause TEXT, -- Terms for bank exit or customer buyout
    declining_percentage_per_period DECIMAL(10, 4), -- For declining musharaka
    buyout_price_formula TEXT,
    venture_performance_metrics JSONB,
    sharia_board_approval_date DATE,
    approver_name VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_musharaka_customer ON islamic_banking.musharaka_contracts(customer_id);
CREATE INDEX idx_musharaka_account ON islamic_banking.musharaka_contracts(account_id);
CREATE INDEX idx_musharaka_status ON islamic_banking.musharaka_contracts(contract_status);
CREATE INDEX idx_musharaka_end_date ON islamic_banking.musharaka_contracts(contract_end_date);

COMMENT ON TABLE islamic_banking.musharaka_contracts IS 'Musharaka (partnership) contracts with profit-sharing arrangements';

-- Mudaraba (Investment Agency) Contracts
CREATE TABLE IF NOT EXISTS islamic_banking.mudaraba_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mudaraba_contract_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    account_id UUID NOT NULL,
    contract_date TIMESTAMPTZ NOT NULL,
    contract_type VARCHAR(50) NOT NULL CHECK (contract_type IN ('restricted_mudaraba', 'unrestricted_mudaraba')),
    contract_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (contract_status IN ('pending_approval', 'active', 'completed', 'defaulted', 'cancelled')),
    investment_description TEXT NOT NULL,
    project_name VARCHAR(255) NOT NULL,
    rabb_al_mal_capital BIGINT NOT NULL, -- Customer's capital
    mudarib_contribution BIGINT NOT NULL, -- Bank's contribution (if any)
    total_investment_capital BIGINT NOT NULL GENERATED ALWAYS AS (rabb_al_mal_capital + mudarib_contribution) STORED,
    currency_code VARCHAR(3) NOT NULL,
    profit_sharing_ratio_rabb_al_mal DECIMAL(10, 4) NOT NULL, -- Customer's profit share
    profit_sharing_ratio_mudarib DECIMAL(10, 4) NOT NULL, -- Bank's profit share
    investment_period_months SMALLINT NOT NULL,
    investment_start_date DATE NOT NULL,
    investment_end_date DATE NOT NULL,
    profit_distribution_frequency VARCHAR(50) NOT NULL CHECK (profit_distribution_frequency IN ('monthly', 'quarterly', 'semi_annually', 'annually', 'at_maturity')),
    next_distribution_date DATE,
    investment_restrictions TEXT, -- e.g., 'No alcohol, gambling, pork industries'
    loss_bearing_arrangement TEXT, -- How losses are borne
    expected_return_percentage DECIMAL(10, 4),
    expected_annual_profit BIGINT,
    mudrib_management_fee_percentage DECIMAL(10, 4),
    sharia_board_approval_date DATE,
    approver_name VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mudaraba_customer ON islamic_banking.mudaraba_contracts(customer_id);
CREATE INDEX idx_mudaraba_account ON islamic_banking.mudaraba_contracts(account_id);
CREATE INDEX idx_mudaraba_status ON islamic_banking.mudaraba_contracts(contract_status);
CREATE INDEX idx_mudaraba_end_date ON islamic_banking.mudaraba_contracts(investment_end_date);

COMMENT ON TABLE islamic_banking.mudaraba_contracts IS 'Mudaraba (investment agency) contracts with capital and profit sharing';

-- Sukuk (Islamic Bonds) Issuances
CREATE TABLE IF NOT EXISTS islamic_banking.sukuk_issuances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sukuk_code VARCHAR(50) NOT NULL UNIQUE,
    sukuk_name VARCHAR(255) NOT NULL,
    sukuk_type VARCHAR(50) NOT NULL CHECK (sukuk_type IN ('murabaha_sukuk', 'ijara_sukuk', 'musharaka_sukuk', 'asset_backed_sukuk')),
    issuer_name VARCHAR(255) NOT NULL,
    issuer_customer_id UUID,
    sukuk_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (sukuk_status IN ('pending_approval', 'active', 'matured', 'redeemed', 'defaulted', 'cancelled')),
    issue_date TIMESTAMPTZ NOT NULL,
    maturity_date DATE NOT NULL,
    total_sukuk_issued BIGINT NOT NULL, -- Total units
    unit_face_value BIGINT NOT NULL, -- Cents
    total_amount_raised BIGINT NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    coupon_rate DECIMAL(10, 4) NOT NULL, -- Periodic distribution rate
    coupon_frequency VARCHAR(50) NOT NULL CHECK (coupon_frequency IN ('monthly', 'quarterly', 'semi_annually', 'annually')),
    next_coupon_payment_date DATE,
    total_coupons_paid BIGINT,
    underlying_asset_description TEXT NOT NULL,
    underlying_asset_location VARCHAR(255),
    rating_agency_rating VARCHAR(20),
    rating_date DATE,
    sharia_compliance_rating VARCHAR(50), -- e.g., 'Sharia Compliant', 'Conditionally Compliant'
    sukuk_trustee_name VARCHAR(255),
    sukuk_rating_certificate_file_id VARCHAR(255),
    sharia_board_approval_date DATE,
    approver_name VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sukuk_issuer ON islamic_banking.sukuk_issuances(issuer_customer_id);
CREATE INDEX idx_sukuk_status ON islamic_banking.sukuk_issuances(sukuk_status);
CREATE INDEX idx_sukuk_maturity ON islamic_banking.sukuk_issuances(maturity_date);
CREATE INDEX idx_sukuk_type ON islamic_banking.sukuk_issuances(sukuk_type);

COMMENT ON TABLE islamic_banking.sukuk_issuances IS 'Sukuk (Islamic bond) issuances and portfolio management';

-- Zakat Calculations and Distributions
CREATE TABLE IF NOT EXISTS islamic_banking.zakat_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    zakat_calculation_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    calculation_date TIMESTAMPTZ NOT NULL,
    islamic_year SMALLINT NOT NULL,
    zakat_year_start_date DATE NOT NULL,
    zakat_year_end_date DATE NOT NULL,
    calculation_method VARCHAR(50) NOT NULL CHECK (calculation_method IN ('net_wealth', 'income_based', 'hybrid')),
    total_assets_value BIGINT NOT NULL, -- Cents
    total_liabilities_value BIGINT NOT NULL,
    zakatable_wealth BIGINT NOT NULL, -- Assets - Liabilities subject to Zakat
    currency_code VARCHAR(3) NOT NULL,
    zakat_percentage DECIMAL(10, 4) NOT NULL DEFAULT 2.5, -- Standard 2.5%
    zakat_amount_due BIGINT NOT NULL GENERATED ALWAYS AS (CAST(zakatable_wealth * (zakat_percentage / 100.0) AS BIGINT)) STORED,
    zakat_paid_amount BIGINT NOT NULL DEFAULT 0,
    zakat_paid_date DATE,
    zakat_status VARCHAR(50) NOT NULL DEFAULT 'calculated' CHECK (zakat_status IN ('calculated', 'partial_paid', 'fully_paid', 'exempted', 'deferred')),
    zakat_distribution_method VARCHAR(50) CHECK (zakat_distribution_method IN ('direct_charitable', 'bank_managed_distribution', 'deferral')),
    distribution_recipient_details TEXT, -- Zakat beneficiaries
    zakat_certificate_issued BOOLEAN NOT NULL DEFAULT FALSE,
    certificate_file_id VARCHAR(255),
    sharia_scholar_endorsement VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_zakat_customer ON islamic_banking.zakat_calculations(customer_id);
CREATE INDEX idx_zakat_date ON islamic_banking.zakat_calculations(calculation_date);
CREATE INDEX idx_zakat_status ON islamic_banking.zakat_calculations(zakat_status);

COMMENT ON TABLE islamic_banking.zakat_calculations IS 'Zakat (almsgiving) calculations and annual settlements';

-- Sharia Board Decisions and Rulings
CREATE TABLE IF NOT EXISTS islamic_banking.sharia_board_decisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    decision_code VARCHAR(50) NOT NULL UNIQUE,
    decision_date TIMESTAMPTZ NOT NULL,
    board_meeting_date TIMESTAMPTZ NOT NULL,
    decision_topic VARCHAR(255) NOT NULL,
    decision_category VARCHAR(50) NOT NULL CHECK (decision_category IN ('product_approval', 'contract_interpretation', 'compliance_ruling', 'fatwa', 'operational_guidance', 'investment_restriction')),
    decision_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (decision_status IN ('draft', 'approved', 'active', 'superseded', 'archived')),
    decision_summary TEXT NOT NULL,
    detailed_reasoning TEXT,
    applicable_products JSONB, -- Array of affected products/contracts
    applicable_to_entity_ids JSONB, -- Array of accounts/customers affected
    effective_date DATE NOT NULL,
    expiry_date DATE,
    unanimous_decision BOOLEAN NOT NULL DEFAULT TRUE,
    dissenting_members TEXT, -- Names of dissenting board members
    sharia_board_member_ids JSONB, -- Array of board member UUIDs
    board_chairman_name VARCHAR(255),
    implementation_requirements TEXT,
    compliance_certification_required BOOLEAN NOT NULL DEFAULT TRUE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sharia_decisions_date ON islamic_banking.sharia_board_decisions(decision_date);
CREATE INDEX idx_sharia_decisions_category ON islamic_banking.sharia_board_decisions(decision_category);
CREATE INDEX idx_sharia_decisions_status ON islamic_banking.sharia_board_decisions(decision_status);
CREATE INDEX idx_sharia_decisions_effective ON islamic_banking.sharia_board_decisions(effective_date);

COMMENT ON TABLE islamic_banking.sharia_board_decisions IS 'Sharia Board decisions, fatawa (rulings), and compliance guidance';

-- Profit Distributions (Murabaha, Musharaka, Mudaraba, Sukuk)
CREATE TABLE IF NOT EXISTS islamic_banking.profit_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    distribution_code VARCHAR(50) NOT NULL UNIQUE,
    distribution_date TIMESTAMPTZ NOT NULL,
    distribution_period_start DATE NOT NULL,
    distribution_period_end DATE NOT NULL,
    contract_type VARCHAR(50) NOT NULL CHECK (contract_type IN ('murabaha', 'ijara', 'musharaka', 'mudaraba', 'sukuk')),
    contract_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    account_id UUID NOT NULL,
    gross_profit_amount BIGINT NOT NULL, -- Cents
    profit_before_distribution BIGINT NOT NULL,
    bank_share_amount BIGINT,
    customer_share_amount BIGINT,
    distribution_amount BIGINT NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    distribution_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (distribution_status IN ('pending', 'approved', 'processed', 'failed', 'reversed')),
    distribution_method VARCHAR(50) NOT NULL CHECK (distribution_method IN ('credit_to_account', 'cheque', 'transfer', 'reinvestment')),
    credit_account_id UUID,
    processing_date TIMESTAMPTZ,
    distribution_certificate_issued BOOLEAN NOT NULL DEFAULT FALSE,
    certificate_file_id VARCHAR(255),
    withholding_tax_amount BIGINT,
    withholding_tax_rate DECIMAL(10, 4),
    net_distribution_amount BIGINT,
    approver_name VARCHAR(255),
    approval_date TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_profit_distributions_customer ON islamic_banking.profit_distributions(customer_id);
CREATE INDEX idx_profit_distributions_account ON islamic_banking.profit_distributions(account_id);
CREATE INDEX idx_profit_distributions_date ON islamic_banking.profit_distributions(distribution_date);
CREATE INDEX idx_profit_distributions_status ON islamic_banking.profit_distributions(distribution_status);
CREATE INDEX idx_profit_distributions_contract ON islamic_banking.profit_distributions(contract_id);

COMMENT ON TABLE islamic_banking.profit_distributions IS 'Profit distribution records for Islamic financing products';
