use std::sync::Arc;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use banko_domain::account::{Currency, CurrencyConverter, ConversionResult, AccountId};
use banko_domain::shared::errors::DomainError;
use banko_domain::shared::value_objects::{self, CustomerId, Rib};

use super::errors::AccountServiceError;
use super::ports::IAccountRepository;

/// Convert shared Currency to account multi-currency Currency
fn to_account_currency(c: value_objects::Currency) -> Currency {
    match c {
        value_objects::Currency::TND => Currency::TND,
        value_objects::Currency::EUR => Currency::EUR,
        value_objects::Currency::USD => Currency::USD,
        value_objects::Currency::GBP => Currency::GBP,
        value_objects::Currency::LYD => Currency::LYD,
    }
}

// ==================== ConsolidatedBalance ====================

#[derive(Debug, Clone)]
pub struct ConsolidatedBalance {
    pub balances: Vec<(Currency, Decimal)>,
    pub total_tnd: Decimal,
    pub rates_used: Vec<(Currency, Decimal)>,
}

// ==================== MultiCurrencyService ====================

pub struct MultiCurrencyService {
    account_repo: Arc<dyn IAccountRepository>,
    converter: CurrencyConverter,
}

impl MultiCurrencyService {
    pub fn new(account_repo: Arc<dyn IAccountRepository>) -> Self {
        MultiCurrencyService {
            account_repo,
            converter: CurrencyConverter::new(),
        }
    }

    pub fn with_margin(account_repo: Arc<dyn IAccountRepository>, margin: Decimal) -> Self {
        MultiCurrencyService {
            account_repo,
            converter: CurrencyConverter::with_margin(margin),
        }
    }

    /// Get consolidated balance across all accounts for a customer
    pub async fn get_consolidated_balance(
        &self,
        customer_id: Uuid,
    ) -> Result<ConsolidatedBalance, AccountServiceError> {
        // Fetch all accounts for the customer
        let cid = CustomerId::from_uuid(customer_id);
        let accounts = self
            .account_repo
            .find_by_customer_id(&cid)
            .await
            .map_err(AccountServiceError::Internal)?;

        // In a real system, we'd fetch current exchange rates
        // For now, use simplified rates
        let mut rates = Vec::new();
        rates.push((Currency::TND, Decimal::from(1)));
        rates.push((Currency::EUR, Decimal::from(3)));
        rates.push((Currency::USD, Decimal::from_str_exact("3.1").unwrap_or(Decimal::ZERO)));
        rates.push((Currency::GBP, Decimal::from_str_exact("3.9").unwrap_or(Decimal::ZERO)));

        let mut balances = Vec::new();
        let mut total_tnd = Decimal::ZERO;

        for account in accounts {
            let balance = account.balance();
            let currency = to_account_currency(balance.currency());
            let amount = balance.amount();

            balances.push((currency, Decimal::from_str_exact(&amount.to_string()).unwrap_or(Decimal::ZERO)));

            // Convert to TND
            if currency != Currency::TND {
                if let Some((_, rate)) = rates.iter().find(|(c, _)| *c == currency) {
                    total_tnd += Decimal::from_str_exact(&amount.to_string()).unwrap_or(Decimal::ZERO) * rate;
                }
            } else {
                total_tnd += Decimal::from_str_exact(&amount.to_string()).unwrap_or(Decimal::ZERO);
            }
        }

        Ok(ConsolidatedBalance {
            balances,
            total_tnd,
            rates_used: rates,
        })
    }

    /// Convert between two accounts of the same customer
    pub async fn convert_between_accounts(
        &self,
        from_account_id: Uuid,
        to_account_id: Uuid,
        amount: Decimal,
        customer_id: Uuid,
    ) -> Result<ConversionResult, AccountServiceError> {
        // Fetch both accounts
        let from_aid = AccountId::from_uuid(from_account_id);
        let from_account = self
            .account_repo
            .find_by_id(&from_aid)
            .await
            .map_err(AccountServiceError::Internal)?
            .ok_or(AccountServiceError::AccountNotFound)?;

        let to_aid = AccountId::from_uuid(to_account_id);
        let to_account = self
            .account_repo
            .find_by_id(&to_aid)
            .await
            .map_err(AccountServiceError::Internal)?
            .ok_or(AccountServiceError::AccountNotFound)?;

        // Validate same customer
        let expected_cid = CustomerId::from_uuid(customer_id);
        if from_account.customer_id() != &expected_cid
            || to_account.customer_id() != &expected_cid
        {
            return Err(AccountServiceError::InvalidInput(
                "Both accounts must belong to the same customer".to_string(),
            ));
        }

        let from_currency = to_account_currency(from_account.balance().currency());
        let to_currency = to_account_currency(to_account.balance().currency());

        // Check monthly limit (simplified - in real system would check against customer preferences)
        let monthly_limit = Decimal::from(100000);
        let already_converted = Decimal::ZERO; // Would fetch from conversion history

        CurrencyConverter::check_monthly_limit(
            customer_id,
            from_currency,
            amount,
            monthly_limit,
            already_converted,
        )
        .map_err(|e| AccountServiceError::InvalidInput(e.to_string()))?;

        // Perform conversion (simplified market rate)
        let market_rate = match (from_currency, to_currency) {
            (Currency::EUR, Currency::TND) => Decimal::from(3),
            (Currency::TND, Currency::EUR) => Decimal::from_str_exact("0.333333").unwrap_or(Decimal::ZERO),
            (Currency::USD, Currency::TND) => Decimal::from_str_exact("3.1").unwrap_or(Decimal::ZERO),
            (Currency::TND, Currency::USD) => Decimal::from_str_exact("0.322580").unwrap_or(Decimal::ZERO),
            _ => Decimal::from(1),
        };

        let is_buying_base = to_currency == Currency::TND;
        let result = self
            .converter
            .convert(amount, from_currency, to_currency, market_rate, is_buying_base)
            .map_err(|e| AccountServiceError::InvalidInput(e.to_string()))?;

        // In a real system:
        // 1. Withdraw from from_account
        // 2. Deposit to to_account
        // 3. Create conversion tracking record
        // 4. Create accounting entries for fees

        Ok(result)
    }

    /// Get monthly conversion usage for a customer and currency
    pub async fn get_monthly_conversion_usage(
        &self,
        customer_id: Uuid,
        _currency: Currency,
        _month: u32,
    ) -> Result<Decimal, AccountServiceError> {
        // In a real system, this would query the conversion_conversions table
        // For now, return 0
        let cid = CustomerId::from_uuid(customer_id);
        let _accounts = self
            .account_repo
            .find_by_customer_id(&cid)
            .await
            .map_err(AccountServiceError::Internal)?;

        Ok(Decimal::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use banko_domain::account::{Account, AccountId, AccountStatus, AccountType};
    use banko_domain::shared::value_objects::{CustomerId, Money, Rib};

    struct MockAccountRepo {
        accounts: Mutex<Vec<Account>>,
    }

    impl MockAccountRepo {
        fn new() -> Self {
            MockAccountRepo {
                accounts: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAccountRepository for MockAccountRepo {
        async fn save(&self, account: &Account) -> Result<(), String> {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.retain(|a| a.id() != account.id());
            accounts.push(account.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, String> {
            Ok(self
                .accounts
                .lock()
                .unwrap()
                .iter()
                .find(|a| a.id() == id)
                .cloned())
        }

        async fn find_by_customer_id(&self, customer_id: &CustomerId) -> Result<Vec<Account>, String> {
            Ok(self
                .accounts
                .lock()
                .unwrap()
                .iter()
                .filter(|a| a.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn find_by_rib(&self, rib: &Rib) -> Result<Option<Account>, String> {
            Ok(self
                .accounts
                .lock()
                .unwrap()
                .iter()
                .find(|a| a.rib() == rib)
                .cloned())
        }

        async fn save_movement(&self, _movement: &banko_domain::account::Movement) -> Result<(), String> {
            Ok(())
        }

        async fn find_movements_by_account(
            &self,
            _account_id: &AccountId,
            _limit: i64,
        ) -> Result<Vec<banko_domain::account::Movement>, String> {
            Ok(Vec::new())
        }

        async fn find_movements_by_account_and_period(
            &self,
            _account_id: &AccountId,
            _from: Option<std::chrono::DateTime<Utc>>,
            _to: Option<std::chrono::DateTime<Utc>>,
        ) -> Result<Vec<banko_domain::account::Movement>, String> {
            Ok(Vec::new())
        }

        async fn delete(&self, id: &AccountId) -> Result<(), String> {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.retain(|a| a.id() != id);
            Ok(())
        }
    }

    #[test]
    fn test_multi_currency_service_new() {
        let repo = Arc::new(MockAccountRepo::new());
        let service = MultiCurrencyService::new(repo);
        assert_eq!(service.converter.bank_margin_percent, Decimal::from(2));
    }

    #[test]
    fn test_multi_currency_service_with_margin() {
        let repo = Arc::new(MockAccountRepo::new());
        let service = MultiCurrencyService::with_margin(repo, Decimal::from(3));
        assert_eq!(service.converter.bank_margin_percent, Decimal::from(3));
    }

    #[tokio::test]
    async fn test_convert_between_accounts_missing_from_account() {
        let repo = Arc::new(MockAccountRepo::new());
        let service = MultiCurrencyService::new(repo);

        let result = service
            .convert_between_accounts(
                Uuid::new_v4(),
                Uuid::new_v4(),
                Decimal::from(100),
                Uuid::new_v4(),
            )
            .await;

        assert!(result.is_err());
    }
}
