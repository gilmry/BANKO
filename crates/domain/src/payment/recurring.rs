use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Value Objects / Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    Yearly,
}

impl Frequency {
    pub fn as_str(&self) -> &str {
        match self {
            Frequency::Daily => "Daily",
            Frequency::Weekly => "Weekly",
            Frequency::BiWeekly => "BiWeekly",
            Frequency::Monthly => "Monthly",
            Frequency::Quarterly => "Quarterly",
            Frequency::Yearly => "Yearly",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Daily" => Ok(Frequency::Daily),
            "Weekly" => Ok(Frequency::Weekly),
            "BiWeekly" => Ok(Frequency::BiWeekly),
            "Monthly" => Ok(Frequency::Monthly),
            "Quarterly" => Ok(Frequency::Quarterly),
            "Yearly" => Ok(Frequency::Yearly),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown frequency: {}",
                s
            ))),
        }
    }

    /// Calculate days between executions for this frequency
    pub fn days_between(&self) -> u32 {
        match self {
            Frequency::Daily => 1,
            Frequency::Weekly => 7,
            Frequency::BiWeekly => 14,
            Frequency::Monthly => 30, // Approximate
            Frequency::Quarterly => 90, // Approximate
            Frequency::Yearly => 365,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StandingOrderStatus {
    Active,
    Suspended,
    Completed,
    Cancelled,
    Failed,
}

impl StandingOrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            StandingOrderStatus::Active => "Active",
            StandingOrderStatus::Suspended => "Suspended",
            StandingOrderStatus::Completed => "Completed",
            StandingOrderStatus::Cancelled => "Cancelled",
            StandingOrderStatus::Failed => "Failed",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Active" => Ok(StandingOrderStatus::Active),
            "Suspended" => Ok(StandingOrderStatus::Suspended),
            "Completed" => Ok(StandingOrderStatus::Completed),
            "Cancelled" => Ok(StandingOrderStatus::Cancelled),
            "Failed" => Ok(StandingOrderStatus::Failed),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown standing order status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MandateStatus {
    PendingSignature,
    Active,
    Suspended,
    Revoked,
    Expired,
}

impl MandateStatus {
    pub fn as_str(&self) -> &str {
        match self {
            MandateStatus::PendingSignature => "PendingSignature",
            MandateStatus::Active => "Active",
            MandateStatus::Suspended => "Suspended",
            MandateStatus::Revoked => "Revoked",
            MandateStatus::Expired => "Expired",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "PendingSignature" => Ok(MandateStatus::PendingSignature),
            "Active" => Ok(MandateStatus::Active),
            "Suspended" => Ok(MandateStatus::Suspended),
            "Revoked" => Ok(MandateStatus::Revoked),
            "Expired" => Ok(MandateStatus::Expired),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown mandate status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebitExecutionStatus {
    Pending,
    Executed,
    Failed,
    Rejected,
}

impl DebitExecutionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            DebitExecutionStatus::Pending => "Pending",
            DebitExecutionStatus::Executed => "Executed",
            DebitExecutionStatus::Failed => "Failed",
            DebitExecutionStatus::Rejected => "Rejected",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Pending" => Ok(DebitExecutionStatus::Pending),
            "Executed" => Ok(DebitExecutionStatus::Executed),
            "Failed" => Ok(DebitExecutionStatus::Failed),
            "Rejected" => Ok(DebitExecutionStatus::Rejected),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown debit execution status: {}",
                s
            ))),
        }
    }
}

// ============================================================
// StandingOrder Aggregate (STORY-RECUR-01)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingOrder {
    id: Uuid,
    account_id: Uuid,
    beneficiary_account: String,
    beneficiary_name: String,
    amount: Decimal,
    currency: String,
    frequency: Frequency,
    reference: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    next_execution_date: NaiveDate,
    last_executed_at: Option<DateTime<Utc>>,
    status: StandingOrderStatus,
    execution_count: u32,
    max_executions: Option<u32>,
    created_at: DateTime<Utc>,
}

impl StandingOrder {
    /// Create a new standing order with validation
    pub fn new(
        account_id: Uuid,
        beneficiary_account: String,
        beneficiary_name: String,
        amount: Decimal,
        currency: String,
        frequency: Frequency,
        reference: String,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        max_executions: Option<u32>,
    ) -> Result<Self, DomainError> {
        // Validation: amount > 0
        if amount <= Decimal::ZERO {
            return Err(DomainError::InvalidPaymentOrder(
                "Amount must be greater than 0".to_string(),
            ));
        }

        // Validation: start_date not in past
        let today = Utc::now().naive_utc().date();
        if start_date < today {
            return Err(DomainError::InvalidPaymentOrder(
                "Start date cannot be in the past".to_string(),
            ));
        }

        // Validation: beneficiary not empty
        if beneficiary_name.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Beneficiary name cannot be empty".to_string(),
            ));
        }

        if beneficiary_account.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Beneficiary account cannot be empty".to_string(),
            ));
        }

        // Validation: end_date must be after start_date if provided
        if let Some(end) = end_date {
            if end <= start_date {
                return Err(DomainError::InvalidPaymentOrder(
                    "End date must be after start date".to_string(),
                ));
            }
        }

        Ok(StandingOrder {
            id: Uuid::new_v4(),
            account_id,
            beneficiary_account,
            beneficiary_name,
            amount,
            currency,
            frequency,
            reference,
            start_date,
            end_date,
            next_execution_date: start_date,
            last_executed_at: None,
            status: StandingOrderStatus::Active,
            execution_count: 0,
            max_executions,
            created_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence (no validation)
    pub fn from_raw(
        id: Uuid,
        account_id: Uuid,
        beneficiary_account: String,
        beneficiary_name: String,
        amount: Decimal,
        currency: String,
        frequency: Frequency,
        reference: String,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        next_execution_date: NaiveDate,
        last_executed_at: Option<DateTime<Utc>>,
        status: StandingOrderStatus,
        execution_count: u32,
        max_executions: Option<u32>,
        created_at: DateTime<Utc>,
    ) -> Self {
        StandingOrder {
            id,
            account_id,
            beneficiary_account,
            beneficiary_name,
            amount,
            currency,
            frequency,
            reference,
            start_date,
            end_date,
            next_execution_date,
            last_executed_at,
            status,
            execution_count,
            max_executions,
            created_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn beneficiary_account(&self) -> &str {
        &self.beneficiary_account
    }

    pub fn beneficiary_name(&self) -> &str {
        &self.beneficiary_name
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn frequency(&self) -> Frequency {
        self.frequency
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn end_date(&self) -> Option<NaiveDate> {
        self.end_date
    }

    pub fn next_execution_date(&self) -> NaiveDate {
        self.next_execution_date
    }

    pub fn last_executed_at(&self) -> Option<DateTime<Utc>> {
        self.last_executed_at
    }

    pub fn status(&self) -> StandingOrderStatus {
        self.status
    }

    pub fn execution_count(&self) -> u32 {
        self.execution_count
    }

    pub fn max_executions(&self) -> Option<u32> {
        self.max_executions
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // --- Business Logic ---

    /// Check if this standing order is due for execution today
    pub fn is_due_today(&self, today: NaiveDate) -> bool {
        self.status == StandingOrderStatus::Active
            && today >= self.next_execution_date
            && self.end_date.map_or(true, |end| today <= end)
            && !self.is_completed()
    }

    /// Calculate the next execution date based on frequency
    pub fn calculate_next_execution_date(&self) -> NaiveDate {
        use chrono::Duration;
        self.next_execution_date
            + Duration::days(self.frequency.days_between() as i64)
    }

    /// Mark this standing order as executed
    pub fn mark_executed(&mut self, execution_date: DateTime<Utc>) {
        self.last_executed_at = Some(execution_date);
        self.execution_count += 1;
        self.next_execution_date = self.calculate_next_execution_date();

        // Check if completed
        if self.is_completed() {
            self.status = StandingOrderStatus::Completed;
        }
    }

    /// Suspend this standing order
    pub fn suspend(&mut self) {
        if self.status == StandingOrderStatus::Active {
            self.status = StandingOrderStatus::Suspended;
        }
    }

    /// Resume this standing order
    pub fn resume(&mut self) {
        if self.status == StandingOrderStatus::Suspended {
            self.status = StandingOrderStatus::Active;
        }
    }

    /// Cancel this standing order
    pub fn cancel(&mut self) {
        self.status = StandingOrderStatus::Cancelled;
    }

    /// Check if this standing order is completed
    pub fn is_completed(&self) -> bool {
        // Completed if end_date reached or max_executions reached
        let end_date_reached = self.end_date.map_or(false, |end| Utc::now().naive_utc().date() > end);
        let max_executions_reached = self
            .max_executions
            .map_or(false, |max| self.execution_count >= max);

        end_date_reached || max_executions_reached
    }
}

// ============================================================
// DirectDebitMandate Aggregate (STORY-RECUR-02)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectDebitMandate {
    id: Uuid,
    debtor_account_id: Uuid,
    debtor_name: String,
    creditor_id: String,
    creditor_name: String,
    amount_limit: Decimal,
    currency: String,
    frequency: Frequency,
    reference: String,
    signed_at: Option<DateTime<Utc>>,
    expires_at: Option<NaiveDate>,
    status: MandateStatus,
    created_at: DateTime<Utc>,
}

impl DirectDebitMandate {
    /// Create a new direct debit mandate with validation
    pub fn new(
        debtor_account_id: Uuid,
        debtor_name: String,
        creditor_id: String,
        creditor_name: String,
        amount_limit: Decimal,
        currency: String,
        frequency: Frequency,
        reference: String,
        expires_at: Option<NaiveDate>,
    ) -> Result<Self, DomainError> {
        // Validation: amount_limit > 0
        if amount_limit <= Decimal::ZERO {
            return Err(DomainError::InvalidPaymentOrder(
                "Amount limit must be greater than 0".to_string(),
            ));
        }

        // Validation: debtor_name not empty
        if debtor_name.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Debtor name cannot be empty".to_string(),
            ));
        }

        // Validation: creditor_id not empty
        if creditor_id.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Creditor ID cannot be empty".to_string(),
            ));
        }

        // Validation: creditor_name not empty
        if creditor_name.trim().is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Creditor name cannot be empty".to_string(),
            ));
        }

        Ok(DirectDebitMandate {
            id: Uuid::new_v4(),
            debtor_account_id,
            debtor_name,
            creditor_id,
            creditor_name,
            amount_limit,
            currency,
            frequency,
            reference,
            signed_at: None,
            expires_at,
            status: MandateStatus::PendingSignature,
            created_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        id: Uuid,
        debtor_account_id: Uuid,
        debtor_name: String,
        creditor_id: String,
        creditor_name: String,
        amount_limit: Decimal,
        currency: String,
        frequency: Frequency,
        reference: String,
        signed_at: Option<DateTime<Utc>>,
        expires_at: Option<NaiveDate>,
        status: MandateStatus,
        created_at: DateTime<Utc>,
    ) -> Self {
        DirectDebitMandate {
            id,
            debtor_account_id,
            debtor_name,
            creditor_id,
            creditor_name,
            amount_limit,
            currency,
            frequency,
            reference,
            signed_at,
            expires_at,
            status,
            created_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn debtor_account_id(&self) -> Uuid {
        self.debtor_account_id
    }

    pub fn debtor_name(&self) -> &str {
        &self.debtor_name
    }

    pub fn creditor_id(&self) -> &str {
        &self.creditor_id
    }

    pub fn creditor_name(&self) -> &str {
        &self.creditor_name
    }

    pub fn amount_limit(&self) -> Decimal {
        self.amount_limit
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn frequency(&self) -> Frequency {
        self.frequency
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn signed_at(&self) -> Option<DateTime<Utc>> {
        self.signed_at
    }

    pub fn expires_at(&self) -> Option<NaiveDate> {
        self.expires_at
    }

    pub fn status(&self) -> MandateStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // --- Business Logic ---

    /// Sign the mandate (PendingSignature -> Active)
    pub fn sign(&mut self, timestamp: DateTime<Utc>) {
        if self.status == MandateStatus::PendingSignature {
            self.status = MandateStatus::Active;
            self.signed_at = Some(timestamp);
        }
    }

    /// Revoke the mandate
    pub fn revoke(&mut self) {
        self.status = MandateStatus::Revoked;
    }

    /// Suspend the mandate
    pub fn suspend(&mut self) {
        if self.status == MandateStatus::Active {
            self.status = MandateStatus::Suspended;
        }
    }

    /// Resume the mandate
    pub fn resume(&mut self) {
        if self.status == MandateStatus::Suspended {
            self.status = MandateStatus::Active;
        }
    }

    /// Check if the mandate is expired
    pub fn is_expired(&self, today: NaiveDate) -> bool {
        if let Some(exp) = self.expires_at {
            today > exp
        } else {
            false
        }
    }

    /// Check if a debit can be executed under this mandate
    pub fn can_debit(&self, amount: Decimal, today: NaiveDate) -> bool {
        self.status == MandateStatus::Active
            && !self.is_expired(today)
            && amount <= self.amount_limit
    }
}

// ============================================================
// DebitExecution Entity (STORY-RECUR-02)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebitExecution {
    id: Uuid,
    mandate_id: Uuid,
    amount: Decimal,
    execution_date: DateTime<Utc>,
    status: DebitExecutionStatus,
    reason: Option<String>,
}

impl DebitExecution {
    pub fn new(mandate_id: Uuid, amount: Decimal) -> Self {
        DebitExecution {
            id: Uuid::new_v4(),
            mandate_id,
            amount,
            execution_date: Utc::now(),
            status: DebitExecutionStatus::Pending,
            reason: None,
        }
    }

    pub fn from_raw(
        id: Uuid,
        mandate_id: Uuid,
        amount: Decimal,
        execution_date: DateTime<Utc>,
        status: DebitExecutionStatus,
        reason: Option<String>,
    ) -> Self {
        DebitExecution {
            id,
            mandate_id,
            amount,
            execution_date,
            status,
            reason,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn mandate_id(&self) -> Uuid {
        self.mandate_id
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn execution_date(&self) -> DateTime<Utc> {
        self.execution_date
    }

    pub fn status(&self) -> DebitExecutionStatus {
        self.status
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn mark_executed(&mut self) {
        self.status = DebitExecutionStatus::Executed;
    }

    pub fn mark_failed(&mut self, reason: String) {
        self.status = DebitExecutionStatus::Failed;
        self.reason = Some(reason);
    }

    pub fn mark_rejected(&mut self, reason: String) {
        self.status = DebitExecutionStatus::Rejected;
        self.reason = Some(reason);
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    // --- StandingOrder Tests ---

    #[test]
    fn test_create_standing_order() {
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let order = StandingOrder::new(
            account_id,
            "TN1234567890".to_string(),
            "Ahmed Ben Ali".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            start,
            None,
            None,
        )
        .unwrap();

        assert_eq!(order.account_id(), account_id);
        assert_eq!(order.status(), StandingOrderStatus::Active);
        assert_eq!(order.execution_count(), 0);
        assert_eq!(order.next_execution_date(), start);
    }

    #[test]
    fn test_standing_order_invalid_amount() {
        let result = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::ZERO,
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            NaiveDate::from_ymd_opt(2026, 4, 10).unwrap(),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_standing_order_start_date_in_past() {
        let result = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_standing_order_empty_beneficiary() {
        let result = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            NaiveDate::from_ymd_opt(2026, 4, 10).unwrap(),
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_standing_order_end_date_before_start() {
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();
        let result = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            start,
            Some(end),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_standing_order_is_due_today() {
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Daily,
            "Loyer".to_string(),
            today,
            None,
            None,
        )
        .unwrap();

        assert!(order.is_due_today(today));
    }

    #[test]
    fn test_standing_order_not_due_yet() {
        let start = NaiveDate::from_ymd_opt(2026, 4, 20).unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            start,
            None,
            None,
        )
        .unwrap();

        assert!(!order.is_due_today(today));
    }

    #[test]
    fn test_frequency_days_between() {
        assert_eq!(Frequency::Daily.days_between(), 1);
        assert_eq!(Frequency::Weekly.days_between(), 7);
        assert_eq!(Frequency::BiWeekly.days_between(), 14);
        assert_eq!(Frequency::Monthly.days_between(), 30);
        assert_eq!(Frequency::Quarterly.days_between(), 90);
        assert_eq!(Frequency::Yearly.days_between(), 365);
    }

    #[test]
    fn test_standing_order_calculate_next_execution() {
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Weekly,
            "Loyer".to_string(),
            today,
            None,
            None,
        )
        .unwrap();

        let next = order.calculate_next_execution_date();
        assert_eq!(next, today + Duration::days(7));
    }

    #[test]
    fn test_standing_order_mark_executed() {
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let mut order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            today,
            None,
            Some(2),
        )
        .unwrap();

        assert_eq!(order.execution_count(), 0);
        order.mark_executed(Utc::now());
        assert_eq!(order.execution_count(), 1);
        assert!(order.last_executed_at().is_some());
        assert_eq!(order.status(), StandingOrderStatus::Active);
    }

    #[test]
    fn test_standing_order_completed_after_max_executions() {
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let mut order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            today,
            None,
            Some(1),
        )
        .unwrap();

        order.mark_executed(Utc::now());
        assert!(order.is_completed());
        assert_eq!(order.status(), StandingOrderStatus::Completed);
    }

    #[test]
    fn test_standing_order_suspend_resume() {
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let mut order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            today,
            None,
            None,
        )
        .unwrap();

        order.suspend();
        assert_eq!(order.status(), StandingOrderStatus::Suspended);

        order.resume();
        assert_eq!(order.status(), StandingOrderStatus::Active);
    }

    #[test]
    fn test_standing_order_cancel() {
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let mut order = StandingOrder::new(
            Uuid::new_v4(),
            "TN1234567890".to_string(),
            "Ahmed".to_string(),
            Decimal::from(500),
            "TND".to_string(),
            Frequency::Monthly,
            "Loyer".to_string(),
            today,
            None,
            None,
        )
        .unwrap();

        order.cancel();
        assert_eq!(order.status(), StandingOrderStatus::Cancelled);
    }

    // --- DirectDebitMandate Tests ---

    #[test]
    fn test_create_mandate() {
        let debtor_id = Uuid::new_v4();
        let mandate = DirectDebitMandate::new(
            debtor_id,
            "Ahmed Ben Ali".to_string(),
            "CRED-001".to_string(),
            "Electricity Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Electricity".to_string(),
            None,
        )
        .unwrap();

        assert_eq!(mandate.debtor_account_id(), debtor_id);
        assert_eq!(mandate.status(), MandateStatus::PendingSignature);
        assert!(mandate.signed_at().is_none());
    }

    #[test]
    fn test_mandate_invalid_amount_limit() {
        let result = DirectDebitMandate::new(
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::ZERO,
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_mandate_empty_debtor_name() {
        let result = DirectDebitMandate::new(
            Uuid::new_v4(),
            "".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_mandate_sign() {
        let debtor_id = Uuid::new_v4();
        let mut mandate = DirectDebitMandate::new(
            debtor_id,
            "Ahmed".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            None,
        )
        .unwrap();

        assert_eq!(mandate.status(), MandateStatus::PendingSignature);
        mandate.sign(Utc::now());
        assert_eq!(mandate.status(), MandateStatus::Active);
        assert!(mandate.signed_at().is_some());
    }

    #[test]
    fn test_mandate_revoke() {
        let debtor_id = Uuid::new_v4();
        let mut mandate = DirectDebitMandate::new(
            debtor_id,
            "Ahmed".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            None,
        )
        .unwrap();

        mandate.sign(Utc::now());
        mandate.revoke();
        assert_eq!(mandate.status(), MandateStatus::Revoked);
    }

    #[test]
    fn test_mandate_suspend_resume() {
        let debtor_id = Uuid::new_v4();
        let mut mandate = DirectDebitMandate::new(
            debtor_id,
            "Ahmed".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            None,
        )
        .unwrap();

        mandate.sign(Utc::now());
        mandate.suspend();
        assert_eq!(mandate.status(), MandateStatus::Suspended);

        mandate.resume();
        assert_eq!(mandate.status(), MandateStatus::Active);
    }

    #[test]
    fn test_mandate_is_expired() {
        let debtor_id = Uuid::new_v4();
        let exp_date = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let mandate = DirectDebitMandate::new(
            debtor_id,
            "Ahmed".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            Some(exp_date),
        )
        .unwrap();

        let before = NaiveDate::from_ymd_opt(2026, 4, 9).unwrap();
        let after = NaiveDate::from_ymd_opt(2026, 4, 11).unwrap();

        assert!(!mandate.is_expired(before));
        assert!(mandate.is_expired(after));
    }

    #[test]
    fn test_mandate_can_debit() {
        let debtor_id = Uuid::new_v4();
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let mut mandate = DirectDebitMandate::new(
            debtor_id,
            "Ahmed".to_string(),
            "CRED-001".to_string(),
            "Company".to_string(),
            Decimal::from(200),
            "TND".to_string(),
            Frequency::Monthly,
            "Reference".to_string(),
            None,
        )
        .unwrap();

        // Before signing
        assert!(!mandate.can_debit(Decimal::from(100), today));

        // After signing
        mandate.sign(Utc::now());
        assert!(mandate.can_debit(Decimal::from(100), today));
        assert!(!mandate.can_debit(Decimal::from(250), today)); // Over limit
    }

    #[test]
    fn test_debit_execution_creation() {
        let mandate_id = Uuid::new_v4();
        let exec = DebitExecution::new(mandate_id, Decimal::from(100));

        assert_eq!(exec.mandate_id(), mandate_id);
        assert_eq!(exec.amount(), Decimal::from(100));
        assert_eq!(exec.status(), DebitExecutionStatus::Pending);
        assert!(exec.reason().is_none());
    }

    #[test]
    fn test_debit_execution_mark_executed() {
        let mut exec = DebitExecution::new(Uuid::new_v4(), Decimal::from(100));
        exec.mark_executed();
        assert_eq!(exec.status(), DebitExecutionStatus::Executed);
    }

    #[test]
    fn test_debit_execution_mark_failed() {
        let mut exec = DebitExecution::new(Uuid::new_v4(), Decimal::from(100));
        exec.mark_failed("Insufficient balance".to_string());
        assert_eq!(exec.status(), DebitExecutionStatus::Failed);
        assert_eq!(exec.reason(), Some("Insufficient balance"));
    }

    #[test]
    fn test_frequency_from_str() {
        assert_eq!(Frequency::from_str("Daily").unwrap(), Frequency::Daily);
        assert_eq!(Frequency::from_str("Monthly").unwrap(), Frequency::Monthly);
        assert!(Frequency::from_str("Unknown").is_err());
    }

    #[test]
    fn test_standing_order_status_from_str() {
        assert_eq!(
            StandingOrderStatus::from_str("Active").unwrap(),
            StandingOrderStatus::Active
        );
        assert_eq!(
            StandingOrderStatus::from_str("Cancelled").unwrap(),
            StandingOrderStatus::Cancelled
        );
        assert!(StandingOrderStatus::from_str("Unknown").is_err());
    }

    #[test]
    fn test_mandate_status_from_str() {
        assert_eq!(
            MandateStatus::from_str("PendingSignature").unwrap(),
            MandateStatus::PendingSignature
        );
        assert_eq!(
            MandateStatus::from_str("Active").unwrap(),
            MandateStatus::Active
        );
        assert!(MandateStatus::from_str("Unknown").is_err());
    }
}
