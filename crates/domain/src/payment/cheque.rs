use chrono::{DateTime, Duration, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChequeType {
    Bearer,
    Crossed,
    NotNegotiable,
}

impl ChequeType {
    pub fn as_str(&self) -> &str {
        match self {
            ChequeType::Bearer => "Bearer",
            ChequeType::Crossed => "Crossed",
            ChequeType::NotNegotiable => "NotNegotiable",
        }
    }

    pub fn description_fr(&self) -> &str {
        match self {
            ChequeType::Bearer => "Chèque au porteur",
            ChequeType::Crossed => "Chèque barré",
            ChequeType::NotNegotiable => "Chèque non négociable",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Bearer" => Ok(ChequeType::Bearer),
            "Crossed" => Ok(ChequeType::Crossed),
            "NotNegotiable" => Ok(ChequeType::NotNegotiable),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown cheque type: {s}"
            ))),
        }
    }
}

impl fmt::Display for ChequeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChequeStatus {
    Issued,
    Presented,
    Encashed,
    Rejected,
    Opposed,
    Cleared,
    Expired,
    Cancelled,
}

impl ChequeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ChequeStatus::Issued => "Issued",
            ChequeStatus::Presented => "Presented",
            ChequeStatus::Encashed => "Encashed",
            ChequeStatus::Rejected => "Rejected",
            ChequeStatus::Opposed => "Opposed",
            ChequeStatus::Cleared => "Cleared",
            ChequeStatus::Expired => "Expired",
            ChequeStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Issued" => Ok(ChequeStatus::Issued),
            "Presented" => Ok(ChequeStatus::Presented),
            "Encashed" => Ok(ChequeStatus::Encashed),
            "Rejected" => Ok(ChequeStatus::Rejected),
            "Opposed" => Ok(ChequeStatus::Opposed),
            "Cleared" => Ok(ChequeStatus::Cleared),
            "Expired" => Ok(ChequeStatus::Expired),
            "Cancelled" => Ok(ChequeStatus::Cancelled),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown cheque status: {s}"
            ))),
        }
    }
}

impl fmt::Display for ChequeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectionReason {
    InsufficientBalance,
    InvalidSignature,
    ExpiredCheque,
    AccountClosed,
    OpposedCheque,
    FormalDefect,
    WritingError,
}

impl RejectionReason {
    pub fn as_str(&self) -> &str {
        match self {
            RejectionReason::InsufficientBalance => "InsufficientBalance",
            RejectionReason::InvalidSignature => "InvalidSignature",
            RejectionReason::ExpiredCheque => "ExpiredCheque",
            RejectionReason::AccountClosed => "AccountClosed",
            RejectionReason::OpposedCheque => "OpposedCheque",
            RejectionReason::FormalDefect => "FormalDefect",
            RejectionReason::WritingError => "WritingError",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "InsufficientBalance" => Ok(RejectionReason::InsufficientBalance),
            "InvalidSignature" => Ok(RejectionReason::InvalidSignature),
            "ExpiredCheque" => Ok(RejectionReason::ExpiredCheque),
            "AccountClosed" => Ok(RejectionReason::AccountClosed),
            "OpposedCheque" => Ok(RejectionReason::OpposedCheque),
            "FormalDefect" => Ok(RejectionReason::FormalDefect),
            "WritingError" => Ok(RejectionReason::WritingError),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown rejection reason: {s}"
            ))),
        }
    }
}

impl fmt::Display for RejectionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClearingStatus {
    Pending,
    Submitted,
    Processed,
    PartiallyRejected,
}

impl ClearingStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ClearingStatus::Pending => "Pending",
            ClearingStatus::Submitted => "Submitted",
            ClearingStatus::Processed => "Processed",
            ClearingStatus::PartiallyRejected => "PartiallyRejected",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Pending" => Ok(ClearingStatus::Pending),
            "Submitted" => Ok(ClearingStatus::Submitted),
            "Processed" => Ok(ClearingStatus::Processed),
            "PartiallyRejected" => Ok(ClearingStatus::PartiallyRejected),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown clearing status: {s}"
            ))),
        }
    }
}

impl fmt::Display for ClearingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// Aggregate Root: Cheque
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cheque {
    id: Uuid,
    cheque_number: String,
    account_id: Uuid,
    drawer_name: String,
    beneficiary_name: String,
    amount: Decimal,
    currency: String,
    cheque_type: ChequeType,
    issue_date: NaiveDate,
    expiry_date: NaiveDate,
    status: ChequeStatus,
    rejection_reason: Option<RejectionReason>,
    opposition_reason: Option<String>,
    encashed_at: Option<DateTime<Utc>>,
    presented_at: Option<DateTime<Utc>>,
    clearing_batch_id: Option<Uuid>,
    created_at: DateTime<Utc>,
}

impl Cheque {
    pub fn new(
        cheque_number: String,
        account_id: Uuid,
        drawer_name: String,
        beneficiary_name: String,
        amount: Decimal,
        cheque_type: ChequeType,
    ) -> Result<Self, String> {
        // Validate amount > 0
        if amount <= Decimal::ZERO {
            return Err("Amount must be greater than 0".to_string());
        }

        // Validate cheque_number is 7 digits
        if cheque_number.len() != 7 || !cheque_number.chars().all(|c| c.is_ascii_digit()) {
            return Err("Cheque number must be exactly 7 digits".to_string());
        }

        // Validate names are not empty
        if drawer_name.trim().is_empty() {
            return Err("Drawer name cannot be empty".to_string());
        }

        if beneficiary_name.trim().is_empty() {
            return Err("Beneficiary name cannot be empty".to_string());
        }

        let today = Utc::now().date_naive();
        // Expiry: issue_date + 8 months (per Tunisian law, loi 2007-37)
        let expiry_date = today + Duration::days(240); // Approximately 8 months

        Ok(Cheque {
            id: Uuid::new_v4(),
            cheque_number,
            account_id,
            drawer_name,
            beneficiary_name,
            amount,
            currency: "TND".to_string(),
            cheque_type,
            issue_date: today,
            expiry_date,
            status: ChequeStatus::Issued,
            rejection_reason: None,
            opposition_reason: None,
            encashed_at: None,
            presented_at: None,
            clearing_batch_id: None,
            created_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        id: Uuid,
        cheque_number: String,
        account_id: Uuid,
        drawer_name: String,
        beneficiary_name: String,
        amount: Decimal,
        currency: String,
        cheque_type: ChequeType,
        issue_date: NaiveDate,
        expiry_date: NaiveDate,
        status: ChequeStatus,
        rejection_reason: Option<RejectionReason>,
        opposition_reason: Option<String>,
        encashed_at: Option<DateTime<Utc>>,
        presented_at: Option<DateTime<Utc>>,
        clearing_batch_id: Option<Uuid>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Cheque {
            id,
            cheque_number,
            account_id,
            drawer_name,
            beneficiary_name,
            amount,
            currency,
            cheque_type,
            issue_date,
            expiry_date,
            status,
            rejection_reason,
            opposition_reason,
            encashed_at,
            presented_at,
            clearing_batch_id,
            created_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn cheque_number(&self) -> &str {
        &self.cheque_number
    }
    pub fn account_id(&self) -> Uuid {
        self.account_id
    }
    pub fn drawer_name(&self) -> &str {
        &self.drawer_name
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
    pub fn cheque_type(&self) -> ChequeType {
        self.cheque_type
    }
    pub fn issue_date(&self) -> NaiveDate {
        self.issue_date
    }
    pub fn expiry_date(&self) -> NaiveDate {
        self.expiry_date
    }
    pub fn status(&self) -> ChequeStatus {
        self.status
    }
    pub fn rejection_reason(&self) -> Option<&RejectionReason> {
        self.rejection_reason.as_ref()
    }
    pub fn opposition_reason(&self) -> Option<&str> {
        self.opposition_reason.as_deref()
    }
    pub fn encashed_at(&self) -> Option<DateTime<Utc>> {
        self.encashed_at
    }
    pub fn presented_at(&self) -> Option<DateTime<Utc>> {
        self.presented_at
    }
    pub fn clearing_batch_id(&self) -> Option<Uuid> {
        self.clearing_batch_id
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // --- State Transitions ---

    pub fn present(&mut self) -> Result<(), String> {
        if self.status != ChequeStatus::Issued {
            return Err(format!(
                "Cannot present cheque from status {}",
                self.status
            ));
        }
        self.status = ChequeStatus::Presented;
        self.presented_at = Some(Utc::now());
        Ok(())
    }

    pub fn encash(&mut self, timestamp: DateTime<Utc>) -> Result<(), String> {
        if self.status != ChequeStatus::Presented {
            return Err(format!(
                "Cannot encash cheque from status {}",
                self.status
            ));
        }
        self.status = ChequeStatus::Encashed;
        self.encashed_at = Some(timestamp);
        Ok(())
    }

    pub fn reject(&mut self, reason: RejectionReason) -> Result<(), String> {
        if self.status != ChequeStatus::Presented {
            return Err(format!(
                "Cannot reject cheque from status {}",
                self.status
            ));
        }
        self.status = ChequeStatus::Rejected;
        self.rejection_reason = Some(reason);
        Ok(())
    }

    pub fn oppose(&mut self, reason: String) -> Result<(), String> {
        if !matches!(
            self.status,
            ChequeStatus::Issued | ChequeStatus::Presented
        ) {
            return Err(format!(
                "Cannot oppose cheque from status {}",
                self.status
            ));
        }
        if reason.trim().is_empty() {
            return Err("Opposition reason cannot be empty".to_string());
        }
        self.status = ChequeStatus::Opposed;
        self.opposition_reason = Some(reason);
        Ok(())
    }

    pub fn clear(&mut self, batch_id: Uuid) -> Result<(), String> {
        if self.status != ChequeStatus::Presented {
            return Err(format!(
                "Cannot clear cheque from status {}",
                self.status
            ));
        }
        self.status = ChequeStatus::Cleared;
        self.clearing_batch_id = Some(batch_id);
        Ok(())
    }

    pub fn is_expired(&self, today: NaiveDate) -> bool {
        today > self.expiry_date
    }

    pub fn can_be_encashed(&self, today: NaiveDate) -> Result<(), String> {
        if self.is_expired(today) {
            return Err("Cheque has expired".to_string());
        }
        if self.status == ChequeStatus::Opposed {
            return Err("Cheque has been opposed".to_string());
        }
        if self.status != ChequeStatus::Presented {
            return Err(format!(
                "Cheque must be Presented to be encashed, current status: {}",
                self.status
            ));
        }
        Ok(())
    }
}

// ============================================================
// ChequeOpposition Entity
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChequeOpposition {
    id: Uuid,
    cheque_id: Uuid,
    account_id: Uuid,
    reason: String,
    is_legal_opposition: bool,
    created_at: DateTime<Utc>,
}

impl ChequeOpposition {
    pub fn new(
        cheque_id: Uuid,
        account_id: Uuid,
        reason: String,
        is_legal_opposition: bool,
    ) -> Result<Self, String> {
        if reason.trim().is_empty() {
            return Err("Opposition reason cannot be empty".to_string());
        }
        Ok(ChequeOpposition {
            id: Uuid::new_v4(),
            cheque_id,
            account_id,
            reason,
            is_legal_opposition,
            created_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: Uuid,
        cheque_id: Uuid,
        account_id: Uuid,
        reason: String,
        is_legal_opposition: bool,
        created_at: DateTime<Utc>,
    ) -> Self {
        ChequeOpposition {
            id,
            cheque_id,
            account_id,
            reason,
            is_legal_opposition,
            created_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn cheque_id(&self) -> Uuid {
        self.cheque_id
    }
    pub fn account_id(&self) -> Uuid {
        self.account_id
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn is_legal_opposition(&self) -> bool {
        self.is_legal_opposition
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// ============================================================
// BankingBlacklist Entity (Interdit Bancaire)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankingBlacklist {
    id: Uuid,
    customer_id: Uuid,
    reason: String,
    blacklisted_at: DateTime<Utc>,
    lifted_at: Option<DateTime<Utc>>,
    rejection_count: u32,
    is_active: bool,
}

impl BankingBlacklist {
    pub fn new(customer_id: Uuid, reason: String, rejection_count: u32) -> Self {
        BankingBlacklist {
            id: Uuid::new_v4(),
            customer_id,
            reason,
            blacklisted_at: Utc::now(),
            lifted_at: None,
            rejection_count,
            is_active: true,
        }
    }

    pub fn from_raw(
        id: Uuid,
        customer_id: Uuid,
        reason: String,
        blacklisted_at: DateTime<Utc>,
        lifted_at: Option<DateTime<Utc>>,
        rejection_count: u32,
        is_active: bool,
    ) -> Self {
        BankingBlacklist {
            id,
            customer_id,
            reason,
            blacklisted_at,
            lifted_at,
            rejection_count,
            is_active,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn blacklisted_at(&self) -> DateTime<Utc> {
        self.blacklisted_at
    }
    pub fn lifted_at(&self) -> Option<DateTime<Utc>> {
        self.lifted_at
    }
    pub fn rejection_count(&self) -> u32 {
        self.rejection_count
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn lift(&mut self, timestamp: DateTime<Utc>) -> Result<(), String> {
        if !self.is_active {
            return Err("Blacklist is not active".to_string());
        }
        self.is_active = false;
        self.lifted_at = Some(timestamp);
        Ok(())
    }

    pub fn duration_months(&self, now: DateTime<Utc>) -> u32 {
        let end = self.lifted_at.unwrap_or(now);
        let duration = end.signed_duration_since(self.blacklisted_at);
        (duration.num_days() / 30) as u32
    }
}

// ============================================================
// ChequeClearing & ClearingResult
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearingResult {
    pub cheque_id: Uuid,
    pub status: ClearingStatus,
    pub rejection_code: Option<String>,
}

impl ClearingResult {
    pub fn new(cheque_id: Uuid, status: ClearingStatus) -> Self {
        ClearingResult {
            cheque_id,
            status,
            rejection_code: None,
        }
    }

    pub fn with_rejection_code(mut self, code: String) -> Self {
        self.rejection_code = Some(code);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChequeClearing {
    id: Uuid,
    clearing_date: NaiveDate,
    cheques: Vec<Uuid>,
    total_amount: Decimal,
    status: ClearingStatus,
    submitted_at: Option<DateTime<Utc>>,
    processed_at: Option<DateTime<Utc>>,
    results: Vec<ClearingResult>,
    created_at: DateTime<Utc>,
}

impl ChequeClearing {
    pub fn new(clearing_date: NaiveDate) -> Self {
        ChequeClearing {
            id: Uuid::new_v4(),
            clearing_date,
            cheques: Vec::new(),
            total_amount: Decimal::ZERO,
            status: ClearingStatus::Pending,
            submitted_at: None,
            processed_at: None,
            results: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn from_raw(
        id: Uuid,
        clearing_date: NaiveDate,
        cheques: Vec<Uuid>,
        total_amount: Decimal,
        status: ClearingStatus,
        submitted_at: Option<DateTime<Utc>>,
        processed_at: Option<DateTime<Utc>>,
        results: Vec<ClearingResult>,
        created_at: DateTime<Utc>,
    ) -> Self {
        ChequeClearing {
            id,
            clearing_date,
            cheques,
            total_amount,
            status,
            submitted_at,
            processed_at,
            results,
            created_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn clearing_date(&self) -> NaiveDate {
        self.clearing_date
    }
    pub fn cheques(&self) -> &[Uuid] {
        &self.cheques
    }
    pub fn total_amount(&self) -> Decimal {
        self.total_amount
    }
    pub fn status(&self) -> ClearingStatus {
        self.status
    }
    pub fn submitted_at(&self) -> Option<DateTime<Utc>> {
        self.submitted_at
    }
    pub fn processed_at(&self) -> Option<DateTime<Utc>> {
        self.processed_at
    }
    pub fn results(&self) -> &[ClearingResult] {
        &self.results
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn add_cheque(&mut self, cheque_id: Uuid, amount: Decimal) {
        self.cheques.push(cheque_id);
        self.total_amount += amount;
    }

    pub fn submit(&mut self) -> Result<(), String> {
        if self.status != ClearingStatus::Pending {
            return Err(format!(
                "Cannot submit clearing from status {}",
                self.status
            ));
        }
        if self.cheques.is_empty() {
            return Err("Cannot submit empty clearing batch".to_string());
        }
        self.status = ClearingStatus::Submitted;
        self.submitted_at = Some(Utc::now());
        Ok(())
    }

    pub fn process(&mut self, results: Vec<ClearingResult>) -> Result<(), String> {
        if self.status != ClearingStatus::Submitted {
            return Err(format!(
                "Cannot process clearing from status {}",
                self.status
            ));
        }
        if results.is_empty() {
            return Err("Cannot process clearing with no results".to_string());
        }

        let all_cleared = results.iter().all(|r| r.status == ClearingStatus::Processed);
        self.status = if all_cleared {
            ClearingStatus::Processed
        } else {
            ClearingStatus::PartiallyRejected
        };
        self.results = results;
        self.processed_at = Some(Utc::now());
        Ok(())
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    // --- Cheque Creation Tests ---

    #[test]
    fn test_create_cheque() {
        let cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed Ben Ali".to_string(),
            "Mohamed Trabelsi".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        assert_eq!(cheque.cheque_number(), "1234567");
        assert_eq!(cheque.drawer_name(), "Ahmed Ben Ali");
        assert_eq!(cheque.beneficiary_name(), "Mohamed Trabelsi");
        assert_eq!(cheque.amount(), dec!(5000));
        assert_eq!(cheque.status(), ChequeStatus::Issued);
        assert_eq!(cheque.currency(), "TND");
        assert_eq!(cheque.cheque_type(), ChequeType::Bearer);
    }

    #[test]
    fn test_cheque_invalid_amount_zero() {
        let result = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(0),
            ChequeType::Bearer,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_invalid_amount_negative() {
        let result = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(-100),
            ChequeType::Bearer,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_invalid_cheque_number_not_digits() {
        let result = Cheque::new(
            "ABC1234".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_invalid_cheque_number_wrong_length() {
        let result = Cheque::new(
            "123456".to_string(), // 6 digits instead of 7
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_invalid_empty_drawer_name() {
        let result = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_invalid_empty_beneficiary_name() {
        let result = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        );
        assert!(result.is_err());
    }

    // --- Status Transition Tests ---

    #[test]
    fn test_cheque_present() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        assert!(cheque.present().is_ok());
        assert_eq!(cheque.status(), ChequeStatus::Presented);
        assert!(cheque.presented_at().is_some());
    }

    #[test]
    fn test_cheque_encash() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.present().unwrap();
        let now = Utc::now();
        assert!(cheque.encash(now).is_ok());
        assert_eq!(cheque.status(), ChequeStatus::Encashed);
        assert!(cheque.encashed_at().is_some());
    }

    #[test]
    fn test_cheque_cannot_encash_from_issued() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        let result = cheque.encash(Utc::now());
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_reject() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.present().unwrap();
        assert!(cheque.reject(RejectionReason::InsufficientBalance).is_ok());
        assert_eq!(cheque.status(), ChequeStatus::Rejected);
        assert_eq!(
            cheque.rejection_reason(),
            Some(&RejectionReason::InsufficientBalance)
        );
    }

    #[test]
    fn test_cheque_oppose() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        assert!(cheque
            .oppose("Cheque was stolen".to_string())
            .is_ok());
        assert_eq!(cheque.status(), ChequeStatus::Opposed);
        assert_eq!(cheque.opposition_reason(), Some("Cheque was stolen"));
    }

    #[test]
    fn test_cheque_oppose_from_presented() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.present().unwrap();
        assert!(cheque.oppose("Stop payment".to_string()).is_ok());
        assert_eq!(cheque.status(), ChequeStatus::Opposed);
    }

    #[test]
    fn test_cheque_oppose_empty_reason_fails() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        let result = cheque.oppose("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_clear() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.present().unwrap();
        let batch_id = Uuid::new_v4();
        assert!(cheque.clear(batch_id).is_ok());
        assert_eq!(cheque.status(), ChequeStatus::Cleared);
        assert_eq!(cheque.clearing_batch_id(), Some(batch_id));
    }

    #[test]
    fn test_cheque_is_expired() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        let today = Utc::now().date_naive();
        assert!(!cheque.is_expired(today));

        // Check after expiry
        let after_expiry = cheque.expiry_date() + Duration::days(1);
        assert!(cheque.is_expired(after_expiry));
    }

    #[test]
    fn test_cheque_can_be_encashed_valid() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.present().unwrap();
        let today = Utc::now().date_naive();
        assert!(cheque.can_be_encashed(today).is_ok());
    }

    #[test]
    fn test_cheque_can_be_encashed_expired() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.present().unwrap();
        let after_expiry = cheque.expiry_date() + Duration::days(1);
        let result = cheque.can_be_encashed(after_expiry);
        assert!(result.is_err());
    }

    #[test]
    fn test_cheque_can_be_encashed_opposed() {
        let mut cheque = Cheque::new(
            "1234567".to_string(),
            Uuid::new_v4(),
            "Ahmed".to_string(),
            "Mohamed".to_string(),
            dec!(5000),
            ChequeType::Bearer,
        )
        .unwrap();

        cheque.oppose("Stop payment".to_string()).unwrap();
        let today = Utc::now().date_naive();
        let result = cheque.can_be_encashed(today);
        assert!(result.is_err());
    }

    // --- ChequeOpposition Tests ---

    #[test]
    fn test_create_cheque_opposition() {
        let cheque_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let opposition =
            ChequeOpposition::new(cheque_id, account_id, "Stolen cheque".to_string(), false)
                .unwrap();

        assert_eq!(opposition.cheque_id(), cheque_id);
        assert_eq!(opposition.account_id(), account_id);
        assert_eq!(opposition.reason(), "Stolen cheque");
        assert!(!opposition.is_legal_opposition());
    }

    #[test]
    fn test_cheque_opposition_legal() {
        let opposition =
            ChequeOpposition::new(Uuid::new_v4(), Uuid::new_v4(), "Court order".to_string(), true)
                .unwrap();
        assert!(opposition.is_legal_opposition());
    }

    // --- BankingBlacklist Tests ---

    #[test]
    fn test_create_banking_blacklist() {
        let customer_id = Uuid::new_v4();
        let blacklist =
            BankingBlacklist::new(customer_id, "3 rejections in 30 days".to_string(), 3);

        assert_eq!(blacklist.customer_id(), customer_id);
        assert_eq!(blacklist.reason(), "3 rejections in 30 days");
        assert_eq!(blacklist.rejection_count(), 3);
        assert!(blacklist.is_active());
    }

    #[test]
    fn test_banking_blacklist_lift() {
        let mut blacklist = BankingBlacklist::new(
            Uuid::new_v4(),
            "3 rejections in 30 days".to_string(),
            3,
        );

        assert!(blacklist.is_active());
        let now = Utc::now();
        assert!(blacklist.lift(now).is_ok());
        assert!(!blacklist.is_active());
        assert!(blacklist.lifted_at().is_some());
    }

    #[test]
    fn test_banking_blacklist_cannot_lift_twice() {
        let mut blacklist = BankingBlacklist::new(
            Uuid::new_v4(),
            "3 rejections in 30 days".to_string(),
            3,
        );

        let now = Utc::now();
        blacklist.lift(now).unwrap();
        let result = blacklist.lift(now);
        assert!(result.is_err());
    }

    #[test]
    fn test_banking_blacklist_duration_months() {
        let mut blacklist = BankingBlacklist::new(
            Uuid::new_v4(),
            "3 rejections in 30 days".to_string(),
            3,
        );

        let now = Utc::now();
        let later = now + Duration::days(90);
        blacklist.lift(later).unwrap();
        let duration = blacklist.duration_months(now);
        assert!(duration >= 2 && duration <= 3);
    }

    // --- ChequeClearing Tests ---

    #[test]
    fn test_create_clearing_batch() {
        let date = Utc::now().date_naive();
        let clearing = ChequeClearing::new(date);

        assert_eq!(clearing.clearing_date(), date);
        assert_eq!(clearing.cheques().len(), 0);
        assert_eq!(clearing.total_amount(), Decimal::ZERO);
        assert_eq!(clearing.status(), ClearingStatus::Pending);
    }

    #[test]
    fn test_clearing_add_cheques() {
        let date = Utc::now().date_naive();
        let mut clearing = ChequeClearing::new(date);

        let cheque_id_1 = Uuid::new_v4();
        let cheque_id_2 = Uuid::new_v4();

        clearing.add_cheque(cheque_id_1, dec!(5000));
        clearing.add_cheque(cheque_id_2, dec!(3000));

        assert_eq!(clearing.cheques().len(), 2);
        assert_eq!(clearing.total_amount(), dec!(8000));
    }

    #[test]
    fn test_clearing_submit() {
        let date = Utc::now().date_naive();
        let mut clearing = ChequeClearing::new(date);
        clearing.add_cheque(Uuid::new_v4(), dec!(5000));

        assert!(clearing.submit().is_ok());
        assert_eq!(clearing.status(), ClearingStatus::Submitted);
        assert!(clearing.submitted_at().is_some());
    }

    #[test]
    fn test_clearing_cannot_submit_empty() {
        let date = Utc::now().date_naive();
        let mut clearing = ChequeClearing::new(date);

        let result = clearing.submit();
        assert!(result.is_err());
    }

    #[test]
    fn test_clearing_process() {
        let date = Utc::now().date_naive();
        let mut clearing = ChequeClearing::new(date);
        let cheque_id = Uuid::new_v4();
        clearing.add_cheque(cheque_id, dec!(5000));
        clearing.submit().unwrap();

        let result = ClearingResult::new(cheque_id, ClearingStatus::Processed);
        assert!(clearing.process(vec![result]).is_ok());
        assert_eq!(clearing.status(), ClearingStatus::Processed);
        assert!(clearing.processed_at().is_some());
    }

    #[test]
    fn test_clearing_process_partially_rejected() {
        let date = Utc::now().date_naive();
        let mut clearing = ChequeClearing::new(date);
        let cheque_id_1 = Uuid::new_v4();
        let cheque_id_2 = Uuid::new_v4();
        clearing.add_cheque(cheque_id_1, dec!(5000));
        clearing.add_cheque(cheque_id_2, dec!(3000));
        clearing.submit().unwrap();

        let results = vec![
            ClearingResult::new(cheque_id_1, ClearingStatus::Processed),
            ClearingResult::new(cheque_id_2, ClearingStatus::PartiallyRejected),
        ];
        assert!(clearing.process(results).is_ok());
        assert_eq!(clearing.status(), ClearingStatus::PartiallyRejected);
    }

    // --- Enum Tests ---

    #[test]
    fn test_cheque_type_display() {
        assert_eq!(ChequeType::Bearer.as_str(), "Bearer");
        assert_eq!(ChequeType::Crossed.as_str(), "Crossed");
        assert_eq!(ChequeType::NotNegotiable.as_str(), "NotNegotiable");
    }

    #[test]
    fn test_cheque_type_description_fr() {
        assert_eq!(ChequeType::Bearer.description_fr(), "Chèque au porteur");
        assert_eq!(ChequeType::Crossed.description_fr(), "Chèque barré");
        assert_eq!(
            ChequeType::NotNegotiable.description_fr(),
            "Chèque non négociable"
        );
    }

    #[test]
    fn test_cheque_status_from_str() {
        assert_eq!(
            ChequeStatus::from_str_type("Issued").unwrap(),
            ChequeStatus::Issued
        );
        assert_eq!(
            ChequeStatus::from_str_type("Presented").unwrap(),
            ChequeStatus::Presented
        );
        assert!(ChequeStatus::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_rejection_reason_from_str() {
        assert_eq!(
            RejectionReason::from_str_type("InsufficientBalance").unwrap(),
            RejectionReason::InsufficientBalance
        );
        assert!(RejectionReason::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_clearing_status_from_str() {
        assert_eq!(
            ClearingStatus::from_str_type("Pending").unwrap(),
            ClearingStatus::Pending
        );
        assert_eq!(
            ClearingStatus::from_str_type("Processed").unwrap(),
            ClearingStatus::Processed
        );
    }
}
