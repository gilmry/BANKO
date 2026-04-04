CREATE SCHEMA IF NOT EXISTS account;

CREATE TABLE account.accounts (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL,
    rib VARCHAR(50) UNIQUE NOT NULL,
    account_type VARCHAR(50) NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    available_balance BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    status VARCHAR(50) NOT NULL DEFAULT 'Active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE account.movements (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES account.accounts(id),
    movement_type VARCHAR(50) NOT NULL,
    amount BIGINT NOT NULL,
    balance_after BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    description TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_accounts_customer ON account.accounts(customer_id);
CREATE INDEX idx_accounts_status ON account.accounts(status);
CREATE INDEX idx_accounts_rib ON account.accounts(rib);
CREATE INDEX idx_movements_account ON account.movements(account_id);
CREATE INDEX idx_movements_created ON account.movements(created_at);
