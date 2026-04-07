use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// TASK 1: BC6 PRUDENTIAL ADVANCED (FR-069 to FR-081)
// Advanced regulatory capital adequacy requirements
// ============================================================

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RiskWeightedAssetId(Uuid);

impl RiskWeightedAssetId {
    pub fn new() -> Self {
        RiskWeightedAssetId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        RiskWeightedAssetId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for RiskWeightedAssetId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RiskWeightedAssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StressScenarioId(Uuid);

impl StressScenarioId {
    pub fn new() -> Self {
        StressScenarioId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        StressScenarioId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for StressScenarioId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StressScenarioId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums (FR-071 to FR-081) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskWeightingMethod {
    /// Standardized Approach (Bâle III Pilier 1)
    StandardizedApproach,
    /// Internal Ratings-Based (IRB Approach)
    IrbApproach,
}

impl RiskWeightingMethod {
    pub fn as_str(&self) -> &str {
        match self {
            RiskWeightingMethod::StandardizedApproach => "StandardizedApproach",
            RiskWeightingMethod::IrbApproach => "IrbApproach",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetClass {
    Sovereigns,
    PublicSectorEntities,
    Banks,
    Corporates,
    Retail,
    Equity,
    CommoditiesAndOther,
}

impl AssetClass {
    pub fn as_str(&self) -> &str {
        match self {
            AssetClass::Sovereigns => "Sovereigns",
            AssetClass::PublicSectorEntities => "PublicSectorEntities",
            AssetClass::Banks => "Banks",
            AssetClass::Corporates => "Corporates",
            AssetClass::Retail => "Retail",
            AssetClass::Equity => "Equity",
            AssetClass::CommoditiesAndOther => "CommoditiesAndOther",
        }
    }

    /// Standard risk weight per Bâle III (simplified)
    pub fn standard_risk_weight(&self) -> f64 {
        match self {
            AssetClass::Sovereigns => 0.0,      // AAA-AA
            AssetClass::PublicSectorEntities => 20.0,
            AssetClass::Banks => 50.0,
            AssetClass::Corporates => 100.0,
            AssetClass::Retail => 75.0,
            AssetClass::Equity => 250.0,
            AssetClass::CommoditiesAndOther => 100.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StressScenarioType {
    /// Historical scenario (e.g., 2008 crisis)
    Historical,
    /// Hypothetical adverse scenario
    Hypothetical,
    /// Reverse stress test scenario
    ReverseStress,
}

impl StressScenarioType {
    pub fn as_str(&self) -> &str {
        match self {
            StressScenarioType::Historical => "Historical",
            StressScenarioType::Hypothetical => "Hypothetical",
            StressScenarioType::ReverseStress => "ReverseStress",
        }
    }
}

// --- FR-080: Risk-Weighted Assets (RWA) Calculation ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskWeightedAsset {
    id: RiskWeightedAssetId,
    institution_id: Uuid,
    asset_class: AssetClass,
    method: RiskWeightingMethod,
    exposure_value: i64,      // Exposure at Default (EAD)
    risk_weight: f64,         // Percentage (0-1250)
    probability_default: f64, // PD for IRB (0-1)
    loss_given_default: f64,  // LGD (0-1)
    calculated_at: DateTime<Utc>,
}

impl RiskWeightedAsset {
    pub fn new(
        institution_id: Uuid,
        asset_class: AssetClass,
        method: RiskWeightingMethod,
        exposure_value: i64,
        risk_weight: f64,
        probability_default: f64,
        loss_given_default: f64,
    ) -> Result<Self, DomainError> {
        if exposure_value <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Exposure value must be positive".to_string(),
            ));
        }
        if !(0.0..=12.5).contains(&risk_weight) && method == RiskWeightingMethod::StandardizedApproach {
            return Err(DomainError::InvalidPrudentialData(
                "Risk weight must be 0-1250%".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&probability_default) {
            return Err(DomainError::InvalidPrudentialData(
                "PD must be 0-1".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&loss_given_default) {
            return Err(DomainError::InvalidPrudentialData(
                "LGD must be 0-1".to_string(),
            ));
        }

        Ok(RiskWeightedAsset {
            id: RiskWeightedAssetId::new(),
            institution_id,
            asset_class,
            method,
            exposure_value,
            risk_weight,
            probability_default,
            loss_given_default,
            calculated_at: Utc::now(),
        })
    }

    /// Calculate RWA = EAD * RW (Standardized Approach)
    pub fn calculate_rwa_standardized(&self) -> i64 {
        ((self.exposure_value as f64) * (self.risk_weight / 100.0)) as i64
    }

    /// Calculate RWA using IRB formula (simplified)
    /// RWA = EAD * (N(G^-1(PD) + sqrt(R)*G^-1(ASRF)) / (1-R)) * (1+b(M-2.5)) * LGD
    /// Simplified: RWA ≈ EAD * LGD * f(PD)
    pub fn calculate_rwa_irb(&self) -> i64 {
        // Simplified IRB formula (real implementation would use normal inverse CDF)
        let maturity_adjustment = 1.0; // Simplified
        let rw = (self.loss_given_default * self.probability_default.sqrt() * 10.0) * maturity_adjustment;
        ((self.exposure_value as f64) * rw.max(0.0)) as i64
    }

    pub fn calculated_rwa(&self) -> i64 {
        match self.method {
            RiskWeightingMethod::StandardizedApproach => self.calculate_rwa_standardized(),
            RiskWeightingMethod::IrbApproach => self.calculate_rwa_irb(),
        }
    }

    // Getters
    pub fn id(&self) -> &RiskWeightedAssetId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn asset_class(&self) -> AssetClass {
        self.asset_class
    }
    pub fn method(&self) -> RiskWeightingMethod {
        self.method
    }
    pub fn exposure_value(&self) -> i64 {
        self.exposure_value
    }
    pub fn risk_weight(&self) -> f64 {
        self.risk_weight
    }
    pub fn probability_default(&self) -> f64 {
        self.probability_default
    }
    pub fn loss_given_default(&self) -> f64 {
        self.loss_given_default
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- FR-069: Stress Testing ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StressScenario {
    id: StressScenarioId,
    institution_id: Uuid,
    scenario_type: StressScenarioType,
    name: String,
    description: String,
    /// Interest rate shock (e.g., +200 bps = 2.0)
    interest_rate_shock: f64,
    /// Credit spread widening (e.g., +300 bps = 3.0)
    credit_spread_shock: f64,
    /// Equity price decline (e.g., -30% = -0.30)
    equity_shock: f64,
    /// FX depreciation (e.g., -10% = -0.10)
    fx_shock: f64,
    /// Default probability increase factor (e.g., 1.5x)
    pd_multiplier: f64,
    /// Loss given default increase factor
    lgd_multiplier: f64,
    created_at: DateTime<Utc>,
}

impl StressScenario {
    pub fn new(
        institution_id: Uuid,
        scenario_type: StressScenarioType,
        name: String,
        description: String,
        interest_rate_shock: f64,
        credit_spread_shock: f64,
        equity_shock: f64,
        fx_shock: f64,
        pd_multiplier: f64,
        lgd_multiplier: f64,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidPrudentialData(
                "Scenario name cannot be empty".to_string(),
            ));
        }
        if pd_multiplier <= 0.0 || lgd_multiplier <= 0.0 {
            return Err(DomainError::InvalidPrudentialData(
                "Multipliers must be positive".to_string(),
            ));
        }

        Ok(StressScenario {
            id: StressScenarioId::new(),
            institution_id,
            scenario_type,
            name,
            description,
            interest_rate_shock,
            credit_spread_shock,
            equity_shock,
            fx_shock,
            pd_multiplier,
            lgd_multiplier,
            created_at: Utc::now(),
        })
    }

    // Getters
    pub fn id(&self) -> &StressScenarioId {
        &self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn scenario_type(&self) -> StressScenarioType {
        self.scenario_type
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn interest_rate_shock(&self) -> f64 {
        self.interest_rate_shock
    }
    pub fn credit_spread_shock(&self) -> f64 {
        self.credit_spread_shock
    }
    pub fn equity_shock(&self) -> f64 {
        self.equity_shock
    }
    pub fn fx_shock(&self) -> f64 {
        self.fx_shock
    }
    pub fn pd_multiplier(&self) -> f64 {
        self.pd_multiplier
    }
    pub fn lgd_multiplier(&self) -> f64 {
        self.lgd_multiplier
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- FR-073: Liquidity Coverage Ratio (LCR) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidityCoverageRatio {
    id: Uuid,
    institution_id: Uuid,
    /// High Quality Liquid Assets
    hqla: i64,
    /// Total net cash outflows (30 days)
    total_net_cash_outflows: i64,
    calculated_at: DateTime<Utc>,
}

impl LiquidityCoverageRatio {
    pub const MINIMUM_LCR: f64 = 100.0; // 100% per BCT/BMAD

    pub fn new(
        institution_id: Uuid,
        hqla: i64,
        total_net_cash_outflows: i64,
    ) -> Result<Self, DomainError> {
        if hqla < 0 {
            return Err(DomainError::InvalidPrudentialData(
                "HQLA cannot be negative".to_string(),
            ));
        }
        if total_net_cash_outflows <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Total net cash outflows must be positive".to_string(),
            ));
        }

        Ok(LiquidityCoverageRatio {
            id: Uuid::new_v4(),
            institution_id,
            hqla,
            total_net_cash_outflows,
            calculated_at: Utc::now(),
        })
    }

    /// LCR = HQLA / Total Net Cash Outflows (30 days) * 100
    pub fn lcr_ratio(&self) -> f64 {
        (self.hqla as f64 / self.total_net_cash_outflows as f64) * 100.0
    }

    pub fn is_compliant(&self) -> bool {
        self.lcr_ratio() >= Self::MINIMUM_LCR
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn hqla(&self) -> i64 {
        self.hqla
    }
    pub fn total_net_cash_outflows(&self) -> i64 {
        self.total_net_cash_outflows
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- FR-074: Net Stable Funding Ratio (NSFR) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetStableFundingRatio {
    id: Uuid,
    institution_id: Uuid,
    /// Available Stable Funding
    available_stable_funding: i64,
    /// Required Stable Funding
    required_stable_funding: i64,
    calculated_at: DateTime<Utc>,
}

impl NetStableFundingRatio {
    pub const MINIMUM_NSFR: f64 = 100.0; // 100% per Bâle III

    pub fn new(
        institution_id: Uuid,
        available_stable_funding: i64,
        required_stable_funding: i64,
    ) -> Result<Self, DomainError> {
        if available_stable_funding < 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Available stable funding cannot be negative".to_string(),
            ));
        }
        if required_stable_funding <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Required stable funding must be positive".to_string(),
            ));
        }

        Ok(NetStableFundingRatio {
            id: Uuid::new_v4(),
            institution_id,
            available_stable_funding,
            required_stable_funding,
            calculated_at: Utc::now(),
        })
    }

    /// NSFR = Available Stable Funding / Required Stable Funding * 100
    pub fn nsfr_ratio(&self) -> f64 {
        (self.available_stable_funding as f64 / self.required_stable_funding as f64) * 100.0
    }

    pub fn is_compliant(&self) -> bool {
        self.nsfr_ratio() >= Self::MINIMUM_NSFR
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn available_stable_funding(&self) -> i64 {
        self.available_stable_funding
    }
    pub fn required_stable_funding(&self) -> i64 {
        self.required_stable_funding
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- FR-076: Leverage Ratio ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverageRatio {
    id: Uuid,
    institution_id: Uuid,
    /// Tier 1 Capital
    tier1_capital: i64,
    /// Total exposure measure (non-RWA based)
    total_exposure: i64,
    calculated_at: DateTime<Utc>,
}

impl LeverageRatio {
    pub const MINIMUM_LEVERAGE: f64 = 3.0; // 3% per Bâle III

    pub fn new(
        institution_id: Uuid,
        tier1_capital: i64,
        total_exposure: i64,
    ) -> Result<Self, DomainError> {
        if tier1_capital < 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Tier 1 capital cannot be negative".to_string(),
            ));
        }
        if total_exposure <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Total exposure must be positive".to_string(),
            ));
        }

        Ok(LeverageRatio {
            id: Uuid::new_v4(),
            institution_id,
            tier1_capital,
            total_exposure,
            calculated_at: Utc::now(),
        })
    }

    /// Leverage Ratio = Tier 1 Capital / Total Exposure * 100
    pub fn leverage_ratio(&self) -> f64 {
        (self.tier1_capital as f64 / self.total_exposure as f64) * 100.0
    }

    pub fn is_compliant(&self) -> bool {
        self.leverage_ratio() >= Self::MINIMUM_LEVERAGE
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn tier1_capital(&self) -> i64 {
        self.tier1_capital
    }
    pub fn total_exposure(&self) -> i64 {
        self.total_exposure
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- FR-072: Capital Conservation Buffer ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapitalConservationBuffer {
    id: Uuid,
    institution_id: Uuid,
    /// Tier 1 capital
    tier1_capital: i64,
    /// Risk-weighted assets
    risk_weighted_assets: i64,
    calculated_at: DateTime<Utc>,
}

impl CapitalConservationBuffer {
    pub const BUFFER_RATIO: f64 = 2.5; // 2.5% per BCBS

    pub fn new(
        institution_id: Uuid,
        tier1_capital: i64,
        risk_weighted_assets: i64,
    ) -> Result<Self, DomainError> {
        if tier1_capital < 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Tier 1 capital cannot be negative".to_string(),
            ));
        }
        if risk_weighted_assets <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "RWA must be positive".to_string(),
            ));
        }

        Ok(CapitalConservationBuffer {
            id: Uuid::new_v4(),
            institution_id,
            tier1_capital,
            risk_weighted_assets,
            calculated_at: Utc::now(),
        })
    }

    /// Buffer = (Tier 1 Capital / RWA) - 7% (core requirement)
    pub fn buffer_above_minimum(&self) -> f64 {
        let ratio = (self.tier1_capital as f64 / self.risk_weighted_assets as f64) * 100.0;
        (ratio - 7.0).max(0.0) // Above 7% minimum Tier 1
    }

    pub fn has_sufficient_buffer(&self) -> bool {
        self.buffer_above_minimum() >= Self::BUFFER_RATIO
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn tier1_capital(&self) -> i64 {
        self.tier1_capital
    }
    pub fn risk_weighted_assets(&self) -> i64 {
        self.risk_weighted_assets
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- FR-078: Recovery Plan ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    id: Uuid,
    institution_id: Uuid,
    name: String,
    description: String,
    /// Minimum capital trigger (e.g., solvency < 12%)
    capital_trigger: f64,
    /// Liquidity trigger (e.g., LCR < 120%)
    liquidity_trigger: f64,
    /// Key recovery measures
    recovery_measures: Vec<String>,
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}

impl RecoveryPlan {
    pub fn new(
        institution_id: Uuid,
        name: String,
        description: String,
        capital_trigger: f64,
        liquidity_trigger: f64,
        recovery_measures: Vec<String>,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidPrudentialData(
                "Plan name cannot be empty".to_string(),
            ));
        }
        if recovery_measures.is_empty() {
            return Err(DomainError::InvalidPrudentialData(
                "At least one recovery measure required".to_string(),
            ));
        }
        if capital_trigger <= 0.0 || liquidity_trigger <= 0.0 {
            return Err(DomainError::InvalidPrudentialData(
                "Triggers must be positive".to_string(),
            ));
        }

        Ok(RecoveryPlan {
            id: Uuid::new_v4(),
            institution_id,
            name,
            description,
            capital_trigger,
            liquidity_trigger,
            recovery_measures,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn capital_trigger(&self) -> f64 {
        self.capital_trigger
    }
    pub fn liquidity_trigger(&self) -> f64 {
        self.liquidity_trigger
    }
    pub fn recovery_measures(&self) -> &[String] {
        &self.recovery_measures
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }
}

// --- FR-079: ICAAP Assessment ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IcaapAssessment {
    id: Uuid,
    institution_id: Uuid,
    assessment_period: String, // e.g., "2026-Q1"
    /// Own Funds Requirement
    own_funds_requirement: i64,
    /// Pillar 2 Requirement (supervisory judgment)
    pillar2_requirement: i64,
    /// ICAAP Pillar 2 Guidance (ICCAP)
    iccap: i64,
    /// Total capital need
    total_capital_need: i64,
    created_at: DateTime<Utc>,
}

impl IcaapAssessment {
    pub fn new(
        institution_id: Uuid,
        assessment_period: String,
        own_funds_requirement: i64,
        pillar2_requirement: i64,
        iccap: i64,
    ) -> Result<Self, DomainError> {
        if own_funds_requirement <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Own Funds Requirement must be positive".to_string(),
            ));
        }
        if pillar2_requirement < 0 || iccap < 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Requirements cannot be negative".to_string(),
            ));
        }

        let total_capital_need = own_funds_requirement + pillar2_requirement + iccap;

        Ok(IcaapAssessment {
            id: Uuid::new_v4(),
            institution_id,
            assessment_period,
            own_funds_requirement,
            pillar2_requirement,
            iccap,
            total_capital_need,
            created_at: Utc::now(),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn institution_id(&self) -> Uuid {
        self.institution_id
    }
    pub fn assessment_period(&self) -> &str {
        &self.assessment_period
    }
    pub fn own_funds_requirement(&self) -> i64 {
        self.own_funds_requirement
    }
    pub fn pillar2_requirement(&self) -> i64 {
        self.pillar2_requirement
    }
    pub fn iccap(&self) -> i64 {
        self.iccap
    }
    pub fn total_capital_need(&self) -> i64 {
        self.total_capital_need
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_institution() -> Uuid {
        Uuid::new_v4()
    }

    // --- FR-080: Risk-Weighted Assets ---

    #[test]
    fn test_rwa_standardized_approach() {
        let rwa = RiskWeightedAsset::new(
            default_institution(),
            AssetClass::Corporates,
            RiskWeightingMethod::StandardizedApproach,
            1_000_000, // EAD = 1M
            100.0,     // RW = 100%
            0.05,      // PD unused
            0.45,      // LGD unused
        )
        .unwrap();

        // RWA = 1M * 100% = 1M
        assert_eq!(rwa.calculate_rwa_standardized(), 1_000_000);
    }

    #[test]
    fn test_rwa_sovereigns_low_weight() {
        let rwa = RiskWeightedAsset::new(
            default_institution(),
            AssetClass::Sovereigns,
            RiskWeightingMethod::StandardizedApproach,
            5_000_000,
            0.0, // AAA-AA
            0.01,
            0.0,
        )
        .unwrap();

        assert_eq!(rwa.calculate_rwa_standardized(), 0);
    }

    #[test]
    fn test_rwa_irb_approach() {
        let rwa = RiskWeightedAsset::new(
            default_institution(),
            AssetClass::Corporates,
            RiskWeightingMethod::IrbApproach,
            1_000_000,
            0.0, // Unused in IRB
            0.05, // 5% PD
            0.45, // 45% LGD
        )
        .unwrap();

        let calculated = rwa.calculate_rwa_irb();
        assert!(calculated > 0);
        assert!(calculated < 5_000_000);
    }

    #[test]
    fn test_rwa_invalid_exposure() {
        let result = RiskWeightedAsset::new(
            default_institution(),
            AssetClass::Corporates,
            RiskWeightingMethod::StandardizedApproach,
            0,
            100.0,
            0.05,
            0.45,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_rwa_invalid_pd() {
        let result = RiskWeightedAsset::new(
            default_institution(),
            AssetClass::Corporates,
            RiskWeightingMethod::IrbApproach,
            1_000_000,
            0.0,
            1.5, // Invalid > 1
            0.45,
        );
        assert!(result.is_err());
    }

    // --- FR-069: Stress Testing ---

    #[test]
    fn test_stress_scenario_creation() {
        let scenario = StressScenario::new(
            default_institution(),
            StressScenarioType::Historical,
            "2008 Crisis".to_string(),
            "Historical financial crisis scenario".to_string(),
            2.0,   // +200 bps
            3.0,   // +300 bps
            -0.30, // -30% equity
            -0.10, // -10% FX
            1.5,   // 1.5x PD
            1.2,   // 1.2x LGD
        )
        .unwrap();

        assert_eq!(scenario.name(), "2008 Crisis");
        assert_eq!(scenario.pd_multiplier(), 1.5);
    }

    #[test]
    fn test_stress_scenario_invalid_empty_name() {
        let result = StressScenario::new(
            default_institution(),
            StressScenarioType::Historical,
            "".to_string(),
            "desc".to_string(),
            2.0,
            3.0,
            -0.30,
            -0.10,
            1.5,
            1.2,
        );
        assert!(result.is_err());
    }

    // --- FR-073: Liquidity Coverage Ratio ---

    #[test]
    fn test_lcr_compliant() {
        let lcr = LiquidityCoverageRatio::new(
            default_institution(),
            120_000_000, // HQLA = 120M
            100_000_000, // Outflows = 100M
        )
        .unwrap();

        // LCR = 120M / 100M = 120% ≥ 100% ✓
        assert!(lcr.is_compliant());
        assert!(lcr.lcr_ratio() >= 100.0);
    }

    #[test]
    fn test_lcr_non_compliant() {
        let lcr = LiquidityCoverageRatio::new(
            default_institution(),
            80_000_000, // HQLA = 80M
            100_000_000, // Outflows = 100M
        )
        .unwrap();

        // LCR = 80M / 100M = 80% < 100% ✗
        assert!(!lcr.is_compliant());
    }

    // --- FR-074: NSFR ---

    #[test]
    fn test_nsfr_compliant() {
        let nsfr = NetStableFundingRatio::new(
            default_institution(),
            1_100_000_000, // ASF = 1.1B
            1_000_000_000, // RSF = 1B
        )
        .unwrap();

        // NSFR = 1.1B / 1B = 110% ≥ 100% ✓
        assert!(nsfr.is_compliant());
    }

    // --- FR-076: Leverage Ratio ---

    #[test]
    fn test_leverage_ratio_compliant() {
        let lr = LeverageRatio::new(
            default_institution(),
            300_000_000, // Tier1 = 300M
            10_000_000_000, // Total Exposure = 10B
        )
        .unwrap();

        // LR = 300M / 10B = 3% ≥ 3% ✓
        assert!(lr.is_compliant());
    }

    // --- FR-072: Conservation Buffer ---

    #[test]
    fn test_capital_conservation_buffer() {
        let buffer = CapitalConservationBuffer::new(
            default_institution(),
            950_000_000, // Tier1 = 950M
            10_000_000_000, // RWA = 10B
        )
        .unwrap();

        // Tier1 / RWA = 950M / 10B = 9.5%
        // Buffer = 9.5% - 7% = 2.5% ✓
        assert!(buffer.has_sufficient_buffer());
    }

    // --- FR-078: Recovery Plan ---

    #[test]
    fn test_recovery_plan_creation() {
        let plan = RecoveryPlan::new(
            default_institution(),
            "Recovery Plan 2026".to_string(),
            "Comprehensive recovery strategy".to_string(),
            12.0, // Capital trigger at 12%
            120.0, // Liquidity trigger at 120% LCR
            vec![
                "Capital raising".to_string(),
                "Asset sales".to_string(),
                "Liability management".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(plan.recovery_measures().len(), 3);
    }

    #[test]
    fn test_recovery_plan_no_measures() {
        let result = RecoveryPlan::new(
            default_institution(),
            "Plan".to_string(),
            "desc".to_string(),
            12.0,
            120.0,
            vec![], // Empty
        );
        assert!(result.is_err());
    }

    // --- FR-079: ICAAP ---

    #[test]
    fn test_icaap_assessment() {
        let icaap = IcaapAssessment::new(
            default_institution(),
            "2026-Q1".to_string(),
            500_000_000, // OFR = 500M
            100_000_000, // P2R = 100M
            50_000_000,  // ICCAP = 50M
        )
        .unwrap();

        // Total = 500M + 100M + 50M = 650M
        assert_eq!(icaap.total_capital_need(), 650_000_000);
    }

    // --- Asset Class risk weights ---

    #[test]
    fn test_asset_class_standard_weights() {
        assert_eq!(AssetClass::Sovereigns.standard_risk_weight(), 0.0);
        assert_eq!(AssetClass::Banks.standard_risk_weight(), 50.0);
        assert_eq!(AssetClass::Corporates.standard_risk_weight(), 100.0);
        assert_eq!(AssetClass::Equity.standard_risk_weight(), 250.0);
    }

    #[test]
    fn test_stress_scenario_type_display() {
        let st = StressScenarioType::Historical;
        assert_eq!(st.as_str(), "Historical");
    }
}
