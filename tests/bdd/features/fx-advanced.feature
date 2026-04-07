Feature: Foreign Exchange Advanced (BC10) - FX Forwards, Swaps, Options, Limits
  As an FX trader
  I want to manage FX forwards, swaps, options, and position limits
  So that the bank manages foreign exchange risk and facilitates international transactions

  Background:
    Given the system is initialized
    And I am authenticated as "fx_trader"
    And FX trading module is operational

  # FX Forward Contracts
  @critical @fr-fxfwd-001 @fx @forwards
  Scenario: Enter FX forward contract
    Given spot rate EUR/TND = 3.15
    When trader initiates FX forward for 3 months
    Then forward contract includes:
      | Parameter | Value |
      | Currency Pair | EUR/TND |
      | Spot Rate | 3.15 |
      | Forward Points | +45 points (0.0045) |
      | Forward Rate | 3.1545 (3.15 + 0.0045) |
      | Maturity | 3 months (2025-07-01) |
      | Notional Amount | EUR 1,000,000 |
    And contract is recorded with status "Active"

  @high @fr-fxfwd-002 @fx @forwards
  Scenario: Track forward contract mark-to-market value
    Given FX forward contracted at 3.1545
    When spot rate moves to 3.20 (after 1 month)
    Then MTM valuation:
      | Field | Value |
      | Original forward rate | 3.1545 |
      | Current spot rate | 3.20 |
      | MTM value on EUR 1M | TND 4,550 loss |
      | Position is in the money for TND holder | Yes |
      | Daily MTM is marked to portfolio | Yes |
    And loss is reflected in daily P&L

  @high @fr-fxfwd-003 @fx @forwards
  Scenario: Settle forward contract at maturity
    Given FX forward EUR/TND at 3.1545, notional EUR 1,000,000
    When maturity date arrives (2025-07-01)
    Then settlement:
      | Action | Amount |
      | EUR payment from counterparty | EUR 1,000,000 |
      | TND payment to counterparty | TND 3,154,500 |
      | Final spot rate | 3.18 (for reference only) |
      | Settlement via SWIFT MT202 | Executed |
    And forward contract is marked "Settled"

  # FX Swaps
  @critical @fr-swap-001 @fx @swaps
  Scenario: Execute FX swap transaction
    Given USD funding need, hold TND
    When entering FX swap:
      | Leg | Currency | Amount | Rate | Maturity |
      | Leg 1 (Spot) | USD/TND spot buy | USD 1,000,000 | 3.12 | Today |
      | Leg 2 (Forward) | USD/TND forward sell | USD 1,000,000 | 3.15 | 6 months |
    Then swap includes:
      | Detail |
      | Receive: TND 3,120,000 (Leg 1) |
      | Pay back: USD 1,000,000 (Leg 2) |
      | Interest cost = forward spread (0.03 × 6 months) |
    And swap is recorded as matched pair

  @high @fr-swap-002 @fx @swaps
  Scenario: Use FX swap for liquidity management
    Given TND liquidity shortage, USD surplus
    When deploying FX swap for liquidity
    Then swap benefits:
      | Benefit |
      | Swap spot leg provides immediate TND liquidity |
      | Forward leg creates forward obligation |
      | No net currency risk (matched pair) |
      | Cost = swap spread (typically 15-50 pips) |
    And swap is used for overnight or short-term funding

  @high @fr-swap-003 @fx @swaps
  Scenario: Calculate FX swap spread and cost
    Given FX swap USD/TND for 6 months
    When calculating cost:
      | Component | Value |
      | Spot rate | 3.12 |
      | 6-month forward rate | 3.15 |
      | Spread (forward - spot) | 0.03 (3 pips × 100) |
      | Notional | USD 1,000,000 |
      | Cost in TND | TND 30,000 (0.03 × 1M) |
      | Annual cost (annualized) | ~2% per annum |
    And cost is factored into funding decisions

  # FX Options
  @critical @fr-option-001 @fx @options
  Scenario: Structure FX call option for customer
    Given customer wants protection on USD payable
    When structuring EUR 500,000 call option on USD/TND
    Then option terms:
      | Parameter | Value |
      | Option Type | Call (right to buy USD) |
      | Notional | USD 500,000 |
      | Strike Price | 3.20 |
      | Current Spot | 3.12 |
      | Maturity | 3 months |
      | Premium | 0.05 (TND per USD) |
      | Premium in TND | TND 25,000 |
    And customer pays premium upfront
    And option is recorded with status "Active"

  @high @fr-option-002 @fx @options
  Scenario: Evaluate option exercise decision at maturity
    Given call option on USD/TND, strike 3.20, premium paid 0.05
    When option maturity arrives and spot = 3.25
    Then exercise analysis:
      | Scenario | Spot Rate | Action | Payoff |
      | Exercise | 3.25 | Buy USD at 3.20 | Profit 0.05 less premium |
      | Not Exercise | 3.25 | Let expire, buy spot | Cost premium (0.05) |
      | Intrinsic Value | 3.25 - 3.20 | = 0.05 | Breakeven |
    And customer exercises: Buy USD 500,000 at 3.20
    And receive USD 500,000

  @high @fr-option-003 @fx @options
  Scenario: Calculate option Greeks for risk management
    Given FX option portfolio
    When calculating Greeks:
      | Greek | Meaning | Value | Usage |
      | Delta | Rate of change in option value | 0.55 | Hedge: buy 0.55 USD per 1 option |
      | Gamma | Delta change rate | 0.025 | Rebalancing frequency |
      | Vega | Volatility sensitivity | 0.15 | Volatility hedging |
      | Theta | Time decay | -0.02/day | Daily P&L impact |
      | Rho | Interest rate sensitivity | 0.08 | Rate risk management |
    And Greeks are used for dynamic hedging

  # Position Limits and Risk Controls
  @critical @fr-limit-001 @fx @limits
  Scenario: Set and enforce FX position limits
    Given FX trading risk framework
    When defining position limits
    Then limits include:
      | Limit | Threshold | Monitoring |
      | Net spot position | ± USD 10M | Real-time |
      | Forward position | ± USD 20M | Daily |
      | Call option exposure | ± USD 15M | Daily |
      | Swap oustanding | ± USD 25M | Daily |
      | VaR (99%, 1-day) | USD 500,000 | Daily |
      | Individual trader limit | USD 5M | Real-time |
    And breaches trigger alerts and restrictions

  @critical @fr-limit-002 @fx @limits
  Scenario: Alert on position limit breach
    Given net spot position limit = USD 10M
    When trader tries to increase position to USD 10.5M
    Then system:
      | Action |
      | Rejects order: "LimitBreach" |
      | Notifies trader of overage (USD 0.5M) |
      | Escalates to Risk Manager |
      | Proposal to hedge or reduce position |
    And position remains at previous level (USD 9.8M)

  @high @fr-limit-003 @fx @limits
  Scenario: Generate daily risk report for FX positions
    Given end-of-day FX positions
    When generating daily risk report
    Then report includes:
      | Metric | Value |
      | Net FX exposure | USD 8.2M long |
      | Forwards outstanding | USD 18M |
      | Options positions | 5 active |
      | Current VaR (99%, 1-day) | USD 420,000 |
      | Limit utilization | 84% |
      | Daily P&L | USD 125,000 profit |
    And report is reviewed by Risk Manager

  # FX Derivative Valuation
  @high @fr-valuation-001 @fx @valuation
  Scenario: Apply Black-Scholes pricing for FX options
    Given FX call option parameters:
      | Parameter | Value |
      | Spot rate (S) | 3.12 |
      | Strike (K) | 3.20 |
      | Time to maturity (T) | 0.25 (3 months) |
      | Volatility (σ) | 12% per annum |
      | Risk-free rate (r) | 5% per annum |
    When calculating Black-Scholes option price
    Then theoretical premium:
      | Component | Value |
      | d1 | -0.385 |
      | N(d1) | 0.350 |
      | d2 | -0.445 |
      | N(d2) | 0.328 |
      | Call price | 0.0245 (TND per USD) |
    And market price validation: Market premium 0.05 vs theoretical 0.0245
    And option is overpriced (good buy for customer, bad for bank)

  @high @fr-valuation-002 @fx @valuation
  Scenario: Mark derivative portfolio to market daily
    Given portfolio of FX derivatives
    When performing daily MTM valuation
    Then MTM process:
      | Step |
      | Revalue each derivative contract |
      | Use current market rates/volatility |
      | Calculate daily P&L |
      | Update balance sheet |
      | Provide risk metrics (Greeks) |
    And daily derivatives statement is generated

  # Regulatory Reporting for FX
  @high @fr-reg-001 @fx @regulatory
  Scenario: Report FX positions to BCT
    Given monthly position reporting requirement
    When preparing BCT FX regulatory report
    Then report includes:
      | Section |
      | Open FX spot positions (net by currency) |
      | Forward contracts (maturity ladder) |
      | FX options (open positions, Greeks) |
      | FX swaps (maturities, spreads) |
      | Concentration by counterparty |
      | VaR and stress test results |
    And report is filed within 10 business days

  @high @fr-reg-002 @fx @regulatory
  Scenario: Conduct FX stress testing
    Given current FX positions
    When running stress test scenarios
    Then scenarios include:
      | Scenario | Assumption | Impact |
      | 10% depreciation | TND weakens 10% vs USD | Loss USD 800,000 |
      | 500 bps rate hike | Interest rates up 500 bps | Loss USD 250,000 |
      | Volatility spike | Volatility +10% | Loss USD 180,000 (options) |
      | Combined worst case | All above simultaneously | Loss USD 1,230,000 |
    And stress test results are documented
    And capital impact is calculated

  # Customer FX Services
  @high @fr-customer-001 @fx @services
  Scenario: Quote customer FX rate
    Given customer requesting EUR/TND quote
    When quoting customer rate
    Then quote includes:
      | Item | Calculation |
      | Interbank rate | 3.1500 |
      | Bank margin | +0.35% |
      | Customer rate | 3.1611 |
      | Bid (we pay customer TND) | 3.1580 |
      | Ask (we receive from customer) | 3.1642 |
      | Quote validity | 2 minutes |
    And customer must accept within validity period

  @high @fr-customer-002 @fx @services
  Scenario: Execute customer FX transaction
    Given customer accepted quote EUR/TND at 3.1611
    When customer wants to buy EUR 100,000
    Then transaction:
      | Action | Amount |
      | Debit customer TND account | TND 316,110 |
      | Credit customer EUR account | EUR 100,000 |
      | Settlement | T+2 (2 business days) |
      | Confirmation sent | Immediately |
    And FX transaction is recorded
    And hedge is executed in interbank market

  @medium @fr-customer-003 @fx @services
  Scenario: Provide FX coverage for customer exposure
    Given customer with USD payment due in 1 month
    When customer buys forward cover (USD 50,000)
    Then customer is protected:
      | Protection |
      | Fixed rate locked at 3.15 |
      | No settlement risk |
      | Certainty on TND cost (TND 157,500) |
      | If TND weakens, customer benefits from protection |
      | If TND strengthens, customer "misses gain" but has certainty |

  # FX Counterparty Risk
  @high @fr-counterparty-001 @fx @counterparty
  Scenario: Monitor counterparty credit limits
    Given limits with 10 major FX counterparties
    When monitoring daily exposures
    Then monitoring includes:
      | Counterparty | Limit | Exposure | %Utilized |
      | Bank A | USD 50M | USD 32M | 64% |
      | Bank B | USD 40M | USD 38M | 95% |
      | Bank C | USD 30M | USD 18M | 60% |
    And counterparties near limit (>90%) trigger alerts
    And new transactions with Bank B rejected until exposure decreases

  @high @fr-counterparty-002 @fx @counterparty
  Scenario: Calculate CVA (Credit Valuation Adjustment)
    Given derivative portfolio with counterparty
    When calculating CVA:
      | Input | Value |
      | Replacement value (if counterparty defaults) | USD 2.5M |
      | Counterparty PD (1-year) | 2% |
      | Recovery rate | 40% |
      | CVA calculation | 2.5M × 2% × (1-40%) = USD 30,000 |
    Then CVA is deducted from derivative valuation
    And CVA is monitored daily for counterparties

  # Trading Risk Management
  @high @fr-trading-001 @fx @trading
  Scenario: Calculate and limit Value at Risk (VaR)
    Given FX trading positions
    When calculating daily VaR (99% confidence, 1-day)
    Then VaR calculation:
      | Metric | Value |
      | Current portfolio value | USD 125M |
      | Standard deviation (daily) | 0.4% |
      | 99% confidence multiplier | 2.326 |
      | Daily VaR | 125M × 0.4% × 2.326 = USD 1,163,000 |
    And VaR limit is USD 1.5M
    And current VaR = USD 1.163M (77% utilization)

  @high @fr-trading-002 @fx @trading
  Scenario: Monitor Expected Shortfall (ES) beyond VaR
    Given VaR does not capture tail risks
    When calculating Expected Shortfall (1% tail)
    Then ES analysis:
      | Scenario | Loss |
      | 1st percentile loss | USD 1.5M |
      | 0.5th percentile loss | USD 2.1M |
      | Expected Shortfall (avg of worst 1%) | USD 1.85M |
    And ES provides buffer for extreme scenarios
    And ES is monitored in addition to VaR

  @medium @fr-trading-003 @fx @trading
  Scenario: Generate daily trading report
    Given daily FX trading activities
    When generating trading report
    Then report includes:
      | Metric | Value |
      | Trades executed | 127 |
      | Notional value | USD 2.5B |
      | Profit/Loss | USD 245,000 |
      | Largest single trade | USD 150M |
      | Average spread earned | 2.3 pips |
      | Positions at end of day | See position sheet |
      | Risk metrics (VaR, ES) | Within limits |
