-- Event store schema for event sourcing and saga pattern support

-- Core event store table
CREATE TABLE IF NOT EXISTS event_store (
    sequence_number BIGSERIAL PRIMARY KEY,
    id UUID NOT NULL UNIQUE,
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_event_store_aggregate ON event_store(aggregate_id);
CREATE INDEX IF NOT EXISTS idx_event_store_type ON event_store(event_type);
CREATE INDEX IF NOT EXISTS idx_event_store_timestamp ON event_store(timestamp);
CREATE INDEX IF NOT EXISTS idx_event_store_sequence ON event_store(sequence_number);

-- Aggregate snapshots for optimization
CREATE TABLE IF NOT EXISTS aggregate_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    state JSONB NOT NULL,
    version BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_snapshots_aggregate ON aggregate_snapshots(aggregate_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_snapshots_aggregate_version ON aggregate_snapshots(aggregate_id, version);

-- Saga execution tracking
CREATE TABLE IF NOT EXISTS sagas (
    id UUID PRIMARY KEY,
    saga_type VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Started' CHECK (status IN ('Started', 'Compensating', 'Compensated', 'Completed', 'Failed')),
    context JSONB NOT NULL DEFAULT '{}',
    completed_steps JSONB NOT NULL DEFAULT '[]',
    idempotency_key VARCHAR(255) UNIQUE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT
);

CREATE INDEX IF NOT EXISTS idx_sagas_status ON sagas(status);
CREATE INDEX IF NOT EXISTS idx_sagas_idempotency ON sagas(idempotency_key);
CREATE INDEX IF NOT EXISTS idx_sagas_started_at ON sagas(started_at);
