use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- SecuritiesAccountId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecuritiesAccountId(Uuid);

impl SecuritiesAccountId {
    pub fn new() -> Self {
        SecuritiesAccountId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        SecuritiesAccountId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(SecuritiesAccountId)
            .map_err(|_| DomainError::ValidationError("Invalid securities account ID".to_string()))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SecuritiesAccountId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SecuritiesAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- TradeOrderId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeOrderId(Uuid);

impl TradeOrderId {
    pub fn new() -> Self {
        TradeOrderId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        TradeOrderId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(TradeOrderId)
            .map_err(|_| DomainError::ValidationError(format!("Invalid TradeOrderId: {s}")))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TradeOrderId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TradeOrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- SettlementId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SettlementId(Uuid);

impl SettlementId {
    pub fn new() -> Self {
        SettlementId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        SettlementId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SettlementId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettlementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- CorporateActionId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CorporateActionId(Uuid);

impl CorporateActionId {
    pub fn new() -> Self {
        CorporateActionId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        CorporateActionId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CorporateActionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CorporateActionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- SecuritiesAccountType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecuritiesAccountType {
    Individual,
    Joint,
    Corporate,
    Nominee,
}

impl SecuritiesAccountType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "individual" => Ok(SecuritiesAccountType::Individual),
            "joint" => Ok(SecuritiesAccountType::Joint),
            "corporate" => Ok(SecuritiesAccountType::Corporate),
            "nominee" => Ok(SecuritiesAccountType::Nominee),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown securities account type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SecuritiesAccountType::Individual => "Individual",
            SecuritiesAccountType::Joint => "Joint",
            SecuritiesAccountType::Corporate => "Corporate",
            SecuritiesAccountType::Nominee => "Nominee",
        }
    }
}

impl fmt::Display for SecuritiesAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SecuritiesAccountStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecuritiesAccountStatus {
    Active,
    Suspended,
    Closed,
}

impl SecuritiesAccountStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "active" => Ok(SecuritiesAccountStatus::Active),
            "suspended" => Ok(SecuritiesAccountStatus::Suspended),
            "closed" => Ok(SecuritiesAccountStatus::Closed),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown securities account status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SecuritiesAccountStatus::Active => "Active",
            SecuritiesAccountStatus::Suspended => "Suspended",
            SecuritiesAccountStatus::Closed => "Closed",
        }
    }
}

impl fmt::Display for SecuritiesAccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SecurityType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityType {
    Equity,
    Bond,
    Fund,      // SICAV/FCP
    Sukuk,
    TreasuryBill,
}

impl SecurityType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "equity" => Ok(SecurityType::Equity),
            "bond" => Ok(SecurityType::Bond),
            "fund" | "sicav" | "fcp" => Ok(SecurityType::Fund),
            "sukuk" => Ok(SecurityType::Sukuk),
            "treasurybill" | "treasury_bill" => Ok(SecurityType::TreasuryBill),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown security type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SecurityType::Equity => "Equity",
            SecurityType::Bond => "Bond",
            SecurityType::Fund => "Fund",
            SecurityType::Sukuk => "Sukuk",
            SecurityType::TreasuryBill => "TreasuryBill",
        }
    }
}

impl fmt::Display for SecurityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- OrderType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

impl OrderType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "buy" => Ok(OrderType::Buy),
            "sell" => Ok(OrderType::Sell),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown order type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Buy => "Buy",
            OrderType::Sell => "Sell",
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- PriceType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriceType {
    Market,
    Limit,
    StopLoss,
}

impl PriceType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "market" => Ok(PriceType::Market),
            "limit" => Ok(PriceType::Limit),
            "stoploss" | "stop_loss" => Ok(PriceType::StopLoss),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown price type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PriceType::Market => "Market",
            PriceType::Limit => "Limit",
            PriceType::StopLoss => "StopLoss",
        }
    }
}

impl fmt::Display for PriceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- OrderStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

impl OrderStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(OrderStatus::Pending),
            "partiallyfilled" | "partially_filled" => Ok(OrderStatus::PartiallyFilled),
            "filled" => Ok(OrderStatus::Filled),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "rejected" => Ok(OrderStatus::Rejected),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown order status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Pending => "Pending",
            OrderStatus::PartiallyFilled => "PartiallyFilled",
            OrderStatus::Filled => "Filled",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::Rejected => "Rejected",
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SettlementType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementType {
    DVP, // Delivery vs Payment
}

impl SettlementType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "dvp" => Ok(SettlementType::DVP),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown settlement type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SettlementType::DVP => "DVP",
        }
    }
}

impl fmt::Display for SettlementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- SettlementStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementStatus {
    Pending,
    Matched,
    Settled,
    Failed,
}

impl SettlementStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(SettlementStatus::Pending),
            "matched" => Ok(SettlementStatus::Matched),
            "settled" => Ok(SettlementStatus::Settled),
            "failed" => Ok(SettlementStatus::Failed),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown settlement status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SettlementStatus::Pending => "Pending",
            SettlementStatus::Matched => "Matched",
            SettlementStatus::Settled => "Settled",
            SettlementStatus::Failed => "Failed",
        }
    }
}

impl fmt::Display for SettlementStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CorporateActionType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorporateActionType {
    Dividend,
    Coupon,
    Split,
    RightsIssue,
    Merger,
    Redemption,
}

impl CorporateActionType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "dividend" => Ok(CorporateActionType::Dividend),
            "coupon" => Ok(CorporateActionType::Coupon),
            "split" => Ok(CorporateActionType::Split),
            "rightsissue" | "rights_issue" => Ok(CorporateActionType::RightsIssue),
            "merger" => Ok(CorporateActionType::Merger),
            "redemption" => Ok(CorporateActionType::Redemption),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown corporate action type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CorporateActionType::Dividend => "Dividend",
            CorporateActionType::Coupon => "Coupon",
            CorporateActionType::Split => "Split",
            CorporateActionType::RightsIssue => "RightsIssue",
            CorporateActionType::Merger => "Merger",
            CorporateActionType::Redemption => "Redemption",
        }
    }
}

impl fmt::Display for CorporateActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- CorporateActionStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorporateActionStatus {
    Announced,
    Effective,
    Completed,
}

impl CorporateActionStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "announced" => Ok(CorporateActionStatus::Announced),
            "effective" => Ok(CorporateActionStatus::Effective),
            "completed" => Ok(CorporateActionStatus::Completed),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown corporate action status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CorporateActionStatus::Announced => "Announced",
            CorporateActionStatus::Effective => "Effective",
            CorporateActionStatus::Completed => "Completed",
        }
    }
}

impl fmt::Display for CorporateActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- ISIN Code (value object) ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IsinCode(String);

impl IsinCode {
    pub fn new(code: &str) -> Result<Self, DomainError> {
        if code.len() != 12 || !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(DomainError::ValidationError(
                "ISIN code must be 12 alphanumeric characters".to_string(),
            ));
        }
        Ok(IsinCode(code.to_uppercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IsinCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securities_account_id_new() {
        let id = SecuritiesAccountId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_securities_account_id_parse_valid() {
        let id = SecuritiesAccountId::parse("550e8400-e29b-41d4-a716-446655440000");
        assert!(id.is_ok());
    }

    #[test]
    fn test_securities_account_id_parse_invalid() {
        assert!(SecuritiesAccountId::parse("not-a-uuid").is_err());
    }

    #[test]
    fn test_trade_order_id_new() {
        let id = TradeOrderId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_settlement_id_new() {
        let id = SettlementId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_corporate_action_id_new() {
        let id = CorporateActionId::new();
        assert!(!id.as_uuid().is_nil());
    }

    #[test]
    fn test_securities_account_type_from_str() {
        assert_eq!(
            SecuritiesAccountType::from_str("Individual").unwrap(),
            SecuritiesAccountType::Individual
        );
        assert_eq!(
            SecuritiesAccountType::from_str("joint").unwrap(),
            SecuritiesAccountType::Joint
        );
        assert_eq!(
            SecuritiesAccountType::from_str("Corporate").unwrap(),
            SecuritiesAccountType::Corporate
        );
        assert_eq!(
            SecuritiesAccountType::from_str("NOMINEE").unwrap(),
            SecuritiesAccountType::Nominee
        );
    }

    #[test]
    fn test_securities_account_type_from_str_invalid() {
        assert!(SecuritiesAccountType::from_str("unknown").is_err());
    }

    #[test]
    fn test_securities_account_status_from_str() {
        assert_eq!(
            SecuritiesAccountStatus::from_str("Active").unwrap(),
            SecuritiesAccountStatus::Active
        );
        assert_eq!(
            SecuritiesAccountStatus::from_str("suspended").unwrap(),
            SecuritiesAccountStatus::Suspended
        );
        assert_eq!(
            SecuritiesAccountStatus::from_str("Closed").unwrap(),
            SecuritiesAccountStatus::Closed
        );
    }

    #[test]
    fn test_security_type_from_str() {
        assert_eq!(
            SecurityType::from_str("Equity").unwrap(),
            SecurityType::Equity
        );
        assert_eq!(
            SecurityType::from_str("bond").unwrap(),
            SecurityType::Bond
        );
        assert_eq!(
            SecurityType::from_str("fund").unwrap(),
            SecurityType::Fund
        );
        assert_eq!(
            SecurityType::from_str("SICAV").unwrap(),
            SecurityType::Fund
        );
        assert_eq!(
            SecurityType::from_str("sukuk").unwrap(),
            SecurityType::Sukuk
        );
        assert_eq!(
            SecurityType::from_str("treasurybill").unwrap(),
            SecurityType::TreasuryBill
        );
    }

    #[test]
    fn test_order_type_from_str() {
        assert_eq!(OrderType::from_str("Buy").unwrap(), OrderType::Buy);
        assert_eq!(OrderType::from_str("sell").unwrap(), OrderType::Sell);
    }

    #[test]
    fn test_price_type_from_str() {
        assert_eq!(PriceType::from_str("Market").unwrap(), PriceType::Market);
        assert_eq!(PriceType::from_str("limit").unwrap(), PriceType::Limit);
        assert_eq!(
            PriceType::from_str("stoploss").unwrap(),
            PriceType::StopLoss
        );
    }

    #[test]
    fn test_order_status_from_str() {
        assert_eq!(OrderStatus::from_str("Pending").unwrap(), OrderStatus::Pending);
        assert_eq!(
            OrderStatus::from_str("partiallyfilled").unwrap(),
            OrderStatus::PartiallyFilled
        );
        assert_eq!(OrderStatus::from_str("filled").unwrap(), OrderStatus::Filled);
        assert_eq!(
            OrderStatus::from_str("cancelled").unwrap(),
            OrderStatus::Cancelled
        );
    }

    #[test]
    fn test_settlement_type_from_str() {
        assert_eq!(SettlementType::from_str("DVP").unwrap(), SettlementType::DVP);
    }

    #[test]
    fn test_settlement_status_from_str() {
        assert_eq!(
            SettlementStatus::from_str("Pending").unwrap(),
            SettlementStatus::Pending
        );
        assert_eq!(
            SettlementStatus::from_str("settled").unwrap(),
            SettlementStatus::Settled
        );
    }

    #[test]
    fn test_corporate_action_type_from_str() {
        assert_eq!(
            CorporateActionType::from_str("Dividend").unwrap(),
            CorporateActionType::Dividend
        );
        assert_eq!(
            CorporateActionType::from_str("coupon").unwrap(),
            CorporateActionType::Coupon
        );
        assert_eq!(
            CorporateActionType::from_str("Split").unwrap(),
            CorporateActionType::Split
        );
    }

    #[test]
    fn test_corporate_action_status_from_str() {
        assert_eq!(
            CorporateActionStatus::from_str("Announced").unwrap(),
            CorporateActionStatus::Announced
        );
        assert_eq!(
            CorporateActionStatus::from_str("effective").unwrap(),
            CorporateActionStatus::Effective
        );
    }

    #[test]
    fn test_isin_code_valid() {
        let isin = IsinCode::new("TN0123456789");
        assert!(isin.is_ok());
        assert_eq!(isin.unwrap().as_str(), "TN0123456789");
    }

    #[test]
    fn test_isin_code_lowercase_conversion() {
        let isin = IsinCode::new("tn0123456789");
        assert!(isin.is_ok());
        assert_eq!(isin.unwrap().as_str(), "TN0123456789");
    }

    #[test]
    fn test_isin_code_invalid_length() {
        assert!(IsinCode::new("TN01234567890").is_err());
        assert!(IsinCode::new("TN012345678").is_err());
    }

    #[test]
    fn test_isin_code_invalid_chars() {
        assert!(IsinCode::new("TN01234567-9").is_err());
    }
}
