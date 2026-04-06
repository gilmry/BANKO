Feature: Governance and Audit (BC11)
  As an internal auditor
  I want to track all operations via audit trail
  So that every action is traceable and immutable

  Scenario: Record an audit trail entry
    Given a user "admin-001" performs action "create_account"
    When the audit trail is recorded
    Then an immutable entry exists with timestamp and actor

  Scenario: Query audit trail by entity
    Given multiple audit entries for entity "ACC-001"
    When I query the audit trail for entity "ACC-001"
    Then all related entries are returned in chronological order
