use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- InsurancePolicyId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InsurancePolicyId(Uuid);

impl InsurancePolicyId {
    pub fn new() -> Self {
        InsurancePolicyId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        InsurancePolicyId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(InsurancePolicyId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid policy ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InsurancePolicyId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InsurancePolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- PolicyType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolicyType {
    Life,
    Property,
    Credit,
    Health,
    Travel,
    CreditInsurance,
}

impl PolicyType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "life" => Ok(PolicyType::Life),
            "property" => Ok(PolicyType::Property),
            "credit" => Ok(PolicyType::Credit),
            "health" => Ok(PolicyType::Health),
            "travel" => Ok(PolicyType::Travel),
            "credit_insurance" => Ok(PolicyType::CreditInsurance),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown policy type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PolicyType::Life => "Life",
            PolicyType::Property => "Property",
            PolicyType::Credit => "Credit",
            PolicyType::Health => "Health",
            PolicyType::Travel => "Travel",
            PolicyType::CreditInsurance => "CreditInsurance",
        }
    }
}

impl fmt::Display for PolicyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- PremiumFrequency ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PremiumFrequency {
    Monthly,
    Quarterly,
    Annual,
}

impl PremiumFrequency {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "monthly" => Ok(PremiumFrequency::Monthly),
            "quarterly" => Ok(PremiumFrequency::Quarterly),
            "annual" => Ok(PremiumFrequency::Annual),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown premium frequency: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PremiumFrequency::Monthly => "Monthly",
            PremiumFrequency::Quarterly => "Quarterly",
            PremiumFrequency::Annual => "Annual",
        }
    }
}

impl fmt::Display for PremiumFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- PolicyStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolicyStatus {
    Proposal,
    Active,
    Suspended,
    Lapsed,
    Claimed,
    Terminated,
}

impl PolicyStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "proposal" => Ok(PolicyStatus::Proposal),
            "active" => Ok(PolicyStatus::Active),
            "suspended" => Ok(PolicyStatus::Suspended),
            "lapsed" => Ok(PolicyStatus::Lapsed),
            "claimed" => Ok(PolicyStatus::Claimed),
            "terminated" => Ok(PolicyStatus::Terminated),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown policy status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PolicyStatus::Proposal => "Proposal",
            PolicyStatus::Active => "Active",
            PolicyStatus::Suspended => "Suspended",
            PolicyStatus::Lapsed => "Lapsed",
            PolicyStatus::Claimed => "Claimed",
            PolicyStatus::Terminated => "Terminated",
        }
    }
}

impl fmt::Display for PolicyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- InsuranceClaimId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InsuranceClaimId(Uuid);

impl InsuranceClaimId {
    pub fn new() -> Self {
        InsuranceClaimId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        InsuranceClaimId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(InsuranceClaimId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid claim ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InsuranceClaimId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InsuranceClaimId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- ClaimStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClaimStatus {
    Filed,
    UnderReview,
    Approved,
    Rejected,
    Paid,
}

impl ClaimStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "filed" => Ok(ClaimStatus::Filed),
            "under_review" => Ok(ClaimStatus::UnderReview),
            "approved" => Ok(ClaimStatus::Approved),
            "rejected" => Ok(ClaimStatus::Rejected),
            "paid" => Ok(ClaimStatus::Paid),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown claim status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ClaimStatus::Filed => "Filed",
            ClaimStatus::UnderReview => "UnderReview",
            ClaimStatus::Approved => "Approved",
            ClaimStatus::Rejected => "Rejected",
            ClaimStatus::Paid => "Paid",
        }
    }
}

impl fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- BancassuranceProductId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BancassuranceProductId(Uuid);

impl BancassuranceProductId {
    pub fn new() -> Self {
        BancassuranceProductId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        BancassuranceProductId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(BancassuranceProductId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid product ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BancassuranceProductId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BancassuranceProductId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- LinkedProductType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkedProductType {
    Mortgage,
    AutoLoan,
    PersonalLoan,
    CreditCard,
    BusinessLoan,
}

impl LinkedProductType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "mortgage" => Ok(LinkedProductType::Mortgage),
            "auto_loan" => Ok(LinkedProductType::AutoLoan),
            "personal_loan" => Ok(LinkedProductType::PersonalLoan),
            "credit_card" => Ok(LinkedProductType::CreditCard),
            "business_loan" => Ok(LinkedProductType::BusinessLoan),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown linked product type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LinkedProductType::Mortgage => "Mortgage",
            LinkedProductType::AutoLoan => "AutoLoan",
            LinkedProductType::PersonalLoan => "PersonalLoan",
            LinkedProductType::CreditCard => "CreditCard",
            LinkedProductType::BusinessLoan => "BusinessLoan",
        }
    }
}

impl fmt::Display for LinkedProductType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- InsuranceCommissionId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InsuranceCommissionId(Uuid);

impl InsuranceCommissionId {
    pub fn new() -> Self {
        InsuranceCommissionId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        InsuranceCommissionId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(InsuranceCommissionId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid commission ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InsuranceCommissionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InsuranceCommissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- CommissionStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommissionStatus {
    Pending,
    Paid,
}

impl CommissionStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(CommissionStatus::Pending),
            "paid" => Ok(CommissionStatus::Paid),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown commission status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CommissionStatus::Pending => "Pending",
            CommissionStatus::Paid => "Paid",
        }
    }
}

impl fmt::Display for CommissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_type_from_str() {
        assert_eq!(PolicyType::from_str("life").unwrap(), PolicyType::Life);
        assert_eq!(PolicyType::from_str("property").unwrap(), PolicyType::Property);
        assert_eq!(PolicyType::from_str("credit").unwrap(), PolicyType::Credit);
    }

    #[test]
    fn test_premium_frequency_from_str() {
        assert_eq!(PremiumFrequency::from_str("monthly").unwrap(), PremiumFrequency::Monthly);
        assert_eq!(
            PremiumFrequency::from_str("quarterly").unwrap(),
            PremiumFrequency::Quarterly
        );
    }

    #[test]
    fn test_policy_status_from_str() {
        assert_eq!(PolicyStatus::from_str("active").unwrap(), PolicyStatus::Active);
        assert_eq!(PolicyStatus::from_str("suspended").unwrap(), PolicyStatus::Suspended);
    }

    #[test]
    fn test_claim_status_from_str() {
        assert_eq!(ClaimStatus::from_str("filed").unwrap(), ClaimStatus::Filed);
        assert_eq!(ClaimStatus::from_str("approved").unwrap(), ClaimStatus::Approved);
    }

    #[test]
    fn test_linked_product_type_from_str() {
        assert_eq!(LinkedProductType::from_str("mortgage").unwrap(), LinkedProductType::Mortgage);
        assert_eq!(
            LinkedProductType::from_str("auto_loan").unwrap(),
            LinkedProductType::AutoLoan
        );
    }

    #[test]
    fn test_insurance_policy_id_new() {
        let id = InsurancePolicyId::new();
        assert!(!id.as_uuid().to_string().is_empty());
