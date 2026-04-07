Feature: Advanced Reporting (BC8) - Scheduled Reports, XBRL, BCT, IFRS9, Tax Reports
  As a reporting analyst
  I want to manage scheduled reports, XBRL exports, regulatory filings, and financial reporting
  So that the bank meets reporting requirements and provides stakeholder transparency

  Background:
    Given the system is initialized
    And I am authenticated as "reporting_analyst"
    And reporting module is operational

  # Scheduled Report Generation
  @critical @fr-scheduled-001 @reporting @scheduling
  Scenario: Create scheduled monthly financial report
    Given no scheduled reports configured
    When setting up monthly financial report
    Then report configuration:
      | Parameter | Value |
      | Report name | Monthly Financial Summary |
      | Schedule | Monthly (1st day of month, 08:00 UTC) |
      | Recipients | CFO, Controller, Board (3 recipients) |
      | Format | PDF + Excel attachment |
      | Data scope | Consolidated financials + by-department breakdown |
      | Retention | 7 years (archival) |
    And report is scheduled
    And first report will run 2025-05-01

  @critical @fr-scheduled-002 @reporting @scheduling
  Scenario: Execute scheduled report and distribute
    Given scheduled monthly report due
    When report execution time arrives
    Then automatic process:
      | Step | Timing |
      | 1. Data extraction | Pull financials from GL |
      | 2. Calculations | Compute ratios, variances, KPIs |
      | 3. Report generation | Format report per template |
      | 4. QA check | Validate completeness + data quality |
      | 5. Distribution | Email to recipients |
      | 6. Archival | Store in reporting system |
    And report completes in < 5 minutes
    And recipients receive notification + download link

  @high @fr-scheduled-003 @reporting @scheduling
  Scenario: Configure conditional report triggers
    Given conditional reporting rule
    When defining trigger conditions
    Then conditions include:
      | Condition | Action |
      | If NPL ratio > 5% | Trigger immediate risk report |
      | If CAR < 11% | Generate capital adequacy alert |
      | If daily trading loss > 1M TND | Send exception report to CEO |
      | If SARs > 20 in a month | File monthly AML summary |
    And conditions are monitored continuously
    And reports generated on-demand when triggered

  @high @fr-scheduled-004 @reporting @scheduling
  Scenario: Manage report delivery and notifications
    Given scheduled report with multiple recipients
    When report is generated and ready
    Then delivery:
      | Element |
      | Email sent to all recipients with PDF attachment |
      | Web portal link provided (download valid for 30 days) |
      | SMS notification to key stakeholders |
      | Report arrival confirmed in audit log |
      | Delivery failure automatically retried (3x) |
    And failed deliveries are escalated
    And reports can be accessed from web portal 30 days

  @medium @fr-scheduled-005 @reporting @scheduling
  Scenario: Archive and retrieve historical reports
    Given 12 months of monthly financial reports
    When accessing historical reports
    Then retrieval features:
      | Feature |
      | Browse by month/year |
      | Search by report name or keyword |
      | View report in web viewer |
      | Download original format (PDF/Excel) |
      | Compare current vs. prior period |
      | Generate trend analysis |
    And archival guarantees data integrity
    And archive is immutable (audit trail only)

  # XBRL (eXtensible Business Reporting Language) Reporting
  @critical @fr-xbrl-001 @reporting @xbrl
  Scenario: Export financial data in XBRL format
    Given monthly consolidated financial statements
    When exporting to XBRL format
    Then XBRL export includes:
      | Component |
      | Instance document (financial facts tagged with GL) |
      | GL mapping to XBRL taxonomy (banking domain) |
      | Context entities (consolidated, by-entity, by-product) |
      | Units (TND, percentages, counts) |
      | Dimensions (time, currency, business line) |
    And XBRL file is validated against schema
    And export completes without errors

  @critical @fr-xbrl-002 @reporting @xbrl
  Scenario: Validate XBRL document conformance
    Given generated XBRL file
    When validating conformance
    Then validation checks:
      | Check |
      | Schema conformance (banking taxonomy) |
      | Mandatory elements presence |
      | Data type validation (amounts, dates) |
      | Dimensional consistency |
      | Context references validity |
      | No duplicate facts |
      | No inconsistent units |
    And validation report is generated
    And errors must be resolved before filing

  @high @fr-xbrl-003 @reporting @xbrl
  Scenario: File XBRL report with regulator
    Given validated XBRL file
    When submitting to BCT XBRL platform
    Then filing process:
      | Step | Details |
      | 1. System authentication | API credentials verified |
      | 2. File upload | XBRL file transmitted securely |
      | 3. Receipt confirmation | BCT provides submission reference |
      | 4. Validation | BCT validates file conformance |
      | 5. Acknowledgment | Email confirmation within 24 hours |
    And filing timestamp is recorded
    And submission ID is used for follow-up

  # BCT Regulatory Reporting (Financial Institution Reporting)
  @critical @fr-bct-001 @reporting @regulatory
  Scenario: Prepare BCT financial return (COREP)
    Given monthly period-end data
    When preparing COREP return
    Then COREP includes:
      | Report Section |
      | Capital (Common Equity, Tier1, Tier2) |
      | Risk-Weighted Assets (by risk type) |
      | Leverage ratio |
      | Liquidity Coverage Ratio |
      | Net Stable Funding Ratio |
      | Large exposures |
      | Counterparty concentrations |
    And report is compiled in BCT-specified format

  @critical @fr-bct-002 @reporting @regulatory
  Scenario: File BCT FINREP (Financial Statement Report)
    Given consolidated balance sheet and P&L
    When preparing FINREP
    Then FINREP submission:
      | Section |
      | Balance sheet with full GL details |
      | P&L with income/expense breakdown |
      | Asset quality (performing vs. non-performing) |
      | Provisions (general and specific) |
      | Equity composition |
      | Related party transactions |
    And filing is in XBRL format per BCT requirements

  @high @fr-bct-003 @reporting @regulatory
  Scenario: Submit timely regulatory reports to BCT
    Given multiple reports due (COREP, FINREP, prudential)
    When filing deadline approaches (10 business days)
    Then filing coordination:
      | Step | Timeline |
      | 1. Data freeze | T-5 days (no post-close adjustments) |
      | 2. Report preparation | T-3 days (all reports compiled) |
      | 3. Review | T-2 days (CFO review and sign-off) |
      | 4. Submission | T-1 day (file with BCT) |
      | 5. Receipt confirmation | T (deadline) - within submission window |
    And reports filed timely, no late submissions
    And filing compliance is tracked

  # IFRS 9 Reporting (Financial Instruments - Expected Credit Loss)
  @critical @fr-ifrs9-001 @reporting @ifrs9
  Scenario: Calculate IFRS 9 Expected Credit Loss (ECL) staging
    Given loan portfolio with staging model
    When classifying loans per IFRS 9 stages
    Then staging includes:
      | Stage | Criteria | Classification |
      | Stage 1 | No significant increase in credit risk | ECL = 12-month PD |
      | Stage 2 | Significant increase in credit risk | ECL = Lifetime PD |
      | Stage 3 (Default) | Objective evidence of impairment | ECL = Lifetime PD + illiquidity margin |
    And loan is migrated between stages per risk changes
    And stage transitions are documented

  @critical @fr-ifrs9-002 @reporting @ifrs9
  Scenario: Determine 12-month vs. Lifetime Probability of Default
    Given customer credit profile
    When calculating PD:
      | Metric | Value |
      | Customer risk rating | BBB (investment grade) |
      | Historical default rate (rating class) | 0.5% per annum |
      | 12-month PD (Stage 1) | 0.5% |
      | Lifetime PD (Stage 2/3, 5-year horizon) | 2.1% |
    And PD is calibrated to through-the-cycle models
    And PD is applied to ECL calculation

  @high @fr-ifrs9-003 @reporting @ifrs9
  Scenario: Calculate Loss Given Default (LGD) and Exposure at Default
    Given loan collateral value and terms
    When determining LGD
    Then LGD calculation:
      | Component | Value |
      | Loan outstanding | TND 100,000 |
      | Collateral value (current appraisal) | TND 65,000 |
      | Recovery rate | (65K / 100K) = 65% |
      | Loss Given Default (LGD) | 1 - 65% = 35% |
      | Exposure at Default (EAD) | TND 100,000 |
    And LGD factors in recovery costs, illiquidity
    And EAD includes drawn + undrawn commitments

  @high @fr-ifrs9-004 @reporting @ifrs9
  Scenario: Calculate and record IFRS 9 ECL provisions
    Given loan portfolio with 1,000 loans
    When calculating total ECL:
      | Portfolio Segment | Count | Avg Exposure | PD | LGD | ECL per Loan | Total ECL |
      | Investment Grade | 700 | TND 50,000 | 0.5% | 30% | TND 75 | TND 52.5M |
      | Sub-Investment Grade | 250 | TND 150,000 | 3.0% | 40% | TND 1,800 | TND 450M |
      | High Risk | 50 | TND 200,000 | 15.0% | 60% | TND 18,000 | TND 900M |
    Then total ECL = TND 1,402.5M (millions)
    And provision is recorded in P&L as "ECL Expense"
    And balance sheet shows "Loan Loss Provision - IFRS9"

  # Tax Reporting
  @critical @fr-tax-001 @reporting @tax
  Scenario: Calculate corporate income tax provision
    Given annual profit before tax
    When calculating tax provision
    Then calculation:
      | Component | Amount |
      | Profit before tax | TND 500M |
      | Statutory tax rate | 25% |
      | Temporary differences | (TND 50M) |
      | Permanent differences | TND 20M |
      | Taxable income | TND 470M |
      | Estimated tax | TND 117.5M |
      | Tax rate (effective) | 23.5% |
    And tax provision is recorded in P&L
    And reconciliation of book vs. tax prepared

  @high @fr-tax-002 @reporting @tax
  Scenario: File quarterly tax returns with tax authorities
    Given quarterly taxable income calculations
    When preparing quarterly tax return
    Then filing includes:
      | Section |
      | Income from operations |
      | Income from investments |
      | Deductions (operating expenses, depreciation) |
      | Tax credits applied |
      | Estimated tax due |
    And return filed within 20 days of quarter-end
    And payment submitted if tax due

  @high @fr-tax-003 @reporting @tax
  Scenario: Track deferred tax assets and liabilities
    Given temporary differences between book and tax
    When measuring deferred tax position
    Then tracking:
      | Temporary Difference | Book Amount | Tax Amount | Deferred Tax Asset/Liability |
      | Provision for doubtful debts | TND 100M | TND 50M | DTA: TND 12.5M |
      | Depreciation timing | (TND 40M) | (TND 60M) | DTL: TND 5M |
      | Accrued bonuses | TND 80M | TND 0 | DTA: TND 20M |
    And deferred tax is recognized on B/S
    And recalculated annually

  # Stakeholder Reporting
  @high @fr-stakeholder-001 @reporting @stakeholder
  Scenario: Generate annual report for shareholders
    Given fiscal year-end financials and business review
    When preparing annual report
    Then annual report includes:
      | Section |
      | Chairman's statement (business outlook) |
      | Financial highlights (5-year summary) |
      | Management discussion & analysis (MD&A) |
      | Financial statements (audited) |
      | Notes to financial statements |
      | Corporate governance report |
      | Risk management framework |
      | Sustainability/CSR initiatives |
      | Board and management profiles |
    And report is published by shareholders' meeting date

  @high @fr-stakeholder-002 @reporting @stakeholder
  Scenario: Publish audited financial statements
    Given year-end financials
    When releasing audited statements
    Then statements include:
      | Statement |
      | Auditor's opinion (unqualified preferred) |
      | Balance sheet |
      | Profit & loss statement |
      | Cash flow statement |
      | Statement of changes in equity |
      | Notes to financial statements (50+ pages) |
    And statements filed with stock exchange (if listed)
    And press release issued
    And investor call held

  # Report Distribution and Access Control
  @high @fr-distribution-001 @reporting @distribution
  Scenario: Control access to confidential reports
    Given sensitive reports (audit findings, risk assessments)
    When managing report access
    Then access controls:
      | Report Type | Access Level | Recipients |
      | Internal Audit Report | Confidential | Board, Audit Committee only |
      | Risk Assessment | Internal Use | Risk Committee + C-suite |
      | Regulatory Filing | Public (if required) | Filed publicly per regulations |
      | Proprietary Analysis | Restricted | Specific departments only |
    And access logging tracks who viewed report
    And digital watermarking identifies recipient

  @high @fr-distribution-002 @reporting @distribution
  Scenario: Manage report versioning and updates
    Given report with discovered errors post-release
    When reissuing corrected report
    Then versioning:
      | Version | Status | Date | Change |
      | v1.0 | Superseded | 2025-03-01 | Original release - had error |
      | v1.1 | Current | 2025-03-05 | Corrected calculation (page 5) |
    And both versions tracked in system
    And recipients notified of correction
    And v1.0 marked as "Superseded - See v1.1"

  # Reporting Analytics and Dashboards
  @high @fr-analytics-001 @reporting @analytics
  Scenario: Create financial KPI dashboard
    Given historical financial data
    When building KPI dashboard
    Then dashboard displays:
      | KPI | Latest | Target | Trend |
      | ROE | 8.5% | 10.0% | Up (monthly average) |
      | CAR | 13.2% | >10.5% | Stable |
      | NPL Ratio | 4.2% | <5.0% | Down |
      | Cost-to-Income | 52% | <50% | Down |
      | Loan-to-Deposit | 82% | 75-85% | In range |
    And dashboard is real-time updated
    And alerts for KPIs out of range

  @medium @fr-analytics-002 @reporting @analytics
  Scenario: Generate trend and variance analysis
    Given monthly P&L for 12 months
    When analyzing trends
    Then analysis includes:
      | Analysis | Finding |
      | YoY growth | Net income up 12% vs. prior year |
      | Month-over-month | March income down 8% vs. February |
      | Budget variance | Operating expenses 5% over budget |
      | Forecast | Q2 income projected TND 125M (vs. budget 120M) |
    And charts and commentary provided
    And outliers highlighted for investigation

  # Report Quality and Validation
  @high @fr-quality-001 @reporting @quality
  Scenario: Implement report data quality checks
    Given report data ready
    When executing quality validation
    Then validation checks:
      | Check |
      | Balance sheet balances (Assets = Liabilities + Equity) |
      | P&L sub-totals reconcile |
      | Cash flow: Operating + Investing + Financing = Change in Cash |
      | GL reconciliations complete |
      | No missing required fields |
      | Data is within expected ranges (no outliers) |
      | Cross-module reconciliations (e.g., GL vs. Subledgers) |
    And report cannot be finalized until all checks pass

  @high @fr-quality-002 @reporting @quality
  Scenario: Segregate report approval workflows
    Given completed report ready for release
    When implementing approval hierarchy
    Then approval workflow:
      | Step | Authority | Verification |
      | 1. Preparer sign-off | Reporting Analyst | Data accuracy |
      | 2. Supervisor review | Reporting Manager | Completeness, format |
      | 3. Financial Controller | Controller | Overall accuracy, compliance |
      | 4. CFO approval | CFO | Strategic alignment, board readiness |
      | 5. Release | Communications | Public/regulatory filing |
    And each approval documented with timestamp
    And approval cannot be skipped
