Feature: Advanced Identity Management (BC12) - WebAuthn, OAuth2, API Keys, Session Management
  As an identity manager
  I want to manage WebAuthn/FIDO2 authentication, OAuth2 flows, API keys, password policies, and session management
  So that the bank ensures secure identity verification and access control

  Background:
    Given the system is initialized
    And I am authenticated as "identity_admin"
    And identity management system is operational

  # WebAuthn / FIDO2 Authentication
  @critical @fr-webauthn-001 @identity @webauthn
  Scenario: Register WebAuthn security key
    Given customer with registered phone + email
    When registering WebAuthn (FIDO2) security key
    Then registration process:
      | Step | Action |
      | 1. Device insertion | Physical security key inserted (e.g., YubiKey) |
      | 2. Challenge generation | Server generates cryptographic challenge |
      | 3. Device authentication | User confirms with PIN/biometric on key |
      | 4. Public key storage | Public key stored on server, private key on device |
      | 5. Registration complete | Device is now enrolled |
    And registration is recorded with device ID
    And device alias is set by user (e.g., "Work key")

  @critical @fr-webauthn-002 @identity @webauthn
  Scenario: Authenticate using WebAuthn
    Given customer with registered WebAuthn key
    When logging in to online banking
    Then authentication flow:
      | Step | Action |
      | 1. Username/password | Traditional credentials provided |
      | 2. WebAuthn prompt | "Insert security key and press button" |
      | 3. Device response | Key creates signature using private key |
      | 4. Server verification | Signature verified using stored public key |
      | 5. Login successful | Session created, user is authenticated |
    And authentication is cryptographically strong
    And phishing-resistant (site URL verified by device)

  @high @fr-webauthn-003 @identity @webauthn
  Scenario: Manage multiple WebAuthn devices
    Given customer with 2 registered security keys
    When reviewing registered devices
    Then device management:
      | Device | Alias | Registered | Last Used |
      | YubiKey 5 | Work Key | 2025-01-15 | 2025-04-05 |
      | Titan Security Key | Backup | 2025-02-01 | 2025-04-03 |
    And customer can:
      | Action |
      | Rename device alias |
      | Set one as primary |
      | Delete unused device |
      | Download recovery codes |
    And changes are logged

  @high @fr-webauthn-004 @identity @webauthn
  Scenario: Revoke compromised WebAuthn key
    Given WebAuthn key potentially compromised
    When customer reports device lost/compromised
    Then revocation process:
      | Action | Timing |
      | Device is immediately disabled | Instant |
      | Authentication with key is rejected | Immediate |
      | Notification to user | SMS + Email within 1 min |
      | Alternative auth methods offered | MFA via SMS/email |
      | Investigation initiated | 24 hours |
    And device cannot be re-activated

  # OAuth 2.0 Authentication Flow
  @critical @fr-oauth-001 @identity @oauth
  Scenario: Implement OAuth 2.0 authorization code flow
    Given third-party app requesting user access
    When user authorizes app via OAuth
    Then OAuth flow:
      | Step | Details |
      | 1. Authorization request | App redirects to BANKO login |
      | 2. User login | User authenticates at BANKO |
      | 3. Consent screen | User grants app permissions (Account Read, Payment Init) |
      | 4. Authorization code | BANKO redirects with authorization code |
      | 5. Token exchange | App exchanges code for access token (backend) |
      | 6. Access granted | App now has access within granted scope |
    And authorization code is single-use
    And access token expires per policy (8 hours default)

  @critical @fr-oauth-002 @identity @oauth
  Scenario: Validate OAuth 2.0 refresh token flow
    Given expired access token
    When app wants to maintain access
    Then refresh token flow:
      | Step |
      | App sends refresh token + client credentials |
      | Server validates refresh token (not revoked, not expired) |
      | New access token is issued |
      | Refresh token may be rotated |
      | Old access token is invalidated |
    And refresh token extends user session
    And no re-authentication required (within refresh token lifetime)

  @high @fr-oauth-003 @identity @oauth
  Scenario: Revoke OAuth 2.0 consent
    Given user with active OAuth consent
    When revoking consent
    Then revocation:
      | Action |
      | Consent status changed to "Revoked" |
      | All access tokens invalidated |
      | Refresh tokens invalidated |
      | App receives "unauthorized" on next API call |
      | User is notified |
      | Revocation is logged |
    And app must request new authorization for continued access

  @high @fr-oauth-004 @identity @oauth
  Scenario: Manage OAuth 2.0 client applications
    Given registered OAuth clients accessing BANKO
    When reviewing client applications
    Then client management:
      | Client App | Client ID | Scopes | Status |
      | ABC Payment App | abc-app-001 | payments, accounts | Active |
      | XYZ Analytics | xyz-app-002 | read:accounts | Suspended |
      | DEF Mobile Wallet | def-app-003 | payments, read:balance | Active |
    And admin can:
      | Action |
      | Revoke all tokens for client |
      | Update scope permissions |
      | Suspend client (reject new auths) |
      | Delete client application |

  # API Key Management
  @critical @fr-apikey-001 @identity @apikey
  Scenario: Generate API key for system integration
    Given system integration requiring API access
    When generating API key
    Then key generation:
      | Field | Value |
      | API Key ID | api-key-b47d8f2c |
      | API Key Secret | [redacted - shown once] |
      | Permissions | read:accounts, read:transactions |
      | Expiration | 1 year (2026-04-07) |
      | Rate limit | 1,000 requests/hour |
      | IP whitelist | 192.168.1.0/24 (optional) |
    And secret is displayed only once
    And secret cannot be recovered (must regenerate)

  @critical @fr-apikey-002 @identity @apikey
  Scenario: Validate API key on each request
    Given API request with API key authentication
    When BANKO receives request with header "Authorization: Bearer api-key-b47d8f2c"
    Then validation:
      | Check | Result |
      | API key exists | Found |
      | API key not expired | Valid (expires 2026-04-07) |
      | API key not revoked | Active |
      | Request count < limit (1000/hour) | OK (45 requests this hour) |
      | Request IP in whitelist | OK (from 192.168.1.10) |
      | Request scope permitted | OK (read:accounts granted) |
    And request proceeds
    And rate limit counter incremented

  @high @fr-apikey-003 @identity @apikey
  Scenario: Rotate API key for security
    Given API key in use for 6 months
    When rotating key
    Then rotation process:
      | Step |
      | 1. Generate new API key with same permissions |
      | 2. Both old and new keys work for 24-hour overlap |
      | 3. Application updates to use new key |
      | 4. Old key is disabled after 24 hours |
      | 5. Old key cannot be recovered |
    And rotation notification sent to key owner
    And rotation is logged

  @high @fr-apikey-004 @identity @apikey
  Scenario: Revoke compromised API key
    Given API key potentially exposed
    When key is compromised/exposed
    Then revocation:
      | Action | Timing |
      | Key is immediately disabled | Instant |
      | Requests with key are rejected | Immediate |
      | Owner is notified | SMS + Email |
      | Access logs reviewed | Investigation |
      | New key issued if needed | After review |
    And revoked key cannot be re-activated

  # Session Management
  @critical @fr-session-001 @identity @session
  Scenario: Create and manage user session
    Given user authenticated successfully
    When session is created
    Then session includes:
      | Element | Value |
      | Session ID | sess-f8d3a9bc (secure random) |
      | User ID | user-001 |
      | Login timestamp | 2025-04-07 10:30:00 |
      | Session timeout | 30 minutes inactivity |
      | Absolute timeout | 8 hours max |
      | IP address | 192.168.1.100 |
      | Device fingerprint | Browser hash + OS hash |
    And session is httponly/secure cookie
    And session is stored in secure session store

  @critical @fr-session-002 @identity @session
  Scenario: Enforce session timeout on inactivity
    Given user with active session
    When no activity for 30 minutes
    Then timeout enforcement:
      | Action |
      | Session is invalidated |
      | Next request redirects to login |
      | User is notified: "Session expired" |
      | Unsaved data may be lost |
      | New login required to continue |
    And inactivity clock resets on each activity
    And user is warned at 25-minute mark (optional)

  @high @fr-session-003 @identity @session
  Scenario: Prevent concurrent session exploitation
    Given user with existing session on Device A
    When user logs in from Device B
    Then concurrent session handling:
      | Option | Behavior |
      | Allow multiple sessions | Both Device A and B sessions active |
      | Force single session (strict) | Device A session invalidated |
      | Notify of new login | Email alert about Device B login |
    And security setting is configurable per customer
    And unusual device login triggers extra verification

  @high @fr-session-004 @identity @session
  Scenario: Detect and prevent session hijacking
    Given user session with potential hijack attempt
    When attacker uses stolen session cookie
    Then detection:
      | Check | Result |
      | Session cookie validity | Valid |
      | IP address match | MISMATCH (new IP detected) |
      | Device fingerprint match | MISMATCH (new device) |
      | Behavioral analytics | Unusual activity pattern |
    And session is challenged:
      | Challenge |
      | Re-authentication required |
      | Device verification (email/SMS) |
      | Risk assessment performed |
      | If high risk, session terminated |

  # Password Policy and Reset
  @critical @fr-pwd-policy-001 @identity @password
  Scenario: Enforce password policy
    Given password policy defined
    When user creates password
    Then policy validation:
      | Requirement | Example |
      | Minimum length | 12 characters minimum |
      | Complexity | Uppercase (A-Z), Lowercase (a-z), Digit (0-9), Special (!@#) |
      | Not in dictionary | Cannot be common words |
      | Not username-based | Cannot contain username |
      | History check | Cannot reuse last 10 passwords |
      | Entropy check | Password strength meter shows strength |
    And system displays real-time feedback
    And weak passwords are rejected

  @critical @fr-pwd-policy-002 @identity @password
  Scenario: Implement password reset flow
    Given user forgot password
    When requesting password reset
    Then reset process:
      | Step | Verification |
      | 1. Identity verification | Email + SMS code |
      | 2. User confirms identity | Provides correct email/phone |
      | 3. Reset link sent | Unique link sent to registered email |
      | 4. Link validity | Link expires in 24 hours |
      | 5. New password creation | User creates new password per policy |
      | 6. Confirmation | SMS + Email notification of password change |
    And old password is invalidated
    And active sessions are terminated
    And reset attempt is logged

  @high @fr-pwd-reset-003 @identity @password
  Scenario: Prevent password reset abuse
    Given multiple password reset requests
    When detecting potential reset spam/attack
    Then protection:
      | Protection | Limit |
      | Reset requests per hour | Max 3 |
      | Reset requests per day | Max 10 |
      | Failure limit | Max 3 failed attempts, then locked |
      | Lock duration | 24 hours |
      | Notification | User notified of abnormal activity |
    And excessive attempts trigger account lockout
    And manual intervention required to unlock

  # Biometric Authentication
  @high @fr-biometric-001 @identity @biometric
  Scenario: Register biometric authentication
    Given customer enabling biometric login
    When registering fingerprint or facial recognition
    Then registration:
      | Component |
      | Biometric template captured (multiple samples) |
      | Templates encrypted at rest |
      | Biometric stored on device only (if supported) |
      | Server stores only biometric hash (not image) |
      | User consent documented |
      | Biometric data destruction timeline set (on account close) |
    And registration with explicit user consent

  @high @fr-biometric-002 @identity @biometric
  Scenario: Authenticate using biometric
    Given customer with registered biometric
    When user requests biometric login
    Then biometric authentication:
      | Step | Action |
      | 1. Request biometric | Prompt for fingerprint/face |
      | 2. Capture | Device captures biometric |
      | 3. Match | Compares against stored template |
      | 4. Score | Similarity score calculated (min 99.5%) |
      | 5. Result | Accept or reject based on threshold |
      | 6. Fallback | If rejected, other auth methods available |
    And biometric cannot be used as sole auth for sensitive ops
    And always require additional factor (password, OTP) for sensitive transactions

  @high @fr-biometric-003 @identity @biometric
  Scenario: Manage biometric data privacy
    Given biometric data enrolled
    When handling biometric data
    Then privacy controls:
      | Control |
      | Biometric is never transmitted in clear |
      | Encryption in transit (TLS 1.3) |
      | Encryption at rest (AES-256) |
      | Limited retention (account lifespan) |
      | Secure deletion on closure or request |
      | No secondary use (only for authentication) |
      | GDPR compliance (user can request deletion) |
    And biometric data is NOT shared with third parties
    And data deletion audit trail maintained

  # Two-Factor Authentication (2FA)
  @critical @fr-2fa-001 @identity @2fa
  Scenario: Enable 2FA with multiple methods
    Given user enabling 2FA
    When configuring 2FA
    Then user can choose methods:
      | Method | Setup |
      | SMS OTP | Phone number verified |
      | Email OTP | Email confirmed |
      | Authenticator app | QR code scanned, backup codes provided |
      | WebAuthn security key | Device registered |
    And user selects primary + backup method
    And backup methods set in priority order

  @critical @fr-2fa-002 @identity @2fa
  Scenario: Execute 2FA challenge during login
    Given user with 2FA enabled
    When user logs in with password
    Then 2FA challenge:
      | Step |
      | 1. Password accepted |
      | 2. 2FA method prompt (SMS code) |
      | 3. SMS code sent to registered phone |
      | 4. Code expires in 10 minutes |
      | 5. User enters code |
      | 6. Server validates code |
      | 7. If valid, login completes |
      | If invalid, 3 retry attempts, then lockout |
    And code is single-use (cannot be reused)
    And 2FA is logged

  @high @fr-2fa-003 @identity @2fa
  Scenario: Handle 2FA backup codes
    Given user unable to receive 2FA code
    When user wants to use backup code
    Then backup code process:
      | Detail |
      | User has 10 backup codes generated at setup |
      | Each backup code is single-use |
      | Backup code format: 8-digit code |
      | Authenticates same as regular 2FA code |
      | Using backup code triggers regeneration request |
      | New codes must be generated and stored securely |
    And backup codes should be printed/stored offline

  # Account Lockout and Unlock
  @high @fr-lockout-001 @identity @lockout
  Scenario: Implement failed login attempt lockout
    Given account security policy
    When user fails login attempts
    Then lockout process:
      | Attempt | Status |
      | 1st failure | Normal - retry allowed |
      | 2nd failure | Warning shown |
      | 3rd failure | Account locked for 30 minutes |
      | 4th+ attempts | Rejected while locked |
      | After 30 min | Lockout expires, user can retry |
    And failed attempts are logged
    And alert sent to registered email

  @high @fr-lockout-002 @identity @lockout
  Scenario: Unlock account after lockout
    Given locked account
    When user contacts support or uses unlock process
    Then unlock options:
      | Option |
      | Automated unlock after lockout period expires |
      | Support staff unlock (verification required) |
      | Identity verification + password reset |
      | SMS code sent to verify identity |
    And unlock is logged with method
    And unlock attempt is verified

  # Identity Verification for Account Recovery
  @critical @fr-recovery-001 @identity @recovery
  Scenario: Implement account recovery procedure
    Given user unable to access account
    When initiating account recovery
    Then recovery steps:
      | Step | Verification |
      | 1. Identify user | Username or email |
      | 2. Verify identity | Security questions + email verification |
      | 3. Alternative verification | SMS to registered phone |
      | 4. Address verification | Confirm recent transaction address |
      | 5. Recovery code check | Optional: customer-provided recovery codes |
    And recovery must verify at least 2 factors
    And recovery process is logged
    And account reset can be offered if verification passes
