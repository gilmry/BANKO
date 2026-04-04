Feature: Account Management (BC2)
  As a bank officer
  I want to manage bank accounts
  So that customers can hold and operate accounts

  Scenario: Open a new current account
    Given a verified customer with id "cust-001"
    When I open a current account in TND
    Then the account is created with balance 0.000 TND

  Scenario: Reject account opening for unverified customer
    Given an unverified customer with id "cust-002"
    When I attempt to open a current account
    Then the account opening is rejected with error "customer_not_verified"
