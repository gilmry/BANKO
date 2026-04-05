use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::DomainError;

// --- Currency ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    TND,
    EUR,
    USD,
    GBP,
    LYD,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::TND => write!(f, "TND"),
            Currency::EUR => write!(f, "EUR"),
            Currency::USD => write!(f, "USD"),
            Currency::GBP => write!(f, "GBP"),
            Currency::LYD => write!(f, "LYD"),
        }
    }
}

impl Currency {
    pub fn from_code(code: &str) -> Result<Self, DomainError> {
        match code.to_uppercase().as_str() {
            "TND" => Ok(Currency::TND),
            "EUR" => Ok(Currency::EUR),
            "USD" => Ok(Currency::USD),
            "GBP" => Ok(Currency::GBP),
            "LYD" => Ok(Currency::LYD),
            _ => Err(DomainError::InvalidCurrency(format!(
                "Unknown currency code: {code}"
            ))),
        }
    }

    pub fn decimal_places(&self) -> u32 {
        match self {
            Currency::TND | Currency::LYD => 3,
            Currency::EUR | Currency::USD | Currency::GBP => 2,
        }
    }
}

// --- Money ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    amount: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Result<Self, DomainError> {
        if amount.is_nan() || amount.is_infinite() {
            return Err(DomainError::InvalidMoney(
                "Amount must be a finite number".to_string(),
            ));
        }
        let factor = 10_f64.powi(currency.decimal_places() as i32);
        let cents = (amount * factor).round() as i64;
        Ok(Money {
            amount: cents,
            currency,
        })
    }

    pub fn from_cents(amount_cents: i64, currency: Currency) -> Self {
        Money {
            amount: amount_cents,
            currency,
        }
    }

    pub fn zero(currency: Currency) -> Self {
        Money {
            amount: 0,
            currency,
        }
    }

    pub fn amount(&self) -> f64 {
        let factor = 10_f64.powi(self.currency.decimal_places() as i32);
        self.amount as f64 / factor
    }

    pub fn amount_cents(&self) -> i64 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn add(&self, other: &Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::InvalidMoney(format!(
                "Cannot add {} and {}",
                self.currency, other.currency
            )));
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }

    pub fn subtract(&self, other: &Money) -> Result<Money, DomainError> {
        if self.currency != other.currency {
            return Err(DomainError::InvalidMoney(format!(
                "Cannot subtract {} from {}",
                other.currency, self.currency
            )));
        }
        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency,
        })
    }

    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let places = self.currency.decimal_places();
        write!(
            f,
            "{:.prec$} {}",
            self.amount(),
            self.currency,
            prec = places as usize
        )
    }
}

impl Eq for Money {}

impl std::hash::Hash for Money {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.amount.hash(state);
        self.currency.hash(state);
    }
}

// --- Percentage ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percentage {
    value: f64,
}

impl Percentage {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value.is_nan() || value.is_infinite() {
            return Err(DomainError::InvalidPercentage(
                "Value must be a finite number".to_string(),
            ));
        }
        if !(-100.0..=100.0).contains(&value) {
            return Err(DomainError::InvalidPercentage(format!(
                "Value must be between -100 and 100, got {value}"
            )));
        }
        Ok(Percentage { value })
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn as_decimal(&self) -> f64 {
        self.value / 100.0
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.value)
    }
}

// --- RIB (Relevé d'Identité Bancaire) ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rib {
    value: String,
}

impl Rib {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let cleaned: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
        if cleaned.len() != 20 {
            return Err(DomainError::InvalidRib(format!(
                "RIB must be 20 digits, got {}",
                cleaned.len()
            )));
        }
        Ok(Rib { value: cleaned })
    }

    pub fn bank_code(&self) -> &str {
        &self.value[0..2]
    }

    pub fn branch_code(&self) -> &str {
        &self.value[2..5]
    }

    pub fn account_number(&self) -> &str {
        &self.value[5..18]
    }

    pub fn check_digits(&self) -> &str {
        &self.value[18..20]
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Rib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}",
            self.bank_code(),
            self.branch_code(),
            self.account_number(),
            self.check_digits()
        )
    }
}

// --- BIC (Bank Identifier Code) ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bic {
    value: String,
}

impl Bic {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let upper = value.trim().to_uppercase();
        if upper.len() != 8 && upper.len() != 11 {
            return Err(DomainError::InvalidBic(format!(
                "BIC must be 8 or 11 characters, got {}",
                upper.len()
            )));
        }
        if !upper.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::InvalidBic(
                "BIC must contain only alphanumeric characters".to_string(),
            ));
        }
        Ok(Bic { value: upper })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Bic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- EmailAddress ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress {
    value: String,
}

impl EmailAddress {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let trimmed = value.trim().to_lowercase();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidEmail(
                "Email cannot be empty".to_string(),
            ));
        }
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 {
            return Err(DomainError::InvalidEmail(format!(
                "Email must contain exactly one @: {trimmed}"
            )));
        }
        if parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::InvalidEmail(format!(
                "Email local and domain parts cannot be empty: {trimmed}"
            )));
        }
        if !parts[1].contains('.') {
            return Err(DomainError::InvalidEmail(format!(
                "Email domain must contain a dot: {trimmed}"
            )));
        }
        Ok(EmailAddress { value: trimmed })
    }

    /// Create an EmailAddress without validation (for anonymization/reconstitution).
    pub fn unchecked(value: &str) -> Self {
        EmailAddress {
            value: value.to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn local_part(&self) -> &str {
        self.value.split('@').next().unwrap_or("")
    }

    pub fn domain(&self) -> &str {
        self.value.split('@').nth(1).unwrap_or("")
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- PhoneNumber ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneNumber {
    value: String,
}

impl PhoneNumber {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let cleaned: String = value
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect();
        if cleaned.len() < 8 {
            return Err(DomainError::InvalidPhoneNumber(format!(
                "Phone number too short: {cleaned}"
            )));
        }
        if cleaned.len() > 15 {
            return Err(DomainError::InvalidPhoneNumber(format!(
                "Phone number too long: {cleaned}"
            )));
        }
        Ok(PhoneNumber { value: cleaned })
    }

    /// Create a PhoneNumber without validation (for anonymization/reconstitution).
    pub fn unchecked(value: &str) -> Self {
        PhoneNumber {
            value: value.to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- AccountNumber ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountNumber {
    value: String,
}

impl AccountNumber {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidAccountNumber(
                "Account number cannot be empty".to_string(),
            ));
        }
        if trimmed.len() < 5 || trimmed.len() > 30 {
            return Err(DomainError::InvalidAccountNumber(format!(
                "Account number must be 5-30 characters, got {}",
                trimmed.len()
            )));
        }
        Ok(AccountNumber { value: trimmed })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- CustomerId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomerId(Uuid);

impl CustomerId {
    pub fn new() -> Self {
        CustomerId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        CustomerId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(CustomerId)
            .map_err(|e| DomainError::InvalidCustomerId(format!("Invalid UUID: {e}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CustomerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CustomerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Currency tests ---

    #[test]
    fn test_currency_from_valid_code() {
        assert_eq!(Currency::from_code("TND").unwrap(), Currency::TND);
        assert_eq!(Currency::from_code("eur").unwrap(), Currency::EUR);
        assert_eq!(Currency::from_code("Usd").unwrap(), Currency::USD);
    }

    #[test]
    fn test_currency_from_invalid_code() {
        assert!(Currency::from_code("XYZ").is_err());
        assert!(Currency::from_code("").is_err());
    }

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::TND.decimal_places(), 3);
        assert_eq!(Currency::EUR.decimal_places(), 2);
    }

    // --- Money tests ---

    #[test]
    fn test_money_valid() {
        let m = Money::new(5000.0, Currency::TND).unwrap();
        assert_eq!(m.amount(), 5000.0);
        assert_eq!(m.currency(), Currency::TND);
    }

    #[test]
    fn test_money_zero() {
        let m = Money::zero(Currency::EUR);
        assert!(m.is_zero());
        assert_eq!(m.amount(), 0.0);
    }

    #[test]
    fn test_money_nan_rejected() {
        assert!(Money::new(f64::NAN, Currency::TND).is_err());
    }

    #[test]
    fn test_money_infinity_rejected() {
        assert!(Money::new(f64::INFINITY, Currency::TND).is_err());
    }

    #[test]
    fn test_money_add_same_currency() {
        let a = Money::new(100.0, Currency::TND).unwrap();
        let b = Money::new(200.5, Currency::TND).unwrap();
        let c = a.add(&b).unwrap();
        assert_eq!(c.amount(), 300.5);
    }

    #[test]
    fn test_money_add_different_currency_fails() {
        let a = Money::new(100.0, Currency::TND).unwrap();
        let b = Money::new(100.0, Currency::EUR).unwrap();
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn test_money_subtract() {
        let a = Money::new(500.0, Currency::EUR).unwrap();
        let b = Money::new(200.0, Currency::EUR).unwrap();
        let c = a.subtract(&b).unwrap();
        assert_eq!(c.amount(), 300.0);
    }

    #[test]
    fn test_money_negative() {
        let a = Money::new(100.0, Currency::TND).unwrap();
        let b = Money::new(200.0, Currency::TND).unwrap();
        let c = a.subtract(&b).unwrap();
        assert!(c.is_negative());
    }

    #[test]
    fn test_money_display() {
        let m = Money::new(1234.567, Currency::TND).unwrap();
        assert_eq!(format!("{m}"), "1234.567 TND");
    }

    // --- Percentage tests ---

    #[test]
    fn test_percentage_valid() {
        let p = Percentage::new(12.5).unwrap();
        assert_eq!(p.value(), 12.5);
        assert_eq!(p.as_decimal(), 0.125);
    }

    #[test]
    fn test_percentage_zero() {
        let p = Percentage::new(0.0).unwrap();
        assert_eq!(p.value(), 0.0);
    }

    #[test]
    fn test_percentage_out_of_range() {
        assert!(Percentage::new(101.0).is_err());
        assert!(Percentage::new(-101.0).is_err());
    }

    #[test]
    fn test_percentage_nan() {
        assert!(Percentage::new(f64::NAN).is_err());
    }

    #[test]
    fn test_percentage_display() {
        let p = Percentage::new(7.50).unwrap();
        assert_eq!(format!("{p}"), "7.50%");
    }

    // --- RIB tests ---

    #[test]
    fn test_rib_valid() {
        let rib = Rib::new("01234567890123456789").unwrap();
        assert_eq!(rib.bank_code(), "01");
        assert_eq!(rib.branch_code(), "234");
        assert_eq!(rib.account_number(), "5678901234567");
        assert_eq!(rib.check_digits(), "89");
    }

    #[test]
    fn test_rib_with_spaces() {
        let rib = Rib::new("01 234 5678901234567 89").unwrap();
        assert_eq!(rib.as_str(), "01234567890123456789");
    }

    #[test]
    fn test_rib_too_short() {
        assert!(Rib::new("12345").is_err());
    }

    #[test]
    fn test_rib_too_long() {
        assert!(Rib::new("012345678901234567890").is_err());
    }

    #[test]
    fn test_rib_display() {
        let rib = Rib::new("01234567890123456789").unwrap();
        assert_eq!(format!("{rib}"), "01-234-5678901234567-89");
    }

    // --- BIC tests ---

    #[test]
    fn test_bic_valid_8() {
        let bic = Bic::new("BIATTNTT").unwrap();
        assert_eq!(bic.as_str(), "BIATTNTT");
    }

    #[test]
    fn test_bic_valid_11() {
        let bic = Bic::new("BIATTNTTXXX").unwrap();
        assert_eq!(bic.as_str(), "BIATTNTTXXX");
    }

    #[test]
    fn test_bic_lowercase_normalized() {
        let bic = Bic::new("biattntt").unwrap();
        assert_eq!(bic.as_str(), "BIATTNTT");
    }

    #[test]
    fn test_bic_invalid_length() {
        assert!(Bic::new("BIAT").is_err());
        assert!(Bic::new("BIATTNTTXXXX").is_err());
    }

    #[test]
    fn test_bic_invalid_chars() {
        assert!(Bic::new("BIAT-TNT").is_err());
    }

    // --- EmailAddress tests ---

    #[test]
    fn test_email_valid() {
        let e = EmailAddress::new("user@banko.tn").unwrap();
        assert_eq!(e.as_str(), "user@banko.tn");
        assert_eq!(e.local_part(), "user");
        assert_eq!(e.domain(), "banko.tn");
    }

    #[test]
    fn test_email_normalized_lowercase() {
        let e = EmailAddress::new("USER@BANKO.TN").unwrap();
        assert_eq!(e.as_str(), "user@banko.tn");
    }

    #[test]
    fn test_email_empty() {
        assert!(EmailAddress::new("").is_err());
    }

    #[test]
    fn test_email_no_at() {
        assert!(EmailAddress::new("userbanko.tn").is_err());
    }

    #[test]
    fn test_email_no_domain_dot() {
        assert!(EmailAddress::new("user@bankotn").is_err());
    }

    #[test]
    fn test_email_double_at() {
        assert!(EmailAddress::new("user@@banko.tn").is_err());
    }

    // --- PhoneNumber tests ---

    #[test]
    fn test_phone_valid_tunisian() {
        let p = PhoneNumber::new("+21671234567").unwrap();
        assert_eq!(p.as_str(), "+21671234567");
    }

    #[test]
    fn test_phone_valid_with_spaces() {
        let p = PhoneNumber::new("+216 71 234 567").unwrap();
        assert_eq!(p.as_str(), "+21671234567");
    }

    #[test]
    fn test_phone_too_short() {
        assert!(PhoneNumber::new("12345").is_err());
    }

    #[test]
    fn test_phone_too_long() {
        assert!(PhoneNumber::new("1234567890123456").is_err());
    }

    // --- AccountNumber tests ---

    #[test]
    fn test_account_number_valid() {
        let a = AccountNumber::new("01-234-0001234-56").unwrap();
        assert_eq!(a.as_str(), "01-234-0001234-56");
    }

    #[test]
    fn test_account_number_empty() {
        assert!(AccountNumber::new("").is_err());
    }

    #[test]
    fn test_account_number_too_short() {
        assert!(AccountNumber::new("1234").is_err());
    }

    // --- CustomerId tests ---

    #[test]
    fn test_customer_id_new() {
        let id = CustomerId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_customer_id_from_str_valid() {
        let id = CustomerId::parse("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(id.to_string(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_customer_id_from_str_invalid() {
        assert!(CustomerId::parse("not-a-uuid").is_err());
    }

    #[test]
    fn test_customer_id_equality() {
        let uuid = Uuid::new_v4();
        let a = CustomerId::from_uuid(uuid);
        let b = CustomerId::from_uuid(uuid);
        assert_eq!(a, b);
    }
}
