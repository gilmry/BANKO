-- EPIC-19: Gestion Chèques (Cheque Management)
-- Schema for cheque processing, opposition, clearing, and banking blacklist

-- ============================================================
-- Main Cheques Table
-- ============================================================

CREATE TABLE IF NOT EXISTS cheques (
    id UUID PRIMARY KEY,
    cheque_number VARCHAR(7) NOT NULL UNIQUE,
    account_id UUID NOT NULL,
    drawer_name VARCHAR(200) NOT NULL,
    beneficiary_name VARCHAR(200) NOT NULL,
    amount DECIMAL(18,3) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    cheque_type VARCHAR(20) NOT NULL CHECK (cheque_type IN ('Bearer','Crossed','NotNegotiable')),
    issue_date DATE NOT NULL,
    expiry_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Issued' CHECK (status IN ('Issued','Presented','Encashed','Rejected','Opposed','Cleared','Expired','Cancelled')),
    rejection_reason VARCHAR(30),
    opposition_reason TEXT,
    encashed_at TIMESTAMPTZ,
    presented_at TIMESTAMPTZ,
    clearing_batch_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cheques_account ON cheques(account_id);
CREATE INDEX idx_cheques_status ON cheques(status);
CREATE INDEX idx_cheques_number ON cheques(cheque_number);
CREATE INDEX idx_cheques_expiry ON cheques(expiry_date);
CREATE INDEX idx_cheques_presented_date ON cheques(presented_at) WHERE status = 'Presented';

-- ============================================================
-- Cheque Opposition Table
-- ============================================================

CREATE TABLE IF NOT EXISTS cheque_oppositions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cheque_id UUID NOT NULL REFERENCES cheques(id) ON DELETE CASCADE,
    account_id UUID NOT NULL,
    reason TEXT NOT NULL,
    is_legal_opposition BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cheque_opp_cheque ON cheque_oppositions(cheque_id);
CREATE INDEX idx_cheque_opp_account ON cheque_oppositions(account_id);
CREATE INDEX idx_cheque_opp_legal ON cheque_oppositions(is_legal_opposition);

-- ============================================================
-- Banking Blacklist Table (Interdit Bancaire)
-- ============================================================

CREATE TABLE IF NOT EXISTS banking_blacklist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL UNIQUE,
    reason TEXT NOT NULL,
    rejection_count INTEGER NOT NULL DEFAULT 0 CHECK (rejection_count >= 0),
    blacklisted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    lifted_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_blacklist_customer ON banking_blacklist(customer_id);
CREATE INDEX idx_blacklist_active ON banking_blacklist(is_active) WHERE is_active = true;
CREATE INDEX idx_blacklist_active_customer ON banking_blacklist(customer_id, is_active) WHERE is_active = true;

-- ============================================================
-- Cheque Clearing Batches Table
-- ============================================================

CREATE TABLE IF NOT EXISTS cheque_clearing_batches (
    id UUID PRIMARY KEY,
    clearing_date DATE NOT NULL,
    total_amount DECIMAL(18,3) NOT NULL DEFAULT 0 CHECK (total_amount >= 0),
    cheque_count INTEGER NOT NULL DEFAULT 0 CHECK (cheque_count >= 0),
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending','Submitted','Processed','PartiallyRejected')),
    submitted_at TIMESTAMPTZ,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clearing_date ON cheque_clearing_batches(clearing_date);
CREATE INDEX idx_clearing_status ON cheque_clearing_batches(status);

-- ============================================================
-- Cheque Clearing Results Table
-- ============================================================

CREATE TABLE IF NOT EXISTS cheque_clearing_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    batch_id UUID NOT NULL REFERENCES cheque_clearing_batches(id) ON DELETE CASCADE,
    cheque_id UUID NOT NULL REFERENCES cheques(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL CHECK (status IN ('Cleared','Rejected','PartiallyRejected')),
    rejection_code VARCHAR(10),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clearing_results_batch ON cheque_clearing_results(batch_id);
CREATE INDEX idx_clearing_results_cheque ON cheque_clearing_results(cheque_id);
CREATE INDEX idx_clearing_results_status ON cheque_clearing_results(status);

-- ============================================================
-- Trigger to update updated_at on cheques
-- ============================================================

CREATE OR REPLACE FUNCTION update_cheques_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_cheques_updated_at
BEFORE UPDATE ON cheques
FOR EACH ROW
EXECUTE FUNCTION update_cheques_updated_at();

-- ============================================================
-- Trigger to update updated_at on banking_blacklist
-- ============================================================

CREATE OR REPLACE FUNCTION update_banking_blacklist_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_banking_blacklist_updated_at
BEFORE UPDATE ON banking_blacklist
FOR EACH ROW
EXECUTE FUNCTION update_banking_blacklist_updated_at();

-- ============================================================
-- Trigger to update updated_at on cheque_clearing_batches
-- ============================================================

CREATE OR REPLACE FUNCTION update_cheque_clearing_batches_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_cheque_clearing_batches_updated_at
BEFORE UPDATE ON cheque_clearing_batches
FOR EACH ROW
EXECUTE FUNCTION update_cheque_clearing_batches_updated_at();
