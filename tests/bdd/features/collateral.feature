Feature: Collateral Management (P1-BC1)
  As a credit officer
  I want to manage collateral for loans and facilities
  So that the bank can secure credit exposures and monitor LTV compliance

  Background:
    Given the system is initialized
    And I am authenticated as "credit_officer"
    And collateral module is operational

  # Collateral Creation and Registration
  @critical @bc-collateral-001 @collateral
  Scenario: Register a real estate collateral
    Given a verified customer with id "cust-rt-001" and name "Ahmed Hassan"
    When I register a real estate collateral with:
      | Property Type | Residential villa |
      | Location | Tunis, Ben Arous |
      | Appraised Value | 500000 TND |
      | Appraisal Method | Professional appraisal |
      | Appraiser License | LIC-2026-001 |
    Then the collateral is created with code "COLL-RE-001"
    And collateral status is "active"
    And insurance requirement flag is set

  @high @bc-collateral-002 @collateral
  Scenario: Register securities as collateral
    Given a corporate customer with id "cust-sec-001"
    When I register securities collateral with:
      | Security Type | Government bonds |
      | Securities Count | 500 bonds |
      | Face Value | 1000 TND each |
      | Market Value | 520000 TND |
      | Valuation Date | 2026-04-07 |
    Then the collateral is registered with code "COLL-SEC-001"
    And collateral can be valued using market comparable method

  @high @bc-collateral-003 @collateral
  Scenario: Register movable assets as collateral
    Given a customer with id "cust-mov-001"
    When I register movable assets with:
      | Asset Description | Construction equipment |
      | Asset Type | Excavator + Bulldozer |
      | Valuation Amount | 80000 TND |
      | Registration | Yes, with Ministry |
    Then movable asset collateral is created with code "COLL-MOV-001"
    And registration number is captured

  @medium @bc-collateral-004 @collateral
  Scenario: Register guarantee as collateral (counter-guarantee)
    Given a bank partner with relationship code "PARTNER-2026-001"
    When I register a bank guarantee as collateral with:
      | Guarantee Issuer | Bank ABC |
      | Guarantee Amount | 150000 TND |
      | Expiry Date | 2026-12-31 |
    Then counter-guarantee is registered with code "COLL-GUAR-001"

  # Collateral Valuation and Revaluation
  @critical @bc-collateral-005 @collateral
  Scenario: Perform initial collateral appraisal
    Given registered collateral with code "COLL-RE-001" and initial value "500000 TND"
    When I trigger initial appraisal with:
      | Appraiser | Mohamed Zinet |
      | Appraisal Method | Professional building appraisal |
      | Appraisal Report | Attached PDF document |
    Then appraisal is recorded with date "2026-04-07"
    And initial valuation is confirmed as "500000 TND"
    And collateral is ready for pledge

  @critical @bc-collateral-006 @collateral
  Scenario: Revalue collateral for LTV monitoring
    Given pledged collateral with code "COLL-RE-001" valued at "500000 TND"
    And collateral pledged for 6 months
    When I trigger annual revaluation on "2026-04-07"
    Then new valuation is recorded as "480000 TND"
    And valuation variance is "-20000 TND" (-4%)
    And valuation status is "current"
    And historical valuation record is archived

  @high @bc-collateral-007 @collateral
  Scenario: Apply valuation haircut for risk mitigation
    Given collateral valued at "100000 TND"
    When I apply 15% haircut due to market volatility
    Then effective collateral value becomes "85000 TND"
    And haircut reason is "market_volatility"
    And haircut percentage is recorded in LTV calculation

  @high @bc-collateral-008 @collateral
  Scenario: Handle disputed collateral valuation
    Given collateral with recent appraisal
    When customer disputes the valuation as "overvalued"
    Then collateral valuation status changes to "disputed"
    And system notifies independent appraiser
    And re-appraisal is scheduled within 5 working days

  # Collateral Pledge and Allocation
  @critical @bc-collateral-009 @collateral
  Scenario: Pledge collateral to a new loan
    Given collateral "COLL-RE-001" valued at "500000 TND"
    And customer account with approved loan "LOAN-2026-001" for "400000 TND"
    When I pledge collateral to the loan
    Then collateral allocation is created with:
      | Allocated Amount | 400000 TND |
      | Facility Code | LOAN-2026-001 |
      | Allocation Status | Active |
      | Priority | 1st position |
    And collateral status becomes "pledged"

  @high @bc-collateral-010 @collateral
  Scenario: Allocate collateral to multiple facilities
    Given collateral "COLL-RE-001" valued at "600000 TND"
    When I allocate to multiple facilities:
      | Facility Code | Amount | Priority |
      | LOAN-001 | 300000 TND | 1 |
      | LOAN-002 | 200000 TND | 2 |
      | OVERDRAFT-001 | 100000 TND | 3 |
    Then all allocations are recorded with correct priority
    And total allocation equals "600000 TND" (100% utilized)

  @medium @bc-collateral-011 @collateral
  Scenario: Partially release pledged collateral
    Given pledged collateral with allocations totaling "500000 TND"
    And loan with outstanding balance "200000 TND"
    When customer requests partial release of "100000 TND"
    And bank approves the release
    Then partial release is processed:
      | Released Amount | 100000 TND |
      | Remaining Pledge | 400000 TND |
      | Status | Partial Release |
    And release date is recorded

  @critical @bc-collateral-012 @collateral
  Scenario: Release collateral upon loan repayment
    Given pledged collateral "COLL-RE-001" valued at "500000 TND"
    And associated loan with outstanding balance "0 TND"
    When I initiate collateral release
    Then release is processed:
      | Release Date | 2026-04-07 |
      | Release Reason | Loan fully repaid |
      | Collateral Status | Released |
    And collateral can be pledged to other facilities

  # LTV (Loan-to-Value) Calculation
  @critical @bc-collateral-013 @collateral
  Scenario: Calculate LTV for new loan
    Given collateral "COLL-RE-001" valued at "500000 TND"
    And proposed loan amount "400000 TND"
    When I calculate LTV ratio
    Then LTV calculation is recorded:
      | Loan Amount | 400000 TND |
      | Collateral Value | 500000 TND |
      | LTV Ratio | 80% |
      | LTV Threshold | 80% |
      | Compliant | Yes |
    And loan is approved within LTV limits

  @high @bc-collateral-014 @collateral
  Scenario: Monitor LTV breach after collateral revaluation
    Given loan with LTV "75%" and threshold "80%"
    When collateral is revalued down from "500000" to "480000 TND"
    Then new LTV becomes "83.33%"
    And LTV is now in breach status
    And breach notification is sent to risk management
    And customer is required to provide additional collateral

  @high @bc-collateral-015 @collateral
  Scenario: Apply haircut to improve LTV compliance
    Given loan in LTV breach with ratio "85%" (threshold 80%)
    When I apply 10% haircut:
      | Original Collateral Value | 470000 TND |
      | Haircut Percentage | 10% |
      | Haircut Amount | 47000 TND |
    Then haircut is recorded in LTV calculation
    And effective LTV becomes "85%" (still above threshold)
    And remediation plan is required

  @critical @bc-collateral-016 @collateral
  Scenario: Calculate margin of safety
    Given collateral valued at "500000 TND"
    And loan amount "400000 TND"
    When I calculate margin of safety
    Then margin of safety = "(500000 - 400000) / 500000 * 100 = 20%"
    And margin is above minimum threshold of 10%
    And facility approval proceeds

  @medium @bc-collateral-017 @collateral
  Scenario: Monitor LTV on declining balance loans
    Given loan with declining balance: Month 1: "400000 TND", Month 6: "300000 TND", Month 12: "200000 TND"
    When monthly LTV recalculation occurs
    Then LTV improves each month as loan balance decreases
    And collateral release consideration becomes applicable at Month 12

  # Insurance Requirements
  @critical @bc-collateral-018 @collateral
  Scenario: Enforce insurance requirement for real estate
    Given pledged real estate collateral valued at "500000 TND"
    And insurance requirement is mandatory
    When I set insurance requirements:
      | Insurance Type | Comprehensive property |
      | Coverage Amount | 500000 TND |
      | Insured Name | Bank + Customer (mortgagee) |
    Then insurance requirement status is "pending_compliance"
    And customer is notified to provide insurance policy within 15 days
    And system blocks collateral release until insurance is confirmed

  @high @bc-collateral-019 @collateral
  Scenario: Verify insurance policy expiry
    Given pledged collateral with active insurance policy
    And insurance expiry date "2026-06-30"
    When I run insurance expiry monitoring
    Then system identifies policies expiring within 90 days
    And customer receives renewal reminder 60 days before expiry
    And collateral status is marked "insurance_expiry_pending"

  @high @bc-collateral-020 @collateral
  Scenario: Handle insurance claim on pledged collateral
    Given pledged collateral with active insurance policy
    When property damage occurs and insurance claim is filed
    Then claim notification is captured in collateral record
    And claim amount is tracked against collateral value
    And replacement or cash settlement is monitored
    And collateral status may be updated to "under_restoration" or "depreciated"

  # Collateral Documentation
  @critical @bc-collateral-021 @collateral
  Scenario: Verify complete collateral documentation
    Given collateral registration in progress
    When I verify documentation checklist:
      | Document | Required | Status |
      | Title Deed / Ownership Proof | Yes | Received |
      | Professional Appraisal Report | Yes | Received |
      | Insurance Policy | Yes | Pending |
      | Registration Certificate | Yes | Received |
      | Customer ID & Address | Yes | Verified |
    Then system confirms documentation is "95% complete" (insurance pending)
    And collateral can proceed with pledge but with insurance condition

  @high @bc-collateral-022 @collateral
  Scenario: Request missing collateral documentation
    Given collateral documentation checklist incomplete
    When documentation verification fails due to "missing_appraisal_report"
    Then system generates documentation request:
      | Requested Document | Professional Appraisal Report |
      | Requested From | Customer |
      | Due Date | 2026-04-21 |
      | Days Given | 14 days |
    And collateral pledge is placed on hold
    And customer is notified via email and SMS

  # Collateral Exception Handling
  @high @bc-collateral-023 @collateral
  Scenario: Handle collateral depreciation
    Given pledged collateral valued at "500000 TND"
    When market conditions deteriorate and revaluation shows "420000 TND"
    Then depreciation is recorded as "-80000 TND" (16%)
    And LTV re-calculation is triggered
    And if LTV breached, remediation is required
    And adjustment may include loan restructuring or additional collateral

  @medium @bc-collateral-024 @collateral
  Scenario: Force liquidation of collateral in default
    Given loan in severe default (>90 days)
    And pledged collateral with clear title
    When collateral enforcement is initiated
    Then collateral status changes to "in_liquidation"
    And liquidation process is logged with dates and actions
    And proceeds are applied to outstanding loan balance
    And surplus (if any) is returned to customer
