-- Initial schema for BANKO
-- Creates required extensions and base schemas for all 12 bounded contexts

-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Schemas for each Bounded Context
CREATE SCHEMA IF NOT EXISTS customer;
CREATE SCHEMA IF NOT EXISTS account;
CREATE SCHEMA IF NOT EXISTS credit;
CREATE SCHEMA IF NOT EXISTS aml;
CREATE SCHEMA IF NOT EXISTS sanctions;
CREATE SCHEMA IF NOT EXISTS prudential;
CREATE SCHEMA IF NOT EXISTS accounting;
CREATE SCHEMA IF NOT EXISTS reporting;
CREATE SCHEMA IF NOT EXISTS payment;
CREATE SCHEMA IF NOT EXISTS fx;
CREATE SCHEMA IF NOT EXISTS governance;
CREATE SCHEMA IF NOT EXISTS identity;

-- Audit trail table (governance BC)
CREATE TABLE IF NOT EXISTS governance.audit_trail (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_type VARCHAR(100) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    actor_id UUID,
    payload JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_trail_entity
    ON governance.audit_trail (entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_trail_created_at
    ON governance.audit_trail (created_at);
