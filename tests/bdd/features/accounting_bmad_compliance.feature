Feature: BMAD v4.0.1 Compliance for Accounting BC7
  As a compliance officer
  I want to verify all BMAD accounting requirements are met
  So that BANKO meets regulatory standards

  # FR-082: Chart of Accounts
  Scenario: Create chart of accounts with NCT class validation
    Given a chart of accounts entry for code "31" with class "Class3"
    When I validate the account code matches the class
    Then the entry is accepted

  Scenario: Reject account with mismatched class
    Given a chart of accounts entry for code "31" with class "Class1"
    When I validate the account code matches the class
    Then the entry is rejected with error "class_mismatch"

  # FR-083 & FR-084: Double-entry bookkeeping
  Scenario: Journal entry enforces balanced debits and credits
    Given a journal entry with debit 1000 TND on account "31" and credit 1000 TND on account "42"
    When I validate the journal entry
    Then the entry is accepted as balanced

  Scenario: Reject unbalanced entry
    Given a journal entry with debit 1000 TND on account "31" and credit 900 TND on account "42"
    When I validate the journal entry
    Then the entry is rejected with error "unbalanced_entry"

  # FR-086: Trial Balance
  Scenario: Compute balanced trial balance
    Given I have posted entries:
      | Account | Debit  | Credit |
      | 31      | 50000  | 0      |
      | 42      | 0      | 50000  |
    When I compute the trial balance as of 2026-04-07
    Then the trial balance is balanced
    And total debits equal total credits
    And both equal 50000 TND

  Scenario: Detect unbalanced trial balance
    Given I have posted entries:
      | Account | Debit  | Credit |
      | 31      | 50000  | 0      |
      | 42      | 0      | 40000  |
    When I compute the trial balance as of 2026-04-07
    Then the trial balance is unbalanced
    And total debits = 50000 TND
    And total credits = 40000 TND

  # FR-087: Reconciliation
  Scenario: Auto-resolve rounding variance
    Given I have accounts with 1 TND variance (within tolerance)
    When I reconcile the accounts
    Then the variance is auto-resolved

  # FR-088: Interest Accrual
  Scenario: Calculate daily interest on savings account
    Given a savings account with 10000 TND at 5% annual rate
    When I accrue daily interest for one day
    Then the daily interest is approximately 1.37 TND

  # FR-089: IFRS 9 ECL with dual PD
  Scenario: Calculate ECL with 12-month and lifetime PD
    Given a Stage 1 loan with 2% PD and 45% LGD on 1M EAD
    When I calculate the ECL
    Then 12-month ECL = 9000 TND
    And lifetime ECL > 12-month ECL
    And lifetime PD > 12-month PD

  # FR-090: Dual Posting
  Scenario: Create dual posting entry (NCT + IFRS 9)
    Given an NCT journal entry
    And an alternative IFRS 9 journal entry
    When I create a dual posting
    Then both posting engines are tracked
    And can be reported separately

  # FR-091: Fees and Commissions
  Scenario: Apply fee with condition
    Given a transaction fee defined as 2% of amount
    And condition: transaction >= 1000 TND
    When a 1500 TND transaction occurs
    Then the fee is 30 TND

  # FR-092: VAT on Fees (19% Tunisia)
  Scenario: Calculate VAT on banking fees
    Given a banking fee of 100 TND
    When I calculate 19% VAT
    Then VAT = 19 TND
    And total = 119 TND

  # FR-093: Daily Closing
  Scenario: Close daily period
    Given entries posted for 2026-04-07
    When I close the daily period
    Then the period status is "Closed"
    And closing shows total debits = total credits

  # FR-094: Monthly Closing
  Scenario: Close monthly period
    Given entries posted for April 2026
    When I close the monthly period for 2026-04
    Then the period status is "Closed"
    And period type is "Monthly"

  # FR-095: Annual Closing
  Scenario: Close annual period
    Given entries posted for year 2026
    When I close the annual period for 2026
    Then the period status is "Closed"
    And period type is "Annual"
    And can be archived for permanent storage

  # FR-096: Audit Trail
  Scenario: Maintain complete audit trail
    Given a posted journal entry
    When I review the entry
    Then created_at timestamp is present
    And posted_at timestamp is present
    And entry status is "Posted"
    And entry ID is immutable

  # FR-097: Export Formats
  Scenario: Export entries in BCT format
    Given journal entries from 2026-04-01 to 2026-04-30
    When I export in BCT format
    Then the export contains:
      | Field       | Example |
      | Header      | BCT_EXPORT |
      | Version     | 1.0 |
      | Entry lines | All posted entries |

  Scenario: Export entries in XBRL format
    Given journal entries from 2026-04-01 to 2026-04-30
    When I export in XBRL format
    Then the export is valid XML
    And contains instance and context elements

  Scenario: Export entries in CSV format
    Given journal entries from 2026-04-01 to 2026-04-30
    When I export in CSV format
    Then the export contains headers
    And each entry line is comma-separated

  Scenario: Export entries in JSON format
    Given journal entries from 2026-04-01 to 2026-04-30
    When I export in JSON format
    Then the export is valid JSON
    And entries are properly structured

  # FR-099: Multi-currency
  Scenario: Support multi-currency accounting
    Given entries in TND and EUR
    When I create journal entries
    Then both currencies are recorded
    And trial balance respects currency columns
