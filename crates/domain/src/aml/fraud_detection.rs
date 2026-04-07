use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Context information for evaluating transaction fraud risk
#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub customer_id: Uuid,
    pub transaction_amount: Decimal,
    pub currency: String,
    pub destination_country: Option<String>,
    pub source_ip: Option<String>,
    pub customer_avg_daily: Decimal,
    pub customer_avg_monthly: Decimal,
    pub transaction_count_last_hour: u32,
    pub transaction_count_last_day: u32,
    pub is_new_beneficiary: bool,
    pub account_age_days: u32,
}

impl TransactionContext {
    pub fn new(customer_id: Uuid, transaction_amount: Decimal, currency: String) -> Self {
        Self {
            customer_id,
            transaction_amount,
            currency,
            destination_country: None,
            source_ip: None,
            customer_avg_daily: Decimal::ZERO,
            customer_avg_monthly: Decimal::ZERO,
            transaction_count_last_hour: 0,
            transaction_count_last_day: 0,
            is_new_beneficiary: false,
            account_age_days: 0,
        }
    }

    pub fn with_destination(mut self, country: String) -> Self {
        self.destination_country = Some(country);
        self
    }

    pub fn with_source_ip(mut self, ip: String) -> Self {
        self.source_ip = Some(ip);
        self
    }

    pub fn with_customer_history(
        mut self,
        avg_daily: Decimal,
        avg_monthly: Decimal,
        tx_count_hour: u32,
        tx_count_day: u32,
    ) -> Self {
        self.customer_avg_daily = avg_daily;
        self.customer_avg_monthly = avg_monthly;
        self.transaction_count_last_hour = tx_count_hour;
        self.transaction_count_last_day = tx_count_day;
        self
    }

    pub fn with_beneficiary_info(mut self, is_new: bool) -> Self {
        self.is_new_beneficiary = is_new;
        self
    }

    pub fn with_account_age(mut self, age_days: u32) -> Self {
        self.account_age_days = age_days;
        self
    }
}

/// Score for a single fraud rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudScore {
    pub rule_name: String,
    pub score: u32, // 0-100
    pub reason: String,
}

impl FraudScore {
    pub fn new(rule_name: &str, score: u32, reason: impl Into<String>) -> Self {
        Self {
            rule_name: rule_name.to_string(),
            score: score.min(100),
            reason: reason.into(),
        }
    }
}

/// Final fraud decision for a transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FraudDecision {
    Allow,
    Challenge,
    Block,
    ManualReview,
}

impl fmt::Display for FraudDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FraudDecision::Allow => write!(f, "Allow"),
            FraudDecision::Challenge => write!(f, "Challenge"),
            FraudDecision::Block => write!(f, "Block"),
            FraudDecision::ManualReview => write!(f, "ManualReview"),
        }
    }
}

/// Trait for fraud detection rules
#[async_trait]
pub trait FraudRule: Send + Sync {
    fn name(&self) -> &str;
    async fn evaluate(&self, context: &TransactionContext) -> FraudScore;
}

/// Rule: Transaction amount significantly higher than customer history
pub struct AbnormalAmountRule;

#[async_trait]
impl FraudRule for AbnormalAmountRule {
    fn name(&self) -> &str {
        "AbnormalAmountRule"
    }

    async fn evaluate(&self, context: &TransactionContext) -> FraudScore {
        if context.customer_avg_daily == Decimal::ZERO {
            return FraudScore::new("AbnormalAmountRule", 0, "No historical data");
        }

        let multiplier_10x = context.transaction_amount / context.customer_avg_daily;
        let multiplier_50x = context.transaction_amount / context.customer_avg_daily;

        if multiplier_50x >= Decimal::from(50) {
            FraudScore::new(
                "AbnormalAmountRule",
                70,
                format!(
                    "Transaction amount {} is 50x+ customer daily average {}",
                    context.transaction_amount, context.customer_avg_daily
                )
                .as_str(),
            )
        } else if multiplier_10x >= Decimal::from(10) {
            FraudScore::new(
                "AbnormalAmountRule",
                40,
                format!(
                    "Transaction amount {} is 10x+ customer daily average {}",
                    context.transaction_amount, context.customer_avg_daily
                )
                .as_str(),
            )
        } else {
            FraudScore::new("AbnormalAmountRule", 0, "Amount within normal range")
        }
    }
}

/// Rule: Destination country is in FATF high-risk list
pub struct HighRiskCountryRule {
    high_risk_countries: Vec<String>,
}

impl HighRiskCountryRule {
    pub fn new() -> Self {
        // FATF grey list and black list countries (simplified)
        let countries = ["IR", "SY", "KP", "JO", "LA", "MM", "TZ", "UA", "VN"]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            high_risk_countries: countries,
        }
    }

    pub fn with_countries(countries: Vec<String>) -> Self {
        Self {
            high_risk_countries: countries,
        }
    }
}

impl Default for HighRiskCountryRule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FraudRule for HighRiskCountryRule {
    fn name(&self) -> &str {
        "HighRiskCountryRule"
    }

    async fn evaluate(&self, context: &TransactionContext) -> FraudScore {
        if let Some(ref country) = context.destination_country {
            if self.high_risk_countries.contains(country) {
                return FraudScore::new(
                    "HighRiskCountryRule",
                    40,
                    format!("Destination country {} is high-risk", country),
                );
            }
        }

        FraudScore::new(
            "HighRiskCountryRule",
            0,
            "Destination country not in high-risk list",
        )
    }
}

/// Rule: Unusual transaction velocity
pub struct VelocityRule;

#[async_trait]
impl FraudRule for VelocityRule {
    fn name(&self) -> &str {
        "VelocityRule"
    }

    async fn evaluate(&self, context: &TransactionContext) -> FraudScore {
        if context.transaction_count_last_hour > 10 {
            FraudScore::new(
                "VelocityRule",
                50,
                format!(
                    "High transaction velocity: {} transactions in last hour",
                    context.transaction_count_last_hour
                ),
            )
        } else if context.transaction_count_last_hour > 5 {
            FraudScore::new(
                "VelocityRule",
                30,
                format!(
                    "Elevated transaction velocity: {} transactions in last hour",
                    context.transaction_count_last_hour
                ),
            )
        } else {
            FraudScore::new("VelocityRule", 0, "Transaction velocity normal")
        }
    }
}

/// Rule: New beneficiary with high transaction amount
pub struct NewBeneficiaryHighAmountRule;

#[async_trait]
impl FraudRule for NewBeneficiaryHighAmountRule {
    fn name(&self) -> &str {
        "NewBeneficiaryHighAmountRule"
    }

    async fn evaluate(&self, context: &TransactionContext) -> FraudScore {
        if context.is_new_beneficiary && context.transaction_amount > Decimal::from(5000) {
            FraudScore::new(
                "NewBeneficiaryHighAmountRule",
                25,
                format!(
                    "New beneficiary with high amount: {}",
                    context.transaction_amount
                ),
            )
        } else {
            FraudScore::new(
                "NewBeneficiaryHighAmountRule",
                0,
                "Beneficiary is established or amount is low",
            )
        }
    }
}

/// Rule: New account with high transaction amount
pub struct NewAccountHighAmountRule;

#[async_trait]
impl FraudRule for NewAccountHighAmountRule {
    fn name(&self) -> &str {
        "NewAccountHighAmountRule"
    }

    async fn evaluate(&self, context: &TransactionContext) -> FraudScore {
        if context.account_age_days < 30 && context.transaction_amount > Decimal::from(10000) {
            FraudScore::new(
                "NewAccountHighAmountRule",
                35,
                format!(
                    "New account ({} days old) with high transaction: {}",
                    context.account_age_days, context.transaction_amount
                ),
            )
        } else {
            FraudScore::new(
                "NewAccountHighAmountRule",
                0,
                "Account is established or amount is low",
            )
        }
    }
}

/// Complete fraud evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudEvaluation {
    pub total_score: u32,
    pub decision: FraudDecision,
    pub rule_scores: Vec<FraudScore>,
}

impl FraudEvaluation {
    pub fn new(total_score: u32, decision: FraudDecision, rule_scores: Vec<FraudScore>) -> Self {
        Self {
            total_score: total_score.min(100),
            decision,
            rule_scores,
        }
    }
}

/// Orchestrates fraud detection using multiple rules
pub struct FraudDetector {
    rules: Vec<Box<dyn FraudRule>>,
    threshold_challenge: u32,
    threshold_block: u32,
}

impl FraudDetector {
    pub fn new() -> Self {
        let rules: Vec<Box<dyn FraudRule>> = vec![
            Box::new(AbnormalAmountRule),
            Box::new(HighRiskCountryRule::new()),
            Box::new(VelocityRule),
            Box::new(NewBeneficiaryHighAmountRule),
            Box::new(NewAccountHighAmountRule),
        ];

        Self {
            rules,
            threshold_challenge: 50,
            threshold_block: 75,
        }
    }

    pub fn with_thresholds(mut self, challenge: u32, block: u32) -> Self {
        self.threshold_challenge = challenge;
        self.threshold_block = block;
        self
    }

    pub fn with_rules(mut self, rules: Vec<Box<dyn FraudRule>>) -> Self {
        self.rules = rules;
        self
    }

    pub async fn evaluate(&self, context: &TransactionContext) -> FraudEvaluation {
        let mut total_score: u32 = 0;
        let mut rule_scores = Vec::new();

        // Evaluate all rules
        for rule in &self.rules {
            let score = rule.evaluate(context).await;
            total_score = (total_score + score.score).min(100);
            rule_scores.push(score);
        }

        // Determine decision based on total score
        let decision = if total_score >= self.threshold_block {
            FraudDecision::Block
        } else if total_score >= self.threshold_challenge {
            FraudDecision::Challenge
        } else {
            FraudDecision::Allow
        };

        FraudEvaluation::new(total_score, decision, rule_scores)
    }
}

impl Default for FraudDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_abnormal_amount_rule_10x_threshold() {
        let rule = AbnormalAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(1000), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 0, 0);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 40);
    }

    #[tokio::test]
    async fn test_abnormal_amount_rule_50x_threshold() {
        let rule = AbnormalAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(5000), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 0, 0);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 70);
    }

    #[tokio::test]
    async fn test_abnormal_amount_rule_within_range() {
        let rule = AbnormalAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(150), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 0, 0);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 0);
    }

    #[tokio::test]
    async fn test_high_risk_country_rule_blocks_iran() {
        let rule = HighRiskCountryRule::new();
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(100), "USD".to_string())
            .with_destination("IR".to_string());

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 40);
    }

    #[tokio::test]
    async fn test_high_risk_country_rule_allows_safe_country() {
        let rule = HighRiskCountryRule::new();
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(100), "USD".to_string())
            .with_destination("US".to_string());

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 0);
    }

    #[tokio::test]
    async fn test_velocity_rule_10plus_transactions() {
        let rule = VelocityRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(100), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 11, 0);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 50);
    }

    #[tokio::test]
    async fn test_velocity_rule_5plus_transactions() {
        let rule = VelocityRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(100), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 6, 0);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 30);
    }

    #[tokio::test]
    async fn test_velocity_rule_normal() {
        let rule = VelocityRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(100), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 2, 0);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 0);
    }

    #[tokio::test]
    async fn test_new_beneficiary_high_amount_rule() {
        let rule = NewBeneficiaryHighAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(6000), "USD".to_string())
            .with_beneficiary_info(true);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 25);
    }

    #[tokio::test]
    async fn test_new_beneficiary_low_amount_rule() {
        let rule = NewBeneficiaryHighAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(1000), "USD".to_string())
            .with_beneficiary_info(true);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 0);
    }

    #[tokio::test]
    async fn test_new_account_high_amount_rule() {
        let rule = NewAccountHighAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(11000), "USD".to_string())
            .with_account_age(15);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 35);
    }

    #[tokio::test]
    async fn test_new_account_low_amount_rule() {
        let rule = NewAccountHighAmountRule;
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(5000), "USD".to_string())
            .with_account_age(15);

        let score = rule.evaluate(&context).await;
        assert_eq!(score.score, 0);
    }

    #[tokio::test]
    async fn test_fraud_detector_allows_low_risk() {
        let detector = FraudDetector::new();
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(150), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 1, 5)
            .with_destination("US".to_string())
            .with_beneficiary_info(false)
            .with_account_age(90);

        let eval = detector.evaluate(&context).await;
        assert_eq!(eval.decision, FraudDecision::Allow);
        assert!(eval.total_score < 50);
    }

    #[tokio::test]
    async fn test_fraud_detector_challenges_medium_risk() {
        let detector = FraudDetector::new();
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(1000), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 6, 10)
            .with_destination("US".to_string())
            .with_beneficiary_info(false)
            .with_account_age(90);

        let eval = detector.evaluate(&context).await;
        assert_eq!(eval.decision, FraudDecision::Challenge);
        assert!(eval.total_score >= 50 && eval.total_score < 75);
    }

    #[tokio::test]
    async fn test_fraud_detector_blocks_high_risk() {
        let detector = FraudDetector::new();
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(5000), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 11, 10)
            .with_destination("IR".to_string())
            .with_beneficiary_info(true)
            .with_account_age(15);

        let eval = detector.evaluate(&context).await;
        assert_eq!(eval.decision, FraudDecision::Block);
        assert!(eval.total_score >= 75);
    }

    #[tokio::test]
    async fn test_fraud_evaluation_contains_all_rule_scores() {
        let detector = FraudDetector::new();
        let context = TransactionContext::new(Uuid::new_v4(), Decimal::from(1000), "USD".to_string())
            .with_customer_history(Decimal::from(100), Decimal::from(3000), 0, 0);

        let eval = detector.evaluate(&context).await;
        assert!(!eval.rule_scores.is_empty());
        assert!(eval.rule_scores.len() >= 5); // At least 5 rules
    }
}
