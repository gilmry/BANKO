-- BANKO Credit BC Enhancement
-- Revolving credit lines, sub-limits, syndicated loans, loan restructuring, moratory interest, early repayment penalties

-- Revolving Credit Lines
CREATE TABLE IF NOT EXISTS credit.revolving_credit_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    credit_line_code VARCHAR(50) NOT NULL UNIQUE,
    facility_type VARCHAR(50) NOT NULL CHECK (facility_type IN ('overdraft', 'revolving_loan', 'credit_card', 'line_of_credit', 'term_loan_with_drawdowns')),
    principal_amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    drawn_amount BIGINT NOT NULL DEFAULT 0,
    available_amount BIGINT NOT NULL GENERATED ALWAYS AS (principal_amount - drawn_amount) STORED,
    interest_rate DECIMAL(10, 6) NOT NULL,
    interest_accrual_method VARCHAR(50) NOT NULL DEFAULT 'daily' CHECK (interest_accrual_method IN ('daily', 'monthly', 'quarterly', 'annual')),
    commitment_fee_rate DECIMAL(10, 6), -- Fee on undrawn amount
    credit_line_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (credit_line_status IN ('approved', 'active', 'suspended', 'closed', 'defaulted')),
    maturity_date TIMESTAMPTZ NOT NULL,
    renewal_date TIMESTAMPTZ,
    auto_renewal BOOLEAN NOT NULL DEFAULT FALSE,
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_revolving_credit_customer ON credit.revolving_credit_lines(customer_id);
CREATE INDEX idx_revolving_credit_status ON credit.revolving_credit_lines(credit_line_status);
CREATE INDEX idx_revolving_credit_maturity ON credit.revolving_credit_lines(maturity_date);

COMMENT ON TABLE credit.revolving_credit_lines IS 'Revolving credit facilities with drawn and available amounts';

-- Credit Line Sub-Limits (tranches for different purposes)
CREATE TABLE IF NOT EXISTS credit.credit_sub_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    revolving_credit_line_id UUID NOT NULL REFERENCES credit.revolving_credit_lines(id) ON DELETE CASCADE,
    sub_limit_code VARCHAR(50) NOT NULL,
    sub_limit_type VARCHAR(50) NOT NULL CHECK (sub_limit_type IN ('domestic_transfer', 'international_transfer', 'supplier_credit', 'working_capital', 'investment', 'standby', 'other')),
    sub_limit_amount BIGINT NOT NULL,
    drawn_amount BIGINT NOT NULL DEFAULT 0,
    available_amount BIGINT NOT NULL GENERATED ALWAYS AS (sub_limit_amount - drawn_amount) STORED,
    interest_rate_override DECIMAL(10, 6), -- If different from main facility
    purpose TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(revolving_credit_line_id, sub_limit_code)
);

CREATE INDEX idx_credit_sub_limits_line ON credit.credit_sub_limits(revolving_credit_line_id);
CREATE INDEX idx_credit_sub_limits_type ON credit.credit_sub_limits(sub_limit_type);

COMMENT ON TABLE credit.credit_sub_limits IS 'Sub-limits within revolving credit lines for different purposes';

-- Syndicated Loans
CREATE TABLE IF NOT EXISTS credit.syndicated_loans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_code VARCHAR(50) NOT NULL UNIQUE,
    loan_type VARCHAR(50) NOT NULL CHECK (loan_type IN ('consortium', 'participant', 'shared_facility', 'bilateral')),
    principal_amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    lead_arranger VARCHAR(255),
    loan_syndicate_members VARCHAR(255)[], -- BIC codes of participating banks
    our_participation_percentage DECIMAL(5, 2) NOT NULL,
    our_syndicated_amount BIGINT NOT NULL GENERATED ALWAYS AS (CAST(principal_amount * (our_participation_percentage / 100.0) AS BIGINT)) STORED,
    customer_id UUID NOT NULL,
    facility_agent VARCHAR(255), -- Bank acting as agent
    deal_margin DECIMAL(10, 6),
    reference_rate VARCHAR(50) CHECK (reference_rate IN ('prime', 'euribor', 'sofr', 'sonia', 'tomr')),
    pricing_terms TEXT,
    loan_status VARCHAR(50) NOT NULL DEFAULT 'proposed' CHECK (loan_status IN ('proposed', 'approved', 'executed', 'drawn', 'repaying', 'paid_off', 'defaulted')),
    sign_date TIMESTAMPTZ,
    first_drawdown_date TIMESTAMPTZ,
    maturity_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_syndicated_loans_customer ON credit.syndicated_loans(customer_id);
CREATE INDEX idx_syndicated_loans_status ON credit.syndicated_loans(loan_status);
CREATE INDEX idx_syndicated_loans_maturity ON credit.syndicated_loans(maturity_date);

COMMENT ON TABLE credit.syndicated_loans IS 'Syndicated loan participation with lead arranger and member details';

-- Loan Restructuring
CREATE TABLE IF NOT EXISTS credit.loan_restructurings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_loan_id UUID,
    restructuring_code VARCHAR(50) NOT NULL UNIQUE,
    restructuring_type VARCHAR(50) NOT NULL CHECK (restructuring_type IN ('extension', 'refinance', 'consolidation', 'waiver', 'haircut', 'debt_swap', 'forbearance')),
    restructuring_reason VARCHAR(100) NOT NULL CHECK (restructuring_reason IN ('financial_hardship', 'market_conditions', 'regulatory_requirement', 'request', 'default_prevention')),
    old_principal_amount BIGINT NOT NULL,
    new_principal_amount BIGINT NOT NULL,
    old_maturity_date TIMESTAMPTZ NOT NULL,
    new_maturity_date TIMESTAMPTZ NOT NULL,
    old_interest_rate DECIMAL(10, 6) NOT NULL,
    new_interest_rate DECIMAL(10, 6) NOT NULL,
    old_fees BIGINT,
    waived_fees BIGINT DEFAULT 0,
    restructuring_date TIMESTAMPTZ NOT NULL,
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    loss_provisions_updated BOOLEAN NOT NULL DEFAULT FALSE,
    restructuring_status VARCHAR(50) NOT NULL DEFAULT 'approved' CHECK (restructuring_status IN ('proposed', 'approved', 'executed', 'completed', 'failed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_loan_restructurings_type ON credit.loan_restructurings(restructuring_type);
CREATE INDEX idx_loan_restructurings_date ON credit.loan_restructurings(restructuring_date);
CREATE INDEX idx_loan_restructurings_status ON credit.loan_restructurings(restructuring_status);

COMMENT ON TABLE credit.loan_restructurings IS 'Loan restructuring and forbearance actions';

-- Moratory Interest (penalty interest on arrears)
CREATE TABLE IF NOT EXISTS credit.moratory_interest (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL,
    moratory_interest_type VARCHAR(50) NOT NULL CHECK (moratory_interest_type IN ('arrears_penalty', 'default_premium', 'late_payment_fee', 'acceleration_interest')),
    base_rate DECIMAL(10, 6) NOT NULL,
    premium_percentage DECIMAL(10, 6) NOT NULL, -- Additional percentage above base rate
    effective_moratory_rate DECIMAL(10, 6) NOT NULL GENERATED ALWAYS AS (base_rate + premium_percentage) STORED,
    accrual_start_date TIMESTAMPTZ NOT NULL,
    accrual_end_date TIMESTAMPTZ,
    accrued_interest BIGINT NOT NULL DEFAULT 0,
    paid_interest BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_moratory_interest_loan ON credit.moratory_interest(loan_id);
CREATE INDEX idx_moratory_interest_type ON credit.moratory_interest(moratory_interest_type);

COMMENT ON TABLE credit.moratory_interest IS 'Penalty interest on arrears and defaulted loans';

-- Early Repayment Penalties
CREATE TABLE IF NOT EXISTS credit.early_repayment_penalties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL,
    penalty_type VARCHAR(50) NOT NULL CHECK (penalty_type IN ('percentage_of_principal', 'interest_spread_shortfall', 'fixed_amount', 'graduated_scale')),
    penalty_formula TEXT NOT NULL, -- Description or formula
    minimum_penalty_amount BIGINT,
    maximum_penalty_amount BIGINT,
    is_waivable BOOLEAN NOT NULL DEFAULT FALSE,
    waiver_approval_required BOOLEAN NOT NULL DEFAULT FALSE,
    applicable_until_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_early_repayment_penalties_loan ON credit.early_repayment_penalties(loan_id);

COMMENT ON TABLE credit.early_repayment_penalties IS 'Early repayment penalty structures and calculations';

-- Early Repayment Transactions
CREATE TABLE IF NOT EXISTS credit.early_repayments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL,
    early_repayment_code VARCHAR(50) NOT NULL UNIQUE,
    repayment_date TIMESTAMPTZ NOT NULL,
    principal_repaid BIGINT NOT NULL,
    interest_paid BIGINT NOT NULL DEFAULT 0,
    moratory_interest_paid BIGINT NOT NULL DEFAULT 0,
    penalty_amount BIGINT NOT NULL DEFAULT 0,
    penalty_waived_amount BIGINT NOT NULL DEFAULT 0,
    waiver_approved_by VARCHAR(255),
    waiver_approved_at TIMESTAMPTZ,
    total_repayment BIGINT NOT NULL GENERATED ALWAYS AS (principal_repaid + interest_paid + moratory_interest_paid + (penalty_amount - penalty_waived_amount)) STORED,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_early_repayments_loan ON credit.early_repayments(loan_id);
CREATE INDEX idx_early_repayments_date ON credit.early_repayments(repayment_date);

COMMENT ON TABLE credit.early_repayments IS 'Early repayment transactions with penalties and waivers';

-- Credit Enhancement (Collateral, Guarantees, etc.)
CREATE TABLE IF NOT EXISTS credit.credit_enhancements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL,
    enhancement_type VARCHAR(50) NOT NULL CHECK (enhancement_type IN ('cash_collateral', 'property_collateral', 'personal_guarantee', 'bank_guarantee', 'insurance', 'pledge', 'charge', 'mortgage')),
    enhancement_description TEXT NOT NULL,
    collateral_value BIGINT,
    currency VARCHAR(3),
    valuation_date TIMESTAMPTZ,
    next_valuation_date TIMESTAMPTZ,
    haircut_percentage DECIMAL(5, 2) DEFAULT 0, -- Risk discount
    secured_amount BIGINT,
    priority_rank SMALLINT, -- For multiple collaterals
    enforcement_authority VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_credit_enhancements_loan ON credit.credit_enhancements(loan_id);
CREATE INDEX idx_credit_enhancements_type ON credit.credit_enhancements(enhancement_type);

COMMENT ON TABLE credit.credit_enhancements IS 'Credit enhancements (collateral, guarantees, insurance)';
