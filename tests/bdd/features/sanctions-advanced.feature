Feature: Sanctions Screening (BC5) - Batch Screening, Whitelist, Escalation
  As a sanctions compliance officer
  I want to manage sanctions screening, false positives, escalation workflows, and reporting
  So that the bank screens all customers and transactions against international sanctions lists

  Background:
    Given the system is initialized
    And I am authenticated as "sanctions_officer"
    And sanctions screening is enabled

  # Batch Screening Operations
  @critical @fr-batch-001 @sanctions @screening
  Scenario: Execute batch screening of customer database
    Given customer database with 50,000 customers
    When running daily batch screening
    Then each customer is checked against:
      | Sanctions List |
      | OFAC SDN (US) |
      | UN Consolidated List |
      | EU Sanctions List |
      | UK OFSI List |
      | Swiss SECO List |
    And screening completes within 2 hours
    And results are logged with timestamp

  @critical @fr-batch-002 @sanctions @screening
  Scenario: Identify matches in batch screening
    Given batch screening execution
    When screening detects matches
    Then 15 matches are identified with details:
      | Match | Confidence | Status |
      | Customer: Hassan Ben Ali | 98% | Likely match |
      | Entity: ABC Trading Company | 75% | Review required |
      | Related Party: Fatima Ben Ali | 82% | Likely match |
    And matches are queued for analyst review
    And automatic alerts are triggered

  @high @fr-batch-003 @sanctions @screening
  Scenario: Validate screening accuracy
    Given batch screening results with 15 matches
    When validating screening database
    Then verification includes:
      | Check |
      | All OFAC SDN entries are present (3,500+ names) |
      | UN list is current (updated within 7 days) |
      | EU sanctions list is current |
      | Fuzzy matching detects name variations |
      | DOB/passport checks are enabled where available |
    And accuracy metrics are generated

  @high @fr-batch-004 @sanctions @screening
  Scenario: Schedule recurring batch screenings
    Given batch screening process defined
    When configuring screening schedule
    Then screenings are scheduled:
      | Frequency | Scope |
      | Daily | Full customer database |
      | Hourly | New transactions > TND 50,000 |
      | Weekly | New customers onboarded |
      | Real-time | Payments and transfers |
    And schedule is maintained in audit log

  # Match Review and False Positive Management
  @critical @fr-match-001 @sanctions @screening
  Scenario: Review and verify sanctions match
    Given match identified: Customer name similar to "Ahmed Al-Abdli" (OFAC SDN)
    When analyst reviews match
    Then analyst evaluates:
      | Criterion |
      | Full name comparison |
      | Date of birth (if available) |
      | Country of residence |
      | Business relationship history |
      | Previous screening results |
    And match is classified as:
      | Classification | Action |
      | Confirmed match (98%+ confidence) | Freeze account, file SAR, notify OFAC |
      | Likely match (75-97%) | Escalate for senior review |
      | Possible match (50-74%) | Enhanced due diligence required |
      | Not a match | Add to false positive whitelist |

  @critical @fr-match-002 @sanctions @screening
  Scenario: Escalate confirmed sanctions match
    Given confirmed sanctions match (98%+ confidence)
    When escalating
    Then actions are taken immediately:
      | Action | Timing |
      | Account freeze | Immediate |
      | Transaction rejection | Immediate |
      | Notice to OFAC | Within 10 days |
      | Customer notification | After regulatory clearance |
      | Board notification | Immediately |
    And freeze reason is documented
    And legal review is initiated

  @high @fr-match-003 @sanctions @screening
  Scenario: Document match investigation findings
    Given ongoing investigation of sanctions match
    When documenting findings
    Then documentation includes:
      | Field |
      | Match details (name, confidence %) |
      | Investigation date and investigator |
      | Sources consulted (OFAC press releases, etc.) |
      | Conclusion (Match / Not a match) |
      | Evidence of decision-making |
      | Signature and date |

  # False Positive Whitelist Management
  @high @fr-fp-001 @sanctions @screening
  Scenario: Create false positive whitelist entry
    Given match determined to be false positive
    When adding to whitelist
    Then entry includes:
      | Field |
      | Customer name (exact spelling) |
      | Date of birth |
      | Nationality |
      | Passport/ID number |
      | Match that was flagged (OFAC name) |
      | Analyst decision |
      | Approval signature |
      | Effective date |

  @high @fr-fp-002 @sanctions @screening
  Scenario: Apply whitelist to future screening
    Given whitelist with 200 entries
    When running daily batch screening
    Then whitelist entries are excluded from matching
    And false positive rate decreases
    And screening efficiency improves (fewer manual reviews)

  @medium @fr-fp-003 @sanctions @screening
  Scenario: Manage whitelist with regular review
    Given whitelist with entries from 2023-2024
    When performing annual whitelist review
    Then each entry is reviewed for:
      | Review Item |
      | Continued validity (customer still known entity) |
      | Sanctions list changes (delisting status) |
      | Business relationship status |
    And entries may be removed if no longer applicable
    And whitelist is re-approved

  @medium @fr-fp-004 @sanctions @screening
  Scenario: Generate false positive statistics
    Given 12 months of screening results
    When analyzing false positives
    Then report includes:
      | Metric | Value |
      | Total matches | 186 |
      | False positives | 147 |
      | False positive rate | 79% |
      | Average time to whitelist | 2.5 days |
      | Repeated FP names | 43 |
    And trends are identified for improvement

  # Escalation Rules and Workflows
  @critical @fr-escalate-001 @sanctions @escalation
  Scenario: Configure sanctions escalation rules
    Given no escalation rules defined
    When establishing escalation rules
    Then rules include:
      | Trigger | Escalation Path |
      | Confirmed match (>95%) | AML Manager → CEO → Regulator |
      | Multiple matches (>3 for one customer) | Sanctions Officer → Compliance Director |
      | Ongoing freeze (>90 days) | Monthly board reporting required |
      | Account with no legitimate business | Closure recommended |
      | Customer dispute of freeze | Legal review required |
    And rules are documented and approved

  @high @fr-escalate-002 @sanctions @escalation
  Scenario: Execute escalation workflow
    Given sanctions match meeting escalation criteria
    When escalation is triggered
    Then workflow moves through:
      | Stage | Action | Timeline |
      | 1. Initial Match | Analyst review | Day 1 |
      | 2. Escalation | Notify Sanctions Officer | Day 1 |
      | 3. Senior Review | Manager assessment | Day 2-3 |
      | 4. Executive Review | CEO/Board notification | Day 3 |
      | 5. Regulatory Notice | OFAC filing | Day 10 |

  @high @fr-escalate-003 @sanctions @escalation
  Scenario: Handle escalation approval and override
    Given escalation in progress
    When escalation requires approval
    Then approval workflow is:
      | Action | Authority |
      | Account freeze > 30 days | Compliance Director |
      | Funds transfer for frozen account | CEO approval |
      | Removal from watchlist | Legal + Compliance Director |
      | SAR coordination | AML Manager + Sanctions Officer |
    And approvals are logged with timestamp and reasoning

  @medium @fr-escalate-004 @sanctions @escalation
  Scenario: Generate escalation metrics and reporting
    Given monthly escalation activities
    When generating escalation report
    Then report includes:
      | Metric | Value |
      | Escalations initiated | 12 |
      | Escalations resolved | 10 |
      | Average resolution time | 5 days |
      | Escalations pending | 2 |
      | Re-escalations | 1 |

  # Regulatory Reporting and Notifications
  @critical @fr-report-001 @sanctions @reporting
  Scenario: File OFAC sanctions match report
    Given confirmed sanctions match identified
    When filing OFAC report
    Then report includes:
      | Field |
      | Institution name and ID |
      | Investigation date |
      | Subject name and identification |
      | SDN number (if applicable) |
      | Accounts/transactions affected |
      | Measures taken (freeze, rejection) |
      | Filing date and signature |
    And report is submitted via eregulations.treasury.gov
    And confirmation is recorded

  @high @fr-report-002 @sanctions @reporting
  Scenario: Submit sanctions activity to BCT
    Given monthly sanctions screening and match summary
    When submitting to BCT
    Then submission includes:
      | Data |
      | Number of customers screened |
      | Matches identified (by list type) |
      | Confirmed matches |
      | False positives |
      | Accounts frozen |
      | Transactions rejected |
      | OFACs filings made |
    And submission is within regulatory deadline

  @high @fr-report-003 @sanctions @reporting
  Scenario: Maintain sanctions screening log
    Given all screening activities
    When maintaining audit log
    Then log includes:
      | Entry |
      | Screening date/time |
      | Lists screened |
      | Number of records screened |
      | Matches identified |
      | Matches resolved |
      | Actions taken |
    And log is retained for 7 years minimum

  # Real-Time Transaction Screening
  @critical @fr-rtscreen-001 @sanctions @realtime
  Scenario: Screen transactions in real-time before processing
    Given incoming customer transaction (transfer, payment, etc.)
    When transaction triggers real-time screening
    Then screening checks:
      | Check |
      | Customer name against sanctions lists |
      | Beneficiary name against sanctions lists |
      | Originating country |
      | Destination country |
      | Counterparty entity information |
    And result is returned within 5 seconds
    And transaction proceeds only if compliant

  @critical @fr-rtscreen-002 @sanctions @realtime
  Scenario: Reject transaction with sanctions hit
    Given real-time screening identifies sanctions match
    When match confidence > 75%
    Then transaction is rejected immediately
    And rejection reason is documented
    And customer is notified (compliance permitting)
    And analyst is assigned for investigation

  @high @fr-rtscreen-003 @sanctions @realtime
  Scenario: Monitor real-time screening performance
    Given daily transaction volumes
    When analyzing screening performance
    Then metrics include:
      | Metric | Target | Actual |
      | Screening latency | < 5 sec | 3.2 sec |
      | Hit rate | 0.1-0.3% | 0.18% |
      | False positive rate | < 50% | 42% |
      | System availability | 99.9% | 99.95% |
    And dashboard shows real-time metrics

  # Sanctions List Management
  @high @fr-list-001 @sanctions @listmgmt
  Scenario: Update sanctions lists regularly
    Given sanctions list update schedule
    When updating lists (OFAC, UN, EU, UK, SECO)
    Then:
      | List | Update Frequency | Last Updated |
      | OFAC SDN | Daily | Today |
      | UN Consolidated | Daily | Today |
      | EU Consolidated | Weekly | This week |
      | UK OFSI | Daily | Today |
      | SECO | Weekly | This week |
    And new entries are loaded into system
    And update log is maintained

  @high @fr-list-002 @sanctions @listmgmt
  Scenario: Validate sanctions list integrity
    Given updated sanctions lists loaded
    When validating data integrity
    Then validation checks:
      | Check |
      | Record count matches source |
      | No duplicate entries |
      | Required fields populated |
      | Date formats are valid |
      | List version/date is current |
    And validation report is generated
    And system is updated only if all checks pass

  @medium @fr-list-003 @sanctions @listmgmt
  Scenario: Track sanctions list version history
    Given multiple sanctions list updates
    When maintaining version control
    Then history includes:
      | Field |
      | List name and version |
      | Update date |
      | Record count |
      | Source URL |
      | Hash/checksum |
    And previous versions are retained for 1 year

  # Compliance Monitoring and Metrics
  @high @fr-compliance-001 @sanctions @monitoring
  Scenario: Monitor sanctions screening coverage
    Given customer and transaction populations
    When analyzing screening coverage
    Then metrics include:
      | Item | Coverage |
      | Existing customers screened | 100% annually |
      | New customers screened | 100% at onboarding |
      | Transactions screened | 100% (real-time) |
      | Non-covered items | 0% (goal) |
    And dashboard tracks coverage in real-time

  @high @fr-compliance-002 @sanctions @monitoring
  Scenario: Generate sanctions compliance dashboard
    Given monthly screening and match data
    When generating compliance dashboard
    Then dashboard displays:
      | KPI | Value |
      | Screening volume | 125,000 records |
      | Matches identified | 42 |
      | Confirmed matches | 3 |
      | False positives | 39 |
      | Hit rate | 0.034% |
      | Whitelist entries | 185 |
      | Accounts frozen | 3 |
      | Days since OFAC filing | 3 |
    And board receives monthly report

  @medium @fr-compliance-003 @sanctions @monitoring
  Scenario: Track sanctions compliance incidents
    Given compliance incidents related to sanctions
    When logging incidents
    Then incident tracking includes:
      | Detail |
      | Incident date and type |
      | Description of non-compliance |
      | Impact (customers affected, funds involved) |
      | Root cause |
      | Corrective action |
      | Resolution date |
    And incidents are reported to regulator if required

  # Sanctions and AML Integration
  @high @fr-integration-001 @sanctions @aml
  Scenario: Coordinate sanctions freeze with AML SAR
    Given customer account frozen for sanctions
    When SAR filing is also triggered
    Then SAR and freeze are coordinated:
      | Coordination |
      | Same investigation case number |
      | SAR notes sanctions match |
      | Timeline aligned |
      | Customer communication coordinated |
    And information is shared securely between teams

  @high @fr-integration-002 @sanctions @aml
  Scenario: Link sanctions hits to AML risk rating
    Given customer with confirmed sanctions match
    When updating AML risk profile
    Then risk rating is elevated to "Critical"
    And EDD process is initiated
    And ongoing monitoring is enhanced
    And relationship review is triggered
