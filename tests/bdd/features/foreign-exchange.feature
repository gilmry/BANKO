Feature: Foreign Exchange (BC10)
  As a treasury operator
  I want to manage FX operations
  So that the bank handles currency conversions compliantly

  Scenario: Execute a spot FX operation
    Given an exchange rate of 3.35 TND/EUR
    When I convert 10000 EUR to TND
    Then the result is 33500 TND

  Scenario: Check FX position limits
    Given the current EUR position is 4500000 TND equivalent
    When I check against the 5000000 TND position limit
    Then the position is within limits
