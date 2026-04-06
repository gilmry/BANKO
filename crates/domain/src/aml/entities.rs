use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{Currency, Money};

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(Uuid);

impl TransactionId {
    pub fn new() -> Self {
        TransactionId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        TransactionId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TransactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Exchange,
}

impl TransactionType {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionType::Deposit => "Deposit",
            TransactionType::Withdrawal => "Withdrawal",
            TransactionType::Transfer => "Transfer",
            TransactionType::Exchange => "Exchange",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Deposit" => Ok(TransactionType::Deposit),
            "Withdrawal" => Ok(TransactionType::Withdrawal),
            "Transfer" => Ok(TransactionType::Transfer),
            "Exchange" => Ok(TransactionType::Exchange),
            _ => Err(DomainError::InvalidTransaction(format!(
                "Unknown transaction type: {s}"
            ))),
        }
    }

    pub fn is_cash(&self) -> bool {
        matches!(self, TransactionType::Deposit | TransactionType::Withdrawal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Inbound,
    Outbound,
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::Inbound => "Inbound",
            Direction::Outbound => "Outbound",
        }
    }

    pub fn from_str_dir(s: &str) -> Result<Self, DomainError> {
        match s {
            "Inbound" => Ok(Direction::Inbound),
            "Outbound" => Ok(Direction::Outbound),
            _ => Err(DomainError::InvalidTransaction(format!(
                "Unknown direction: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &str {
        match self {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }

    pub fn from_str_level(s: &str) -> Result<Self, DomainError> {
        match s {
            "Low" => Ok(RiskLevel::Low),
            "Medium" => Ok(RiskLevel::Medium),
            "High" => Ok(RiskLevel::High),
            "Critical" => Ok(RiskLevel::Critical),
            _ => Err(DomainError::InvalidAlert(format!(
                "Unknown risk level: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertStatus {
    New,
    UnderReview,
    Confirmed,
    Dismissed,
}

impl AlertStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AlertStatus::New => "New",
            AlertStatus::UnderReview => "UnderReview",
            AlertStatus::Confirmed => "Confirmed",
            AlertStatus::Dismissed => "Dismissed",
        }
    }

    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s {
            "New" => Ok(AlertStatus::New),
            "UnderReview" => Ok(AlertStatus::UnderReview),
            "Confirmed" => Ok(AlertStatus::Confirmed),
            "Dismissed" => Ok(AlertStatus::Dismissed),
            _ => Err(DomainError::InvalidAlert(format!(
                "Unknown alert status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvestigationStatus {
    Open,
    InProgress,
    Escalated,
    ClosedConfirmed,
    ClosedDismissed,
}

impl InvestigationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            InvestigationStatus::Open => "Open",
            InvestigationStatus::InProgress => "InProgress",
            InvestigationStatus::Escalated => "Escalated",
            InvestigationStatus::ClosedConfirmed => "ClosedConfirmed",
            InvestigationStatus::ClosedDismissed => "ClosedDismissed",
        }
    }

    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s {
            "Open" => Ok(InvestigationStatus::Open),
            "InProgress" => Ok(InvestigationStatus::InProgress),
            "Escalated" => Ok(InvestigationStatus::Escalated),
            "ClosedConfirmed" => Ok(InvestigationStatus::ClosedConfirmed),
            "ClosedDismissed" => Ok(InvestigationStatus::ClosedDismissed),
            _ => Err(DomainError::InvalidInvestigation(format!(
                "Unknown investigation status: {s}"
            ))),
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(
            self,
            InvestigationStatus::ClosedConfirmed | InvestigationStatus::ClosedDismissed
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FreezeStatus {
    Active,
    Lifted,
}

impl FreezeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FreezeStatus::Active => "Active",
            FreezeStatus::Lifted => "Lifted",
        }
    }

    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s {
            "Active" => Ok(FreezeStatus::Active),
            "Lifted" => Ok(FreezeStatus::Lifted),
            _ => Err(DomainError::InvalidTransaction(format!(
                "Unknown freeze status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportStatus {
    Draft,
    Submitted,
    Acknowledged,
}

impl ReportStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ReportStatus::Draft => "Draft",
            ReportStatus::Submitted => "Submitted",
            ReportStatus::Acknowledged => "Acknowledged",
        }
    }

    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s {
            "Draft" => Ok(ReportStatus::Draft),
            "Submitted" => Ok(ReportStatus::Submitted),
            "Acknowledged" => Ok(ReportStatus::Acknowledged),
            _ => Err(DomainError::InvalidSuspicionReport(format!(
                "Unknown report status: {s}"
            ))),
        }
    }
}

// --- AML Threshold constant (INV-08) ---

/// AML cash threshold in TND (INV-08: ≥ 5000 TND)
pub const AML_CASH_THRESHOLD_TND: f64 = 5000.0;

// --- Transaction aggregate ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    id: TransactionId,
    account_id: Uuid,
    customer_id: Uuid,
    counterparty: String,
    amount: Money,
    transaction_type: TransactionType,
    direction: Direction,
    timestamp: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        account_id: Uuid,
        customer_id: Uuid,
        counterparty: String,
        amount: Money,
        transaction_type: TransactionType,
        direction: Direction,
        timestamp: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if amount.amount() <= 0.0 {
            return Err(DomainError::InvalidTransaction(
                "Transaction amount must be positive".to_string(),
            ));
        }
        let counterparty = counterparty.trim().to_string();
        if counterparty.is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Counterparty cannot be empty".to_string(),
            ));
        }

        Ok(Transaction {
            id: TransactionId::new(),
            account_id,
            customer_id,
            counterparty,
            amount,
            transaction_type,
            direction,
            timestamp,
            created_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: TransactionId,
        account_id: Uuid,
        customer_id: Uuid,
        counterparty: String,
        amount: Money,
        transaction_type: TransactionType,
        direction: Direction,
        timestamp: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Transaction {
            id,
            account_id,
            customer_id,
            counterparty,
            amount,
            transaction_type,
            direction,
            timestamp,
            created_at,
        }
    }

    /// INV-08: Cash transaction ≥ 5000 TND → automatic AML check
    pub fn requires_aml_check(&self) -> bool {
        self.transaction_type.is_cash()
            && self.amount.currency() == Currency::TND
            && self.amount.amount() >= AML_CASH_THRESHOLD_TND
    }

    // Accessors
    pub fn id(&self) -> &TransactionId {
        &self.id
    }
    pub fn account_id(&self) -> Uuid {
        self.account_id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn counterparty(&self) -> &str {
        &self.counterparty
    }
    pub fn amount(&self) -> &Money {
        &self.amount
    }
    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- Alert entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    id: Uuid,
    transaction_id: TransactionId,
    risk_level: RiskLevel,
    reason: String,
    status: AlertStatus,
    created_at: DateTime<Utc>,
}

impl Alert {
    pub fn new(
        transaction_id: TransactionId,
        risk_level: RiskLevel,
        reason: String,
    ) -> Result<Self, DomainError> {
        if reason.trim().is_empty() {
            return Err(DomainError::InvalidAlert(
                "Alert reason cannot be empty".to_string(),
            ));
        }
        Ok(Alert {
            id: Uuid::new_v4(),
            transaction_id,
            risk_level,
            reason,
            status: AlertStatus::New,
            created_at: Utc::now(),
        })
    }

    pub fn from_parts(
        id: Uuid,
        transaction_id: TransactionId,
        risk_level: RiskLevel,
        reason: String,
        status: AlertStatus,
        created_at: DateTime<Utc>,
    ) -> Self {
        Alert {
            id,
            transaction_id,
            risk_level,
            reason,
            status,
            created_at,
        }
    }

    pub fn mark_under_review(&mut self) {
        self.status = AlertStatus::UnderReview;
    }

    pub fn confirm(&mut self) {
        self.status = AlertStatus::Confirmed;
    }

    pub fn dismiss(&mut self) {
        self.status = AlertStatus::Dismissed;
    }

    // Accessors
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }
    pub fn risk_level(&self) -> RiskLevel {
        self.risk_level
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn status(&self) -> AlertStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- Investigation Note ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestigationNote {
    note: String,
    author: String,
    created_at: DateTime<Utc>,
}

impl InvestigationNote {
    pub fn new(note: String, author: String) -> Result<Self, DomainError> {
        if note.trim().is_empty() {
            return Err(DomainError::InvalidInvestigation(
                "Note cannot be empty".to_string(),
            ));
        }
        if author.trim().is_empty() {
            return Err(DomainError::InvalidInvestigation(
                "Author cannot be empty".to_string(),
            ));
        }
        Ok(InvestigationNote {
            note: note.trim().to_string(),
            author: author.trim().to_string(),
            created_at: Utc::now(),
        })
    }

    pub fn from_parts(note: String, author: String, created_at: DateTime<Utc>) -> Self {
        InvestigationNote {
            note,
            author,
            created_at,
        }
    }

    pub fn note(&self) -> &str {
        &self.note
    }
    pub fn author(&self) -> &str {
        &self.author
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- Investigation entity ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Investigation {
    id: Uuid,
    alert_id: Uuid,
    status: InvestigationStatus,
    assigned_to: Option<String>,
    notes: Vec<InvestigationNote>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Investigation {
    pub fn new(alert_id: Uuid, assigned_to: Option<String>) -> Self {
        let now = Utc::now();
        Investigation {
            id: Uuid::new_v4(),
            alert_id,
            status: InvestigationStatus::Open,
            assigned_to,
            notes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_parts(
        id: Uuid,
        alert_id: Uuid,
        status: InvestigationStatus,
        assigned_to: Option<String>,
        notes: Vec<InvestigationNote>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Investigation {
            id,
            alert_id,
            status,
            assigned_to,
            notes,
            created_at,
            updated_at,
        }
    }

    pub fn add_note(&mut self, note: InvestigationNote) -> Result<(), DomainError> {
        if self.status.is_closed() {
            return Err(DomainError::InvalidInvestigationTransition(
                "Cannot add notes to a closed investigation".to_string(),
            ));
        }
        self.notes.push(note);
        self.updated_at = Utc::now();
        if self.status == InvestigationStatus::Open {
            self.status = InvestigationStatus::InProgress;
        }
        Ok(())
    }

    pub fn escalate(&mut self) -> Result<(), DomainError> {
        if self.status.is_closed() {
            return Err(DomainError::InvalidInvestigationTransition(
                "Cannot escalate a closed investigation".to_string(),
            ));
        }
        self.status = InvestigationStatus::Escalated;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn close_confirmed(&mut self) -> Result<(), DomainError> {
        if self.status.is_closed() {
            return Err(DomainError::InvalidInvestigationTransition(
                "Investigation is already closed".to_string(),
            ));
        }
        self.status = InvestigationStatus::ClosedConfirmed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn close_dismissed(&mut self) -> Result<(), DomainError> {
        if self.status.is_closed() {
            return Err(DomainError::InvalidInvestigationTransition(
                "Investigation is already closed".to_string(),
            ));
        }
        self.status = InvestigationStatus::ClosedDismissed;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Accessors
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn alert_id(&self) -> Uuid {
        self.alert_id
    }
    pub fn status(&self) -> InvestigationStatus {
        self.status
    }
    pub fn assigned_to(&self) -> Option<&str> {
        self.assigned_to.as_deref()
    }
    pub fn notes(&self) -> &[InvestigationNote] {
        &self.notes
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Suspicion Report (DOS) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuspicionReport {
    id: Uuid,
    investigation_id: Uuid,
    customer_info: String,
    transaction_details: String,
    reasons: String,
    evidence: Option<String>,
    timeline: Option<String>,
    status: ReportStatus,
    created_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
}

impl SuspicionReport {
    pub fn new(
        investigation_id: Uuid,
        customer_info: String,
        transaction_details: String,
        reasons: String,
        evidence: Option<String>,
        timeline: Option<String>,
    ) -> Result<Self, DomainError> {
        if customer_info.trim().is_empty() {
            return Err(DomainError::InvalidSuspicionReport(
                "Customer info cannot be empty".to_string(),
            ));
        }
        if reasons.trim().is_empty() {
            return Err(DomainError::InvalidSuspicionReport(
                "Reasons cannot be empty".to_string(),
            ));
        }
        Ok(SuspicionReport {
            id: Uuid::new_v4(),
            investigation_id,
            customer_info,
            transaction_details,
            reasons,
            evidence,
            timeline,
            status: ReportStatus::Draft,
            created_at: Utc::now(),
            submitted_at: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: Uuid,
        investigation_id: Uuid,
        customer_info: String,
        transaction_details: String,
        reasons: String,
        evidence: Option<String>,
        timeline: Option<String>,
        status: ReportStatus,
        created_at: DateTime<Utc>,
        submitted_at: Option<DateTime<Utc>>,
    ) -> Self {
        SuspicionReport {
            id,
            investigation_id,
            customer_info,
            transaction_details,
            reasons,
            evidence,
            timeline,
            status,
            created_at,
            submitted_at,
        }
    }

    pub fn submit(&mut self) -> Result<(), DomainError> {
        if self.status != ReportStatus::Draft {
            return Err(DomainError::InvalidSuspicionReport(
                "Can only submit draft reports".to_string(),
            ));
        }
        self.status = ReportStatus::Submitted;
        self.submitted_at = Some(Utc::now());
        Ok(())
    }

    pub fn acknowledge(&mut self) -> Result<(), DomainError> {
        if self.status != ReportStatus::Submitted {
            return Err(DomainError::InvalidSuspicionReport(
                "Can only acknowledge submitted reports".to_string(),
            ));
        }
        self.status = ReportStatus::Acknowledged;
        Ok(())
    }

    // Accessors
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn investigation_id(&self) -> Uuid {
        self.investigation_id
    }
    pub fn customer_info(&self) -> &str {
        &self.customer_info
    }
    pub fn transaction_details(&self) -> &str {
        &self.transaction_details
    }
    pub fn reasons(&self) -> &str {
        &self.reasons
    }
    pub fn evidence(&self) -> Option<&str> {
        self.evidence.as_deref()
    }
    pub fn timeline(&self) -> Option<&str> {
        self.timeline.as_deref()
    }
    pub fn status(&self) -> ReportStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn submitted_at(&self) -> Option<DateTime<Utc>> {
        self.submitted_at
    }
}

// --- Asset Freeze (INV-09) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetFreeze {
    id: Uuid,
    account_id: Uuid,
    reason: String,
    ordered_by: String,
    status: FreezeStatus,
    frozen_at: DateTime<Utc>,
    lifted_at: Option<DateTime<Utc>>,
    lifted_by: Option<String>,
}

impl AssetFreeze {
    /// Freeze is IMMEDIATE (INV-09). No pending state.
    pub fn freeze(
        account_id: Uuid,
        reason: String,
        ordered_by: String,
    ) -> Result<Self, DomainError> {
        if reason.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Freeze reason cannot be empty".to_string(),
            ));
        }
        if ordered_by.trim().is_empty() {
            return Err(DomainError::InvalidTransaction(
                "Freeze ordered_by cannot be empty".to_string(),
            ));
        }
        Ok(AssetFreeze {
            id: Uuid::new_v4(),
            account_id,
            reason,
            ordered_by,
            status: FreezeStatus::Active,
            frozen_at: Utc::now(),
            lifted_at: None,
            lifted_by: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: Uuid,
        account_id: Uuid,
        reason: String,
        ordered_by: String,
        status: FreezeStatus,
        frozen_at: DateTime<Utc>,
        lifted_at: Option<DateTime<Utc>>,
        lifted_by: Option<String>,
    ) -> Self {
        AssetFreeze {
            id,
            account_id,
            reason,
            ordered_by,
            status,
            frozen_at,
            lifted_at,
            lifted_by,
        }
    }

    /// Lift freeze — requires CTAF authorization (lifted_by must be provided).
    pub fn lift(&mut self, lifted_by: String) -> Result<(), DomainError> {
        if self.status == FreezeStatus::Lifted {
            return Err(DomainError::FreezeIrrevocable);
        }
        if lifted_by.trim().is_empty() {
            return Err(DomainError::FreezeIrrevocable);
        }
        self.status = FreezeStatus::Lifted;
        self.lifted_at = Some(Utc::now());
        self.lifted_by = Some(lifted_by);
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.status == FreezeStatus::Active
    }

    // Accessors
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn account_id(&self) -> Uuid {
        self.account_id
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn ordered_by(&self) -> &str {
        &self.ordered_by
    }
    pub fn status(&self) -> FreezeStatus {
        self.status
    }
    pub fn frozen_at(&self) -> DateTime<Utc> {
        self.frozen_at
    }
    pub fn lifted_at(&self) -> Option<DateTime<Utc>> {
        self.lifted_at
    }
    pub fn lifted_by(&self) -> Option<&str> {
        self.lifted_by.as_deref()
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    fn sample_transaction(amount: f64, tx_type: TransactionType) -> Transaction {
        Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "John Doe".to_string(),
            tnd(amount),
            tx_type,
            Direction::Inbound,
            Utc::now(),
        )
        .unwrap()
    }

    // --- Transaction tests ---

    #[test]
    fn test_create_valid_transaction() {
        let tx = sample_transaction(1000.0, TransactionType::Deposit);
        assert_eq!(tx.amount().amount(), 1000.0);
        assert_eq!(tx.counterparty(), "John Doe");
        assert_eq!(tx.transaction_type(), TransactionType::Deposit);
    }

    #[test]
    fn test_transaction_zero_amount_rejected() {
        let result = Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test".to_string(),
            tnd(0.0),
            TransactionType::Deposit,
            Direction::Inbound,
            Utc::now(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_negative_amount_rejected() {
        let result = Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test".to_string(),
            tnd(-100.0),
            TransactionType::Deposit,
            Direction::Inbound,
            Utc::now(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_empty_counterparty_rejected() {
        let result = Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "  ".to_string(),
            tnd(1000.0),
            TransactionType::Deposit,
            Direction::Inbound,
            Utc::now(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_inv08_cash_above_threshold() {
        let tx = sample_transaction(5000.0, TransactionType::Deposit);
        assert!(tx.requires_aml_check());
    }

    #[test]
    fn test_inv08_cash_below_threshold() {
        let tx = sample_transaction(4999.0, TransactionType::Deposit);
        assert!(!tx.requires_aml_check());
    }

    #[test]
    fn test_inv08_transfer_above_threshold_no_check() {
        let tx = sample_transaction(10000.0, TransactionType::Transfer);
        assert!(!tx.requires_aml_check());
    }

    #[test]
    fn test_inv08_withdrawal_above_threshold() {
        let tx = sample_transaction(6000.0, TransactionType::Withdrawal);
        assert!(tx.requires_aml_check());
    }

    // --- Alert tests ---

    #[test]
    fn test_create_alert() {
        let alert = Alert::new(
            TransactionId::new(),
            RiskLevel::Medium,
            "Threshold exceeded".to_string(),
        )
        .unwrap();
        assert_eq!(alert.status(), AlertStatus::New);
        assert_eq!(alert.risk_level(), RiskLevel::Medium);
    }

    #[test]
    fn test_alert_empty_reason_rejected() {
        let result = Alert::new(TransactionId::new(), RiskLevel::High, "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_alert_status_transitions() {
        let mut alert = Alert::new(
            TransactionId::new(),
            RiskLevel::High,
            "Suspicious pattern".to_string(),
        )
        .unwrap();
        alert.mark_under_review();
        assert_eq!(alert.status(), AlertStatus::UnderReview);
        alert.confirm();
        assert_eq!(alert.status(), AlertStatus::Confirmed);
    }

    // --- Investigation tests ---

    #[test]
    fn test_create_investigation() {
        let inv = Investigation::new(Uuid::new_v4(), Some("analyst1".to_string()));
        assert_eq!(inv.status(), InvestigationStatus::Open);
        assert_eq!(inv.assigned_to(), Some("analyst1"));
    }

    #[test]
    fn test_investigation_add_note_moves_to_in_progress() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        let note =
            InvestigationNote::new("Initial review".to_string(), "analyst".to_string()).unwrap();
        inv.add_note(note).unwrap();
        assert_eq!(inv.status(), InvestigationStatus::InProgress);
        assert_eq!(inv.notes().len(), 1);
    }

    #[test]
    fn test_investigation_escalate() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        inv.escalate().unwrap();
        assert_eq!(inv.status(), InvestigationStatus::Escalated);
    }

    #[test]
    fn test_investigation_close_confirmed() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        inv.close_confirmed().unwrap();
        assert_eq!(inv.status(), InvestigationStatus::ClosedConfirmed);
    }

    #[test]
    fn test_investigation_close_dismissed() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        inv.close_dismissed().unwrap();
        assert_eq!(inv.status(), InvestigationStatus::ClosedDismissed);
    }

    #[test]
    fn test_investigation_cannot_escalate_when_closed() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        inv.close_confirmed().unwrap();
        assert!(inv.escalate().is_err());
    }

    #[test]
    fn test_investigation_cannot_add_note_when_closed() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        inv.close_dismissed().unwrap();
        let note = InvestigationNote::new("Late note".to_string(), "analyst".to_string()).unwrap();
        assert!(inv.add_note(note).is_err());
    }

    #[test]
    fn test_investigation_cannot_close_twice() {
        let mut inv = Investigation::new(Uuid::new_v4(), None);
        inv.close_confirmed().unwrap();
        assert!(inv.close_dismissed().is_err());
    }

    // --- Suspicion Report tests ---

    #[test]
    fn test_create_suspicion_report() {
        let report = SuspicionReport::new(
            Uuid::new_v4(),
            "Customer info".to_string(),
            "Transaction details".to_string(),
            "Suspicious activity".to_string(),
            Some("Evidence".to_string()),
            None,
        )
        .unwrap();
        assert_eq!(report.status(), ReportStatus::Draft);
    }

    #[test]
    fn test_report_empty_reasons_rejected() {
        let result = SuspicionReport::new(
            Uuid::new_v4(),
            "info".to_string(),
            "details".to_string(),
            "".to_string(),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_report_submit() {
        let mut report = SuspicionReport::new(
            Uuid::new_v4(),
            "info".to_string(),
            "details".to_string(),
            "reasons".to_string(),
            None,
            None,
        )
        .unwrap();
        report.submit().unwrap();
        assert_eq!(report.status(), ReportStatus::Submitted);
        assert!(report.submitted_at().is_some());
    }

    #[test]
    fn test_report_cannot_submit_twice() {
        let mut report = SuspicionReport::new(
            Uuid::new_v4(),
            "info".to_string(),
            "details".to_string(),
            "reasons".to_string(),
            None,
            None,
        )
        .unwrap();
        report.submit().unwrap();
        assert!(report.submit().is_err());
    }

    #[test]
    fn test_report_acknowledge() {
        let mut report = SuspicionReport::new(
            Uuid::new_v4(),
            "info".to_string(),
            "details".to_string(),
            "reasons".to_string(),
            None,
            None,
        )
        .unwrap();
        report.submit().unwrap();
        report.acknowledge().unwrap();
        assert_eq!(report.status(), ReportStatus::Acknowledged);
    }

    // --- Asset Freeze tests (INV-09) ---

    #[test]
    fn test_freeze_immediate() {
        let freeze = AssetFreeze::freeze(
            Uuid::new_v4(),
            "Suspicious activity".to_string(),
            "supervisor".to_string(),
        )
        .unwrap();
        assert_eq!(freeze.status(), FreezeStatus::Active);
        assert!(freeze.is_active());
    }

    #[test]
    fn test_freeze_empty_reason_rejected() {
        let result = AssetFreeze::freeze(Uuid::new_v4(), "".to_string(), "supervisor".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_freeze_lift_with_authorization() {
        let mut freeze = AssetFreeze::freeze(
            Uuid::new_v4(),
            "Suspicious activity".to_string(),
            "supervisor".to_string(),
        )
        .unwrap();
        freeze.lift("CTAF_officer".to_string()).unwrap();
        assert_eq!(freeze.status(), FreezeStatus::Lifted);
        assert!(freeze.lifted_at().is_some());
        assert_eq!(freeze.lifted_by(), Some("CTAF_officer"));
    }

    #[test]
    fn test_freeze_lift_without_authorization_rejected() {
        let mut freeze = AssetFreeze::freeze(
            Uuid::new_v4(),
            "reason".to_string(),
            "supervisor".to_string(),
        )
        .unwrap();
        assert!(freeze.lift("".to_string()).is_err());
    }

    #[test]
    fn test_freeze_cannot_lift_twice() {
        let mut freeze = AssetFreeze::freeze(
            Uuid::new_v4(),
            "reason".to_string(),
            "supervisor".to_string(),
        )
        .unwrap();
        freeze.lift("CTAF".to_string()).unwrap();
        assert!(freeze.lift("CTAF".to_string()).is_err());
    }

    // --- InvestigationNote tests ---

    #[test]
    fn test_note_empty_rejected() {
        assert!(InvestigationNote::new("".to_string(), "author".to_string()).is_err());
    }

    #[test]
    fn test_note_empty_author_rejected() {
        assert!(InvestigationNote::new("note".to_string(), "".to_string()).is_err());
    }
}
