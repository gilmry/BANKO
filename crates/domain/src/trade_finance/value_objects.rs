use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- LetterOfCreditId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LetterOfCreditId(Uuid);

impl LetterOfCreditId {
    pub fn new() -> Self {
        LetterOfCreditId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        LetterOfCreditId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(LetterOfCreditId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid LC ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for LetterOfCreditId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LetterOfCreditId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- LCType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LCType {
    Import,
    Export,
    Standby,
    Transferable,
}

impl LCType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "import" => Ok(LCType::Import),
            "export" => Ok(LCType::Export),
            "standby" => Ok(LCType::Standby),
            "transferable" => Ok(LCType::Transferable),
            _ => Err(DomainError::ValidationError(format!("Unknown LC type: {s}"))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LCType::Import => "Import",
            LCType::Export => "Export",
            LCType::Standby => "Standby",
            LCType::Transferable => "Transferable",
        }
    }
}

impl fmt::Display for LCType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- LCStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LCStatus {
    Draft,
    Issued,
    Amended,
    Utilized,
    Expired,
    Cancelled,
}

impl LCStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(LCStatus::Draft),
            "issued" => Ok(LCStatus::Issued),
            "amended" => Ok(LCStatus::Amended),
            "utilized" => Ok(LCStatus::Utilized),
            "expired" => Ok(LCStatus::Expired),
            "cancelled" => Ok(LCStatus::Cancelled),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown LC status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LCStatus::Draft => "Draft",
            LCStatus::Issued => "Issued",
            LCStatus::Amended => "Amended",
            LCStatus::Utilized => "Utilized",
            LCStatus::Expired => "Expired",
            LCStatus::Cancelled => "Cancelled",
        }
    }
}

impl fmt::Display for LCStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- BankGuaranteeId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BankGuaranteeId(Uuid);

impl BankGuaranteeId {
    pub fn new() -> Self {
        BankGuaranteeId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        BankGuaranteeId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(BankGuaranteeId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid guarantee ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BankGuaranteeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BankGuaranteeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- GuaranteeType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GuaranteeType {
    Performance,
    Payment,
    BidBond,
    AdvancePayment,
    Customs,
}

impl GuaranteeType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "performance" => Ok(GuaranteeType::Performance),
            "payment" => Ok(GuaranteeType::Payment),
            "bid_bond" => Ok(GuaranteeType::BidBond),
            "advance_payment" => Ok(GuaranteeType::AdvancePayment),
            "customs" => Ok(GuaranteeType::Customs),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown guarantee type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            GuaranteeType::Performance => "Performance",
            GuaranteeType::Payment => "Payment",
            GuaranteeType::BidBond => "BidBond",
            GuaranteeType::AdvancePayment => "AdvancePayment",
            GuaranteeType::Customs => "Customs",
        }
    }
}

impl fmt::Display for GuaranteeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- GuaranteeStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GuaranteeStatus {
    Active,
    Called,
    Released,
    Expired,
}

impl GuaranteeStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "active" => Ok(GuaranteeStatus::Active),
            "called" => Ok(GuaranteeStatus::Called),
            "released" => Ok(GuaranteeStatus::Released),
            "expired" => Ok(GuaranteeStatus::Expired),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown guarantee status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            GuaranteeStatus::Active => "Active",
            GuaranteeStatus::Called => "Called",
            GuaranteeStatus::Released => "Released",
            GuaranteeStatus::Expired => "Expired",
        }
    }
}

impl fmt::Display for GuaranteeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- DocumentaryCollectionId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentaryCollectionId(Uuid);

impl DocumentaryCollectionId {
    pub fn new() -> Self {
        DocumentaryCollectionId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DocumentaryCollectionId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(DocumentaryCollectionId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid collection ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DocumentaryCollectionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DocumentaryCollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- CollectionType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollectionType {
    DocumentsAgainstPayment,
    DocumentsAgainstAcceptance,
}

impl CollectionType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "documents_against_payment" => Ok(CollectionType::DocumentsAgainstPayment),
            "documents_against_acceptance" => Ok(CollectionType::DocumentsAgainstAcceptance),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown collection type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CollectionType::DocumentsAgainstPayment => "DocumentsAgainstPayment",
            CollectionType::DocumentsAgainstAcceptance => "DocumentsAgainstAcceptance",
        }
    }
}

impl fmt::Display for CollectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CollectionStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollectionStatus {
    Received,
    Presented,
    Accepted,
    Paid,
    Protested,
}

impl CollectionStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "received" => Ok(CollectionStatus::Received),
            "presented" => Ok(CollectionStatus::Presented),
            "accepted" => Ok(CollectionStatus::Accepted),
            "paid" => Ok(CollectionStatus::Paid),
            "protested" => Ok(CollectionStatus::Protested),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown collection status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CollectionStatus::Received => "Received",
            CollectionStatus::Presented => "Presented",
            CollectionStatus::Accepted => "Accepted",
            CollectionStatus::Paid => "Paid",
            CollectionStatus::Protested => "Protested",
        }
    }
}

impl fmt::Display for CollectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- TradeFinanceLimitId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeFinanceLimitId(Uuid);

impl TradeFinanceLimitId {
    pub fn new() -> Self {
        TradeFinanceLimitId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        TradeFinanceLimitId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(TradeFinanceLimitId)
            .map_err(|_| DomainError::InvalidId(format!("Invalid limit ID: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TradeFinanceLimitId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TradeFinanceLimitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- LimitType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LimitType {
    LC,
    Guarantee,
    Collection,
}

impl LimitType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "lc" => Ok(LimitType::LC),
            "guarantee" => Ok(LimitType::Guarantee),
            "collection" => Ok(LimitType::Collection),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown limit type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LimitType::LC => "LC",
            LimitType::Guarantee => "Guarantee",
            LimitType::Collection => "Collection",
        }
    }
}

impl fmt::Display for LimitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lc_type_from_str() {
        assert_eq!(LCType::from_str("import").unwrap(), LCType::Import);
        assert_eq!(LCType::from_str("export").unwrap(), LCType::Export);
        assert_eq!(LCType::from_str("standby").unwrap(), LCType::Standby);
        assert_eq!(LCType::from_str("transferable").unwrap(), LCType::Transferable);
    }

    #[test]
    fn test_lc_status_from_str() {
        assert_eq!(LCStatus::from_str("draft").unwrap(), LCStatus::Draft);
        assert_eq!(LCStatus::from_str("issued").unwrap(), LCStatus::Issued);
        assert_eq!(LCStatus::from_str("amended").unwrap(), LCStatus::Amended);
    }

    #[test]
    fn test_guarantee_type_from_str() {
        assert_eq!(GuaranteeType::from_str("performance").unwrap(), GuaranteeType::Performance);
        assert_eq!(GuaranteeType::from_str("payment").unwrap(), GuaranteeType::Payment);
    }

    #[test]
    fn test_collection_type_from_str() {
        assert_eq!(
            CollectionType::from_str("documents_against_payment").unwrap(),
            CollectionType::DocumentsAgainstPayment
        );
    }

    #[test]
    fn test_letter_of_credit_id_new() {
        let id = LetterOfCreditId::new();
        as