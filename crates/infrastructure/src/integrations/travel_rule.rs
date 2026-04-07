//! Travel Rule R.16 Compliance for International Wire Transfers
//!
//! Implements STORY-COMP-13: Compliance with Financial Action Task Force (FATF)
//! Recommendation 16 (Travel Rule) for cross-border wire transfers.
//!
//! Travel Rule R.16 requires that originator and beneficiary information be
//! transmitted with wire transfers to jurisdictions that require it, or when
//! transfers exceed certain thresholds.
//!
//! Reference: FATF Recommendations (2012), Travel Rule Guidance

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TravelRuleError {
    InvalidData(String),
    ValidationFailed(String),
    EnrichmentFailed(String),
    MissingRequiredField(String),
}

impl fmt::Display for TravelRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TravelRuleError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            TravelRuleError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            TravelRuleError::EnrichmentFailed(msg) => write!(f, "Enrichment failed: {}", msg),
            TravelRuleError::MissingRequiredField(msg) => write!(f, "Missing required field: {}", msg),
        }
    }
}

impl std::error::Error for TravelRuleError {}

// ============================================================================
// ENUMS
// ============================================================================

/// Travel Rule compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TravelRuleCompliance {
    /// Transfer is compliant with Travel Rule
    Compliant,
    /// Transfer requires Travel Rule information
    RequiresInformation,
    /// Transfer is exempt from Travel Rule
    Exempt,
    /// Transfer is non-compliant
    NonCompliant,
}

impl TravelRuleCompliance {
    pub fn as_str(&self) -> &str {
        match self {
            TravelRuleCompliance::Compliant => "COMPLIANT",
            TravelRuleCompliance::RequiresInformation => "REQUIRES_INFORMATION",
            TravelRuleCompliance::Exempt => "EXEMPT",
            TravelRuleCompliance::NonCompliant => "NON_COMPLIANT",
        }
    }
}

impl fmt::Display for TravelRuleCompliance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Type of identification document
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdType {
    Passport,
    NationalId,
    DriverLicense,
    BusinessRegistration,
    TaxId,
    Other(String),
}

impl IdType {
    pub fn as_str(&self) -> &str {
        match self {
            IdType::Passport => "PASSPORT",
            IdType::NationalId => "NATIONAL_ID",
            IdType::DriverLicense => "DRIVER_LICENSE",
            IdType::BusinessRegistration => "BUSINESS_REGISTRATION",
            IdType::TaxId => "TAX_ID",
            IdType::Other(s) => s,
        }
    }
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Originator (sender) information for Travel Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Originator {
    /// Account number at the originating bank
    pub account_number: String,
    /// Full name or legal entity name
    pub name: String,
    /// Physical address
    pub address: String,
    /// City
    pub city: String,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Postal code
    pub postal_code: String,
    /// Type of identification
    pub id_type: Option<IdType>,
    /// Identification number
    pub id_number: Option<String>,
    /// Date of birth (for natural persons)
    pub date_of_birth: Option<String>,
}

impl Originator {
    pub fn new(
        account_number: String,
        name: String,
        address: String,
        city: String,
        country: String,
        postal_code: String,
    ) -> Result<Self, TravelRuleError> {
        if account_number.trim().is_empty() {
            return Err(TravelRuleError::MissingRequiredField(
                "Account number cannot be empty".to_string(),
            ));
        }
        if name.trim().is_empty() {
            return Err(TravelRuleError::MissingRequiredField(
                "Name cannot be empty".to_string(),
            ));
        }
        if address.trim().is_empty() {
            return Err(TravelRuleError::MissingRequiredField(
                "Address cannot be empty".to_string(),
            ));
        }

        Ok(Originator {
            account_number,
            name,
            address,
            city,
            country,
            postal_code,
            id_type: None,
            id_number: None,
            date_of_birth: None,
        })
    }

    pub fn with_id(mut self, id_type: IdType, id_number: String) -> Self {
        self.id_type = Some(id_type);
        self.id_number = Some(id_number);
        self
    }

    pub fn with_date_of_birth(mut self, dob: String) -> Self {
        self.date_of_birth = Some(dob);
        self
    }
}

/// Beneficiary (recipient) information for Travel Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beneficiary {
    /// Account number at the beneficiary bank
    pub account_number: String,
    /// Full name or legal entity name
    pub name: String,
    /// Physical address
    pub address: String,
    /// City
    pub city: String,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Postal code
    pub postal_code: String,
    /// Bank name
    pub bank_name: Option<String>,
    /// Bank code/SWIFT code
    pub bank_code: Option<String>,
}

impl Beneficiary {
    pub fn new(
        account_number: String,
        name: String,
        address: String,
        city: String,
        country: String,
        postal_code: String,
    ) -> Result<Self, TravelRuleError> {
        if account_number.trim().is_empty() {
            return Err(TravelRuleError::MissingRequiredField(
                "Account number cannot be empty".to_string(),
            ));
        }
        if name.trim().is_empty() {
            return Err(TravelRuleError::MissingRequiredField(
                "Name cannot be empty".to_string(),
            ));
        }

        Ok(Beneficiary {
            account_number,
            name,
            address,
            city,
            country,
            postal_code,
            bank_name: None,
            bank_code: None,
        })
    }

    pub fn with_bank(mut self, bank_name: String, bank_code: String) -> Self {
        self.bank_name = Some(bank_name);
        self.bank_code = Some(bank_code);
        self
    }
}

/// Travel Rule data to be transmitted with payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelRuleData {
    /// Unique identifier for this travel rule record
    pub id: String,
    /// Originator information
    pub originator: Originator,
    /// Beneficiary information
    pub beneficiary: Beneficiary,
    /// Transfer amount in lowest currency unit (e.g., cents)
    pub amount: f64,
    /// Currency code (ISO 4217)
    pub currency: String,
    /// Ordering customer reference
    pub reference: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl TravelRuleData {
    pub fn new(
        originator: Originator,
        beneficiary: Beneficiary,
        amount: f64,
        currency: String,
    ) -> Result<Self, TravelRuleError> {
        if amount <= 0.0 {
            return Err(TravelRuleError::ValidationFailed(
                "Amount must be positive".to_string(),
            ));
        }
        if currency.trim().is_empty() {
            return Err(TravelRuleError::ValidationFailed(
                "Currency cannot be empty".to_string(),
            ));
        }

        Ok(TravelRuleData {
            id: Uuid::new_v4().to_string(),
            originator,
            beneficiary,
            amount,
            currency,
            reference: None,
            created_at: Utc::now(),
        })
    }

    pub fn with_reference(mut self, reference: String) -> Self {
        self.reference = Some(reference);
        self
    }
}

/// Payment enriched with Travel Rule compliance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedPayment {
    /// Payment ID
    pub payment_id: String,
    /// Original amount
    pub amount: f64,
    /// Currency
    pub currency: String,
    /// Beneficiary account
    pub beneficiary_account: String,
    /// Travel Rule compliance status
    pub travel_rule_compliance: TravelRuleCompliance,
    /// Travel Rule data if applicable
    pub travel_rule_data: Option<TravelRuleData>,
    /// Whether this is a cross-border transfer
    pub is_cross_border: bool,
    /// Country of origination
    pub originating_country: String,
    /// Country of destination
    pub destination_country: String,
    /// Timestamp of enrichment
    pub enriched_at: DateTime<Utc>,
}

// ============================================================================
// TRAVEL RULE VALIDATOR
// ============================================================================

/// Validator for Travel Rule compliance
pub struct TravelRuleValidator {
    /// Threshold amount (in smallest currency unit) above which Travel Rule applies
    /// Default: 3000 USD equivalent (varies by jurisdiction)
    pub threshold: f64,
    /// List of jurisdictions requiring Travel Rule compliance
    pub restricted_jurisdictions: Vec<String>,
    /// List of exempt jurisdictions
    pub exempt_jurisdictions: Vec<String>,
}

impl TravelRuleValidator {
    pub fn new() -> Self {
        TravelRuleValidator {
            threshold: 3000.0, // Configurable threshold
            restricted_jurisdictions: vec![],
            exempt_jurisdictions: vec!["TN".to_string()], // Tunisia to domestic is exempt
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn add_restricted_jurisdiction(mut self, country: String) -> Self {
        self.restricted_jurisdictions.push(country);
        self
    }

    /// Determine if Travel Rule is required for this transfer
    pub fn requires_travel_rule(
        &self,
        originating_country: &str,
        destination_country: &str,
        amount: f64,
    ) -> bool {
        // Domestic transfers are exempt
        if originating_country == destination_country {
            return false;
        }

        // Check exempt jurisdictions
        if self.exempt_jurisdictions.contains(&destination_country.to_string()) {
            return false;
        }

        // Check if transfer exceeds threshold
        amount >= self.threshold
    }

    /// Validate Travel Rule compliance
    pub fn validate_travel_rule(
        &self,
        originating_country: &str,
        destination_country: &str,
        amount: f64,
        has_travel_rule_data: bool,
    ) -> Result<TravelRuleCompliance, TravelRuleError> {
        // Check if Travel Rule is required
        if !self.requires_travel_rule(originating_country, destination_country, amount) {
            return Ok(TravelRuleCompliance::Exempt);
        }

        // Travel Rule is required
        if has_travel_rule_data {
            Ok(TravelRuleCompliance::Compliant)
        } else {
            Ok(TravelRuleCompliance::RequiresInformation)
        }
    }
}

impl Default for TravelRuleValidator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PAYMENT ENRICHMENT
// ============================================================================

/// Enrich a payment with Travel Rule compliance information
pub fn enrich_payment_with_travel_data(
    payment_id: String,
    amount: f64,
    currency: String,
    beneficiary_account: String,
    originating_country: String,
    destination_country: String,
    originator: Option<Originator>,
    beneficiary: Option<Beneficiary>,
) -> Result<EnrichedPayment, TravelRuleError> {
    let validator = TravelRuleValidator::new();

    // Determine if Travel Rule applies
    let requires_tr = validator.requires_travel_rule(&originating_country, &destination_country, amount);

    // If Travel Rule is required and we have both parties, create Travel Rule data
    let (travel_rule_data, compliance_status) = if requires_tr {
        if let (Some(orig), Some(bene)) = (originator, beneficiary) {
            let tr_data = TravelRuleData::new(orig, bene, amount, currency.clone())?;
            (Some(tr_data), TravelRuleCompliance::Compliant)
        } else {
            (None, TravelRuleCompliance::RequiresInformation)
        }
    } else {
        (None, TravelRuleCompliance::Exempt)
    };

    Ok(EnrichedPayment {
        payment_id,
        amount,
        currency,
        beneficiary_account,
        travel_rule_compliance: compliance_status,
        travel_rule_data,
        is_cross_border: originating_country != destination_country,
        originating_country,
        destination_country,
        enriched_at: Utc::now(),
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_originator_creation() {
        let orig = Originator::new(
            "ACC-001".to_string(),
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Tunis".to_string(),
            "TN".to_string(),
            "1000".to_string(),
        )
        .unwrap();

        assert_eq!(orig.account_number, "ACC-001");
        assert_eq!(orig.name, "John Doe");
    }

    #[test]
    fn test_originator_with_id() {
        let orig = Originator::new(
            "ACC-001".to_string(),
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Tunis".to_string(),
            "TN".to_string(),
            "1000".to_string(),
        )
        .unwrap()
        .with_id(IdType::Passport, "A123456".to_string());

        assert_eq!(orig.id_type, Some(IdType::Passport));
    }

    #[test]
    fn test_beneficiary_creation() {
        let bene = Beneficiary::new(
            "ACC-002".to_string(),
            "Jane Smith".to_string(),
            "456 Oak Ave".to_string(),
            "Paris".to_string(),
            "FR".to_string(),
            "75001".to_string(),
        )
        .unwrap();

        assert_eq!(bene.name, "Jane Smith");
        assert_eq!(bene.country, "FR");
    }

    #[test]
    fn test_travel_rule_data_creation() {
        let orig = Originator::new(
            "ACC-001".to_string(),
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Tunis".to_string(),
            "TN".to_string(),
            "1000".to_string(),
        )
        .unwrap();

        let bene = Beneficiary::new(
            "ACC-002".to_string(),
            "Jane Smith".to_string(),
            "456 Oak Ave".to_string(),
            "Paris".to_string(),
            "FR".to_string(),
            "75001".to_string(),
        )
        .unwrap();

        let tr_data =
            TravelRuleData::new(orig, bene, 5000.0, "EUR".to_string()).unwrap();

        assert_eq!(tr_data.amount, 5000.0);
        assert_eq!(tr_data.currency, "EUR");
    }

    #[test]
    fn test_travel_rule_validator_domestic_transfer() {
        let validator = TravelRuleValidator::new();

        let requires = validator.requires_travel_rule("TN", "TN", 5000.0);
        assert!(!requires); // Domestic transfer is exempt
    }

    #[test]
    fn test_travel_rule_validator_low_amount() {
        let validator = TravelRuleValidator::new().with_threshold(3000.0);

        let requires = validator.requires_travel_rule("TN", "FR", 1000.0);
        assert!(!requires); // Below threshold
    }

    #[test]
    fn test_travel_rule_validator_high_amount() {
        let validator = TravelRuleValidator::new().with_threshold(3000.0);

        let requires = validator.requires_travel_rule("TN", "FR", 5000.0);
        assert!(requires); // Above threshold
    }

    #[test]
    fn test_validate_travel_rule_exempt() {
        let validator = TravelRuleValidator::new();

        let compliance =
            validator.validate_travel_rule("TN", "TN", 5000.0, false).unwrap();
        assert_eq!(compliance, TravelRuleCompliance::Exempt);
    }

    #[test]
    fn test_validate_travel_rule_compliant() {
        let validator = TravelRuleValidator::new();

        let compliance =
            validator.validate_travel_rule("TN", "FR", 5000.0, true).unwrap();
        assert_eq!(compliance, TravelRuleCompliance::Compliant);
    }

    #[test]
    fn test_validate_travel_rule_requires_information() {
        let validator = TravelRuleValidator::new();

        let compliance =
            validator.validate_travel_rule("TN", "FR", 5000.0, false).unwrap();
        assert_eq!(compliance, TravelRuleCompliance::RequiresInformation);
    }

    #[test]
    fn test_enrich_payment_exempt() {
        let enriched = enrich_payment_with_travel_data(
            "PAY-001".to_string(),
            5000.0,
            "TND".to_string(),
            "ACC-002".to_string(),
            "TN".to_string(),
            "TN".to_string(),
            None,
            None,
        )
        .unwrap();

        assert_eq!(enriched.travel_rule_compliance, TravelRuleCompliance::Exempt);
        assert!(!enriched.is_cross_border);
    }

    #[test]
    fn test_enrich_payment_compliant() {
        let orig = Originator::new(
            "ACC-001".to_string(),
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Tunis".to_string(),
            "TN".to_string(),
            "1000".to_string(),
        )
        .unwrap();

        let bene = Beneficiary::new(
            "ACC-002".to_string(),
            "Jane Smith".to_string(),
            "456 Oak Ave".to_string(),
            "Paris".to_string(),
            "FR".to_string(),
            "75001".to_string(),
        )
        .unwrap();

        let enriched = enrich_payment_with_travel_data(
            "PAY-001".to_string(),
            5000.0,
            "EUR".to_string(),
            "ACC-002".to_string(),
            "TN".to_string(),
            "FR".to_string(),
            Some(orig),
            Some(bene),
        )
        .unwrap();

        assert_eq!(enriched.travel_rule_compliance, TravelRuleCompliance::Compliant);
        assert!(enriched.is_cross_border);
        assert!(enriched.travel_rule_data.is_some());
    }
}
