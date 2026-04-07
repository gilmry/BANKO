-- BANKO Trade Finance BC Schema
-- Letters of Credit, Bank Guarantees, Documentary Collections, Trade Limits

-- Letters of Credit (LC)
CREATE TABLE IF NOT EXISTS trade_finance.letters_of_credit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lc_code VARCHAR(50) NOT NULL UNIQUE,
    lc_status VARCHAR(50) NOT NULL DEFAULT 'issued' CHECK (lc_status IN ('requested', 'issued', 'accepted', 'amended', 'presentation_pending', 'claimed', 'expired', 'cancelled', 'paid')),
    lc_type VARCHAR(50) NOT NULL CHECK (lc_type IN ('sight', 'usance', 'revolving', 'back_to_back', 'red_clause')),
    lc_form VARCHAR(50) NOT NULL CHECK (lc_form IN ('documentary', 'standby')),
    applicant_customer_id UUID NOT NULL, -- Importer/Buyer
    applicant_name VARCHAR(255) NOT NULL,
    applicant_address TEXT,
    beneficiary_name VARCHAR(255) NOT NULL, -- Exporter/Seller
    beneficiary_address TEXT,
    beneficiary_bank_code VARCHAR(50),
    beneficiary_bank_name VARCHAR(255),
    issuing_bank_code VARCHAR(50),
    issuing_bank_name VARCHAR(255),
    advising_bank_code VARCHAR(50),
    advising_bank_name VARCHAR(255),
    confirming_bank_code VARCHAR(50),
    negotiating_bank_code VARCHAR(50),
    lc_currency_code VARCHAR(3) NOT NULL,
    lc_amount BIGINT NOT NULL, -- Cents
    lc_amount_tolerance_percentage DECIMAL(10, 4), -- Typically 5%
    lc_issue_date DATE NOT NULL,
    lc_expiry_date DATE NOT NULL,
    lc_presentation_deadline DATE NOT NULL,
    documentary_requirements TEXT NOT NULL, -- Invoice, packing list, B/L, etc.
    goods_description TEXT NOT NULL,
    goods_shipping_terms VARCHAR(50), -- Incoterms: CIF, FOB, etc.
    port_of_loading VARCHAR(100),
    port_of_discharge VARCHAR(100),
    vessel_details TEXT,
    insurance_required BOOLEAN NOT NULL DEFAULT TRUE,
    insurance_coverage_amount BIGINT,
    partial_shipments_allowed BOOLEAN,
    transhipment_allowed BOOLEAN,
    inspection_required BOOLEAN,
    lc_issue_commission BIGINT, -- Cents
    lc_amendment_commission BIGINT,
    negotiation_commission DECIMAL(10, 4), -- Percentage
    confirmation_commission DECIMAL(10, 4),
    advising_commission BIGINT,
    total_commissions_collected BIGINT DEFAULT 0,
    amendment_count SMALLINT DEFAULT 0,
    amendment_details JSONB,
    presentation_count SMALLINT DEFAULT 0,
    discrepancy_count SMALLINT DEFAULT 0,
    lc_availability VARCHAR(50) CHECK (lc_availability IN ('by_sight_payment', 'by_usance_payment', 'by_negotiation', 'by_mixed_payment')),
    usance_days SMALLINT, -- For usance LCs
    usance_acceptance_expiry DATE,
    credit_risk_assessment VARCHAR(50),
    country_risk_rating VARCHAR(50),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_lc_applicant ON trade_finance.letters_of_credit(applicant_customer_id);
CREATE INDEX idx_lc_status ON trade_finance.letters_of_credit(lc_status);
CREATE INDEX idx_lc_expiry ON trade_finance.letters_of_credit(lc_expiry_date);
CREATE INDEX idx_lc_type ON trade_finance.letters_of_credit(lc_type);
CREATE INDEX idx_lc_beneficiary ON trade_finance.letters_of_credit(beneficiary_name);

COMMENT ON TABLE trade_finance.letters_of_credit IS 'Letter of Credit transactions with terms, amendments, and presentations';

-- Bank Guarantees
CREATE TABLE IF NOT EXISTS trade_finance.bank_guarantees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guarantee_code VARCHAR(50) NOT NULL UNIQUE,
    guarantee_status VARCHAR(50) NOT NULL DEFAULT 'issued' CHECK (guarantee_status IN ('requested', 'issued', 'amended', 'claimed', 'expired', 'cancelled', 'fully_claimed')),
    guarantee_type VARCHAR(50) NOT NULL CHECK (guarantee_type IN ('bid_bond', 'performance_bond', 'advance_payment_guarantee', 'retention_money_guarantee', 'payment_guarantee', 'customs_guarantee', 'other')),
    applicant_customer_id UUID NOT NULL, -- Principal
    applicant_name VARCHAR(255) NOT NULL,
    beneficiary_name VARCHAR(255) NOT NULL, -- Obligee
    beneficiary_address TEXT,
    beneficiary_contact VARCHAR(50),
    issuing_bank_code VARCHAR(50),
    issuing_bank_name VARCHAR(255),
    instructing_bank_code VARCHAR(50),
    confirming_bank_code VARCHAR(50),
    guarantee_currency_code VARCHAR(3) NOT NULL,
    guarantee_amount BIGINT NOT NULL, -- Cents
    guarantee_issue_date DATE NOT NULL,
    guarantee_expiry_date DATE NOT NULL,
    beneficiary_claim_deadline DATE,
    underlying_contract_reference VARCHAR(100),
    underlying_contract_value BIGINT,
    underlying_contract_currency VARCHAR(3),
    underlying_contract_terms TEXT,
    guarantee_percentage_of_contract DECIMAL(10, 4), -- e.g., 10% for bid bond
    guarantee_conditions TEXT, -- On-demand, conditional, etc.
    claim_notification_required BOOLEAN NOT NULL DEFAULT TRUE,
    claim_documentation_requirements TEXT,
    claim_presentation_period_days SMALLINT,
    claim_count SMALLINT DEFAULT 0,
    claim_amount_total BIGINT DEFAULT 0,
    claim_amount_paid BIGINT DEFAULT 0,
    guarantee_issue_commission BIGINT, -- Cents
    guarantee_amendment_commission BIGINT,
    confirmation_commission DECIMAL(10, 4),
    amendment_count SMALLINT DEFAULT 0,
    country_risk_rating VARCHAR(50),
    principal_credit_risk VARCHAR(50),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_guarantee_applicant ON trade_finance.bank_guarantees(applicant_customer_id);
CREATE INDEX idx_guarantee_status ON trade_finance.bank_guarantees(guarantee_status);
CREATE INDEX idx_guarantee_type ON trade_finance.bank_guarantees(guarantee_type);
CREATE INDEX idx_guarantee_expiry ON trade_finance.bank_guarantees(guarantee_expiry_date);
CREATE INDEX idx_guarantee_beneficiary ON trade_finance.bank_guarantees(beneficiary_name);

COMMENT ON TABLE trade_finance.bank_guarantees IS 'Bank guarantee facilities including bid bonds, performance bonds, and payment guarantees';

-- Documentary Collections
CREATE TABLE IF NOT EXISTS trade_finance.documentary_collections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_code VARCHAR(50) NOT NULL UNIQUE,
    collection_status VARCHAR(50) NOT NULL DEFAULT 'in_process' CHECK (collection_status IN ('created', 'in_process', 'delivery_pending', 'delivered', 'returned', 'cancelled')),
    collection_type VARCHAR(50) NOT NULL CHECK (collection_type IN ('documents_against_payment', 'documents_against_acceptance')),
    remitting_customer_id UUID NOT NULL, -- Exporter/Drawer
    remitting_customer_name VARCHAR(255) NOT NULL,
    remitting_bank_code VARCHAR(50),
    remitting_bank_name VARCHAR(255),
    collecting_bank_code VARCHAR(50),
    collecting_bank_name VARCHAR(255),
    drawee_name VARCHAR(255) NOT NULL, -- Importer/Payer
    drawee_address TEXT,
    drawee_bank_code VARCHAR(50),
    drawee_bank_name VARCHAR(255),
    collection_currency_code VARCHAR(3) NOT NULL,
    collection_amount BIGINT NOT NULL, -- Cents
    commercial_invoice_reference VARCHAR(100),
    invoice_amount BIGINT,
    invoice_date DATE,
    collection_date TIMESTAMPTZ NOT NULL,
    bill_of_lading_number VARCHAR(100),
    bl_date DATE,
    bl_amount BIGINT,
    shipping_date DATE,
    port_of_shipment VARCHAR(100),
    port_of_discharge VARCHAR(100),
    goods_description TEXT,
    documents_required JSONB, -- Array of required documents
    documents_received BOOLEAN NOT NULL DEFAULT FALSE,
    documents_received_date TIMESTAMPTZ,
    documents_sent_date TIMESTAMPTZ,
    payment_due_date DATE,
    acceptance_due_date DATE,
    payment_terms TEXT, -- At sight, 30 days, etc.
    interest_rate_for_extension DECIMAL(10, 4),
    charges_to_drawee BOOLEAN NOT NULL DEFAULT FALSE,
    collection_charges_amount BIGINT,
    collection_commission BIGINT, -- Cents
    collection_commission_rate DECIMAL(10, 4),
    courier_fee BIGINT,
    bank_charges BIGINT,
    payment_received_date TIMESTAMPTZ,
    payment_received_amount BIGINT,
    payment_method VARCHAR(50), -- Cheque, wire transfer, etc.
    collection_status_update_date TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_collections_remitting ON trade_finance.documentary_collections(remitting_customer_id);
CREATE INDEX idx_collections_status ON trade_finance.documentary_collections(collection_status);
CREATE INDEX idx_collections_date ON trade_finance.documentary_collections(collection_date);
CREATE INDEX idx_collections_type ON trade_finance.documentary_collections(collection_type);

COMMENT ON TABLE trade_finance.documentary_collections IS 'Documentary collection transactions (D/P and D/A)';

-- Trade Finance Limits
CREATE TABLE IF NOT EXISTS trade_finance.trade_finance_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    limit_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    limit_type VARCHAR(50) NOT NULL CHECK (limit_type IN ('lc_limit', 'guarantee_limit', 'collection_limit', 'combined_limit')),
    limit_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (limit_status IN ('draft', 'pending_approval', 'active', 'suspended', 'cancelled')),
    limit_currency_code VARCHAR(3) NOT NULL,
    limit_amount BIGINT NOT NULL, -- Cents
    limit_utilization_amount BIGINT NOT NULL DEFAULT 0,
    limit_available_amount BIGINT GENERATED ALWAYS AS (limit_amount - limit_utilization_amount) STORED,
    utilization_percentage DECIMAL(10, 4) GENERATED ALWAYS AS (
        CASE
            WHEN limit_amount > 0 THEN (limit_utilization_amount::DECIMAL / limit_amount * 100)
            ELSE 0
        END
    ) STORED,
    limit_effective_date DATE NOT NULL,
    limit_expiry_date DATE NOT NULL,
    limit_approval_authority VARCHAR(100),
    limit_approval_date DATE,
    limit_review_frequency VARCHAR(50) CHECK (limit_review_frequency IN ('annual', 'bi_annual', 'quarterly', 'on_demand')),
    next_limit_review_date DATE,
    tenor_limit_for_lc VARCHAR(50) CHECK (tenor_limit_for_lc IN ('sight', 'usance_up_to_30', 'usance_up_to_60', 'usance_up_to_90', 'usance_up_to_180', 'no_limit')),
    sub_limits JSONB, -- {lc: 500000, guarantee: 300000, collection: 200000}
    country_concentration_limit DECIMAL(10, 4), -- Max percentage for single country
    maximum_single_transaction_limit BIGINT,
    pricing_margin_bps SMALLINT, -- Basis points
    annual_review_notes TEXT,
    collateral_requirement BOOLEAN NOT NULL DEFAULT FALSE,
    collateral_type VARCHAR(50),
    collateral_coverage_percentage DECIMAL(10, 4),
    credit_rating_requirement VARCHAR(50),
    financial_covenants TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tf_limits_customer ON trade_finance.trade_finance_limits(customer_id);
CREATE INDEX idx_tf_limits_status ON trade_finance.trade_finance_limits(limit_status);
CREATE INDEX idx_tf_limits_type ON trade_finance.trade_finance_limits(limit_type);
C