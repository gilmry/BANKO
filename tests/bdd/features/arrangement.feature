Feature: Arrangement Lifecycle Management (P2-BC3)
  As a relationship manager and portfolio manager
  I want to manage arrangement lifecycle from creation through maturity
  So that customer banking relationships are efficiently managed with proper term management and event tracking

  Background:
    Given the system is initialized
    And I am authenticated as "relationship_manager"
    And arrangement module is operational

  # Arrangement Creation and Approval
  @critical @bc-arrangement-001 @arrangement
  Scenario: Create a loan arrangement proposal
    Given customer "Ahmed Hassan" with credit approval
    When I create arrangement proposal with:
      | Arrangement Type | Loan |
      | Product | Personal Loan 24M |
      | Principal Amount | 100000 TND |
      | Currency | TND |
      | Interest Rate | 8.5% fixed |
      | Term | 24 months |
      | Purpose | Home renovation |
    Then arrangement is created with code "ARR-LOAN-2026-001"
    And arrangement status is "proposed"
    And default values are set:
      | Activation Date | Pending |
      | Maturity Date | 2028-04-07 |
      | Installment Frequency | Monthly |
      | Total Installments | 24 |
      | Installment Amount | 4700 TND |

  @critical @bc-arrangement-002 @arrangement
  Scenario: Approve arrangement proposal
    Given arrangement proposal "ARR-LOAN-2026-001"
    When credit committee approves with:
      | Approval Date | 2026-04-07 |
      | Approved By | Credit Committee |
      | Conditions | Provide payroll certificates |
    Then arrangement status changes to "approved"
    And approval date is recorded
    And approval conditions are captured
    And customer is notified of approval

  @high @bc-arrangement-003 @arrangement
  Scenario: Create credit line arrangement
    Given corporate customer "Tech Corp"
    When I create credit line arrangement with:
      | Arrangement Type | Credit Line |
      | Credit Limit | 500000 TND |
      | Currency | TND |
      | Maturity | 2027-04-07 (1 year) |
      | Interest Rate | EURIBOR + 2.5% |
      | Facility Type | Revolving |
    Then credit line arrangement is created with code "ARR-CRED-2026-001"
    And arrangement status is "proposed"

  @high @bc-arrangement-004 @arrangement
  Scenario: Create deposit arrangement
    Given individual customer for savings
    When I create deposit arrangement with:
      | Arrangement Type | Deposit |
      | Product | Fixed Deposit 12M |
      | Principal Amount | 50000 TND |
      | Interest Rate | 3.5% fixed |
      | Maturity | 2027-04-07 |
    Then deposit arrangement is created with code "ARR-DEP-2026-001"
    And arrangement status is "proposed"

  @medium @bc-arrangement-005 @arrangement
  Scenario: Activate an approved arrangement
    Given approved arrangement "ARR-LOAN-2026-001"
    When I trigger activation with:
      | Activation Date | 2026-04-08 |
      | Funding Method | Bank transfer |
    Then arrangement status changes to "activated"
    And activation date is set
    And funds are disbursed to customer account
    And first installment schedule is created
    And arrangement events log records "activated" event

  # Arrangement Modifications
  @critical @bc-arrangement-006 @arrangement
  Scenario: Request interest rate reduction
    Given active loan arrangement "ARR-LOAN-2026-001" at 8.5%
    When customer requests rate reduction to 8.0%
    Then modification request is created with code "MOD-2026-001"
    And modification type is "rate_change"
    And modification status is "pending"
    And request date is recorded

  @high @bc-arrangement-007 @arrangement
  Scenario: Evaluate modification impact
    Given rate reduction request "MOD-2026-001" from 8.5% to 8.0%
    When impact analysis is performed
    Then analysis calculates:
      | Previous Interest Cost | 12000 TND |
      | New Interest Cost | 11294 TND |
      | Benefit to Customer | 706 TND |
      | Bank Impact | -706 TND |
      | Risk Impact Score | -0.5% |
    And impact report is generated

  @high @bc-arrangement-008 @arrangement
  Scenario: Approve modification request
    Given modification request "MOD-2026-001" with analysis
    When modification is approved by manager
    Then modification status changes to "approved"
    And approval date is recorded
    And new terms are set effective "2026-04-15"
    And customer is notified

  @high @bc-arrangement-009 @arrangement
  Scenario: Implement term extension modification
    Given loan arrangement near maturity (3 months remaining)
    When customer requests term extension by 12 months
    Then modification request is created with code "MOD-2026-002"
    And modification type is "term_extension"
    And new maturity date becomes "2029-04-07"
    And new interest rate "8.2%" is offered
    And remaining installments recalculated to 36 months

  @high @bc-arrangement-010 @arrangement
  Scenario: Process loan increase modification
    Given credit line arrangement "ARR-CRED-2026-001" at 500000 TND
    When customer requests increase to 750000 TND
    Then modification request is created with code "MOD-2026-003"
    And modification type is "amount_increase"
    And increase amount is "250000 TND"
    And credit assessment is triggered
    And new terms are evaluated

  @high @bc-arrangement-011 @arrangement
  Scenario: Restructure payment terms
    Given loan in arrears with missed 2 payments
    When restructuring proposal offered:
      | New Payment Schedule | 36 months (extended from 24) |
      | Revised Rate | 7.5% (reduced from 8.5%) |
      | Fees Waived | Yes |
    Then modification request is created with code "MOD-2026-004"
    And modification type is "payment_restructure"
    And customer acceptance is required

  @medium @bc-arrangement-012 @arrangement
  Scenario: Modify collateral requirements
    Given loan arrangement with collateral "Property X"
    When collateral valuation declines and triggers requirement
    Then modification request for additional collateral is created
    And modification type is "collateral_change"
    And additional collateral "Property Y" is pledged
    And LTV ratio is restored

  # Arrangement Events and State Management
  @critical @bc-arrangement-013 @arrangement
  Scenario: Record arrangement activation event
    Given approved arrangement being activated
    When activation occurs on "2026-04-08"
    Then arrangement event is created with:
      | Event Type | activated |
      | Event Date | 2026-04-08 |
      | Event Status | completed |
      | Triggered By | system (scheduled) |
      | Previous State | approved |
      | New State | activated |
    And event is logged in event history

  @high @bc-arrangement-014 @arrangement
  Scenario: Track payment events
    Given active loan with first installment due "2026-05-08"
    When payment of "4700 TND" is received
    Then arrangement event is created with:
      | Event Type | payment_made |
      | Payment Amount | 4700 TND |
      | Payment Date | 2026-05-08 |
      | Remaining Principal | 95300 TND |
    And remaining_installments decrements to 23
    And payment status remains "on_track"

  @high @bc-arrangement-015 @arrangement
  Scenario: Generate missed payment event and notification
    Given loan payment due "2026-05-08"
    When payment is not received and 10 days pass
    Then arrangement event is created with:
      | Event Type | payment_missed |
      | Days Past Due | 10 |
      | Amount Overdue | 4700 TND |
    And arrangement status changes to "payment_status: past_due"
    And customer notification is triggered
    And collection workflow is initiated

  @high @bc-arrangement-016 @arrangement
  Scenario: Record suspension event
    Given arrangement with multiple missed payments
    When suspension decision is made
    Then arrangement event is created with:
      | Event Type | suspended |
      | Suspension Date | 2026-04-27 |
      | Reason | Multiple missed payments |
    And arrangement status changes to "suspended"
    And no new transactions allowed
    And resolution plan is required

  @high @bc-arrangement-017 @arrangement
  Scenario: Record maturity event
    Given loan arrangement with maturity date "2028-04-07"
    When maturity date is reached
    Then arrangement event is created with:
      | Event Type | matured |
      | Maturity Date | 2028-04-07 |
      | Principal Due | 0 TND (fully paid) |
      | Interest Accrued | 0 TND |
    And arrangement status changes to "matured"
    And closure instructions are requested

  # Arrangement Renewal
  @critical @bc-arrangement-018 @arrangement
  Scenario: Initiate automatic renewal
    Given deposit arrangement "ARR-DEP-2026-001" maturing "2027-04-07"
    When 30 days before maturity
    Then renewal notification is sent to customer
    And automatic renewal is proposed with current terms
    And customer has 15 days to opt-out
    And renewal type is "automatic"

  @high @bc-arrangement-019 @arrangement
  Scenario: Process customer acceptance of renewal
    Given renewal proposal for deposit arrangement
    When customer confirms renewal
    Then renewal request status changes to "accepted"
    And new arrangement is created with code "ARR-DEP-2027-001"
    And original arrangement status becomes "closed"
    And new arrangement receives new terms (if changed)

  @high @bc-arrangement-020 @arrangement
  Scenario: Decline renewal and close arrangement
    Given renewal proposal for loan arrangement
    When customer declines renewal
    Then renewal status becomes "declined"
    And arrangement closure is initiated
    And closure date is set to maturity date
    And final statement is generated
    And customer is notified

  # Arrangement Bundles
  @critical @bc-arrangement-021 @arrangement
  Scenario: Create arrangement bundle
    Given customer "Ahmed Hassan" with multiple facilities
    When I create bundle with:
      | Bundle Name | Ahmed Hassan Comprehensive Banking |
      | Customer | Ahmed Hassan |
      | Bundle Type | relationship |
      | Included Arrangements | Loan, Credit Line, Deposit |
    Then bundle is created with code "BUNDLE-2026-001"
    And bundle status is "active"
    And total principal is sum of all arrangements
    And cross-default clause is enabled

  @high @bc-arrangement-022 @arrangement
  Scenario: Add arrangement to bundle
    Given existing bundle "BUNDLE-2026-001"
    When new loan arrangement "ARR-LOAN-2026-002" is created
    Then bundle member is created with:
      | Bundle | BUNDLE-2026-001 |
      | Arrangement | ARR-LOAN-2026-002 |
      | Member Role | primary |
      | Sequence | 3 |
      | Status | active |

  @high @bc-arrangement-023 @arrangement
  Scenario: Apply bundle cross-default clause
    Given bundle "BUNDLE-2026-001" with cross-default enabled
    When payment is missed on one loan
    Then cross-default event is triggered
    And all other arrangements in bundle are flagged
    And potential default on all facilities is considered
    And customer and credit committee are notified

  # Arrangement Closure
  @critical @bc-arrangement-024 @arrangement
  Scenario: Close matured arrangement
    Given matured loan arrangement "ARR-LOAN-2026-001"
    When closure is initiated
    Then arrangement status changes to "closed"
    And closure date is recorded as "2028-04-07"
    And final balance is verified as "0"
    And closure statement is generated
    And arrangement event "closed" is recorded

  @high @bc-arrangement-025 @arrangement
  Scenario: Early loan payoff and closure
    Given active loan with remaining principal "10000 TND"
    When customer pays off entire remaining balance
    Then early payoff is processed
    And arrangement status changes to "closed"
    And closure event is recorded with "early payoff" reason
    And no further interest is charged
    And payoff statement is issued

  @high @bc-arrangement-026 @arrangement
  Scenario: Cancel arrangement proposal
    Given arrangement in "proposed" status
    When cancellation is requested
    Then arrangement status changes to "cancelled"
    And cancellation date is recorded
    And cancellation reason is documented
    And customer is notified
    And no financial impact occurs

  # Arrangement Reporting
  @high @bc-arrangement-027 @arrangement
  Scenario: Generate arrangement statement
    Given active loan arrangement with transaction history
    When monthly statement is generated
    Then statement includes:
      | Arrangement Details | Code, Type, Status |
      | Balance Information | Principal, Interest |
      | Payment Information | Due Date, Amount Due, Paid |
      | Period Summary | Payments, Interest Charged |
      | Next Steps | Actions Required |

  @high @bc-arrangement-028 @arrangement
  Scenario: Generate portfolio analysis by arrangement type
    Given multiple active arrangements:
      | Loans | 15 arrangements, 5M TND |
      | Credit Lines | 8 arrangements, 3M TND |
      | Deposits | 22 arrangements, 2M TND |
    When portfolio analysis is triggered
    Then analysis shows:
      | Total Arrangements | 45 |
      | Total Principal | 10M TND |
      | By Type Distribution | Loans 50%, Deposits 20%, Credit 30% |
      | Maturity Profile | Next 12 months: 12 arrangements |
      | Risk Profile | Concentration, Default Risk |

  @medium @bc-arrangement-029 @arrangement
  Scenario: Generate customer relationship summary
    Given customer with multiple arrangements in bundle
    When relationship summary is requested
    Then summary includes:
      | Customer Profile | Name, Status |
      | Arrangements | All active arrangements |
      | Total Exposure | Sum of all principals |
      | Status Overview | Payments, Compliance |
      | Next Reviews | Upcoming renewals, modifications |
      | Risk Assessment | Overall relationship health |

  @medium @bc-arrangement-030 @arrangement
  Scenario: Alert on approaching maturity
    Given arrangements with maturity dates in next 90 days
    When maturity alert check is run
    Then alerts are generated for:
      | Loan "ARR-LOAN-2026-001" | Matures in 75 days |
      | Deposit "ARR-DEP-2026-001" | Matures in 45 days |
      | Credit Line "ARR-CRED-2026-001" | Matures in 30 days |
    And alerts are routed to relationship manager
    And renewal/closure planning is triggered
