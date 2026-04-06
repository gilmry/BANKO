-- Reporting bounded context schema (BC8)

CREATE SCHEMA IF NOT EXISTS reporting;

CREATE TABLE reporting.report_templates (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    report_type VARCHAR(20) NOT NULL,
    version VARCHAR(20) NOT NULL,
    definition JSONB NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE reporting.regulatory_reports (
    id UUID PRIMARY KEY,
    report_type VARCHAR(20) NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    template_id UUID NOT NULL REFERENCES reporting.report_templates(id),
    template_version VARCHAR(20) NOT NULL,
    data JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft',
    generated_by UUID NOT NULL,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_at TIMESTAMPTZ,
    acknowledged_at TIMESTAMPTZ,
    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed BCT report templates
INSERT INTO reporting.report_templates (id, name, report_type, version, definition) VALUES
(gen_random_uuid(), 'Rapport Hebdomadaire BCT', 'Weekly', '1.0', '{"sections": ["deposits", "credits", "liquidity"]}'),
(gen_random_uuid(), 'Rapport Mensuel BCT', 'Monthly', '1.0', '{"sections": ["balance_sheet", "income", "ratios", "provisions"]}'),
(gen_random_uuid(), 'Rapport Trimestriel BCT', 'Quarterly', '1.0', '{"sections": ["prudential_ratios", "risk_concentration", "aml_summary", "provisions"]}'),
(gen_random_uuid(), 'Rapport Annuel BCT', 'Annual', '1.0', '{"sections": ["full_financials", "audit", "governance", "risk_management"]}');

CREATE INDEX idx_reports_type ON reporting.regulatory_reports(report_type);
CREATE INDEX idx_reports_status ON reporting.regulatory_reports(status);
CREATE INDEX idx_reports_period ON reporting.regulatory_reports(period_start, period_end);
CREATE INDEX idx_templates_type ON reporting.report_templates(report_type);
