# INPDP Compliance Implementation - Story Reference

This document provides a quick reference for the INPDP compliance features implemented in BANKO's compliance bounded context (BC13).

## Stories Implemented

### STORY-COMP-06: Migrate Consent to BC13

**Domain Model**:
```
InpdpConsent (Aggregate Root)
├── id: InpdpConsentId
├── customer_id: Uuid
├── purpose: ConsentPurpose (Marketing | Analytics | ThirdPartySharing | Profiling | CrossBorder)
├── legal_basis: LegalBasis (Consent | ContractualNecessity | LegalObligation | VitalInterest | PublicInterest | LegitimateInterest)
├── data_categories: Vec<String>
├── granted: bool
├── granted_at: Option<DateTime<Utc>>
├── revoked_at: Option<DateTime<Utc>>
└── expiry_date: Option<DateTime<Utc>>

Key Methods:
- new(customer_id, purpose, legal_basis, data_categories, expiry_date) -> InpdpConsent
- revoke() -> Result<(), DomainError>
- is_valid() -> bool (checks granted + not revoked + not expired)
```

**File**: `/crates/domain/src/compliance/entities.rs` (lines ~1300+)

---

### STORY-COMP-07: Consent Dashboard Endpoints

**Service**: `InpdpConsentService`

```rust
// Methods
pub async fn grant_consent(
    customer_id: Uuid,
    purpose: &str,
    legal_basis: &str,
    data_categories: Vec<String>,
    expiry_days: Option<i64>,
) -> Result<InpdpConsent, ComplianceError>

pub async fn revoke_consent(consent_id: &InpdpConsentId) -> Result<InpdpConsent, ComplianceError>

pub async fn get_consent_dashboard(customer_id: Uuid) 
    -> Result<(i64, i64, i64, HashMap<String, i64>), ComplianceError>
    // Returns: (total, active, revoked, by_purpose_counts)

pub async fn list_consents_by_customer(customer_id: Uuid) 
    -> Result<Vec<InpdpConsent>, ComplianceError>
```

**Request/Response DTOs**:
- `GrantConsentRequest`: {customer_id, purpose, legal_basis, data_categories, expiry_days}
- `RevokeConsentRequest`: {customer_id, consent_id}
- `ConsentResponse`: {id, customer_id, purpose, granted, granted_at, revoked_at, expiry_date, legal_basis, data_categories}
- `ConsentDashboardResponse`: {customer_id, total_consents, active_consents, revoked_consents, consents_by_purpose, generated_at}

**Repository Port**: `IInpdpConsentRepository`

**File**: `/crates/application/src/compliance/service.rs` (lines ~20-120)

---

### STORY-COMP-08: DPIA + Breach Notification

#### DPIA (Data Protection Impact Assessment)

**Domain Model**:
```
Dpia (Aggregate Root)
├── id: DpiaId
├── title: String
├── description: String
├── processing_activity: String
├── risk_assessment: String
├── mitigations: Vec<String>
├── status: DpiaStatus (Draft | UnderReview | Approved | Rejected)
├── created_at: DateTime<Utc>
├── approved_by: Option<String>
└── approved_at: Option<DateTime<Utc>>

State Machine:
Draft -> UnderReview -> Approved
              └----> Rejected
```

**Service**: `DpiaService`

```rust
pub async fn create_dpia(...) -> Result<Dpia, ComplianceError>
pub async fn submit_dpia_for_review(dpia_id: &DpiaId) -> Result<Dpia, ComplianceError>
pub async fn approve_dpia(dpia_id: &DpiaId, approved_by: String) -> Result<Dpia, ComplianceError>
pub async fn reject_dpia(dpia_id: &DpiaId) -> Result<Dpia, ComplianceError>
pub async fn list_dpias_by_status(status: &str) -> Result<Vec<Dpia>, ComplianceError>
```

#### Breach Notification (72-Hour Compliance)

**Domain Model**:
```
BreachNotification (Aggregate Root)
├── id: BreachNotificationId
├── breach_type: String
├── description: String
├── affected_data: Vec<String>
├── affected_count: u32
├── detected_at: DateTime<Utc>
├── notified_authority_at: Option<DateTime<Utc>> (must be within 72h)
├── notified_subjects_at: Option<DateTime<Utc>>
└── status: BreachStatus (Detected | AuthorityNotified | SubjectsNotified | Resolved)

Key Compliance: Authority notification MUST occur within 72 hours of detection
```

**Service**: `BreachNotificationService`

```rust
pub async fn report_breach(...) -> Result<BreachNotification, ComplianceError>
pub async fn notify_authority(breach_id: &BreachNotificationId) 
    -> Result<BreachNotification, ComplianceError>
    // Validates 72h deadline
pub async fn notify_subjects(breach_id: &BreachNotificationId) 
    -> Result<BreachNotification, ComplianceError>
pub async fn resolve_breach(breach_id: &BreachNotificationId) 
    -> Result<BreachNotification, ComplianceError>
pub async fn check_72h_compliance() -> Result<i64, ComplianceError>
    // Returns count of overdue breaches
```

**Files**: 
- Domain: `/crates/domain/src/compliance/entities.rs` (lines ~1400+)
- Service: `/crates/application/src/compliance/service.rs` (lines ~300-450)

---

### STORY-COMP-09: Data Portability + Erasure

#### Data Portability

**Service**: `DataPortabilityService`

```rust
pub async fn request_data_portability(
    customer_id: Uuid,
    reason: Option<String>,
) -> Result<DataPortabilityRequest, ComplianceError>

pub async fn list_requests(customer_id: Uuid) 
    -> Result<Vec<DataPortabilityRequest>, ComplianceError>
```

**Request/Response DTOs**:
- `RequestDataPortabilityRequest`: {customer_id, reason}
- `DataPortabilityResponse`: {request_id, customer_id, status, requested_at, scheduled_for}

#### Right to Erasure with Retention Compliance

**Service**: `ErasureService`

```rust
pub async fn request_erasure(
    customer_id: Uuid,
    reason: Option<String>,
) -> Result<ErasureRequest, ComplianceError>

pub fn check_erasure_eligibility(customer_closed_at: Option<DateTime<Utc>>) 
    -> (bool, String)
    // Returns: (is_eligible, reason_message)
    // Regulation: 7-year retention for financial records (Tunisia banking)
```

**Request/Response DTOs**:
- `RequestErasureRequest`: {customer_id, reason}
- `ErasureEligibilityResponse`: {customer_id, is_eligible, reason, checked_at}
- `ErasureRequestResponse`: {request_id, customer_id, status, requested_at, scheduled_for}

**File**: `/crates/application/src/compliance/service.rs` (lines ~500-650)

---

## Testing

All implementations include comprehensive unit tests:

### Domain Tests (28 test cases)
- Consent lifecycle (grant, revoke, expiry validation)
- DPIA state machine (Draft -> UnderReview -> Approved/Rejected)
- Breach notification workflow (72h compliance)
- Purpose and legal basis parsing

### Service Tests (13 test cases)
- Consent dashboard calculations
- DPIA approval workflow
- Breach notification deadline enforcement
- Retention period validation

### Mock Repositories
- `MockConsentRepository`: In-memory consent storage
- `MockDpiaRepository`: In-memory DPIA storage
- `MockBreachRepository`: In-memory breach storage

Run tests with: `make test-unit`

---

## Architecture

### Layers

**Domain Layer** (`/crates/domain/src/compliance/`):
- Value objects: InpdpConsentId, DpiaId, BreachNotificationId
- Aggregates: InpdpConsent, Dpia, BreachNotification
- Enums: ConsentPurpose, LegalBasis, DpiaStatus, BreachStatus
- No external dependencies (pure business logic)

**Application Layer** (`/crates/application/src/compliance/`):
- Services: InpdpConsentService, DpiaService, BreachNotificationService, DataPortabilityService, ErasureService
- DTOs: Request/Response types for API contracts
- Ports: Repository interfaces (IInpdpConsentRepository, IDpiaRepository, IBreachNotificationRepository, IDataPortabilityRepository, IErasureRepository)
- Error handling: ComplianceError enum

**Infrastructure Layer** (To be implemented):
- PostgreSQL repositories implementing ports
- HTTP handlers in Actix-web
- Database migrations

---

## Regulatory Compliance

### INPDP Requirements
- Consent purposes and legal basis aligned with Tunisian data protection law
- Data categories configurable per consent
- Right to revoke consent anytime
- Expiration date support

### GDPR-Equivalent Features
- 72-hour breach notification to authorities (GDPR Art. 33)
- Data subject notification after authority notification
- Data portability request support
- Right to erasure with retention validation

### Tunisian Banking Regulation
- 7-year retention period for financial records
- Erasure eligibility check prevents premature deletion
- Audit trail via status tracking

---

## Next Steps

To complete the implementation, add to Infrastructure Layer:

1. **Database Migrations**
   - `inpdp_consents` table
   - `dpias` table
   - `breach_notifications` table
   - `data_portability_requests` table
   - `erasure_requests` table

2. **Repository Implementations**
   - PostgreSQL repositories in `/crates/infrastructure/src/compliance/repositories/`

3. **HTTP Handlers**
   - Consent endpoints in `/crates/infrastructure/src/web/handlers/compliance/`
   - DPIA endpoints
   - Breach notification endpoints
   - Data portability/erasure endpoints

4. **Routes Registration**
   - Update `/crates/infrastructure/src/web/routes.rs`

5. **Integration Tests**
   - E2E tests for complete flows
   - Deadline validation tests
   - Retention period validation tests

---

**Last Updated**: April 6, 2026
**Status**: Domain & Application layers complete, Infrastructure layer pending
**Test Coverage**: 41 unit tests included
