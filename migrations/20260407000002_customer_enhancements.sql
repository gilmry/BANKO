-- BANKO Customer BC Enhancement (FR-006, FR-007, FR-008)
-- Customer segmentation, document lifecycle, and group management

-- Add segmentation to customers
ALTER TABLE customer.customers ADD COLUMN IF NOT EXISTS segment VARCHAR(20) DEFAULT 'retail' CHECK (segment IN ('retail', 'corporate', 'smb', 'premium', 'inactive'));

-- Customer Documents table (FR-007)
CREATE TABLE IF NOT EXISTS customer.customer_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customer.customers(id) ON DELETE CASCADE,
    document_type VARCHAR(30) NOT NULL CHECK (document_type IN ('passport', 'id_card', 'driving_license', 'business_registration', 'tax_cert', 'other')),
    document_number VARCHAR(100) NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ,
    issuing_authority VARCHAR(200),
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verified_by VARCHAR(255),
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customer_documents_customer ON customer.customer_documents(customer_id);
CREATE INDEX idx_customer_documents_type ON customer.customer_documents(document_type);
CREATE INDEX idx_customer_documents_verified ON customer.customer_documents(is_verified);
CREATE INDEX idx_customer_documents_expires ON customer.customer_documents(expires_at);

COMMENT ON TABLE customer.customer_documents IS 'Customer document lifecycle management (passports, IDs, certificates)';

-- Customer Groups table (FR-008)
CREATE TABLE IF NOT EXISTS customer.customer_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    group_type VARCHAR(30) NOT NULL CHECK (group_type IN ('family', 'business', 'syndicate', 'partnership', 'other')),
    description TEXT,
    parent_customer_id UUID REFERENCES customer.customers(id) ON DELETE SET NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'archived')),
    created_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_customer_groups_type ON customer.customer_groups(group_type);
CREATE INDEX idx_customer_groups_status ON customer.customer_groups(status);
CREATE INDEX idx_customer_groups_parent ON customer.customer_groups(parent_customer_id);

COMMENT ON TABLE customer.customer_groups IS 'Customer group management for family offices, business groups, syndicates';

-- Customer Group Members table (FR-008)
CREATE TABLE IF NOT EXISTS customer.customer_group_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES customer.customer_groups(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customer.customers(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL CHECK (role IN ('administrator', 'member', 'beneficiary', 'observer')),
    ownership_percentage DECIMAL(5, 2) CHECK (ownership_percentage >= 0 AND ownership_percentage <= 100),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(group_id, customer_id)
);

CREATE INDEX idx_customer_group_members_group ON customer.customer_group_members(group_id);
CREATE INDEX idx_customer_group_members_customer ON customer.customer_group_members(customer_id);
CREATE INDEX idx_customer_group_members_role ON customer.customer_group_members(role);

COMMENT ON TABLE customer.customer_group_members IS 'Membership records for customer groups with role-based access';
