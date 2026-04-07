Feature: Advanced Credit Management (BC3) - Revolving Lines, Syndication, Restructuring
  As a credit manager
  I want to manage revolving credit lines, syndicated facilities, debt restructuring, and moratory interest
  So that the bank manages advanced credit products and credit risk effectively

  Background:
    Given the system is initialized
    And I am authenticated as "credit_manager"
    And credit management module is active

  # Revolving Credit Lines
  @critical @fr-revol-001 @credit @facility
  Scenario: Create revolving credit line facility
    Given approved customer with credit rating "BBB"
    When establishing revolving line facility
    Then facility is created with:
      | Parameter | Value |
      | Facility Type | Revolving Credit Line |
      | Total Limit | TND 500,000 |
      | Drawdown Structure | Flexible (up to limit) |
      | Tenor | 3 years |
      | Interest Rate | TUSD + 3.5% |
      | Commitment Fee | 0.25% per annum |
      | Undrawn Fee | 0.10% per annum |
    And facility status is "Active"

  @high @fr-revol-002 @credit @facility
  Scenario: Track drawdowns on revolving line
    Given active revolving line with TND 500,000 limit
    When customer makes drawdowns:
      | Date | Amount | Running Balance |
      | 2025-04-01 | TND 100,000 | TND 100,000 |
      | 2025-04-15 | TND 75,000 | TND 175,000 |
      | 2025-05-01 | TND 150,000 | TND 325,000 |
    Then each drawdown is recorded with:
      | Detail |
      | Drawdown date |
      | Amount |
      | Interest accrual begins |
      | Available balance updates (TND 175,000) |

  @high @fr-revol-003 @credit @facility
  Scenario: Enforce revolving line limits and covenants
    Given revolving line with TND 500,000 limit and covenant on leverage ratio (max 3.5x)
    When customer attempts drawdown of TND 250,000 (total would be TND 575,000)
    Then drawdown is rejected: "ExceededFacilityLimit"
    And available amount shown as TND 175,000
    And when customer attempts drawdown of TND 100,000 but leverage ratio would breach
    Then drawdown is rejected: "CovenantsBreached"
    And customer is notified of reason

  @high @fr-revol-004 @credit @facility
  Scenario: Manage revolving line renewals
    Given revolving line expiring in 90 days (expires 2025-07-01)
    When initiating renewal process at 120-day mark
    Then renewal process includes:
      | Step |
      | Customer financial statements review |
      | Credit rating reassessment |
      | Covenant compliance verification |
      | Updated facility agreement |
      | Board approval for renewal |
    And new facility is activated before expiry date

  @medium @fr-revol-005 @credit @facility
  Scenario: Calculate and charge commitment fees
    Given revolving line: limit TND 500,000, drawn TND 300,000, commitment fee 0.25%
    When charging annual commitment fee
    Then fee calculation:
      | Component | Amount |
      | Fee base (undrawn portion) | TND 200,000 |
      | Commitment fee rate | 0.25% |
      | Annual fee | TND 500 |
    And quarterly installment = TND 125
    And fee is charged on billing date

  # Sub-limits and Product Mix
  @high @fr-sublimit-001 @credit @sublimit
  Scenario: Establish sub-limits within credit line
    Given revolving credit line facility of TND 500,000
    When defining sub-limits
    Then sub-limits are:
      | Sub-limit | Amount | Purpose |
      | Trade Finance | TND 150,000 | Letters of credit, bills |
      | Treasury | TND 100,000 | FX forwards, money market |
      | Overdraft | TND 100,000 | Overdraft facility |
      | Cash Credit | TND 150,000 | Working capital loans |
    And total sub-limits = facility limit
    And each sub-limit is separately tracked

  @high @fr-sublimit-002 @credit @sublimit
  Scenario: Enforce sub-limit controls
    Given sub-limit: Trade Finance = TND 150,000, currently drawn TND 145,000
    When customer requests TND 10,000 LC issuance
    Then request is rejected: "SubLimitExceeded"
    And available under sub-limit = TND 5,000
    And customer must reduce existing exposure or increase sub-limit

  @medium @fr-sublimit-003 @credit @sublimit
  Scenario: Generate sub-limit utilization report
    Given facility with 4 sub-limits
    When generating monthly utilization report
    Then report shows:
      | Sub-limit | Limit | Drawn | Available | Utilization |
      | Trade Finance | 150,000 | 145,000 | 5,000 | 96.7% |
      | Treasury | 100,000 | 45,000 | 55,000 | 45% |
      | Overdraft | 100,000 | 30,000 | 70,000 | 30% |
      | Cash Credit | 150,000 | 125,000 | 25,000 | 83.3% |
    And alerts for >90% utilization

  # Syndicated Facilities
  @critical @fr-syndicate-001 @credit @syndication
  Scenario: Create syndicated facility with multiple lenders
    Given large facility needed (TND 5,000,000)
    When establishing syndication
    Then syndicate is structured with:
      | Lender Role | Bank | Commitment | Percentage |
      | Arranger/Lead | BANKO | TND 1,500,000 | 30% |
      | Co-Arranger | Bank A | TND 1,250,000 | 25% |
      | Participant | Bank B | TND 1,250,000 | 25% |
      | Participant | Bank C | TND 1,000,000 | 20% |
    And syndicate agreement is executed
    And facility is activated when commitments confirmed

  @high @fr-syndicate-002 @credit @syndication
  Scenario: Track syndicate fund commitments and disbursements
    Given syndicated facility with TND 5,000,000 total
    When managing disbursements
    Then tracking includes:
      | Lender | Commitment | Drawn | Outstanding |
      | BANKO | 1,500,000 | 1,200,000 | 1,200,000 |
      | Bank A | 1,250,000 | 1,000,000 | 1,000,000 |
      | Bank B | 1,250,000 | 1,100,000 | 1,100,000 |
      | Bank C | 1,000,000 | 700,000 | 700,000 |
      | Total | 5,000,000 | 4,000,000 | 4,000,000 |

  @high @fr-syndicate-003 @credit @syndication
  Scenario: Distribute interest and fees to syndicate partners
    Given syndicated facility with outstanding TND 4,000,000
    When calculating monthly distributions (TUSD+3.5%, rates vary):
      | Component | BANKO (30%) | Bank A (25%) | Bank B (25%) | Bank C (20%) | Total |
      | Interest income | 12,000 | 10,000 | 10,000 | 8,000 | 40,000 |
      | Arrangement fee share | 3,000 | 2,500 | 2,500 | 2,000 | 10,000 |
    Then distributions are calculated and recorded
    And each lender receives proportional share
    And settlement occurs within 5 business days

  @high @fr-syndicate-004 @credit @syndication
  Scenario: Manage covenant compliance in syndicated facility
    Given syndicated facility with financial covenants
    When quarterly covenant testing occurs
    Then covenant compliance is tracked:
      | Covenant | Target | Actual | Status |
      | Leverage Ratio | < 3.0x | 2.8x | Compliant |
      | Interest Coverage | > 3.5x | 4.2x | Compliant |
      | Debt/Equity | < 2.0x | 1.95x | Compliant |
    And compliance status is reported to all syndicate lenders
    And certificate is signed by borrower

  # Credit Restructuring and Forbearance
  @critical @fr-restruc-001 @credit @restructuring
  Scenario: Initiate debt restructuring proposal
    Given customer in financial distress with NPL status
    When proposing restructuring
    Then restructuring proposal includes:
      | Element |
      | Current outstanding | TND 250,000 |
      | Payment arrears | TND 18,000 |
      | Proposed new terms | Extend tenor 2 years |
      | Proposed rate | Reduce from 8% to 6.5% |
      | Haircut | 2% (TND 5,000) |
    And proposal is presented to customer and internal credit committee
    And financial impact is assessed

  @critical @fr-restruc-002 @credit @restructuring
  Scenario: Apply accounting treatment for restructured loan
    Given loan restructured with:
      | Terms | Before | After |
      | Outstanding | TND 250,000 | TND 245,000 |
      | Rate | 8.0% | 6.5% |
      | Tenor | 1 year | 3 years |
      | Classification | Class 3 (NPL) | Class 2 (Close Monitoring) |
    When recording restructuring in accounting
    Then entries:
      | Description | Debit | Credit |
      | Loan write-down | 5,000 | (Credit loss expense) |
      | Accrued interest adjustment | (accrued) | Loan balance |
    And classification changes to "Restructured - Performing"
    And specific provision may be reduced

  @high @fr-restruc-003 @credit @restructuring
  Scenario: Monitor restructured loan performance
    Given restructured loan with new terms
    When monitoring post-restructuring
    Then tracking includes:
      | Metric |
      | Monthly payments on time |
      | Financial covenants compliance |
      | Customer financial improvement |
      | Potential for further deterioration |
    And if customer meets all conditions for 12 consecutive months
    Then loan can be removed from restructured status

  @high @fr-restruc-004 @credit @restructuring
  Scenario: Exit restructured loan status
    Given restructured loan performing for 12+ months
    When customer demonstrates financial recovery
    Then restructuring exit process:
      | Step |
      | 12-month performance verification |
      | Financial statement review (improving trends) |
      | Board approval for exit |
      | Reclassification to normal status |
    And loan is returned to normal monitoring
    And history is retained for audit trail

  # Moratory Interest
  @critical @fr-moratory-001 @credit @interest
  Scenario: Calculate and apply moratory interest on arrears
    Given loan with:
      | Detail | Amount |
      | Outstanding | TND 100,000 |
      | Arrears (60+ days) | TND 8,000 |
      | Normal rate | 8.0% per annum |
      | Moratory rate | 12.0% per annum |
    When calculating monthly moratory interest
    Then calculation:
      | Item | Amount |
      | Regular interest on principal | TND 667 |
      | Moratory interest on arrears | TND 80 |
      | Total interest charge | TND 747 |
    And moratory interest is applied until arrears cleared

  @high @fr-moratory-002 @credit @interest
  Scenario: Stop moratory interest upon arrear clearance
    Given account with moratory interest being charged
    When customer pays all arrears (TND 8,000)
    Then:
      | Action |
      | Moratory interest charge stops |
      | Regular interest rate resumes |
      | Arrears balance = TND 0 |
      | Account status improves |
    And interest rate reverts to normal 8.0%

  @high @fr-moratory-003 @credit @interest
  Scenario: Report moratory interest separately for regulatory purposes
    Given monthly interest calculations
    When generating regulatory reporting
    Then moratory interest is:
      | Field |
      | Separately tracked and reported |
      | Classified by arrear period (30-60, 60-90, >90) |
      | Included in problem loan metrics |
      | Monitored for trend analysis |
    And BCT reporting shows moratory interest separately

  # Early Repayment and Prepayment
  @high @fr-early-001 @credit @repayment
  Scenario: Allow early repayment with prepayment penalty
    Given loan with:
      | Term | Detail |
      | Outstanding | TND 50,000 |
      | Maturity | 24 months (12 months remaining) |
      | Prepayment Penalty | 1.5% of outstanding |
    When customer requests early repayment
    Then prepayment is calculated:
      | Component | Amount |
      | Outstanding principal | TND 50,000 |
      | Accrued interest (to repayment date) | TND 4,000 |
      | Prepayment penalty (1.5%) | TND 750 |
      | Total due | TND 54,750 |
    And customer approves terms
    And loan is repaid and closed

  @high @fr-early-002 @credit @repayment
  Scenario: Waive prepayment penalty in certain conditions
    Given customer with excellent repayment history
    When requesting early repayment with penalty waiver
    Then criteria for waiver:
      | Criterion | Status |
      | No missed payments | Yes |
      | 12+ months of on-time performance | Yes |
      | No covenant breaches | Yes |
      | Customer retention benefit | Yes |
    And waiver is approved by credit manager
    And prepayment penalty is waived
    And customer repays only principal + accrued interest

  @medium @fr-early-003 @credit @repayment
  Scenario: Refinance maturing loan with new facility
    Given loan maturing in 30 days
    When initiating refinancing
    Then refinance process:
      | Step | Action |
      | 1. Customer Request | Approved for new facility |
      | 2. Credit Review | New assessment conducted |
      | 3. Facility Approval | TND 50,000, 24-month tenor |
      | 4. Documentation | New facility agreement signed |
      | 5. Drawdown | Funds applied to repay maturing loan |
      | 6. Maturity | Original loan paid off |
    And no gap in facility availability

  # Loan Syndication and Assignments
  @high @fr-assign-001 @credit @assignment
  Scenario: Sell loan participation to secondary market
    Given BANKO-originated loan of TND 200,000
    When selling 50% participation to Bank X
    Then assignment includes:
      | Detail |
      | Principal transferred | TND 100,000 |
      | Interest share transferred | 50% |
      | Credit approval | Shared with Bank X |
      | Covenant monitoring | Bank X participates |
    And assignment agreement is executed
    And proceeds are credited to BANKO
    And Bank X becomes junior lender

  @high @fr-assign-002 @credit @assignment
  Scenario: Track loan assignment and participation
    Given loan with multiple participations
    When tracking ownership
    Then tracking shows:
      | Participant | Share | Interest | Covenant |
      | BANKO (Arranger) | 40% | 40% | Lead |
      | Bank X | 30% | 30% | Participant |
      | Bank Y | 30% | 30% | Participant |
    And each participant receives proportional distributions

  # Covenant Monitoring and Breach
  @critical @fr-covenant-001 @credit @covenants
  Scenario: Monitor financial covenants on term loans
    Given term loan with financial covenants:
      | Covenant | Threshold | Test Frequency |
      | Leverage Ratio (Debt/EBITDA) | < 3.5x | Quarterly |
      | Interest Coverage (EBITDA/Interest) | > 2.5x | Quarterly |
      | Minimum Equity | > TND 100,000 | Annual |
      | Debt Service Coverage Ratio | > 1.2x | Quarterly |
    When conducting quarterly covenant testing
    Then results are compared and verified

  @critical @fr-covenant-002 @credit @covenants
  Scenario: Alert and escalate on covenant breach
    Given leverage ratio = 3.7x (exceeds 3.5x threshold)
    When testing quarterly covenants
    Then breach is identified:
      | Action | Timing |
      | Breach notification to borrower | Immediate |
      | Cure period begins | 30 days |
      | Internal escalation to CFO/CEO | Within 1 day |
      | Board notification | Next board meeting |
    And borrower has 30 days to cure

  @high @fr-covenant-003 @credit @covenants
  Scenario: Process covenant waiver request
    Given covenant breach with no cure possible
    When borrower requests waiver
    Then waiver process:
      | Criterion | Assessment |
      | Justification for breach | Temporary market conditions |
      | Cure timeline | 90 days |
      | Additional collateral | Required (10% increase) |
      | Interest rate adjustment | +0.5% |
    And waiver is approved conditionally
    And new terms are documented in amendment

  # Credit Line Automatic Renewals
  @high @fr-auto-renew-001 @credit @renewal
  Scenario: Configure automatic renewal terms
    Given revolving credit facility expiring 2025-07-01
    When setting up automatic renewal
    Then renewal terms are configured:
      | Parameter | Setting |
      | Renewal Type | Automatic unless notice given |
      | Notice period required | 90 days |
      | New tenor | 1 year (rolling) |
      | Rate adjustment | TUSD +3.5% (market rate) |
      | Limit adjustment | Increase by 2% or flat |
    And terms are documented

  @high @fr-auto-renew-002 @credit @renewal
  Scenario: Process automatic facility renewal
    Given facility with auto-renewal enabled
    When facility approaches expiry and no non-renewal notice received
    Then automatic renewal occurs:
      | Action | Date |
      | Final notice sent to customer | 60 days before |
      | Renewal conditions confirmed | 30 days before |
      | Updated facility agreement | 15 days before |
      | Auto-renewal effective | Expiry date |
    And new facility is seamlessly activated
