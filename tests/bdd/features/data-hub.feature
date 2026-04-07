Feature: Data Hub Master Data Management (P2-BC2)
  As a data steward and compliance officer
  I want to manage master data quality, lineage, reconciliation, and governance
  So that the organization maintains reliable, consistent, and compliant data across all systems

  Background:
    Given the system is initialized
    And I am authenticated as "data_steward"
    And data hub module is operational

  # Master Data Entity Management
  @critical @bc-data-hub-001 @data-hub
  Scenario: Register a new customer as master data entity
    Given customer onboarding for "Ahmed Hassan"
    When I register the customer as a master data entity with:
      | Entity Type | customer |
      | System of Record | Customer CRM |
      | Data Quality Target | 95% |
    Then data entity is created with code "DE-CUST-001"
    And entity status is "active"
    And primary system of record is "Customer CRM"
    And integration status is "pending"

  @high @bc-data-hub-002 @data-hub
  Scenario: Register a counterparty as master data entity
    Given new counterparty "Bank of Tunisia"
    When I register as master data entity with:
      | Entity Type | counterparty |
      | System of Record | Risk System |
    Then data entity is created with code "DE-CNTRPTY-001"
    And entity status is "active"

  @high @bc-data-hub-003 @data-hub
  Scenario: Register a security product as master data entity
    Given security "Orange Tunisia Shares"
    When I register as master data entity with:
      | Entity Type | security |
      | System of Record | Trading System |
      | ISIN | TN0000000000 |
    Then data entity is created with code "DE-SEC-001"
    And entity status is "active"

  @medium @bc-data-hub-004 @data-hub
  Scenario: Update master data entity version
    Given registered data entity "DE-CUST-001" at version 1
    When customer information is updated:
      | Field | Phone Number |
      | Old Value | +216 21 123 456 |
      | New Value | +216 22 654 321 |
    Then version number increments to 2
    And update is recorded with timestamp
    And previous version is archived
    And change reason is documented

  # Data Quality Rules and Validation
  @critical @bc-data-hub-005 @data-hub
  Scenario: Create data quality rule for completeness
    When I define a quality rule with:
      | Rule Name | Customer Email Required |
      | Entity Type | customer |
      | Rule Type | completeness |
      | Severity | critical |
      | Expression | email IS NOT NULL |
    Then quality rule is created with code "DQR-001"
    And rule status is "active"
    And violations tracked from rule creation

  @critical @bc-data-hub-006 @data-hub
  Scenario: Enforce uniqueness rule for ISIN
    When I define a quality rule with:
      | Rule Name | ISIN Uniqueness |
      | Entity Type | security |
      | Rule Type | uniqueness |
      | Expression | ISIN must be unique |
    Then quality rule is created with code "DQR-002"
    And rule is enforced on all security registrations

  @high @bc-data-hub-007 @data-hub
  Scenario: Execute data quality validation on entity
    Given data entity "DE-CUST-001" registered
    When quality validation is triggered
    Then all applicable rules are executed
    And validation status is "validated"
    And data quality score is "87.5%"
    And completeness score is "90%"
    And consistency score is "85%"
    And validation results are recorded

  @high @bc-data-hub-008 @data-hub
  Scenario: Generate data quality violations report
    Given multiple entities with quality issues
    When violations report is generated for period "2026-04-01 to 2026-04-07"
    Then violations are categorized by severity:
      | Critical | 5 violations |
      | High | 12 violations |
      | Medium | 23 violations |
      | Low | 41 violations |
    And remediation steps are recommended
    And violation owner is assigned
    And SLA deadline is set

  @high @bc-data-hub-009 @data-hub
  Scenario: Remediate data quality violation
    Given violation "Missing phone number for customer"
    When phone number is provided: "+216 21 123 456"
    Then violation status changes to "resolved"
    And remediation date is recorded
    And quality rule re-validates entity
    And data quality score improves

  @medium @bc-data-hub-010 @data-hub
  Scenario: Exempt entity from specific quality rule
    Given strict quality rule "Annual Audit Required"
    When customer requests exemption for "2026"
    Then exemption request is created with code "EXP-2026-001"
    And exemption status is "pending_approval"
    And exemption approval is routed to manager

  @medium @bc-data-hub-011 @data-hub
  Scenario: Approve quality rule exemption
    Given pending exemption request "EXP-2026-001"
    When manager approves exemption with reason "Startup company exception"
    Then exemption status becomes "approved"
    And approval date is recorded
    And entity is excluded from rule for duration
    And exemption audit trail is created

  # Data Lineage and Flow
  @critical @bc-data-hub-012 @data-hub
  Scenario: Create data lineage mapping from source to target
    When I define data lineage:
      | Source System | Customer CRM |
      | Source Entity | customer |
      | Target System | Core Banking |
      | Target Entity | customer |
      | Field Mapping | name → customer_name |
      | Transformation | Trim whitespace, uppercase |
    Then lineage is created with code "DL-CUST-001"
    And lineage status is "active"
    And transformation logic is documented

  @high @bc-data-hub-013 @data-hub
  Scenario: Track data flow frequency and latency
    Given active lineage "DL-CUST-001"
    When data flows from source to target "50 times in last hour"
    Then flow statistics are recorded:
      | Flow Count | 50 |
      | Average Latency | 245 ms |
      | Max Latency | 890 ms |
      | Error Count | 0 |
      | Success Rate | 100% |

  @high @bc-data-hub-014 @data-hub
  Scenario: Identify data lineage impact on downstream entities
    Given change request: "Add new field to Customer entity"
    When lineage impact analysis is triggered
    Then downstream impact includes:
      | System | Account Management |
      | Entities Affected | 5 |
      | Data Flows Impacted | 8 |
      | Rules Affected | 12 |
    And impact assessment is documented
    And change approval workflow is initiated

  # Data Reconciliation
  @critical @bc-data-hub-015 @data-hub
  Scenario: Execute monthly balance reconciliation
    When balance reconciliation is run for "April 2026"
    Then reconciliation record is created with code "RECON-2026-04-01"
    And source system records: "50,000 records"
    And target system records: "49,998 records"
    And matched records: "49,998"
    And unmatched source: "2 records"
    And unmatched target: "0 records"
    And reconciliation status is "matched"

  @high @bc-data-hub-016 @data-hub
  Scenario: Handle reconciliation discrepancies
    Given unmatched record in reconciliation:
      | Source Account | ACC-2026-001 |
      | Balance | 100000 TND |
      | Target | Missing |
    When discrepancy is investigated
    Then discrepancy is recorded in reconciliation:
      | Discrepancy Code | DISC-2026-001 |
      | Type | Missing account in target |
      | Amount | 100000 TND |
    And issue is routed to operations team

  @high @bc-data-hub-017 @data-hub
  Scenario: Approve reconciliation results
    Given completed reconciliation with all discrepancies resolved
    When reconciliation owner approves
    Then reconciliation status becomes "approved"
    And approval date is recorded
    And reconciliation summary is archived
    And next reconciliation is scheduled

  @high @bc-data-hub-018 @data-hub
  Scenario: Cross-system reconciliation for master data
    When master data reconciliation is run between:
      | System 1 | Customer CRM |
      | System 2 | Core Banking |
      | Entity | customer |
    Then reconciliation matches "99.2%" of records
    And 2 conflicting records are identified
    And system determines "source of truth"
    And conflict resolution workflow is initiated

  @medium @bc-data-hub-019 @data-hub
  Scenario: Generate reconciliation audit trail
    Given completed reconciliation "RECON-2026-04-01"
    When audit trail is requested
    Then audit report includes:
      | Reconciliation Date | 2026-04-07 |
      | Run Duration | 45 minutes |
      | Records Processed | 50000 |
      | Discrepancies Found | 2 |
      | Discrepancies Resolved | 2 |
      | Approval Status | Approved |
      | Approver | John Smith |
    And report is available for compliance review

  # Master Data Golden Records
  @critical @bc-data-hub-020 @data-hub
  Scenario: Create golden record for customer entity
    Given customer "Ahmed Hassan" with data from multiple sources:
      | CRM System | name: Ahmed Hassan, phone: +216 21 123 456 |
      | Banking System | name: A. Hassan, phone: +216 21 123 456 |
      | KYC System | name: Ahmed Hassan, email: ahmed@example.com |
    When golden record is created
    Then master record is created with code "MR-CUST-001"
    And canonical data merges all sources
    And record status is "active"
    And version is "1"

  @high @bc-data-hub-021 @data-hub
  Scenario: Update golden record with new version
    Given golden record "MR-CUST-001" version 1
    When customer updates phone number to "+216 22 654 321"
    Then new version "2" is created
    And effective_from_date is set to "2026-04-07"
    And previous version is marked "superseded"
    And version history is maintained
    And change reason is documented as "Customer update"

  @high @bc-data-hub-022 @data-hub
  Scenario: Govern golden record approval workflow
    Given draft golden record in "pending_approval" status
    When data governance steward reviews record
    Then record can be:
      | Action | Approved |
      | Reason | Complete and accurate |
    And status changes to "approved"
    And approval date and approver are recorded

  @medium @bc-data-hub-023 @data-hub
  Scenario: Retire old golden record version
    Given golden record "MR-CUST-001" version 1 superseded by version 2
    When version retention policy dictates archival
    Then version 1 status becomes "archived"
    And version is still accessible for audit
    And archival date is recorded

  # Data Governance Policies
  @critical @bc-data-hub-024 @data-hub
  Scenario: Create data ownership policy
    When I define governance policy:
      | Policy Name | Data Ownership for Customer Master |
      | Entity Type | customer |
      | Policy Type | ownership |
      | Policy Owner | CRM Manager |
      | Scope | All customer data in organization |
    Then governance policy is created with code "DGP-001"
    And policy status is "draft"

  @high @bc-data-hub-025 @data-hub
  Scenario: Approve and activate governance policy
    Given draft governance policy "DGP-001"
    When policy is reviewed and approved by governance committee
    Then policy status becomes "active"
    And effective date is set
    And enforcement rules are activated
    And compliance audit is scheduled

  @high @bc-data-hub-026 @data-hub
  Scenario: Enforce data retention policy
    Given policy: "Delete personal data 7 years after relationship end"
    When customer closes account on "2018-04-07"
    And 7 years pass (2025-04-07)
    Then automatic data deletion is triggered on "2025-04-07"
    And data is permanently deleted per GDPR
    And deletion audit trail is recorded

  @high @bc-data-hub-027 @data-hub
  Scenario: Audit governance policy compliance
    Given data governance policy "DGP-001" in effect for 12 months
    When annual compliance audit is triggered
    Then audit verifies:
      | Compliance Rate | 98.5% |
      | Non-Compliance Issues | 3 |
      | Corrective Actions | In Progress |
    And audit report is generated
    And findings are shared with management

  # Data Quality Dashboard and Metrics
  @high @bc-data-hub-028 @data-hub
  Scenario: Generate data quality scorecard
    When quality scorecard is generated for "April 2026"
    Then scorecard shows:
      | Overall Quality Score | 92.3% |
      | Completeness | 95% |
      | Accuracy | 90% |
      | Consistency | 89% |
      | Timeliness | 93% |
      | Uniqueness | 96% |
    And improvement areas are identified
    And trend comparison shows month-over-month
    And benchmarking against industry standards

  @high @bc-data-hub-029 @data-hub
  Scenario: Track data quality metrics over time
    Given quality metrics collected monthly for 12 months
    When trend analysis is performed
    Then trends show:
      | Trend Direction | Improving |
      | Q1 2026 Average | 88.5% |
      | Q2 2026 Average | 91.2% |
      | Improvement Rate | +2.7% |
    And root causes of improvement are identified
    And anomalies are flagged

  @medium @bc-data-hub-030 @data-hub
  Scenario: Alert on critical data quality threshold breach
    Given data quality rule with critical threshold "95%"
    When actual quality score drops to "92%"
    Then alert is generated with code "ALERT-DQ-001"
    And severity is "critical"
    And alert is escalated to data steward
    And remediation actions are recommended
    And stakeholders are notified
