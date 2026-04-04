Feature: Prudential Ratios (BC6)
  As a risk manager
  I want to calculate regulatory ratios
  So that the bank meets BCT prudential requirements

  Scenario: Calculate solvency ratio
    Given total capital of 100000000 TND and risk-weighted assets of 800000000 TND
    When I calculate the solvency ratio
    Then the ratio is 12.5% which is above the 10% minimum

  Scenario: Flag insufficient Tier 1 ratio
    Given Tier 1 capital of 40000000 TND and risk-weighted assets of 800000000 TND
    When I calculate the Tier 1 ratio
    Then the ratio is 5.0% which is below the 7% minimum
