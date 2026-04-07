Feature: Notification Management - Preferences, Multi-Channel Delivery, Templates
  As a notification manager
  I want to manage customer notification preferences, multi-channel delivery, and templates
  So that customers receive timely, relevant, and personalized notifications across their preferred channels

  Background:
    Given the system is initialized
    And I am authenticated as "notification_admin"
    And notification system is operational

  # Notification Preference Management
  @critical @fr-notif-001 @notification @preferences
  Scenario: Configure customer notification preferences
    Given new customer onboarded
    When customer sets notification preferences
    Then preferences include:
      | Preference | Options |
      | Primary channel | Email, SMS, Push notification, All |
      | Transaction alerts | All, Large (>TND 50K), None |
      | Security alerts | Always enabled, Critical only |
      | Marketing | Enabled, Disabled |
      | Promotional offers | Enabled, Disabled |
      | Newsletter frequency | Weekly, Monthly, Never |
      | Language | Arabic, French, English |
      | Quiet hours | 22:00-08:00 (no notifications) |
    And preferences are stored per customer
    And customer can update anytime

  @high @fr-notif-002 @notification @preferences
  Scenario: Manage notification opt-in and opt-out
    Given customer with various notification subscriptions
    When managing opt-in/opt-out
    Then management includes:
      | Action | Effect |
      | Opt-in to marketing | Marketing emails start |
      | Opt-out of promotions | Promotional notifications stop |
      | Opt-out of SMS | SMS alerts disabled, email alternative |
      | Unsubscribe from newsletter | Newsletter emails stop |
      | Opt-out of all non-critical | Only critical alerts sent |
    And opt-in/out requests are logged
    And GDPR compliance ensured (explicit consent)

  @high @fr-notif-003 @notification @preferences
  Scenario: Honor quiet hours for notifications
    Given customer with quiet hours 22:00-08:00
    When scheduling notification during quiet period
    Then handling:
      | Scenario | Action |
      | Non-critical notification at 23:00 | Queued until 08:00 |
      | Critical security alert at 23:00 | Sent immediately (bypass quiet) |
      | Transaction confirmation at 07:30 | Queued until 08:00 |
      | Emergency notification | Always sent (fraud/security) |
    And queued notifications sent at 08:00 in batch

  @high @fr-notif-004 @notification @preferences
  Scenario: Customize notification frequency
    Given customer with high-volume transactions
    When customizing notification frequency
    Then frequency options:
      | Option | Behavior |
      | Real-time | Notification sent immediately |
      | Batched (hourly) | Notifications grouped, sent hourly |
      | Batched (daily) | Daily digest at specified time |
      | Weekly digest | All notifications summarized weekly |
      | Off | No notifications (critical only) |
    And customer selects per notification type
    And frequency preference is honored

  # Multi-Channel Notification Delivery
  @critical @fr-channel-001 @notification @channels
  Scenario: Send notification via email channel
    Given transaction requires notification
    When email channel is preferred
    Then email delivery:
      | Component |
      | Recipient verified email address |
      | Template-based content (branded) |
      | Subject line appropriate and actionable |
      | Content includes: date, amount, account, reference |
      | Footer with support contact info |
      | Unsubscribe link (if applicable) |
      | Sent from verified domain (banko.tn) |
    And email is delivered within 5 minutes
    And delivery status tracked (sent, delivered, opened)

  @critical @fr-channel-002 @notification @channels
  Scenario: Send notification via SMS channel
    Given notification requires SMS delivery
    When sending SMS alert
    Then SMS delivery:
      | Element |
      | Message concise (160 characters max) |
      | Includes key info: amount, type, reference |
      | Language matches customer preference |
      | Sent to verified phone number |
      | Delivery confirmation (DLR) tracked |
      | Rate limiting (max 5 SMS per hour per customer) |
    And SMS delivered within 1 minute
    And two-way SMS reply supported (customer can reply)

  @critical @fr-channel-003 @notification @channels
  Scenario: Send push notification to mobile app
    Given mobile app user
    When sending push notification
    Then push delivery:
      | Feature |
      | App must be installed (push permission granted) |
      | Notification displayed on lock screen |
      | Title + short message (up to 240 chars) |
      | Action button (Confirm/Dismiss/Detail) |
      | Sound/vibration per device settings |
      | Rich notification with icon/image |
      | Deep linking to relevant app screen |
    And notification delivered within 30 seconds
    And interaction logged (opened, clicked, dismissed)

  @high @fr-channel-004 @notification @channels
  Scenario: Send in-app notification to web/mobile portal
    Given customer logged into online banking
    When in-app notification triggered
    Then in-app delivery:
      | Feature |
      | Notification banner at top of portal |
      | "New notification" counter badge |
      | Notification center (inbox of all notifications) |
      | Clickable to view full details |
      | Mark as read / clear actions |
      | Notification lifespan: 30 days in history |
    And notification visible immediately if user is online
    And stored for review when user logs back in

  @high @fr-channel-005 @notification @channels
  Scenario: Select best channel when multiple preferred
    Given customer with SMS + Email + Push enabled
    When notification must be delivered
    Then channel selection logic:
      | Priority | Channel | Condition |
      | 1 | Push | App installed + user online |
      | 2 | SMS | High priority / urgent |
      | 3 | Email | Standard priority |
      | 4 | In-app | Fallback (user logs in later) |
    And system tries channels in priority order
    And confirms delivery on at least one channel

  # Notification Templates
  @high @fr-template-001 @notification @templates
  Scenario: Create notification template
    Given no templates for "Payment Confirmation"
    When creating template
    Then template definition:
      | Element |
      | Template ID: payment_confirmed_v1 |
      | Type: Transactional (non-marketing) |
      | Channels: Email, SMS, Push (select applicable) |
      | Language variations: Arabic, French, English |
      | Variables: {amount}, {recipient}, {date}, {reference} |
      | Subject (email): "TND {amount} transferred to {recipient}" |
      | Content (email): Full transaction details with branding |
      | Content (SMS): Concise summary in 160 chars |
      | Footer: Support contact, unsubscribe link |
    And template is saved as draft
    And approval workflow required before activation

  @high @fr-template-002 @notification @templates
  Scenario: Activate and version notification template
    Given draft template for "Account Alert"
    When activating template
    Then activation:
      | Step |
      | Template reviewed by compliance |
      | Language accuracy verified |
      | Sample notification preview approved |
      | Template version set to v1.0 |
      | Status changed to "Active" |
      | Effective date recorded |
    And active template used for new notifications
    And v1.0 can be used until v2.0 released

  @high @fr-template-003 @notification @templates
  Scenario: Update template and manage versions
    Given active template v1.0
    When updating for improved wording
    Then versioning:
      | Version | Status | Effective Date |
      | v1.0 | Superseded | 2025-02-01 |
      | v1.1 | Active | 2025-04-07 |
    And new notifications use v1.1
    And historical notifications reference correct version
    And old version retained for audit (2+ years)

  @medium @fr-template-004 @notification @templates
  Scenario: Personalize template with customer data
    Given notification template with variables
    When sending notification to customer
    Then personalization:
      | Variable | Replacement |
      | {customer_name} | "Ahmed Ben Ali" |
      | {account_number} | "ACC-001" (masked: ACC-**1) |
      | {amount} | "TND 500" |
      | {date} | "2025-04-07 14:30" |
      | {reference} | "REF-12345" |
    And rendered notification includes real data
    And sensitive data masked where appropriate

  # Notification Scheduling
  @high @fr-schedule-001 @notification @scheduling
  Scenario: Schedule notifications for future delivery
    Given transaction scheduled for future date
    When configuring scheduled notification
    Then scheduling:
      | Parameter |
      | Trigger event: Scheduled payment executes |
      | Delivery time: 2025-05-15 10:00 AM (customer timezone) |
      | Recipients: Primary account holder + secondary |
      | Channels: Email + SMS |
      | Retry on failure: 3 attempts over 24 hours |
    And notification queued for execution
    And execution audit trail maintained

  @high @fr-schedule-002 @notification @scheduling
  Scenario: Handle scheduled notification cancellation
    Given scheduled notification queued
    When source event is cancelled (e.g., payment reversed)
    Then notification cancellation:
      | Action |
      | Scheduled notification is cancelled |
      | Notification is not sent |
      | Cancellation logged with reason |
      | Related notifications also cancelled |
    And customer does not receive unnecessary notification

  @medium @fr-schedule-003 @notification @scheduling
  Scenario: Bulk schedule notifications for campaign
    Given marketing campaign launch date
    When scheduling bulk campaign notifications
    Then bulk scheduling:
      | Element |
      | Recipient list: 50,000 customers (filtered criteria) |
      | Message: "Special offer: 0% balance transfer" |
      | Scheduled send: 2025-04-15 09:00 AM |
      | Channels: Email + SMS (per preference) |
      | Personalization: First name, offer code |
      | Opt-out handling: Skip opted-out customers |
    And all notifications sent within 10-minute window
    And delivery metrics captured per channel

  # Notification Performance and Monitoring
  @high @fr-monitoring-001 @notification @monitoring
  Scenario: Track notification delivery status
    Given daily notification volume
    When monitoring delivery performance
    Then metrics tracked:
      | Metric | Value |
      | Total notifications sent | 125,000 |
      | Delivery confirmed (delivered) | 122,500 (98%) |
      | Failed (hard bounce) | 2,000 (1.6%) |
      | Bounced - wrong address/number | 500 (0.4%) |
      | Bounced - opted out | - (not attempted) |
    And failed notifications retried
    And addresses/numbers flagged for cleanup

  @high @fr-monitoring-002 @notification @monitoring
  Scenario: Analyze notification engagement metrics
    Given email notifications sent (10,000)
    When analyzing engagement
    Then engagement metrics:
      | Metric | Value |
      | Delivered | 9,800 (98%) |
      | Opened | 6,860 (68.6% of delivered) |
      | Clicked | 2,060 (20.6% of delivered) |
      | Unsubscribed | 12 (0.12%) |
      | Marked as spam | 5 (0.05%) |
      | Bounce rate | 2% |
    And high engagement indicates effective messaging
    And low engagement indicates need for template refresh

  @medium @fr-monitoring-003 @notification @monitoring
  Scenario: Generate notification delivery report
    Given monthly notification activities
    When generating report
    Then report includes:
      | Section |
      | Total notifications by type (transactional, marketing, alert) |
      | Channel breakdown (email, SMS, push, in-app) |
      | Delivery success rate |
      | Average delivery time |
      | Engagement metrics (email open rate) |
      | Failed deliveries by reason |
      | Customer opt-out trend |
      | Top templates used |
    And report is distributed to stakeholders

  # Notification Compliance and Opt-Out
  @critical @fr-compliance-001 @notification @compliance
  Scenario: Respect marketing opt-out per GDPR/INPDP
    Given customer opted out of marketing
    When marketing notification is due
    Then system enforces:
      | Check |
      | Customer unsubscribed status verified |
      | Notification is NOT sent |
      | Opt-out request is honored immediately |
      | Historical opt-out status checked (no re-subscription) |
      | Audit log confirms non-send |
    And system prevents accidental re-subscription
    And unsubscribe link always included in marketing emails

  @critical @fr-compliance-002 @notification @compliance
  Scenario: Implement Do Not Call list compliance
    Given SMS marketing campaign ready
    When filtering recipient list
    Then compliance filtering:
      | Filter |
      | Remove Do Not Call registry numbers |
      | Remove customer-requested opt-out SMS |
      | Remove invalid phone numbers |
      | Remove numbers with >3 bounces |
      | Remaining list: 45,000 of original 50,000 |
    And SMS sent only to compliant numbers
    And non-compliance risks avoided

  @high @fr-compliance-003 @notification @compliance
  Scenario: Generate consent audit trail for notifications
    Given notification with consent requirement
    When customer receives notification
    Then audit trail includes:
      | Entry |
      | Consent given: Yes (2025-04-01 10:00) |
      | Consent type: Marketing communications |
      | Consent channel: Online banking portal |
      | IP address: 192.168.1.100 |
      | Notification sent: Yes (2025-04-07 09:00) |
      | Delivery confirmed: Yes (email opened) |
    And audit proves compliance with consent requirement

  # Critical and Emergency Notifications
  @critical @fr-critical-001 @notification @emergency
  Scenario: Send critical security notification immediately
    Given potential fraud detected on account
    When critical alert required
    Then immediate delivery:
      | Action |
      | Bypass quiet hours (if configured) |
      | Send via all enabled channels simultaneously |
      | Priority delivery (top of queue) |
      | SMS sent immediately (no batching) |
      | Email marked high priority |
      | Push notification with red alert |
      | In-app banner displayed prominently |
    And delivery within 30 seconds
    And no rate limiting applied

  @critical @fr-critical-002 @notification @emergency
  Scenario: Escalate critical notification if not acknowledged
    Given critical notification sent
    When customer does not acknowledge within 30 minutes
    Then escalation:
      | Step | Timing |
      | 1. Resend via different channel | T+15 min |
      | 2. If no response, SMS alert | T+30 min |
      | 3. If no response, phone call (IVR) | T+45 min |
      | 4. If no response, escalate to support | T+60 min |
    And escalation continues until acknowledged
    And support team may contact manually

  @high @fr-critical-003 @notification @emergency
  Scenario: Test notification system reliability
    Given notification system in production
    When running synthetic test alerts
    Then testing:
      | Test Type |
      | Email delivery test (sample) |
      | SMS delivery test (sample) |
      | Push notification test |
      | End-to-end latency measurement |
      | Failover mechanisms (if one channel down) |
      | Performance under load (1000 simultaneous) |
    And tests run daily/weekly per criticality
    And results published in system status dashboard

  # Notification Troubleshooting
  @high @fr-troubleshoot-001 @notification @troubleshooting
  Scenario: Diagnose notification delivery failure
    Given notification failed to deliver
    When investigating failure
    Then diagnosis:
      | Check | Result | Action |
      | Email address valid | No (malformed) | Flag for cleanup |
      | Phone number valid | Yes | Check SMS provider |
      | Provider delivery | Failed (queued) | Check SMS gateway logs |
      | Retry attempts | 2 of 3 | Will retry again |
      | Customer opt-out | No | Continue investigation |
    And root cause documented
    And remediation action initiated

  @high @fr-troubleshoot-002 @notification @troubleshooting
  Scenario: Manually retry failed notification
    Given notification failed to deliver
    When operations team initiates manual retry
    Then retry process:
      | Step |
      | Previous failure logged |
      | Alternative contact info checked |
      | Notification sent to verified address |
      | Delivery status monitored |
      | Customer notified if previously missing communication |
    And manual retry logged separately
    And success/failure recorded
