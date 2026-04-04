use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{EmailAddress, PhoneNumber};

// --- CustomerType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomerType {
    Individual,
    LegalEntity,
}

impl CustomerType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "individual" => Ok(CustomerType::Individual),
            "legalentity" | "legal_entity" => Ok(CustomerType::LegalEntity),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown customer type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CustomerType::Individual => "Individual",
            CustomerType::LegalEntity => "LegalEntity",
        }
    }
}

impl fmt::Display for CustomerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CustomerStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomerStatus {
    Pending,
    Approved,
    Rejected,
    Suspended,
}

impl CustomerStatus {
    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(CustomerStatus::Pending),
            "approved" => Ok(CustomerStatus::Approved),
            "rejected" => Ok(CustomerStatus::Rejected),
            "suspended" => Ok(CustomerStatus::Suspended),
            _ => Err(DomainError::InvalidCustomerStatus(format!(
                "Unknown customer status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CustomerStatus::Pending => "Pending",
            CustomerStatus::Approved => "Approved",
            CustomerStatus::Rejected => "Rejected",
            CustomerStatus::Suspended => "Suspended",
        }
    }
}

impl fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- PepStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PepStatus {
    Yes,
    No,
    Unknown,
}

impl PepStatus {
    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "yes" => Ok(PepStatus::Yes),
            "no" => Ok(PepStatus::No),
            "unknown" => Ok(PepStatus::Unknown),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown PEP status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PepStatus::Yes => "Yes",
            PepStatus::No => "No",
            PepStatus::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for PepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SourceOfFunds ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceOfFunds {
    Salary,
    Business,
    Investment,
    Other,
}

impl SourceOfFunds {
    pub fn from_str_source(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "salary" => Ok(SourceOfFunds::Salary),
            "business" => Ok(SourceOfFunds::Business),
            "investment" => Ok(SourceOfFunds::Investment),
            "other" => Ok(SourceOfFunds::Other),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown source of funds: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SourceOfFunds::Salary => "Salary",
            SourceOfFunds::Business => "Business",
            SourceOfFunds::Investment => "Investment",
            SourceOfFunds::Other => "Other",
        }
    }
}

impl fmt::Display for SourceOfFunds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ConsentStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentStatus {
    Given,
    NotGiven,
}

impl ConsentStatus {
    pub fn from_str_consent(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "given" => Ok(ConsentStatus::Given),
            "notgiven" | "not_given" => Ok(ConsentStatus::NotGiven),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown consent status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConsentStatus::Given => "Given",
            ConsentStatus::NotGiven => "NotGiven",
        }
    }
}

impl fmt::Display for ConsentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- Cin (Carte d'Identite Nationale) ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cin {
    value: String,
}

impl Cin {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let cleaned: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
        if cleaned.len() != 8 {
            return Err(DomainError::InvalidCin(format!(
                "CIN must be exactly 8 digits, got {}",
                cleaned.len()
            )));
        }
        Ok(Cin { value: cleaned })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Cin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- Address ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    street: String,
    city: String,
    postal_code: String,
    country: String,
}

impl Address {
    pub fn new(
        street: &str,
        city: &str,
        postal_code: &str,
        country: &str,
    ) -> Result<Self, DomainError> {
        let street = street.trim().to_string();
        let city = city.trim().to_string();
        let postal_code = postal_code.trim().to_string();
        let country = country.trim().to_string();

        if street.is_empty() {
            return Err(DomainError::InvalidAddress(
                "Street cannot be empty".to_string(),
            ));
        }
        if city.is_empty() {
            return Err(DomainError::InvalidAddress(
                "City cannot be empty".to_string(),
            ));
        }
        if postal_code.is_empty() {
            return Err(DomainError::InvalidAddress(
                "Postal code cannot be empty".to_string(),
            ));
        }
        if country.is_empty() {
            return Err(DomainError::InvalidAddress(
                "Country cannot be empty".to_string(),
            ));
        }

        Ok(Address {
            street,
            city,
            postal_code,
            country,
        })
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    pub fn country(&self) -> &str {
        &self.country
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {} {}, {}",
            self.street, self.postal_code, self.city, self.country
        )
    }
}

// --- RiskLevel ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

// --- RiskScore ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RiskScore {
    value: u8,
}

impl RiskScore {
    pub fn new(value: u8) -> Result<Self, DomainError> {
        if value > 100 {
            return Err(DomainError::InvalidRiskScore(format!(
                "Risk score must be between 0 and 100, got {value}"
            )));
        }
        Ok(RiskScore { value })
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn risk_level(&self) -> RiskLevel {
        match self.value {
            0..=25 => RiskLevel::Low,
            26..=50 => RiskLevel::Medium,
            51..=75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

impl fmt::Display for RiskScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.value, self.risk_level())
    }
}

// --- Beneficiary ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beneficiary {
    id: Uuid,
    full_name: String,
    share_percentage: f64,
}

impl Beneficiary {
    pub fn new(full_name: &str, share_percentage: f64) -> Result<Self, DomainError> {
        let full_name = full_name.trim().to_string();
        if full_name.is_empty() {
            return Err(DomainError::ValidationError(
                "Beneficiary full_name cannot be empty".to_string(),
            ));
        }
        if !(0.0..=100.0).contains(&share_percentage) {
            return Err(DomainError::ValidationError(format!(
                "Beneficiary share must be between 0 and 100, got {share_percentage}"
            )));
        }
        Ok(Beneficiary {
            id: Uuid::new_v4(),
            full_name,
            share_percentage,
        })
    }

    pub fn reconstitute(id: Uuid, full_name: String, share_percentage: f64) -> Self {
        Beneficiary {
            id,
            full_name,
            share_percentage,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn share_percentage(&self) -> f64 {
        self.share_percentage
    }
}

// --- KycProfile ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KycProfile {
    full_name: String,
    cin_or_rcs: String,
    birth_date: Option<NaiveDate>,
    nationality: String,
    profession: String,
    address: Address,
    phone: PhoneNumber,
    email: EmailAddress,
    pep_status: PepStatus,
    source_of_funds: SourceOfFunds,
    sector: Option<String>,
    submission_date: Option<DateTime<Utc>>,
    approval_date: Option<DateTime<Utc>>,
    rejection_reason: Option<String>,
}

impl KycProfile {
    /// Create a KYC profile for an Individual customer.
    #[allow(clippy::too_many_arguments)]
    pub fn new_individual(
        full_name: &str,
        cin: Cin,
        birth_date: NaiveDate,
        nationality: &str,
        profession: &str,
        address: Address,
        phone: PhoneNumber,
        email: EmailAddress,
        pep_status: PepStatus,
        source_of_funds: SourceOfFunds,
    ) -> Result<Self, DomainError> {
        let full_name = full_name.trim().to_string();
        if full_name.is_empty() {
            return Err(DomainError::ValidationError(
                "KYC full_name cannot be empty".to_string(),
            ));
        }

        Ok(KycProfile {
            full_name,
            cin_or_rcs: cin.as_str().to_string(),
            birth_date: Some(birth_date),
            nationality: nationality.trim().to_string(),
            profession: profession.trim().to_string(),
            address,
            phone,
            email,
            pep_status,
            source_of_funds,
            sector: None,
            submission_date: Some(Utc::now()),
            approval_date: None,
            rejection_reason: None,
        })
    }

    /// Create a KYC profile for a LegalEntity customer.
    #[allow(clippy::too_many_arguments)]
    pub fn new_legal_entity(
        company_name: &str,
        registration_number: &str,
        sector: &str,
        address: Address,
        phone: PhoneNumber,
        email: EmailAddress,
        pep_status: PepStatus,
        source_of_funds: SourceOfFunds,
    ) -> Result<Self, DomainError> {
        let company_name = company_name.trim().to_string();
        if company_name.is_empty() {
            return Err(DomainError::ValidationError(
                "KYC company_name cannot be empty".to_string(),
            ));
        }
        let registration_number = registration_number.trim().to_string();
        if registration_number.is_empty() {
            return Err(DomainError::ValidationError(
                "Registration number (RCS) cannot be empty".to_string(),
            ));
        }

        Ok(KycProfile {
            full_name: company_name,
            cin_or_rcs: registration_number,
            birth_date: None,
            nationality: "Tunisia".to_string(),
            profession: String::new(),
            address,
            phone,
            email,
            pep_status,
            source_of_funds,
            sector: Some(sector.trim().to_string()),
            submission_date: Some(Utc::now()),
            approval_date: None,
            rejection_reason: None,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        full_name: String,
        cin_or_rcs: String,
        birth_date: Option<NaiveDate>,
        nationality: String,
        profession: String,
        address: Address,
        phone: PhoneNumber,
        email: EmailAddress,
        pep_status: PepStatus,
        source_of_funds: SourceOfFunds,
        sector: Option<String>,
        submission_date: Option<DateTime<Utc>>,
        approval_date: Option<DateTime<Utc>>,
        rejection_reason: Option<String>,
    ) -> Self {
        KycProfile {
            full_name,
            cin_or_rcs,
            birth_date,
            nationality,
            profession,
            address,
            phone,
            email,
            pep_status,
            source_of_funds,
            sector,
            submission_date,
            approval_date,
            rejection_reason,
        }
    }

    // --- Getters ---

    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    pub fn cin_or_rcs(&self) -> &str {
        &self.cin_or_rcs
    }

    pub fn birth_date(&self) -> Option<NaiveDate> {
        self.birth_date
    }

    pub fn nationality(&self) -> &str {
        &self.nationality
    }

    pub fn profession(&self) -> &str {
        &self.profession
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn phone(&self) -> &PhoneNumber {
        &self.phone
    }

    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    pub fn pep_status(&self) -> PepStatus {
        self.pep_status
    }

    pub fn source_of_funds(&self) -> SourceOfFunds {
        self.source_of_funds
    }

    pub fn sector(&self) -> Option<&str> {
        self.sector.as_deref()
    }

    pub fn submission_date(&self) -> Option<DateTime<Utc>> {
        self.submission_date
    }

    pub fn approval_date(&self) -> Option<DateTime<Utc>> {
        self.approval_date
    }

    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }

    // --- Mutators ---

    pub fn set_pep_status(&mut self, status: PepStatus) {
        self.pep_status = status;
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_address() -> Address {
        Address::new("10 Rue de la Liberte", "Tunis", "1000", "Tunisia").unwrap()
    }

    fn valid_phone() -> PhoneNumber {
        PhoneNumber::new("+21698123456").unwrap()
    }

    fn valid_email() -> EmailAddress {
        EmailAddress::new("ahmed@example.com").unwrap()
    }

    fn valid_cin() -> Cin {
        Cin::new("12345678").unwrap()
    }

    // --- Cin tests ---

    #[test]
    fn test_cin_valid() {
        let cin = Cin::new("12345678").unwrap();
        assert_eq!(cin.as_str(), "12345678");
    }

    #[test]
    fn test_cin_strips_non_digits() {
        let cin = Cin::new("1234 5678").unwrap();
        assert_eq!(cin.as_str(), "12345678");
    }

    #[test]
    fn test_cin_too_short() {
        assert!(Cin::new("1234567").is_err());
    }

    #[test]
    fn test_cin_too_long() {
        assert!(Cin::new("123456789").is_err());
    }

    #[test]
    fn test_cin_empty() {
        assert!(Cin::new("").is_err());
    }

    // --- Address tests ---

    #[test]
    fn test_address_valid() {
        let addr = valid_address();
        assert_eq!(addr.street(), "10 Rue de la Liberte");
        assert_eq!(addr.city(), "Tunis");
        assert_eq!(addr.postal_code(), "1000");
        assert_eq!(addr.country(), "Tunisia");
    }

    #[test]
    fn test_address_empty_street() {
        assert!(Address::new("", "Tunis", "1000", "Tunisia").is_err());
    }

    #[test]
    fn test_address_empty_city() {
        assert!(Address::new("Street", "", "1000", "Tunisia").is_err());
    }

    #[test]
    fn test_address_empty_postal_code() {
        assert!(Address::new("Street", "City", "", "Tunisia").is_err());
    }

    #[test]
    fn test_address_empty_country() {
        assert!(Address::new("Street", "City", "1000", "").is_err());
    }

    // --- RiskScore tests ---

    #[test]
    fn test_risk_score_valid() {
        let score = RiskScore::new(50).unwrap();
        assert_eq!(score.value(), 50);
    }

    #[test]
    fn test_risk_score_zero() {
        let score = RiskScore::new(0).unwrap();
        assert_eq!(score.value(), 0);
        assert_eq!(score.risk_level(), RiskLevel::Low);
    }

    #[test]
    fn test_risk_score_max() {
        let score = RiskScore::new(100).unwrap();
        assert_eq!(score.value(), 100);
        assert_eq!(score.risk_level(), RiskLevel::Critical);
    }

    #[test]
    fn test_risk_score_too_high() {
        assert!(RiskScore::new(101).is_err());
    }

    #[test]
    fn test_risk_score_levels() {
        assert_eq!(RiskScore::new(10).unwrap().risk_level(), RiskLevel::Low);
        assert_eq!(RiskScore::new(25).unwrap().risk_level(), RiskLevel::Low);
        assert_eq!(RiskScore::new(26).unwrap().risk_level(), RiskLevel::Medium);
        assert_eq!(RiskScore::new(50).unwrap().risk_level(), RiskLevel::Medium);
        assert_eq!(RiskScore::new(51).unwrap().risk_level(), RiskLevel::High);
        assert_eq!(RiskScore::new(75).unwrap().risk_level(), RiskLevel::High);
        assert_eq!(
            RiskScore::new(76).unwrap().risk_level(),
            RiskLevel::Critical
        );
    }

    // --- Beneficiary tests ---

    #[test]
    fn test_beneficiary_valid() {
        let b = Beneficiary::new("Ahmed Ben Ayed", 50.0).unwrap();
        assert_eq!(b.full_name(), "Ahmed Ben Ayed");
        assert_eq!(b.share_percentage(), 50.0);
    }

    #[test]
    fn test_beneficiary_empty_name() {
        assert!(Beneficiary::new("", 50.0).is_err());
    }

    #[test]
    fn test_beneficiary_negative_share() {
        assert!(Beneficiary::new("Test", -1.0).is_err());
    }

    #[test]
    fn test_beneficiary_over_100_share() {
        assert!(Beneficiary::new("Test", 101.0).is_err());
    }

    #[test]
    fn test_beneficiary_zero_share() {
        let b = Beneficiary::new("Test", 0.0).unwrap();
        assert_eq!(b.share_percentage(), 0.0);
    }

    // --- CustomerType tests ---

    #[test]
    fn test_customer_type_from_str() {
        assert_eq!(
            CustomerType::from_str_type("Individual").unwrap(),
            CustomerType::Individual
        );
        assert_eq!(
            CustomerType::from_str_type("LegalEntity").unwrap(),
            CustomerType::LegalEntity
        );
        assert_eq!(
            CustomerType::from_str_type("legal_entity").unwrap(),
            CustomerType::LegalEntity
        );
    }

    #[test]
    fn test_customer_type_invalid() {
        assert!(CustomerType::from_str_type("unknown").is_err());
    }

    // --- CustomerStatus tests ---

    #[test]
    fn test_customer_status_from_str() {
        assert_eq!(
            CustomerStatus::from_str_status("Pending").unwrap(),
            CustomerStatus::Pending
        );
        assert_eq!(
            CustomerStatus::from_str_status("approved").unwrap(),
            CustomerStatus::Approved
        );
        assert_eq!(
            CustomerStatus::from_str_status("Rejected").unwrap(),
            CustomerStatus::Rejected
        );
        assert_eq!(
            CustomerStatus::from_str_status("Suspended").unwrap(),
            CustomerStatus::Suspended
        );
    }

    #[test]
    fn test_customer_status_invalid() {
        assert!(CustomerStatus::from_str_status("active").is_err());
    }

    // --- PepStatus tests ---

    #[test]
    fn test_pep_status_from_str() {
        assert_eq!(PepStatus::from_str_status("Yes").unwrap(), PepStatus::Yes);
        assert_eq!(PepStatus::from_str_status("no").unwrap(), PepStatus::No);
        assert_eq!(
            PepStatus::from_str_status("Unknown").unwrap(),
            PepStatus::Unknown
        );
    }

    // --- SourceOfFunds tests ---

    #[test]
    fn test_source_of_funds_from_str() {
        assert_eq!(
            SourceOfFunds::from_str_source("Salary").unwrap(),
            SourceOfFunds::Salary
        );
        assert_eq!(
            SourceOfFunds::from_str_source("business").unwrap(),
            SourceOfFunds::Business
        );
    }

    // --- ConsentStatus tests ---

    #[test]
    fn test_consent_status_from_str() {
        assert_eq!(
            ConsentStatus::from_str_consent("Given").unwrap(),
            ConsentStatus::Given
        );
        assert_eq!(
            ConsentStatus::from_str_consent("NotGiven").unwrap(),
            ConsentStatus::NotGiven
        );
    }

    // --- KycProfile tests ---

    #[test]
    fn test_kyc_profile_individual_valid() {
        let profile = KycProfile::new_individual(
            "Ahmed Ben Ayed",
            valid_cin(),
            NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            "Tunisia",
            "Banker",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Salary,
        );
        assert!(profile.is_ok());
        let p = profile.unwrap();
        assert_eq!(p.full_name(), "Ahmed Ben Ayed");
        assert_eq!(p.cin_or_rcs(), "12345678");
        assert!(p.submission_date().is_some());
        assert!(p.sector().is_none());
    }

    #[test]
    fn test_kyc_profile_individual_empty_name() {
        let profile = KycProfile::new_individual(
            "",
            valid_cin(),
            NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            "Tunisia",
            "Banker",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Salary,
        );
        assert!(profile.is_err());
    }

    #[test]
    fn test_kyc_profile_legal_entity_valid() {
        let profile = KycProfile::new_legal_entity(
            "Banko SA",
            "RCS-12345",
            "Banking",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Business,
        );
        assert!(profile.is_ok());
        let p = profile.unwrap();
        assert_eq!(p.full_name(), "Banko SA");
        assert_eq!(p.cin_or_rcs(), "RCS-12345");
        assert_eq!(p.sector(), Some("Banking"));
    }

    #[test]
    fn test_kyc_profile_legal_entity_empty_company() {
        let profile = KycProfile::new_legal_entity(
            "",
            "RCS-12345",
            "Banking",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Business,
        );
        assert!(profile.is_err());
    }

    #[test]
    fn test_kyc_profile_legal_entity_empty_rcs() {
        let profile = KycProfile::new_legal_entity(
            "Banko SA",
            "",
            "Banking",
            valid_address(),
            valid_phone(),
            valid_email(),
            PepStatus::No,
            SourceOfFunds::Business,
        );
        assert!(profile.is_err());
    }
}
