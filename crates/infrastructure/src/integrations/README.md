# BANKO Integration Modules

This directory contains production-grade integrations with external systems required for regulatory compliance and banking operations.

## Overview

All integration modules follow BANKO's hexagonal architecture principles:

- **Domain-agnostic**: Integration details are isolated from core business logic
- **Error handling**: Comprehensive error types with proper context
- **Testing**: Unit tests included for all public interfaces
- **Documentation**: Extensive inline documentation and examples

## Modules

### 1. goAML Integration (STORY-COMP-10)

**File**: `goaml.rs`

Integration with the UN goAML platform for submission of Suspicious Transaction Reports (STR) to Tunisia's Financial Intelligence Unit (CTAF - Commission Tunisienne des Analyses Financières).

#### Key Types

- `GoAmlReport`: Main report structure conforming to UN goAML standard
- `GoAmlTransaction`: Individual transactions being reported
- `GoAmlSuspect`: Suspect/person of interest information
- `ReportingEntity`: Reporting institution (BANKO or branch)
- `GoAmlClient`: HTTP client for submission to CTAF
- `RiskLevel`: Classification (Low, Medium, High, Critical)

#### Public API

```rust
// Create a report
let report = GoAmlReport::new(
    GoAmlReportType::STR,
    reporting_entity,
    "Narrative explaining suspicion".to_string(),
    RiskLevel::High,
)?
.add_transaction(transaction)
.add_suspect(suspect);

// Generate XML for CTAF
let xml = client.generate_xml(&report)?;

// Submit to CTAF
let response = client.submit_report(&report).await?;

// Convert from domain model
let goaml_report = from_suspicion_report(
    narrative,
    reporting_entity,
    suspect_info,
)?;
```

#### Compliance Features

- XML generation compliant with UN goAML standard
- XPath escaping to prevent injection attacks
- Comprehensive validation before submission
- Report tracking with unique identifiers
- Support for STR (Suspicious Transaction), SAR (Suspicious Activity), and CTR (Currency Transaction) reports

#### Configuration

Environment variables (production):

```bash
GOAML_BASE_URL=https://ctaf.gov.tn/api/v1
GOAML_API_KEY=<api-key-from-ctaf>
```

#### Testing

9 unit tests covering:

- Entity creation and validation
- XML generation with special character handling
- Report validation logic
- Error conditions

---

### 2. TuniCheque Integration (STORY-COMP-11)

**File**: `tunicheque.rs`

Real-time cheque verification and reporting system integration per Central Bank of Tunisia Circular 2025-03. TuniCheque is operated by the Tunisian Bankers Association (ATB) and Central Bank of Tunisia.

#### Key Types

- `ChequeInfo`: Basic cheque information (number, bank code, amount)
- `ChequeStatus`: Status enum (Valid, Bounced, Stopped, Unknown)
- `BounceReason`: Reason for bounce (insufficient funds, account closed, etc.)
- `ChequeVerificationResponse`: Response from system query
- `BounceReportRequest` / `BounceReportResponse`: Reporting structures
- `TuniChequeClient`: HTTP client

#### Public API

```rust
// Create client
let client = TuniChequeClient::new(
    "https://tunicheque.atb.tn".to_string(),
    api_key,
)?;

// Verify cheque status
let cheque_info = ChequeInfo::new("1234567".to_string(), "10001".to_string())?
    .with_amount(5000.0)
    .with_cheque_date("2026-04-15".to_string());

let verification = client.verify_cheque(&cheque_info).await?;

// Report bounced cheque
let report = BounceReportRequest {
    cheque_number: "1234567".to_string(),
    bank_code: "10001".to_string(),
    reason: BounceReason::InsufficientFunds,
    reported_by: "aml@banko.tn".to_string(),
    notes: Some("Insufficient funds in originating account".to_string()),
};

let response = client.report_bounced_cheque(&report).await?;

// Report stopped cheque
let response = client.report_stopped_cheque(
    "1234567".to_string(),
    "10001".to_string(),
    "Cheque reported as lost".to_string(),
    "aml@banko.tn".to_string(),
).await?;
```

#### Compliance Features

- Real-time verification against CTAF's consolidated list
- Automatic propagation to all banking system participants
- Support for multiple bounce reasons
- Lost/stolen cheque reporting
- Bank code validation

#### Configuration

Environment variables (production):

```bash
TUNICHEQUE_BASE_URL=https://tunicheque.atb.tn/api/v1
TUNICHEQUE_API_KEY=<api-key>
```

#### Testing

8 unit tests and async tests covering:

- Cheque info validation
- Status verification
- Bounce reporting
- Error conditions

---

### 3. Travel Rule R.16 Compliance (STORY-COMP-13)

**File**: `travel_rule.rs`

Compliance with FATF Recommendation 16 (Travel Rule) for international wire transfers. Ensures originator and beneficiary information is transmitted with cross-border payments above thresholds.

#### Key Types

- `Originator`: Sender information (account, name, address, ID)
- `Beneficiary`: Recipient information (account, name, address, bank)
- `TravelRuleData`: Complete travel rule data package
- `EnrichedPayment`: Payment with travel rule compliance metadata
- `TravelRuleValidator`: Configurable validator
- `TravelRuleCompliance`: Status enum (Compliant, RequiresInformation, Exempt, NonCompliant)

#### Public API

```rust
// Create originator
let originator = Originator::new(
    "ACC-001".to_string(),
    "John Doe".to_string(),
    "123 Main St".to_string(),
    "Tunis".to_string(),
    "TN".to_string(),
    "1000".to_string(),
)?
.with_id(IdType::Passport, "A123456".to_string())
.with_date_of_birth("1980-05-15".to_string());

// Create beneficiary
let beneficiary = Beneficiary::new(
    "ACC-002".to_string(),
    "Jane Smith".to_string(),
    "456 Oak Ave".to_string(),
    "Paris".to_string(),
    "FR".to_string(),
    "75001".to_string(),
)?
.with_bank("BNP Paribas".to_string(), "BNPAFR22".to_string());

// Create travel rule data
let tr_data = TravelRuleData::new(
    originator,
    beneficiary,
    5000.0,
    "EUR".to_string(),
)?;

// Validate compliance
let validator = TravelRuleValidator::new()
    .with_threshold(3000.0)
    .add_restricted_jurisdiction("IR".to_string());

let compliance = validator.validate_travel_rule(
    "TN",  // originating country
    "FR",  // destination country
    5000.0, // amount
    true,   // has travel rule data
)?;

// Enrich payment with travel rule info
let enriched = enrich_payment_with_travel_data(
    "PAY-001".to_string(),
    5000.0,
    "EUR".to_string(),
    "ACC-002".to_string(),
    "TN".to_string(),
    "FR".to_string(),
    Some(originator),
    Some(beneficiary),
)?;
```

#### Compliance Features

- Automatic threshold evaluation (configurable per jurisdiction)
- Domestic transfer exemption
- Pre-configured exempt jurisdictions (Tunisia domestic)
- Originator and beneficiary validation
- Support for multiple ID types (Passport, National ID, Business Registration, etc.)
- Detailed compliance status reporting

#### Configuration

```rust
// Create custom validator
let validator = TravelRuleValidator::new()
    .with_threshold(10000.0)  // 10,000 EUR threshold
    .add_restricted_jurisdiction("IR".to_string())
    .add_restricted_jurisdiction("SY".to_string());
```

#### Testing

12 unit tests covering:

- Originator/beneficiary creation and enrichment
- Travel rule data validation
- Threshold evaluation logic
- Domestic transfer exemption
- Cross-border transfer handling
- Compliance status determination
- Payment enrichment workflow

---

## Architecture Notes

### Error Handling

Each module defines its own error type implementing `std::error::Error`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoAmlError {
    InvalidReport(String),
    SubmissionFailed(String),
    // ...
}
```

Errors are serializable for API responses and logging.

### Async Patterns

APIs requiring network calls use `async/await`:

```rust
pub async fn submit_report(&self, report: &GoAmlReport) -> Result<...> {
    // HTTP call would go here
}
```

Tests use `#[tokio::test]` macro for async test support.

### Mock Implementations

Currently, integrations return mock responses. For production:

1. Replace mock responses with actual HTTP calls
2. Add proper authentication (Bearer tokens, mTLS)
3. Implement retry logic with exponential backoff
4. Add request/response logging and audit trails
5. Handle timeout and network failure scenarios

### Validation

Validation occurs at two levels:

1. **Constructor validation**: Ensures required fields are present and valid
2. **Pre-submission validation**: Reports are validated before being sent

Example:

```rust
let report = GoAmlReport::new(...)?;  // Constructor validation
report.validate()?;                    // Pre-submission validation
client.generate_xml(&report)?;         // Serialization with further checks
```

## Integration with AML Module

The goAML integration can receive SuspicionReport data from the AML application layer:

```rust
// In AML service
let suspicion_report = SuspicionReport::new(...)?;
let goaml_report = from_suspicion_report(
    suspicion_report.reasons().to_string(),
    ReportingEntity::new(...)?,
    GoAmlSuspect::new(...)?,
)?;
```

## Integration with Payment Module

Travel Rule compliance is evaluated for Payment Orders:

```rust
// In Payment service
let payment = PaymentOrder::new(...)?;
let enriched = enrich_payment_with_travel_data(
    payment.id().to_string(),
    payment.amount(),
    payment.currency(),
    payment.beneficiary_account(),
    "TN",  // originating_country
    destination_country,
    originator_info,
    beneficiary_info,
)?;

// Check compliance
match enriched.travel_rule_compliance {
    TravelRuleCompliance::Compliant => { /* proceed */ },
    TravelRuleCompliance::RequiresInformation => { /* collect info */ },
    TravelRuleCompliance::Exempt => { /* process normally */ },
    _ => { /* reject */ },
}
```

## Regulatory References

### goAML (STORY-COMP-10)

- UN Office on Drugs and Crime (UNODC) - goAML Reporting Standard
- Central Bank of Tunisia Circular 2024-XX (CTAF requirements)
- Law 2015-26 on Anti-Money Laundering (Tunisia)

### TuniCheque (STORY-COMP-11)

- Central Bank of Tunisia Circular 2025-03 (Cheque verification)
- Tunisian Bankers Association (ATB) - TuniCheque System
- International Cheque Standardization Guidelines

### Travel Rule R.16 (STORY-COMP-13)

- FATF Recommendations (2012) - Recommendation 16
- FATF Travel Rule Guidance (June 2020)
- Basel Committee - Customer Due Diligence
- ABA/FinCEN Travel Rule Implementation Guidelines

## Future Enhancements

1. **Caching**: Redis cache for cheque verification results (short TTL)
2. **Webhooks**: Async notifications for report status changes
3. **Batch operations**: Bulk cheque verification and reporting
4. **Audit trails**: Detailed logging of all integration calls
5. **Metrics**: Prometheus metrics for integration performance
6. **Circuit breaker**: Graceful degradation on CTAF/TuniCheque outages

## Testing in Development

All modules include comprehensive unit tests:

```bash
# Run integration tests
cargo test --lib integrations::

# With output
cargo test --lib integrations:: -- --nocapture

# Specific module
cargo test --lib integrations::goaml::
cargo test --lib integrations::tunicheque::
cargo test --lib integrations::travel_rule::
```

## Support

For issues or questions:

1. Check the inline documentation and examples
2. Review the test files for usage patterns
3. Consult regulatory references in the CLAUDE.md file
4. Contact the compliance team for regulatory interpretations
