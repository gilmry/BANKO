Feature: Identity and Authentication (BC12)
  As a system administrator
  I want to manage user authentication and authorization
  So that access is secure and role-based

  Scenario: Register a new user
    Given a registration request with email "user@banko.tn" and password "Str0ng!Pass"
    When I register the user
    Then the user is created with role "user"

  Scenario: Login with valid credentials
    Given a registered user with email "user@banko.tn"
    When I login with correct password
    Then a session token is returned

  Scenario: Reject login with wrong password
    Given a registered user with email "user@banko.tn"
    When I login with wrong password
    Then login is rejected with error "invalid_credentials"
