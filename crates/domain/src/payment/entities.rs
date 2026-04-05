use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(Uuid);

impl OrderId {
    pub fn new() -> Self {
        OrderId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        OrderId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for OrderId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransferId(Uuid);

impl TransferId {
    pub fn new() -> Self {
        TransferId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        TransferId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TransferId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TransferId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentType {
    Domestic,
    International,
    Swift,
    Sepa,
}

impl PaymentType {
    pub fn as_str(&self) -> &str {
        match self {
            PaymentType::Domestic => "Domestic",
            PaymentType::International => "International",
            PaymentType::Swift => "Swift",
            PaymentType::Sepa => "Sepa",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Domestic" => Ok(PaymentType::Domestic),
            "International" => Ok(PaymentType::International),
            "Swift" => Ok(PaymentType::Swift),
            "Sepa" => Ok(PaymentType::Sepa),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown payment type: {s}"
            ))),
        }
    }
}

impl fmt::Display for PaymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    Draft,
    PendingScreening,
    ScreeningCleared,
    Submitted,
    Processing,
    Cleared,
    Executed,
    Rejected,
    Failed,
}

impl PaymentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PaymentStatus::Draft => "Draft",
            PaymentStatus::PendingScreening => "PendingScreening",
            PaymentStatus::ScreeningCleared => "ScreeningCleared",
            PaymentStatus::Submitted => "Submitted",
            PaymentStatus::Processing => "Processing",
            PaymentStatus::Cleared => "Cleared",
            PaymentStatus::Executed => "Executed",
            PaymentStatus::Rejected => "Rejected",
            PaymentStatus::Failed => "Failed",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Draft" => Ok(PaymentStatus::Draft),
            "PendingScreening" => Ok(PaymentStatus::PendingScreening),
            "ScreeningCleared" => Ok(PaymentStatus::ScreeningCleared),
            "Submitted" => Ok(PaymentStatus::Submitted),
            "Processing" => Ok(PaymentStatus::Processing),
            "Cleared" => Ok(PaymentStatus::Cleared),
            "Executed" => Ok(PaymentStatus::Executed),
            "Rejected" => Ok(PaymentStatus::Rejected),
            "Failed" => Ok(PaymentStatus::Failed),
            _ => Err(DomainError::InvalidPaymentTransition(format!(
                "Unknown payment status: {s}"
            ))),
        }
    }
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreeningStatus {
    NotScreened,
    Cleared,
    Hit,
    Pending,
}

impl ScreeningStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ScreeningStatus::NotScreened => "NotScreened",
            ScreeningStatus::Cleared => "Cleared",
            ScreeningStatus::Hit => "Hit",
            ScreeningStatus::Pending => "Pending",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "NotScreened" => Ok(ScreeningStatus::NotScreened),
            "Cleared" => Ok(ScreeningStatus::Cleared),
            "Hit" => Ok(ScreeningStatus::Hit),
            "Pending" => Ok(ScreeningStatus::Pending),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown screening status: {s}"
            ))),
        }
    }
}

impl fmt::Display for ScreeningStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SwiftMessageStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwiftMessageStatus {
    Draft,
    Sent,
    Acknowledged,
}

impl SwiftMessageStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SwiftMessageStatus::Draft => "Draft",
            SwiftMessageStatus::Sent => "Sent",
            SwiftMessageStatus::Acknowledged => "Acknowledged",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Draft" => Ok(SwiftMessageStatus::Draft),
            "Sent" => Ok(SwiftMessageStatus::Sent),
            "Acknowledged" => Ok(SwiftMessageStatus::Acknowledged),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown SWIFT message status: {s}"
            ))),
        }
    }
}

impl fmt::Display for SwiftMessageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// PaymentOrder Aggregate
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentOrder {
    order_id: OrderId,
    sender_account_id: Uuid,
    beneficiary_name: String,
    beneficiary_rib: Option<String>,
    beneficiary_bic: Option<String>,
    amount: i64,
    currency: String,
    payment_type: PaymentType,
    status: PaymentStatus,
    screening_status: ScreeningStatus,
    reference: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
    executed_at: Option<DateTime<Utc>>,
    rejection_reason: Option<String>,
}

impl PaymentOrder {
    pub fn new(
        sender_account_id: Uuid,
        beneficiary_name: String,
        beneficiary_rib: Option<String>,
        beneficiary_bic: Option<String>,
        amount: i64,
        currency: String,
        payment_type: PaymentType,
        reference: String,
        description: Option<String>,
    ) -> Result<Self, DomainError> {
        if amount <= 0 {
            return Err(DomainError::InvalidPaymentOrder(
                "Amount must be greater than 0".to_string(),
            ));
        }
        if reference.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Reference cannot be empty".to_string(),
            ));
        }
        if beneficiary_name.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Beneficiary name cannot be empty".to_string(),
            ));
        }
        // For International/Swift payments, BIC is required
        if matches!(
            payment_type,
            PaymentType::International | PaymentType::Swift
        ) && beneficiary_bic.as_ref().is_none_or(|b| b.trim().is_empty())
        {
            return Err(DomainError::InvalidPaymentOrder(
                "BIC is required for international/SWIFT payments".to_string(),
            ));
        }

        Ok(PaymentOrder {
            order_id: OrderId::new(),
            sender_account_id,
            beneficiary_name,
            beneficiary_rib,
            beneficiary_bic,
            amount,
            currency,
            payment_type,
            status: PaymentStatus::Draft,
            screening_status: ScreeningStatus::NotScreened,
            reference,
            description,
            created_at: Utc::now(),
            submitted_at: None,
            executed_at: None,
            rejection_reason: None,
        })
    }

    /// Reconstruct from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        order_id: OrderId,
        sender_account_id: Uuid,
        beneficiary_name: String,
        beneficiary_rib: Option<String>,
        beneficiary_bic: Option<String>,
        amount: i64,
        currency: String,
        payment_type: PaymentType,
        status: PaymentStatus,
        screening_status: ScreeningStatus,
        reference: String,
        description: Option<String>,
        created_at: DateTime<Utc>,
        submitted_at: Option<DateTime<Utc>>,
        executed_at: Option<DateTime<Utc>>,
        rejection_reason: Option<String>,
    ) -> Self {
        PaymentOrder {
            order_id,
            sender_account_id,
            beneficiary_name,
            beneficiary_rib,
            beneficiary_bic,
            amount,
            currency,
            payment_type,
            status,
            screening_status,
            reference,
            description,
            created_at,
            submitted_at,
            executed_at,
            rejection_reason,
        }
    }

    // --- Getters ---

    pub fn order_id(&self) -> &OrderId {
        &self.order_id
    }
    pub fn sender_account_id(&self) -> Uuid {
        self.sender_account_id
    }
    pub fn beneficiary_name(&self) -> &str {
        &self.beneficiary_name
    }
    pub fn beneficiary_rib(&self) -> Option<&str> {
        self.beneficiary_rib.as_deref()
    }
    pub fn beneficiary_bic(&self) -> Option<&str> {
        self.beneficiary_bic.as_deref()
    }
    pub fn amount(&self) -> i64 {
        self.amount
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn payment_type(&self) -> PaymentType {
        self.payment_type
    }
    pub fn status(&self) -> PaymentStatus {
        self.status
    }
    pub fn screening_status(&self) -> ScreeningStatus {
        self.screening_status
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn submitted_at(&self) -> Option<DateTime<Utc>> {
        self.submitted_at
    }
    pub fn executed_at(&self) -> Option<DateTime<Utc>> {
        self.executed_at
    }
    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }

    // --- State Transitions ---

    pub fn requires_screening(&self) -> bool {
        matches!(
            self.payment_type,
            PaymentType::International | PaymentType::Swift
        )
    }

    pub fn mark_pending_screening(&mut self) {
        self.status = PaymentStatus::PendingScreening;
        self.screening_status = ScreeningStatus::Pending;
    }

    pub fn mark_screening_cleared(&mut self) {
        self.screening_status = ScreeningStatus::Cleared;
        self.status = PaymentStatus::ScreeningCleared;
    }

    pub fn mark_screening_hit(&mut self, reason: String) {
        self.screening_status = ScreeningStatus::Hit;
        self.status = PaymentStatus::Rejected;
        self.rejection_reason = Some(reason);
    }

    /// Submit the payment. INV-14: International payments require screening cleared.
    pub fn submit(&mut self) -> Result<(), DomainError> {
        if self.requires_screening() && self.screening_status != ScreeningStatus::Cleared {
            return Err(DomainError::SanctionsScreeningRequired);
        }
        if !matches!(
            self.status,
            PaymentStatus::Draft | PaymentStatus::ScreeningCleared
        ) {
            return Err(DomainError::InvalidPaymentTransition(format!(
                "Cannot submit from status {}",
                self.status
            )));
        }
        self.status = PaymentStatus::Submitted;
        self.submitted_at = Some(Utc::now());
        Ok(())
    }

    pub fn process(&mut self) -> Result<(), DomainError> {
        if self.status != PaymentStatus::Submitted {
            return Err(DomainError::InvalidPaymentTransition(format!(
                "Cannot process from status {}",
                self.status
            )));
        }
        self.status = PaymentStatus::Processing;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), DomainError> {
        if self.status != PaymentStatus::Processing {
            return Err(DomainError::InvalidPaymentTransition(format!(
                "Cannot clear from status {}",
                self.status
            )));
        }
        self.status = PaymentStatus::Cleared;
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), DomainError> {
        if !matches!(
            self.status,
            PaymentStatus::Submitted | PaymentStatus::Cleared
        ) {
            return Err(DomainError::InvalidPaymentTransition(format!(
                "Cannot execute from status {}",
                self.status
            )));
        }
        self.status = PaymentStatus::Executed;
        self.executed_at = Some(Utc::now());
        Ok(())
    }

    pub fn reject(&mut self, reason: String) {
        self.status = PaymentStatus::Rejected;
        self.rejection_reason = Some(reason);
    }

    pub fn fail(&mut self, reason: String) {
        self.status = PaymentStatus::Failed;
        self.rejection_reason = Some(reason);
    }
}

// ============================================================
// Transfer Entity
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    transfer_id: TransferId,
    order_id: OrderId,
    counterparty_rib: String,
    clearing_ref: Option<String>,
    amount: i64,
    currency: String,
    transfer_date: NaiveDate,
    status: PaymentStatus,
    created_at: DateTime<Utc>,
}

impl Transfer {
    pub fn new(
        order_id: OrderId,
        counterparty_rib: String,
        amount: i64,
        currency: String,
        transfer_date: NaiveDate,
    ) -> Self {
        Transfer {
            transfer_id: TransferId::new(),
            order_id,
            counterparty_rib,
            clearing_ref: None,
            amount,
            currency,
            transfer_date,
            status: PaymentStatus::Draft,
            created_at: Utc::now(),
        }
    }

    pub fn from_raw(
        transfer_id: TransferId,
        order_id: OrderId,
        counterparty_rib: String,
        clearing_ref: Option<String>,
        amount: i64,
        currency: String,
        transfer_date: NaiveDate,
        status: PaymentStatus,
        created_at: DateTime<Utc>,
    ) -> Self {
        Transfer {
            transfer_id,
            order_id,
            counterparty_rib,
            clearing_ref,
            amount,
            currency,
            transfer_date,
            status,
            created_at,
        }
    }

    pub fn transfer_id(&self) -> &TransferId {
        &self.transfer_id
    }
    pub fn order_id(&self) -> &OrderId {
        &self.order_id
    }
    pub fn counterparty_rib(&self) -> &str {
        &self.counterparty_rib
    }
    pub fn clearing_ref(&self) -> Option<&str> {
        self.clearing_ref.as_deref()
    }
    pub fn amount(&self) -> i64 {
        self.amount
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn transfer_date(&self) -> NaiveDate {
        self.transfer_date
    }
    pub fn status(&self) -> PaymentStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn set_clearing_ref(&mut self, clearing_ref: String) {
        self.clearing_ref = Some(clearing_ref);
    }

    pub fn mark_cleared(&mut self) {
        self.status = PaymentStatus::Cleared;
    }
}

// ============================================================
// SwiftMessage Entity (Stub)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwiftMessage {
    message_id: Uuid,
    order_id: OrderId,
    message_type: String,
    sender_bic: String,
    receiver_bic: String,
    amount: i64,
    currency: String,
    reference: String,
    status: SwiftMessageStatus,
    created_at: DateTime<Utc>,
}

impl SwiftMessage {
    pub fn generate_mt103(order: &PaymentOrder, sender_bic: &str) -> Result<Self, DomainError> {
        let receiver_bic = order.beneficiary_bic().ok_or_else(|| {
            DomainError::InvalidPaymentOrder(
                "Beneficiary BIC is required for SWIFT message generation".to_string(),
            )
        })?;

        Ok(SwiftMessage {
            message_id: Uuid::new_v4(),
            order_id: order.order_id().clone(),
            message_type: "MT103".to_string(),
            sender_bic: sender_bic.to_string(),
            receiver_bic: receiver_bic.to_string(),
            amount: order.amount(),
            currency: order.currency().to_string(),
            reference: order.reference().to_string(),
            status: SwiftMessageStatus::Draft,
            created_at: Utc::now(),
        })
    }

    pub fn from_raw(
        message_id: Uuid,
        order_id: OrderId,
        message_type: String,
        sender_bic: String,
        receiver_bic: String,
        amount: i64,
        currency: String,
        reference: String,
        status: SwiftMessageStatus,
        created_at: DateTime<Utc>,
    ) -> Self {
        SwiftMessage {
            message_id,
            order_id,
            message_type,
            sender_bic,
            receiver_bic,
            amount,
            currency,
            reference,
            status,
            created_at,
        }
    }

    pub fn message_id(&self) -> Uuid {
        self.message_id
    }
    pub fn order_id(&self) -> &OrderId {
        &self.order_id
    }
    pub fn message_type(&self) -> &str {
        &self.message_type
    }
    pub fn sender_bic(&self) -> &str {
        &self.sender_bic
    }
    pub fn receiver_bic(&self) -> &str {
        &self.receiver_bic
    }
    pub fn amount(&self) -> i64 {
        self.amount
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }
    pub fn status(&self) -> SwiftMessageStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn mark_sent(&mut self) {
        self.status = SwiftMessageStatus::Sent;
    }

    pub fn mark_acknowledged(&mut self) {
        self.status = SwiftMessageStatus::Acknowledged;
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_domestic_order() -> PaymentOrder {
        PaymentOrder::new(
            Uuid::new_v4(),
            "Ahmed Ben Ali".to_string(),
            Some("01234567890123456789".to_string()),
            None,
            500_000, // 500.000 TND in millimes
            "TND".to_string(),
            PaymentType::Domestic,
            "REF-2026-001".to_string(),
            Some("Domestic transfer".to_string()),
        )
        .unwrap()
    }

    fn make_international_order() -> PaymentOrder {
        PaymentOrder::new(
            Uuid::new_v4(),
            "Pierre Dupont".to_string(),
            None,
            Some("BNPAFRPP".to_string()),
            1_000_00, // 1000.00 EUR in cents
            "EUR".to_string(),
            PaymentType::International,
            "REF-INT-001".to_string(),
            Some("International transfer".to_string()),
        )
        .unwrap()
    }

    fn make_swift_order() -> PaymentOrder {
        PaymentOrder::new(
            Uuid::new_v4(),
            "John Smith".to_string(),
            None,
            Some("CHASUS33".to_string()),
            5_000_00,
            "USD".to_string(),
            PaymentType::Swift,
            "REF-SWIFT-001".to_string(),
            None,
        )
        .unwrap()
    }

    // --- Creation Tests ---

    #[test]
    fn test_create_domestic_payment_order() {
        let order = make_domestic_order();
        assert_eq!(order.status(), PaymentStatus::Draft);
        assert_eq!(order.screening_status(), ScreeningStatus::NotScreened);
        assert_eq!(order.payment_type(), PaymentType::Domestic);
        assert!(!order.requires_screening());
    }

    #[test]
    fn test_create_international_payment_order() {
        let order = make_international_order();
        assert_eq!(order.payment_type(), PaymentType::International);
        assert!(order.requires_screening());
        assert_eq!(order.beneficiary_bic(), Some("BNPAFRPP"));
    }

    #[test]
    fn test_international_requires_bic() {
        let result = PaymentOrder::new(
            Uuid::new_v4(),
            "Pierre Dupont".to_string(),
            None,
            None, // No BIC
            1_000_00,
            "EUR".to_string(),
            PaymentType::International,
            "REF-INT-002".to_string(),
            None,
        );
        assert!(result.is_err());
        assert!(matches!(result, Err(DomainError::InvalidPaymentOrder(_))));
    }

    #[test]
    fn test_swift_requires_bic() {
        let result = PaymentOrder::new(
            Uuid::new_v4(),
            "John Smith".to_string(),
            None,
            Some("".to_string()), // Empty BIC
            5_000_00,
            "USD".to_string(),
            PaymentType::Swift,
            "REF-SWIFT-002".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_amount_zero() {
        let result = PaymentOrder::new(
            Uuid::new_v4(),
            "Ahmed".to_string(),
            None,
            None,
            0,
            "TND".to_string(),
            PaymentType::Domestic,
            "REF-001".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_amount_negative() {
        let result = PaymentOrder::new(
            Uuid::new_v4(),
            "Ahmed".to_string(),
            None,
            None,
            -100,
            "TND".to_string(),
            PaymentType::Domestic,
            "REF-001".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_reference_rejected() {
        let result = PaymentOrder::new(
            Uuid::new_v4(),
            "Ahmed".to_string(),
            None,
            None,
            1000,
            "TND".to_string(),
            PaymentType::Domestic,
            "".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_beneficiary_name_rejected() {
        let result = PaymentOrder::new(
            Uuid::new_v4(),
            "".to_string(),
            None,
            None,
            1000,
            "TND".to_string(),
            PaymentType::Domestic,
            "REF-001".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    // --- Screening Workflow Tests ---

    #[test]
    fn test_screening_cleared_then_submit_ok() {
        let mut order = make_international_order();
        order.mark_pending_screening();
        assert_eq!(order.status(), PaymentStatus::PendingScreening);
        assert_eq!(order.screening_status(), ScreeningStatus::Pending);

        order.mark_screening_cleared();
        assert_eq!(order.status(), PaymentStatus::ScreeningCleared);
        assert_eq!(order.screening_status(), ScreeningStatus::Cleared);

        assert!(order.submit().is_ok());
        assert_eq!(order.status(), PaymentStatus::Submitted);
        assert!(order.submitted_at().is_some());
    }

    #[test]
    fn test_screening_hit_rejects_order() {
        let mut order = make_international_order();
        order.mark_pending_screening();
        order.mark_screening_hit("Sanctions hit: UN list match".to_string());

        assert_eq!(order.status(), PaymentStatus::Rejected);
        assert_eq!(order.screening_status(), ScreeningStatus::Hit);
        assert_eq!(
            order.rejection_reason(),
            Some("Sanctions hit: UN list match")
        );
    }

    #[test]
    fn test_cannot_submit_international_without_screening_cleared_inv14() {
        let mut order = make_international_order();
        // screening_status is NotScreened
        let result = order.submit();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(DomainError::SanctionsScreeningRequired)
        ));
    }

    #[test]
    fn test_cannot_submit_swift_without_screening_cleared_inv14() {
        let mut order = make_swift_order();
        let result = order.submit();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(DomainError::SanctionsScreeningRequired)
        ));
    }

    #[test]
    fn test_domestic_can_submit_without_screening() {
        let mut order = make_domestic_order();
        assert!(!order.requires_screening());
        assert!(order.submit().is_ok());
        assert_eq!(order.status(), PaymentStatus::Submitted);
    }

    // --- Status Transition Tests ---

    #[test]
    fn test_submit_then_execute() {
        let mut order = make_domestic_order();
        order.submit().unwrap();
        assert!(order.execute().is_ok());
        assert_eq!(order.status(), PaymentStatus::Executed);
        assert!(order.executed_at().is_some());
    }

    #[test]
    fn test_submit_process_clear_execute() {
        let mut order = make_domestic_order();
        order.submit().unwrap();
        order.process().unwrap();
        assert_eq!(order.status(), PaymentStatus::Processing);
        order.clear().unwrap();
        assert_eq!(order.status(), PaymentStatus::Cleared);
        order.execute().unwrap();
        assert_eq!(order.status(), PaymentStatus::Executed);
    }

    #[test]
    fn test_cannot_execute_from_draft() {
        let mut order = make_domestic_order();
        let result = order.execute();
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_clear_from_draft() {
        let mut order = make_domestic_order();
        let result = order.clear();
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_process_from_draft() {
        let mut order = make_domestic_order();
        let result = order.process();
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_payment() {
        let mut order = make_domestic_order();
        order.reject("Compliance issue".to_string());
        assert_eq!(order.status(), PaymentStatus::Rejected);
        assert_eq!(order.rejection_reason(), Some("Compliance issue"));
    }

    #[test]
    fn test_fail_payment() {
        let mut order = make_domestic_order();
        order.submit().unwrap();
        order.fail("Network timeout".to_string());
        assert_eq!(order.status(), PaymentStatus::Failed);
        assert_eq!(order.rejection_reason(), Some("Network timeout"));
    }

    // --- SWIFT Message Tests ---

    #[test]
    fn test_generate_mt103() {
        let order = make_swift_order();
        let msg = SwiftMessage::generate_mt103(&order, "BIATTNTT").unwrap();
        assert_eq!(msg.message_type(), "MT103");
        assert_eq!(msg.sender_bic(), "BIATTNTT");
        assert_eq!(msg.receiver_bic(), "CHASUS33");
        assert_eq!(msg.amount(), order.amount());
        assert_eq!(msg.currency(), "USD");
        assert_eq!(msg.reference(), "REF-SWIFT-001");
        assert_eq!(msg.status(), SwiftMessageStatus::Draft);
    }

    #[test]
    fn test_generate_mt103_requires_bic() {
        let order = make_domestic_order(); // No BIC
        let result = SwiftMessage::generate_mt103(&order, "BIATTNTT");
        assert!(result.is_err());
    }

    #[test]
    fn test_swift_message_state_transitions() {
        let order = make_swift_order();
        let mut msg = SwiftMessage::generate_mt103(&order, "BIATTNTT").unwrap();
        assert_eq!(msg.status(), SwiftMessageStatus::Draft);

        msg.mark_sent();
        assert_eq!(msg.status(), SwiftMessageStatus::Sent);

        msg.mark_acknowledged();
        assert_eq!(msg.status(), SwiftMessageStatus::Acknowledged);
    }

    // --- Transfer Tests ---

    #[test]
    fn test_create_transfer() {
        let order_id = OrderId::new();
        let transfer = Transfer::new(
            order_id.clone(),
            "01234567890123456789".to_string(),
            500_000,
            "TND".to_string(),
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
        );
        assert_eq!(transfer.order_id(), &order_id);
        assert_eq!(transfer.amount(), 500_000);
        assert_eq!(transfer.status(), PaymentStatus::Draft);
        assert!(transfer.clearing_ref().is_none());
    }

    #[test]
    fn test_transfer_clearing() {
        let mut transfer = Transfer::new(
            OrderId::new(),
            "01234567890123456789".to_string(),
            500_000,
            "TND".to_string(),
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
        );
        transfer.set_clearing_ref("CLR-2026-001".to_string());
        transfer.mark_cleared();
        assert_eq!(transfer.clearing_ref(), Some("CLR-2026-001"));
        assert_eq!(transfer.status(), PaymentStatus::Cleared);
    }

    // --- PaymentType / PaymentStatus from_str Tests ---

    #[test]
    fn test_payment_type_from_str() {
        assert_eq!(
            PaymentType::from_str_type("Domestic").unwrap(),
            PaymentType::Domestic
        );
        assert_eq!(
            PaymentType::from_str_type("International").unwrap(),
            PaymentType::International
        );
        assert_eq!(
            PaymentType::from_str_type("Swift").unwrap(),
            PaymentType::Swift
        );
        assert_eq!(
            PaymentType::from_str_type("Sepa").unwrap(),
            PaymentType::Sepa
        );
        assert!(PaymentType::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_payment_status_from_str() {
        assert_eq!(
            PaymentStatus::from_str_type("Draft").unwrap(),
            PaymentStatus::Draft
        );
        assert_eq!(
            PaymentStatus::from_str_type("Executed").unwrap(),
            PaymentStatus::Executed
        );
        assert!(PaymentStatus::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_screening_status_from_str() {
        assert_eq!(
            ScreeningStatus::from_str_type("NotScreened").unwrap(),
            ScreeningStatus::NotScreened
        );
        assert_eq!(
            ScreeningStatus::from_str_type("Cleared").unwrap(),
            ScreeningStatus::Cleared
        );
        assert!(ScreeningStatus::from_str_type("Unknown").is_err());
    }

    // --- from_raw reconstruction ---

    #[test]
    fn test_payment_order_from_raw() {
        let id = OrderId::new();
        let account_id = Uuid::new_v4();
        let now = Utc::now();
        let order = PaymentOrder::from_raw(
            id.clone(),
            account_id,
            "Ahmed".to_string(),
            Some("01234567890123456789".to_string()),
            None,
            500_000,
            "TND".to_string(),
            PaymentType::Domestic,
            PaymentStatus::Executed,
            ScreeningStatus::NotScreened,
            "REF-001".to_string(),
            None,
            now,
            Some(now),
            Some(now),
            None,
        );
        assert_eq!(order.order_id(), &id);
        assert_eq!(order.status(), PaymentStatus::Executed);
    }
}
