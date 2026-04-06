CREATE SCHEMA IF NOT EXISTS payment;

CREATE TABLE payment.payment_orders (
    id UUID PRIMARY KEY,
    sender_account_id UUID NOT NULL,
    beneficiary_name VARCHAR(255) NOT NULL,
    beneficiary_rib VARCHAR(30),
    beneficiary_bic VARCHAR(11),
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    payment_type VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    screening_status VARCHAR(20) NOT NULL DEFAULT 'NotScreened',
    reference VARCHAR(100) NOT NULL,
    description TEXT,
    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_at TIMESTAMPTZ,
    executed_at TIMESTAMPTZ
);

CREATE TABLE payment.transfers (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES payment.payment_orders(id),
    counterparty_rib VARCHAR(30) NOT NULL,
    clearing_ref VARCHAR(100),
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    transfer_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE payment.swift_messages (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES payment.payment_orders(id),
    message_type VARCHAR(10) NOT NULL,
    sender_bic VARCHAR(11) NOT NULL,
    receiver_bic VARCHAR(11) NOT NULL,
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL,
    reference VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_orders_account ON payment.payment_orders(sender_account_id);
CREATE INDEX idx_payment_orders_status ON payment.payment_orders(status);
CREATE INDEX idx_payment_orders_screening ON payment.payment_orders(screening_status);
CREATE INDEX idx_transfers_order ON payment.transfers(order_id);
CREATE INDEX idx_swift_messages_order ON payment.swift_messages(order_id);
