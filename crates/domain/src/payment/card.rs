use chrono::{DateTime, Datelike, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// CardType Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardType {
    Debit,
    Credit,
    Prepaid,
}

impl CardType {
    pub fn as_str(&self) -> &str {
        match self {
            CardType::Debit => "Debit",
            CardType::Credit => "Credit",
            CardType::Prepaid => "Prepaid",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Debit" => Ok(CardType::Debit),
            "Credit" => Ok(CardType::Credit),
            "Prepaid" => Ok(CardType::Prepaid),
            _ => Err(DomainError::InvalidInput(format!("Unknown card type: {s}"))),
        }
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// CardStatus Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardStatus {
    Issued,
    ActivationPending,
    Active,
    Blocked,
    Suspended,
    Cancelled,
    Expired,
}

impl CardStatus {
    pub fn as_str(&self) -> &str {
        match self {
            CardStatus::Issued => "Issued",
            CardStatus::ActivationPending => "ActivationPending",
            CardStatus::Active => "Active",
            CardStatus::Blocked => "Blocked",
            CardStatus::Suspended => "Suspended",
            CardStatus::Cancelled => "Cancelled",
            CardStatus::Expired => "Expired",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Issued" => Ok(CardStatus::Issued),
            "ActivationPending" => Ok(CardStatus::ActivationPending),
            "Active" => Ok(CardStatus::Active),
            "Blocked" => Ok(CardStatus::Blocked),
            "Suspended" => Ok(CardStatus::Suspended),
            "Cancelled" => Ok(CardStatus::Cancelled),
            "Expired" => Ok(CardStatus::Expired),
            _ => Err(DomainError::InvalidInput(format!("Unknown card status: {s}"))),
        }
    }
}

impl std::fmt::Display for CardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// CardNetwork Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardNetwork {
    Visa,
    Mastercard,
    Local,
}

impl CardNetwork {
    pub fn as_str(&self) -> &str {
        match self {
            CardNetwork::Visa => "Visa",
            CardNetwork::Mastercard => "Mastercard",
            CardNetwork::Local => "Local",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Visa" => Ok(CardNetwork::Visa),
            "Mastercard" => Ok(CardNetwork::Mastercard),
            "Local" => Ok(CardNetwork::Local),
            _ => Err(DomainError::InvalidInput(format!("Unknown card network: {s}"))),
        }
    }
}

impl std::fmt::Display for CardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// TransactionStatus Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionStatus {
    Authorized,
    Captured,
    Declined,
    Reversed,
}

impl TransactionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionStatus::Authorized => "Authorized",
            TransactionStatus::Captured => "Captured",
            TransactionStatus::Declined => "Declined",
            TransactionStatus::Reversed => "Reversed",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Authorized" => Ok(TransactionStatus::Authorized),
            "Captured" => Ok(TransactionStatus::Captured),
            "Declined" => Ok(TransactionStatus::Declined),
            "Reversed" => Ok(TransactionStatus::Reversed),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown transaction status: {s}"
            ))),
        }
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// Card Aggregate
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    id: Uuid,
    account_id: Uuid,
    customer_id: Uuid,
    card_type: CardType,
    network: CardNetwork,
    pan_hash: String,
    masked_pan: String,
    cvv_hash: String,
    expiry_month: u8,
    expiry_year: u16,
    status: CardStatus,
    activation_code_hash: Option<String>,
    daily_limit: Decimal,
    monthly_limit: Decimal,
    daily_spent: Decimal,
    monthly_spent: Decimal,
    is_contactless_enabled: bool,
    created_at: DateTime<Utc>,
    activated_at: Option<DateTime<Utc>>,
    cancelled_at: Option<DateTime<Utc>>,
}

impl Card {
    /// Create a new card with generated mock PAN and CVV hashes
    pub fn new(
        account_id: Uuid,
        customer_id: Uuid,
        card_type: CardType,
        network: CardNetwork,
        validity_years: u8,
    ) -> Self {
        let now = Utc::now();
        let expiry_year = (now.year() as u16) + (validity_years as u16);
        let expiry_month = now.month() as u8;

        // Generate mock PAN (for demo: 411111****1111 format)
        let mock_pan = "4111111111111111";
        let pan_hash = Self::hash_value(mock_pan);
        let masked_pan = "411111****1111".to_string();

        // Generate mock CVV hash
        let mock_cvv = "123";
        let cvv_hash = Self::hash_value(mock_cvv);

        Card {
            id: Uuid::new_v4(),
            account_id,
            customer_id,
            card_type,
            network,
            pan_hash,
            masked_pan,
            cvv_hash,
            expiry_month,
            expiry_year,
            status: CardStatus::Issued,
            activation_code_hash: None,
            daily_limit: Decimal::new(2_000_000, 3), // 2000.000 TND
            monthly_limit: Decimal::new(50_000_000, 3), // 50000.000 TND
            daily_spent: Decimal::ZERO,
            monthly_spent: Decimal::ZERO,
            is_contactless_enabled: true,
            created_at: now,
            activated_at: None,
            cancelled_at: None,
        }
    }

    /// Reconstruct from persistence
    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        id: Uuid,
        account_id: Uuid,
        customer_id: Uuid,
        card_type: CardType,
        network: CardNetwork,
        pan_hash: String,
        masked_pan: String,
        cvv_hash: String,
        expiry_month: u8,
        expiry_year: u16,
        status: CardStatus,
        activation_code_hash: Option<String>,
        daily_limit: Decimal,
        monthly_limit: Decimal,
        daily_spent: Decimal,
        monthly_spent: Decimal,
        is_contactless_enabled: bool,
        created_at: DateTime<Utc>,
        activated_at: Option<DateTime<Utc>>,
        cancelled_at: Option<DateTime<Utc>>,
    ) -> Self {
        Card {
            id,
            account_id,
            customer_id,
            card_type,
            network,
            pan_hash,
            masked_pan,
            cvv_hash,
            expiry_month,
            expiry_year,
            status,
            activation_code_hash,
            daily_limit,
            monthly_limit,
            daily_spent,
            monthly_spent,
            is_contactless_enabled,
            created_at,
            activated_at,
            cancelled_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn card_type(&self) -> CardType {
        self.card_type
    }

    pub fn network(&self) -> CardNetwork {
        self.network
    }

    pub fn pan_hash(&self) -> &str {
        &self.pan_hash
    }

    pub fn masked_pan(&self) -> &str {
        &self.masked_pan
    }

    pub fn expiry_month(&self) -> u8 {
        self.expiry_month
    }

    pub fn expiry_year(&self) -> u16 {
        self.expiry_year
    }

    pub fn status(&self) -> CardStatus {
        self.status
    }

    pub fn daily_limit(&self) -> Decimal {
        self.daily_limit
    }

    pub fn monthly_limit(&self) -> Decimal {
        self.monthly_limit
    }

    pub fn daily_spent(&self) -> Decimal {
        self.daily_spent
    }

    pub fn monthly_spent(&self) -> Decimal {
        self.monthly_spent
    }

    pub fn is_contactless_enabled(&self) -> bool {
        self.is_contactless_enabled
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn activated_at(&self) -> Option<DateTime<Utc>> {
        self.activated_at
    }

    pub fn cancelled_at(&self) -> Option<DateTime<Utc>> {
        self.cancelled_at
    }

    // --- State Transitions ---

    /// Activate the card with an activation code
    pub fn activate(&mut self, code: &str) -> Result<(), DomainError> {
        if self.status != CardStatus::Issued && self.status != CardStatus::ActivationPending {
            return Err(DomainError::InvalidInput(
                "Card can only be activated from Issued or ActivationPending status".to_string(),
            ));
        }

        // For demo: validate code (normally would match against stored hash)
        if code.trim().is_empty() {
            return Err(DomainError::InvalidInput(
                "Activation code cannot be empty".to_string(),
            ));
        }

        self.status = CardStatus::Active;
        self.activated_at = Some(Utc::now());
        Ok(())
    }

    /// Block the card (only from Active status)
    pub fn block(&mut self) -> Result<(), DomainError> {
        if self.status != CardStatus::Active {
            return Err(DomainError::InvalidInput(
                "Card can only be blocked from Active status".to_string(),
            ));
        }
        self.status = CardStatus::Blocked;
        Ok(())
    }

    /// Unblock the card (only from Blocked status)
    pub fn unblock(&mut self) -> Result<(), DomainError> {
        if self.status != CardStatus::Blocked {
            return Err(DomainError::InvalidInput(
                "Card can only be unblocked from Blocked status".to_string(),
            ));
        }
        self.status = CardStatus::Active;
        Ok(())
    }

    /// Cancel the card
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == CardStatus::Cancelled {
            return Err(DomainError::InvalidInput("Card is already cancelled".to_string()));
        }
        self.status = CardStatus::Cancelled;
        self.cancelled_at = Some(Utc::now());
        Ok(())
    }

    /// Suspend the card (only from Active status)
    pub fn suspend(&mut self) -> Result<(), DomainError> {
        if self.status != CardStatus::Active {
            return Err(DomainError::InvalidInput(
                "Card can only be suspended from Active status".to_string(),
            ));
        }
        self.status = CardStatus::Suspended;
        Ok(())
    }

    /// Check if card is active
    pub fn is_active(&self) -> bool {
        self.status == CardStatus::Active
    }

    /// Check if card is expired
    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        let now_month = now.month() as u8;
        let now_year = now.year() as u16;

        if now_year > self.expiry_year {
            return true;
        }

        if now_year == self.expiry_year && now_month > self.expiry_month {
            return true;
        }

        false
    }

    /// Check if card can transact (active, not expired, within limits)
    pub fn can_transact(&self, amount: Decimal, now: DateTime<Utc>) -> Result<(), DomainError> {
        if !self.is_active() {
            return Err(DomainError::InvalidInput(
                "Card is not in Active status".to_string(),
            ));
        }

        if self.is_expired(now) {
            return Err(DomainError::InvalidInput("Card is expired".to_string()));
        }

        if self.daily_spent + amount > self.daily_limit {
            return Err(DomainError::InvalidInput(
                "Daily spending limit exceeded".to_string(),
            ));
        }

        if self.monthly_spent + amount > self.monthly_limit {
            return Err(DomainError::InvalidInput(
                "Monthly spending limit exceeded".to_string(),
            ));
        }

        Ok(())
    }

    /// Record a transaction (increment spending)
    pub fn record_transaction(&mut self, amount: Decimal) {
        self.daily_spent += amount;
        self.monthly_spent += amount;
    }

    /// Reset daily spending (typically called once per day)
    pub fn reset_daily_spending(&mut self) {
        self.daily_spent = Decimal::ZERO;
    }

    /// Reset monthly spending (typically called on first day of month)
    pub fn reset_monthly_spending(&mut self) {
        self.monthly_spent = Decimal::ZERO;
    }

    /// Set daily limit (must be > 0)
    pub fn set_daily_limit(&mut self, limit: Decimal) -> Result<(), DomainError> {
        if limit <= Decimal::ZERO {
            return Err(DomainError::InvalidInput(
                "Daily limit must be greater than 0".to_string(),
            ));
        }
        self.daily_limit = limit;
        Ok(())
    }

    /// Set monthly limit (must be > 0)
    pub fn set_monthly_limit(&mut self, limit: Decimal) -> Result<(), DomainError> {
        if limit <= Decimal::ZERO {
            return Err(DomainError::InvalidInput(
                "Monthly limit must be greater than 0".to_string(),
            ));
        }
        self.monthly_limit = limit;
        Ok(())
    }

    /// Toggle contactless feature
    pub fn set_contactless_enabled(&mut self, enabled: bool) {
        self.is_contactless_enabled = enabled;
    }

    // --- Helper ---

    fn hash_value(value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

// ============================================================
// CardTransaction Entity
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardTransaction {
    id: Uuid,
    card_id: Uuid,
    amount: Decimal,
    currency: String,
    merchant_name: String,
    mcc_code: String,
    status: TransactionStatus,
    auth_code: String,
    timestamp: DateTime<Utc>,
    is_contactless: bool,
    is_online: bool,
}

impl CardTransaction {
    /// Create a new transaction
    pub fn new(
        card_id: Uuid,
        amount: Decimal,
        currency: String,
        merchant_name: String,
        mcc_code: String,
        is_contactless: bool,
        is_online: bool,
    ) -> Self {
        CardTransaction {
            id: Uuid::new_v4(),
            card_id,
            amount,
            currency,
            merchant_name,
            mcc_code,
            status: TransactionStatus::Authorized,
            auth_code: Self::generate_auth_code(),
            timestamp: Utc::now(),
            is_contactless,
            is_online,
        }
    }

    /// Reconstruct from persistence
    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        id: Uuid,
        card_id: Uuid,
        amount: Decimal,
        currency: String,
        merchant_name: String,
        mcc_code: String,
        status: TransactionStatus,
        auth_code: String,
        timestamp: DateTime<Utc>,
        is_contactless: bool,
        is_online: bool,
    ) -> Self {
        CardTransaction {
            id,
            card_id,
            amount,
            currency,
            merchant_name,
            mcc_code,
            status,
            auth_code,
            timestamp,
            is_contactless,
            is_online,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn card_id(&self) -> Uuid {
        self.card_id
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn merchant_name(&self) -> &str {
        &self.merchant_name
    }

    pub fn mcc_code(&self) -> &str {
        &self.mcc_code
    }

    pub fn status(&self) -> TransactionStatus {
        self.status
    }

    pub fn auth_code(&self) -> &str {
        &self.auth_code
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn is_contactless(&self) -> bool {
        self.is_contactless
    }

    pub fn is_online(&self) -> bool {
        self.is_online
    }

    // --- State Transitions ---

    pub fn capture(&mut self) -> Result<(), DomainError> {
        if self.status != TransactionStatus::Authorized {
            return Err(DomainError::InvalidInput(
                "Only authorized transactions can be captured".to_string(),
            ));
        }
        self.status = TransactionStatus::Captured;
        Ok(())
    }

    pub fn decline(&mut self) -> Result<(), DomainError> {
        if !matches!(
            self.status,
            TransactionStatus::Authorized | TransactionStatus::Captured
        ) {
            return Err(DomainError::InvalidInput(
                "Only authorized or captured transactions can be declined".to_string(),
            ));
        }
        self.status = TransactionStatus::Declined;
        Ok(())
    }

    pub fn reverse(&mut self) -> Result<(), DomainError> {
        if self.status != TransactionStatus::Captured {
            return Err(DomainError::InvalidInput(
                "Only captured transactions can be reversed".to_string(),
            ));
        }
        self.status = TransactionStatus::Reversed;
        Ok(())
    }

    // --- Helper ---

    fn generate_auth_code() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("{:06}", (now % 1000000))
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Card Creation Tests ---

    #[test]
    fn test_create_debit_card() {
        let account_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let card = Card::new(account_id, customer_id, CardType::Debit, CardNetwork::Visa, 5);

        assert_eq!(card.account_id(), account_id);
        assert_eq!(card.customer_id(), customer_id);
        assert_eq!(card.card_type(), CardType::Debit);
        assert_eq!(card.network(), CardNetwork::Visa);
        assert_eq!(card.status(), CardStatus::Issued);
        assert!(!card.is_active());
        assert_eq!(card.masked_pan(), "411111****1111");
    }

    #[test]
    fn test_create_credit_card() {
        let card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Credit,
            CardNetwork::Mastercard,
            3,
        );

        assert_eq!(card.card_type(), CardType::Credit);
        assert_eq!(card.network(), CardNetwork::Mastercard);
    }

    #[test]
    fn test_create_prepaid_card() {
        let card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Prepaid,
            CardNetwork::Local,
            2,
        );

        assert_eq!(card.card_type(), CardType::Prepaid);
        assert_eq!(card.network(), CardNetwork::Local);
    }

    #[test]
    fn test_card_default_limits() {
        let card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert_eq!(card.daily_limit(), Decimal::new(2000_000, 3));
        assert_eq!(card.monthly_limit(), Decimal::new(50000_000, 3));
        assert_eq!(card.daily_spent(), Decimal::ZERO);
        assert_eq!(card.monthly_spent(), Decimal::ZERO);
    }

    // --- Activation Tests ---

    #[test]
    fn test_activate_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.activate("123456").is_ok());
        assert_eq!(card.status(), CardStatus::Active);
        assert!(card.is_active());
        assert!(card.activated_at().is_some());
    }

    #[test]
    fn test_activate_with_empty_code_fails() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.activate("").is_err());
        assert!(!card.is_active());
    }

    #[test]
    fn test_cannot_activate_from_active() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        assert!(card.activate("654321").is_err());
    }

    // --- Blocking/Unblocking Tests ---

    #[test]
    fn test_block_and_unblock_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        assert!(card.block().is_ok());
        assert_eq!(card.status(), CardStatus::Blocked);

        assert!(card.unblock().is_ok());
        assert_eq!(card.status(), CardStatus::Active);
    }

    #[test]
    fn test_cannot_block_inactive_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.block().is_err());
    }

    #[test]
    fn test_cannot_unblock_non_blocked_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        assert!(card.unblock().is_err());
    }

    // --- Cancellation Tests ---

    #[test]
    fn test_cancel_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.cancel().is_ok());
        assert_eq!(card.status(), CardStatus::Cancelled);
        assert!(card.cancelled_at().is_some());
    }

    #[test]
    fn test_cannot_cancel_already_cancelled() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.cancel().unwrap();
        assert!(card.cancel().is_err());
    }

    // --- Suspension Tests ---

    #[test]
    fn test_suspend_active_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        assert!(card.suspend().is_ok());
        assert_eq!(card.status(), CardStatus::Suspended);
    }

    #[test]
    fn test_cannot_suspend_inactive_card() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.suspend().is_err());
    }

    // --- Expiry Tests ---

    #[test]
    fn test_card_not_expired_within_validity() {
        let card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        let now = Utc::now();
        assert!(!card.is_expired(now));
    }

    #[test]
    fn test_card_expired_after_validity() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            1,
        );

        // Manually set expiry to past
        card.expiry_year = 2020;
        card.expiry_month = 12;

        let now = Utc::now();
        assert!(card.is_expired(now));
    }

    // --- Transaction Limits Tests ---

    #[test]
    fn test_can_transact_within_daily_limit() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        let amount = Decimal::new(1000_000, 3); // 1000 TND
        let now = Utc::now();

        assert!(card.can_transact(amount, now).is_ok());
    }

    #[test]
    fn test_cannot_transact_exceeding_daily_limit() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        card.daily_spent = Decimal::new(1500_000, 3); // 1500 TND spent

        let amount = Decimal::new(600_000, 3); // 600 TND more would exceed 2000
        let now = Utc::now();

        assert!(card.can_transact(amount, now).is_err());
    }

    #[test]
    fn test_cannot_transact_exceeding_monthly_limit() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        card.monthly_spent = Decimal::new(49000_000, 3); // 49000 TND spent

        let amount = Decimal::new(1500_000, 3); // 1500 TND more would exceed 50000
        let now = Utc::now();

        assert!(card.can_transact(amount, now).is_err());
    }

    #[test]
    fn test_cannot_transact_if_not_active() {
        let card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        let amount = Decimal::new(100_000, 3);
        let now = Utc::now();

        assert!(card.can_transact(amount, now).is_err());
    }

    #[test]
    fn test_cannot_transact_if_expired() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.activate("123456").unwrap();
        card.expiry_year = 2020;

        let amount = Decimal::new(100_000, 3);
        let now = Utc::now();

        assert!(card.can_transact(amount, now).is_err());
    }

    // --- Spending Recording Tests ---

    #[test]
    fn test_record_transaction_updates_spent() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        let amount = Decimal::new(500_000, 3);
        card.record_transaction(amount);

        assert_eq!(card.daily_spent(), amount);
        assert_eq!(card.monthly_spent(), amount);
    }

    #[test]
    fn test_reset_daily_spending() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.record_transaction(Decimal::new(500_000, 3));
        assert_eq!(card.daily_spent(), Decimal::new(500_000, 3));

        card.reset_daily_spending();
        assert_eq!(card.daily_spent(), Decimal::ZERO);
        assert_eq!(card.monthly_spent(), Decimal::new(500_000, 3)); // Monthly not reset
    }

    #[test]
    fn test_reset_monthly_spending() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.record_transaction(Decimal::new(500_000, 3));
        card.reset_monthly_spending();
        assert_eq!(card.monthly_spent(), Decimal::ZERO);
    }

    // --- Limits Tests ---

    #[test]
    fn test_set_daily_limit() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        let new_limit = Decimal::new(3000_000, 3);
        assert!(card.set_daily_limit(new_limit).is_ok());
        assert_eq!(card.daily_limit(), new_limit);
    }

    #[test]
    fn test_set_daily_limit_zero_fails() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.set_daily_limit(Decimal::ZERO).is_err());
    }

    #[test]
    fn test_set_monthly_limit() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        let new_limit = Decimal::new(100000_000, 3);
        assert!(card.set_monthly_limit(new_limit).is_ok());
        assert_eq!(card.monthly_limit(), new_limit);
    }

    // --- Contactless Tests ---

    #[test]
    fn test_contactless_enabled_by_default() {
        let card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        assert!(card.is_contactless_enabled());
    }

    #[test]
    fn test_toggle_contactless() {
        let mut card = Card::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            CardType::Debit,
            CardNetwork::Visa,
            5,
        );

        card.set_contactless_enabled(false);
        assert!(!card.is_contactless_enabled());

        card.set_contactless_enabled(true);
        assert!(card.is_contactless_enabled());
    }

    // --- CardTransaction Tests ---

    #[test]
    fn test_create_transaction() {
        let card_id = Uuid::new_v4();
        let txn = CardTransaction::new(
            card_id,
            Decimal::new(100_000, 3),
            "TND".to_string(),
            "Merchant ABC".to_string(),
            "5411".to_string(),
            false,
            true,
        );

        assert_eq!(txn.card_id(), card_id);
        assert_eq!(txn.amount(), Decimal::new(100_000, 3));
        assert_eq!(txn.currency(), "TND");
        assert_eq!(txn.merchant_name(), "Merchant ABC");
        assert_eq!(txn.status(), TransactionStatus::Authorized);
        assert!(!txn.is_contactless());
        assert!(txn.is_online());
    }

    #[test]
    fn test_transaction_capture() {
        let mut txn = CardTransaction::new(
            Uuid::new_v4(),
            Decimal::new(100_000, 3),
            "TND".to_string(),
            "Merchant".to_string(),
            "5411".to_string(),
            false,
            true,
        );

        assert!(txn.capture().is_ok());
        assert_eq!(txn.status(), TransactionStatus::Captured);
    }

    #[test]
    fn test_transaction_decline() {
        let mut txn = CardTransaction::new(
            Uuid::new_v4(),
            Decimal::new(100_000, 3),
            "TND".to_string(),
            "Merchant".to_string(),
            "5411".to_string(),
            false,
            true,
        );

        assert!(txn.decline().is_ok());
        assert_eq!(txn.status(), TransactionStatus::Declined);
    }

    #[test]
    fn test_transaction_reverse() {
        let mut txn = CardTransaction::new(
            Uuid::new_v4(),
            Decimal::new(100_000, 3),
            "TND".to_string(),
            "Merchant".to_string(),
            "5411".to_string(),
            false,
            true,
        );

        txn.capture().unwrap();
        assert!(txn.reverse().is_ok());
        assert_eq!(txn.status(), TransactionStatus::Reversed);
    }

    // --- Enum Tests ---

    #[test]
    fn test_card_type_from_str() {
        assert_eq!(CardType::from_str_type("Debit").unwrap(), CardType::Debit);
        assert_eq!(CardType::from_str_type("Credit").unwrap(), CardType::Credit);
        assert_eq!(CardType::from_str_type("Prepaid").unwrap(), CardType::Prepaid);
        assert!(CardType::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_card_status_from_str() {
        assert_eq!(
            CardStatus::from_str_type("Active").unwrap(),
            CardStatus::Active
        );
        assert_eq!(
            CardStatus::from_str_type("Blocked").unwrap(),
            CardStatus::Blocked
        );
        assert!(CardStatus::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_card_network_from_str() {
        assert_eq!(
            CardNetwork::from_str_type("Visa").unwrap(),
            CardNetwork::Visa
        );
        assert_eq!(
            CardNetwork::from_str_type("Mastercard").unwrap(),
            CardNetwork::Mastercard
        );
        assert!(CardNetwork::from_str_type("Unknown").is_err());
    }

    #[test]
    fn test_transaction_status_from_str() {
        assert_eq!(
            TransactionStatus::from_str_type("Authorized").unwrap(),
            TransactionStatus::Authorized
        );
        assert_eq!(
            TransactionStatus::from_str_type("Captured").unwrap(),
            TransactionStatus::Captured
        );
        assert!(TransactionStatus::from_str_type("Unknown").is_err());
    }
}
