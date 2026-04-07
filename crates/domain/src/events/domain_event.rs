use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================
// DomainEvent Trait
// ============================================================

/// Core trait for all domain events in BANKO.
/// Events represent facts that have occurred in the system.
pub trait DomainEvent {
    /// Returns the event type identifier (e.g., "AccountOpened", "PaymentInitiated")
    fn event_type(&self) -> &str;

    /// Returns the ID of the aggregate this event applies to
    fn aggregate_id(&self) -> Uuid;

    /// Returns the type of aggregate this event applies to
    fn aggregate_type(&self) -> &str;

    /// Returns when this event occurred
    fn timestamp(&self) -> DateTime<Utc>;

    /// Returns the event payload as JSON
    fn payload(&self) -> serde_json::Value;

    /// Returns unique ID for this event
    fn event_id(&self) -> Uuid;

    /// Returns the version of this event type (for migration purposes)
    fn version(&self) -> u32 {
        1
    }
}

// ============================================================
// StoredEvent - Persisted Event Form
// ============================================================

/// Represents an event as stored in the event store.
/// This is the "persisted" representation of domain events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredEvent {
    /// Unique identifier for this event
    pub id: Uuid,
    /// ID of the aggregate this event applies to
    pub aggregate_id: Uuid,
    /// Type of aggregate (e.g., "Account", "Payment", "Customer")
    pub aggregate_type: String,
    /// Event type identifier (e.g., "AccountOpened")
    pub event_type: String,
    /// The event payload as JSON
    pub payload: serde_json::Value,
    /// Version of this event type for schema evolution
    pub version: u32,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// Global sequence number (assigned by event store on insert)
    pub sequence_number: i64,
}

impl StoredEvent {
    /// Creates a new StoredEvent from domain event data
    pub fn new(
        aggregate_id: Uuid,
        aggregate_type: String,
        event_type: String,
        payload: serde_json::Value,
    ) -> Self {
        StoredEvent {
            id: Uuid::new_v4(),
            aggregate_id,
            aggregate_type,
            event_type,
            payload,
            version: 1,
            timestamp: Utc::now(),
            sequence_number: 0, // Will be assigned by store
        }
    }

    /// Sets the version for this event
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Sets the timestamp for this event
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the sequence number (typically assigned by event store)
    pub fn with_sequence_number(mut self, sequence_number: i64) -> Self {
        self.sequence_number = sequence_number;
        self
    }
}

// ============================================================
// Concrete Domain Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountOpenedEvent {
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub account_type: String,
    pub timestamp: DateTime<Utc>,
}

impl AccountOpenedEvent {
    pub fn new(account_id: Uuid, customer_id: Uuid, account_type: String) -> Self {
        AccountOpenedEvent {
            account_id,
            customer_id,
            account_type,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for AccountOpenedEvent {
    fn event_type(&self) -> &str {
        "AccountOpened"
    }
    fn aggregate_id(&self) -> Uuid {
        self.account_id
    }
    fn aggregate_type(&self) -> &str {
        "Account"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize AccountOpenedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentInitiatedEvent {
    pub payment_id: Uuid,
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: String,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
}

impl PaymentInitiatedEvent {
    pub fn new(
        payment_id: Uuid,
        from_account: Uuid,
        to_account: Uuid,
        amount: String,
        currency: String,
    ) -> Self {
        PaymentInitiatedEvent {
            payment_id,
            from_account,
            to_account,
            amount,
            currency,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for PaymentInitiatedEvent {
    fn event_type(&self) -> &str {
        "PaymentInitiated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.payment_id
    }
    fn aggregate_type(&self) -> &str {
        "Payment"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize PaymentInitiatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentCompletedEvent {
    pub payment_id: Uuid,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

impl PaymentCompletedEvent {
    pub fn new(payment_id: Uuid, status: String) -> Self {
        PaymentCompletedEvent {
            payment_id,
            status,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for PaymentCompletedEvent {
    fn event_type(&self) -> &str {
        "PaymentCompleted"
    }
    fn aggregate_id(&self) -> Uuid {
        self.payment_id
    }
    fn aggregate_type(&self) -> &str {
        "Payment"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize PaymentCompletedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanApprovedEvent {
    pub loan_id: Uuid,
    pub customer_id: Uuid,
    pub amount: String,
    pub timestamp: DateTime<Utc>,
}

impl LoanApprovedEvent {
    pub fn new(loan_id: Uuid, customer_id: Uuid, amount: String) -> Self {
        LoanApprovedEvent {
            loan_id,
            customer_id,
            amount,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for LoanApprovedEvent {
    fn event_type(&self) -> &str {
        "LoanApproved"
    }
    fn aggregate_id(&self) -> Uuid {
        self.loan_id
    }
    fn aggregate_type(&self) -> &str {
        "Loan"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize LoanApprovedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmlAlertRaisedEvent {
    pub alert_id: Uuid,
    pub customer_id: Uuid,
    pub severity: String,
    pub timestamp: DateTime<Utc>,
}

impl AmlAlertRaisedEvent {
    pub fn new(alert_id: Uuid, customer_id: Uuid, severity: String) -> Self {
        AmlAlertRaisedEvent {
            alert_id,
            customer_id,
            severity,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for AmlAlertRaisedEvent {
    fn event_type(&self) -> &str {
        "AmlAlertRaised"
    }
    fn aggregate_id(&self) -> Uuid {
        self.alert_id
    }
    fn aggregate_type(&self) -> &str {
        "AmlAlert"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize AmlAlertRaisedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KycApprovedEvent {
    pub customer_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl KycApprovedEvent {
    pub fn new(customer_id: Uuid) -> Self {
        KycApprovedEvent {
            customer_id,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for KycApprovedEvent {
    fn event_type(&self) -> &str {
        "KycApproved"
    }
    fn aggregate_id(&self) -> Uuid {
        self.customer_id
    }
    fn aggregate_type(&self) -> &str {
        "Customer"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize KycApprovedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FxOperationSettledEvent {
    pub operation_id: Uuid,
    pub source_currency: String,
    pub target_currency: String,
    pub amount: String,
    pub timestamp: DateTime<Utc>,
}

impl FxOperationSettledEvent {
    pub fn new(
        operation_id: Uuid,
        source_currency: String,
        target_currency: String,
        amount: String,
    ) -> Self {
        FxOperationSettledEvent {
            operation_id,
            source_currency,
            target_currency,
            amount,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for FxOperationSettledEvent {
    fn event_type(&self) -> &str {
        "FxOperationSettled"
    }
    fn aggregate_id(&self) -> Uuid {
        self.operation_id
    }
    fn aggregate_type(&self) -> &str {
        "FxOperation"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize FxOperationSettledEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountingEntryCreatedEvent {
    pub entry_id: Uuid,
    pub debit_account: String,
    pub credit_account: String,
    pub amount: String,
    pub timestamp: DateTime<Utc>,
}

impl AccountingEntryCreatedEvent {
    pub fn new(
        entry_id: Uuid,
        debit_account: String,
        credit_account: String,
        amount: String,
    ) -> Self {
        AccountingEntryCreatedEvent {
            entry_id,
            debit_account,
            credit_account,
            amount,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for AccountingEntryCreatedEvent {
    fn event_type(&self) -> &str {
        "AccountingEntryCreated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.entry_id
    }
    fn aggregate_type(&self) -> &str {
        "AccountingEntry"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize AccountingEntryCreatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Customer Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerCreatedEvent {
    pub customer_id: Uuid,
    pub customer_type: String,
    pub timestamp: DateTime<Utc>,
}

impl CustomerCreatedEvent {
    pub fn new(customer_id: Uuid, customer_type: String) -> Self {
        CustomerCreatedEvent {
            customer_id,
            customer_type,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for CustomerCreatedEvent {
    fn event_type(&self) -> &str {
        "CustomerCreated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.customer_id
    }
    fn aggregate_type(&self) -> &str {
        "Customer"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize CustomerCreatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerKycUpdatedEvent {
    pub customer_id: Uuid,
    pub kyc_level: String,
    pub timestamp: DateTime<Utc>,
}

impl CustomerKycUpdatedEvent {
    pub fn new(customer_id: Uuid, kyc_level: String) -> Self {
        CustomerKycUpdatedEvent {
            customer_id,
            kyc_level,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for CustomerKycUpdatedEvent {
    fn event_type(&self) -> &str {
        "CustomerKycUpdated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.customer_id
    }
    fn aggregate_type(&self) -> &str {
        "Customer"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize CustomerKycUpdatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerRiskScoredEvent {
    pub customer_id: Uuid,
    pub risk_score: u32,
    pub timestamp: DateTime<Utc>,
}

impl CustomerRiskScoredEvent {
    pub fn new(customer_id: Uuid, risk_score: u32) -> Self {
        CustomerRiskScoredEvent {
            customer_id,
            risk_score,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for CustomerRiskScoredEvent {
    fn event_type(&self) -> &str {
        "CustomerRiskScored"
    }
    fn aggregate_id(&self) -> Uuid {
        self.customer_id
    }
    fn aggregate_type(&self) -> &str {
        "Customer"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize CustomerRiskScoredEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Credit Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanDisbursedEvent {
    pub loan_id: Uuid,
    pub amount_cents: i64,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
}

impl LoanDisbursedEvent {
    pub fn new(loan_id: Uuid, amount_cents: i64, currency: String) -> Self {
        LoanDisbursedEvent {
            loan_id,
            amount_cents,
            currency,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for LoanDisbursedEvent {
    fn event_type(&self) -> &str {
        "LoanDisbursed"
    }
    fn aggregate_id(&self) -> Uuid {
        self.loan_id
    }
    fn aggregate_type(&self) -> &str {
        "Loan"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize LoanDisbursedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanRepaymentReceivedEvent {
    pub loan_id: Uuid,
    pub installment_id: Uuid,
    pub amount_cents: i64,
    pub timestamp: DateTime<Utc>,
}

impl LoanRepaymentReceivedEvent {
    pub fn new(loan_id: Uuid, installment_id: Uuid, amount_cents: i64) -> Self {
        LoanRepaymentReceivedEvent {
            loan_id,
            installment_id,
            amount_cents,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for LoanRepaymentReceivedEvent {
    fn event_type(&self) -> &str {
        "LoanRepaymentReceived"
    }
    fn aggregate_id(&self) -> Uuid {
        self.loan_id
    }
    fn aggregate_type(&self) -> &str {
        "Loan"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize LoanRepaymentReceivedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanClassificationChangedEvent {
    pub loan_id: Uuid,
    pub old_class: u8,
    pub new_class: u8,
    pub timestamp: DateTime<Utc>,
}

impl LoanClassificationChangedEvent {
    pub fn new(loan_id: Uuid, old_class: u8, new_class: u8) -> Self {
        LoanClassificationChangedEvent {
            loan_id,
            old_class,
            new_class,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for LoanClassificationChangedEvent {
    fn event_type(&self) -> &str {
        "LoanClassificationChanged"
    }
    fn aggregate_id(&self) -> Uuid {
        self.loan_id
    }
    fn aggregate_type(&self) -> &str {
        "Loan"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize LoanClassificationChangedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Sanctions Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SanctionsScreeningCompletedEvent {
    pub entity_id: Uuid,
    pub matches_found: u32,
    pub timestamp: DateTime<Utc>,
}

impl SanctionsScreeningCompletedEvent {
    pub fn new(entity_id: Uuid, matches_found: u32) -> Self {
        SanctionsScreeningCompletedEvent {
            entity_id,
            matches_found,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for SanctionsScreeningCompletedEvent {
    fn event_type(&self) -> &str {
        "SanctionsScreeningCompleted"
    }
    fn aggregate_id(&self) -> Uuid {
        self.entity_id
    }
    fn aggregate_type(&self) -> &str {
        "SanctionsScreening"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize SanctionsScreeningCompletedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SanctionsHitConfirmedEvent {
    pub screening_id: Uuid,
    pub entry_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl SanctionsHitConfirmedEvent {
    pub fn new(screening_id: Uuid, entry_id: Uuid) -> Self {
        SanctionsHitConfirmedEvent {
            screening_id,
            entry_id,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for SanctionsHitConfirmedEvent {
    fn event_type(&self) -> &str {
        "SanctionsHitConfirmed"
    }
    fn aggregate_id(&self) -> Uuid {
        self.screening_id
    }
    fn aggregate_type(&self) -> &str {
        "SanctionsScreening"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize SanctionsHitConfirmedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Prudential Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrudentialRatioCalculatedEvent {
    pub ratio_type: String,
    pub value: f64,
    pub threshold: f64,
    pub timestamp: DateTime<Utc>,
}

impl PrudentialRatioCalculatedEvent {
    pub fn new(ratio_type: String, value: f64, threshold: f64) -> Self {
        PrudentialRatioCalculatedEvent {
            ratio_type,
            value,
            threshold,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for PrudentialRatioCalculatedEvent {
    fn event_type(&self) -> &str {
        "PrudentialRatioCalculated"
    }
    fn aggregate_id(&self) -> Uuid {
        Uuid::new_v4() // Prudential ratios are system-level, not tied to a specific entity
    }
    fn aggregate_type(&self) -> &str {
        "PrudentialRatio"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize PrudentialRatioCalculatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrudentialBreachDetectedEvent {
    pub ratio_type: String,
    pub value: f64,
    pub threshold: f64,
    pub timestamp: DateTime<Utc>,
}

impl PrudentialBreachDetectedEvent {
    pub fn new(ratio_type: String, value: f64, threshold: f64) -> Self {
        PrudentialBreachDetectedEvent {
            ratio_type,
            value,
            threshold,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for PrudentialBreachDetectedEvent {
    fn event_type(&self) -> &str {
        "PrudentialBreachDetected"
    }
    fn aggregate_id(&self) -> Uuid {
        Uuid::new_v4() // System-level event
    }
    fn aggregate_type(&self) -> &str {
        "PrudentialRatio"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize PrudentialBreachDetectedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Governance Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditEntryCreatedEvent {
    pub entry_id: Uuid,
    pub action: String,
    pub actor_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl AuditEntryCreatedEvent {
    pub fn new(entry_id: Uuid, action: String, actor_id: Uuid) -> Self {
        AuditEntryCreatedEvent {
            entry_id,
            action,
            actor_id,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for AuditEntryCreatedEvent {
    fn event_type(&self) -> &str {
        "AuditEntryCreated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.entry_id
    }
    fn aggregate_type(&self) -> &str {
        "AuditEntry"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize AuditEntryCreatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommitteeDecisionRecordedEvent {
    pub committee_id: Uuid,
    pub decision: String,
    pub timestamp: DateTime<Utc>,
}

impl CommitteeDecisionRecordedEvent {
    pub fn new(committee_id: Uuid, decision: String) -> Self {
        CommitteeDecisionRecordedEvent {
            committee_id,
            decision,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for CommitteeDecisionRecordedEvent {
    fn event_type(&self) -> &str {
        "CommitteeDecisionRecorded"
    }
    fn aggregate_id(&self) -> Uuid {
        self.committee_id
    }
    fn aggregate_type(&self) -> &str {
        "CommitteeDecision"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize CommitteeDecisionRecordedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Identity Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub role: String,
    pub timestamp: DateTime<Utc>,
}

impl UserCreatedEvent {
    pub fn new(user_id: Uuid, role: String) -> Self {
        UserCreatedEvent {
            user_id,
            role,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for UserCreatedEvent {
    fn event_type(&self) -> &str {
        "UserCreated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
    fn aggregate_type(&self) -> &str {
        "User"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize UserCreatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAuthenticatedEvent {
    pub user_id: Uuid,
    pub method: String,
    pub timestamp: DateTime<Utc>,
}

impl UserAuthenticatedEvent {
    pub fn new(user_id: Uuid, method: String) -> Self {
        UserAuthenticatedEvent {
            user_id,
            method,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for UserAuthenticatedEvent {
    fn event_type(&self) -> &str {
        "UserAuthenticated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
    fn aggregate_type(&self) -> &str {
        "User"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize UserAuthenticatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwoFactorEnabledEvent {
    pub user_id: Uuid,
    pub method: String,
    pub timestamp: DateTime<Utc>,
}

impl TwoFactorEnabledEvent {
    pub fn new(user_id: Uuid, method: String) -> Self {
        TwoFactorEnabledEvent {
            user_id,
            method,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for TwoFactorEnabledEvent {
    fn event_type(&self) -> &str {
        "TwoFactorEnabled"
    }
    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
    fn aggregate_type(&self) -> &str {
        "User"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize TwoFactorEnabledEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Compliance Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsentGrantedEvent {
    pub customer_id: Uuid,
    pub purpose: String,
    pub timestamp: DateTime<Utc>,
}

impl ConsentGrantedEvent {
    pub fn new(customer_id: Uuid, purpose: String) -> Self {
        ConsentGrantedEvent {
            customer_id,
            purpose,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for ConsentGrantedEvent {
    fn event_type(&self) -> &str {
        "ConsentGranted"
    }
    fn aggregate_id(&self) -> Uuid {
        self.customer_id
    }
    fn aggregate_type(&self) -> &str {
        "Consent"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize ConsentGrantedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsentRevokedEvent {
    pub customer_id: Uuid,
    pub purpose: String,
    pub timestamp: DateTime<Utc>,
}

impl ConsentRevokedEvent {
    pub fn new(customer_id: Uuid, purpose: String) -> Self {
        ConsentRevokedEvent {
            customer_id,
            purpose,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for ConsentRevokedEvent {
    fn event_type(&self) -> &str {
        "ConsentRevoked"
    }
    fn aggregate_id(&self) -> Uuid {
        self.customer_id
    }
    fn aggregate_type(&self) -> &str {
        "Consent"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize ConsentRevokedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BreachReportedEvent {
    pub breach_id: Uuid,
    pub severity: String,
    pub timestamp: DateTime<Utc>,
}

impl BreachReportedEvent {
    pub fn new(breach_id: Uuid, severity: String) -> Self {
        BreachReportedEvent {
            breach_id,
            severity,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for BreachReportedEvent {
    fn event_type(&self) -> &str {
        "BreachReported"
    }
    fn aggregate_id(&self) -> Uuid {
        self.breach_id
    }
    fn aggregate_type(&self) -> &str {
        "Breach"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize BreachReportedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DpiaCompletedEvent {
    pub dpia_id: Uuid,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

impl DpiaCompletedEvent {
    pub fn new(dpia_id: Uuid, status: String) -> Self {
        DpiaCompletedEvent {
            dpia_id,
            status,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for DpiaCompletedEvent {
    fn event_type(&self) -> &str {
        "DpiaCompleted"
    }
    fn aggregate_id(&self) -> Uuid {
        self.dpia_id
    }
    fn aggregate_type(&self) -> &str {
        "DPIA"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize DpiaCompletedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Notification Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotificationSentEvent {
    pub notification_id: Uuid,
    pub channel: String,
    pub recipient_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl NotificationSentEvent {
    pub fn new(notification_id: Uuid, channel: String, recipient_id: Uuid) -> Self {
        NotificationSentEvent {
            notification_id,
            channel,
            recipient_id,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for NotificationSentEvent {
    fn event_type(&self) -> &str {
        "NotificationSent"
    }
    fn aggregate_id(&self) -> Uuid {
        self.notification_id
    }
    fn aggregate_type(&self) -> &str {
        "Notification"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize NotificationSentEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Product Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductCreatedEvent {
    pub product_id: Uuid,
    pub product_type: String,
    pub timestamp: DateTime<Utc>,
}

impl ProductCreatedEvent {
    pub fn new(product_id: Uuid, product_type: String) -> Self {
        ProductCreatedEvent {
            product_id,
            product_type,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for ProductCreatedEvent {
    fn event_type(&self) -> &str {
        "ProductCreated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.product_id
    }
    fn aggregate_type(&self) -> &str {
        "Product"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize ProductCreatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductActivatedEvent {
    pub product_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl ProductActivatedEvent {
    pub fn new(product_id: Uuid) -> Self {
        ProductActivatedEvent {
            product_id,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for ProductActivatedEvent {
    fn event_type(&self) -> &str {
        "ProductActivated"
    }
    fn aggregate_id(&self) -> Uuid {
        self.product_id
    }
    fn aggregate_type(&self) -> &str {
        "Product"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize ProductActivatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// ReferenceData Events
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferenceDataUpdatedEvent {
    pub data_type: String,
    pub code: String,
    pub timestamp: DateTime<Utc>,
}

impl ReferenceDataUpdatedEvent {
    pub fn new(data_type: String, code: String) -> Self {
        ReferenceDataUpdatedEvent {
            data_type,
            code,
            timestamp: Utc::now(),
        }
    }
}

impl DomainEvent for ReferenceDataUpdatedEvent {
    fn event_type(&self) -> &str {
        "ReferenceDataUpdated"
    }
    fn aggregate_id(&self) -> Uuid {
        Uuid::new_v4() // Reference data is system-level
    }
    fn aggregate_type(&self) -> &str {
        "ReferenceData"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    fn payload(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize ReferenceDataUpdatedEvent")
    }
    fn event_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_opened_event_trait() {
        let account_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let event = AccountOpenedEvent::new(account_id, customer_id, "Checking".to_string());

        assert_eq!(event.event_type(), "AccountOpened");
        assert_eq!(event.aggregate_id(), account_id);
        assert_eq!(event.aggregate_type(), "Account");
        assert_eq!(event.version(), 1);
    }

    #[test]
    fn test_payment_initiated_event_creation() {
        let payment_id = Uuid::new_v4();
        let from_account = Uuid::new_v4();
        let to_account = Uuid::new_v4();

        let event = PaymentInitiatedEvent::new(
            payment_id,
            from_account,
            to_account,
            "1000.00".to_string(),
            "EUR".to_string(),
        );

        assert_eq!(event.event_type(), "PaymentInitiated");
        assert_eq!(event.aggregate_id(), payment_id);
        assert_eq!(event.aggregate_type(), "Payment");
    }

    #[test]
    fn test_stored_event_new() {
        let aggregate_id = Uuid::new_v4();
        let payload = serde_json::json!({"test": "data"});

        let event = StoredEvent::new(
            aggregate_id,
            "TestAggregate".to_string(),
            "TestEvent".to_string(),
            payload.clone(),
        );

        assert_eq!(event.aggregate_id, aggregate_id);
        assert_eq!(event.aggregate_type, "TestAggregate");
        assert_eq!(event.event_type, "TestEvent");
        assert_eq!(event.payload, payload);
        assert_eq!(event.version, 1);
    }

    #[test]
    fn test_stored_event_with_version() {
        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "Test".to_string(),
            serde_json::json!({}),
        )
        .with_version(2);

        assert_eq!(event.version, 2);
    }

    #[test]
    fn test_stored_event_with_sequence_number() {
        let event = StoredEvent::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "Test".to_string(),
            serde_json::json!({}),
        )
        .with_sequence_number(42);

        assert_eq!(event.sequence_number, 42);
    }

    #[test]
    fn test_stored_event_serialization() {
        let event = StoredEvent::new(
            Uuid::new_v4(),
            "TestAggregate".to_string(),
            "TestEvent".to_string(),
            serde_json::json!({"key": "value"}),
        );

        let json = serde_json::to_string(&event).expect("Should serialize");
        let deserialized: StoredEvent = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_loan_approved_event_payload() {
        let loan_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let event = LoanApprovedEvent::new(loan_id, customer_id, "5000.00".to_string());

        let payload = event.payload();
        assert!(payload.is_object());
    }

    #[test]
    fn test_kyc_approved_event_properties() {
        let customer_id = Uuid::new_v4();
        let event = KycApprovedEvent::new(customer_id);

        assert_eq!(event.event_type(), "KycApproved");
        assert_eq!(event.aggregate_id(), customer_id);
        assert_eq!(event.aggregate_type(), "Customer");
    }

    #[test]
    fn test_accounting_entry_created_event() {
        let entry_id = Uuid::new_v4();
        let event = AccountingEntryCreatedEvent::new(
            entry_id,
            "101".to_string(),
            "201".to_string(),
            "1000.00".to_string(),
        );

        assert_eq!(event.event_type(), "AccountingEntryCreated");
        assert_eq!(event.aggregate_type(), "AccountingEntry");
        assert_eq!(event.debit_account, "101");
        assert_eq!(event.credit_account, "201");
    }

    #[test]
    fn test_customer_created_event() {
        let customer_id = Uuid::new_v4();
        let event = CustomerCreatedEvent::new(customer_id, "INDIVIDUAL".to_string());

        assert_eq!(event.event_type(), "CustomerCreated");
        assert_eq!(event.aggregate_id(), customer_id);
        assert_eq!(event.aggregate_type(), "Customer");
        assert_eq!(event.customer_type, "INDIVIDUAL");
    }

    #[test]
    fn test_loan_disbursed_event() {
        let loan_id = Uuid::new_v4();
        let event = LoanDisbursedEvent::new(loan_id, 100000, "EUR".to_string());

        assert_eq!(event.event_type(), "LoanDisbursed");
        assert_eq!(event.aggregate_id(), loan_id);
        assert_eq!(event.amount_cents, 100000);
        assert_eq!(event.currency, "EUR");
    }

    #[test]
    fn test_sanctions_screening_completed_event() {
        let entity_id = Uuid::new_v4();
        let event = SanctionsScreeningCompletedEvent::new(entity_id, 2);

        assert_eq!(event.event_type(), "SanctionsScreeningCompleted");
        assert_eq!(event.aggregate_id(), entity_id);
        assert_eq!(event.matches_found, 2);
    }

    #[test]
    fn test_user_created_event() {
        let user_id = Uuid::new_v4();
        let event = UserCreatedEvent::new(user_id, "ADMIN".to_string());

        assert_eq!(event.event_type(), "UserCreated");
        assert_eq!(event.aggregate_id(), user_id);
        assert_eq!(event.aggregate_type(), "User");
        assert_eq!(event.role, "ADMIN");
    }

    #[test]
    fn test_consent_granted_event() {
        let customer_id = Uuid::new_v4();
        let event = ConsentGrantedEvent::new(customer_id, "MARKETING".to_string());

        assert_eq!(event.event_type(), "ConsentGranted");
        assert_eq!(event.aggregate_id(), customer_id);
        assert_eq!(event.aggregate_type(), "Consent");
        assert_eq!(event.purpose, "MARKETING");
    }

    #[test]
    fn test_product_created_event() {
        let product_id = Uuid::new_v4();
        let event = ProductCreatedEvent::new(product_id, "SAVINGS_ACCOUNT".to_string());

        assert_eq!(event.event_type(), "ProductCreated");
        assert_eq!(event.aggregate_id(), product_id);
        assert_eq!(event.product_type, "SAVINGS_ACCOUNT");
    }
}
