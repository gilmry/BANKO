use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RatioId(Uuid);

impl RatioId {
    pub fn new() -> Self {
        RatioId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        RatioId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for RatioId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RatioId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstitutionId(Uuid);

impl InstitutionId {
    pub fn new() -> Self {
        InstitutionId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        InstitutionId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InstitutionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InstitutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RatioType {
    Solvency,
    Tier1,
    CreditToDeposit,
    Concentration,
}

impl RatioType {
    pub fn as_str(&self) -> &str {
        match self {
            RatioType::Solvency => "Solvency",
            RatioType::Tier1 => "Tier1",
            RatioType::CreditToDeposit => "CreditToDeposit",
            RatioType::Concentration => "Concentration",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Solvency" => Ok(RatioType::Solvency),
            "Tier1" => Ok(RatioType::Tier1),
            "CreditToDeposit" => Ok(RatioType::CreditToDeposit),
            "Concentration" => Ok(RatioType::Concentration),
            _ => Err(DomainError::InvalidPrudentialData(format!(
                "Unknown ratio type: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachType {
    SolvencyBreach,
    Tier1Breach,
    CreditToDepositBreach,
    ConcentrationBreach,
}

impl BreachType {
    pub fn as_str(&self) -> &str {
        match self {
            BreachType::SolvencyBreach => "SolvencyBreach",
            BreachType::Tier1Breach => "Tier1Breach",
            BreachType::CreditToDepositBreach => "CreditToDepositBreach",
            BreachType::ConcentrationBreach => "ConcentrationBreach",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "SolvencyBreach" => Ok(BreachType::SolvencyBreach),
            "Tier1Breach" => Ok(BreachType::Tier1Breach),
            "CreditToDepositBreach" => Ok(BreachType::CreditToDepositBreach),
            "ConcentrationBreach" => Ok(BreachType::ConcentrationBreach),
            _ => Err(DomainError::InvalidPrudentialData(format!(
                "Unknown breach type: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachStatus {
    Clear,
    Warning,
    Breach,
}

impl BreachStatus {
    pub fn as_str(&self) -> &str {
        match self {
            BreachStatus::Clear => "Clear",
            BreachStatus::Warning => "Warning",
            BreachStatus::Breach => "Breach",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Clear" => Ok(BreachStatus::Clear),
            "Warning" => Ok(BreachStatus::Warning),
            "Breach" => Ok(BreachStatus::Breach),
            _ => Err(DomainError::InvalidPrudentialData(format!(
                "Unknown breach status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertSeverity {
    Warning,
    Critical,
}

impl AlertSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            AlertSeverity::Warning => "Warning",
            AlertSeverity::Critical => "Critical",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Warning" => Ok(AlertSeverity::Warning),
            "Critical" => Ok(AlertSeverity::Critical),
            _ => Err(DomainError::InvalidPrudentialData(format!(
                "Unknown alert severity: {s}"
            ))),
        }
    }
}

// --- Exposure ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exposure {
    beneficiary_id: Uuid,
    amount: i64,
    description: String,
}

impl Exposure {
    pub fn new(beneficiary_id: Uuid, amount: i64, description: String) -> Self {
        Exposure {
            beneficiary_id,
            amount,
            description,
        }
    }

    pub fn beneficiary_id(&self) -> Uuid {
        self.beneficiary_id
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

// --- Breach Alert ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachAlert {
    id: Uuid,
    ratio_id: RatioId,
    breach_type: BreachType,
    current_value: f64,
    threshold: f64,
    severity: AlertSeverity,
    status: BreachStatus,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
}

impl BreachAlert {
    pub fn new(
        ratio_id: RatioId,
        breach_type: BreachType,
        current_value: f64,
        threshold: f64,
        severity: AlertSeverity,
    ) -> Self {
        BreachAlert {
            id: Uuid::new_v4(),
            ratio_id,
            breach_type,
            current_value,
            threshold,
            severity,
            status: BreachStatus::Breach,
            created_at: Utc::now(),
            resolved_at: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn ratio_id(&self) -> &RatioId {
        &self.ratio_id
    }
    pub fn breach_type(&self) -> BreachType {
        self.breach_type
    }
    pub fn current_value(&self) -> f64 {
        self.current_value
    }
    pub fn threshold(&self) -> f64 {
        self.threshold
    }
    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }
    pub fn status(&self) -> BreachStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn resolved_at(&self) -> Option<DateTime<Utc>> {
        self.resolved_at
    }

    pub fn resolve(&mut self) {
        self.status = BreachStatus::Clear;
        self.resolved_at = Some(Utc::now());
    }
}

// --- Ratio Snapshot ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatioSnapshot {
    id: Uuid,
    ratio_id: RatioId,
    institution_id: InstitutionId,
    snapshot_date: chrono::NaiveDate,
    solvency_ratio: f64,
    tier1_ratio: f64,
    credit_deposit_ratio: f64,
    breach_type: Option<BreachType>,
    created_at: DateTime<Utc>,
}

impl RatioSnapshot {
    pub fn new(
        ratio_id: RatioId,
        institution_id: InstitutionId,
        snapshot_date: chrono::NaiveDate,
        solvency_ratio: f64,
        tier1_ratio: f64,
        credit_deposit_ratio: f64,
        breach_type: Option<BreachType>,
    ) -> Self {
        RatioSnapshot {
            id: Uuid::new_v4(),
            ratio_id,
            institution_id,
            snapshot_date,
            solvency_ratio,
            tier1_ratio,
            credit_deposit_ratio,
            breach_type,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn ratio_id(&self) -> &RatioId {
        &self.ratio_id
    }
    pub fn institution_id(&self) -> &InstitutionId {
        &self.institution_id
    }
    pub fn snapshot_date(&self) -> chrono::NaiveDate {
        self.snapshot_date
    }
    pub fn solvency_ratio(&self) -> f64 {
        self.solvency_ratio
    }
    pub fn tier1_ratio(&self) -> f64 {
        self.tier1_ratio
    }
    pub fn credit_deposit_ratio(&self) -> f64 {
        self.credit_deposit_ratio
    }
    pub fn breach_type(&self) -> Option<BreachType> {
        self.breach_type
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- PrudentialRatio Aggregate Root ---

/// Regulatory thresholds per Circ. 2016-03, 2018-06, 2018-10
pub const SOLVENCY_MINIMUM: f64 = 10.0;
pub const TIER1_MINIMUM: f64 = 7.0;
pub const CREDIT_DEPOSIT_MAXIMUM: f64 = 120.0;
pub const CONCENTRATION_MAXIMUM: f64 = 25.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrudentialRatio {
    ratio_id: RatioId,
    institution_id: InstitutionId,
    capital_tier1: i64,
    capital_tier2: i64,
    risk_weighted_assets: i64,
    total_credits: i64,
    total_deposits: i64,
    exposures: Vec<Exposure>,
    calculated_at: DateTime<Utc>,
}

impl PrudentialRatio {
    pub fn new(
        institution_id: InstitutionId,
        capital_tier1: i64,
        capital_tier2: i64,
        risk_weighted_assets: i64,
        total_credits: i64,
        total_deposits: i64,
        exposures: Vec<Exposure>,
    ) -> Result<Self, DomainError> {
        if risk_weighted_assets <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Risk weighted assets must be positive".to_string(),
            ));
        }
        if total_deposits <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Total deposits must be positive".to_string(),
            ));
        }
        if capital_tier1 < 0 || capital_tier2 < 0 {
            return Err(DomainError::InvalidPrudentialData(
                "Capital cannot be negative".to_string(),
            ));
        }

        let ratio = PrudentialRatio {
            ratio_id: RatioId::new(),
            institution_id,
            capital_tier1,
            capital_tier2,
            risk_weighted_assets,
            total_credits,
            total_deposits,
            exposures,
            calculated_at: Utc::now(),
        };

        // Validate all invariants
        let solvency = ratio.solvency_ratio();
        if solvency < SOLVENCY_MINIMUM {
            return Err(DomainError::SolvencyRatioBreach {
                ratio: solvency,
                minimum: SOLVENCY_MINIMUM,
            });
        }

        let tier1 = ratio.tier1_ratio();
        if tier1 < TIER1_MINIMUM {
            return Err(DomainError::Tier1RatioBreach {
                ratio: tier1,
                minimum: TIER1_MINIMUM,
            });
        }

        let cd = ratio.credit_deposit_ratio();
        if cd > CREDIT_DEPOSIT_MAXIMUM {
            return Err(DomainError::CreditToDepositBreach {
                ratio: cd,
                maximum: CREDIT_DEPOSIT_MAXIMUM,
            });
        }

        // Check concentration for all exposures
        let fpn = ratio.fonds_propres_nets();
        if fpn > 0 {
            for exp in &ratio.exposures {
                let concentration = (exp.amount as f64 / fpn as f64) * 100.0;
                if concentration > CONCENTRATION_MAXIMUM {
                    return Err(DomainError::ConcentrationBreach {
                        beneficiary_id: exp.beneficiary_id,
                        ratio: concentration,
                        maximum: CONCENTRATION_MAXIMUM,
                    });
                }
            }
        }

        Ok(ratio)
    }

    /// Reconstruct from persistence (bypasses validation)
    pub fn from_raw(
        ratio_id: RatioId,
        institution_id: InstitutionId,
        capital_tier1: i64,
        capital_tier2: i64,
        risk_weighted_assets: i64,
        total_credits: i64,
        total_deposits: i64,
        exposures: Vec<Exposure>,
        calculated_at: DateTime<Utc>,
    ) -> Self {
        PrudentialRatio {
            ratio_id,
            institution_id,
            capital_tier1,
            capital_tier2,
            risk_weighted_assets,
            total_credits,
            total_deposits,
            exposures,
            calculated_at,
        }
    }

    // --- Getters ---

    pub fn ratio_id(&self) -> &RatioId {
        &self.ratio_id
    }
    pub fn institution_id(&self) -> &InstitutionId {
        &self.institution_id
    }
    pub fn capital_tier1(&self) -> i64 {
        self.capital_tier1
    }
    pub fn capital_tier2(&self) -> i64 {
        self.capital_tier2
    }
    pub fn risk_weighted_assets(&self) -> i64 {
        self.risk_weighted_assets
    }
    pub fn total_credits(&self) -> i64 {
        self.total_credits
    }
    pub fn total_deposits(&self) -> i64 {
        self.total_deposits
    }
    pub fn exposures(&self) -> &[Exposure] {
        &self.exposures
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }

    // --- Ratio Calculations ---

    /// Fonds Propres Nets = Tier1 + Tier2
    pub fn fonds_propres_nets(&self) -> i64 {
        self.capital_tier1 + self.capital_tier2
    }

    /// Solvency ratio = FPN / RWA * 100 (INV-02: must be ≥ 10%)
    pub fn solvency_ratio(&self) -> f64 {
        if self.risk_weighted_assets == 0 {
            return 0.0;
        }
        (self.fonds_propres_nets() as f64 / self.risk_weighted_assets as f64) * 100.0
    }

    /// Tier 1 ratio = Tier1 / RWA * 100 (INV-03: must be ≥ 7%)
    pub fn tier1_ratio(&self) -> f64 {
        if self.risk_weighted_assets == 0 {
            return 0.0;
        }
        (self.capital_tier1 as f64 / self.risk_weighted_assets as f64) * 100.0
    }

    /// Credit-to-Deposit ratio = Credits / Deposits * 100 (INV-04: must be ≤ 120%)
    pub fn credit_deposit_ratio(&self) -> f64 {
        if self.total_deposits == 0 {
            return 0.0;
        }
        (self.total_credits as f64 / self.total_deposits as f64) * 100.0
    }

    /// Check concentration for a specific beneficiary (INV-05: must be ≤ 25% FPN)
    pub fn check_concentration(&self, beneficiary_id: Uuid) -> Result<f64, DomainError> {
        let fpn = self.fonds_propres_nets();
        if fpn <= 0 {
            return Err(DomainError::InvalidPrudentialData(
                "FPN must be positive for concentration check".to_string(),
            ));
        }

        let total_exposure: i64 = self
            .exposures
            .iter()
            .filter(|e| e.beneficiary_id == beneficiary_id)
            .map(|e| e.amount)
            .sum();

        let ratio = (total_exposure as f64 / fpn as f64) * 100.0;

        if ratio > CONCENTRATION_MAXIMUM {
            return Err(DomainError::ConcentrationBreach {
                beneficiary_id,
                ratio,
                maximum: CONCENTRATION_MAXIMUM,
            });
        }

        Ok(ratio)
    }

    /// Returns all current breaches
    pub fn check_all_ratios(&self) -> Vec<BreachType> {
        let mut breaches = Vec::new();

        if self.solvency_ratio() < SOLVENCY_MINIMUM {
            breaches.push(BreachType::SolvencyBreach);
        }
        if self.tier1_ratio() < TIER1_MINIMUM {
            breaches.push(BreachType::Tier1Breach);
        }
        if self.credit_deposit_ratio() > CREDIT_DEPOSIT_MAXIMUM {
            breaches.push(BreachType::CreditToDepositBreach);
        }

        let fpn = self.fonds_propres_nets();
        if fpn > 0 {
            for exp in &self.exposures {
                let concentration = (exp.amount as f64 / fpn as f64) * 100.0;
                if concentration > CONCENTRATION_MAXIMUM {
                    breaches.push(BreachType::ConcentrationBreach);
                    break;
                }
            }
        }

        breaches
    }

    /// Generate breach alerts for current state
    pub fn generate_alerts(&self) -> Vec<BreachAlert> {
        let mut alerts = Vec::new();

        let solvency = self.solvency_ratio();
        if solvency < SOLVENCY_MINIMUM {
            alerts.push(BreachAlert::new(
                self.ratio_id.clone(),
                BreachType::SolvencyBreach,
                solvency,
                SOLVENCY_MINIMUM,
                AlertSeverity::Critical,
            ));
        }

        let tier1 = self.tier1_ratio();
        if tier1 < TIER1_MINIMUM {
            alerts.push(BreachAlert::new(
                self.ratio_id.clone(),
                BreachType::Tier1Breach,
                tier1,
                TIER1_MINIMUM,
                AlertSeverity::Critical,
            ));
        }

        let cd = self.credit_deposit_ratio();
        if cd > CREDIT_DEPOSIT_MAXIMUM {
            alerts.push(BreachAlert::new(
                self.ratio_id.clone(),
                BreachType::CreditToDepositBreach,
                cd,
                CREDIT_DEPOSIT_MAXIMUM,
                AlertSeverity::Critical,
            ));
        }

        let fpn = self.fonds_propres_nets();
        if fpn > 0 {
            for exp in &self.exposures {
                let concentration = (exp.amount as f64 / fpn as f64) * 100.0;
                if concentration > CONCENTRATION_MAXIMUM {
                    alerts.push(BreachAlert::new(
                        self.ratio_id.clone(),
                        BreachType::ConcentrationBreach,
                        concentration,
                        CONCENTRATION_MAXIMUM,
                        AlertSeverity::Critical,
                    ));
                }
            }
        }

        alerts
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_institution() -> InstitutionId {
        InstitutionId::from_uuid(Uuid::new_v4())
    }

    // --- PRU-01: Domain invariants ---

    #[test]
    fn test_solvency_ratio_valid() {
        // FPN = 150 + 50 = 200, RWA = 1000 → 20% ≥ 10% ✓
        let ratio = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        )
        .unwrap();
        assert!((ratio.solvency_ratio() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_solvency_ratio_breach() {
        // FPN = 50 + 20 = 70, RWA = 1000 → 7% < 10% ✗
        let result = PrudentialRatio::new(
            default_institution(),
            50_000,
            20_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        );
        assert!(matches!(
            result,
            Err(DomainError::SolvencyRatioBreach { .. })
        ));
    }

    #[test]
    fn test_solvency_exactly_10_percent() {
        // FPN = 100, RWA = 1000 → exactly 10% ✓
        let ratio = PrudentialRatio::new(
            default_institution(),
            70_000,
            30_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        )
        .unwrap();
        assert!((ratio.solvency_ratio() - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_tier1_ratio_valid() {
        // Tier1 = 80, RWA = 1000 → 8% ≥ 7% ✓
        let ratio = PrudentialRatio::new(
            default_institution(),
            80_000,
            30_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        )
        .unwrap();
        assert!((ratio.tier1_ratio() - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_tier1_ratio_breach() {
        // Tier1 = 60, RWA = 1000 → 6% < 7%, but solvency = (60+50)/1000 = 11% OK
        let result = PrudentialRatio::new(
            default_institution(),
            60_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        );
        assert!(matches!(result, Err(DomainError::Tier1RatioBreach { .. })));
    }

    #[test]
    fn test_credit_deposit_ratio_valid() {
        // C/D = 500/800 = 62.5% ≤ 120% ✓
        let ratio = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        )
        .unwrap();
        assert!((ratio.credit_deposit_ratio() - 62.5).abs() < 0.01);
    }

    #[test]
    fn test_credit_deposit_ratio_breach() {
        // C/D = 1210/1000 = 121% > 120% ✗
        let result = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            1_210_000,
            1_000_000,
            vec![],
        );
        assert!(matches!(
            result,
            Err(DomainError::CreditToDepositBreach { .. })
        ));
    }

    #[test]
    fn test_credit_deposit_exactly_120() {
        // C/D = 960/800 = 120% ≤ 120% ✓
        let ratio = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            960_000,
            800_000,
            vec![],
        )
        .unwrap();
        assert!((ratio.credit_deposit_ratio() - 120.0).abs() < 0.01);
    }

    #[test]
    fn test_concentration_valid() {
        let ben_id = Uuid::new_v4();
        // FPN = 200, exposure = 40 → 20% ≤ 25% ✓
        let ratio = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![Exposure::new(ben_id, 40_000, "Loan A".into())],
        )
        .unwrap();
        let conc = ratio.check_concentration(ben_id).unwrap();
        assert!((conc - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_concentration_breach() {
        let ben_id = Uuid::new_v4();
        // FPN = 200, exposure = 60 → 30% > 25% ✗
        let result = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![Exposure::new(ben_id, 60_000, "Loan A".into())],
        );
        assert!(matches!(
            result,
            Err(DomainError::ConcentrationBreach { .. })
        ));
    }

    #[test]
    fn test_check_all_ratios_no_breaches() {
        let ratio = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        )
        .unwrap();
        assert!(ratio.check_all_ratios().is_empty());
    }

    #[test]
    fn test_invalid_rwa_zero() {
        let result = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            0,
            500_000,
            800_000,
            vec![],
        );
        assert!(matches!(result, Err(DomainError::InvalidPrudentialData(_))));
    }

    #[test]
    fn test_invalid_deposits_zero() {
        let result = PrudentialRatio::new(
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            0,
            vec![],
        );
        assert!(matches!(result, Err(DomainError::InvalidPrudentialData(_))));
    }

    #[test]
    fn test_negative_capital_rejected() {
        let result = PrudentialRatio::new(
            default_institution(),
            -100,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
        );
        assert!(matches!(result, Err(DomainError::InvalidPrudentialData(_))));
    }

    // --- PRU-05 to PRU-08: Breach alert generation ---

    #[test]
    fn test_generate_alerts_solvency_breach() {
        let ratio = PrudentialRatio::from_raw(
            RatioId::new(),
            default_institution(),
            50_000,
            20_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
            Utc::now(),
        );
        let alerts = ratio.generate_alerts();
        assert!(alerts
            .iter()
            .any(|a| a.breach_type() == BreachType::SolvencyBreach));
    }

    #[test]
    fn test_generate_alerts_no_breach() {
        let ratio = PrudentialRatio::from_raw(
            RatioId::new(),
            default_institution(),
            150_000,
            50_000,
            1_000_000,
            500_000,
            800_000,
            vec![],
            Utc::now(),
        );
        let alerts = ratio.generate_alerts();
        assert!(alerts.is_empty());
    }

    #[test]
    fn test_breach_alert_resolve() {
        let mut alert = BreachAlert::new(
            RatioId::new(),
            BreachType::SolvencyBreach,
            8.5,
            10.0,
            AlertSeverity::Critical,
        );
        assert_eq!(alert.status(), BreachStatus::Breach);
        alert.resolve();
        assert_eq!(alert.status(), BreachStatus::Clear);
        assert!(alert.resolved_at().is_some());
    }

    #[test]
    fn test_ratio_snapshot_creation() {
        let snapshot = RatioSnapshot::new(
            RatioId::new(),
            default_institution(),
            chrono::NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            12.5,
            8.2,
            95.0,
            None,
        );
        assert!((snapshot.solvency_ratio() - 12.5).abs() < 0.01);
        assert!(snapshot.breach_type().is_none());
    }

    // --- Integration test: loans + deposits → solvency ---

    #[test]
    fn test_create_loans_deposits_verify_solvency() {
        // Simulating: bank has 200M FPN, 1.5B RWA, 1B credits, 900M deposits
        let ratio = PrudentialRatio::new(
            default_institution(),
            120_000_000,   // tier1: 120M
            80_000_000,    // tier2: 80M
            1_500_000_000, // RWA: 1.5B
            1_000_000_000, // credits: 1B
            900_000_000,   // deposits: 900M
            vec![],
        )
        .unwrap();

        // Solvency = 200M / 1.5B = 13.33% ≥ 10% ✓
        assert!(ratio.solvency_ratio() >= 10.0);
        // Tier1 = 120M / 1.5B = 8% ≥ 7% ✓
        assert!(ratio.tier1_ratio() >= 7.0);
        // C/D = 1B / 900M = 111.11% ≤ 120% ✓
        assert!(ratio.credit_deposit_ratio() <= 120.0);
    }

    #[test]
    fn test_ratio_id_display() {
        let id = RatioId::new();
        let s = format!("{id}");
        assert!(!s.is_empty());
    }

    #[test]
    fn test_institution_id_display() {
        let id = InstitutionId::new();
        let s = format!("{id}");
        assert!(!s.is_empty());
    }

    #[test]
    fn test_ratio_type_roundtrip() {
        for rt in [
            RatioType::Solvency,
            RatioType::Tier1,
            RatioType::CreditToDeposit,
            RatioType::Concentration,
        ] {
            assert_eq!(RatioType::from_str_value(rt.as_str()).unwrap(), rt);
        }
    }

    #[test]
    fn test_breach_type_roundtrip() {
        for bt in [
            BreachType::SolvencyBreach,
            BreachType::Tier1Breach,
            BreachType::CreditToDepositBreach,
            BreachType::ConcentrationBreach,
        ] {
            assert_eq!(BreachType::from_str_value(bt.as_str()).unwrap(), bt);
        }
    }
}
