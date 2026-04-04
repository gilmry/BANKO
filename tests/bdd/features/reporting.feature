Feature: Regulatory Reporting (BC8)
  As a compliance officer
  I want to generate regulatory reports
  So that the bank meets BCT reporting deadlines

  Scenario: Generate monthly prudential report
    Given the reporting period is "2026-03"
    When I generate the monthly prudential report
    Then the report is created with status "draft"

  Scenario: Submit report to BCT
    Given a validated report with id "report-001"
    When I submit the report
    Then the report status changes to "submitted"
