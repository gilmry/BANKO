Feature: Sanctions Screening (BC5)
  As a compliance officer
  I want to screen entities against sanctions lists
  So that the bank does not deal with sanctioned parties

  Scenario: Screen a customer against sanctions lists
    Given a customer named "John Smith"
    When I screen against ONU and UE sanctions lists
    Then the screening result is "no_match"

  Scenario: Detect a potential sanctions match
    Given a customer named "Sanctioned Entity"
    When I screen against ONU sanctions lists
    Then the screening result is "potential_match" with score above 80
