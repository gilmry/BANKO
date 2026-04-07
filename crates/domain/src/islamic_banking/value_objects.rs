use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

/// Unique identifier for an Islamic contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IslamicContractId(Uuid);

impl IslamicContractId {
    pub fn new() -> Self {
        IslamicContractId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        IslamicContractId(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for IslamicContractId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for IslamicContractId {
    fn default() -> Self {
        Self::new()
    }
}

/// Status of Murabaha contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MurabahaStatus {
    Proposed,
    Approved,
    Active,
    Completed,
    Defaulted,
}

impl fmt::Display for MurabahaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MurabahaStatus::Proposed => write!(f, "Proposed"),
            MurabahaStatus::Approved => write!(f, "Approved"),
            MurabahaStatus::Active => write!(f, "Active"),
            MurabahaStatus::Completed => write!(f, "Completed"),
            MurabahaStatus::Defaulted => write!(f, "Defaulted"),
        }
    }
}

impl MurabahaStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(MurabahaStatus::Proposed),
            "approved" => Ok(MurabahaStatus::Approved),
            "active" => Ok(MurabahaStatus::Active),
            "completed" => Ok(MurabahaStatus::Completed),
            "defaulted" => Ok(MurabahaStatus::Defaulted),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Murabaha status: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MurabahaStatus::Proposed => "Proposed",
            MurabahaStatus::Approved => "Approved",
            MurabahaStatus::Active => "Active",
            MurabahaStatus::Completed => "Completed",
            MurabahaStatus::Defaulted => "Defaulted",
        }
    }
}

/// Status of Ijara contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IjaraStatus {
    Proposed,
    Approved,
    Active,
    Completed,
    Terminated,
}

impl fmt::Display for IjaraStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IjaraStatus::Proposed => write!(f, "Proposed"),
            IjaraStatus::Approved => write!(f, "Approved"),
            IjaraStatus::Active => write!(f, "Active"),
            IjaraStatus::Completed => write!(f, "Completed"),
            IjaraStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

impl IjaraStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(IjaraStatus::Proposed),
            "approved" => Ok(IjaraStatus::Approved),
            "active" => Ok(IjaraStatus::Active),
            "completed" => Ok(IjaraStatus::Completed),
            "terminated" => Ok(IjaraStatus::Terminated),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Ijara status: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            IjaraStatus::Proposed => "Proposed",
            IjaraStatus::Approved => "Approved",
            IjaraStatus::Active => "Active",
            IjaraStatus::Completed => "Completed",
            IjaraStatus::Terminated => "Terminated",
        }
    }
}

/// Maintenance responsibility in Ijara
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceResponsibility {
    Lessor,
    Lessee,
}

impl fmt::Display for MaintenanceResponsibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaintenanceResponsibility::Lessor => write!(f, "Lessor"),
            MaintenanceResponsibility::Lessee => write!(f, "Lessee"),
        }
    }
}

impl MaintenanceResponsibility {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "lessor" => Ok(MaintenanceResponsibility::Lessor),
            "lessee" => Ok(MaintenanceResponsibility::Lessee),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown maintenance responsibility: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MaintenanceResponsibility::Lessor => "Lessor",
            MaintenanceResponsibility::Lessee => "Lessee",
        }
    }
}

/// Status of Musharaka contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MusharakaStatus {
    Proposed,
    Approved,
    Active,
    Completed,
    Dissolved,
}

impl fmt::Display for MusharakaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MusharakaStatus::Proposed => write!(f, "Proposed"),
            MusharakaStatus::Approved => write!(f, "Approved"),
            MusharakaStatus::Active => write!(f, "Active"),
            MusharakaStatus::Completed => write!(f, "Completed"),
            MusharakaStatus::Dissolved => write!(f, "Dissolved"),
        }
    }
}

impl MusharakaStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(MusharakaStatus::Proposed),
            "approved" => Ok(MusharakaStatus::Approved),
            "active" => Ok(MusharakaStatus::Active),
            "completed" => Ok(MusharakaStatus::Completed),
            "dissolved" => Ok(MusharakaStatus::Dissolved),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Musharaka status: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MusharakaStatus::Proposed => "Proposed",
            MusharakaStatus::Approved => "Approved",
            MusharakaStatus::Active => "Active",
            MusharakaStatus::Completed => "Completed",
            MusharakaStatus::Dissolved => "Dissolved",
        }
    }
}

/// Status of Mudaraba contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MudarabaStatus {
    Proposed,
    Approved,
    Active,
    Completed,
    Terminated,
}

impl fmt::Display for MudarabaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MudarabaStatus::Proposed => write!(f, "Proposed"),
            MudarabaStatus::Approved => write!(f, "Approved"),
            MudarabaStatus::Active => write!(f, "Active"),
            MudarabaStatus::Completed => write!(f, "Completed"),
            MudarabaStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

impl MudarabaStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(MudarabaStatus::Proposed),
            "approved" => Ok(MudarabaStatus::Approved),
            "active" => Ok(MudarabaStatus::Active),
            "completed" => Ok(MudarabaStatus::Completed),
            "terminated" => Ok(MudarabaStatus::Terminated),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Mudaraba status: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MudarabaStatus::Proposed => "Proposed",
            MudarabaStatus::Approved => "Approved",
            MudarabaStatus::Active => "Active",
            MudarabaStatus::Completed => "Completed",
            MudarabaStatus::Terminated => "Terminated",
        }
    }
}

/// Status of Sukuk issuance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SukukStatus {
    Proposed,
    Approved,
    Outstanding,
    MaturityReached,
    Redeemed,
}

impl fmt::Display for SukukStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SukukStatus::Proposed => write!(f, "Proposed"),
            SukukStatus::Approved => write!(f, "Approved"),
            SukukStatus::Outstanding => write!(f, "Outstanding"),
            SukukStatus::MaturityReached => write!(f, "MaturityReached"),
            SukukStatus::Redeemed => write!(f, "Redeemed"),
        }
    }
}

impl SukukStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(SukukStatus::Proposed),
            "approved" => Ok(SukukStatus::Approved),
            "outstanding" => Ok(SukukStatus::Outstanding),
            "maturityreached" => Ok(SukukStatus::MaturityReached),
            "redeemed" => Ok(SukukStatus::Redeemed),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Sukuk status: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SukukStatus::Proposed => "Proposed",
            SukukStatus::Approved => "Approved",
            SukukStatus::Outstanding => "Outstanding",
            SukukStatus::MaturityReached => "MaturityReached",
            SukukStatus::Redeemed => "Redeemed",
        }
    }
}

/// Payment status for Zakat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZakatPaymentStatus {
    Pending,
    Paid,
    Deferred,
    Exempt,
}

impl fmt::Display for ZakatPaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZakatPaymentStatus::Pending => write!(f, "Pending"),
            ZakatPaymentStatus::Paid => write!(f, "Paid"),
            ZakatPaymentStatus::Deferred => write!(f, "Deferred"),
            ZakatPaymentStatus::Exempt => write!(f, "Exempt"),
        }
    }
}

impl ZakatPaymentStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ZakatPaymentStatus::Pending),
            "paid" => Ok(ZakatPaymentStatus::Paid),
            "deferred" => Ok(ZakatPaymentStatus::Deferred),
            "exempt" => Ok(ZakatPaymentStatus::Exempt),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Zakat payment status: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ZakatPaymentStatus::Pending => "Pending",
            ZakatPaymentStatus::Paid => "Paid",
            ZakatPaymentStatus::Deferred => "Deferred",
            ZakatPaymentStatus::Exempt => "Exempt",
        }
    }
}

/// Sharia board ruling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShariaRuling {
    Halal,
    Haram,
    Makruh,
    Conditional,
}

impl fmt::Display for ShariaRuling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShariaRuling::Halal => write!(f, "Halal"),
            ShariaRuling::Haram => write!(f, "Haram"),
            ShariaRuling::Makruh => write!(f, "Makruh"),
            ShariaRuling::Conditional => write!(f, "Conditional"),
        }
    }
}

impl ShariaRuling {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "halal" => Ok(ShariaRuling::Halal),
            "haram" => Ok(ShariaRuling::Haram),
            "makruh" => Ok(ShariaRuling::Makruh),
            "conditional" => Ok(ShariaRuling::Conditional),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown Sharia ruling: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ShariaRuling::Halal => "Halal",
            ShariaRuling::Haram => "Haram",
            ShariaRuling::Makruh => "Makruh",
            ShariaRuling::Conditional => "Conditional",
        }
    }
}

/// Product types requiring Sharia board approval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductType {
    Murabaha,
    Ijara,
    Musharaka,
    Mudaraba,
    Sukuk,
    Waqf,
    Takaful,
}

impl fmt::Display for ProductType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductType::Murabaha => write!(f, "Murabaha"),
            ProductType::Ijara => write!(f, "Ijara"),
            ProductType::Musharaka => write!(f, "Musharaka"),
            ProductType::Mudaraba => write!(f, "Mudaraba"),
            ProductType::Sukuk => write!(f, "Sukuk"),
            ProductType::Waqf => write!(f, "Waqf"),
            ProductType::Takaful => write!(f, "Takaful"),
        }
    }
}

impl ProductType {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "murabaha" => Ok(ProductType::Murabaha),
            "ijara" => Ok(ProductType::Ijara),
            "musharaka" => Ok(ProductType::Musharaka),
            "mudaraba" => Ok(ProductType::Mudaraba),
            "sukuk" => Ok(ProductType::Sukuk),
            "waqf" => Ok(ProductType::Waqf),
            "takaful" => Ok(ProductType::Takaful),
            _ => Err(DomainError::InvalidInput(format!(
                "Unknown product type: {}",
                s
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ProductType::Murabaha => "Murabaha",
            ProductType::Ijara => "Ijara",
            ProductType::Musharaka => "Musharaka",
            ProductType::Mudaraba => "Mudaraba",
            ProductType::Sukuk => "Sukuk",
            ProductType::Waqf => "Waqf",
            ProductType::Takaful => "Takaful",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_islamic_contract_id_new() {
        let id1 = IslamicContractId::new();
        let id2 = IslamicContractId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_islamic_contract_id_default() {
        let id = IslamicContractId::default();
        assert_eq!(id, id);
    }

    #[test]
    fn test_murabaha_status_conversion() {
        let status = MurabahaStatus::Active;
        assert_eq!(status.as_str(), "Active");
        assert_eq!(MurabahaStatus::from_str("Active").unwrap(), status);
    }

    #[test]
    fn test_ijara_status_conversion() {
        let status = IjaraStatus::Completed;
        assert_eq!(status.as_str(), "Completed");
        assert_eq!(IjaraStatus::from_str("Completed").unwrap(), status);
    }

    #[test]
    fn test_musharaka_status_conversion() {
        let status = MusharakaStatus::Active;
        assert_eq!(status.as_str(), "Active");
        assert_eq!(MusharakaStatus::from_str("Active").unwrap(), status);
    }

    #[test]
    fn test_mudaraba_status_conversion() {
        let status = MudarabaStatus::Active;
        assert_eq!(status.as_str(), "Active");
        assert_eq!(MudarabaStatus::from_str("Active").unwrap(), status);
    }

    #[test]
    fn test_sukuk_status_conversion() {
        let status = SukukStatus::Outstanding;
        assert_eq!(status.as_str(), "Outstanding");
        assert_eq!(SukukStatus::from_str("Outstanding").unwrap(), status);
    }

    #[test]
    fn test_zakat_payment_status_conversion() {
        let status = ZakatPaymentStatus::Paid;
        assert_eq!(status.as_str(), "Paid");
        assert_eq!(ZakatPaymentStatus::from_str("Paid").unwrap(), status);
    }

    #[test]
    fn test_sharia_ruling_conversion() {
        let ruling = ShariaRuling::Halal;
        assert_eq!(ruling.as_str(), "Halal");
        assert_eq!(ShariaRuling::from_str("Halal").unwrap(), ruling);
    }

    #[test]
    fn test_product_type_conversion() {
        let product = ProductType::Murabaha;
        assert_eq!(product.as_str(), "Murabaha");
        assert_eq!(ProductType::from_str("Murabaha").unwrap(), product);
    }

    #[test]
    fn test_maintenance_responsibility_conversion() {
        let resp = MaintenanceResponsibility::Lessor;
        assert_eq!(resp.as_str(), "Lessor");
        assert_eq!(
            MaintenanceResponsibility::from_str("Lessor").unwrap(),
            resp
        );
    }

    #[test]
    fn test_invalid_status_conversion() {
        assert!(MurabahaStatus::from_str("InvalidStatus").is_err());
    }
}
