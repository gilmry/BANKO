Feature: Anti-Money Laundering (BC4)
  As a compliance officer
  I want to detect suspicious transactions
  So that the bank complies with LBC/FT regulations

  Scenario: Detect large cash transaction above threshold
    Given a transaction of 15000 TND in cash
    When I run AML screening
    Then an alert is generated with type "large_cash_transaction"

  Scenario: No alert for normal transaction
    Given a transaction of 500 TND by transfer
    When I run AML screening
    Then no alert is generated
