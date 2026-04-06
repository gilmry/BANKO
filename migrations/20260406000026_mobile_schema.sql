-- Mobile Devices Registration
CREATE TABLE IF NOT EXISTS mobile_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    device_id VARCHAR(255) NOT NULL UNIQUE,
    device_name VARCHAR(200) NOT NULL,
    platform VARCHAR(10) NOT NULL CHECK (platform IN ('Ios','Android')),
    push_token TEXT,
    biometric_enabled BOOLEAN NOT NULL DEFAULT false,
    pin_hash VARCHAR(100),
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX idx_mobile_devices_customer ON mobile_devices(customer_id);
CREATE INDEX idx_mobile_devices_device ON mobile_devices(device_id);
CREATE INDEX idx_mobile_devices_active ON mobile_devices(is_active, customer_id);

-- Mobile Sessions
CREATE TABLE IF NOT EXISTS mobile_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    device_id VARCHAR(255) NOT NULL,
    token_hash VARCHAR(64) NOT NULL,
    refresh_token_hash VARCHAR(64) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mobile_sessions_customer ON mobile_sessions(customer_id);
CREATE INDEX idx_mobile_sessions_token ON mobile_sessions(token_hash);
CREATE INDEX idx_mobile_sessions_expires ON mobile_sessions(expires_at);

-- Frequent Beneficiaries for Quick Transfers
CREATE TABLE IF NOT EXISTS frequent_beneficiaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    beneficiary_name VARCHAR(200) NOT NULL,
    beneficiary_iban VARCHAR(34),
    beneficiary_phone VARCHAR(20),
    transfer_count INTEGER NOT NULL DEFAULT 1,
    last_transfer_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT check_iban_or_phone CHECK (beneficiary_iban IS NOT NULL OR beneficiary_phone IS NOT NULL),
    UNIQUE(customer_id, COALESCE(beneficiary_iban, ''), COALESCE(beneficiary_phone, ''))
);

CREATE INDEX idx_freq_beneficiary_customer ON frequent_beneficiaries(customer_id);
CREATE INDEX idx_freq_beneficiary_last_transfer ON frequent_beneficiaries(customer_id, last_transfer_at DESC);

-- Offline Cache Metadata
CREATE TABLE IF NOT EXISTS offline_cache_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    cache_version INTEGER NOT NULL,
    last_sync_at TIMESTAMPTZ NOT NULL,
    cache_ttl_hours INTEGER NOT NULL DEFAULT 24,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(customer_id)
);

CREATE INDEX idx_cache_metadata_customer ON offline_cache_metadata(customer_id);
