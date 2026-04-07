Feature: Advanced Payment Processing (BC9) - PSD3, Open Banking, Instant Payments, QR
  As a payment operations manager
  I want to manage Open Banking consent, instant payments, QR payments, and third-party initiated transactions
  So that the bank offers modern payment services compliant with PSD3 and regulatory requirements

  Background:
    Given the system is initialized
    And I am authenticated as "payment_manager"
    And payment processing is enabled

  # Open Banking PSD3 Consent Management
  @critical @fr-openbank-001 @payment @consent
  Scenario: Request Open Banking consent from customer
    Given customer accessing third-party Open Banking app
    When TPP (Third Party Provider) requests access
    Then consent request includes:
      | Consent Element |
      | Access to account information |
      | Read transaction history (last 90 days) |
      | Account balances |
      | Initiate payments (limited to TND 1,000 per transaction) |
      | Scope: Current Accounts only |
    And customer is presented with consent screen
    And clear disclosure of data being shared is shown

  @critical @fr-openbank-002 @payment @consent
  Scenario: Grant granular Open Banking consent
    Given consent request from TPP
    When customer grants consent
    Then consent is recorded with:
      | Field | Value |
      | TPP Identifier | XYZ Payment App |
      | Consent Type | AccountInformation + PaymentInitiation |
      | Scope | Current Account (ACC-001) |
      | Permissions | Read: Balance, Transactions; Write: Payments <= TND 1,000 |
      | Validity | 90 days (auto-refresh available) |
      | Revocation | Customer can revoke anytime |
    And consent token is issued to TPP
    And consent is logged in audit trail

  @high @fr-openbank-003 @payment @consent
  Scenario: Revoke Open Banking consent
    Given active Open Banking consent with TPP
    When customer revokes consent in online banking
    Then revocation occurs:
      | Action |
      | Consent status changes to "Revoked" |
      | TPP is notified of revocation |
      | No further API access allowed |
      | Existing valid tokens expire within 24 hours |
      | Access logs are retained |
    And confirmation is sent to customer

  @high @fr-openbank-004 @payment @consent
  Scenario: Manage Open Banking consent lifecycle
    Given multiple active consents from different TPPs
    When generating monthly consent report
    Then report shows:
      | Metric | Value |
      | Total active consents | 45 |
      | By permission (AIS - Account Info) | 28 |
      | By permission (PIS - Payment Init) | 17 |
      | Consents near expiry (< 30 days) | 8 |
      | Revoked this month | 3 |
    And expiring consents trigger renewal reminders

  @medium @fr-openbank-005 @payment @consent
  Scenario: Implement strong customer authentication for Open Banking
    Given TPP initiating payment via Open Banking with SCA required
    When customer authorizes transaction (TND 2,500)
    Then SCA process includes:
      | Method |
      | SMS OTP to registered phone |
      | Biometric (if device-enabled) |
      | Push notification + approval |
    And transaction proceeds only after successful authentication
    And authentication method is logged

  # Instant Payments (TIPS/SEPA Instant Credit Transfer)
  @critical @fr-instant-001 @payment @instant
  Scenario: Process instant payment request
    Given customer initiating instant payment
    When entering payment details:
      | Field | Value |
      | Beneficiary IBAN | FR1420041010050500013M02606 |
      | Amount | EUR 500 |
      | Description | Invoice 12345 |
    Then payment is submitted for instant processing
    And receipt is provided immediately
    And payment status is "Accepted"

  @critical @fr-instant-002 @payment @instant
  Scenario: Execute instant credit transfer in 10 seconds
    Given instant payment request accepted
    When processing through TIPS (Target Instant Payment Settlement)
    Then transfer occurs:
      | Timeline | Event |
      | T+0 sec | Payment validated |
      | T+2 sec | Debit customer account |
      | T+8 sec | Credit sent to beneficiary bank |
      | T+10 sec | Beneficiary receives credit |
    And end-to-end execution = < 10 seconds
    And confirmation sent to both parties

  @high @fr-instant-003 @payment @instant
  Scenario: Retry instant payment on first attempt failure
    Given instant payment request that fails initially
    When failure occurs (beneficiary bank unavailable)
    Then automatic retry:
      | Attempt | Timing | Outcome |
      | 1st | Immediate | Failed |
      | 2nd | 5 seconds | Failed |
      | 3rd | 30 seconds | Success |
    And transaction eventually succeeds
    And customer is notified once successful

  @high @fr-instant-004 @payment @instant
  Scenario: Set instant payment limits and controls
    Given customer with instant payment limit
    When configuring limits
    Then limits are:
      | Control | Setting |
      | Daily limit | EUR 10,000 |
      | Per-transaction limit | EUR 5,000 |
      | Monthly limit | EUR 50,000 |
      | Velocity (transactions per hour) | 10 maximum |
    And limits are enforced by system
    And exceeding limit triggers alert

  @medium @fr-instant-005 @payment @instant
  Scenario: Reverse instant payment after transmission
    Given instant payment executed 2 minutes ago
    When customer requests reversal
    Then reversal process:
      | Condition | Outcome |
      | Beneficiary bank supports return | Automatic reversal initiated |
      | 24 hours not passed | Return is likely successful |
      | Receiver already withdrew funds | Reversal may fail (manual follow-up) |
    And reversal status is tracked
    And customer is notified of outcome

  # QR Code Payments
  @high @fr-qr-001 @payment @qr
  Scenario: Generate QR code for payment request
    Given merchant creating payment request
    When generating QR code
    Then QR code encodes:
      | Field |
      | Merchant identifier |
      | Amount (TND 150) |
      | Payment reference |
      | Merchant name |
      | Validity period (15 minutes) |
    And QR code is displayed on POS/invoice
    And QR contains secure hash to prevent tampering

  @high @fr-qr-002 @payment @qr
  Scenario: Scan and process QR code payment
    Given customer at merchant location
    When scanning QR code with mobile banking app
    Then app decodes QR and displays:
      | Detail |
      | Merchant name |
      | Payment amount |
      | Transaction reference |
      | "Confirm" button |
    And customer confirms with SCA (if required)
    And payment is processed immediately
    And receipt is generated

  @high @fr-qr-003 @payment @qr
  Scenario: Track QR payment volume and metrics
    Given daily QR payments processed
    When generating QR payment report
    Then metrics include:
      | Metric | Value |
      | QR codes generated | 450 |
      | QR payments completed | 380 |
      | Completion rate | 84% |
      | Average transaction amount | TND 125 |
      | Total transaction value | TND 47,500 |
    And trends are analyzed (daily, weekly, monthly)

  @medium @fr-qr-004 @payment @qr
  Scenario: Implement QR payment fraud detection
    Given QR payment with unusual pattern
    When processing QR transaction with:
      | Anomaly |
      | Device location changed significantly (1000 km) |
      | Time difference unusual (customer normally active 9-5) |
      | Amount double customer's typical transaction |
    Then fraud check is triggered:
      | Action |
      | Transaction is held for review |
      | Device fingerprint is verified |
      | OTP verification may be required |
      | Transaction proceeds if verified, else rejected |

  # Third-Party Initiated Payments (TIP)
  @critical @fr-tip-001 @payment @tip
  Scenario: Enable third-party initiated payment service
    Given customer consents to TIP (e.g., subscription service)
    When TIP service makes payment request
    Then request includes:
      | Field |
      | Service identifier (subscription provider) |
      | Mandate reference |
      | Amount (within approved limit) |
      | Frequency (monthly) |
    And BANKO validates:
      | Validation |
      | Consent is active |
      | Amount is within limits |
      | Frequency is as agreed |
      | Mandate has not expired |

  @critical @fr-tip-002 @payment @tip
  Scenario: Process recurring TIP payment
    Given active TIP consent (Netflix subscription)
    When monthly subscription payment is due
    Then automated payment:
      | Step |
      | TIP request received from provider |
      | Mandate verification (valid, within limits) |
      | Customer account debited (TND 45) |
      | Provider account credited (TND 45) |
      | Confirmation sent to customer (email/SMS) |
    And transaction is marked as "TIP - Recurring"

  @high @fr-tip-003 @payment @tip
  Scenario: Revoke TIP mandate and prevent future payments
    Given active TIP consent/mandate
    When customer revokes TIP mandate
    Then revocation:
      | Action |
      | Mandate status changes to "Revoked" |
      | TIP service is notified |
      | Future TIP payments rejected |
      | Customer receives confirmation |
      | Revocation is logged |
    And service must obtain new consent for future payments

  @high @fr-tip-004 @payment @tip
  Scenario: Dispute unauthorized TIP payment
    Given TIP payment received (TND 45)
    When customer disputes unauthorized payment
    Then dispute process:
      | Step | Timeline |
      | 1. Dispute filed | Immediate |
      | 2. Investigation initiated | Within 24 hours |
      | 3. Provisional credit (if eligible) | Within 10 days |
      | 4. Provider response | Within 30 days |
      | 5. Final determination | Within 45 days |
    And evidence is gathered (consent records, mandates)
    And resolution is determined

  # Payment Routing and Clearing
  @critical @fr-routing-001 @payment @routing
  Scenario: Determine optimal payment routing
    Given outgoing SEPA transfer to France
    When payment routing engine evaluates options
    Then routing considers:
      | Route | Time | Cost | Reliability |
      | TIPS (Instant) | 10 sec | EUR 0.50 | 99.9% |
      | SEPA SCT (Standard) | 1 day | EUR 0.25 | 100% |
      | Correspondent Bank | 3-5 days | EUR 5.00 | 99% |
    And optimal route is selected: "TIPS"
    And payment follows selected routing

  @high @fr-routing-002 @payment @routing
  Scenario: Reconcile payment clearing with correspondent banks
    Given daily clearing batches sent to correspondents
    When reconciling end-of-day
    Then reconciliation:
      | Batch | Sent | Confirmed | Status |
      | SWIFT MT103 (30 payments) | 15:00 | 15:15 | Reconciled |
      | SEPA SCT (150 payments) | 15:30 | 16:00 | Reconciled |
      | Correspondent wire (20) | 16:00 | 16:30 | Reconciled |
    And all batches are accounted for
    And exceptions are flagged for follow-up

  # Multi-Currency Payments
  @high @fr-multicur-001 @payment @forex
  Scenario: Execute multi-currency payment with FX conversion
    Given customer account in TND, paying in EUR
    When customer initiates EUR 1,000 payment
    Then FX conversion:
      | Step | Rate | Amount |
      | 1. Get spot rate | 1 EUR = 3.15 TND | TND 3,150 |
      | 2. Apply FX margin | +2.5% | TND 3,225.75 |
      | 3. Final amount charged | TND 3,225.75 | EUR 1,000 delivered |
    And FX rate and margin are disclosed
    And customer approves before execution

  @high @fr-multicur-002 @payment @forex
  Scenario: Manage multi-currency liquidity
    Given daily multi-currency payment requests
    When monitoring currency positions
    Then positions include:
      | Currency | Long/Short | Amount |
      | EUR | Long | EUR 250,000 |
      | USD | Long | USD 500,000 |
      | GBP | Short | GBP 100,000 |
    And FX traders manage positions
    And hedge strategies are executed if needed

  # Payment Fraud Detection
  @critical @fr-fraud-001 @payment @fraud
  Scenario: Detect fraudulent payment attempt
    Given customer payment with anomalies
    When processing payment for EUR 50,000 with:
      | Anomaly |
      | Device IP location = foreign country |
      | Device fingerprint unknown |
      | Amount 20x customer's typical transaction |
      | Beneficiary = new recipient (not in whitelist) |
      | Time outside customer's normal activity window |
    Then fraud detection triggers:
      | Action |
      | Transaction is declined |
      | Customer receives alert |
      | Verification required (call + SMS) |
      | Transaction approved/denied based on verification |

  @high @fr-fraud-002 @payment @fraud
  Scenario: Implement velocity checks for payments
    Given velocity limits per customer
    When customer attempts multiple transactions
    Then velocity rules:
      | Rule | Limit | Current |
      | Transactions per hour | 5 | 5 reached |
      | Daily transaction count | 20 | 18 used |
      | Daily amount limit | EUR 100,000 | EUR 78,000 used |
    And 6th transaction is declined: "VelocityLimitExceeded"
    And customer is informed of limits

  @high @fr-fraud-003 @payment @fraud
  Scenario: Track and report payment fraud metrics
    Given monthly fraud detection results
    When generating fraud report
    Then report includes:
      | Metric | Value |
      | Transactions monitored | 125,000 |
      | Fraud alerts triggered | 890 |
      | Confirmed fraud cases | 23 |
      | False positive rate | 97.4% |
      | Fraud loss prevented (TND) | 1,250,000 |

  # Cross-Border Payments and Compliance
  @critical @fr-xcountry-001 @payment @xcross
  Scenario: Screen cross-border payment for sanctions
    Given outgoing payment to Iran-based entity (high-risk)
    When processing international payment
    Then compliance checks:
      | Check | Result |
      | Beneficiary country sanctions | Iran - OFAC/UN listed |
      | Beneficiary entity screening | Matched to SDN list |
      | Purpose (narration) | "Technology transfer" - restricted |
    And payment is REJECTED
    And transaction cannot proceed
    And SAR filing is considered

  @critical @fr-xcountry-002 @payment @xcross
  Scenario: Validate beneficiary information for SWIFT
    Given SWIFT payment to international account
    When sending via SWIFT MT103
    Then validation includes:
      | Field | Requirement |
      | Beneficiary IBAN | Valid format per country |
      | Beneficiary name | Complete and verified |
      | BIC/SWIFT | Valid and matching IBAN |
      | Intermediary Bank | If required for routing |
    And SWIFT message is validated
    And Payment is transmitted only if all fields valid

  # Payment Exceptions and Follow-up
  @high @fr-exception-001 @payment @exception
  Scenario: Handle payment return/rejection
    Given payment rejected by beneficiary bank
    When return reason received:
      | Reason | Action |
      | Invalid IBAN | Contact customer, request correction |
      | Account closed | Research alternative account |
      | Insufficient information | Resubmit with corrected details |
    Then return process:
      | Step | Timeline |
      | 1. Return received | T+1 day |
      | 2. Customer contacted | T+1 day |
      | 3. Funds re-credited | T+2 days |
      | 4. Correction obtained | T+5 days |
      | 5. Re-transmission | T+6 days |

  @high @fr-exception-002 @payment @exception
  Scenario: Generate payment operations report
    Given daily payment activities
    When generating operations report
    Then report includes:
      | Metric |
      | Total payments processed | 1,250 |
      | Domestic transfers | 950 |
      | International transfers | 200 |
      | Instant payments | 100 |
      | Failed/returned | 12 |
      | Exceptions requiring follow-up | 8 |
      | Average processing time | 2.5 hours |

  # Real-Time Payment Notifications
  @high @fr-notify-001 @payment @notification
  Scenario: Send real-time payment confirmation
    Given payment successfully processed
    When transaction completes
    Then customer receives notification:
      | Channel | Content |
      | SMS | "TND 500 transferred to ACC-XYZ at 15:34" |
      | Email | Detailed transaction receipt with full details |
      | In-app | Real-time update in banking app |
      | SWIFT | Optional for international payments |
    And notification includes transaction reference
    And timestamp is recorded

  @medium @fr-notify-002 @payment @notification
  Scenario: Alert on unusual payment activity
    Given baseline customer payment profile
    When activity deviates significantly
    Then alert is sent:
      | Alert Type |
      | "Payment amount 5x higher than usual" |
      | "New beneficiary - please confirm" |
      | "International payment to unfamiliar country" |
      | "Multiple payments in short time" |
    And customer can confirm or block transaction
