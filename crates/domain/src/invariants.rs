//! BANKO Domain Invariants — Compiled Business Rules
//!
//! Each constant maps to a legal reference (BCT circular, GAFI recommendation,
//! ISO standard, or Tunisian law). Violations are enforced at compile-time
//! (type system) or construction-time (Result<T, DomainError>).
//!
//! Reference: docs/bmad/03-architecture.md Section 16

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::time::Duration;

// ═══════════════════════════════════════════════════════════
// PRUDENTIAL RATIOS (Circ. 2016-03, 2018-06, 2025-08)
// ═══════════════════════════════════════════════════════════

/// INV-06: Minimum solvency ratio (FPN/RWA)
pub const SOLVENCY_RATIO_MIN: Decimal = dec!(10);
/// Alert threshold for solvency (pre-breach warning)
pub const SOLVENCY_ALERT_THRESHOLD: Decimal = dec!(12);

/// INV-07: Minimum Tier 1 ratio (CET1+AT1/RWA)
pub const TIER1_RATIO_MIN: Decimal = dec!(7);
/// Alert threshold for Tier 1
pub const TIER1_ALERT_THRESHOLD: Decimal = dec!(8);

/// INV-08: Maximum Credit/Deposit ratio
pub const CD_RATIO_MAX: Decimal = dec!(120);
/// Alert threshold for C/D ratio
pub const CD_ALERT_THRESHOLD: Decimal = dec!(110);

/// INV-05: Maximum concentration per beneficiary (% of FPN)
pub const CONCENTRATION_MAX_PCT: Decimal = dec!(25);
/// Alert threshold for concentration
pub const CONCENTRATION_ALERT_THRESHOLD: Decimal = dec!(20);

// ═══════════════════════════════════════════════════════════
// CREDIT CLASSIFICATION (Circ. 91-24, 2023-02)
// ═══════════════════════════════════════════════════════════

/// INV-09: Minimum provision rates by asset class
/// Class 0 = Current (performing), Class 4 = Compromised (loss)
pub fn minimum_provision_rate(asset_class: u8) -> Decimal {
    match asset_class {
        0 => dec!(0),
        1 => dec!(20),
        2 => dec!(50),
        3 => dec!(75),
        4 => dec!(100),
        _ => dec!(100), // Unknown class = full provision (conservative)
    }
}

/// Number of days past due for each class transition
pub fn days_past_due_for_class(asset_class: u8) -> u32 {
    match asset_class {
        0 => 0,
        1 => 0,   // Classe 1: uncertain but not past due
        2 => 30,  // Pre-doubtful: 30 days
        3 => 60,  // Doubtful: 60 days
        4 => 90,  // Compromised: 90 days
        _ => 0,
    }
}

// ═══════════════════════════════════════════════════════════
// AML / LBC-FT (Loi 2015-26, Circ. 2025-17, GAFI R.16)
// ═══════════════════════════════════════════════════════════

/// INV-08: Cash transaction threshold for AML alert (TND)
pub const AML_CASH_THRESHOLD_TND: Decimal = dec!(5000);

/// INV-17: Travel rule threshold for international transfers (TND)
pub const TRAVEL_RULE_THRESHOLD_TND: Decimal = dec!(250000);

/// Cumulative daily threshold for structuring detection (TND)
pub const AML_DAILY_CUMULATIVE_THRESHOLD_TND: Decimal = dec!(200000);

// ═══════════════════════════════════════════════════════════
// DATA RETENTION & SLAs (Loi 2015-26, Loi 2025, Circ. 2006-19)
// ═══════════════════════════════════════════════════════════

/// INV-10: KYC data retention post account closure
pub const KYC_RETENTION_YEARS: u32 = 10;

/// INV-15: DOS (Suspicious Activity Report) submission SLA
pub const DOS_SUBMISSION_SLA: Duration = Duration::from_secs(24 * 3600);

/// INV-24: Breach notification to INPDP SLA (Loi 2025)
pub const BREACH_NOTIFICATION_SLA: Duration = Duration::from_secs(72 * 3600);

/// INV-25: Data portability request SLA (days)
pub const DATA_PORTABILITY_SLA_DAYS: u32 = 30;

/// EDD (Enhanced Due Diligence) completion SLA (business days)
pub const EDD_COMPLETION_BUSINESS_DAYS: u32 = 10;

/// PEP continuous monitoring interval
pub const PEP_MONITORING_INTERVAL: Duration = Duration::from_secs(24 * 3600);

// ═══════════════════════════════════════════════════════════
// ACCOUNT RULES (Circ. 2025-17)
// ═══════════════════════════════════════════════════════════

/// Minimum DAT (Term Deposit) amount in TND
pub const DAT_MINIMUM_AMOUNT_TND: Decimal = dec!(1000);

/// DAT allowed durations in months
pub const DAT_ALLOWED_DURATIONS_MONTHS: &[u32] = &[6, 12, 24, 36, 60];

/// Session timeout for inactivity (PCI DSS)
pub const SESSION_TIMEOUT: Duration = Duration::from_secs(30 * 60);

// ═══════════════════════════════════════════════════════════
// SECURITY (PCI DSS v4.0.1, ISO 27001:2022)
// ═══════════════════════════════════════════════════════════

/// Minimum password length (PCI DSS 8.3.6)
pub const PASSWORD_MIN_LENGTH: usize = 12;

/// Password expiration in days (PCI DSS 8.3.9)
pub const PASSWORD_EXPIRY_DAYS: u32 = 90;

/// Maximum failed login attempts before lockout (PCI DSS 8.3.4)
pub const MAX_FAILED_LOGIN_ATTEMPTS: u32 = 5;

/// Key rotation interval for encryption keys
pub const KEY_ROTATION_DAYS: u32 = 90;

// ═══════════════════════════════════════════════════════════
// ISLAMIC BANKING (Loi 2016-33)
// ═══════════════════════════════════════════════════════════

/// Zakat rate on eligible assets
pub const ZAKAT_RATE: Decimal = dec!(2.5);

/// Minimum Sharia board members for product approval
pub const SHARIA_BOARD_MIN_MEMBERS: u32 = 3;

// ═══════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solvency_thresholds_ordering() {
        assert!(SOLVENCY_ALERT_THRESHOLD > SOLVENCY_RATIO_MIN);
        assert!(TIER1_ALERT_THRESHOLD > TIER1_RATIO_MIN);
    }

    #[test]
    fn test_cd_thresholds_ordering() {
        assert!(CD_ALERT_THRESHOLD < CD_RATIO_MAX);
    }

    #[test]
    fn test_provision_rates_monotonic() {
        for class in 0..4u8 {
            assert!(minimum_provision_rate(class) <= minimum_provision_rate(class + 1),
                "Provision rate for class {} should be <= class {}", class, class + 1);
        }
    }

    #[test]
    fn test_provision_rate_class_0_is_zero() {
        assert_eq!(minimum_provision_rate(0), dec!(0));
    }

    #[test]
    fn test_provision_rate_class_4_is_100() {
        assert_eq!(minimum_provision_rate(4), dec!(100));
    }

    #[test]
    fn test_provision_rate_unknown_class_is_100() {
        assert_eq!(minimum_provision_rate(5), dec!(100));
        assert_eq!(minimum_provision_rate(255), dec!(100));
    }

    #[test]
    fn test_aml_threshold_positive() {
        assert!(AML_CASH_THRESHOLD_TND > Decimal::ZERO);
    }

    #[test]
    fn test_travel_rule_threshold() {
        assert!(TRAVEL_RULE_THRESHOLD_TND > AML_CASH_THRESHOLD_TND);
    }

    #[test]
    fn test_dat_minimum_positive() {
        assert!(DAT_MINIMUM_AMOUNT_TND > Decimal::ZERO);
    }

    #[test]
    fn test_dat_durations_sorted() {
        for i in 0..DAT_ALLOWED_DURATIONS_MONTHS.len() - 1 {
            assert!(DAT_ALLOWED_DURATIONS_MONTHS[i] < DAT_ALLOWED_DURATIONS_MONTHS[i + 1]);
        }
    }

    #[test]
    fn test_password_min_length() {
        assert!(PASSWORD_MIN_LENGTH >= 12); // PCI DSS 8.3.6
    }

    #[test]
    fn test_session_timeout_30min() {
        assert_eq!(SESSION_TIMEOUT, Duration::from_secs(1800));
    }

    #[test]
    fn test_zakat_rate() {
        assert_eq!(ZAKAT_RATE, dec!(2.5));
    }

    #[test]
    fn test_sharia_board_minimum() {
        assert!(SHARIA_BOARD_MIN_MEMBERS >= 3);
    }

    #[test]
    fn test_concentration_below_solvency() {
        assert!(CONCENTRATION_MAX_PCT < Decimal::from(100));
    }
}
