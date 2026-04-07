Feature: Securities Trading and Portfolio Management (P2-BC1)
  As a securities trader and portfolio manager
  I want to manage securities accounts, execute trades, and monitor portfolios
  So that customers can invest in securities with proper settlement and corporate action handling

  Background:
    Given the system is initialized
    And I am authenticated as "securities_trader"
    And securities module is operational

  # Securities Account Management
  @critical @bc-securities-001 @securities
  Scenario: Open a securities trading account
    Given a verified customer with id "cust-sec-001" and name "Fatima Ali"
    When I create a securities account with:
      | Account Type | Cash |
      | Currency | TND |
      | Custodian | BNP Paribas Securities Services |
      | Nominee | BANKO Nominee Ltd |
    Then securities account is created with code "SSEC-001"
    And account status is "active"
    And cash balance is initialized to 0 TND
    And statement frequency is set to "monthly"

  @high @bc-securities-002 @securities
  Scenario: Open a margin trading account with leverage
    Given a corporate customer with id "cust-marg-001"
    When I create a margin account with:
      | Account Type | Margin |
      | Max Leverage | 2.5x |
      | Initial Deposit | 100000 TND |
      | Margin Requirement | 40% |
    Then margin account is created with code "SMARG-001"
    And leverage ratio is "1.0x" (fully funded)
    And max leverage limit is enforced at 2.5x

  @high @bc-securities-003 @securities
  Scenario: Open a retirement investment account
    Given a customer planning for retirement
    When I create a retirement account with:
      | Account Type | Retirement |
      | Tax Status | Tax-advantaged |
      | Withdrawal Restrictions | Age 65+ |
    Then retirement account is created with code "SRET-001"
    And tax-advantaged status is flagged

  @medium @bc-securities-004 @securities
  Scenario: Deposit cash into securities account
    Given securities account "SSEC-001" with zero balance
    When customer deposits "50000 TND" via bank transfer
    Then cash balance increases to "50000 TND"
    And settled cash is "50000 TND"
    And deposit transaction is recorded

  @medium @bc-securities-005 @securities
  Scenario: Request cash withdrawal from securities account
    Given securities account "SSEC-001" with cash balance "50000 TND"
    When customer requests withdrawal of "25000 TND"
    Then withdrawal is processed within 2 business days
    And cash balance reduces to "25000 TND"
    And settlement status is tracked

  # Buy and Sell Orders
  @critical @bc-securities-006 @securities
  Scenario: Place a market buy order for equities
    Given securities account "SSEC-001" with cash balance "50000 TND"
    And equity security "Orange Tunisia" with ISIN "TN0000000000"
    When I place a buy order with:
      | Order Type | Market |
      | Quantity | 100 shares |
      | Security | TN0000000000 |
    Then order is created with code "ORD-2026-001"
    And order status is "pending"
    And order execution is triggered immediately

  @critical @bc-securities-007 @securities
  Scenario: Execute market buy order and update position
    Given pending buy order "ORD-2026-001" for 100 shares at market
    When order execution occurs at "25.50 TND" per share
    Then order status changes to "executed"
    And execution price is recorded as "25.50 TND"
    And total order value is "2550 TND" (100 × 25.50)
    And commission of "25.50 TND" (1%) is deducted
    And cash balance decreases to "47424.50 TND"
    And position created with 100 shares at "25.50 TND" cost basis

  @critical @bc-securities-008 @securities
  Scenario: Place a limit buy order
    Given securities account "SSEC-001"
    When I place a buy limit order with:
      | Quantity | 50 shares |
      | Security ISIN | TN0000000000 |
      | Limit Price | 24.00 TND |
      | Expiry | 30 days |
    Then order is created with code "ORD-2026-002"
    And order status is "pending"
    And order remains pending until price reaches "24.00 TND"

  @high @bc-securities-009 @securities
  Scenario: Execute partial fill on limit order
    Given limit order "ORD-2026-002" for 50 shares at 24.00 TND
    When 30 shares execute at 24.00 TND and 20 shares remain unfilled
    Then order status is "partially_executed"
    And execution count is 30 shares
    And remaining quantity is 20 shares
    And position updated with 30 shares added

  @high @bc-securities-010 @securities
  Scenario: Cancel an unfilled order
    Given buy order "ORD-2026-002" with 20 shares still pending
    When I cancel the remaining unfilled order
    Then order status changes to "cancelled"
    And remaining quantity is zeroed
    And cash is released back to account balance

  @high @bc-securities-011 @securities
  Scenario: Place a stop-loss sell order
    Given position with 100 shares of Orange Tunisia at "25.50 TND" cost basis
    And current market price is "27.00 TND"
    When I place a stop-loss order with:
      | Trigger Price | 26.00 TND |
      | Sell Quantity | 100 shares |
    Then stop-loss order is created with code "ORD-2026-003"
    And order remains pending until price falls to "26.00 TND"

  @high @bc-securities-012 @securities
  Scenario: Execute stop-loss order on price decline
    Given stop-loss order "ORD-2026-003" triggered at "26.00 TND"
    When market price falls to "26.00 TND"
    Then stop-loss order automatically converts to sell market order
    And sale executes at "25.95 TND"
    And unrealized loss of "155 TND" (100 × (26 - 25.50)) is realized
    And position is closed

  @high @bc-securities-013 @securities
  Scenario: Place a sell order for existing position
    Given position with 100 shares at "25.50 TND" cost basis
    And current market price is "26.50 TND"
    When I place a sell market order for 100 shares
    Then order executes at "26.50 TND" per share
    And sale proceeds are "2650 TND" (100 × 26.50)
    And commission of "26.50 TND" (1%) is deducted
    And unrealized gain of "100 TND" (100 × (26.50 - 25.50)) is realized
    And net proceeds are "2623.50 TND"
    And position is closed

  # T+2 Settlement
  @critical @bc-securities-014 @securities
  Scenario: Automatic T+2 settlement for buy trade
    Given executed buy order on trade date "2026-04-07" (Monday)
    When settlement date arrives on "2026-04-09" (Wednesday, T+2)
    Then settlement record is created with code "SETT-2026-001"
    And DVP status is "settled"
    And securities are transferred to account
    And cash is debited from securities account
    And settlement status is "settled"

  @critical @bc-securities-015 @securities
  Scenario: Monitor unsettled cash and securities
    Given buy order executed on "2026-04-07" for "50000 TND"
    When current date is "2026-04-08" (T+1)
    Then unsettled cash shows "50000 TND"
    And settled cash remains "0 TND"
    And position shows as "pending settlement"

  @high @bc-securities-016 @securities
  Scenario: Handle settlement failure and exception
    Given settlement record "SETT-2026-001" scheduled for "2026-04-09"
    When counterparty fails to deliver securities
    Then settlement status changes to "failed"
    And failure reason is recorded
    And system generates exception alert
    And trading desk is notified for manual intervention

  @high @bc-securities-017 @securities
  Scenario: Reverse failed settlement
    Given failed settlement "SETT-2026-001"
    When resolution occurs and settlement is reversed
    Then settlement status changes to "reversed"
    And reverse event is recorded
    And cash and securities are restored to pre-settlement state

  # Corporate Actions
  @critical @bc-securities-018 @securities
  Scenario: Process dividend payment corporate action
    Given security "Tunisie Telecom" with ISIN "TN0000000000"
    And customer holding 1000 shares with ex-date "2026-04-15"
    When dividend corporate action is announced:
      | Ex-Date | 2026-04-15 |
      | Record Date | 2026-04-17 |
      | Payment Date | 2026-05-01 |
      | Dividend per Share | 0.50 TND |
    Then corporate action is created with code "CA-DIV-001"
    And action status is "announced"

  @high @bc-securities-019 @securities
  Scenario: Apply dividend corporate action on ex-date
    Given corporate action "CA-DIV-001" with ex-date "2026-04-15"
    When ex-date is reached on "2026-04-15"
    Then action status changes to "ex_date_passed"
    And customers must have owned shares as of record date
    And dividend entitlement is "500 TND" (1000 × 0.50)

  @high @bc-securities-020 @securities
  Scenario: Pay dividend to customer account
    Given dividend entitlement of "500 TND"
    When payment date is reached on "2026-05-01"
    Then dividend payment is credited to cash balance
    And gross distribution is "500 TND"
    And withholding tax (20%) of "100 TND" is deducted
    And net distribution of "400 TND" is credited
    And dividend payment is recorded

  @medium @bc-securities-021 @securities
  Scenario: Handle stock split corporate action
    Given holding 100 shares at "100 TND" each (total "10000 TND")
    When 2:1 stock split is announced and executed
    Then holding becomes 200 shares
    And cost basis per share becomes "50 TND"
    And total holding value remains "10000 TND"
    And split ratio is "2:1"

  @medium @bc-securities-022 @securities
  Scenario: Handle merger/spin-off corporate action
    Given holding in parent company scheduled for spin-off
    When spin-off becomes effective
    Then original holdings split between parent and new entity
    And new ISIN is assigned to spin-off company
    And customer receives shares in both entities

  # Portfolio Valuation and Reporting
  @critical @bc-securities-023 @securities
  Scenario: Calculate daily portfolio valuation
    Given securities account with mixed holdings:
      | 100 shares @ 25.50 TND (Orange) |
      | 50 shares @ 18.00 TND (Tunisie Telecom) |
      | 30000 TND cash |
    When daily valuation is triggered on "2026-04-07"
    Then portfolio valuation is created for "2026-04-07"
    And total holdings value = "4350 TND" (100×25.50 + 50×18.00)
    And total portfolio value = "34350 TND" (4350 + 30000)
    And daily gain/loss is calculated

  @high @bc-securities-024 @securities
  Scenario: Calculate and track unrealized gains/losses
    Given position: 100 shares @ cost "25.50 TND", current price "26.50 TND"
    When daily valuation occurs
    Then unrealized gain = "100 TND" (100 × (26.50 - 25.50))
    And cost basis = "2550 TND"
    And current market value = "2650 TND"
    And gain/loss percentage = "3.92%"

  @high @bc-securities-025 @securities
  Scenario: Track year-to-date performance metrics
    Given portfolio with:
      | Dividend income YTD | 1500 TND |
      | Interest income YTD | 250 TND |
      | Commissions paid YTD | 500 TND |
      | Tax paid YTD | 350 TND |
      | Realized gains YTD | 2000 TND |
    When monthly valuation is calculated
    Then performance metrics are summarized in statement
    And net income YTD = "2900 TND" (1500 + 250 - 500 - 350 + 2000)

  @high @bc-securities-026 @securities
  Scenario: Calculate portfolio diversification score
    Given portfolio with:
      | Technology sector | 25% |
      | Healthcare | 20% |
      | Finance | 20% |
      | Other sectors | 35% |
    When diversification analysis is run
    Then diversification score is calculated (0.0-1.0)
    And largest position is "25%"
    And concentration risk is assessed
    And diversification recommendation is provided

  @medium @bc-securities-027 @securities
  Scenario: Generate portfolio statement
    Given securities account "SSEC-001" for period "2026-01-01" to "2026-04-07"
    When monthly statement is generated
    Then statement includes:
      | Opening balances |
      | All transactions |
      | Current holdings |
      | Portfolio value |
      | Performance metrics |
      | YTD summary |
    And statement is emailed to customer
