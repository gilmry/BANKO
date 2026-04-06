-- Cards table for STORY-CARD-01 through CARD-06
CREATE TABLE IF NOT EXISTS cards (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    card_type VARCHAR(20) NOT NULL CHECK (card_type IN ('Debit','Credit','Prepaid')),
    network VARCHAR(20) NOT NULL DEFAULT 'Visa',
    pan_hash VARCHAR(64) NOT NULL,
    masked_pan VARCHAR(19) NOT NULL,
    cvv_hash VARCHAR(64) NOT NULL,
    expiry_month SMALLINT NOT NULL,
    expiry_year SMALLINT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Issued' CHECK (status IN ('Issued','ActivationPending','Active','Blocked','Suspended','Cancelled','Expired')),
    activation_code_hash VARCHAR(64),
    daily_limit DECIMAL(18,3) NOT NULL DEFAULT 2000.000,
    monthly_limit DECIMAL(18,3) NOT NULL DEFAULT 50000.000,
    daily_spent DECIMAL(18,3) NOT NULL DEFAULT 0,
    monthly_spent DECIMAL(18,3) NOT NULL DEFAULT 0,
    is_contactless_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activated_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_cards_account ON cards(account_id);
CREATE INDEX IF NOT EXISTS idx_cards_customer ON cards(customer_id);
CREATE INDEX IF NOT EXISTS idx_cards_status ON cards(status);

-- Card transactions table for STORY-CARD-05 and CARD-06
CREATE TABLE IF NOT EXISTS card_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    card_id UUID NOT NULL REFERENCES cards(id),
    amount DECIMAL(18,3) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    merchant_name VARCHAR(200),
    mcc_code VARCHAR(4),
    status VARCHAR(20) NOT NULL DEFAULT 'Authorized' CHECK (status IN ('Authorized','Captured','Declined','Reversed')),
    auth_code VARCHAR(10) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_contactless BOOLEAN NOT NULL DEFAULT false,
    is_online BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX IF NOT EXISTS idx_card_tx_card ON card_transactions(card_id);
CREATE INDEX IF NOT EXISTS idx_card_tx_timestamp ON card_transactions(timestamp);
