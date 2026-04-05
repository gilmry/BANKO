use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- LoanId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoanId(Uuid);

impl LoanId {
    pub fn new() -> Self {
        LoanId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        LoanId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(LoanId)
            .map_err(|_| DomainError::LoanNotFound)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for LoanId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LoanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- InstallmentId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstallmentId(Uuid);

impl InstallmentId {
    pub fn new() -> Self {
        InstallmentId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        InstallmentId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InstallmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InstallmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- LoanStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoanStatus {
    Applied,
    Approved,
    Disbursed,
    Active,
    Closed,
    Defaulted,
}

impl LoanStatus {
    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "applied" => Ok(LoanStatus::Applied),
            "approved" => Ok(LoanStatus::Approved),
            "disbursed" => Ok(LoanStatus::Disbursed),
            "active" => Ok(LoanStatus::Active),
            "closed" => Ok(LoanStatus::Closed),
            "defaulted" => Ok(LoanStatus::Defaulted),
            _ => Err(DomainError::InvalidLoanStatus(format!(
                "Unknown loan status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LoanStatus::Applied => "Applied",
            LoanStatus::Approved => "Approved",
            LoanStatus::Disbursed => "Disbursed",
            LoanStatus::Active => "Active",
            LoanStatus::Closed => "Closed",
            LoanStatus::Defaulted => "Defaulted",
        }
    }
}

impl fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- AssetClass (INV-06) ---
// Circ. 91-24 art. 8 [REF-14]: Classification des créances

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AssetClass {
    Class0, // Standard (0 jours de retard)
    Class1, // Actifs nécessitant un suivi particulier (1-90 jours)
    Class2, // Actifs incertains (91-180 jours)
    Class3, // Actifs préoccupants (181-360 jours)
    Class4, // Actifs compromis (>360 jours)
}

impl AssetClass {
    /// Classification automatique basée sur les jours de retard.
    /// Circ. 91-24 art. 8 [REF-14]
    pub fn from_days_past_due(days: u32) -> Self {
        match days {
            0..=90 if days == 0 => AssetClass::Class0,
            1..=90 => AssetClass::Class1,
            91..=180 => AssetClass::Class2,
            181..=360 => AssetClass::Class3,
            _ => AssetClass::Class4,
        }
    }

    /// Taux de provisionnement minimum réglementaire.
    /// Circ. 91-24 [REF-14] + Circ. 2023-02 [REF-24]
    /// INV-07: Classe 2 ≥ 20%, Classe 3 ≥ 50%, Classe 4 = 100%
    pub fn min_provision_pct(&self) -> f64 {
        match self {
            AssetClass::Class0 => 0.0,
            AssetClass::Class1 => 0.0,
            AssetClass::Class2 => 0.20,
            AssetClass::Class3 => 0.50,
            AssetClass::Class4 => 1.00,
        }
    }

    pub fn from_str_class(s: &str) -> Result<Self, DomainError> {
        match s {
            "0" | "Class0" => Ok(AssetClass::Class0),
            "1" | "Class1" => Ok(AssetClass::Class1),
            "2" | "Class2" => Ok(AssetClass::Class2),
            "3" | "Class3" => Ok(AssetClass::Class3),
            "4" | "Class4" => Ok(AssetClass::Class4),
            _ => Err(DomainError::InvalidAssetClass(format!(
                "Unknown asset class: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AssetClass::Class0 => "0",
            AssetClass::Class1 => "1",
            AssetClass::Class2 => "2",
            AssetClass::Class3 => "3",
            AssetClass::Class4 => "4",
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            AssetClass::Class0 => 0,
            AssetClass::Class1 => 1,
            AssetClass::Class2 => 2,
            AssetClass::Class3 => 3,
            AssetClass::Class4 => 4,
        }
    }

    pub fn from_i32(v: i32) -> Result<Self, DomainError> {
        match v {
            0 => Ok(AssetClass::Class0),
            1 => Ok(AssetClass::Class1),
            2 => Ok(AssetClass::Class2),
            3 => Ok(AssetClass::Class3),
            4 => Ok(AssetClass::Class4),
            _ => Err(DomainError::InvalidAssetClass(format!(
                "Asset class must be 0-4, got {v}"
            ))),
        }
    }
}

impl fmt::Display for AssetClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class {}", self.as_str())
    }
}

// --- PaymentFrequency ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentFrequency {
    Monthly,
    Quarterly,
}

impl PaymentFrequency {
    pub fn from_str_freq(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "monthly" => Ok(PaymentFrequency::Monthly),
            "quarterly" => Ok(PaymentFrequency::Quarterly),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown frequency: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PaymentFrequency::Monthly => "Monthly",
            PaymentFrequency::Quarterly => "Quarterly",
        }
    }

    pub fn periods_per_year(&self) -> u32 {
        match self {
            PaymentFrequency::Monthly => 12,
            PaymentFrequency::Quarterly => 4,
        }
    }
}

impl fmt::Display for PaymentFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- AmortizationType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmortizationType {
    Linear,   // Principal constant, intérêt décroissant
    Constant, // Paiement constant (annuité constante)
}

impl AmortizationType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "linear" => Ok(AmortizationType::Linear),
            "constant" => Ok(AmortizationType::Constant),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown amortization type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AmortizationType::Linear => "Linear",
            AmortizationType::Constant => "Constant",
        }
    }
}

impl fmt::Display for AmortizationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_id_new() {
        let id = LoanId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_loan_id_parse_valid() {
        let id = LoanId::parse("550e8400-e29b-41d4-a716-446655440000");
        assert!(id.is_ok());
    }

    #[test]
    fn test_loan_id_parse_invalid() {
        assert!(LoanId::parse("not-a-uuid").is_err());
    }

    #[test]
    fn test_loan_status_from_str() {
        assert_eq!(
            LoanStatus::from_str_status("Applied").unwrap(),
            LoanStatus::Applied
        );
        assert_eq!(
            LoanStatus::from_str_status("approved").unwrap(),
            LoanStatus::Approved
        );
        assert_eq!(
            LoanStatus::from_str_status("disbursed").unwrap(),
            LoanStatus::Disbursed
        );
        assert_eq!(
            LoanStatus::from_str_status("active").unwrap(),
            LoanStatus::Active
        );
        assert_eq!(
            LoanStatus::from_str_status("closed").unwrap(),
            LoanStatus::Closed
        );
        assert_eq!(
            LoanStatus::from_str_status("defaulted").unwrap(),
            LoanStatus::Defaulted
        );
    }

    #[test]
    fn test_loan_status_from_str_invalid() {
        assert!(LoanStatus::from_str_status("unknown").is_err());
    }

    // --- AssetClass classification tests (Circ. 91-24) ---

    #[test]
    fn test_asset_class_from_days_past_due_class0() {
        assert_eq!(AssetClass::from_days_past_due(0), AssetClass::Class0);
    }

    #[test]
    fn test_asset_class_from_days_past_due_class1() {
        assert_eq!(AssetClass::from_days_past_due(1), AssetClass::Class1);
        assert_eq!(AssetClass::from_days_past_due(31), AssetClass::Class1);
        assert_eq!(AssetClass::from_days_past_due(90), AssetClass::Class1);
    }

    #[test]
    fn test_asset_class_from_days_past_due_class2() {
        assert_eq!(AssetClass::from_days_past_due(91), AssetClass::Class2);
        assert_eq!(AssetClass::from_days_past_due(180), AssetClass::Class2);
    }

    #[test]
    fn test_asset_class_from_days_past_due_class3() {
        assert_eq!(AssetClass::from_days_past_due(181), AssetClass::Class3);
        assert_eq!(AssetClass::from_days_past_due(360), AssetClass::Class3);
    }

    #[test]
    fn test_asset_class_from_days_past_due_class4() {
        assert_eq!(AssetClass::from_days_past_due(361), AssetClass::Class4);
        assert_eq!(AssetClass::from_days_past_due(365), AssetClass::Class4);
        assert_eq!(AssetClass::from_days_past_due(1000), AssetClass::Class4);
    }

    // --- Provision percentage tests (INV-07) ---

    #[test]
    fn test_min_provision_pct_class0() {
        assert_eq!(AssetClass::Class0.min_provision_pct(), 0.0);
    }

    #[test]
    fn test_min_provision_pct_class1() {
        assert_eq!(AssetClass::Class1.min_provision_pct(), 0.0);
    }

    #[test]
    fn test_min_provision_pct_class2() {
        // INV-07: Classe 2 ≥ 20%
        assert_eq!(AssetClass::Class2.min_provision_pct(), 0.20);
    }

    #[test]
    fn test_min_provision_pct_class3() {
        // INV-07: Classe 3 ≥ 50%
        assert_eq!(AssetClass::Class3.min_provision_pct(), 0.50);
    }

    #[test]
    fn test_min_provision_pct_class4() {
        assert_eq!(AssetClass::Class4.min_provision_pct(), 1.00);
    }

    #[test]
    fn test_asset_class_from_str() {
        assert_eq!(AssetClass::from_str_class("0").unwrap(), AssetClass::Class0);
        assert_eq!(
            AssetClass::from_str_class("Class2").unwrap(),
            AssetClass::Class2
        );
        assert!(AssetClass::from_str_class("5").is_err());
    }

    #[test]
    fn test_asset_class_from_i32() {
        assert_eq!(AssetClass::from_i32(0).unwrap(), AssetClass::Class0);
        assert_eq!(AssetClass::from_i32(4).unwrap(), AssetClass::Class4);
        assert!(AssetClass::from_i32(5).is_err());
        assert!(AssetClass::from_i32(-1).is_err());
    }

    #[test]
    fn test_payment_frequency() {
        assert_eq!(PaymentFrequency::Monthly.periods_per_year(), 12);
        assert_eq!(PaymentFrequency::Quarterly.periods_per_year(), 4);
        assert_eq!(
            PaymentFrequency::from_str_freq("monthly").unwrap(),
            PaymentFrequency::Monthly
        );
    }

    #[test]
    fn test_amortization_type() {
        assert_eq!(
            AmortizationType::from_str_type("linear").unwrap(),
            AmortizationType::Linear
        );
        assert_eq!(
            AmortizationType::from_str_type("constant").unwrap(),
            AmortizationType::Constant
        );
        assert!(AmortizationType::from_str_type("unknown").is_err());
    }
}
