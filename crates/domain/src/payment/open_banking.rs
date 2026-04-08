use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// TASK 2: BC9 PAYMENT OPEN BANKING (FR-113 to FR-116)
// Open Banking APIs, Instant Payments, QR codes, PIS
// ============================================================

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentConsentId(Uuid);

impl PaymentConsentId {
    pub fn new() -> Self {
        PaymentConsentId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        PaymentConsentId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PaymentConsentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PaymentConsentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstantPaymentId(Uuid);

impl InstantPaymentId {
    pub fn new() -> Self {
        InstantPaymentId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        InstantPaymentId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InstantPaymentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InstantPaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QrPaymentCodeId(Uuid);

impl QrPaymentCodeId {
    pub fn new() -> Self {
        QrPaymentCodeId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        QrPaymentCodeId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for QrPaymentCodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QrPaymentCodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThirdPartyPisId(Uuid);

impl ThirdPartyPisId {
    pub fn new() -> Self {
        ThirdPartyPisId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        ThirdPartyPisId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ThirdPartyPisId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ThirdPartyPisId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentStatus {
    Pending,
    Authorised,
    AwaitingAuthorisation,
    Rejected,
    Revoked,
    Expired,
}

impl ConsentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ConsentStatus::Pending => "Pending",
            ConsentStatus::Authorised => "Authorised",
            ConsentStatus::AwaitingAuthorisation => "AwaitingAuthorisation",
            ConsentStatus::Rejected => "Rejected",
            ConsentStatus::Revoked => "Revoked",
            ConsentStatus::Expired => "Expired",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Pending" => Ok(ConsentStatus::Pending),
            "Authorised" => Ok(ConsentStatus::Authorised),
            "AwaitingAuthorisation" => Ok(ConsentStatus::AwaitingAuthorisation),
            "Rejected" => Ok(ConsentStatus::Rejected),
            "Revoked" => Ok(ConsentStatus::Revoked),
            "Expired" => Ok(ConsentStatus::Expired),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown consent status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstantPaymentStatus {
    Initiated,
    Processing,
    Settled,
    Failed,
    Cancelled,
    Rejected,
}

impl InstantPaymentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            InstantPaymentStatus::Initiated => "Initiated",
            InstantPaymentStatus::Processing => "Processing",
            InstantPaymentStatus::Settled => "Settled",
            InstantPaymentStatus::Failed => "Failed",
            InstantPaymentStatus::Cancelled => "Cancelled",
            InstantPaymentStatus::Rejected => "Rejected",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Initiated" => Ok(InstantPaymentStatus::Initiated),
            "Processing" => Ok(InstantPaymentStatus::Processing),
            "Settled" => Ok(InstantPaymentStatus::Settled),
            "Failed" => Ok(InstantPaymentStatus::Failed),
            "Cancelled" => Ok(InstantPaymentStatus::Cancelled),
            "Rejected" => Ok(InstantPaymentStatus::Rejected),
            _ => Err(DomainError::InvalidPaymentOrder(format!(
                "Unknown instant payment status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentFrequency {
    OneOff,
    Recurring,
}

impl ConsentFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            ConsentFrequency::OneOff => "OneOff",
            ConsentFrequency::Recurring => "Recurring",
        }
    }
}

// --- FR-113: Payment Consent (PSD3-ready) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentConsent {
    id: PaymentConsentId,
    customer_id: Uuid,
    from_account_id: Uuid,
    to_iban: String,
    to_bic: Option<String>,
    amount: i64,
    currency: String,
    frequency: ConsentFrequency,
    status: ConsentStatus,
    /// Consent valid from
    valid_from: DateTime<Utc>,
    /// Consent valid until
    valid_until: DateTime<Utc>,
    /// PSD3 Third-party consent identifier
    third_party_id: Option<Uuid>,
    /// Whether this is for third-party initiated payments (PIS)
    third_party_pis: bool,
    created_at: DateTime<Utc>,
}

impl PaymentConsent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        customer_id: Uuid,
        from_account_id: Uuid,
        to_iban: String,
        to_bic: Option<String>,
        amount: i64,
        currency: String,
        frequency: ConsentFrequency,
        valid_from: DateTime<Utc>,
        valid_until: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if amount <= 0 {
            return Err(DomainError::InvalidPaymentOrder(
                "Payment amount must be positive".to_string(),
            ));
        }
        if to_iban.is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "Recipient IBAN required".to_string(),
            ));
        }
        if valid_until <= valid_from {
            return Err(DomainError::InvalidPaymentOrder(
                "Validity period invalid".to_string(),
            ));
        }

        Ok(PaymentConsent {
            id: PaymentConsentId::new(),
            customer_id,
            from_account_id,
            to_iban,
            to_bic,
            amount,
            currency,
            frequency,
            status: ConsentStatus::Pending,
            valid_from,
            valid_until,
            third_party_id: None,
            third_party_pis: false,
            created_at: Utc::now(),
        })
    }

    pub fn authorise(&mut self) {
        self.status = ConsentStatus::Authorised;
    }

    pub fn reject(&mut self) {
        self.status = ConsentStatus::Rejected;
    }

    pub fn revoke(&mut self) {
        self.status = ConsentStatus::Revoked;
    }

    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        self.status == ConsentStatus::Authorised
            && now >= self.valid_from
            && now <= self.valid_until
    }

    pub fn enable_third_party_pis(&mut self, third_party_id: Uuid) {
        self.third_party_id = Some(third_party_id);
        self.third_party_pis = true;
    }

    // Getters
    pub fn id(&self) -> &PaymentConsentId {
        &self.id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn from_account_id(&self) -> Uuid {
        self.from_account_id
    }
    pub fn to_iban(&self) -> &str {
        &self.to_iban
    }
    pub fn to_bic(&self) -> Option<&str> {
        self.to_bic.as_deref()
    }
    pub fn amount(&self) -> i64 {
        self.amount
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn frequency(&self) -> ConsentFrequency {
        self.frequency
    }
    pub fn status(&self) -> ConsentStatus {
        self.status
    }
    pub fn valid_from(&self) -> DateTime<Utc> {
        self.valid_from
    }
    pub fn valid_until(&self) -> DateTime<Utc> {
        self.valid_until
    }
    pub fn third_party_id(&self) -> Option<Uuid> {
        self.third_party_id
    }
    pub fn third_party_pis(&self) -> bool {
        self.third_party_pis
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- FR-114: Instant Payment (TIPS-like, <10 seconds) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantPayment {
    id: InstantPaymentId,
    customer_id: Uuid,
    from_account_id: Uuid,
    to_iban: String,
    to_bic: String,
    amount: i64,
    currency: String,
    status: InstantPaymentStatus,
    reference: String,
    /// Target settlement time (must be <10 seconds)
    settlement_deadline: DateTime<Utc>,
    created_at: DateTime<Utc>,
    settled_at: Option<DateTime<Utc>>,
}

impl InstantPayment {
    /// Target settlement: <10 seconds per TIPS standard (FR-114)
    pub const MAX_SETTLEMENT_SECONDS: i64 = 10;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        customer_id: Uuid,
        from_account_id: Uuid,
        to_iban: String,
        to_bic: String,
        amount: i64,
        currency: String,
        reference: String,
    ) -> Result<Self, DomainError> {
        if amount <= 0 {
            return Err(DomainError::InvalidPaymentOrder(
                "Amount must be positive".to_string(),
            ));
        }
        if to_iban.is_empty() || to_bic.is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "IBAN and BIC required".to_string(),
            ));
        }

        let now = Utc::now();
        let settlement_deadline = now + chrono::Duration::seconds(Self::MAX_SETTLEMENT_SECONDS);

        Ok(InstantPayment {
            id: InstantPaymentId::new(),
            customer_id,
            from_account_id,
            to_iban,
            to_bic,
            amount,
            currency,
            status: InstantPaymentStatus::Initiated,
            reference,
            settlement_deadline,
            created_at: now,
            settled_at: None,
        })
    }

    pub fn mark_processing(&mut self) {
        if self.status == InstantPaymentStatus::Initiated {
            self.status = InstantPaymentStatus::Processing;
        }
    }

    pub fn mark_settled(&mut self) {
        self.status = InstantPaymentStatus::Settled;
        self.settled_at = Some(Utc::now());
    }

    pub fn mark_failed(&mut self) {
        self.status = InstantPaymentStatus::Failed;
    }

    /// Check if payment is still within settlement window
    pub fn is_within_settlement_window(&self) -> bool {
        Utc::now() <= self.settlement_deadline
    }

    // Getters
    pub fn id(&self) -> &InstantPaymentId {
        &self.id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn from_account_id(&self) -> Uuid {
        self.from_account_id
    }
    pub fn to_iban(&self) -> &str {
        &self.to_iban
    }
    pub fn to_bic(&self) -> &str {
        &self.to_bic
    }
    pub fn amount(&self) -> i64 {
        self.amount
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn status(&self) -> InstantPaymentStatus {
        self.status
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }
    pub fn settlement_deadline(&self) -> DateTime<Utc> {
        self.settlement_deadline
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn settled_at(&self) -> Option<DateTime<Utc>> {
        self.settled_at
    }
}

// --- FR-115: QR Code Payment ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QrPaymentCode {
    id: QrPaymentCodeId,
    merchant_id: Uuid,
    amount: i64,
    currency: String,
    reference: String,
    qr_data: String, // EMV QR code payload
    valid_from: DateTime<Utc>,
    valid_until: DateTime<Utc>,
    created_at: DateTime<Utc>,
    scanned_count: i64,
    paid: bool,
}

impl QrPaymentCode {
    pub fn new(
        merchant_id: Uuid,
        amount: i64,
        currency: String,
        reference: String,
        qr_data: String,
        valid_until: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if amount <= 0 {
            return Err(DomainError::InvalidPaymentOrder(
                "Amount must be positive".to_string(),
            ));
        }
        if qr_data.is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "QR data required".to_string(),
            ));
        }

        let now = Utc::now();
        if valid_until <= now {
            return Err(DomainError::InvalidPaymentOrder(
                "Expiry must be in future".to_string(),
            ));
        }

        Ok(QrPaymentCode {
            id: QrPaymentCodeId::new(),
            merchant_id,
            amount,
            currency,
            reference,
            qr_data,
            valid_from: now,
            valid_until,
            created_at: now,
            scanned_count: 0,
            paid: false,
        })
    }

    pub fn mark_paid(&mut self) {
        self.paid = true;
    }

    pub fn increment_scan_count(&mut self) {
        self.scanned_count += 1;
    }

    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        !self.paid && now >= self.valid_from && now <= self.valid_until
    }

    // Getters
    pub fn id(&self) -> &QrPaymentCodeId {
        &self.id
    }
    pub fn merchant_id(&self) -> Uuid {
        self.merchant_id
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
    pub fn qr_data(&self) -> &str {
        &self.qr_data
    }
    pub fn valid_from(&self) -> DateTime<Utc> {
        self.valid_from
    }
    pub fn valid_until(&self) -> DateTime<Utc> {
        self.valid_until
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn scanned_count(&self) -> i64 {
        self.scanned_count
    }
    pub fn paid(&self) -> bool {
        self.paid
    }
}

// --- FR-116: Third-Party Payment Initiation Service (PIS) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThirdPartyPis {
    id: ThirdPartyPisId,
    pis_provider_id: Uuid,
    pis_provider_name: String,
    customer_id: Uuid,
    payment_consent_id: PaymentConsentId,
    from_account_id: Uuid,
    to_iban: String,
    amount: i64,
    currency: String,
    reference: String,
    authorized: bool,
    authorized_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl ThirdPartyPis {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pis_provider_id: Uuid,
        pis_provider_name: String,
        customer_id: Uuid,
        payment_consent_id: PaymentConsentId,
        from_account_id: Uuid,
        to_iban: String,
        amount: i64,
        currency: String,
        reference: String,
    ) -> Result<Self, DomainError> {
        if pis_provider_name.is_empty() {
            return Err(DomainError::InvalidPaymentOrder(
                "PIS provider name required".to_string(),
            ));
        }
        if amount <= 0 {
            return Err(DomainError::InvalidPaymentOrder(
                "Amount must be positive".to_string(),
            ));
        }

        Ok(ThirdPartyPis {
            id: ThirdPartyPisId::new(),
            pis_provider_id,
            pis_provider_name,
            customer_id,
            payment_consent_id,
            from_account_id,
            to_iban,
            amount,
            currency,
            reference,
            authorized: false,
            authorized_at: None,
            created_at: Utc::now(),
        })
    }

    pub fn authorize(&mut self) {
        if !self.authorized {
            self.authorized = true;
            self.authorized_at = Some(Utc::now());
        }
    }

    // Getters
    pub fn id(&self) -> &ThirdPartyPisId {
        &self.id
    }
    pub fn pis_provider_id(&self) -> Uuid {
        self.pis_provider_id
    }
    pub fn pis_provider_name(&self) -> &str {
        &self.pis_provider_name
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn payment_consent_id(&self) -> &PaymentConsentId {
        &self.payment_consent_id
    }
    pub fn from_account_id(&self) -> Uuid {
        self.from_account_id
    }
    pub fn to_iban(&self) -> &str {
        &self.to_iban
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
    pub fn authorized(&self) -> bool {
        self.authorized
    }
    pub fn authorized_at(&self) -> Option<DateTime<Utc>> {
        self.authorized_at
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_customer() -> Uuid {
        Uuid::new_v4()
    }

    fn default_account() -> Uuid {
        Uuid::new_v4()
    }

    fn default_iban() -> String {
        "TN5910000010000456606".to_string()
    }

    // --- FR-113: Payment Consent ---

    #[test]
    fn test_consent_creation() {
        let consent = PaymentConsent::new(
            default_customer(),
            default_account(),
            default_iban(),
            Some("BKTN".to_string()),
            100_000,
            "TND".to_string(),
            ConsentFrequency::OneOff,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        )
        .unwrap();

        assert_eq!(consent.status(), ConsentStatus::Pending);
        assert!(!consent.is_valid());
    }

    #[test]
    fn test_consent_authorize_and_validate() {
        let mut consent = PaymentConsent::new(
            default_customer(),
            default_account(),
            default_iban(),
            Some("BKTN".to_string()),
            100_000,
            "TND".to_string(),
            ConsentFrequency::OneOff,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        )
        .unwrap();

        consent.authorise();
        assert!(consent.is_valid());
        assert_eq!(consent.status(), ConsentStatus::Authorised);
    }

    #[test]
    fn test_consent_revoke() {
        let mut consent = PaymentConsent::new(
            default_customer(),
            default_account(),
            default_iban(),
            Some("BKTN".to_string()),
            100_000,
            "TND".to_string(),
            ConsentFrequency::Recurring,
            Utc::now(),
            Utc::now() + chrono::Duration::days(365),
        )
        .unwrap();

        consent.authorise();
        assert!(consent.is_valid());

        consent.revoke();
        assert!(!consent.is_valid());
        assert_eq!(consent.status(), ConsentStatus::Revoked);
    }

    #[test]
    fn test_consent_third_party_pis() {
        let mut consent = PaymentConsent::new(
            default_customer(),
            default_account(),
            default_iban(),
            Some("BKTN".to_string()),
            100_000,
            "TND".to_string(),
            ConsentFrequency::OneOff,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        )
        .unwrap();

        let pis_id = Uuid::new_v4();
        consent.enable_third_party_pis(pis_id);

        assert!(consent.third_party_pis());
        assert_eq!(consent.third_party_id(), Some(pis_id));
    }

    #[test]
    fn test_consent_invalid_amount() {
        let result = PaymentConsent::new(
            default_customer(),
            default_account(),
            default_iban(),
            None,
            0, // Invalid
            "TND".to_string(),
            ConsentFrequency::OneOff,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_consent_invalid_validity() {
        let now = Utc::now();
        let result = PaymentConsent::new(
            default_customer(),
            default_account(),
            default_iban(),
            None,
            100_000,
            "TND".to_string(),
            ConsentFrequency::OneOff,
            now,
            now - chrono::Duration::days(1), // In the past
        );
        assert!(result.is_err());
    }

    // --- FR-114: Instant Payment ---

    #[test]
    fn test_instant_payment_creation() {
        let payment = InstantPayment::new(
            default_customer(),
            default_account(),
            default_iban(),
            "BKTN".to_string(),
            50_000,
            "TND".to_string(),
            "INV-001".to_string(),
        )
        .unwrap();

        assert_eq!(payment.status(), InstantPaymentStatus::Initiated);
        assert!(payment.is_within_settlement_window());
    }

    #[test]
    fn test_instant_payment_settlement_flow() {
        let mut payment = InstantPayment::new(
            default_customer(),
            default_account(),
            default_iban(),
            "BKTN".to_string(),
            50_000,
            "TND".to_string(),
            "INV-001".to_string(),
        )
        .unwrap();

        payment.mark_processing();
        assert_eq!(payment.status(), InstantPaymentStatus::Processing);

        payment.mark_settled();
        assert_eq!(payment.status(), InstantPaymentStatus::Settled);
        assert!(payment.settled_at().is_some());
    }

    #[test]
    fn test_instant_payment_max_settlement_seconds() {
        // Validate constant
        assert_eq!(InstantPayment::MAX_SETTLEMENT_SECONDS, 10);
    }

    #[test]
    fn test_instant_payment_invalid_amount() {
        let result = InstantPayment::new(
            default_customer(),
            default_account(),
            default_iban(),
            "BKTN".to_string(),
            0, // Invalid
            "TND".to_string(),
            "INV-001".to_string(),
        );
        assert!(result.is_err());
    }

    // --- FR-115: QR Payment Code ---

    #[test]
    fn test_qr_payment_code_creation() {
        let qr = QrPaymentCode::new(
            Uuid::new_v4(),
            25_000,
            "TND".to_string(),
            "MERCHANT-001".to_string(),
            "00020122630007TN.BTK01001020123456789".to_string(),
            Utc::now() + chrono::Duration::hours(24),
        )
        .unwrap();

        assert!(qr.is_valid());
        assert!(!qr.paid());
    }

    #[test]
    fn test_qr_payment_mark_paid() {
        let mut qr = QrPaymentCode::new(
            Uuid::new_v4(),
            25_000,
            "TND".to_string(),
            "MERCHANT-001".to_string(),
            "00020122630007TN.BTK01001020123456789".to_string(),
            Utc::now() + chrono::Duration::hours(24),
        )
        .unwrap();

        qr.mark_paid();
        assert!(qr.paid());
        assert!(!qr.is_valid());
    }

    #[test]
    fn test_qr_payment_scan_count() {
        let mut qr = QrPaymentCode::new(
            Uuid::new_v4(),
            25_000,
            "TND".to_string(),
            "MERCHANT-001".to_string(),
            "00020122630007TN.BTK01001020123456789".to_string(),
            Utc::now() + chrono::Duration::hours(24),
        )
        .unwrap();

        assert_eq!(qr.scanned_count(), 0);
        qr.increment_scan_count();
        qr.increment_scan_count();
        assert_eq!(qr.scanned_count(), 2);
    }

    #[test]
    fn test_qr_payment_invalid_expiry() {
        let result = QrPaymentCode::new(
            Uuid::new_v4(),
            25_000,
            "TND".to_string(),
            "MERCHANT-001".to_string(),
            "data".to_string(),
            Utc::now() - chrono::Duration::hours(1), // In the past
        );
        assert!(result.is_err());
    }

    // --- FR-116: Third-Party PIS ---

    #[test]
    fn test_third_party_pis_creation() {
        let pis = ThirdPartyPis::new(
            Uuid::new_v4(),
            "FinTechProvider".to_string(),
            default_customer(),
            PaymentConsentId::new(),
            default_account(),
            default_iban(),
            75_000,
            "TND".to_string(),
            "TXN-123".to_string(),
        )
        .unwrap();

        assert!(!pis.authorized());
        assert_eq!(pis.pis_provider_name(), "FinTechProvider");
    }

    #[test]
    fn test_third_party_pis_authorize() {
        let mut pis = ThirdPartyPis::new(
            Uuid::new_v4(),
            "FinTechProvider".to_string(),
            default_customer(),
            PaymentConsentId::new(),
            default_account(),
            default_iban(),
            75_000,
            "TND".to_string(),
            "TXN-123".to_string(),
        )
        .unwrap();

        pis.authorize();
        assert!(pis.authorized());
        assert!(pis.authorized_at().is_some());
    }

    #[test]
    fn test_third_party_pis_invalid_provider_name() {
        let result = ThirdPartyPis::new(
            Uuid::new_v4(),
            "".to_string(), // Empty
            default_customer(),
            PaymentConsentId::new(),
            default_account(),
            default_iban(),
            75_000,
            "TND".to_string(),
            "TXN-123".to_string(),
        );
        assert!(result.is_err());
    }

    // --- Enum tests ---

    #[test]
    fn test_consent_status_roundtrip() {
        for status in [
            ConsentStatus::Pending,
            ConsentStatus::Authorised,
            ConsentStatus::Rejected,
        ] {
            assert_eq!(ConsentStatus::from_str_type(status.as_str()).unwrap(), status);
        }
    }

    #[test]
    fn test_instant_payment_status_roundtrip() {
        for status in [
            InstantPaymentStatus::Initiated,
            InstantPaymentStatus::Settled,
            InstantPaymentStatus::Failed,
        ] {
            assert_eq!(
                InstantPaymentStatus::from_str_type(status.as_str()).unwrap(),
                status
            );
        }
    }
}
