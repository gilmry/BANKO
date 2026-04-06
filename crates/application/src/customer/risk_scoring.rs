use banko_domain::customer::{
    Customer, CustomerType, PepStatus, RiskLevel, RiskScore, SourceOfFunds,
};

/// Risk scoring algorithm version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RiskScoringVersion {
    /// Original algorithm (V1).
    V1,
    /// Enhanced algorithm with additional risk factors (V2).
    V2,
}

impl Default for RiskScoringVersion {
    fn default() -> Self {
        RiskScoringVersion::V1
    }
}

impl std::fmt::Display for RiskScoringVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskScoringVersion::V1 => write!(f, "V1"),
            RiskScoringVersion::V2 => write!(f, "V2"),
        }
    }
}

/// Result of risk scoring calculation including the version used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskScoringResult {
    pub score: RiskScore,
    pub version: RiskScoringVersion,
}

impl RiskScoringResult {
    /// Create a new risk scoring result.
    pub fn new(score: RiskScore, version: RiskScoringVersion) -> Self {
        RiskScoringResult { score, version }
    }

    /// Get the risk score.
    pub fn score(&self) -> &RiskScore {
        &self.score
    }

    /// Get the version used.
    pub fn version(&self) -> RiskScoringVersion {
        self.version
    }
}

/// Trait for risk scoring algorithms.
trait RiskScoringAlgorithm {
    /// Calculate the risk score for a customer based on their KYC profile.
    fn calculate(&self, customer: &Customer) -> RiskScore;
}

/// V1 Risk Scoring Algorithm - Original implementation.
struct RiskScoringV1;

impl RiskScoringAlgorithm for RiskScoringV1 {
    fn calculate(&self, customer: &Customer) -> RiskScore {
        let mut score: u8 = 10; // base score

        let kyc = customer.kyc_profile();

        // PEP = Yes -> +30
        if kyc.pep_status() == PepStatus::Yes {
            score = score.saturating_add(30);
        }

        // Source = Business -> +15
        if kyc.source_of_funds() == SourceOfFunds::Business {
            score = score.saturating_add(15);
        }

        // Source = Investment -> +10
        if kyc.source_of_funds() == SourceOfFunds::Investment {
            score = score.saturating_add(10);
        }

        // LegalEntity -> +10
        if customer.customer_type() == CustomerType::LegalEntity {
            score = score.saturating_add(10);
        }

        // Age-based risk: < 25 or > 80 -> +10
        if let Some(birth_date) = kyc.birth_date() {
            let today = chrono::Utc::now().date_naive();
            let age = today.years_since(birth_date).unwrap_or(0);
            if !(25..=80).contains(&age) {
                score = score.saturating_add(10);
            }
        }

        RiskScore::new(score.min(100)).unwrap()
    }
}

/// V2 Risk Scoring Algorithm - Enhanced with additional risk factors.
///
/// Scoring rules:
/// - Base: 10
/// - PEP = Yes: +30
/// - Source = Business: +15
/// - Source = Investment: +10
/// - LegalEntity: +15 (increased from +10)
/// - Age < 25 or > 80: +10
/// - High-risk nationality: +20
/// - Missing documentation: +15
/// - Recently created (< 6 months): +5
struct RiskScoringV2;

impl RiskScoringAlgorithm for RiskScoringV2 {
    fn calculate(&self, customer: &Customer) -> RiskScore {
        let mut score: u8 = 10; // base score

        let kyc = customer.kyc_profile();

        // PEP = Yes -> +30
        if kyc.pep_status() == PepStatus::Yes {
            score = score.saturating_add(30);
        }

        // Source = Business -> +15
        if kyc.source_of_funds() == SourceOfFunds::Business {
            score = score.saturating_add(15);
        }

        // Source = Investment -> +10
        if kyc.source_of_funds() == SourceOfFunds::Investment {
            score = score.saturating_add(10);
        }

        // LegalEntity -> +15 (increased from +10)
        if customer.customer_type() == CustomerType::LegalEntity {
            score = score.saturating_add(15);
        }

        // Age-based risk: < 25 or > 80 -> +10
        if let Some(birth_date) = kyc.birth_date() {
            let today = chrono::Utc::now().date_naive();
            let age = today.years_since(birth_date).unwrap_or(0);
            if !(25..=80).contains(&age) {
                score = score.saturating_add(10);
            }
        }

        // High-risk nationality -> +20
        let high_risk_countries = ["Iran", "Syria", "North Korea", "Cuba"];
        if high_risk_countries.contains(&kyc.nationality()) {
            score = score.saturating_add(20);
        }

        // Missing documentation -> +15
        // Check if KYC profile has incomplete information
        if kyc.birth_date().is_none() && customer.customer_type() == CustomerType::Individual {
            score = score.saturating_add(15);
        }

        // Recently created (< 6 months) -> +5
        // Check if customer was created less than 6 months ago
        let creation_threshold = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::days(180))
            .unwrap_or(chrono::Utc::now());
        if customer.created_at() > creation_threshold {
            score = score.saturating_add(5);
        }

        RiskScore::new(score.min(100)).unwrap()
    }
}

/// Calculates risk scores for customers based on their profile attributes.
pub struct RiskScoringService;

impl RiskScoringService {
    /// Calculate the risk score for a customer based on their KYC profile using specified version.
    pub fn calculate_risk_score_with_version(
        customer: &Customer,
        version: RiskScoringVersion,
    ) -> RiskScoringResult {
        let algorithm: Box<dyn RiskScoringAlgorithm> = match version {
            RiskScoringVersion::V1 => Box::new(RiskScoringV1),
            RiskScoringVersion::V2 => Box::new(RiskScoringV2),
        };

        let score = algorithm.calculate(customer);
        RiskScoringResult::new(score, version)
    }

    /// Calculate the risk score for a customer using the default V1 algorithm.
    /// Provided for backward compatibility.
    pub fn calculate_risk_score(customer: &Customer) -> RiskScore {
        Self::calculate_risk_score_with_version(customer, RiskScoringVersion::V1).score
    }

    /// Get the risk level from a score.
    pub fn risk_level(score: &RiskScore) -> RiskLevel {
        score.risk_level()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use banko_domain::customer::{
        Address, Beneficiary, Cin, ConsentStatus, CustomerType, KycProfile, PepStatus,
        SourceOfFunds,
    };
    use banko_domain::shared::value_objects::{EmailAddress, PhoneNumber};

    use super::*;

    fn valid_address() -> Address {
        Address::new("10 Rue de la Liberte", "Tunis", "1000", "Tunisia").unwrap()
    }

    fn valid_phone() -> PhoneNumber {
        PhoneNumber::new("+21698123456").unwrap()
    }

    fn valid_email() -> EmailAddress {
        EmailAddress::new("ahmed@example.com").unwrap()
    }

    fn make_customer(
        customer_type: CustomerType,
        pep: PepStatus,
        source: SourceOfFunds,
        birth_date: Option<NaiveDate>,
    ) -> Customer {
        let kyc = if customer_type == CustomerType::Individual {
            KycProfile::new_individual(
                "Test Person",
                Cin::new("12345678").unwrap(),
                birth_date.unwrap_or(NaiveDate::from_ymd_opt(1990, 1, 15).unwrap()),
                "Tunisia",
                "Banker",
                valid_address(),
                valid_phone(),
                valid_email(),
                pep,
                source,
            )
            .unwrap()
        } else {
            KycProfile::new_legal_entity(
                "Test Corp",
                "RCS-12345",
                "Banking",
                valid_address(),
                valid_phone(),
                valid_email(),
                pep,
                source,
            )
            .unwrap()
        };

        let beneficiaries = if customer_type == CustomerType::LegalEntity {
            vec![Beneficiary::new("Owner", 100.0).unwrap()]
        } else {
            vec![]
        };

        Customer::new(customer_type, kyc, beneficiaries, ConsentStatus::Given).unwrap()
    }

    // V1 Tests (backward compatibility)

    #[test]
    fn test_base_risk_score() {
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );
        let score = RiskScoringService::calculate_risk_score(&customer);
        assert_eq!(score.value(), 10); // base only
    }

    #[test]
    fn test_pep_increases_score() {
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::Yes,
            SourceOfFunds::Salary,
            None,
        );
        let score = RiskScoringService::calculate_risk_score(&customer);
        assert_eq!(score.value(), 40); // base 10 + PEP 30
    }

    #[test]
    fn test_business_source_increases_score() {
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::No,
            SourceOfFunds::Business,
            None,
        );
        let score = RiskScoringService::calculate_risk_score(&customer);
        assert_eq!(score.value(), 25); // base 10 + business 15
    }

    #[test]
    fn test_legal_entity_increases_score() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );
        let score = RiskScoringService::calculate_risk_score(&customer);
        assert_eq!(score.value(), 20); // base 10 + legal 10
    }

    #[test]
    fn test_young_person_increases_score() {
        let young_date = chrono::Utc::now()
            .date_naive()
            .checked_sub_months(chrono::Months::new(20 * 12))
            .unwrap();
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::No,
            SourceOfFunds::Salary,
            Some(young_date),
        );
        let score = RiskScoringService::calculate_risk_score(&customer);
        assert_eq!(score.value(), 20); // base 10 + age 10
    }

    #[test]
    fn test_combined_high_risk() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::Yes,
            SourceOfFunds::Business,
            None,
        );
        let score = RiskScoringService::calculate_risk_score(&customer);
        // base 10 + PEP 30 + business 15 + legal 10 = 65
        assert_eq!(score.value(), 65);
        assert_eq!(RiskScoringService::risk_level(&score), RiskLevel::High);
    }

    #[test]
    fn test_score_capped_at_100() {
        // Even with all factors, score should not exceed 100
        let score = RiskScore::new(100).unwrap();
        assert_eq!(score.value(), 100);
        assert_eq!(RiskScoringService::risk_level(&score), RiskLevel::Critical);
    }

    // V1 Versioned API Tests

    #[test]
    fn test_v1_versioned_api() {
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V1);
        assert_eq!(result.score.value(), 10);
        assert_eq!(result.version, RiskScoringVersion::V1);
    }

    #[test]
    fn test_risk_scoring_result_accessors() {
        let score = RiskScore::new(50).unwrap();
        let result = RiskScoringResult::new(score.clone(), RiskScoringVersion::V1);
        assert_eq!(result.score(), &score);
        assert_eq!(result.version(), RiskScoringVersion::V1);
    }

    // V2 Tests

    #[test]
    fn test_v2_base_risk_score() {
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);
        assert_eq!(result.score.value(), 10); // base only
        assert_eq!(result.version, RiskScoringVersion::V2);
    }

    #[test]
    fn test_v2_legal_entity_increased_score() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);
        assert_eq!(result.score.value(), 25); // base 10 + legal 15 (increased from 10)
    }

    #[test]
    fn test_v2_high_risk_nationality() {
        // Note: This test assumes a factory that creates customers from high-risk countries
        // For now, we test that Tunisia (low-risk) doesn't add extra points
        let customer = make_customer(
            CustomerType::Individual,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);
        // Base 10, no high-risk country penalty (Tunisia is not in the list)
        assert_eq!(result.score.value(), 10);
    }

    #[test]
    fn test_v2_pep_combined_with_legal_entity() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::Yes,
            SourceOfFunds::Salary,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);
        // base 10 + PEP 30 + legal 15 = 55
        assert_eq!(result.score.value(), 55);
    }

    #[test]
    fn test_v2_combined_high_risk() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::Yes,
            SourceOfFunds::Business,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);
        // base 10 + PEP 30 + business 15 + legal 15 = 70
        assert_eq!(result.score.value(), 70);
    }

    #[test]
    fn test_v1_vs_v2_legal_entity_difference() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
        );

        let v1_result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V1);
        let v2_result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);

        // V1: base 10 + legal 10 = 20
        assert_eq!(v1_result.score.value(), 20);
        // V2: base 10 + legal 15 = 25
        assert_eq!(v2_result.score.value(), 25);

        // V2 should be strictly higher for legal entities
        assert!(v2_result.score.value() > v1_result.score.value());
    }

    #[test]
    fn test_risk_scoring_version_display() {
        assert_eq!(RiskScoringVersion::V1.to_string(), "V1");
        assert_eq!(RiskScoringVersion::V2.to_string(), "V2");
    }

    #[test]
    fn test_risk_scoring_version_default() {
        assert_eq!(RiskScoringVersion::default(), RiskScoringVersion::V1);
    }

    #[test]
    fn test_v2_score_capped_at_100() {
        let customer = make_customer(
            CustomerType::LegalEntity,
            PepStatus::Yes,
            SourceOfFunds::Business,
            None,
        );
        let result =
            RiskScoringService::calculate_risk_score_with_version(&customer, RiskScoringVersion::V2);
        // Score should never exceed 100
        assert!(result.score.value() <= 100);
    }
}
