-- Sanctions bounded context schema
-- STORY-SAN-03: Sanctions lists, entries, screening results
-- GAFI Rec. 16 [REF-66], INV-14

CREATE SCHEMA IF NOT EXISTS sanctions;

-- Sanctions lists metadata
CREATE TABLE sanctions.lists (
    id UUID PRIMARY KEY,
    source VARCHAR(20) NOT NULL,
    version VARCHAR(50) NOT NULL,
    entry_count INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Sanctions entries
CREATE TABLE sanctions.entries (
    id UUID PRIMARY KEY,
    list_id UUID NOT NULL REFERENCES sanctions.lists(id),
    list_source VARCHAR(20) NOT NULL,
    full_name VARCHAR(500) NOT NULL,
    normalized_name VARCHAR(500) NOT NULL,
    aliases TEXT[] DEFAULT '{}',
    country VARCHAR(100),
    listing_date DATE,
    delisting_date DATE,
    additional_info TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Screening results
CREATE TABLE sanctions.screening_results (
    id UUID PRIMARY KEY,
    screened_name VARCHAR(500) NOT NULL,
    normalized_name VARCHAR(500) NOT NULL,
    status VARCHAR(20) NOT NULL,
    highest_score INTEGER NOT NULL DEFAULT 0,
    match_count INTEGER NOT NULL DEFAULT 0,
    match_details JSONB NOT NULL DEFAULT '[]',
    screened_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Screening audit log (append-only)
CREATE TABLE sanctions.screening_audit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    screening_result_id UUID NOT NULL REFERENCES sanctions.screening_results(id),
    action VARCHAR(50) NOT NULL,
    details TEXT,
    performed_by VARCHAR(100),
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- List sync history
CREATE TABLE sanctions.sync_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    list_source VARCHAR(20) NOT NULL,
    version VARCHAR(50) NOT NULL,
    entries_added INTEGER NOT NULL DEFAULT 0,
    entries_removed INTEGER NOT NULL DEFAULT 0,
    entries_total INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'Completed',
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_sanctions_entries_list ON sanctions.entries(list_id);
CREATE INDEX idx_sanctions_entries_source ON sanctions.entries(list_source);
CREATE INDEX idx_sanctions_entries_name ON sanctions.entries(normalized_name);
CREATE INDEX idx_sanctions_entries_active ON sanctions.entries(active) WHERE active = TRUE;
CREATE INDEX idx_sanctions_results_status ON sanctions.screening_results(status);
CREATE INDEX idx_sanctions_results_date ON sanctions.screening_results(screened_at);
CREATE INDEX idx_sanctions_results_name ON sanctions.screening_results(normalized_name);
CREATE INDEX idx_sanctions_sync_source ON sanctions.sync_history(list_source);
