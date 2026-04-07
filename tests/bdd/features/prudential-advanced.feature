Feature: Prudential Capital Management (BC6) - Basel III, LCR, NSFR, ICAAP
  As a prudential officer
  I want to manage capital adequacy, liquidity ratios, and regulatory capital requirements
  So that the bank maintains Basel III compliance and adequate capital buffers

  Background:
    Given the system is initialized
    And I am authenticated as "prudential_officer"
    And prudential calculations are enabled

  # Capital Adequacy Ratio (CAR) - Basel III
  @critical @fr-basel-001 @prudential @capital
  Scenario: Calculate Total Capital Ratio
    Given period-end position with:
      | Metric | Value |
      | Tier1Capital | 50,000,000 |
      | Tier2Capital | 20,000,000 |
      | RiskWeightedAssets | 500,000,000 |
    When calculating Total Capital Ratio
    Then Total Capital Ratio = (50M+20M)/500M = 14%
    And ratio meets Basel III minimum of 10.5%
    And compliance status is "Compliant"

  @critical @fr-basel-002 @prudential @capital
  Scenario: Monitor Tier 1 Capital Ratio
    Given Tier1 Capital of 50,000,000 and RWA of 400,000,000
    When calculating Tier1 Capital Ratio
    Then Tier1 Ratio = 50M/400M = 12.5%
    And ratio exceeds Basel III minimum of 8.5%
    And buffer above minimum is 4%

  @high @fr-basel-003 @prudential @capital
  Scenario: Alert on capital ratio deterioration
    Given historical Tier1 Capital Ratio of 12.5%
    When Tier1 Capital Ratio drops to 8.6% (below 8.5% minimum)
    Then alert is triggered with severity "Critical"
    And board is notified immediately
    And remediation plan must be filed
    And ratio monitoring frequency increases to daily

  @high @fr-basel-004 @prudential @capital
  Scenario: Calculate Common Equity Tier 1 (CET1) Ratio
    Given:
      | Component | Value |
      | Common Stock | 30,000,000 |
      | Retained Earnings | 20,000,000 |
      | Regulatory Adjustments | (5,000,000) |
      | RWA | 450,000,000 |
    When calculating CET1 Ratio
    Then CET1 = (30M + 20M - 5M) / 450M = 11.1%
    And meets Basel III CET1 minimum of 7.5%

  # Risk-Weighted Assets (RWA) Calculation
  @critical @fr-rwa-001 @prudential @risk
  Scenario: Calculate RWA for credit risk exposures
    Given portfolio with:
      | Asset Type | Balance | Risk Weight |
      | Government Bonds | 100,000,000 | 0% |
      | Corporate Loans | 150,000,000 | 100% |
      | Retail Mortgages | 80,000,000 | 35% |
      | Other Assets | 20,000,000 | 50% |
    When calculating total RWA
    Then RWA = 0 + 150M + 28M + 10M = 188,000,000
    And total assets = 350,000,000
    And average risk weight = 53.7%

  @high @fr-rwa-002 @prudential @risk
  Scenario: Apply standardized approach for RWA
    Given corporate loan of 1,000,000 TND with counterparty rating A
    When applying Standardized Approach
    Then risk weight = 50% (for A-rated corporate)
    And RWA contribution = 500,000
    And exposure is recorded by rating class

  @high @fr-rwa-003 @prudential @risk
  Scenario: Reweight assets when credit rating changes
    Given loan with current risk weight 50% (A-rated)
    When credit rating downgraded to B-
    Then new risk weight = 100%
    And RWA impact = additional 500,000
    And adjustment is processed T+1
    And report shows rating migration

  @medium @fr-rwa-004 @prudential @risk
  Scenario: Calculate RWA for market risk
    Given FX position of 10,000,000 USD with EUR/USD = 0.92
    When calculating market risk RWA
    Then Value at Risk (VaR) at 99% confidence is calculated
    And RWA is set to VaR × 12.5 multiplier
    And position limits are enforced

  # Liquidity Coverage Ratio (LCR)
  @critical @fr-lcr-001 @prudential @liquidity
  Scenario: Calculate Liquidity Coverage Ratio
    Given 30-day stress scenario with:
      | High Quality Liquid Assets (HQLA) | 50,000,000 |
      | Total Net Cash Outflow (TNCO) | 40,000,000 |
    When calculating LCR
    Then LCR = 50M / 40M = 125%
    And meets Basel III minimum of 100%
    And surplus HQLA = 10,000,000

  @critical @fr-lcr-002 @prudential @liquidity
  Scenario: LCR alert on insufficient coverage
    Given LCR = 95% (below 100% minimum)
    When monitoring LCR daily
    Then alert is triggered with severity "High"
    And funding plan must be executed
    And asset sales or deposit campaigns required
    And board report is generated

  @high @fr-lcr-003 @prudential @liquidity
  Scenario: Classify HQLA by level and haircut
    Given liquidity buffer with:
      | Asset Type | Amount | Level | Haircut |
      | Sovereign Bonds (A+) | 20,000,000 | 1 | 0% |
      | Corporate Bonds (AAA) | 15,000,000 | 2a | 15% |
      | Residential Mortgage-Backed (AAA) | 10,000,000 | 2b | 25% |
    When applying haircuts
    Then HQLA Level 1 = 20M
    And HQLA Level 2a = 12.75M (15M × 85%)
    And HQLA Level 2b = 7.5M (10M × 75%)
    And total HQLA = 40.25M

  @high @fr-lcr-004 @prudential @liquidity
  Scenario: Model stress outflows for deposit customers
    Given deposit base of 200,000,000 TND:
      | Customer Type | Balance | Runoff Rate |
      | Demand Deposits (Retail) | 50,000,000 | 10% |
      | Stable Deposits (Term) | 80,000,000 | 5% |
      | Deposits (Large Corp) | 70,000,000 | 25% |
    When calculating 30-day outflows
    Then Retail outflow = 50M × 10% = 5M
    And Stable outflow = 80M × 5% = 4M
    And Corporate outflow = 70M × 25% = 17.5M
    And total outflow = 26.5M

  # Net Stable Funding Ratio (NSFR)
  @critical @fr-nsfr-001 @prudential @liquidity
  Scenario: Calculate Net Stable Funding Ratio
    Given funding and asset structure:
      | Available Stable Funding (ASF) | 150,000,000 |
      | Required Stable Funding (RSF) | 120,000,000 |
    When calculating NSFR
    Then NSFR = 150M / 120M = 125%
    And meets Basel III minimum of 100%
    And adequately funded for 1-year horizon

  @high @fr-nsfr-002 @prudential @liquidity
  Scenario: Monitor NSFR deterioration trend
    Given NSFR history:
      | Month | NSFR |
      | January | 130% |
      | February | 125% |
      | March | 118% |
      | April | 115% |
    When trending NSFR
    Then declining trend identified (average decline 5% per month)
    And 3-month projection shows NSFR = 105% (breaching minimum)
    And action plan is required

  @high @fr-nsfr-003 @prudential @liquidity
  Scenario: Calculate ASF weights for different liabilities
    Given liability structure:
      | Liability Type | Balance | ASF Weight |
      | Deposits (insured, stable) | 80,000,000 | 100% |
      | Deposits (insured, less stable) | 40,000,000 | 90% |
      | Deposits (uninsured, large) | 20,000,000 | 50% |
      | Wholesale Funding (< 1y) | 10,000,000 | 0% |
    When calculating ASF
    Then ASF = (80M×100%) + (40M×90%) + (20M×50%) + (10M×0%) = 155M

  # Leverage Ratio
  @critical @fr-leverage-001 @prudential @capital
  Scenario: Calculate Basel III Leverage Ratio
    Given:
      | Tier1 Capital | 50,000,000 |
      | Total Exposures (not risk-weighted) | 600,000,000 |
    When calculating Leverage Ratio
    Then Leverage Ratio = 50M / 600M = 8.33%
    And meets Basel III minimum of 3%
    And buffer is 5.33%

  @high @fr-leverage-002 @prudential @capital
  Scenario: Monitor leverage ratio for single large exposure
    Given single exposure to customer = 50,000,000 (8.3% of assets)
    When leverage ratio drops to 3.2% (approaching minimum)
    Then large exposure limits are enforced
    And new large exposures > 25M require board approval
    And concentration risk report is generated

  # Capital Buffers (Pillar 2)
  @critical @fr-buffer-001 @prudential @capital
  Scenario: Determine Pillar 2 Capital Requirement (P2R)
    Given supervisory assessment:
      | Risk | Multiplier | RWA Impact |
      | Credit Risk | 1.2x | 20% higher RWA |
      | Operational Risk | 1.5x | 15% higher RWA |
      | Interest Rate Risk (IRRBB) | 1.1x | 5% higher RWA |
    When calculating P2R
    Then base RWA = 500M
    And adjusted RWA = 500M × (1.2 × 1.5 × 1.1) ÷ 3 = 550M
    And P2R capital requirement = 55M (10% of 550M)

  @critical @fr-buffer-002 @prudential @capital
  Scenario: Maintain Capital Conservation Buffer (2.5% of RWA)
    Given RWA of 500,000,000
    When calculating Conservation Buffer requirement
    Then minimum buffer = 500M × 2.5% = 12.5M
    And if capital falls below buffer, dividend restrictions apply
    And board is notified immediately

  @high @fr-buffer-003 @prudential @capital
  Scenario: Apply Countercyclical Buffer (varies 0%-2.5%)
    Given regulatory environment signals credit boom
    When supervisor increases CCyB from 0% to 1.5%
    Then additional capital requirement = RWA × 1.5% = 7.5M
    And requirement becomes effective in next quarter
    And bank must adjust capital planning

  # Stress Testing
  @critical @fr-stress-001 @prudential @stress
  Scenario: Conduct quarterly stress test under adverse scenario
    Given adverse economic scenario:
      | Parameter | Change |
      | GDP growth | -2% |
      | Unemployment | +3% |
      | Default Rate | +5% |
      | Interest Rates | +200 bps |
    When running stress test
    Then credit losses calculated = estimated 25,000,000
    And interest income impact = (18,000,000)
    And stressed CAR = 8.2% (from 14%)
    And stress test result: "Acceptable with monitoring"

  @high @fr-stress-002 @prudential @stress
  Scenario: Validate stress test assumptions and models
    Given stress test model for credit losses
    When validating model accuracy
    Then backtesting against actual losses shows 85% accuracy
    And model assumptions are appropriate
    And documentation is complete

  @high @fr-stress-003 @prudential @stress
  Scenario: Report stress test results to regulator
    Given completed quarterly stress test
    When preparing regulatory report
    Then results are submitted to BCT within 15 days
    And detailed methodology is included
    And comparison with previous quarters is provided
    And board certification is attached

  # Internal Capital Adequacy Assessment Process (ICAAP)
  @critical @fr-icaap-001 @prudential @icaap
  Scenario: Develop ICAAP Framework
    Given no internal capital assessment process
    When establishing ICAAP
    Then framework includes:
      | Component |
      | Risk identification |
      | Capital needs assessment |
      | Stress testing |
      | Risk appetite statement |
      | Capital planning |
    And documentation is comprehensive
    And board governance is defined

  @critical @fr-icaap-002 @prudential @icaap
  Scenario: Perform ICAAP assessment of capital adequacy
    Given ICAAP framework in place
    When assessing internal capital needs
    Then each risk category is evaluated:
      | Risk | Internal CAR |
      | Credit Risk | 7.5% |
      | Market Risk | 1.2% |
      | Operational Risk | 2.5% |
      | Interest Rate Risk | 0.8% |
    And total internal CAR = 12%
    And compared to regulatory minimum of 10.5%
    And buffer of 1.5% identified

  @high @fr-icaap-003 @prudential @icaap
  Scenario: Establish internal risk appetite
    Given ICAAP assessment complete
    When defining risk appetite statement
    Then risk limits are set for:
      | Risk Type | Limit |
      | NPL Ratio | 3.5% |
      | Loan Loss Reserve / Total Loans | 2.0% |
      | Large Exposure / Capital | 25% |
      | Dividend Payout Ratio | 30% |
    And limits are monitored monthly
    And breaches trigger escalation

  @high @fr-icaap-004 @prudential @icaap
  Scenario: Capital planning under stress and normal scenarios
    Given ICAAP assessment with internal CAR of 12%
    When creating 3-year capital plan
    Then scenarios include:
      | Scenario | Projected CAR Y3 |
      | Base Case | 13.2% |
      | Stress Case | 9.8% |
      | Adverse Case | 7.2% |
    And mitigation actions defined for Stress/Adverse
    And capital raising or expense reduction planned

  # Recovery Plan (Pillar 2 Guidance - BCBS 255)
  @critical @fr-recovery-001 @prudential @recovery
  Scenario: Develop Bank Recovery Plan
    Given regulatory requirement for recovery plan
    When establishing recovery framework
    Then plan includes:
      | Trigger | Recovery Option |
      | CAR < 12% | Reduce dividend by 50% |
      | NPL > 5% | Restrict new lending |
      | CAR < 10% | Asset sales (TND 20M) |
      | Capital < 8% | Capital raise (TND 30M) |
    And triggers are monitored daily
    And plan is tested annually

  @high @fr-recovery-002 @prudential @recovery
  Scenario: Execute recovery measure when trigger breached
    Given CAR = 11.8% (below 12% trigger)
    When recovery plan is activated
    Then dividend reduction to 50% is implemented
    And board approves deferral mechanism
    And impact statement is generated
    And stakeholders are informed

  # Basel III Monitoring and Reporting
  @high @fr-basel-report-001 @prudential @reporting
  Scenario: Generate monthly prudential reporting
    Given month-end close completed
    When generating prudential report
    Then report includes:
      | Metric | Value |
      | Total Capital Ratio | 14.0% |
      | Tier 1 Ratio | 12.5% |
      | CET1 Ratio | 11.1% |
      | Leverage Ratio | 8.3% |
      | LCR | 125% |
      | NSFR | 125% |
    And comparisons with limits are shown
    And trends are analyzed (3-month)

  @high @fr-basel-report-002 @prudential @reporting
  Scenario: Regulatory submission to Central Bank (BCT)
    Given monthly prudential metrics calculated
    When preparing BCT submission
    Then COREP report is filed in XML format
    And FINREP report with balance sheet detail is included
    And filing is transmitted within 10 business days
    And BCT acknowledgment is recorded

  @medium @fr-basel-report-003 @prudential @reporting
  Scenario: Disclose capital information to market
    Given quarterly results prepared
    When publishing capital disclosure
    Then public disclosure includes:
      | Item | Detail |
      | Capital Composition | Common Equity, Tier 1, Tier 2 |
      | Risk-Weighted Assets | by risk type (credit, market, operational) |
      | Capital Ratios | all regulatory ratios |
      | Capital Management | dividend policy, capital plans |
    And disclosure complies with pillar 3
