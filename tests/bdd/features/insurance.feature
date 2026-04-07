Feature: Insurance Products (P1-BC5)
  As an insurance and bancassurance officer
  I want to manage insurance policies, claims, and bancassurance products
  So that the bank can offer comprehensive risk management solutions

  Background:
    Given the system is initialized
    And I am authenticated as "insurance_officer"
    And insurance module is operational

  # Insurance Policy Management
  @critical @bc-insurance-001 @policy
  Scenario: Create life insurance policy for credit protection
    Given customer taking loan of "500000 TND"
    When life insurance policy is purchased:
      | Policy Type | Credit Life Insurance |
      | Policyholder | Customer |
      | Sum Assured | 500000 TND (loan amount) |
      | Premium | 50000 TND annually (10%) |
      | Premium Frequency | Monthly: 4166.67 TND |
      | Term | 5 years (loan period) |
      | Decreasing Benefit | Yes (benefits decline with loan balance) |
      | Insurance Company | AXA Insurance Tunisia |
    Then policy is created with code "POL-CLI-2026-001"
    And policy is linked to loan account
    And premiums are auto-deducted from account
    And policy documentation is delivered to customer

  @high @bc-insurance-002 @policy
  Scenario: Issue health insurance policy
    Given corporate customer with 50 employees
    When group health insurance policy is requested:
      | Policy Type | Group Health Insurance |
      | Policyholder | Company ABC |
      | Covered Lives | 50 employees + spouses |
      | Coverage Type | In-patient + Out-patient |
      | Annual Premium | 500000 TND |
      | Monthly Premium | 41666.67 TND |
      | Deductible | 500 TND per claim |
      | Annual Limit | 5000000 TND per employee |
      | Network Hospitals | 100+ registered hospitals |
    Then group health policy is issued with code "POL-GHP-2026-001"
    And policy schedule is provided
    And member cards are issued
    And employer receives policy documents

  @high @bc-insurance-003 @policy
  Scenario: Create property insurance policy
    Given businessman owning commercial property
    When property insurance is purchased:
      | Property Type | Commercial building |
      | Location | Industrial zone, Sfax |
      | Insured Value | 2000000 TND |
      | Coverage | Fire, theft, earthquake, burglary |
      | Annual Premium | 80000 TND (4% of value) |
      | Deductible | 10000 TND |
      | Policy Term | 1 year |
      | Valuation | Professional appraisal from 2026 |
    Then property insurance policy is issued with code "POL-PROP-2026-001"
    And policy is linked to property collateral record
    And insured receives policy documents
    And annual renewal reminder is scheduled

  @high @bc-insurance-004 @policy
  Scenario: Process insurance policy renewal
    Given health insurance policy expiring on "2026-04-30"
    When renewal is processed 30 days before expiry:
      | Current Premium | 41666.67 TND/month |
      | Premium Increase | 5% (due to inflation) |
      | New Premium | 43750 TND/month |
      | Renewal Date | 2026-05-01 |
      | New Expiry | 2027-04-30 |
    Then renewal is processed automatically
    And new premium is charged
    And customer receives renewal confirmation
    And coverage continues uninterrupted

  @high @bc-insurance-005 @policy
  Scenario: Suspend insurance policy for non-payment
    Given active insurance policy with premium due "2026-04-15"
    When premium is not paid by "2026-04-30"
    And grace period expires
    Then policy suspension occurs:
      | Suspension Date | 2026-05-01 |
      | Status | Suspended |
      | Customer Notice | Sent via email and SMS |
      | Reinstatement Option | 30-day period available |
      | Back Premiums Required | Yes, plus interest |
    And coverage is temporarily suspended
    And claims cannot be filed during suspension period

  @high @bc-insurance-006 @policy
  Scenario: Cancel insurance policy with surrender
    Given active life insurance policy held for 3 years
    When policyholder requests cancellation:
      | Cancellation Request Date | 2026-04-07 |
      | Policy Held For | 3 years |
      | Total Premiums Paid | 150000 TND |
      | Surrender Value (50% for year 3) | 75000 TND |
      | Administrative Fees | 5000 TND |
      | Net Surrender Benefit | 70000 TND |
    Then policy is cancelled:
      | Effective Cancellation Date | 2026-05-07 (grace period) |
      | Surrender proceeds are paid to customer |
      | Final policy document marked as "Cancelled" |

  # Insurance Claims
  @critical @bc-insurance-007 @claim
  Scenario: File insurance claim for death benefit
    Given active credit life insurance policy "POL-CLI-2026-001"
    And policyholder deceased
    When claim is filed:
      | Claim Type | Death Benefit |
      | Claim Date | 2026-04-01 (date of death) |
      | Claim Submission Date | 2026-04-07 |
      | Claim Amount Requested | 500000 TND (full sum assured) |
      | Supporting Documents | Death certificate, policy document, ID |
    Then claim is registered with code "CLM-2026-001"
    And claim status is "submitted"
    And required documentation checklist is generated
    And beneficiary is contacted for verification

  @high @bc-insurance-008 @claim
  Scenario: Process insurance claim investigation
    Given submitted claim "CLM-2026-001" for death benefit
    When investigation is initiated:
      | Investigator Assigned | Mohamed Salem |
      | Investigation Start Date | 2026-04-08 |
      | Investigation Scope | Verify death, policy eligibility, no exclusions |
    Then investigation is conducted:
      | Verify Death Certificate | Authentic, issued by health authority |
      | Verify Policy Status | Active, no lapses |
      | Verify Exclusions | None applicable (natural death covered) |
      | Investigation Complete Date | 2026-04-14 |
    And investigation report is prepared

  @high @bc-insurance-009 @claim
  Scenario: Approve and pay insurance claim
    Given completed investigation supporting claim
    When claim is reviewed for approval:
      | Review Status | All documentation complete |
      | Investigation Findings | Claim eligible |
      | Claim Amount Verified | 500000 TND |
      | Approval Authority | Claims Manager |
      | Approval Date | 2026-04-14 |
    Then claim is approved:
      | Approved Amount | 500000 TND |
      | Payment Method | Bank transfer to beneficiary account |
      | Payment Date | 2026-04-15 |
      | Reference Number | PAY-CLM-2026-001 |
    And payment is processed
    And beneficiary receives funds
    And claim is marked "Paid"

  @high @bc-insurance-010 @claim
  Scenario: Reject insurance claim with explanation
    Given claim "CLM-HEALTH-2026-001" for surgical procedure
    When claim review reveals issue:
      | Issue Found | Procedure is on exclusion list (cosmetic surgery) |
      | Policy Terms | Cosmetic procedures explicitly excluded |
      | Claim Amount | 8000 TND |
    Then claim is rejected:
      | Rejection Reason | Service not covered under policy |
      | Denial Letter | Sent to claimant |
      | Appeal Option | Available within 30 days |
      | Denial Date | 2026-04-20 |
    And claimant can appeal decision

  @high @bc-insurance-011 @claim
  Scenario: Handle insurance claim appeal
    Given rejected claim "CLM-HEALTH-2026-001"
    When claimant files appeal on "2026-05-07":
      | Appeal Reason | Procedure was medically necessary, not cosmetic |
      | Additional Evidence | Doctor's medical recommendation letter |
      | Appeal Amount | Original 8000 TND |
    Then appeal is reviewed:
      | Appeal Reviewer | Senior Claims Manager (different from original) |
      | Review Findings | Evidence suggests medical necessity |
      | Appeal Decision | Partially approved - 70% of claim |
      | Appeal Payment | 5600 TND |
    And payment is made to claimant

  @high @bc-insurance-012 @claim
  Scenario: Process partial insurance claim approval
    Given health insurance claim for hospitalization
    When claim is reviewed:
      | Claimed Amount | 50000 TND |
      | Hospital Bills Submitted | 50000 TND |
      | Policy Limit per Incident | 40000 TND |
      | Amount Exceeding Limit | 10000 TND |
    Then partial claim approval is processed:
      | Approved Amount | 40000 TND (within policy limit) |
      | Non-Covered Amount | 10000 TND (patient responsibility) |
      | Deductible Applied | 500 TND (deducted from approved amount) |
      | Final Payment | 39500 TND |

  # Bancassurance Products
  @critical @bc-insurance-013 @bancassurance
  Scenario: Launch new bancassurance product
    Given partnership with insurance company "AXA Tunisia"
    When new bancassurance product is launched:
      | Product Name | Wealth Protection Plan |
      | Product Type | Linked insurance + investment |
      | Target Segment | Retail/High Net Worth |
      | Distribution Channel | Bank branches + online |
      | Product Launch Date | 2026-04-07 |
      | Initial Premium | 50000 TND minimum |
      | Annual Premium | 10000+ TND flexible |
      | Investment Options | Equity Fund, Fixed Income, Balanced |
      | Mortality Charge | 0.5% p.a. |
      | Expense Charge | 1.5% p.a. |
    Then bancassurance product is activated
    And banking system is configured for sales
    And staff training is completed
    And customer marketing begins

  @high @bc-insurance-014 @bancassurance
  Scenario: Cross-sell bancassurance to deposit customer
    Given retail customer with savings account "SAV-2026-001"
    And account balance "500000 TND"
    When relationship manager identifies cross-sell opportunity:
      | Customer Profile | High balance, stable income, age 45 |
      | Recommended Product | Wealth Protection Plan |
      | Benefit Proposition | Insurance + Investment returns |
    Then bancassurance product is presented:
      | Proposal Date | 2026-04-15 |
      | Customer Interest | Yes |
      | Premium Amount | 50000 TND initial |
      | Investment Fund | Balanced Fund (60% equity, 40% fixed income) |
    And customer completes application
    And policy is issued

  @high @bc-insurance-015 @bancassurance
  Scenario: Monitor bancassurance product performance
    Given Wealth Protection Plan launched 3 months ago
    When quarterly performance review occurs:
      | Policies Issued | 250 policies |
      | Total Premium Collected | 12500000 TND |
      | Average Policy Value | 50000 TND |
      | Customer Satisfaction | 4.5/5.0 |
      | Lapse Rate | 2% (low, acceptable) |
      | Cross-sell Ratio | 15% of new customers |
      | Profitability | On track (positive margins) |
    Then product performance is strong
    And expansion of distribution is planned
    And additional product variants are considered

  # Insurance Commissions
  @critical @bc-insurance-016 @commission
  Scenario: Calculate agent commission for policy sale
    Given insurance agent "Ali Hassan" sells Wealth Protection policy
    And policy premium "50000 TND" (initial)
    When commission is calculated:
      | Commission Type | First Year Commission |
      | Commission Rate | 15% (contractual) |
      | Commission Amount | 7500 TND |
      | Agent Tier | Tier 2 (mid-level) |
      | Bonus Applicability | Yes (if 10+ policies in quarter) |
      | Bonus Amount | 2000 TND (incentive) |
      | Total Commission Payable | 9500 TND |
    Then commission is recorded with code "COM-2026-001"
    And commission status is "calculated"
    And awaiting approval

  @high @bc-insurance-017 @commission
  Scenario: Process insurance commission payment
    Given calculated commission "COM-2026-001" for "9500 TND"
    When commission is approved and due for payment:
      | Commission Amount | 9500 TND |
      | Withholding Tax Rate | 20% (taxation) |
      | Withholding Tax Amount | 1900 TND |
      | Net Commission Payable | 7600 TND |
      | Payment Method | Bank transfer |
      | Payment Date | 2026-05-07 |
    Then commission is paid to agent
    And tax documentation is generated
    And commission ledger is updated

  @high @bc-insurance-018 @commission
  Scenario: Monitor agent commission performance
    Given insurance agents with multiple policies
    When quarterly commission review occurs:
      | Top Performer | Agent A - 50 policies, 75000 TND commission |
      | Mid Performer | Agent B - 30 policies, 42000 TND commission |
      | Bottom Performer | Agent C - 5 policies, 6000 TND commission |
      | Team Total | 100 policies, 150000 TND commission |
    Then performance is tracked and reported
    And high performers are recognized
    And performance incentives are adjusted

  # Insurance Underwriting
  @high @bc-insurance-019 @underwriting
  Scenario: Complete medical underwriting for life policy
    Given customer applying for life insurance policy
    When medical underwriting is initiated:
      | Medical Exam Required | Yes (sum assured > 100000 TND) |
      | Exam Type | Full medical examination |
      | Exam Components | Blood test, BP check, ECG, medical history |
      | Exam Completed Date | 2026-04-20 |
    Then underwriting findings are reviewed:
      | Health Status | Good (no pre-existing conditions) |
      | Exam Result | Approved |
      | Underwriting Decision | Full cover at standard rate |
      | Approval Date | 2026-04-22 |
    And policy is issued at standard premium rate

  @high @bc-insurance-020 @underwriting
  Scenario: Apply loading/surcharge for health risk
    Given underwriting review for life policy applicant
    When medical examination reveals borderline condition:
      | Condition | Pre-diabetic state |
      | Risk Impact | Increased mortality risk |
      | Standard Premium | 50000 TND |
      | Loading Applied | 25% |
      | Loaded Premium | 62500 TND |
      | Underwriting Decision | Approved with loading |
    Then policy is issued with loaded premium
    And applicant accepts terms
    And policy document notes the loading reason
