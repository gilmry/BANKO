-- ReferenceData Bounded Context Schema

CREATE SCHEMA IF NOT EXISTS reference_data;

-- Country Codes Table (ISO 3166)
CREATE TABLE reference_data.country_codes (
    id UUID PRIMARY KEY,
    iso_alpha2 VARCHAR(2) NOT NULL UNIQUE,
    iso_alpha3 VARCHAR(3) NOT NULL UNIQUE,
    iso_numeric VARCHAR(3) NOT NULL UNIQUE,
    name_en VARCHAR(255) NOT NULL,
    name_fr VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    is_sanctioned BOOLEAN NOT NULL DEFAULT false,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_country_codes_iso2 ON reference_data.country_codes(iso_alpha2);
CREATE INDEX idx_country_codes_iso3 ON reference_data.country_codes(iso_alpha3);
CREATE INDEX idx_country_codes_effective ON reference_data.country_codes(effective_from, effective_to);

-- Currency References Table (ISO 4217)
CREATE TABLE reference_data.currency_references (
    id UUID PRIMARY KEY,
    code VARCHAR(3) NOT NULL UNIQUE,
    name_en VARCHAR(255) NOT NULL,
    name_fr VARCHAR(255) NOT NULL,
    decimal_places INTEGER NOT NULL DEFAULT 2,
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_currency_code ON reference_data.currency_references(code);
CREATE INDEX idx_currency_active ON reference_data.currency_references(is_active, effective_from, effective_to);

-- Bank Codes Table (BIC/SWIFT)
CREATE TABLE reference_data.bank_codes (
    id UUID PRIMARY KEY,
    bic VARCHAR(11) NOT NULL UNIQUE,
    bank_name VARCHAR(255) NOT NULL,
    country_iso_alpha2 VARCHAR(2) NOT NULL,
    country_iso_alpha3 VARCHAR(3) NOT NULL,
    country_iso_numeric VARCHAR(3) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bank_codes_bic ON reference_data.bank_codes(bic);
CREATE INDEX idx_bank_codes_active ON reference_data.bank_codes(is_active);
CREATE INDEX idx_bank_codes_country ON reference_data.bank_codes(country_iso_alpha2);

-- Branch Codes Table
CREATE TABLE reference_data.branch_codes (
    id UUID PRIMARY KEY,
    branch_code VARCHAR(50) NOT NULL UNIQUE,
    branch_name VARCHAR(255) NOT NULL,
    bank_bic VARCHAR(11) NOT NULL,
    city VARCHAR(100) NOT NULL,
    address VARCHAR(500) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (bank_bic) REFERENCES reference_data.bank_codes(bic)
);

CREATE INDEX idx_branch_codes_code ON reference_data.branch_codes(branch_code);
CREATE INDEX idx_branch_codes_bic ON reference_data.branch_codes(bank_bic);
CREATE INDEX idx_branch_codes_active ON reference_data.branch_codes(is_active);
CREATE INDEX idx_branch_codes_city ON reference_data.branch_codes(city);

-- Holiday Calendar Table (Tunisian Banking Holidays)
CREATE TABLE reference_data.holiday_calendar (
    id UUID PRIMARY KEY,
    holiday_date TIMESTAMPTZ NOT NULL,
    holiday_name_en VARCHAR(255) NOT NULL,
    holiday_name_fr VARCHAR(255) NOT NULL,
    holiday_name_ar VARCHAR(255) NOT NULL,
    holiday_type VARCHAR(50) NOT NULL, -- National, Banking, Religious
    is_banking_holiday BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_holiday_date ON reference_data.holiday_calendar(holiday_date);
CREATE INDEX idx_holiday_banking ON reference_data.holiday_calendar(is_banking_holiday);
CREATE INDEX idx_holiday_type ON reference_data.holiday_calendar(holiday_type);

-- System Parameters Table (Configurable Thresholds)
CREATE TABLE reference_data.system_parameters (
    id UUID PRIMARY KEY,
    key VARCHAR(255) NOT NULL UNIQUE,
    value TEXT NOT NULL,
    parameter_type VARCHAR(50) NOT NULL, -- Integer, Decimal, String, Boolean
    category VARCHAR(100) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_system_param_key ON reference_data.system_parameters(key);
CREATE INDEX idx_system_param_category ON reference_data.system_parameters(category);
CREATE INDEX idx_system_param_active ON reference_data.system_parameters(is_active, effective_from, effective_to);

-- Regulatory Codes Table (BCT Classifications, IFRS Categories)
CREATE TABLE reference_data.regulatory_codes (
    id UUID PRIMARY KEY,
    code VARCHAR(50) NOT NULL UNIQUE,
    description_en TEXT NOT NULL,
    description_fr TEXT NOT NULL,
    classification VARCHAR(100) NOT NULL, -- StandardRisk, LowerRisk, HigherRisk, AmortizedCost, FairValueThroughOci, FairValueThroughPl
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_regulatory_code ON reference_data.regulatory_codes(code);
CREATE INDEX idx_regulatory_classification ON reference_data.regulatory_codes(classification);
CREATE INDEX idx_regulatory_active ON reference_data.regulatory_codes(is_active, effective_from, effective_to);

-- Fee Schedule References Table
CREATE TABLE reference_data.fee_schedule_references (
    id UUID PRIMARY KEY,
    fee_type VARCHAR(100) NOT NULL, -- AccountMaintenance, Transaction, Transfer, ForeignExchange, LatePayment, Overdraft, ATMWithdrawal, CheckIssue
    amount_cents BIGINT NOT NULL DEFAULT 0,
    currency_code VARCHAR(3) NOT NULL,
    description_en VARCHAR(500) NOT NULL,
    description_fr VARCHAR(500) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (currency_code) REFERENCES reference_data.currency_references(code)
);

CREATE INDEX idx_fee_schedule_type ON reference_data.fee_schedule_references(fee_type);
CREATE INDEX idx_fee_schedule_currency ON reference_data.fee_schedule_references(currency_code);
CREATE INDEX idx_fee_schedule_active ON reference_data.fee_schedule_references(is_active, effective_from, effective_to);

-- Comments for documentation
COMMENT ON SCHEMA reference_data IS 'ReferenceData Bounded Context - Centralized reference data for the entire banking system';
COMMENT ON TABLE reference_data.country_codes IS 'ISO 3166 country codes with sanctions status and multilingual names';
COMMENT ON TABLE reference_data.currency_references IS 'ISO 4217 currency codes with decimal places configuration';
COMMENT ON TABLE reference_data.bank_codes IS 'BIC/SWIFT bank codes with country association';
COMMENT ON TABLE reference_data.branch_codes IS 'Bank branch codes with location information';
COMMENT ON TABLE reference_data.holiday_calendar IS 'Tunisian banking holidays (national, banking, religious)';
COMMENT ON TABLE reference_data.system_parameters IS 'System configuration parameters with effective date ranges';
COMMENT ON TABLE reference_data.regulatory_codes IS 'Regulatory classification codes (BCT asset classes, IFRS categories)';
COMMENT ON TABLE reference_data.fee_schedule_references IS 'Fee schedules with effective date ranges';
