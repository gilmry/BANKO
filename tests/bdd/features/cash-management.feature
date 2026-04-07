Feature: Cash Management (P1-BC3)
  As a treasury officer
  I want to optimize liquidity and manage cash across accounts and pools
  So that the bank maintains optimal liquidity while minimizing idle cash

  Background:
    Given the system is initialized
    And I am authenticated as "treasury_officer"
    And cash management module is operational

  # Sweep Accounts
  @critical @bc-cash-001 @sweep
  Scenario: Create zero-balance sweep account
    Given a corporate customer with id "cust-cash-001"
    And multiple operating accounts with varying balances
    When I create zero-balance sweep account:
      | Sweep Type | Zero Balance |
      | Sweep Frequency | Daily |
      | Sweep Time | 5:00 PM |
      | Master Account | ACC-MASTER-001 (collects surplus) |
      | Operating Accounts | ACC-OP-001, ACC-OP-002, ACC-OP-003 |
    Then sweep account structure is established with code "SWEEP-ZB-001"
    And daily sweep trigger is configured
    And system begins monitoring account balances

  @high @bc-cash-002 @sweep
  Scenario: Execute daily zero-balance sweep
    Given zero-balance sweep configured on master + 3 operating accounts
    And balances at end of day:
      | Account | Balance |
      | ACC-OP-001 | 45000 TND |
      | ACC-OP-002 | 32000 TND |
      | ACC-OP-003 | 18000 TND |
    When daily sweep is executed at 5:00 PM
    Then sweep is processed:
      | Total Swept | 95000 TND |
      | Operating Accounts Balance | 0 TND each (after sweep) |
      | Master Account Receives | 95000 TND |
      | Sweep Fee | 500 TND |
    And sweep log records all transactions
    And ACH entries are posted overnight

  @high @bc-cash-003 @sweep
  Scenario: Create target-balance sweep account
    Given corporate customer with variable cash needs
    When I create target-balance sweep account:
      | Sweep Type | Target Balance |
      | Target Balance Amount | 50000 TND |
      | Operating Account | ACC-OPERATING-001 |
      | Reserve Account | ACC-RESERVE-001 |
      | Sweep Direction | Bidirectional (in/out) |
    Then sweep maintains minimum 50000 TND in operating account
    And excess automatically sweeps to reserve
    And shortfall automatically transfers from reserve

  @high @bc-cash-004 @sweep
  Scenario: Handle sweep reversal for business needs
    Given sweep account with daily sweep of "95000 TND"
    When urgent customer need arises for "30000 TND"
    And customer requests reversal before sweep settlement
    Then sweep reversal is processed:
      | Original Sweep Amount | 95000 TND |
      | Reversal Amount | 30000 TND |
      | Net Sweep | 65000 TND |
      | Operating Account Balance | 30000 TND |
    And audit log records the reversal with reason

  # Cash Pools
  @critical @bc-cash-005 @pool
  Scenario: Establish corporate cash pool
    Given 5 group companies needing liquidity optimization
    When I establish cash pool:
      | Pool Name | GroupCorp Cash Pool 2026 |
      | Pool Type | Notional pool |
      | Administrator | GroupCorp Finance Ltd |
      | Participants | 5 companies |
      | Currency | TND |
      | Master Account | ACC-POOL-MASTER-001 |
      | Interest Rate | 4.5% p.a. on daily balances |
      | Distribution Method | Pro-rata by daily balance |
    Then cash pool is registered with code "POOL-2026-001"
    And participation agreement is signed by all parties
    And pool terms and conditions are documented

  @high @bc-cash-006 @pool
  Scenario: Manage cash pool participant entry
    Given established cash pool "POOL-2026-001"
    And new subsidiary company joining
    When new participant joins on "2026-04-07":
      | Participant Name | NewCorp Ltd |
      | Initial Balance | 100000 TND |
      | Pool Share % | 15% (pro-rata) |
      | Interest Bearing | Yes |
    Then participant account is activated
    And initial balance is transferred to pool
    And participant receives participation statement
    And interest accrual begins

  @high @bc-cash-007 @pool
  Scenario: Calculate and distribute pool interest
    Given cash pool with 5 participants and daily balances
    When monthly interest distribution occurs for March 2026:
      | Pool Interest Rate | 4.5% p.a. |
      | Days in Month | 31 |
      | Total Daily Balances | 50000000 TND |
      | Monthly Interest Earned | 151438 TND |
    Then interest is distributed pro-rata:
      | Company A | Balance 35% | Interest 53003 TND |
      | Company B | Balance 25% | Interest 37860 TND |
      | Company C | Balance 20% | Interest 30288 TND |
      | Company D | Balance 15% | Interest 22716 TND |
      | Company E | Balance 5% | Interest 7572 TND |
    And distribution statement is sent to each participant
    And funds are credited to respective accounts

  @high @bc-cash-008 @pool
  Scenario: Handle participant withdrawal from pool
    Given participant "Company C" in pool with balance "10000000 TND"
    When Company C requests exit on "2026-04-07"
    Then withdrawal is processed:
      | Notice Period | 5 business days (standard) |
      | Settlement Date | 2026-04-14 |
      | Final Balance | 10002500 TND (includes accrued interest) |
      | Final Interest | 2500 TND |
    And participant receives final settlement
    And participant account is closed
    And pool rebalances for remaining 4 participants

  @high @bc-cash-009 @pool
  Scenario: Manage pool overdraft facility
    Given cash pool with overdraft limit "2000000 TND"
    When one participant temporarily needs liquidity:
      | Participant | Company D |
      | Overdraft Amount | 500000 TND |
      | Overdraft Rate | 6.5% p.a. |
    Then overdraft is provided:
      | Balance | -500000 TND |
      | Overdraft Interest Rate | 6.5% p.a. |
      | Daily Charge | 89.04 TND |
    And overdraft is monitored and reported daily
    And participant repays overdraft within agreed timeframe

  # Cash Forecasting
  @critical @bc-cash-010 @forecast
  Scenario: Create 30-day cash flow forecast
    Given account "ACC-MASTER-001" with transaction history
    When 30-day cash forecast is generated on "2026-04-07":
      | Forecast Type | Base case |
      | Method | Historical trend analysis |
      | Opening Balance | 450000 TND |
      | Projected Inflows (30 days) | 2500000 TND |
      | Projected Outflows (30 days) | 2300000 TND |
      | Projected Closing Balance | 650000 TND |
      | Confidence Level | 90% |
    Then forecast is recorded with code "FCT-2026-001"
    And daily forecast breakdown is provided
    And forecast assumptions are documented

  @high @bc-cash-011 @forecast
  Scenario: Monitor seasonal cash patterns
    Given account with 3 years of historical data
    When annual forecast is created with seasonal adjustments:
      | Q1 Seasonal Index | 0.95 (lower activity) |
      | Q2 Seasonal Index | 1.1 (peak activity) |
      | Q3 Seasonal Index | 1.05 |
      | Q4 Seasonal Index | 0.9 (year-end effects) |
    Then forecast incorporates seasonal patterns
    And management can plan for peak and trough periods
    And staffing and liquidity preparations are made

  @high @bc-cash-012 @forecast
  Scenario: Update forecast with variance analysis
    Given 30-day forecast issued on "2026-03-08"
    When actual results for March are known:
      | Projected Closing Balance | 650000 TND |
      | Actual Closing Balance | 620000 TND |
      | Variance | -30000 TND (-4.6%) |
      | Key Driver | Lower than expected receivables |
    Then variance is analyzed and documented
    And root causes are identified
    And forecast methodology is reviewed for adjustment
    And new forecast incorporates learnings

  @high @bc-cash-013 @forecast
  Scenario: Identify liquidity risk in forecast
    Given 7-day and 30-day forecasts
    When analysis reveals potential liquidity stress:
      | Day 5 Minimum Balance | 25000 TND (below 50000 TND target) |
      | Risk Level | Medium |
      | Trigger | Collection delays + Unplanned outflow |
    Then liquidity risk is flagged to management
    And contingency plans are reviewed
    And credit lines are on standby
    And customer collection efforts are prioritized

  # Liquidity Position Monitoring
  @critical @bc-cash-014 @liquidity
  Scenario: Monitor daily bank liquidity position
    Given all customer accounts and pool positions
    When daily liquidity position is calculated at 5:00 PM:
      | Cash and Equivalents | 5000000 TND |
      | Unencumbered Securities | 3000000 TND |
      | Committed Credit Lines (available) | 2000000 TND |
      | Total Available Liquidity | 10000000 TND |
      | Required Minimum (Regulatory) | 4000000 TND |
      | Liquidity Surplus | 6000000 TND |
      | Liquidity Coverage Ratio | 250% |
    Then position is recorded with code "LIQ-DAILY-2026-0407"
    And position status is "normal"
    And reporting is sent to treasury management

  @high @bc-cash-015 @liquidity
  Scenario: Monitor liquidity under stress scenario
    Given current liquidity position
    When stress test is run assuming:
      | Scenario | Moderate stress |
      | Large customer withdrawal | 1500000 TND |
      | Market volatility | Securities value down 10% |
      | Deposit outflows | 800000 TND |
    Then stressed liquidity position is calculated:
      | Stressed Available Liquidity | 6700000 TND |
      | Minimum Requirement (stress) | 5000000 TND |
      | Stressed Liquidity Coverage Ratio | 134% |
      | Status | Still above stress minimum |
    And stress test results are documented
    And management is informed of resilience

  @high @bc-cash-016 @liquidity
  Scenario: Raise liquidity alert for tight conditions
    Given liquidity position with surplus reducing
    When liquidity ratio approaches minimum threshold:
      | Current Liquidity Ratio | 105% (minimum 100%) |
      | Trend | Declining (3 consecutive days below normal) |
      | Projected Next Day | 98% (below minimum) |
    Then alert is triggered to treasury head
    And contingency funding plan is activated
    And borrowing options are evaluated
    And less critical outflows are deferred

  # Funding Strategies
  @critical @bc-cash-017 @funding
  Scenario: Approve annual funding strategy
    Given bank's capital and liquidity requirements
    When annual funding strategy is approved on "2026-04-07":
      | Strategy Type | Short/Medium term (12 months) |
      | Funding Goal | 50000000 TND |
      | Deposit Target | 60% (30000000 TND) |
      | Bond Issuance | 25% (12500000 TND) |
      | Market Operations | 15% (7500000 TND) |
      | Duration Target | 2.5 years |
      | Interest Rate Hedge | Yes (20% of liabilities) |
    Then strategy code is "FND-2026-ANN"
    And board approval is documented
    And implementation team is mobilized

  @high @bc-cash-018 @funding
  Scenario: Monitor funding strategy execution
    Given approved funding strategy with quarterly milestones
    When quarterly review occurs for Q1 2026:
      | Target for Q1 | 12500000 TND |
      | Deposits Raised | 9500000 TND (76%) |
      | Bonds Issued | 3000000 TND (100% quarterly target met) |
      | Total Raised | 12500000 TND |
      | Status | On track |
    Then execution is reported as on-track
    And deposit and bond mix is within tolerance bands
    And liquidity position is stable
    And Q2 planning continues per strategy

  @high @bc-cash-019 @funding
  Scenario: Adjust funding strategy mid-year
    Given market conditions change (rising rates)
    When strategy adjustment is needed:
      | Original Bond Issuance Plan | 25% at 5.5% |
      | New Market Conditions | Rates at 6.5% |
      | Adjustment | Increase deposit focus to 70%, reduce bonds to 15% |
      | Reason | Manage cost of funds in rising rate environment |
    Then strategy is updated with board approval
    And implementation plan is revised
    And funding team executes adjusted plan
    And stakeholders are communicated

  @high @bc-cash-020 @funding
  Scenario: Execute contingency funding plan
    Given contingency funding plan on file for stress scenarios
    When major funding disruption occurs:
      | Trigger Event | Large customer withdrawal + market stress |
      | Funding Gap | 5000000 TND |
      | Contingency Plan | Activated |
    Then contingency funding is 