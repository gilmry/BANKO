Feature: Payment Processing (BC9)
  As a bank operator
  I want to process payments and transfers
  So that customers can send and receive funds

  Scenario: Process a domestic transfer
    Given an account "ACC-001" with balance 5000 TND
    When I initiate a transfer of 1000 TND to account "ACC-002"
    Then the transfer is created with status "pending"

  Scenario: Reject transfer with insufficient funds
    Given an account "ACC-003" with balance 100 TND
    When I initiate a transfer of 500 TND to account "ACC-004"
    Then the transfer is rejected with error "insufficient_funds"
