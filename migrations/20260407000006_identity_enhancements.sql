-- BANKO Identity BC Enhancement
-- WebAuthn credentials, password history, API keys, OAuth2 clients/codes/tokens

ALTER TABLE identity.users ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMPTZ;
ALTER TABLE identity.users ADD COLUMN IF NOT EXISTS failed_login_attempts SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE identity.users ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ;
ALTER TABLE identity.users ADD COLUMN IF NOT EXISTS password_reset_required BOOLEAN NOT NULL DEFAULT FALSE;

-- WebAuthn Credentials
CREATE TABLE IF NOT EXISTS identity.webauthn_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    credential_id VARCHAR(500) NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    counter BIGINT NOT NULL DEFAULT 0,
    transports VARCHAR(50)[] DEFAULT '{"usb"}', -- usb, nfc, ble, internal
    credential_type VARCHAR(50) NOT NULL CHECK (credential_type IN ('platform', 'cross_platform')),
    attestation_type VARCHAR(50) CHECK (attestation_type IN ('none', 'indirect', 'direct', 'enterprise')),
    aaguid VARCHAR(36), -- Authenticator AAGUID
    nickname VARCHAR(100),
    is_backup_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    is_backup_state BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_webauthn_user ON identity.webauthn_credentials(user_id);
CREATE INDEX idx_webauthn_credential_id ON identity.webauthn_credentials(credential_id);
CREATE INDEX idx_webauthn_created ON identity.webauthn_credentials(created_at);

COMMENT ON TABLE identity.webauthn_credentials IS 'WebAuthn FIDO2 credentials for passwordless MFA';

-- Password History (prevent reuse)
CREATE TABLE IF NOT EXISTS identity.password_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    set_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    changed_by_user BOOLEAN NOT NULL DEFAULT TRUE,
    password_strength_score SMALLINT CHECK (password_strength_score BETWEEN 1 AND 5)
);

CREATE INDEX idx_password_history_user ON identity.password_history(user_id);
CREATE INDEX idx_password_history_set ON identity.password_history(set_at);

COMMENT ON TABLE identity.password_history IS 'Password history to enforce non-reuse policies';

-- API Keys for service-to-service authentication
CREATE TABLE IF NOT EXISTS identity.api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    key_name VARCHAR(255) NOT NULL,
    key_hash VARCHAR(255) NOT NULL UNIQUE,
    scopes VARCHAR(100)[] NOT NULL DEFAULT '{"read"}', -- read, write, delete, admin
    rate_limit_per_hour INTEGER DEFAULT 1000,
    ip_whitelist VARCHAR(50)[], -- CIDR format, e.g., ['192.168.1.0/24']
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_api_keys_user ON identity.api_keys(user_id);
CREATE INDEX idx_api_keys_active ON identity.api_keys(is_active);
CREATE INDEX idx_api_keys_expires ON identity.api_keys(expires_at);

COMMENT ON TABLE identity.api_keys IS 'API keys for programmatic access with scope and rate limiting';

-- OAuth2 Clients (for third-party integrations)
CREATE TABLE IF NOT EXISTS identity.oauth2_clients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id VARCHAR(100) NOT NULL UNIQUE,
    client_secret_hash VARCHAR(255) NOT NULL,
    client_name VARCHAR(255) NOT NULL,
    client_uri VARCHAR(500),
    logo_uri VARCHAR(500),
    description TEXT,
    redirect_uris VARCHAR(500)[] NOT NULL,
    post_logout_redirect_uris VARCHAR(500)[],
    allowed_grant_types VARCHAR(50)[] NOT NULL CHECK (allowed_grant_types <@ '{"authorization_code", "client_credentials", "refresh_token", "implicit", "password"}'::text[]),
    allowed_response_types VARCHAR(50)[] DEFAULT '{"code"}' CHECK (allowed_response_types <@ '{"code", "token", "id_token"}'::text[]),
    scopes VARCHAR(100)[] NOT NULL DEFAULT '{"openid", "profile", "email"}',
    token_endpoint_auth_method VARCHAR(50) NOT NULL DEFAULT 'client_secret_basic' CHECK (token_endpoint_auth_method IN ('client_secret_basic', 'client_secret_post', 'none')),
    token_expiry_seconds INTEGER DEFAULT 3600,
    refresh_token_expiry_seconds INTEGER DEFAULT 2592000, -- 30 days
    require_pkce BOOLEAN NOT NULL DEFAULT FALSE,
    is_confidential BOOLEAN NOT NULL DEFAULT TRUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_oauth2_client_id ON identity.oauth2_clients(client_id);
CREATE INDEX idx_oauth2_active ON identity.oauth2_clients(is_active);

COMMENT ON TABLE identity.oauth2_clients IS 'OAuth2 client registrations for third-party integrations';

-- OAuth2 Authorization Codes
CREATE TABLE IF NOT EXISTS identity.oauth2_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id VARCHAR(100) NOT NULL REFERENCES identity.oauth2_clients(client_id),
    user_id UUID REFERENCES identity.users(id) ON DELETE SET NULL,
    code VARCHAR(255) NOT NULL UNIQUE,
    code_hash VARCHAR(255) NOT NULL UNIQUE,
    redirect_uri VARCHAR(500) NOT NULL,
    scopes VARCHAR(100)[] NOT NULL,
    nonce VARCHAR(500), -- for OIDC
    pkce_challenge VARCHAR(128),
    pkce_challenge_method VARCHAR(10) CHECK (pkce_challenge_method IN ('plain', 'S256')),
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_oauth2_codes_client ON identity.oauth2_codes(client_id);
CREATE INDEX idx_oauth2_codes_user ON identity.oauth2_codes(user_id);
CREATE INDEX idx_oauth2_codes_expires ON identity.oauth2_codes(expires_at);

COMMENT ON TABLE identity.oauth2_codes IS 'OAuth2 authorization codes with PKCE support';

-- OAuth2 Access Tokens
CREATE TABLE IF NOT EXISTS identity.oauth2_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id VARCHAR(100) NOT NULL REFERENCES identity.oauth2_clients(client_id),
    user_id UUID REFERENCES identity.users(id) ON DELETE SET NULL,
    access_token_hash VARCHAR(255) NOT NULL UNIQUE,
    refresh_token_hash VARCHAR(255) UNIQUE,
    scopes VARCHAR(100)[] NOT NULL,
    token_type VARCHAR(20) NOT NULL DEFAULT 'Bearer' CHECK (token_type IN ('Bearer', 'DPoP')),
    dpop_proof VARCHAR(1000), -- DPoP (Demonstration of Proof-of-Possession) proof
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ,
    ip_address VARCHAR(45), -- IPv6 compatible
    user_agent VARCHAR(500),
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_oauth2_tokens_client ON identity.oauth2_tokens(client_id);
CREATE INDEX idx_oauth2_tokens_user ON identity.oauth2_tokens(user_id);
CREATE INDEX idx_oauth2_tokens_expires ON identity.oauth2_tokens(expires_at);
CREATE INDEX idx_oauth2_tokens_revoked ON identity.oauth2_tokens(is_revoked);

COMMENT ON TABLE identity.oauth2_tokens IS 'OAuth2 access and refresh tokens with DPoP support';

-- MFA Methods (phone, email, app, biometric)
CREATE TABLE IF NOT EXISTS identity.mfa_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    mfa_type VARCHAR(50) NOT NULL CHECK (mfa_type IN ('totp', 'sms', 'email', 'webauthn', 'push_notification')),
    mfa_value VARCHAR(255) NOT NULL, -- phone number, email, or TOTP secret (encrypted)
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    verified_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    backup_codes TEXT[], -- encrypted backup codes
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mfa_methods_user ON identity.mfa_methods(user_id);
CREATE INDEX idx_mfa_methods_type ON identity.mfa_methods(mfa_type);
CREATE INDEX idx_mfa_methods_active ON identity.mfa_methods(is_active);

COMMENT ON TABLE identity.mfa_methods IS 'Multi-factor authentication methods per user';

-- Device Trust (for step-up authentication)
CREATE TABLE IF NOT EXISTS identity.trusted_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    device_fingerprint VARCHAR(255) NOT NULL,
    device_name VARCHAR(255),
    device_type VARCHAR(50) CHECK (device_type IN ('mobile', 'desktop', 'tablet', 'other')),
    browser_info VARCHAR(500),
    ip_address VARCHAR(45),
    is_trusted BOOLEAN NOT NULL DEFAULT FALSE,
    trusted_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    trust_expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_trusted_devices_user ON identity.trusted_devices(user_id);
CREATE INDEX idx_trusted_devices_trusted ON identity.trusted_devices(is_trusted);

COMMENT ON TABLE identity.trusted_devices IS 'Device trust registry for passwordless re-authentication';
