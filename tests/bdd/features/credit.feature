Feature: Credit Management (BC3)
  As a credit analyst
  I want to manage loan applications and classifications
  So that credit risk is properly assessed

  Scenario: Submit a loan application
    Given a customer with id "cust-001" and risk score "B"
    When I submit a loan application for 50000 TND over 60 months
    Then the loan application is created with status "pending_analysis"

  Scenario: Classify a performing loan
    Given a loan with id "loan-001" and no missed payments
    When I run the classification engine
    Then the loan is classified as class 0 with provision rate 0%
