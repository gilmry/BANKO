-- STORY-CONS-01 + STORY-CONS-02: INPDP Consent Management + Data Rights
-- Loi 2004-63 compliance

CREATE TABLE customer.consents (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL,
    purpose VARCHAR(30) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Active',
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ
);

CREATE TABLE customer.data_rights_requests (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL,
    request_type VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    details TEXT,
    response TEXT,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    deadline TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_consents_customer ON customer.consents(customer_id);
CREATE INDEX idx_consents_purpose ON customer.consents(customer_id, purpose);
CREATE INDEX idx_data_rights_customer ON customer.data_rights_requests(customer_id);
CREATE INDEX idx_data_rights_status ON customer.data_rights_requests(status);
