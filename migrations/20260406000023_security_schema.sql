-- Rate Limiting Configuration
CREATE TABLE IF NOT EXISTS rate_limit_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint_pattern VARCHAR(200) NOT NULL,
    max_requests_per_minute INTEGER NOT NULL DEFAULT 60,
    window_seconds INTEGER NOT NULL DEFAULT 60,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_rate_limit_endpoint ON rate_limit_config(endpoint_pattern);

-- IP Whitelists for Customers
CREATE TABLE IF NOT EXISTS ip_whitelists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    allowed_ips TEXT[] NOT NULL DEFAULT '{}',
    is_strict_mode BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ip_whitelist_customer ON ip_whitelists(customer_id);
CREATE UNIQUE INDEX idx_ip_whitelist_customer_unique ON ip_whitelists(customer_id);

-- Fraud Evaluation Records
CREATE TABLE IF NOT EXISTS fraud_evaluations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    transaction_id UUID,
    total_score INTEGER NOT NULL CHECK (total_score >= 0 AND total_score <= 100),
    decision VARCHAR(20) NOT NULL CHECK (decision IN ('Allow','Challenge','Block','ManualReview')),
    rule_scores JSONB NOT NULL DEFAULT '[]',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_fraud_eval_customer ON fraud_evaluations(customer_id);
CREATE INDEX idx_fraud_eval_decision ON fraud_evaluations(decision);
CREATE INDEX idx_fraud_eval_timestamp ON fraud_evaluations(evaluated_at DESC);

-- Hash Chain for Audit Trail
CREATE TABLE IF NOT EXISTS hash_chain (
    sequence BIGSERIAL PRIMARY KEY,
    previous_hash VARCHAR(64) NOT NULL,
    current_hash VARCHAR(64) NOT NULL,
    operation_type VARCHAR(100) NOT NULL,
    operation_id UUID NOT NULL,
    data_hash VARCHAR(64) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_hash_chain_operation ON hash_chain(operation_id);
CREATE INDEX idx_hash_chain_timestamp ON hash_chain(timestamp DESC);
CREATE INDEX idx_hash_chain_type ON hash_chain(operation_type);

-- Geolocation Blocks for Customers
CREATE TABLE IF NOT EXISTS geolocation_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    blocked_countries TEXT[] NOT NULL DEFAULT '{}',
    alert_countries TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_geolocation_blocks_customer ON geolocation_blocks(customer_id);
CREATE UNIQUE INDEX idx_geolocation_blocks_customer_unique ON geolocation_blocks(customer_id);

-- Transaction Security Context Log
CREATE TABLE IF NOT EXISTS transaction_security_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    source_ip VARCHAR(45),
    detected_location_country VARCHAR(2),
    detected_location_city VARCHAR(100),
    is_vpn_detected BOOLEAN NOT NULL DEFAULT false,
    fraud_score INTEGER,
    fraud_decision VARCHAR(20),
    rate_limit_triggered BOOLEAN NOT NULL DEFAULT false,
    ip_whitelisted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_security_logs_transaction ON transaction_security_logs(transaction_id);
CREATE INDEX idx_security_logs_customer ON transaction_security_logs(customer_id);
CREATE INDEX idx_security_logs_timestamp ON transaction_security_logs(created_at DESC);
