-- Identity BC: Sessions table
CREATE TABLE IF NOT EXISTS identity.sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES identity.users(id) ON DELETE CASCADE,
    token_hash VARCHAR(512) NOT NULL,
    ip_address VARCHAR(45),
    user_agent VARCHAR(512),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id
    ON identity.sessions (user_id);

CREATE INDEX IF NOT EXISTS idx_sessions_token_hash
    ON identity.sessions (token_hash);

CREATE INDEX IF NOT EXISTS idx_sessions_expires_at
    ON identity.sessions (expires_at);

-- Identity BC: Two-factor auth table
CREATE TABLE IF NOT EXISTS identity.two_factor_auth (
    user_id UUID PRIMARY KEY REFERENCES identity.users(id) ON DELETE CASCADE,
    secret VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending_verification',
    backup_codes TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
