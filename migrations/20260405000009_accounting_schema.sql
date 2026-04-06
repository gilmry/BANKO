-- Accounting (BC7) schema — NCT 01/21/24/25
CREATE SCHEMA IF NOT EXISTS accounting;

-- Chart of accounts (plan comptable bancaire NCT)
CREATE TABLE accounting.chart_of_accounts (
    code VARCHAR(20) PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    account_type VARCHAR(20) NOT NULL,
    nct_ref VARCHAR(50),
    parent_code VARCHAR(20),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Journal entries (immutable after posting)
CREATE TABLE accounting.journal_entries (
    id UUID PRIMARY KEY,
    journal_code VARCHAR(10) NOT NULL,
    entry_date DATE NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    reversal_of UUID REFERENCES accounting.journal_entries(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    posted_at TIMESTAMPTZ
);

-- Journal lines (debit/credit per account)
CREATE TABLE accounting.journal_lines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entry_id UUID NOT NULL REFERENCES accounting.journal_entries(id),
    account_code VARCHAR(20) NOT NULL REFERENCES accounting.chart_of_accounts(code),
    debit BIGINT NOT NULL DEFAULT 0,
    credit BIGINT NOT NULL DEFAULT 0,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_debit_or_credit CHECK (debit >= 0 AND credit >= 0 AND (debit > 0 OR credit > 0))
);

-- Period closing
CREATE TABLE accounting.closed_periods (
    period VARCHAR(7) PRIMARY KEY,
    closed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_by VARCHAR(100)
);

-- IFRS 9 ECL staging (preparation)
CREATE TABLE accounting.ecl_staging (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id UUID NOT NULL,
    stage VARCHAR(10) NOT NULL,
    probability_of_default DOUBLE PRECISION NOT NULL,
    loss_given_default DOUBLE PRECISION NOT NULL,
    exposure_at_default BIGINT NOT NULL,
    ecl_amount BIGINT NOT NULL,
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed NCT chart of accounts (plan comptable bancaire)
-- Classe 1: Capitaux propres et assimilés
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('10', 'Capital', 'Equity', 'NCT-01'),
('101', 'Capital social', 'Equity', 'NCT-01'),
('106', 'Réserves', 'Equity', 'NCT-01'),
('12', 'Résultat de l''exercice', 'Equity', 'NCT-01');

-- Classe 2: Actifs immobilisés
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('20', 'Immobilisations incorporelles', 'Asset', 'NCT-21'),
('21', 'Immobilisations corporelles', 'Asset', 'NCT-21'),
('22', 'Immobilisations financières', 'Asset', 'NCT-21');

-- Classe 3: Actifs courants (créances bancaires)
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('30', 'Créances sur les établissements bancaires', 'Asset', 'NCT-24'),
('31', 'Créances sur la clientèle', 'Asset', 'NCT-24'),
('32', 'Portefeuille-titres commercial', 'Asset', 'NCT-25'),
('33', 'Portefeuille d''investissement', 'Asset', 'NCT-25');

-- Classe 4: Passifs courants
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('40', 'Banque Centrale et CCP', 'Liability', 'NCT-21'),
('41', 'Dépôts et avoirs des établissements bancaires', 'Liability', 'NCT-21'),
('42', 'Dépôts et avoirs de la clientèle', 'Liability', 'NCT-21'),
('43', 'Emprunts et ressources spéciales', 'Liability', 'NCT-21'),
('44', 'Autres passifs', 'Liability', 'NCT-21');

-- Classe 5: Trésorerie
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('50', 'Caisse', 'Asset', 'NCT-21'),
('51', 'Comptes bancaires', 'Asset', 'NCT-21');

-- Classe 6: Charges
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('60', 'Charges d''exploitation bancaire', 'Expense', 'NCT-21'),
('61', 'Charges de personnel', 'Expense', 'NCT-21'),
('62', 'Charges générales d''exploitation', 'Expense', 'NCT-21'),
('63', 'Dotations aux provisions et résultat des corrections de valeurs', 'Expense', 'NCT-21');

-- Classe 7: Produits
INSERT INTO accounting.chart_of_accounts (code, label, account_type, nct_ref) VALUES
('70', 'Produits d''exploitation bancaire', 'Revenue', 'NCT-21'),
('71', 'Intérêts et revenus assimilés', 'Revenue', 'NCT-24'),
('72', 'Commissions', 'Revenue', 'NCT-21'),
('73', 'Gains sur portefeuille-titres', 'Revenue', 'NCT-25'),
('74', 'Revenus du portefeuille d''investissement', 'Revenue', 'NCT-25');

-- Indexes
CREATE INDEX idx_accounting_entries_date ON accounting.journal_entries(entry_date);
CREATE INDEX idx_accounting_entries_status ON accounting.journal_entries(status);
CREATE INDEX idx_accounting_entries_journal ON accounting.journal_entries(journal_code);
CREATE INDEX idx_accounting_lines_entry ON accounting.journal_lines(entry_id);
CREATE INDEX idx_accounting_lines_account ON accounting.journal_lines(account_code);
CREATE INDEX idx_accounting_ecl_loan ON accounting.ecl_staging(loan_id);
CREATE INDEX idx_accounting_ecl_stage ON accounting.ecl_staging(stage);
