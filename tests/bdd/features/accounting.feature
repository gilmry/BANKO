Feature: General Accounting (BC7)
  As an accountant
  I want to manage journal entries and ledgers
  So that the books are balanced per NCT standards

  Scenario: Create a balanced journal entry
    Given a journal entry with debit 1000 TND on account "512" and credit 1000 TND on account "411"
    When I validate the journal entry
    Then the entry is accepted as balanced

  Scenario: Reject an unbalanced journal entry
    Given a journal entry with debit 1000 TND on account "512" and credit 900 TND on account "411"
    When I validate the journal entry
    Then the entry is rejected with error "unbalanced_entry"
