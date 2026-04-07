Feature: Trade Finance (P1-BC4)
  As a trade finance officer
  I want to manage letters of credit, guarantees, and collections
  So that the bank can support cross-border and domestic trade

  Background:
    Given the system is initialized
    And I am authenticated as "trade_finance_officer"
    And trade finance module is operational

  # Letters of Credit
  @critical @bc-trade-001 @lc
  Scenario: Issue letter of credit for import
    Given importer customer "cust-trade-001" and exporter in foreign country
    When letter of credit is requested:
      | LC Amount | 500000 USD |
      | LC Type | Sight LC |
      | Applicant | Importer (Tunisia) |
      | Beneficiary | Exporter (Germany) |
      | Goods | Industrial machinery |
      | Incoterm | CIF Tunis |
      | Documents Required | Invoice, Packing list, B/L, Certificate of Origin |
      | Expiry Date | 120 days from issue |
      | Presentation Deadline | 21 days after shipment |
    Then LC is issued with code "LC-2026-001"
    And advising bank is selected (beneficiary's bank)
    And LC is transmitted to beneficiary via SWIFT
    And applicant is charged LC issuance commission

  @high @bc-trade-002 @lc
  Scenario: Add amendment to letter of credit
    Given issued LC "LC-2026-001" with amount "500000 USD"
    When importer requests amendment:
      | Amendment Type | Increase amount |
      | Old Amount | 500000 USD |
      | New Amount | 600000 USD |
      | New Expiry | Extended to 150 days |
      | Beneficiary Notification | Required |
    Then amendment is issued with code "AM-LC-2026-001"
    And amendment is sent to advising bank
    And amendment fee is charged
    And both importer and exporter must accept amendment

  @high @bc-trade-003 @lc
  Scenario: Process LC presentation with documents
    Given LC "LC-2026-001" issued for machinery import
    And shipment has occurred
    When exporter submits documents:
      | Invoice | Submitted |
      | Packing List | Submitted |
      | Bill of Lading | Submitted |
      | Certificate of Origin | Submitted |
      | Insurance Certificate | Submitted |
      | Quality Inspection | Submitted |
    Then document review is performed
    And system checks for discrepancies
    And documents are compliant and presentation is accepted
    And payment is processed per LC terms

  @high @bc-trade-004 @lc
  Scenario: Handle LC presentation discrepancy
    Given LC presentation submitted with documents
    When document review identifies discrepancy:
      | Issue | Invoice date is earlier than B/L date |
      | Severity | Minor technical discrepancy |
      | Type | Date inconsistency |
    Then discrepancy is reported to applicant
    And applicant is given option to:
      | Accept discrepancy with waiver | Or |
      | Request from exporter to correct | Or |
      | Reject presentation |
    And final decision is documented

  @critical @bc-trade-005 @lc
  Scenario: Make payment under letter of credit
    Given LC presentation accepted and compliant
    And LC terms: Sight payment
    When payment is due
    Then LC payment is processed:
      | Presentation Amount | 500000 USD |
      | Exchange Rate | 3.10 TND/USD |
      | Payment in TND | 1550000 TND |
      | Payment Method | SWIFT transfer to exporter's bank |
      | Charges | Recovered from applicant's account |
    And payment confirmation is sent to exporter
    And payment is recorded in trade finance system

  @high @bc-trade-006 @lc
  Scenario: Process usance LC acceptance and payment
    Given usance LC with 90-day tenor
    And documents presented and accepted
    When acceptance date is "2026-04-07"
    Then acceptance is recorded:
      | Acceptance Date | 2026-04-07 |
      | Maturity Date | 2026-07-06 (90 days) |
      | Payment Date | 2026-07-06 |
      | Discount Rate (if discounted) | 5.5% p.a. |
    And LC becomes negotiable instrument (if authorized)
    And exporter can discount for early funding

  @high @bc-trade-007 @lc
  Scenario: Monitor LC expiry and closure
    Given LC "LC-2026-001" with expiry date "2026-07-06"
    When expiry date approaches
    And no more presentations are expected
    Then LC is marked for closure:
      | Final Status | Expired |
      | Last Action Date | 2026-07-06 |
      | Total Presentations | 1 |
      | Total Amount Paid | 500000 USD |
    And LC record is archived
    And statistics are recorded

  # Bank Guarantees
  @critical @bc-trade-008 @guarantee
  Scenario: Issue bid bond guarantee for tender
    Given contractor bidding on government contract
    When bank guarantee is requested:
      | Guarantee Type | Bid Bond |
      | Principal (Contractor) | Contractor Corp |
      | Beneficiary (Project Owner) | Ministry of Infrastructure |
      | Tender Reference | TENDER-2026-05-001 |
      | Contract Value | 5000000 TND |
      | Guarantee Amount | 500000 TND (10% of contract) |
      | Validity | 60 days |
      | Claim Condition | On-demand |
    Then bid bond guarantee is issued with code "BG-BID-2026-001"
    And guarantee is sent to beneficiary
    And principal receives guarantee document for tender submission

  @high @bc-trade-009 @guarantee
  Scenario: Issue performance bond guarantee
    Given contractor awarded contract for "5000000 TND"
    When performance bond is requested:
      | Guarantee Type | Performance Bond |
      | Principal | Contractor Corp |
      | Beneficiary | Project Owner |
      | Contract Value | 5000000 TND |
      | Guarantee Amount | 1000000 TND (20% of contract) |
      | Validity | Until contract completion + 30 days |
      | Claim Condition | Performance failure |
      | Contract Completion Date | 2027-04-07 |
    Then performance bond is issued with code "BG-PERF-2026-001"
    And project owner receives guarantee
    And contractor is protected against performance claims

  @high @bc-trade-010 @guarantee
  Scenario: Process guarantee claim
    Given performance bond guarantee "BG-PERF-2026-001"
    When project owner files claim:
      | Claim Date | 2027-03-15 |
      | Claim Reason | Work quality deficiency - 300000 TND to remediate |
      | Evidence | Inspection report + remediation quote |
      | Claim Amount | 300000 TND |
      | Claim Code | CLAIM-2027-001 |
    Then claim is reviewed by bank:
      | Document Review | Complete |
      | Claim Terms Compliance | Verified |
      | Recommendation | Approve 300000 TND |
    And claim is paid to beneficiary
    And contractor is notified
    And subrogation may be pursued against contractor

  @high @bc-trade-011 @guarantee
  Scenario: Handle guarantee amendment and cancellation
    Given bid bond guarantee "BG-BID-2026-001" with 60-day validity
    When contractor wins tender but requests cancellation:
      | Original Validity | 60 days |
      | Request Date | 45 days from issue |
      | Reason | Bid bond no longer needed (contract awarded) |
    Then cancellation is processed:
      | Guarantee Amount | Released to contractor |
      | Cancellation Fee | Charged for early release |
      | Status | Cancelled |
    And guarantee obligation is terminated

  # Documentary Collections
  @critical @bc-trade-012 @collection
  Scenario: Initiate documentary collection
    Given exporter selling goods to importer
    When exporter initiates documentary collection:
      | Collection Type | Documents Against Payment (D/P) |
      | Remitter | Exporter (Exporting Bank) |
      | Drawee | Importer (Importing Bank) |
      | Collection Amount | 250000 USD |
      | Invoice Reference | INV-2026-04-001 |
      | Documents | Bill of Lading, Invoice, Insurance |
      | Collection Terms | At sight |
    Then collection order is created with code "COL-2026-001"
    And documents are received and verified
    And collection is sent to collecting bank

  @high @bc-trade-013 @collection
  Scenario: Process collection payment at sight
    Given documentary collection "COL-2026-001" with sight terms
    When documents arrive at importer's bank
    And payment is made by importer
    Then collection is settled:
      | Payment Date | 2026-04-20 |
      | Payment Amount | 250000 USD |
      | Collection Charges | Recovered from proceeds |
      | Net Amount to Exporter | 249500 USD |
      | Remittance | Sent to exporting bank |
    And goods are released to importer
    And transaction is closed

  @high @bc-trade-014 @collection
  Scenario: Handle collection document delivery against acceptance
    Given documentary collection with acceptance terms (D/A)
    When documents are presented at importer's bank
    And importer accepts the bill of exchange
    Then acceptance is recorded:
      | Acceptance Date | 2026-04-20 |
      | Maturity Date | 90 days from acceptance (2026-07-19) |
      | Bill Status | Accepted, now negotiable |
    And exporter receives acceptance notification
    And exporter can discount bill for early cash

  @high @bc-trade-015 @collection
  Scenario: Handle collection non-payment or rejection
    Given documentary collection awaiting payment
    When importer cannot or refuses to pay
    Then non-payment is processed:
      | Days Awaiting Payment | 10 days past due date |
      | Action | Documents are returned to exporter |
      | Return Method | SWIFT with scanned documents |
      | Notes | Non-payment documented |
    And exporter can pursue other remedies
    And collection fee is charged for handling

  # Trade Finance Limits
  @critical @bc-trade-016 @limit
  Scenario: Establish trade finance facility limit
    Given approved corporate customer "Corp X"
    When trade finance limit is set:
      | Limit Code | TF-LIM-2026-001 |
      | Limit Type | Combined (LC + Guarantee + Collection) |
      | Total Facility | 2000000 USD |
      | Sub-limits | LC: 1200000 USD, Guarantee: 800000 USD |
      | Currency | USD |
      | Tenor Limit | Up to 180 days |
      | Effective Period | 2026-04-07 to 2027-04-06 (1 year) |
      | Collateral Required | 30% (600000 USD in securities) |
    Then facility limit is established
    And customer receives facility letter
    And collateral is pledged or secured

  @high @bc-trade-017 @limit
  Scenario: Monitor trade finance limit utilization
    Given TF facility limit "TF-LIM-2026-001" for "2000000 USD"
    When utilization monitoring occurs:
      | LC Outstanding | 800000 USD |
      | Guarantees Outstanding | 600000 USD |
      | Collections in Process | 200000 USD |
      | Total Utilization | 1600000 USD (80%) |
      | Available Balance | 400000 USD |
    Then utilization report is generated
    And customer is within limits
    And approaching limit threshold (80%) triggers review

  @high @bc-trade-018 @limit
  Scenario: Adjust limit due to customer requirement
    Given TF facility limit "TF-LIM-2026-001" for "2000000 USD" (80% utilized)
    When customer requests increase for large import order:
      | Additional Requirement | 500000 USD LC |
      | Current Available | 400000 USD |
      | Requested Increase | 200000 USD (new limit: 2200000 USD) |
      | Reason | Major import order from new supplier |
    Then limit increase is reviewed and approved
    And new limit becomes "2200000 USD"
    And customer is notified of new availability
    And facility letter is updated

  # Trade Finance Reporting
  @high @bc-trade-019 @reporting
  Scenario: Generate monthly trade finance report
    Given all trade finance transactions for March 2026
    When monthly report is generated:
      | LC Issued | 5 LCs for total 3500000 TND |
      | LC Amendments | 2 amendments processed |
      | Guarantees Issued | 3 guarantees for total 1200000 TND |
      | Collections Processed | 8 collections for total 2400000 TND |
      | Total Trade Volume | 7100000 TND |
      | Commission Income | 125000 TND |
    Then report is compiled and reviewed
    And report is submitted to management
    And statistics are tracked for KPIs

  @high @bc-trade-020 @reporting
  Scenario: Monitor trade finance portfolio credit exposure
    Given all outstanding LCs, guarantees, and collections
    When credit exposure analysis is performed:
      | Total Outstanding LC | 8000000 TND |
      | Total Outstanding Guarantees | 4500000 TND |
      | Country Concentration | Egypt 25%, Libya 20%, Germany 15%, Others 40% |
      | Tenor Distribution | 0-30 days: 30%, 30-90: 45%, 90-180: 25% |
      | Risk Rating | Diversified, Low concentration risk |
    Then exposure is monitored for limits compliance
    And concentration limits are maintained
    And country risk is monitored and reported
