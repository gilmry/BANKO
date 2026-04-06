use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ==================== Currency Enum ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    TND,
    EUR,
    USD,
    GBP,
    SAR,
    AED,
    LYD,
    DZD,
    MAD,
}

impl Currency {
    /// Get the ISO 4217 code for the currency
    pub fn code(&self) -> &str {
        match self {
            Currency::TND => "TND",
            Currency::EUR => "EUR",
            Currency::USD => "USD",
            Currency::GBP => "GBP",
            Currency::SAR => "SAR",
            Currency::AED => "AED",
            Currency::LYD => "LYD",
            Currency::DZD => "DZD",
            Currency::MAD => "MAD",
        }
    }

    /// Get the French name for the currency
    pub fn name_fr(&self) -> &str {
        match self {
            Currency::TND => "Dinar tunisien",
            Currency::EUR => "Euro",
            Currency::USD => "Dollar américain",
            Currency::GBP => "Livre sterling",
            Currency::SAR => "Riyal saoudien",
            Currency::AED => "Dirham des Émirats",
            Currency::LYD => "Dinar libyen",
            Currency::DZD => "Dinar algérien",
            Currency::MAD => "Dirham marocain",
        }
    }

    /// Get the number of decimal places for the currency
    pub fn decimal_places(&self) -> u8 {
        match self {
            Currency::TND => 3,
            Currency::EUR | Currency::USD | Currency::GBP | Currency::SAR
            | Currency::AED | Currency::LYD | Currency::DZD | Currency::MAD => 2,
        }
    }

    /// Parse currency from code string
    pub fn from_code(code: &str) -> Option<Currency> {
        match code {
            "TND" => Some(Currency::TND),
            "EUR" => Some(Currency::EUR),
            "USD" => Some(Currency::USD),
            "GBP" => Some(Currency::GBP),
            "SAR" => Some(Currency::SAR),
            "AED" => Some(Currency::AED),
            "LYD" => Some(Currency::LYD),
            "DZD" => Some(Currency::DZD),
            "MAD" => Some(Currency::MAD),
            _ => None,
        }
    }
}

// ==================== MultiCurrencyBalance ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCurrencyBalance {
    balances: HashMap<Currency, Decimal>,
}

impl MultiCurrencyBalance {
    /// Create a new empty multi-currency balance
    pub fn new() -> Self {
        MultiCurrencyBalance {
            balances: HashMap::new(),
        }
    }

    /// Add an amount to a specific currency
    pub fn add(&mut self, currency: Currency, amount: Decimal) -> Result<(), DomainError> {
        if amount < Decimal::ZERO {
            return Err(DomainError::InvalidMovement(
                "Amount must be non-negative".to_string(),
            ));
        }

        let entry = self.balances.entry(currency).or_insert(Decimal::ZERO);
        *entry += amount;
        Ok(())
    }

    /// Get the balance for a specific currency
    pub fn get(&self, currency: &Currency) -> Decimal {
        self.balances.get(currency).copied().unwrap_or(Decimal::ZERO)
    }

    /// Get all balances
    pub fn all_balances(&self) -> &HashMap<Currency, Decimal> {
        &self.balances
    }

    /// Convert all balances to a base currency using provided exchange rates
    pub fn total_in_base(
        &self,
        rates: &HashMap<(Currency, Currency), Decimal>,
        base: Currency,
    ) -> Result<Decimal, DomainError> {
        let mut total = Decimal::ZERO;

        for (&currency, &amount) in &self.balances {
            if currency == base {
                total += amount;
            } else {
                let key = (currency, base);
                let rate = rates
                    .get(&key)
                    .ok_or_else(|| {
                        DomainError::InvalidMovement(format!(
                            "No exchange rate found for {:?}/{:?}",
                            currency, base
                        ))
                    })?;
                total += amount * rate;
            }
        }

        Ok(total)
    }
}

impl Default for MultiCurrencyBalance {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== CurrencyConverter ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub original_amount: Decimal,
    pub original_currency: Currency,
    pub converted_amount: Decimal,
    pub target_currency: Currency,
    pub market_rate: Decimal,
    pub bank_rate: Decimal,
    pub margin_applied: Decimal,
    pub conversion_date: DateTime<Utc>,
}

pub struct CurrencyConverter {
    bank_margin_percent: Decimal,
}

impl CurrencyConverter {
    /// Create a new currency converter with default bank margin (2%)
    pub fn new() -> Self {
        CurrencyConverter {
            bank_margin_percent: Decimal::from(2),
        }
    }

    /// Create a new currency converter with custom bank margin
    pub fn with_margin(margin_percent: Decimal) -> Self {
        CurrencyConverter {
            bank_margin_percent: margin_percent,
        }
    }

    /// Perform currency conversion with bank margin applied
    /// For buy (selling TND): bank_rate = market_rate * (1 + margin/100)
    /// For sell (buying TND): bank_rate = market_rate * (1 - margin/100)
    pub fn convert(
        &self,
        amount: Decimal,
        from: Currency,
        to: Currency,
        market_rate: Decimal,
        is_buying_base: bool,
    ) -> Result<ConversionResult, DomainError> {
        if amount <= Decimal::ZERO {
            return Err(DomainError::InvalidMovement(
                "Amount must be positive".to_string(),
            ));
        }

        if market_rate <= Decimal::ZERO {
            return Err(DomainError::InvalidMovement(
                "Exchange rate must be positive".to_string(),
            ));
        }

        if from == to {
            return Err(DomainError::InvalidMovement(
                "Source and target currencies must be different".to_string(),
            ));
        }

        // Apply margin: buying base (expensive) adds margin, selling base (cheap) subtracts
        let margin_multiplier = if is_buying_base {
            Decimal::ONE + (self.bank_margin_percent / Decimal::from(100))
        } else {
            Decimal::ONE - (self.bank_margin_percent / Decimal::from(100))
        };

        let bank_rate = market_rate * margin_multiplier;
        let converted_amount = (amount * bank_rate)
            .round_dp(to.decimal_places() as u32);

        Ok(ConversionResult {
            original_amount: amount,
            original_currency: from,
            converted_amount,
            target_currency: to,
            market_rate,
            bank_rate,
            margin_applied: self.bank_margin_percent,
            conversion_date: Utc::now(),
        })
    }

    /// Check monthly limit for currency conversion
    pub fn check_monthly_limit(
        customer_id: Uuid,
        currency: Currency,
        amount: Decimal,
        monthly_limit: Decimal,
        already_converted: Decimal,
    ) -> Result<(), DomainError> {
        if already_converted + amount > monthly_limit {
            return Err(DomainError::InvalidMovement(format!(
                "Monthly limit exceeded for {}. Limit: {}, Already converted: {}, Requested: {}",
                currency.code(),
                monthly_limit,
                already_converted,
                amount
            )));
        }
        Ok(())
    }
}

impl Default for CurrencyConverter {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::TND.code(), "TND");
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::USD.code(), "USD");
    }

    #[test]
    fn test_currency_name_fr() {
        assert_eq!(Currency::TND.name_fr(), "Dinar tunisien");
        assert_eq!(Currency::EUR.name_fr(), "Euro");
    }

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::TND.decimal_places(), 3);
        assert_eq!(Currency::EUR.decimal_places(), 2);
        assert_eq!(Currency::USD.decimal_places(), 2);
    }

    #[test]
    fn test_currency_from_code() {
        assert_eq!(Currency::from_code("TND"), Some(Currency::TND));
        assert_eq!(Currency::from_code("EUR"), Some(Currency::EUR));
        assert_eq!(Currency::from_code("INVALID"), None);
    }

    #[test]
    fn test_multi_currency_balance_add() {
        let mut balance = MultiCurrencyBalance::new();
        balance
            .add(Currency::TND, Decimal::from(100))
            .unwrap();
        balance.add(Currency::EUR, Decimal::from(50)).unwrap();

        assert_eq!(balance.get(&Currency::TND), Decimal::from(100));
        assert_eq!(balance.get(&Currency::EUR), Decimal::from(50));
    }

    #[test]
    fn test_multi_currency_balance_add_negative_fails() {
        let mut balance = MultiCurrencyBalance::new();
        let result = balance.add(Currency::TND, Decimal::from(-100));
        assert!(result.is_err());
    }

    #[test]
    fn test_multi_currency_balance_get_nonexistent() {
        let balance = MultiCurrencyBalance::new();
        assert_eq!(balance.get(&Currency::USD), Decimal::ZERO);
    }

    #[test]
    fn test_multi_currency_balance_multiple_adds() {
        let mut balance = MultiCurrencyBalance::new();
        balance.add(Currency::TND, Decimal::from(100)).unwrap();
        balance.add(Currency::TND, Decimal::from(50)).unwrap();
        assert_eq!(balance.get(&Currency::TND), Decimal::from(150));
    }

    #[test]
    fn test_currency_converter_new() {
        let converter = CurrencyConverter::new();
        assert_eq!(converter.bank_margin_percent, Decimal::from(2));
    }

    #[test]
    fn test_currency_converter_custom_margin() {
        let converter = CurrencyConverter::with_margin(Decimal::from(3));
        assert_eq!(converter.bank_margin_percent, Decimal::from(3));
    }

    #[test]
    fn test_currency_converter_buying_base() {
        let converter = CurrencyConverter::new(); // 2% margin
        let result = converter
            .convert(
                Decimal::from(100),
                Currency::EUR,
                Currency::TND,
                Decimal::from_str_exact("3.0").unwrap(),
                true, // buying TND (expensive)
            )
            .unwrap();

        // market_rate = 3.0, margin applied = 2%, so bank_rate = 3.0 * 1.02 = 3.06
        // 100 EUR * 3.06 = 306 TND
        assert_eq!(result.original_amount, Decimal::from(100));
        assert_eq!(result.original_currency, Currency::EUR);
        assert_eq!(result.target_currency, Currency::TND);
        assert_eq!(result.market_rate, Decimal::from_str_exact("3.0").unwrap());
        assert!(result.converted_amount > Decimal::from(300)); // Should be > 300
    }

    #[test]
    fn test_currency_converter_selling_base() {
        let converter = CurrencyConverter::new(); // 2% margin
        let result = converter
            .convert(
                Decimal::from(300),
                Currency::TND,
                Currency::EUR,
                Decimal::from_str_exact("3.0").unwrap(),
                false, // selling TND (cheaper)
            )
            .unwrap();

        // market_rate = 3.0, margin applied = 2%, so bank_rate = 3.0 * (1 - 0.02) = 2.94
        // 300 TND / 3.0 at bank_rate 2.94 = 300 * 2.94 / 3.0 = 294
        // Wait, the formula is: amount * bank_rate
        // So 300 * 2.94 / 3.0 = 294
        assert_eq!(result.original_amount, Decimal::from(300));
        assert_eq!(result.margin_applied, Decimal::from(2));
    }

    #[test]
    fn test_currency_converter_zero_amount_fails() {
        let converter = CurrencyConverter::new();
        let result = converter.convert(
            Decimal::ZERO,
            Currency::TND,
            Currency::EUR,
            Decimal::from(3),
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_currency_converter_negative_amount_fails() {
        let converter = CurrencyConverter::new();
        let result = converter.convert(
            Decimal::from(-100),
            Currency::TND,
            Currency::EUR,
            Decimal::from(3),
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_currency_converter_zero_rate_fails() {
        let converter = CurrencyConverter::new();
        let result = converter.convert(
            Decimal::from(100),
            Currency::TND,
            Currency::EUR,
            Decimal::ZERO,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_currency_converter_same_currency_fails() {
        let converter = CurrencyConverter::new();
        let result = converter.convert(
            Decimal::from(100),
            Currency::EUR,
            Currency::EUR,
            Decimal::from(1),
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_check_monthly_limit_within_limit() {
        let customer_id = Uuid::new_v4();
        let result = CurrencyConverter::check_monthly_limit(
            customer_id,
            Currency::TND,
            Decimal::from(1000),
            Decimal::from(10000),
            Decimal::from(5000),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_monthly_limit_exceeds() {
        let customer_id = Uuid::new_v4();
        let result = CurrencyConverter::check_monthly_limit(
            customer_id,
            Currency::TND,
            Decimal::from(6000),
            Decimal::from(10000),
            Decimal::from(5000),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_check_monthly_limit_at_boundary() {
        let customer_id = Uuid::new_v4();
        let result = CurrencyConverter::check_monthly_limit(
            customer_id,
            Currency::EUR,
            Decimal::from(5000),
            Decimal::from(10000),
            Decimal::from(5000),
        );
        assert!(result.is_ok()); // Exactly at limit is ok
    }

    #[test]
    fn test_multi_currency_balance_total_in_base() {
        let mut balance = MultiCurrencyBalance::new();
        balance.add(Currency::TND, Decimal::from(100)).unwrap();
        balance.add(Currency::EUR, Decimal::from(50)).unwrap();

        let mut rates = HashMap::new();
        rates.insert((Currency::TND, Currency::TND), Decimal::from(1));
        rates.insert((Currency::EUR, Currency::TND), Decimal::from(3));

        let total = balance.total_in_base(&rates, Currency::TND).unwrap();
        // 100 TND + 50 EUR * 3 = 100 + 150 = 250
        assert_eq!(total, Decimal::from(250));
    }

    #[test]
    fn test_multi_currency_balance_total_in_base_missing_rate() {
        let mut balance = MultiCurrencyBalance::new();
        balance.add(Currency::TND, Decimal::from(100)).unwrap();
        balance.add(Currency::EUR, Decimal::from(50)).unwrap();

        let rates = HashMap::new(); // Empty rates

        let result = balance.total_in_base(&rates, Currency::TND);
        assert!(result.is_err());
    }
}
