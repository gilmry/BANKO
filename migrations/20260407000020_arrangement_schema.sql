-- BANKO Arrangement BC Schema
-- Arrangement lifecycle management, events, bundles, and term management

CREATE SCHEMA IF NOT EXISTS arrangement;

-- Arrangements (Core entities)
CREATE TABLE IF NOT EXISTS arrangement.arrangements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    arrangement_code VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL,
    arrangement_type VARCHAR(100) NOT NULL CHECK (arrangement_type IN ('loan', 'credit_line', 'deposit', 'savings', 'investment', 'insurance', 'lease', 'other')),
    product_code VARCHAR(50) NOT NULL,
    product_name VARCHAR(255) NOT NULL,
    principal_amount BIGINT NOT NULL, -- Cents
    currency_code VARCHAR(3) NOT NULL DEFAULT 'TND',
    arrangement_status VARCHAR(50) NOT NULL DEFAULT 'proposed' CHECK (arrangement_status IN ('proposed', 'approved', 'activated', 'suspended', 'matured', 'closed', 'cancelled')),
    activation_date DATE,
    maturity_date DATE NOT NULL,
    closure_date DATE,
    interest_rate_type VARCHAR(50) CHECK (interest_rate_type IN ('fixed', 'variable', 'floating', 'mixed')),
    interest_rate DECIMAL(10, 8),
    annual_percentage_rate DECIMAL(10, 8),
    base_rate VARCHAR(50), -- EURIBOR, LIBOR, etc.
    margin DECIMAL(10, 8),
    effective_interest_rate DECIMAL(10, 8), -- Calculated
    term_length_days INTEGER,
    term_length_months INTEGER,
    term_unit VARCHAR(20) CHECK (term_unit IN ('days', 'months', 'years')),
    installment_amount BIGINT, -- Cents
    installment_frequency VARCHAR(50) CHECK (installment_frequency IN ('weekly', 'biweekly', 'monthly', 'quarterly', 'semi_annual', 'annual')),
    total_installments INTEGER,
    remaining_installments INTEGER,
    payment_status VARCHAR(50) CHECK (payment_status IN ('on_track', 'past_due', 'in_default', 'restructured')),
    days_past_due INTEGER,
    collateral_requirement BOOLEAN DEFAULT FALSE,
    collateral_id UUID,
    collateral_description TEXT,
    guarantor_required BOOLEAN DEFAULT FALSE,
    guarantor_name VARCHAR(255),
    guarantor_relationship VARCHAR(50),
    purpose TEXT,
    purpose_code VARCHAR(50),
    regulatory_classification VARCHAR(100),
    is_syndicated BOOLEAN DEFAULT FALSE,
    lead_arranger VARCHAR(255),
    arrangement_owner VARCHAR(255),
    document_reference VARCHAR(100),
    legal_document_link VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255)
);

CREATE INDEX idx_arrangements_customer ON arrangement.arrangements(customer_id);
CREATE INDEX idx_arrangements_status ON arrangement.arrangements(arrangement_status);
CREATE INDEX idx_arrangements_type ON arrangement.arrangements(arrangement_type);
CREATE INDEX idx_arrangements_maturity ON arrangement.arrangements(maturity_date);
CREATE INDEX idx_arrangements_code ON arrangement.arrangements(arrangement_code);

COMMENT ON TABLE arrangement.arrangements IS 'Core arrangement entities with terms, status, and lifecycle';

-- Arrangement Events (State transitions and significant events)
CREATE TABLE IF NOT EXISTS arrangement.arrangement_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_code VARCHAR(50) NOT NULL UNIQUE,
    arrangement_id UUID NOT NULL REFERENCES arrangement.arrangements(id),
    event_type VARCHAR(100) NOT NULL CHECK (event_type IN ('created', 'approved', 'activated', 'modified', 'suspended', 'resumed', 'renewed', 'restructured', 'matured', 'closed', 'cancelled', 'payment_made', 'payment_missed', 'interest_accrued', 'rate_change', 'collateral_change', 'guarantor_change', 'alert_generated', 'audit_event')),
    event_date TIMESTAMPTZ NOT NULL,
    event_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_status VARCHAR(50) NOT NULL DEFAULT 'completed' CHECK (event_status IN ('scheduled', 'in_progress', 'completed', 'failed', 'cancelled')),
    event_description TEXT,
    event_details JSONB, -- Flexible data structure for event-specific details
    previous_state VARCHAR(255), -- Serialized previous state snapshot
    new_state VARCHAR(255), -- Serialized new state snapshot
    state_change JSONB, -- What changed: {field: {old: value, new: value}}
    triggered_by VARCHAR(50) NOT NULL CHECK (triggered_by IN ('system', 'user', 'scheduled', 'external', 'auto_calculation')),
    triggered_by_user VARCHAR(255),
    triggered_by_system VARCHAR(100),
    is_reversible BOOLEAN DEFAULT FALSE,
    reverse_event_id UUID REFERENCES arrangement.arrangement_events(id),
    parent_event_id UUID REFERENCES arrangement.arrangement_events(id),
    notification_sent BOOLEAN DEFAULT FALSE,
    notification_recipients JSONB, -- Array of emails/users
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_arrangement_events_arrangement ON arrangement.arrangement_events(arrangement_id);
CREATE INDEX idx_arrangement_events_type ON arrangement.arrangement_events(event_type);
CREATE INDEX idx_arrangement_events_date ON arrangement.arrangement_events(event_date);
CREATE INDEX idx_arrangement_events_status ON arrangement.arrangement_events(event_status);

COMMENT ON TABLE arrangement.arrangement_events IS 'Event log for arrangement state transitions and significant business events';

-- Arrangement Bundles (Grouping of related arrangements)
CREATE TABLE IF NOT EXISTS arrangement.arrangement_bundles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bundle_code VARCHAR(50) NOT NULL UNIQUE,
    bundle_name VARCHAR(255) NOT NULL,
    customer_id UUID NOT NULL,
    bundle_type VARCHAR(100) NOT NULL CHECK (bundle_type IN ('relationship', 'product_suite', 'facility_group', 'syndicated_deal', 'project_finance', 'other')),
    bundle_description TEXT,
    bundle_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (bundle_status IN ('active', 'inactive', 'archived', 'closed')),
    total_principal_amount BIGINT NOT NULL, -- Sum of all arrangements
    currency_code VARCHAR(3) NOT NULL,
    bundle_manager VARCHAR(255),
    relationship_manager VARCHAR(255),
    create_date DATE NOT NULL,
    close_date DATE,
    bundle_reference VARCHAR(100),
    master_agreement_reference VARCHAR(100),
    cross_default_clause BOOLEAN DEFAULT FALSE,
    cross_collateralization BOOLEAN DEFAULT FALSE,
    netting_agreement BOOLEAN DEFAULT FALSE,
    complexity_level VARCHAR(50) CHECK (complexity_level IN ('simple', 'moderate', 'complex', 'highly_complex')),
    regulatory_requirements JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255)
);

CREATE INDEX idx_arrangement_bundles_customer ON arrangement.arrangement_bundles(customer_id);
CREATE INDEX idx_arrangement_bundles_status ON arrangement.arrangement_bundles(bundle_status);
CREATE INDEX idx_arrangement_bundles_type ON arrangement.arrangement_bundles(bundle_type);

COMMENT ON TABLE arrangement.arrangement_bundles IS 'Bundles grouping related arrangements (relationships, facility groups, deals)';

-- Arrangement Bundle Members (Mapping between arrangements and bundles)
CREATE TABLE IF NOT EXISTS arrangement.arrangement_bundle_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bundle_id UUID NOT NULL REFERENCES arrangement.arrangement_bundles(id),
    arrangement_id UUID NOT NULL REFERENCES arrangement.arrangements(id),
    member_sequence INTEGER NOT NULL, -- Order within bundle
    member_role VARCHAR(50) CHECK (member_role IN ('primary', 'secondary', 'supporting', 'cross_default_trigger', 'netting_party')),
    allocation_percentage DECIMAL(10, 4), -- % of bundle resources allocated
    priority_level INTEGER DEFAULT 1, -- 1 = highest
    inclusion_reason TEXT,
    effective_from_date DATE NOT NULL DEFAULT CURRENT_DATE,
    effective_to_date DATE,
    membership_status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (membership_status IN ('active', 'suspended', 'removed', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bundle_members_bundle ON arrangement.arrangement_bundle_members(bundle_id);
CREATE INDEX idx_bundle_members_arrangement ON arrangement.arrangement_bundle_members(arrangement_id);
CREATE UNIQUE INDEX idx_bundle_members_unique ON arrangement.arrangement_bundle_members(bundle_id, arrangement_id);

COMMENT ON TABLE arrangement.arrangement_bundle_members IS 'Membership and allocation within arrangement bundles';

-- Arrangement Renewals and Maturity Management
CREATE TABLE IF NOT EXISTS arrangement.arrangement_renewals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    renewal_code VARCHAR(50) NOT NULL UNIQUE,
    arrangement_id UUID NOT NULL REFERENCES arrangement.arrangements(id),
    original_arrangement_id UUID REFERENCES arrangement.arrangements(id), -- If this is a renewal
    renewal_date DATE NOT NULL,
    renewal_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (renewal_status IN ('pending', 'requested', 'approved', 'declined', 'expired', 'completed')),
    renewal_type VARCHAR(50) NOT NULL CHECK (renewal_type IN ('automatic', 'optional', 'mandatory')),
    new_principal_amount BIGINT, -- If different from current
    new_maturity_date DATE,
    new_interest_rate DECIMAL(10, 8), -- If terms change
    new_terms_description TEXT,
    request_date TIMESTAMPTZ,
    approval_date TIMESTAMPTZ,
    approved_by VARCHAR(255),
    decline_reason TEXT,
    customer_notification_sent BOOLEAN DEFAULT FALSE,
    customer_notification_date TIMESTAMPTZ,
    acceptance_deadline DATE,
    customer_decision VARCHAR(50) CHECK (customer_decision IN ('accepted', 'declined', 'pending', 'not_yet_notified')),
    customer_response_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_renewals_arrangement ON arrangement.arrangement_renewals(arrangement_id);
CREATE INDEX idx_renewals_status ON arrangement.arrangement_renewals(renewal_status);
CREATE INDEX idx_renewals_date ON arrangement.arrangement_renewals(renewal_date);

COMMENT ON TABLE arrangement.arrangement_renewals IS 'Arrangement renewal and maturity management';

-- Arrangement Term Modifications
CREATE TABLE IF NOT EXISTS arrangement.arrangement_modifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    modification_code VARCHAR(50) NOT NULL UNIQUE,
    arrangement_id UUID NOT NULL REFERENCES arrangement.arrangements(id),
    modification_type VARCHAR(100) NOT NULL CHECK (modification_type IN ('rate_change', 'term_extension', 'amount_increase', 'amount_decrease', 'payment_restructure', 'collateral_change', 'guarantor_change', 'other')),
    modification_date TIMESTAMPTZ NOT NULL,
    effective_date DATE NOT NULL,
    modification_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (modification_status IN ('pending', 'approved', 'declined', 'implemented', 'reversed', 'expired')),
    request_date TIMESTAMPTZ,
    request_reason TEXT,
    requested_by VARCHAR(255),
    approval_date TIMESTAMPTZ,
    approved_by VARCHAR(255),
    approval_notes TEXT,
    previous_terms JSONB, -- Snapshot of terms before modification
    new_terms JSONB, -- Snapshot of terms after modification
    impact_analysis TEXT, -- Impact on customer, bank, risk profile
    customer_impact_amount BIGINT, -- Positive = benefit to customer, negative = cost
    bank_impact_amount BIGINT,
    risk_impact_score DECIMAL(10, 8),
    regulatory_approval_required BOOLEAN DEFAULT FALSE,
    regulatory_approval_status VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_modifications_arrangement ON arrangement.arrangement_modifications(arrangement_id);
CREATE INDEX idx_modifications_type ON arrangement.arrangement_modifications(modification_type);
CREATE INDEX idx_modifications_status ON arrangement.arrangement_modifications(modification_status);
CREATE INDEX idx_modifications_date ON arrangement.arrangement_modifications(effective_date);

COMMENT ON TABLE arrangement.arrangement_modifications IS 'Term changes, restructures, and modifications to arrangements';

-- CHECK: Maturity date must be after activation date
ALTER TABLE arrangement.arrangements
ADD CONSTRAINT chk_maturity_after_activation CHECK (
    maturity_date IS NULL OR (activation_date IS NULL OR maturity_date > activation_date)
);

-- CHECK: Closure date must be after activation date
ALTER TABLE arrangement.arrangements
ADD CONSTRAINT chk_closure_after_activation CHECK (
    closure_date IS NULL OR (activation_date IS NULL OR closure_date >= activation_date)
);
