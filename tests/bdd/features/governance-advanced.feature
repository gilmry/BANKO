Feature: Advanced Governance (BC11) - RBAC, SoD, Multi-Level Approval, Delegation
  As a governance administrator
  I want to manage role-based access control, segregation of duties, multi-level approvals, and delegation
  So that the bank maintains control, prevents fraud, and ensures appropriate access management

  Background:
    Given the system is initialized
    And I am authenticated as "governance_admin"
    And governance module is operational

  # Role-Based Access Control (RBAC) with Inheritance
  @critical @fr-rbac-001 @governance @rbac
  Scenario: Create roles with inheritance hierarchy
    Given no role hierarchy defined
    When establishing RBAC structure
    Then role hierarchy includes:
      | Role | Parent Role | Permissions | Users |
      | Admin | None | All permissions | 2 |
      | Manager | Admin | All except system config | 15 |
      | Officer | Manager | Core operations | 50 |
      | Clerk | Officer | Data entry, basic queries | 200 |
      | Viewer | Clerk | Read-only access | 100 |
    And child roles inherit parent permissions
    And hierarchy is enforced by system

  @high @fr-rbac-002 @governance @rbac
  Scenario: Grant permission through role assignment
    Given new employee "Ahmed"
    When assigning role "Credit Officer"
    Then automatic permission grant:
      | Permission | Reason |
      | Create loan applications | Role directly grants |
      | View customer profiles | Inherited from "Officer" parent |
      | Read reports | Inherited from "Manager" parent |
      | Execute payments | NOT inherited (only in Payment Officer role) |
    And permissions are tracked by role inheritance
    And audit log records role assignment

  @high @fr-rbac-003 @governance @rbac
  Scenario: Remove inheritance when custom permission needed
    Given employee with inherited permissions
    When exception override is required
    Then override mechanism:
      | Action | Requirement |
      | Remove inherited permission | Must document justification |
      | Add custom permission outside role | Requires manager approval |
      | Create custom role for edge case | Must be temporary (max 3 months) |
    And exceptions are tracked for audit
    And quarterly review removes stale exceptions

  @medium @fr-rbac-004 @governance @rbac
  Scenario: Generate RBAC access matrix report
    Given 400 users with role assignments
    When generating RBAC report
    Then report includes:
      | Matrix Content |
      | Rows: All users (400) |
      | Columns: All permissions (125) |
      | Cell values: Direct (D), Inherited (I), None (-) |
      | Role aggregations |
      | Outlier detection (users with unusual permission combos) |
    And outliers are flagged for review

  # Segregation of Duties (SoD)
  @critical @fr-sod-001 @governance @sod
  Scenario: Define segregation of duties rules
    Given no SoD rules configured
    When establishing SoD framework
    Then SoD rules include:
      | Role 1 | Role 2 | Conflict | Enforcement |
      | Loan Approver | Credit Analyst | Same person cannot both initiate and approve | System blocks |
      | Payment Initiator | Payment Authorizer | Different approvers required | Payment requires dual auth |
      | Record Keeper | Record Auditor | Cannot audit own records | Role assignment blocked |
      | Treasury Dealer | Middle Office Manager | Conflict of interest | Cannot hold both |
      | Fund Custodian | Fund Auditor | Independence required | Role conflict detected |
    And rules are enforced by system
    And violations trigger alerts

  @critical @fr-sod-002 @governance @sod
  Scenario: Prevent SoD violation at role assignment
    Given SoD rule: Cannot hold both "Payment Initiator" and "Payment Authorizer"
    When assigning "Payment Initiator" role to user already holding "Payment Authorizer"
    Then system:
      | Check |
      | Detects SoD conflict |
      | Blocks assignment: "SoDViolation" |
      | Suggests: Remove Authorizer role first, then assign Initiator |
      | Requires manager approval to proceed (with override justification) |
    And assignment cannot complete without override approval
    And override is logged with approver name and reason

  @high @fr-sod-003 @governance @sod
  Scenario: Monitor SoD compliance continuously
    Given 400 users with active roles
    When running daily SoD compliance check
    Then compliance scan identifies:
      | Finding | Count | Action |
      | Active SoD violations | 2 | Immediate escalation |
      | Pending role assignments with SoD risk | 5 | Requires override approval |
      | Recently granted conflicting roles | 3 | Review if temporary |
    And report is generated for governance committee
    And violations must be remediated within 24 hours

  @high @fr-sod-004 @governance @sod
  Scenario: Create temporary exception to SoD
    Given critical business need (emergency coverage)
    When requesting temporary SoD exception
    Then exception process:
      | Element |
      | Duration limit: 72 hours maximum |
      | Requires: CFO + Compliance Officer approval |
      | Justification: Must document business reason |
      | Monitoring: Enhanced monitoring during exception |
      | Exit: Must be removed after expiry (no auto-renewal) |
    And exception is logged with approvers
    And system reminder at 48-hour mark to prepare transition

  # Multi-Level Approval Workflows
  @critical @fr-approval-001 @governance @approval
  Scenario: Configure 4-eyes approval for transactions
    Given high-value transaction threshold
    When setting up 4-eyes control (dual approval)
    Then approval rule:
      | Threshold | TND 100,000 |
      | Initiator | Loan Officer |
      | Approver 1 | Branch Manager |
      | Approver 2 | Credit Manager |
      | Both approvers must be different individuals | Required |
      | Sequence | Serial (both must approve, any order) |
    And system enforces dual auth on eligible transactions

  @critical @fr-approval-002 @governance @approval
  Scenario: Execute 4-eyes approval workflow
    Given loan application for TND 150,000 (exceeds threshold)
    When initiating approval workflow
    Then workflow progression:
      | Step | Actor | Action | Status |
      | 1 | Loan Officer | Submits application | Pending |
      | 2 | Branch Manager | Approves (11:30 AM) | Partially Approved |
      | 3 | Credit Manager | Approves (2:15 PM) | Fully Approved |
      | 4 | System | Releases transaction | Executed |
    And both approvals recorded with timestamps
    And transaction can only execute after both approvals

  @high @fr-approval-003 @governance @approval
  Scenario: Implement 6-eyes approval for critical operations
    Given critical transaction: Board-level funding transfer TND 5,000,000
    When requiring 6-eyes approval
    Then approval matrix:
      | Level | Roles (must have 2 independent approvers) |
      | Level 1 | CFO, CEO (pick 1) |
      | Level 2 | Risk Manager, Compliance Officer (pick 1) |
      | Level 3 | Board Director A, Board Director B (pick 1) |
    And all 3 levels must approve
    And no single person approves twice
    And approval audit is comprehensive

  @high @fr-approval-004 @governance @approval
  Scenario: Manage approval rejections and rework
    Given transaction pending for approval
    When Approver 1 rejects transaction
    Then rejection workflow:
      | Action | Details |
      | Rejection recorded | Timestamp, reason documented |
      | Transaction status | Changed to "Rejected" |
      | Initiator notification | Notified with rejection reason |
      | Rework process | Can modify and resubmit |
      | Re-approval cycle | Starts fresh with new approvers |
    And rejection history is retained

  @medium @fr-approval-005 @governance @approval
  Scenario: Escalate approval for expedited processing
    Given transaction pending approval for 4 hours
    When requesting approval escalation
    Then escalation process:
      | Level | Action |
      | 1. Regular approver | May not respond |
      | 2. Escalation trigger | Auto-escalates to manager level |
      | 3. Manager review | High-priority queue |
      | 4. Approval or rejection | Expedited decision (30 min SLA) |
    And escalation is logged
    And original approver is notified

  # Delegation of Authority
  @critical @fr-delegation-001 @governance @delegation
  Scenario: Create delegation of authority arrangement
    Given manager who needs leave coverage
    When delegating approval authority
    Then delegation includes:
      | Parameter | Value |
      | Delegator | Branch Manager "Hassan" |
      | Delegate | Senior Officer "Fatima" |
      | Permissions delegated | All approval authority up to TND 250,000 |
      | Period | 2025-05-01 to 2025-05-31 (1 month) |
      | Notice to delegates | Immediate |
      | Notice to approvers | Immediate |
    And delegation is recorded in system
    And time-bound (expires automatically)

  @high @fr-delegation-002 @governance @delegation
  Scenario: Track and audit delegated authority usage
    Given delegation active for Hassan → Fatima
    When Fatima approves 5 transactions during delegation period
    Then tracking:
      | Transaction | Amount | Delegated Authority |
      | 1 | TND 75,000 | Yes (within limit) |
      | 2 | TND 180,000 | Yes (within limit) |
      | 3 | TND 300,000 | No (exceeds limit) |
      | 4 | TND 45,000 | Yes (within limit) |
      | 5 | TND 95,000 | Yes (within limit) |
    And transaction 3 is rejected: "DelegatedLimitExceeded"
    And all delegated approvals are flagged in audit report

  @high @fr-delegation-003 @governance @delegation
  Scenario: Revoke delegation early
    Given delegation in place for Hassan → Fatima (until 2025-05-31)
    When Hassan returns from leave early (2025-05-15)
    Then revocation process:
      | Action |
      | Delegator requests early termination |
      | Delegation is immediately revoked |
      | Delegate is notified |
      | New transactions require original delegator approval |
      | Pending approvals revert to delegator |
    And revocation timestamp is logged

  # Access Review and Certification
  @critical @fr-access-review-001 @governance @review
  Scenario: Initiate quarterly access review
    Given 400 users with role assignments
    When scheduling quarterly access review
    Then review process:
      | Phase | Owner | Timeline |
      | 1. Data Export | Governance Admin | Day 1 |
      | 2. Manager Certification | Department Managers (15) | Days 2-10 |
      | 3. Remediation | Governance Admin | Days 11-20 |
      | 4. Completion Report | Governance Director | Day 25 |
    And review deadline is enforced
    And escalation for non-response

  @critical @fr-access-review-002 @governance @review
  Scenario: Manager certifies user access during review
    Given quarterly review with user list assigned to manager
    When manager reviews 30 users' access
    Then manager certifies:
      | User | Current Roles | Certification |
      | Ahmed | Loan Officer | Correct - Required for role |
      | Fatima | Credit Manager, Approver | Correct - Both needed |
      | Hassan | Loan Officer, Payment Auth | Issue - Should not hold both |
      | Layla | Viewer (read-only) | Correct - Least privilege |
    And Hassan's conflicting roles flagged
    And remediation plan required for Hassan

  @high @fr-access-review-003 @governance @review
  Scenario: Execute access remediation
    Given access issues identified in review
    When executing remediation
    Then actions taken:
      | Issue | Remediation | Authority |
      | Hassan conflicting roles | Remove Payment Auth role | Manager approved |
      | Inactive user (Ahmed, no login 90+ days) | Disable account | Admin executed |
      | Excessive permissions (Fatima) | Reduce to necessary roles | Manager approved |
    And remediation completion is documented
    And follow-up audit confirms changes

  @medium @fr-access-review-004 @governance @review
  Scenario: Generate access certification report
    Given quarterly review completed
    When generating certification report
    Then report includes:
      | Section |
      | Users reviewed: 400 |
      | Issues identified: 23 |
      | Issues remediated: 22 |
      | Pending remediation: 1 (due 2025-05-30) |
      | SoD violations found/fixed: 3 |
      | Orphaned accounts disabled: 7 |
      | Certification attestation (signed by CRO) |
    And report is retained for audit trail

  # Privileged Access Management (PAM)
  @critical @fr-pam-001 @governance @pam
  Scenario: Establish privileged access request workflow
    Given sensitive operations requiring elevated access
    When requesting privileged access (e.g., database admin)
    Then request process:
      | Step | Requirement |
      | 1. Request submission | Justification required (business reason) |
      | 2. Approval | Security Officer + Department Head approval |
      | 3. Provisioning | Access granted for defined period (max 8 hours) |
      | 4. Session recording | All activities logged and recorded |
      | 5. De-provisioning | Access automatically revoked after period |
    And request audit is comprehensive

  @critical @fr-pam-002 @governance @pam
  Scenario: Monitor and audit privileged access usage
    Given elevated database access granted to DBA for 4 hours
    When DBA performs maintenance during session
    Then monitoring:
      | Monitoring Element |
      | Session recording (video + keystroke) |
      | All commands executed are logged |
      | Data accessed is tracked |
      | Files accessed/modified are recorded |
      | Session duration limited (4 hours) |
      | Automatic session termination |
    And session recording is retained for 1 year

  # Password and Authentication Policies
  @critical @fr-password-001 @governance @password
  Scenario: Enforce password policy requirements
    Given password policy configuration
    When users create passwords
    Then policy enforces:
      | Requirement | Setting |
      | Minimum length | 12 characters |
      | Complexity | Uppercase, lowercase, digit, special char |
      | History | Cannot reuse last 5 passwords |
      | Expiration | 90 days |
      | Failed attempts | Lock after 5 failed (30 min lockout) |
      | Lockout notification | User + Manager notified |
    And system validates against policy on each change

  @high @fr-password-002 @governance @password
  Scenario: Implement Multi-Factor Authentication (MFA) for sensitive access
    Given sensitive operations (large transfers, system config)
    When user accesses sensitive function
    Then MFA required:
      | Factor | Method |
      | Factor 1 | Username + password |
      | Factor 2 | SMS OTP to registered phone |
      | Factor 3 (optional) | Security questions for high-risk |
    And user must provide all factors
    And factors authenticated by independent systems

  @high @fr-password-003 @governance @password
  Scenario: Force password reset for compromised credentials
    Given credentials suspected compromised
    When security incident triggers password reset
    Then forced reset process:
      | Action |
      | User account locked immediately |
      | User notified of lock + reason |
      | User required to reset password on next login |
      | Temporary credentials sent to registered email |
      | MFA authentication required for password reset |
      | Session history review for suspicious activity |
    And user cannot proceed without password reset

  # Audit and Monitoring
  @high @fr-audit-001 @governance @audit
  Scenario: Generate comprehensive access and action audit log
    Given all user actions in system
    When generating daily audit report
    Then audit captures:
      | Log Entry | Details |
      | User ID | Who performed action |
      | Timestamp | When (date, time, timezone) |
      | Action | What (login, approve, create, delete) |
      | Resource | What was accessed (customer #, account) |
      | Status | Success/Failure |
      | IP Address | From where |
      | Device info | Desktop/Mobile, browser |
    And audit is immutable (cannot be edited)
    And audit is retained for 7 years minimum

  @high @fr-audit-002 @governance @audit
  Scenario: Analyze audit logs for suspicious patterns
    Given daily audit logs
    When analyzing for suspicious patterns
    Then detection includes:
      | Pattern | Alert Type |
      | Multiple failed login attempts | Brute force attempt |
      | Access outside business hours | Off-hours activity |
      | Unusual transaction amount for user | Outlier transaction |
      | Access to accounts unrelated to role | Unauthorized access |
      | Bulk data extraction | Data exfiltration attempt |
    And suspicious patterns trigger escalation to security team

  @medium @fr-audit-003 @governance @audit
  Scenario: Investigate suspicious user activity
    Given alert for unusual access pattern
    When initiating investigation
    Then investigation steps:
      | Step | Details |
      | 1. Review session logs | Full audit of user's session |
      | 2. Verify legitimacy | Contact user to confirm activities |
      | 3. Assess impact | Determine if any data compromised |
      | 4. Escalate if needed | Notify security/compliance |
      | 5. Document findings | Investigation report prepared |
      | 6. Remediate | Revoke access if necessary, reset MFA |
    And investigation report is filed
