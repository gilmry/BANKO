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
}
