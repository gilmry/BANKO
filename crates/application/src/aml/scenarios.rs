use banko_domain::aml::{
    Alert, RiskLevel, Transaction, TransactionType, AML_CASH_THRESHOLD_TND,
};
use banko_domain::shared::Currency;

use super::ports::IAmlScenario;

// --- Threshold Scenario (INV-08) ---

/// INV-08: Cash transaction ≥ 5000 TND → Medium alert
pub struct ThresholdScenario;

impl IAmlScenario for ThresholdScenario {
    fn name(&self) -> &str {
        "threshold_5000_tnd"
    }

    fn evaluate(
        &self,
        transaction: &Transaction,
        _history: &[Transaction],
    ) -> Option<Alert> {
        if transaction.requires_aml_check() {
            Alert::new(
                transaction.id().clone(),
                RiskLevel::Medium,
                format!(
                    "Cash transaction of {:.3} TND exceeds AML threshold of {} TND (INV-08)",
                    transaction.amount().amount(),
                    AML_CASH_THRESHOLD_TND
                ),
            )
            .ok()
        } else {
            None
        }
    }
}

// --- Structuring Scenario ---

/// Detects structured deposits: multiple transactions just below threshold in short period.
pub struct StructuringScenario {
    pub window_hours: i64,
    pub min_transactions: usize,
    pub near_threshold_pct: f64, // e.g., 0.80 → amounts above 80% of threshold
}

impl Default for StructuringScenario {
    fn default() -> Self {
        StructuringScenario {
            window_hours: 24,
            min_transactions: 3,
            near_threshold_pct: 0.80,
        }
    }
}

impl IAmlScenario for StructuringScenario {
    fn name(&self) -> &str {
        "structuring_detection"
    }

    fn evaluate(
        &self,
        transaction: &Transaction,
        history: &[Transaction],
    ) -> Option<Alert> {
        if !transaction.transaction_type().is_cash() {
            return None;
        }
        if transaction.amount().currency() != Currency::TND {
            return None;
        }

        let near_threshold = AML_CASH_THRESHOLD_TND * self.near_threshold_pct;
        let window_start = transaction.timestamp()
            - chrono::Duration::hours(self.window_hours);

        // Count recent cash transactions near threshold from same account
        let near_threshold_count = history
            .iter()
            .filter(|tx| {
                tx.account_id() == transaction.account_id()
                    && tx.transaction_type().is_cash()
                    && tx.amount().currency() == Currency::TND
                    && tx.amount().amount() >= near_threshold
                    && tx.amount().amount() < AML_CASH_THRESHOLD_TND
                    && tx.timestamp() >= window_start
            })
            .count();

        // Include current transaction if it fits
        let current_near = transaction.amount().amount() >= near_threshold
            && transaction.amount().amount() < AML_CASH_THRESHOLD_TND;
        let total = near_threshold_count + if current_near { 1 } else { 0 };

        if total >= self.min_transactions {
            Alert::new(
                transaction.id().clone(),
                RiskLevel::Critical,
                format!(
                    "Potential structuring detected: {} transactions near threshold within {}h",
                    total, self.window_hours
                ),
            )
            .ok()
        } else {
            None
        }
    }
}

// --- High Risk Country Scenario ---

/// Detects transfers to/from high-risk jurisdictions.
pub struct HighRiskCountryScenario {
    pub high_risk_countries: Vec<String>,
}

impl Default for HighRiskCountryScenario {
    fn default() -> Self {
        HighRiskCountryScenario {
            high_risk_countries: vec![
                "AF".to_string(), // Afghanistan
                "KP".to_string(), // North Korea
                "IR".to_string(), // Iran
                "SY".to_string(), // Syria
                "YE".to_string(), // Yemen
                "MM".to_string(), // Myanmar
            ],
        }
    }
}

impl IAmlScenario for HighRiskCountryScenario {
    fn name(&self) -> &str {
        "high_risk_country"
    }

    fn evaluate(
        &self,
        transaction: &Transaction,
        _history: &[Transaction],
    ) -> Option<Alert> {
        if transaction.transaction_type() != TransactionType::Transfer {
            return None;
        }

        let counterparty_upper = transaction.counterparty().to_uppercase();
        for country in &self.high_risk_countries {
            if counterparty_upper.contains(country) {
                return Alert::new(
                    transaction.id().clone(),
                    RiskLevel::High,
                    format!(
                        "Transfer involves high-risk jurisdiction: {}",
                        country
                    ),
                )
                .ok();
            }
        }

        None
    }
}

// --- Volume Scenario ---

/// 3+ transactions > 100k TND in 24h → High alert
pub struct VolumeScenario {
    pub threshold_amount: f64,
    pub window_hours: i64,
    pub min_count: usize,
}

impl Default for VolumeScenario {
    fn default() -> Self {
        VolumeScenario {
            threshold_amount: 100_000.0,
            window_hours: 24,
            min_count: 3,
        }
    }
}

impl IAmlScenario for VolumeScenario {
    fn name(&self) -> &str {
        "high_volume"
    }

    fn evaluate(
        &self,
        transaction: &Transaction,
        history: &[Transaction],
    ) -> Option<Alert> {
        if transaction.amount().currency() != Currency::TND {
            return None;
        }

        let window_start = transaction.timestamp()
            - chrono::Duration::hours(self.window_hours);

        let large_tx_count = history
            .iter()
            .filter(|tx| {
                tx.account_id() == transaction.account_id()
                    && tx.amount().currency() == Currency::TND
                    && tx.amount().amount() > self.threshold_amount
                    && tx.timestamp() >= window_start
            })
            .count();

        let current_large = transaction.amount().amount() > self.threshold_amount;
        let total = large_tx_count + if current_large { 1 } else { 0 };

        if total >= self.min_count {
            Alert::new(
                transaction.id().clone(),
                RiskLevel::High,
                format!(
                    "High volume: {} transactions > {:.0} TND within {}h",
                    total, self.threshold_amount, self.window_hours
                ),
            )
            .ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use banko_domain::aml::*;
    use banko_domain::shared::{Currency, Money};

    use super::*;

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    fn make_tx(amount: f64, tx_type: TransactionType) -> Transaction {
        Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test Counterparty".to_string(),
            tnd(amount),
            tx_type,
            Direction::Inbound,
            Utc::now(),
        )
        .unwrap()
    }

    fn make_tx_with_account(amount: f64, tx_type: TransactionType, account_id: Uuid) -> Transaction {
        Transaction::new(
            account_id,
            Uuid::new_v4(),
            "Test".to_string(),
            tnd(amount),
            tx_type,
            Direction::Inbound,
            Utc::now(),
        )
        .unwrap()
    }

    fn make_transfer_to(counterparty: &str, amount: f64) -> Transaction {
        Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            counterparty.to_string(),
            tnd(amount),
            TransactionType::Transfer,
            Direction::Outbound,
            Utc::now(),
        )
        .unwrap()
    }

    // --- Threshold scenario (INV-08) ---

    #[test]
    fn test_threshold_triggers_on_cash_above_5000() {
        let scenario = ThresholdScenario;
        let tx = make_tx(6000.0, TransactionType::Deposit);
        let alert = scenario.evaluate(&tx, &[]);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().risk_level(), RiskLevel::Medium);
    }

    #[test]
    fn test_threshold_no_trigger_below_5000() {
        let scenario = ThresholdScenario;
        let tx = make_tx(4999.0, TransactionType::Deposit);
        assert!(scenario.evaluate(&tx, &[]).is_none());
    }

    #[test]
    fn test_threshold_no_trigger_on_transfer() {
        let scenario = ThresholdScenario;
        let tx = make_tx(10000.0, TransactionType::Transfer);
        assert!(scenario.evaluate(&tx, &[]).is_none());
    }

    // --- Structuring scenario ---

    #[test]
    fn test_structuring_detected() {
        let scenario = StructuringScenario::default();
        let account_id = Uuid::new_v4();

        let history: Vec<Transaction> = (0..3)
            .map(|_| make_tx_with_account(4500.0, TransactionType::Deposit, account_id))
            .collect();

        let tx = make_tx_with_account(4800.0, TransactionType::Deposit, account_id);
        let alert = scenario.evaluate(&tx, &history);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().risk_level(), RiskLevel::Critical);
    }

    #[test]
    fn test_structuring_not_detected_small_amounts() {
        let scenario = StructuringScenario::default();
        let account_id = Uuid::new_v4();

        let history: Vec<Transaction> = (0..2)
            .map(|_| make_tx_with_account(1000.0, TransactionType::Deposit, account_id))
            .collect();

        let tx = make_tx_with_account(1000.0, TransactionType::Deposit, account_id);
        assert!(scenario.evaluate(&tx, &history).is_none());
    }

    // --- High risk country ---

    #[test]
    fn test_high_risk_country_detected() {
        let scenario = HighRiskCountryScenario::default();
        let tx = make_transfer_to("Company in AF region", 10000.0);
        let alert = scenario.evaluate(&tx, &[]);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().risk_level(), RiskLevel::High);
    }

    #[test]
    fn test_high_risk_country_not_detected() {
        let scenario = HighRiskCountryScenario::default();
        let tx = make_transfer_to("Company in France", 10000.0);
        assert!(scenario.evaluate(&tx, &[]).is_none());
    }

    #[test]
    fn test_high_risk_country_only_transfers() {
        let scenario = HighRiskCountryScenario::default();
        // Deposit (not transfer) with "AF" in counterparty
        let tx = Transaction::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "AF person".to_string(),
            tnd(5000.0),
            TransactionType::Deposit,
            Direction::Inbound,
            Utc::now(),
        )
        .unwrap();
        assert!(scenario.evaluate(&tx, &[]).is_none());
    }

    // --- Volume scenario ---

    #[test]
    fn test_volume_detected() {
        let scenario = VolumeScenario::default();
        let account_id = Uuid::new_v4();

        let history: Vec<Transaction> = (0..3)
            .map(|_| make_tx_with_account(150_000.0, TransactionType::Transfer, account_id))
            .collect();

        let tx = make_tx_with_account(200_000.0, TransactionType::Transfer, account_id);
        let alert = scenario.evaluate(&tx, &history);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().risk_level(), RiskLevel::High);
    }

    #[test]
    fn test_volume_not_detected_low_count() {
        let scenario = VolumeScenario::default();
        let account_id = Uuid::new_v4();

        let history = vec![
            make_tx_with_account(150_000.0, TransactionType::Transfer, account_id),
        ];

        let tx = make_tx_with_account(200_000.0, TransactionType::Transfer, account_id);
        assert!(scenario.evaluate(&tx, &history).is_none());
    }
}
