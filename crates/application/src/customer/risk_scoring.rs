use banko_domain::customer::{
    Customer, CustomerType, PepStatus, RiskLevel, RiskScore, SourceOfFunds,
};

/// Calculates risk scores for customers based on their profile attributes.
pub struct RiskScoringService;

impl RiskScoringService {
    /// Calculate the risk score for a customer based on their KYC profile.
    pub fn calculate_risk_score(customer: &Customer) -> RiskScore {
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
}
