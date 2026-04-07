Feature: AML Advanced Management (BC4) - goAML, Travel Rule, EDD, Screening
  As an AML analyst
  I want to manage advanced AML compliance including goAML reporting, travel rule, enhanced due diligence, and PEP screening
  So that the bank meets GAFI recommendations and regulatory requirements

  Background:
    Given the system is initialized
    And I am authenticated as "aml_analyst"
    And AML module is operational

  # goAML Reporting (BCT Requirement)
  @critical @fr-goaml-001 @aml @reporting
  Scenario: Prepare goAML submission to BCT
    Given suspicious transactions identified for October 2025
    When preparing monthly goAML report
    Then goAML XML structure includes:
      | Element |
      | Report metadata (bank ID, period, country) |
      | Entity information (originator, beneficiary) |
      | Transaction details (amount, date, narration) |
      | Reason for suspicion |
    And 25 suspicious transactions are included
    And integrity check validates all fields

  @critical @fr-goaml-002 @aml @reporting
  Scenario: Submit goAML to FIDES platform
    Given completed goAML report
    When submitting to FIDES (Central Bank platform)
    Then submission is encrypted and authenticated
    And transmission log is recorded with timestamp
    And BCT acknowledgment is captured
    And submission deadline (10th of month) is met

  @high @fr-goaml-003 @aml @reporting
  Scenario: Track goAML resubmission for corrections
    Given goAML submission with validation errors
    When correcting errors and resubmitting
    Then previous version is archived
    And correction reason is documented
    And resubmission within 3 days
    And audit trail shows all versions

  @high @fr-goaml-004 @aml @reporting
  Scenario: Generate goAML statistics report
    Given 12 months of goAML submissions
    When analyzing annual submission patterns
    Then statistics include:
      | Metric | Value |
      | Total SARs Filed | 285 |
      | Average SARs per Month | 23.75 |
      | SARs with Arrests | 12 |
      | SARs with Forfeitures | 8 |
      | Most Flagged Risk Type | Unusual Volume |
    And report is presented to compliance committee

  # Travel Rule (GAFI Rec. 16 - Virtual Asset Transfer)
  @critical @fr-travel-001 @aml @travelrule
  Scenario: Implement travel rule for fund transfers
    Given cross-border fund transfer over 3,000 USD
    When processing transfer
    Then originator information is collected:
      | Field |
      | Originator Name |
      | Originator Account Number |
      | Originator Address |
      | Beneficiary Name |
      | Beneficiary Account Number |
    And information is transmitted to beneficiary institution
    And receipt confirmation is obtained

  @critical @fr-travel-002 @aml @travelrule
  Scenario: Validate travel rule compliance
    Given incoming international transfer
    When receiving transfer from foreign bank
    Then originator information is captured
    And information completeness is verified (100% of required fields)
    And originator is checked against sanctions lists
    And transaction proceeds if compliant

  @high @fr-travel-003 @aml @travelrule
  Scenario: Reject transfer with incomplete originator information
    Given transfer with missing originator address
    When validating travel rule requirements
    Then transfer is rejected with error "IncompleteOriginatorInfo"
    And sending bank is notified
    And customer is asked to resubmit with complete information
    And rejection reason is logged

  @high @fr-travel-004 @aml @travelrule
  Scenario: Monitor travel rule compliance metrics
    Given monthly transfer statistics
    When analyzing travel rule compliance
    Then compliance rate = 99.2%
    And total transfers = 1,240
    And rejections = 10 (due to incomplete info)
    And report is submitted to compliance

  # Enhanced Due Diligence (EDD)
  @critical @fr-edd-001 @aml @duediligence
  Scenario: Trigger EDD for high-risk customers
    Given customer profile with:
      | Risk Factor |
      | PEP Status = True |
      | Beneficial Owner Country = High-Risk |
      | Transaction Volume = Unusual |
    When risk score exceeds 75 (threshold)
    Then EDD process is automatically triggered
    And EDD form is created
    And analyst is assigned

  @critical @fr-edd-002 @aml @duediligence
  Scenario: Conduct Enhanced Due Diligence investigation
    Given high-risk customer requires EDD
    When conducting EDD investigation
    Then analyst collects:
      | Information |
      | Source of funds explanation |
      | Beneficial ownership documentation |
      | Business purpose and transaction justification |
      | PEP relationship details |
      | Sanctions/adverse media search results |
    And investigation findings are documented
    And risk assessment is updated

  @high @fr-edd-003 @aml @duediligence
  Scenario: Escalate EDD to senior management
    Given EDD findings indicate suspected ML/TF
    When finalizing EDD
    Then case is escalated to AML Manager
    And SAR filing is recommended
    And customer relationship is reviewed
    And potential termination is considered

  @high @fr-edd-004 @aml @duediligence
  Scenario: Document EDD approval workflow
    Given completed EDD investigation
    When obtaining approval from AML Manager
    Then EDD is marked "Approved" or "NeedsRevision"
    And approval timestamp and approver are recorded
    And if approved, customer can continue with normal monitoring
    And if revision needed, analyst updates documentation

  @medium @fr-edd-005 @aml @duediligence
  Scenario: Periodic EDD refresh for ongoing relationships
    Given customer in EDD status for 12 months
    When conducting annual EDD refresh
    Then previous findings are reviewed
    And new risk assessment is performed
    And source of funds is re-verified
    And updated EDD is documented

  # Continuous PEP Screening
  @critical @fr-pep-001 @aml @screening
  Scenario: Daily PEP screening against database
    Given customer base of 50,000 customers
    When running daily PEP screening
    Then system checks against:
      | PEP Database |
      | BCT PEP list |
      | International PEP databases |
      | OFAC SDN list |
      | EU sanctions list |
    And 3 potential matches are identified
    And matches are scored (confidence %)

  @critical @fr-pep-002 @aml @screening
  Scenario: Evaluate and verify PEP matches
    Given 3 PEP matches identified
    When analyst reviews each match
    Then for Match 1 (95% confidence):
      | Status |
      | Exact name match |
      | Date of birth matches |
      | Action: Mark customer as Confirmed PEP |
    And for Match 2 (60% confidence):
      | Status |
      | Possible name variation |
      | Different country |
      | Action: Add to review queue for manual verification |
    And for Match 3 (30% confidence):
      | Status |
      | Action: Mark as false positive, add to whitelist |

  @high @fr-pep-003 @aml @screening
  Scenario: Alert on new PEP designation
    Given customer "Hassan Ben Ali" is currently marked as Non-PEP
    When PEP database is updated with new PEP list
    And Hassan is added as new PEP (minister designation)
    Then system alerts AML team immediately
    And customer relationship is escalated
    And EDD is initiated
    And pending transactions may be frozen

  @high @fr-pep-004 @aml @screening
  Scenario: Manage false positives in PEP screening
    Given 150 PEP matches processed
    When 18 matches are determined to be false positives
    Then false positives are added to "Whitelist"
    And whitelist is used in future screening
    And false positive rate = 12% (18/150)
    And report identifies systemic issues if rate > 10%

  @medium @fr-pep-005 @aml @screening
  Scenario: Generate PEP screening metrics report
    Given monthly PEP screening results
    When generating screening report
    Then report includes:
      | Metric | Value |
      | Transactions screened | 125,000 |
      | Matches identified | 150 |
      | Confirmed PEPs | 18 |
      | False positives | 18 |
      | New PEP alerts | 4 |
      | Hit rate | 0.14% |

  # AML Training and Certification
  @high @fr-aml-train-001 @aml @training
  Scenario: Assign mandatory AML training to staff
    Given new employee in customer-facing role
    When onboarding to bank
    Then AML/CFT training is assigned with:
      | Module |
      | GAFI Recommendations |
      | SAR Filing Procedures |
      | Sanctions Screening |
      | PEP Identification |
      | Suspicious Activity Recognition |
      | Customer Due Diligence |
    And deadline is set to 30 days
    And training is mandatory before client interaction

  @high @fr-aml-train-002 @aml @training
  Scenario: Complete AML training with assessment
    Given employee with pending AML training
    When completing AML training modules
    Then employee passes assessment (minimum 70% score)
    And certificate is issued
    And completion is logged
    And system grants permissions for client interaction

  @medium @fr-aml-train-003 @aml @training
  Scenario: Refresh AML training annually
    Given employee completed AML training 12 months ago
    When annual training cycle begins
    Then employee is assigned refresher training
    And training includes regulatory updates from past year
    And completion is mandatory before next anniversary
    And non-completion flags employee for escalation

  # Suspicious Activity Monitoring
  @critical @fr-sar-001 @aml @sar
  Scenario: Detect suspicious activity pattern
    Given customer with pattern:
      | Transaction | Amount | Interval |
      | Monthly transfer | TND 9,900 | Every 28 days |
      | Monthly transfer | TND 9,900 | Every 28 days |
      | Monthly transfer | TND 9,900 | Every 28 days |
    When analyzing patterns
    Then pattern matches "Structuring" (just below TND 10,000 reporting threshold)
    And alert is triggered automatically
    And analyst is assigned for investigation

  @critical @fr-sar-002 @aml @sar
  Scenario: File Suspicious Activity Report (SAR)
    Given suspicious structuring activity confirmed
    When filing SAR with BCT
    Then SAR includes:
      | Field |
      | Bank details |
      | Customer information |
      | Account details |
      | Description of suspicious activity |
      | Amount and period involved (TND 29,700 over 3 months) |
      | Reason for suspicion (structuring to avoid threshold) |
      | Recommended action (monitor/investigate/refer) |
    And SAR is filed within 10 business days
    And confidential filing indicator is set

  @high @fr-sar-003 @aml @sar
  Scenario: Track SAR filing outcomes
    Given SAR filed on 2025-03-15
    When tracking investigation results
    Then after 60 days:
      | Possible Outcome |
      | Investigation closed - no evidence of ML/TF |
      | Referred to Financial Intelligence Unit |
      | Customer account closed |
      | Funds frozen pending investigation |
    And outcome is documented
    And SAR status is updated
    And enforcement actions are recorded

  @high @fr-sar-004 @aml @sar
  Scenario: Generate SAR monthly filing report
    Given 25 SARs filed in March 2025
    When generating monthly SAR report
    Then report includes:
      | Metric |
      | Total SARs | 25 |
      | By Category: Structuring | 8 |
      | By Category: Unusual Volume | 12 |
      | By Category: Sanctions Breach | 3 |
      | By Category: PEP Activities | 2 |
      | Average filing time | 6 days |

  # Asset Freeze and Sanctions Compliance
  @critical @fr-freeze-001 @aml @sanctions
  Scenario: Implement asset freeze for sanctioned party
    Given customer matched on OFAC SDN list (confirmed)
    When executing freeze
    Then all customer accounts are frozen immediately
    And new transactions are rejected
    And freeze reason is documented
    And bcT is notified within 24 hours
    And customer is informed (except in terrorism cases)

  @critical @fr-freeze-002 @aml @sanctions
  Scenario: Monitor frozen accounts for compliance
    Given 5 accounts with active freeze status
    When monitoring frozen accounts
    Then:
      | Check |
      | No outgoing transactions allowed |
      | Income can be received (for some cases) |
      | Freeze is maintained until license obtained |
      | Quarterly freeze status report is generated |

  @high @fr-freeze-003 @aml @sanctions
  Scenario: Lift freeze after sanctions removal
    Given customer removed from OFAC SDN list
    When receiving official notice of delisting
    Then freeze is lifted
    And customer is notified
    And normal account access is restored
    And lift action is documented with authorizing official

  # AML Investigation Workflow
  @high @fr-invest-001 @aml @investigation
  Scenario: Create and manage AML investigation
    Given suspicious activity requiring investigation
    When opening investigation case
    Then case is created with:
      | Detail |
      | Unique investigation ID |
      | Assigned to investigator |
      | Priority level (Low/Medium/High/Critical) |
      | Investigation scope defined |
      | Timeline set (target resolution date) |

  @high @fr-invest-002 @aml @investigation
  Scenario: Document investigation evidence and findings
    Given open investigation case
    When investigating
    Then evidence is gathered:
      | Evidence |
      | Customer transaction history |
      | Correspondence and emails |
      | Account statements |
      | Third-party confirmations |
      | Adverse media search results |
    And findings are documented with analysis
    And investigator's conclusions are recorded

  @medium @fr-invest-003 @aml @investigation
  Scenario: Escalate investigation findings
    Given investigation findings indicating ML/TF
    When investigation is near completion
    Then case is escalated to AML Manager
    And recommendation is made (Close / File SAR / Refer FIU)
    And approval is obtained
    And action is executed

  # Transaction Monitoring Rules
  @high @fr-monitor-001 @aml @monitoring
  Scenario: Configure transaction monitoring rules
    Given no custom monitoring rules defined
    When setting up monitoring rules
    Then rules include:
      | Rule | Threshold | Action |
      | Single transaction amount | > TND 100,000 | Alert |
      | Daily aggregate | > TND 250,000 | Alert |
      | Transactions to high-risk countries | Any amount | Alert |
      | Rapid transfers (day trading) | > 10 per day | Alert |
      | Round amounts (structuring indicator) | TND 9,900 | Alert |
    And rules are documented and approved

  @high @fr-monitor-002 @aml @monitoring
  Scenario: Execute transaction monitoring
    Given configured monitoring rules
    When processing daily transactions
    Then 1,250 transactions are monitored
    And 47 alerts are triggered
    And alerts are queued for analyst review
    And alerts include rule name and reasoning

  @medium @fr-monitor-003 @aml @monitoring
  Scenario: Manage monitoring rule false positives
    Given 47 monitoring alerts
    When analyst reviews each alert
    Then:
      | Outcome | Count |
      | True suspicious activity | 8 |
      | False positive (legitimate business) | 35 |
      | Requires additional investigation | 4 |
    And false positive patterns are identified
    And rule thresholds may be adjusted
    And false positive rate is tracked (74% this week)

  # Risk Rating and Customer Classification
  @high @fr-risk-class-001 @aml @risk
  Scenario: Assign customer AML risk rating
    Given customer profile with attributes
    When assigning risk rating
    Then rating considers:
      | Factor | Weight |
      | Customer Type (Corporate = higher) | 20% |
      | Country Risk | 25% |
      | PEP Status | 30% |
      | Business Sector (Cash-intensive = higher) | 15% |
      | Transaction Profile | 10% |
    And overall risk score (1-100) is calculated
    And customer is classified as Low/Medium/High/Critical risk
    And rating is reviewed annually

  @high @fr-risk-class-002 @aml @risk
  Scenario: Update customer risk rating based on activity
    Given customer with Low risk rating
    When customer activity changes to unusual patterns
    Then risk reassessment is triggered
    And new risk rating may increase to Medium/High
    And EDD process is initiated if threshold exceeded
    And customer communication may be updated
