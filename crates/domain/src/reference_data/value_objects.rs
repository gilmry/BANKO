use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- ReferenceDataId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReferenceDataId(Uuid);

impl ReferenceDataId {
    pub fn new() -> Self {
        ReferenceDataId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ReferenceDataId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(ReferenceDataId)
            .map_err(|_| DomainError::ValidationError("Invalid ReferenceDataId UUID".to_string()))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ReferenceDataId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReferenceDataId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- CountryCode value object ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CountryCodeVo {
    iso_alpha2: String, // "TN", "FR", "US"
    iso_alpha3: String, // "TUN", "FRA", "USA"
    iso_numeric: String, // "788", "250", "840"
}

impl CountryCodeVo {
    pub fn new(iso_alpha2: &str, iso_alpha3: &str, iso_numeric: &str) -> Result<Self, DomainError> {
        if iso_alpha2.len() != 2 {
            return Err(DomainError::ValidationError(
                "ISO Alpha-2 code must be exactly 2 characters".to_string(),
            ));
        }
        if iso_alpha3.len() != 3 {
            return Err(DomainError::ValidationError(
                "ISO Alpha-3 code must be exactly 3 characters".to_string(),
            ));
        }
        if iso_numeric.len() != 3 {
            return Err(DomainError::ValidationError(
                "ISO Numeric code must be exactly 3 characters".to_string(),
            ));
        }
        if !iso_numeric.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::ValidationError(
                "ISO Numeric code must contain only digits".to_string(),
            ));
        }

        Ok(CountryCodeVo {
            iso_alpha2: iso_alpha2.to_uppercase(),
            iso_alpha3: iso_alpha3.to_uppercase(),
            iso_numeric: iso_numeric.to_string(),
        })
    }

    pub fn iso_alpha2(&self) -> &str {
        &self.iso_alpha2
    }

    pub fn iso_alpha3(&self) -> &str {
        &self.iso_alpha3
    }

    pub fn iso_numeric(&self) -> &str {
        &self.iso_numeric
    }
}

// --- CurrencyCode value object ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyCodeVo {
    code: String, // "USD", "EUR", "TND"
}

impl CurrencyCodeVo {
    pub fn new(code: &str) -> Result<Self, DomainError> {
        if code.len() != 3 {
            return Err(DomainError::ValidationError(
                "Currency code must be exactly 3 characters (ISO 4217)".to_string(),
            ));
        }
        if !code.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(DomainError::ValidationError(
                "Currency code must contain only uppercase ASCII characters".to_string(),
            ));
        }

        Ok(CurrencyCodeVo {
            code: code.to_string(),
        })
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for CurrencyCodeVo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

// --- BicCode value object ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BicCodeVo {
    code: String, // SWIFT/BIC code, 8 or 11 characters
}

impl BicCodeVo {
    pub fn new(code: &str) -> Result<Self, DomainError> {
        let clean_code = code.to_uppercase().replace("-", "");
        if clean_code.len() != 8 && clean_code.len() != 11 {
            return Err(DomainError::ValidationError(
                "BIC code must be 8 or 11 characters (excluding hyphens)".to_string(),
            ));
        }
        if !clean_code.chars().take(6).all(|c| c.is_ascii_alphabetic()) {
            return Err(DomainError::ValidationError(
                "BIC code: first 6 characters must be alphabetic".to_string(),
            ));
        }

        Ok(BicCodeVo { code: clean_code })
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl fmt::Display for BicCodeVo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

// --- Fee Type enumeration ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeeType {
    AccountMaintenance,
    Transaction,
    Transfer,
    ForeignExchange,
    LatePayment,
    Overdraft,
    ATMWithdrawal,
    CheckIssue,
}

impl FeeType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "accountmaintenance" | "account_maintenance" => Ok(FeeType::AccountMaintenance),
            "transaction" => Ok(FeeType::Transaction),
            "transfer" => Ok(FeeType::Transfer),
            "foreignexchange" | "foreign_exchange" | "fx" => Ok(FeeType::ForeignExchange),
            "latepayment" | "late_payment" => Ok(FeeType::LatePayment),
            "overdraft" => Ok(FeeType::Overdraft),
            "atmwithdrawal" | "atm_withdrawal" => Ok(FeeType::ATMWithdrawal),
            "checkissue" | "check_issue" => Ok(FeeType::CheckIssue),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown fee type: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FeeType::AccountMaintenance => "AccountMaintenance",
            FeeType::Transaction => "Transaction",
            FeeType::Transfer => "Transfer",
            FeeType::ForeignExchange => "ForeignExchange",
            FeeType::LatePayment => "LatePayment",
            FeeType::Overdraft => "Overdraft",
            FeeType::ATMWithdrawal => "ATMWithdrawal",
            FeeType::CheckIssue => "CheckIssue",
        }
    }
}

impl fmt::Display for FeeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- RegulatoryClassification enumeration ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulatoryClassification {
    // BCT (Central Bank of Tunisia) asset classes
    StandardRisk,
    LowerRisk,
    HigherRisk,
    // IFRS Categories
    AmortizedCost,
    FairValueThroughOci,
    FairValueThroughPl,
}

impl RegulatoryClassification {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "standardrisk" | "standard_risk" => Ok(RegulatoryClassification::StandardRisk),
            "lowerrisk" | "lower_risk" => Ok(RegulatoryClassification::LowerRisk),
            "higherrisk" | "higher_risk" => Ok(RegulatoryClassification::HigherRisk),
            "amortizedcost" | "amortized_cost" => Ok(RegulatoryClassification::AmortizedCost),
            "fairvaluethroughoci" | "fair_value_through_oci" => {
                Ok(RegulatoryClassification::FairValueThroughOci)
            }
            "fairvaluethroughpl" | "fair_value_through_pl" => {
                Ok(RegulatoryClassification::FairValueThroughPl)
            }
            _ => Err(DomainError::ValidationError(format!(
                "Unknown regulatory classification: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RegulatoryClassification::StandardRisk => "StandardRisk",
            RegulatoryClassification::LowerRisk => "LowerRisk",
            RegulatoryClassification::HigherRisk => "HigherRisk",
            RegulatoryClassification::AmortizedCost => "AmortizedCost",
            RegulatoryClassification::FairValueThroughOci => "FairValueThroughOci",
            RegulatoryClassification::FairValueThroughPl => "FairValueThroughPl",
        }
    }
}

impl fmt::Display for RegulatoryClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- HolidayType enumeration ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HolidayType {
    National,
    Banking,
    Religious,
}

impl HolidayType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "national" => Ok(HolidayType::National),
            "banking" => Ok(HolidayType::Banking),
            "religious" => Ok(HolidayType::Religious),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown holiday type: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            HolidayType::National => "National",
            HolidayType::Banking => "Banking",
            HolidayType::Religious => "Religious",
        }
    }
}

impl fmt::Display for HolidayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SystemParameterType enumeration ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemParameterType {
    Integer,
    Decimal,
    String,
    Boolean,
}

impl SystemParameterType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "integer" => Ok(SystemParameterType::Integer),
            "decimal" => Ok(SystemParameterType::Decimal),
            "string" => Ok(SystemParameterType::String),
            "boolean" => Ok(SystemParameterType::Boolean),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown parameter type: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SystemParameterType::Integer => "Integer",
            SystemParameterType::Decimal => "Decimal",
            SystemParameterType::String => "String",
            SystemParameterType::Boolean => "Boolean",
        }
    }
}

impl fmt::Display for SystemParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_data_id_new() {
        let id = ReferenceDataId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_country_code_valid() {
        let code = CountryCodeVo::new("TN", "TUN", "788");
        assert!(code.is_ok());
        let code = code.unwrap();
        assert_eq!(code.iso_alpha2(), "TN");
        assert_eq!(code.iso_alpha3(), "TUN");
        assert_eq!(code.iso_numeric(), "788");
    }

    #[test]
    fn test_country_code_invalid_alpha2() {
        assert!(CountryCodeVo::new("TUN", "TUN", "788").is_err());
    }

    #[test]
    fn test_country_code_invalid_alpha3() {
        assert!(CountryCodeVo::new("TN", "TN", "788").is_err());
    }

    #[test]
    fn test_country_code_invalid_numeric() {
        assert!(CountryCodeVo::new("TN", "TUN", "78").is_err());
        assert!(CountryCodeVo::new("TN", "TUN", "ABC").is_err());
    }

    #[test]
    fn test_currency_code_valid() {
        let code = CurrencyCodeVo::new("USD");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().code(), "USD");
    }

    #[test]
    fn test_currency_code_invalid_length() {
        assert!(CurrencyCodeVo::new("US").is_err());
        assert!(CurrencyCodeVo::new("USDA").is_err());
    }

    #[test]
    fn test_currency_code_invalid_case() {
        assert!(CurrencyCodeVo::new("usd").is_err());
    }

    #[test]
    fn test_bic_code_valid_8_chars() {
        let code = BicCodeVo::new("BNAFFRPP");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().code(), "BNAFFRPP");
    }

    #[test]
    fn test_bic_code_valid_11_chars() {
        let code = BicCodeVo::new("BNAFFRPPXXX");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().code(), "BNAFFRPPXXX");
    }

    #[test]
    fn test_bic_code_with_hyphens() {
        let code = BicCodeVo::new("BNA-FFR-PP");
        assert!(code.is_ok());
        assert_eq!(code.unwrap().code(), "BNAFFRPP");
    }

    #[test]
    fn test_bic_code_invalid_length() {
        assert!(BicCodeVo::new("BNAFR").is_err());
    }

    #[test]
    fn test_fee_type_from_str() {
        assert_eq!(
            FeeType::from_str("AccountMaintenance").unwrap(),
            FeeType::AccountMaintenance
        );
        assert_eq!(FeeType::from_str("transaction").unwrap(), FeeType::Transaction);
        assert_eq!(FeeType::from_str("fx").unwrap(), FeeType::ForeignExchange);
    }

    #[test]
    fn test_regulatory_classification_from_str() {
        assert_eq!(
            RegulatoryClassification::from_str("StandardRisk").unwrap(),
            RegulatoryClassification::StandardRisk
        );
        assert_eq!(
            RegulatoryClassification::from_str("amortized_cost").unwrap(),
            RegulatoryClassification::AmortizedCost
        );
    }

    #[test]
    fn test_holiday_type_from_str() {
        assert_eq!(
            HolidayType::from_str("National").unwrap(),
            HolidayType::National
        );
        assert_eq!(
            HolidayType::from_str("banking").unwrap(),
            HolidayType::Banking
        );
    }

    #[test]
    fn test_system_parameter_type_from_str() {
        assert_eq!(
            SystemParameterType::from_str("Integer").unwrap(),
            SystemParameterType::Integer
        );
        assert_eq!(
            SystemParameterType::from_str("decimal").unwrap(),
            SystemParameterType::Decimal
        );
    }
}
