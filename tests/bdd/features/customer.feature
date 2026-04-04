Feature: Customer Management (BC1)
  As a bank officer
  I want to manage customer onboarding and KYC
  So that clients are properly identified and compliant

  Scenario: Create a new customer with valid KYC
    Given a new customer with name "Ahmed Ben Ali" and email "ahmed@example.tn"
    When I submit the customer onboarding form
    Then the customer is created with status "pending_kyc"

  Scenario: Reject customer with invalid email
    Given a new customer with name "Test User" and email "invalid-email"
    When I submit the customer onboarding form
    Then the onboarding is rejected with error "invalid_email"
