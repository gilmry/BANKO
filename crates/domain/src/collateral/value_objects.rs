use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- CollateralId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollateralId(Uuid);

impl CollateralId {
    pub fn new() -> Self {
        CollateralId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        CollateralId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(CollateralId)
            .map_err(|_| DomainError::CollateralNotFound)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CollateralId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CollateralId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- CollateralType enum ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollateralType {
    RealEstate,
    FinancialDeposit,
    Securities,
    PersonalGuarantee,
    CorporateGuarantee,
    Equipment,
    Inventory,
    Other,
}

impl CollateralType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "real_estate" => Ok(CollateralType::RealEstate),
            "financial_deposit" => Ok(CollateralType::FinancialDeposit),
            "securities" => Ok(CollateralType::Securities),
            "personal_guarantee" => Ok(CollateralType::PersonalGuarantee),
            "corporate_guarantee" => Ok(CollateralType::CorporateGuarantee),
            "equipment" => Ok(CollateralType::Equipment),
            "inventory" => Ok(CollateralType::Inventory),
            "other" => Ok(CollateralType::Other),
            _ => Err(DomainError::InvalidCollateralType(format!(
                "Unknown collateral type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CollateralType::RealEstate => "RealEstate",
            CollateralType::FinancialDeposit => "FinancialDeposit",
            CollateralType::Securities => "Securities",
            CollateralType::PersonalGuarantee => "PersonalGuarantee",
            CollateralType::CorporateGuarantee => "CorporateGuarantee",
            CollateralType::Equipment => "Equipment",
            CollateralType::Inventory => "Inventory",
            CollateralType::Other => "Other",
        }
    }

    /// Returns the default haircut percentage for this collateral type (regulatory).
    /// BCT Circ. 91-24 compliant haircuts.
    pub fn default_haircut_pct(&self) -> f64 {
        match self {
            CollateralType::RealEstate => 0.0,          // No haircut, separate LTV control
            CollateralType::FinancialDeposit => 0.02,   // 2% (conservative)
            CollateralType::Securities => 0.35,         // 35% average
            CollateralType::PersonalGuarantee => 0.5,   // 50% (not regulatory eligible)
            CollateralType::CorporateGuarantee => 0.25, // 25%
            CollateralType::Equipment => 0.40,          // 40% depreciation
            CollateralType::Inventory => 0.45,          // 45% (volatile)
            CollateralType::Other => 0.50,              // 50% (conservative default)
        }
    }

    /// Maximum LTV allowed for this collateral type (BCT Circ. 91-24).
    pub fn max_ltv_pct(&self) -> f64 {
        match self {
            CollateralType::RealEstate => 0.70,         // 70% max LTV
            CollateralType::FinancialDeposit => 0.95,   // 95% max (cash backed)
            CollateralType::Securities => 0.70,         // 70% max
            CollateralType::PersonalGuarantee => 0.0,   // Not eligible for LTV relief
            CollateralType::CorporateGuarantee => 0.80, // 80% max
            CollateralType::Equipment => 0.60,          // 60% max
            CollateralType::Inventory => 0.50,          // 50% max
            CollateralType::Other => 0.50,              // 50% conservative
        }
    }

    /// Revaluation frequency in months.
    pub fn revaluation_frequency_months(&self) -> u32 {
        match self {
            CollateralType::RealEstate => 12,           // Annual revaluation
            CollateralType::FinancialDeposit => 1,      // Monthly (if floating)
            CollateralType::Securities => 3,            // Quarterly
            CollateralType::PersonalGuarantee => 6,     // Semi-annual
            CollateralType::CorporateGuarantee => 6,    // Semi-annual
            CollateralType::Equipment => 6,             // Semi-annual (depreciation)
            CollateralType::Inventory => 3,             // Quarterly (volatile)
            CollateralType::Other => 6,                 // Conservative default
        }
    }

    /// Whether insurance is mandatory for this collateral type.
    pub fn insurance_mandatory(&self) -> bool {
        match self {
            CollateralType::RealEstate => true,
            _ => false,
        }
    }
}

impl fmt::Display for CollateralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CollateralStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollateralStatus {
    Pending,   // Under appraisal/setup
    Active,    // Linked to loan(s)
    Released,  // No longer securing loans
    Impaired,  // Value impaired (requires revaluation)
}

impl CollateralStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(CollateralStatus::Pending),
            "active" => Ok(CollateralStatus::Active),
            "released" => Ok(CollateralStatus::Released),
            "impaired" => Ok(CollateralStatus::Impaired),
            _ => Err(DomainError::InvalidCollateralStatus(format!(
                "Unknown collateral status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CollateralStatus::Pending => "Pending",
            CollateralStatus::Active => "Active",
            CollateralStatus::Released => "Released",
            CollateralStatus::Impaired => "Impaired",
        }
    }
}

impl fmt::Display for CollateralStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ValuationMethod ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValuationMethod {
    MarketComparison,
    DiscountedCashFlow,
    CostApproach,
    IndexBased,
}

impl ValuationMethod {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "market_comparison" => Ok(ValuationMethod::MarketComparison),
            "discounted_cash_flow" => Ok(ValuationMethod::DiscountedCashFlow),
            "cost_approach" => Ok(ValuationMethod::CostApproach),
            "index_based" => Ok(ValuationMethod::IndexBased),
            _ => Err(DomainError::InvalidValuationMethod(format!(
                "Unknown valuation method: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ValuationMethod::MarketComparison => "MarketComparison",
            ValuationMethod::DiscountedCashFlow => "DiscountedCashFlow",
            ValuationMethod::CostApproach => "CostApproach",
            ValuationMethod::IndexBased => "IndexBased",
        }
    }
}

impl fmt::Display for ValuationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collateral_id_new() {
        let id = CollateralId::new();
        assert_ne!(id, CollateralId::new());
    }

    #[test]
    fn test_collateral_id_parse() {
        let id = CollateralId::new();
        let id_str = id.to_string();
        let parsed = CollateralId::parse(&id_str).unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn test_collateral_type_from_str() {
        assert_eq!(
            CollateralType::from_str("real_estate").unwrap(),
            CollateralType::RealEstate
        );
        assert_eq!(
            CollateralType::from_str("securities").unwrap(),
            CollateralType::Securities
        );
        assert!(CollateralType::from_str("invalid").is_err());
    }

    #[test]
    fn test_collateral_type_haircuts() {
        assert_eq!(CollateralType::RealEstate.default_haircut_pct(), 0.0);
        assert_eq!(
            CollateralType::FinancialDeposit.default_haircut_pct(),
            0.02
        );
        assert_eq!(CollateralType::Securities.default_haircut_pct(), 0.35);
    }

    #[test]
    fn test_collateral_type_max_ltv() {
        assert_eq!(CollateralType::RealEstate.max_ltv_pct(), 0.70);
        assert_eq!(CollateralType::FinancialDeposit.max_ltv_pct(), 0.95);
        assert_eq!(CollateralType::Securities.max_ltv_pct(), 0.70);
    }

    #[test]
    fn test_collateral_type_revaluation_frequency() {
        assert_eq!(CollateralType::RealEstate.revaluation_frequency_months(), 12);
        assert_eq!(
            CollateralType::Securities.revaluation_frequency_months(),
            3
        );
        assert_eq!(
            CollateralType::FinancialDeposit.revaluation_frequency_months(),
            1
        );
    }

    #[test]
    fn test_collateral_type_insurance_mandatory() {
        assert!(CollateralType::RealEstate.insurance_mandatory());
        assert!(!CollateralType::Securities.insurance_mandatory());
        assert!(!CollateralType::FinancialDeposit.insurance_mandatory());
    }

    #[test]
    fn test_collateral_status_from_str() {
        assert_eq!(
            CollateralStatus::from_str("active").unwrap(),
            CollateralStatus::Active
        );
        assert_eq!(
            CollateralStatus::from_str("pending").unwrap(),
            CollateralStatus::Pending
        );
        assert!(CollateralStatus::from_str("invalid").is_err());
    }

    #[test]
    fn test_valuation_method_from_str() {
        assert_eq!(
            ValuationMethod: