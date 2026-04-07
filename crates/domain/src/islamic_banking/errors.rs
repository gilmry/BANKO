use std::fmt;

use crate::shared::errors::DomainError;

/// Errors specific to Islamic Banking domain
#[derive(Debug, Clone)]
pub enum IslamicBankingError {
    /// Murabaha validation errors
    InvalidMurabahaMargin(String),
    MissingAssetDescription,
    InvalidMurabahaInstallments,
    ProfitMarginMismatch,

    /// Ijara validation errors
    InvalidIjaraAsset(String),
    InvalidIjaraRental(String),
    InvalidLeasePeriod(String),
    InvalidPurchaseOption,

    /// Musharaka validation errors
    InvalidSharePercentage(String),
    SharesSumError(String),
    InvalidProfitRatio,
    EmptyDiminishingSchedule,

    /// Mudaraba validation errors
    InvalidCapitalAmount(String),
    InvalidInvestmentType,
    InvalidMudarabaRatio,

    /// Sukuk validation errors
    InvalidSukukDenomination(String),
    InvalidSukukAmount(String),
    InvalidSukukCoupon(String),
    InvalidMaturityDate,
    MissingUnderlyingAsset,

    /// Zakat validation errors
    ZakatCalculationError(String),
    BelowNisabThreshold,
    InvalidAssessmentYear,

    /// Sharia Board errors
    InsufficientBoardMembers(String),
    QuorumNotMet,
    InvalidRuling(String),
    ConditionalRulingWithoutConditions,

    /// Profit Distribution errors
    InvalidProfitAmount(String),
    DistributionMismatch(String),
    InvalidDistributionPeriod,

    /// General errors
    ContractNotFound(String),
    InvalidContractStatus(String),
    InvalidProductType(String),
    OperationNotPermitted(String),
}

impl fmt::Display for IslamicBankingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IslamicBankingError::InvalidMurabahaMargin(msg) => {
                write!(f, "Invalid Murabaha margin: {}", msg)
            }
            IslamicBankingError::MissingAssetDescription => {
                write!(f, "Asset description is required for Murabaha contract")
            }
            IslamicBankingError::InvalidMurabahaInstallments => {
                write!(f, "Number of installments must be greater than 0")
            }
            IslamicBankingError::ProfitMarginMismatch => {
                write!(f, "Profit margin calculation does not match selling price")
            }
            IslamicBankingError::InvalidIjaraAsset(msg) => {
                write!(f, "Invalid Ijara asset: {}", msg)
            }
            IslamicBankingError::InvalidIjaraRental(msg) => {
                write!(f, "Invalid Ijara rental amount: {}", msg)
            }
            IslamicBankingError::InvalidLeasePeriod(msg) => {
                write!(f, "Invalid lease period: {}", msg)
            }
            IslamicBankingError::InvalidPurchaseOption => {
                write!(f, "Purchase option price must be positive")
            }
            IslamicBankingError::InvalidSharePercentage(msg) => {
                write!(f, "Invalid share percentage: {}", msg)
            }
            IslamicBankingError::SharesSumError(msg) => {
                write!(f, "Shares must sum to 100%: {}", msg)
            }
            IslamicBankingError::InvalidProfitRatio => {
                write!(f, "Profit sharing ratio must be between 0% and 100%")
            }
            IslamicBankingError::EmptyDiminishingSchedule => {
                write!(f, "Diminishing schedule cannot be empty for Musharaka")
            }
            IslamicBankingError::InvalidCapitalAmount(msg) => {
                write!(f, "Invalid capital amount: {}", msg)
            }
            IslamicBankingError::InvalidInvestmentType => {
                write!(f, "Investment type is required for Mudaraba")
            }
            IslamicBankingError::InvalidMudarabaRatio => {
                write!(f, "Mudaraba profit sharing ratio must be between 0% and 100%")
            }
            IslamicBankingError::InvalidSukukDenomination(msg) => {
                write!(f, "Invalid Sukuk denomination: {}", msg)
            }
            IslamicBankingError::InvalidSukukAmount(msg) => {
                write!(f, "Invalid Sukuk total amount: {}", msg)
            }
            IslamicBankingError::InvalidSukukCoupon(msg) => {
                write!(f, "Invalid Sukuk coupon rate: {}", msg)
            }
            IslamicBankingError::InvalidMaturityDate => {
                write!(f, "Maturity date must be in the future")
            }
            IslamicBankingError::MissingUnderlyingAsset => {
                write!(f, "Underlying asset is required for Sukuk issuance")
            }
            IslamicBankingError::ZakatCalculationError(msg) => {
                write!(f, "Zakat calculation error: {}", msg)
            }
            IslamicBankingError::BelowNisabThreshold => {
                write!(f, "Wealth is below Nisab threshold; no Zakat due")
            }
            IslamicBankingError::InvalidAssessmentYear => {
                write!(f, "Assessment year must be valid")
            }
            IslamicBankingError::InsufficientBoardMembers(msg) => {
                write!(f, "Sharia Board must have at least 3 members: {}", msg)
            }
            IslamicBankingError::QuorumNotMet => {
                write!(f, "Sharia Board quorum not met for decision")
            }
            IslamicBankingError::InvalidRuling(msg) => {
                write!(f, "Invalid Sharia Board ruling: {}", msg)
            }
            IslamicBankingError::ConditionalRulingWithoutConditions => {
                write!(f, "Conditional ruling requires conditions to be specified")
            }
            IslamicBankingError::InvalidProfitAmount(msg) => {
                write!(f, "Invalid profit amount: {}", msg)
            }
            IslamicBankingError::DistributionMismatch(msg) => {
                write!(f, "Profit distribution mismatch: {}", msg)
            }
            IslamicBankingError::InvalidDistributionPeriod => {
                write!(f, "Distribution period must be valid")
            }
            IslamicBankingError::ContractNotFound(msg) => {
                write!(f, "Islamic contract not found: {}", msg)
            }
            IslamicBankingError::InvalidContractStatus(msg) => {
                write!(f, "Invalid contract status transition: {}", msg)
            }
            IslamicBankingError::InvalidProductType(msg) => {
                write!(f, "Invalid Islamic product type: {}", msg)
            }
            IslamicBankingError::OperationNotPermitted(msg) => {
                write!(f, "Operation not permitted: {}", msg)
            }
        }
    }
}

impl From<IslamicBankingError> for DomainError {
    fn from(err: IslamicBankingError) -> Self {
        DomainError::InvalidInput(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = IslamicBankingError::MissingAssetDescription;
        assert_eq!(
            err.to_string(),
            "Asset description is required for Murabaha contract"
        );
    }

    #[test]
    fn test_error_conversion_to_domain_error() {
        let err = IslamicBankingError::InvalidMurabahaMargin("negative".to_string());
        let domain_err = DomainError::from(err);
        assert!(domain_err.to_string().contains("Murabaha margin"));
    }

    #[test]
    fn test_invalid_share_error() {
        let err = IslamicBankingError::InvalidSharePercentage("150%".to_string());
        assert!(err.to_string().contains("share percentage"));
    }

    #[test]
    fn test_zakat_error() {
        let err = IslamicBankingError::BelowNisabThreshold;
        assert!(err.to_string().contains("Nisab threshold"));
    }
}
