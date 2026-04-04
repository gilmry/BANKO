CREATE SCHEMA IF NOT EXISTS customer;

CREATE TABLE customer.customers (
    id UUID PRIMARY KEY,
    customer_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'Pending',
    risk_score INT NOT NULL DEFAULT 0 CHECK (risk_score >= 0 AND risk_score <= 100),
    consent VARCHAR(20) NOT NULL DEFAULT 'NotGiven',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE customer.kyc_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customer.customers(id) ON DELETE CASCADE,
    full_name VARCHAR(255) NOT NULL,
    cin_or_rcs VARCHAR(100),
    birth_date DATE,
    nationality VARCHAR(100) DEFAULT 'Tunisia',
    profession VARCHAR(255),
    street VARCHAR(500),
    city VARCHAR(100),
    postal_code VARCHAR(20),
    country VARCHAR(100) DEFAULT 'Tunisia',
    phone VARCHAR(20),
    email VARCHAR(255),
    pep_status VARCHAR(50) DEFAULT 'Unknown',
    source_of_funds VARCHAR(100) DEFAULT 'Other',
    sector VARCHAR(255),
    submission_date TIMESTAMPTZ,
    approval_date TIMESTAMPTZ,
    rejection_reason TEXT,
    UNIQUE(customer_id)
);

CREATE TABLE customer.beneficiaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customer.customers(id) ON DELETE CASCADE,
    full_name VARCHAR(255) NOT NULL,
    share_percentage DECIMAL(5, 2) NOT NULL CHECK (share_percentage >= 0 AND share_percentage <= 100)
);

CREATE INDEX idx_customers_status ON customer.customers(status);
CREATE INDEX idx_customers_type ON customer.customers(customer_type);
CREATE INDEX idx_kyc_profiles_customer ON customer.kyc_profiles(customer_id);
CREATE INDEX idx_beneficiaries_customer ON customer.beneficiaries(customer_id);
