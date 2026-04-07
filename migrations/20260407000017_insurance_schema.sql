-- BANKO Insurance BC Schema
-- Insurance policies, claims, bancassurance products, commissions

-- Insurance Policies
CREATE TABLE IF NOT EXISTS insurance.insurance_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_code VARCHAR(50) NOT NULL UNIQUE,
    policy_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (policy_status IN ('quoted', 'pending_approval', 'active', 'suspended', 'expired', 'cancelled', 'lapsed')),
    policyholder_customer_id UUID NOT NULL,
    policyholder_name VARCHAR(255) NOT NULL,
    policyholder_id_number VARCHAR(50),
    policyholder_contact_phone VARCHAR(20),
    policyholder_contact_email VARCHAR(100),
    insured_party_name VARCHAR(255), -- May differ from policyholder
    insured_party_relationship VARCHAR(100), -- e.g., 'spouse', 'child', 'business_partner'
    insurance_company_code VARCHAR(50),
    insurance_company_name VARCHAR(255),
    insurance_agent_name VARCHAR(255),
    insurance_agent_id VARCHAR(50),
    policy_type VARCHAR(50) NOT NULL CHECK (policy_type IN ('life', 'health', 'property', 'casualty', 'marine', 'auto', 'liability', 'credit_life', 'other')),
    policy_sub_type VARCHAR(100), -- e.g., 'whole_life', 'term_life', 'critical_illness'
    coverage_amount BIGINT NOT NULL, -- Cents
    coverage_currency VARCHAR(3) NOT NULL,
    coverage_description TEXT NOT NULL,
    policy_start_date DATE NOT NULL,
    policy_end_date DATE NOT NULL,
    policy_term_months SMALLINT,
    premium_amount BIGINT NOT NULL, -- Cents, per payment period
    premium_currency VARCHAR(3) NOT NULL,
    premium_frequency VARCHAR(50) NOT NULL CHECK (premium_frequency IN ('monthly', 'quarterly', 'semi_annually', 'annually')),
    total_annual_premium BIGINT GENERATED ALWAYS AS (
        CASE
            WHEN premium_frequency = 'monthly' THEN premium_amount * 12
            WHEN premium_frequency = 'quarterly' THEN premium_amount * 4
            WHEN premium_frequency = 'semi_annually' THEN premium_amount * 2
            ELSE premium_amount
        END
    ) STORED,
    next_premium_due_date DATE,
    last_premium_paid_date DATE,
    last_premium_amount BIGINT,
    arrears_amount BIGINT DEFAULT 0,
    payment_method VARCHAR(50) CHECK (payment_method IN ('bank_transfer', 'cheque', 'cash', 'direct_debit', 'auto_renewal')),
    auto_payment_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    auto_renewal_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    deductible_amount BIGINT,
    deductible_percentage DECIMAL(10, 4),
    waiting_period_days SMALLINT,
    exclusions_list TEXT,
    riders JSONB, -- Additional coverage riders
    policy_number VARCHAR(100) NOT NULL,
    certificate_issued_date DATE,
    certificates_count SMALLINT DEFAULT 1,
    claim_history_count SMALLINT DEFAULT 0,
    total_claimed_amount BIGINT DEFAULT 0,
    policy_document_file_id VARCHAR(255),
    policy_amendment_count SMALLINT DEFAULT 0,
    linked_loan_account_id UUID, -- For credit life insurance
    linked_account_balance BIGINT,
    decreasing_term_applicable BOOLEAN NOT NULL DEFAULT FALSE,
    decreasing_term_schedule JSONB,
    medical_underwriting_completed BOOLEAN NOT NULL DEFAULT FALSE,
    medical_underwriting_date DATE,
    underwriting_approver_name VARCHAR(255),
    underwriting_approval_date DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_insurance_policies_holder ON insurance.insurance_policies(policyholder_customer_id);
CREATE INDEX idx_insurance_policies_status ON insurance.insurance_policies(policy_status);
CREATE INDEX idx_insurance_policies_type ON insurance.insurance_policies(policy_type);
CREATE INDEX idx_insurance_policies_end_date ON insurance.insurance_policies(policy_end_date);
CREATE INDEX idx_insurance_policies_company ON insurance.insurance_policies(insurance_company_name);

COMMENT ON TABLE insurance.insurance_policies IS 'Insurance policy master data with coverage, premium, and underwriting details';

-- Insurance Claims
CREATE TABLE IF NOT EXISTS insurance.insurance_claims (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    claim_code VARCHAR(50) NOT NULL UNIQUE,
    policy_id UUID NOT NULL REFERENCES insurance.insurance_policies(id),
    claim_status VARCHAR(50) NOT NULL DEFAULT 'submitted' CHECK (claim_status IN ('submitted', 'under_review', 'approved', 'rejected', 'partial_approval', 'paid', 'appealed', 'closed')),
    claim_type VARCHAR(50) NOT NULL CHECK (claim_type IN ('death_benefit', 'disability', 'hospitalization', 'critical_illness', 'property_damage', 'accident', 'other')),
    claim_date TIMESTAMPTZ NOT NULL, -- Date of incident
    claim_submission_date DATE NOT NULL,
    claim_amount_requested BIGINT NOT NULL, -- Cents
    claim_amount_approved BIGINT,
    claim_amount_paid BIGINT DEFAULT 0,
    claim_currency VARCHAR(3) NOT NULL,
    claim_reason TEXT NOT NULL,
    claim_description TEXT,
    supporting_documents_count SMALLINT DEFAULT 0,
    supporting_documents_complete BOOLEAN NOT NULL DEFAULT FALSE,
    claim_investigator_name VARCHAR(255),
    claim_investigation_report_file_id VARCHAR(255),
    claim_investigation_findings TEXT,
    claim_investigation_completed_date DATE,
    claim_reviewer_name VARCHAR(255),
    claim_review_date TIMESTAMPTZ,
    claim_approval_authority VARCHAR(100),
    claim_approval_date DATE,
    denial_reason TEXT, -- If rejected
    appeal_submitted BOOLEAN NOT NULL DEFAULT FALSE,
    appeal_submission_date DATE,
    appeal_outcome VARCHAR(50) CHECK (appeal_outcome IN ('upheld', 'overturned', 'partial_approval')),
    appeal_decision_date DATE,
    claim_payment_date TIMESTAMPTZ,
    claim_payment_method VARCHAR(50), -- Bank transfer, cheque, etc.
    claim_payment_reference VARCHAR(100),
    deductible_applied BIGINT,
    net_claim_payment BIGINT,
    claim_processing_days SMALLINT GENERATED ALWAYS AS (CAST(EXTRACT(DAY FROM (claim_payment_date - claim_submission_date)) AS SMALLINT)) STORED,
    recovery_subrogation_applicable BOOLEAN NOT NULL DEFAULT FALSE,
    recovery_amount BIGINT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_insurance_claims_policy ON insurance.insurance_claims(policy_id);
CREATE INDEX idx_insurance_claims_status ON insurance.insurance_claims(claim_status);
CREATE INDEX idx_insurance_claims_date ON insurance.insurance_claims(claim_date);
CREATE INDEX idx_insurance_claims_type ON insurance.insurance_claims(claim_type);
CREATE INDEX idx_insurance_claims_payment_date ON insurance.insurance_claims(claim_payment_date);

COMMENT ON TABLE insurance.insurance_claims IS 'Insurance claim submissions, reviews, approvals, and payments';

-- Bancassurance Products
CREATE TABLE IF NOT EXISTS insurance.bancassurance_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_code VARCHAR(50) NOT NULL UNIQUE,
    product_name VARCHAR(255) NOT NULL,
    product_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (product_status IN ('development', 'approved', 'active', 'inactive', 'discontinued')),
    product_type VARCHAR(50) NOT NULL CHECK (product_type IN ('life_insurance', 'health_insurance', 'property_insurance', 'investment_linked', 'credit_life', 'group_insurance')),
    partner_insurance_company_code VARCHAR(50),
    partner_insurance_company_name VARCHAR(255),
    target_customer_segment VARCHAR(100), -- e.g., 'retail', 'corporate', 'sme'
    distribution_channel VARCHAR(50) CHECK (distribution_channel IN ('bank_branches', 'online', 'phone_banking', 'relationship_manager')),
    product_launch_date DATE,
    product_description TEXT NOT NULL,
    coverage_features JSONB, -- Key coverage details
    minimum_sum_assured BIGINT,
    maximum_sum_assured BIGINT,
    minimum_premium BIGINT,
    maximum_annual_premium BIGINT,
    standard_premium_rate DECIMAL(10, 4), -- Base premium percentage of sum assured
    age_minimum SMALLINT,
    age_maximum SMALLINT,
    health_underwriting_required BOOLEAN NOT NULL DEFAULT TRUE,
    waiting_period_days SMALLINT,
    exclusions_list TEXT,
    policy_term_options JSONB, -- Available terms: 10, 15, 20, 25 years
    claim_processing_sla_days SMALLINT DEFAULT 30,
    renewal_options VARCHAR(50) CHECK (renewal_options IN ('renewable', 'non_renewable', 'convertible')),
    integration_with_loan_products BOOLEAN NOT NULL DEFAULT FALSE,
    associated_loan_product_codes JSONB, -- e.g., ['home_loan', 'auto_loan']
    cross_sell_opportunity BOOLEAN NOT NULL DEFAULT FALSE,
    related_products JSONB,
    training_completed BOOLEAN NOT NULL DEFAULT FALSE,
    branch_awareness_program_date DATE,
    product_approval_date DATE,
    approver_name VARCHAR(255),
    total_policies_issued BIGINT DEFAULT 0,
    total_premium_collected BIGINT DEFAULT 0,
    lapse_rate_percentage DECIMAL(10, 4),
    customer_satisfaction_rating DECIMAL(10, 2), -- Out of 5 or 10
    commission_structure JSONB, -- {first_year: 15%, renewal: 5%}
    profitability_status VARCHAR(50) CHECK (profitability_status IN ('profitable', 'break_even', 'loss_making')),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bancassurance_products_type ON insurance.bancassurance_products(product_type);
CREATE INDEX idx_bancassurance_products_status ON insurance.bancassurance_products(product_status);
CREATE INDEX idx_bancassurance_products_company ON insurance.bancassurance_products(partner_insurance_company_name);

COMMENT ON TABLE insurance.bancassurance_products IS 'Bancassurance products and insurance partnerships';

-- Insurance Commissions
CREATE TABLE IF NOT EXISTS insurance.insurance_commissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    commission_code VARCHAR(50) NOT NULL UNIQUE,
    policy_id UUID NOT NULL REFERENCES insurance.insurance_policies(id),
    commission_type VARCHAR(50) NOT NULL CHECK (commission_type IN ('first_year_commission', 'renewal_commission', 'override_commission', 'bonus_commission')),
    commission_date TIMESTAMPTZ NOT NULL,
    commission_payment_date TIMESTAMPTZ,
    commission_period_start DATE,
    commission_period_end DATE,
    commission_amount BIGINT NOT NULL, -- Cents
    commission_currency VARCHAR(3) NOT NULL,
    commission_percentage DECIMAL(10, 4),
    commission_basis BIGINT, -- Premium amount commission is based on
    commissioned_to_agent_name VARCHAR(255),
    commissioned_to_agent_code VARCHAR(50),
    commissioned_to_branch_name VARCHAR(255),
    commissioned_to_branch_code VARCHAR(50),
    agent_tier_level VARCHAR(50) CHECK (agent_tier_level IN ('tier_1', 'tier_2', 'tier_3', 'tier_4')),
    incentive_bonus_applicable BOOLEAN NOT NULL DEFAULT FALSE,
    incentive_bonus_amount BIGINT,
    total_commission_payable BIGINT GENERATED ALWAYS AS (commission_amount + COALESCE(incentive_bonus_amount, 0)) STORED,
    commission_status VARCHAR(50) NOT NULL DEFAULT 'calculated' CHECK (commission_status IN ('calculated', 'approved', 'paid', 'reversed', 'disputed')),
    approval_authority VARCHAR(100),
    approval_date TIMESTAMPTZ,
    payment_method VARCHAR(50) CHECK (payment_method IN ('bank_transfer', 'cheque', 'deposit_to_account')),
    payment_reference VARCHAR(100),
    withholding_tax_applicable BOOLEAN NOT NULL DEFAULT FALSE,
    withholding_tax_percentage DECIMAL(10, 4),
    withholding_tax_amount BIGINT,
    net_commission_payable BIGINT,
    commission_recovery_reasons TEXT, -- If commission is reversed
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_insurance_commissions_policy ON insurance.insurance_commissions(policy_id);
CREATE INDEX idx_insurance_commissions_date ON insurance.insurance_commissions(commission_date);
CREATE INDEX idx_insurance_commissions_status ON insurance.insurance_commissions(commission_status);
CREATE INDEX idx_insurance_commissions_agent ON insurance.insurance_commissions(commissioned_to_agent_name);
CREATE 