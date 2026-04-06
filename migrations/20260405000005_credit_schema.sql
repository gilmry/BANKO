-- Credit bounded context schema
-- STORY-CR-03: Loan, LoanSchedule, Provision tables
-- Circ. 91-24 [REF-14], Circ. 2023-02 [REF-24]

CREATE SCHEMA IF NOT EXISTS credit;

-- Loans table (aggregate root)
CREATE TABLE credit.loans (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL,
    account_id UUID NOT NULL,
    amount BIGINT NOT NULL,           -- stored in millimes (TND has 3 decimal places)
    interest_rate DOUBLE PRECISION NOT NULL,
    term_months INTEGER NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    asset_class INTEGER NOT NULL DEFAULT 0 CHECK (asset_class BETWEEN 0 AND 4),
    status VARCHAR(20) NOT NULL DEFAULT 'Applied',
    days_past_due INTEGER NOT NULL DEFAULT 0,
    disbursement_date DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Loan provisions table
CREATE TABLE credit.loan_provisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL REFERENCES credit.loans(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    asset_class INTEGER NOT NULL CHECK (asset_class BETWEEN 0 AND 4),
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Loan schedule / installments table
CREATE TABLE credit.loan_installments (
    id UUID PRIMARY KEY,
    loan_id UUID NOT NULL REFERENCES credit.loans(id) ON DELETE CASCADE,
    installment_number INTEGER NOT NULL,
    due_date DATE NOT NULL,
    principal_amount BIGINT NOT NULL,
    interest_amount BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,
    remaining_balance BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TND',
    paid BOOLEAN NOT NULL DEFAULT FALSE,
    paid_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for frequent queries
CREATE INDEX idx_loans_account_id ON credit.loans(account_id);
CREATE INDEX idx_loans_customer_id ON credit.loans(customer_id);
CREATE INDEX idx_loans_status ON credit.loans(status);
CREATE INDEX idx_loans_asset_class ON credit.loans(asset_class);
CREATE INDEX idx_loans_status_asset_class ON credit.loans(status, asset_class);
CREATE INDEX idx_loan_provisions_loan_id ON credit.loan_provisions(loan_id);
CREATE INDEX idx_loan_installments_loan_id ON credit.loan_installments(loan_id);
CREATE INDEX idx_loan_installments_due_date ON credit.loan_installments(due_date);
CREATE INDEX idx_loan_installments_unpaid ON credit.loan_installments(loan_id, paid) WHERE NOT paid;
