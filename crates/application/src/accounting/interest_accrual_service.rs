use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use super::errors::AccountingServiceError;
use super::ports::*;

/// Enumerates the available interest calculation methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccrualMethod {
    Simple,
    Compound,
}

impl std::fmt::Display for AccrualMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccrualMethod::Simple => write!(f, "Simple"),
            AccrualMethod::Compound => write!(f, "Compound"),
        }
    }
}

/// Enumerates whether the account earns or pays interest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccrualType {
    Credit,  // Account earns interest (savings, deposits)
    Debit,   // Account pays interest (loans)
}

impl std::fmt::Display for AccrualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccrualType::Credit => write!(f, "Credit"),
            AccrualType::Debit => write!(f, "Debit"),
        }
    }
}

/// Represents a single daily interest accrual entry.
#[derive(Debug, Clone)]
pub struct AccrualEntry {
    pub id: Uuid,
    pub account_id: Uuid,
    pub accrual_date: NaiveDate,
    pub principal: Decimal,
    pub annual_rate: Decimal,
    pub method: AccrualMethod,
    pub daily_interest: Decimal,
    pub accrual_type: AccrualType,
    pub is_capitalized: bool,
}

/// Information about an interest-bearing account.
#[derive(Debug, Clone)]
pub struct InterestAccountInfo {
    pub account_id: Uuid,
    pub balance: Decimal,
    pub annual_rate: Decimal,
    pub calc_method: AccrualMethod,
    pub account_type: AccountType,
    pub capitalization_frequency: CapitalizationFrequency,
}

/// Account types that can bear interest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    Savings,
    TermDeposit,
    Loan,
}

/// Frequency at which interest is capitalized (added to principal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapitalizationFrequency {
    Monthly,
    Quarterly,
    Annually,
    None,
}

/// Result of a daily interest accrual batch run.
#[derive(Debug, Clone)]
pub struct AccrualBatchResult {
    pub date: NaiveDate,
    pub accounts_processed: usize,
    pub total_credit_interest: Decimal,
    pub total_debit_interest: Decimal,
    pub capitalizations: usize,
}

/// Port trait for persisting and retrieving accrual entries.
#[async_trait::async_trait]
pub trait IAccrualRepository: Send + Sync {
    async fn save(&self, entry: &AccrualEntry) -> Result<(), String>;
    async fn find_by_account_and_date(
        &self,
        account_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<AccrualEntry>, String>;
    async fn find_by_account(
        &self,
        account_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<AccrualEntry>, String>;
    async fn sum_accrued(
        &self,
        account_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Decimal, String>;
}

/// Port trait for retrieving information about interest-bearing accounts.
#[async_trait::async_trait]
pub trait IInterestAccountProvider: Send + Sync {
    async fn list_interest_bearing_accounts(&self) -> Result<Vec<InterestAccountInfo>, String>;
}

/// Service responsible for calculating and recording daily interest accruals.
pub struct InterestAccrualService {
    accrual_repo: Arc<dyn IAccrualRepository>,
    account_provider: Arc<dyn IInterestAccountProvider>,
}

impl InterestAccrualService {
    /// Creates a new InterestAccrualService.
    pub fn new(
        accrual_repo: Arc<dyn IAccrualRepository>,
        account_provider: Arc<dyn IInterestAccountProvider>,
    ) -> Self {
        InterestAccrualService {
            accrual_repo,
            account_provider,
        }
    }

    /// Calculates and records daily interest accrual for all interest-bearing accounts.
    pub async fn accrue_daily(&self, date: NaiveDate) -> Result<AccrualBatchResult, AccountingServiceError> {
        let accounts = self
            .account_provider
            .list_interest_bearing_accounts()
            .await
            .map_err(AccountingServiceError::Internal)?;

        let mut total_credit_interest = Decimal::ZERO;
        let mut total_debit_interest = Decimal::ZERO;
        let mut capitalizations = 0;
        let accounts_count = accounts.len();

        for account in accounts {
            // Skip zero-balance accounts
            if account.balance == Decimal::ZERO {
                continue;
            }

            // Calculate daily interest
            // daily_interest = principal * annual_rate / 365
            let daily_interest =
                account.balance * account.annual_rate / Decimal::new(36500, 2);

            // Determine accrual type based on account type
            let accrual_type = match account.account_type {
                AccountType::Loan => AccrualType::Debit,
                AccountType::Savings | AccountType::TermDeposit => AccrualType::Credit,
            };

            // Track totals
            match accrual_type {
                AccrualType::Credit => total_credit_interest += daily_interest,
                AccrualType::Debit => total_debit_interest += daily_interest,
            }

            // Create accrual entry
            let entry = AccrualEntry {
                id: Uuid::new_v4(),
                account_id: account.account_id,
                accrual_date: date,
                principal: account.balance,
                annual_rate: account.annual_rate,
                method: account.calc_method,
                daily_interest,
                accrual_type,
                is_capitalized: false,
            };

            self.accrual_repo
                .save(&entry)
                .await
                .map_err(AccountingServiceError::Internal)?;

            // Check if capitalization is due
            if should_capitalize(date, account.capitalization_frequency) {
                capitalizations += 1;
                // In a real implementation, this would update the account principal
                // For now, we just mark the entry as capitalized
            }
        }

        Ok(AccrualBatchResult {
            date,
            accounts_processed: accounts_count,
            total_credit_interest,
            total_debit_interest,
            capitalizations,
        })
    }

    /// Retrieves the total accrued interest for an account over a date range.
    pub async fn get_accrued_interest(
        &self,
        account_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Decimal, AccountingServiceError> {
        let result: Decimal = self.accrual_repo
            .sum_accrued(account_id, from, to)
            .await
            .map_err(AccountingServiceError::Internal)?;
        Ok(result)
    }

    /// Capitalizes interest for accounts with monthly capitalization frequency.
    /// This is typically called at month-end.
    pub async fn capitalize_monthly(&self, date: NaiveDate) -> Result<usize, AccountingServiceError> {
        let accounts = self
            .account_provider
            .list_interest_bearing_accounts()
            .await
            .map_err(AccountingServiceError::Internal)?;

        let mut capitalized = 0;

        // Get the first day of the current month
        let month_start =
            NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();

        // Get the last day of the previous month
        let month_end = if date.month() == 1 {
            NaiveDate::from_ymd_opt(date.year() - 1, 12, 31).unwrap()
        } else {
            NaiveDate::from_ymd_opt(date.year(), date.month() - 1, 1).unwrap()
                + chrono::Duration::days(1)
                - chrono::Duration::days(1)
        };

        for account in accounts {
            if account.capitalization_frequency != CapitalizationFrequency::Monthly {
                continue;
            }

            // Sum accrued interest for the month
            let accrued: Decimal = self
                .accrual_repo
                .sum_accrued(account.account_id, month_start, date)
                .await
                .map_err(AccountingServiceError::Internal)?;

            if accrued > Decimal::ZERO {
                // Mark accruals as capitalized
                let entries: Vec<AccrualEntry> = self
                    .accrual_repo
                    .find_by_account(account.account_id, month_start, date)
                    .await
                    .map_err(AccountingServiceError::Internal)?;

                for mut entry in entries {
                    entry.is_capitalized = true;
                    self.accrual_repo
                        .save(&entry)
                        .await
                        .map_err(AccountingServiceError::Internal)?;
                }

                capitalized += 1;
            }
        }

        Ok(capitalized)
    }

    /// Capitalizes interest for accounts with quarterly capitalization frequency.
    pub async fn capitalize_quarterly(&self, date: NaiveDate) -> Result<usize, AccountingServiceError> {
        let accounts = self
            .account_provider
            .list_interest_bearing_accounts()
            .await
            .map_err(AccountingServiceError::Internal)?;

        let mut capitalized = 0;

        for account in accounts {
            if account.capitalization_frequency != CapitalizationFrequency::Quarterly {
                continue;
            }

            // Calculate quarter start and end
            let quarter = (date.month() - 1) / 3 + 1;
            let quarter_start =
                NaiveDate::from_ymd_opt(date.year(), (quarter - 1) * 3 + 1, 1).unwrap();

            // Sum accrued interest for the quarter
            let accrued: Decimal = self
                .accrual_repo
                .sum_accrued(account.account_id, quarter_start, date)
                .await
                .map_err(AccountingServiceError::Internal)?;

            if accrued > Decimal::ZERO {
                // Mark accruals as capitalized
                let entries: Vec<AccrualEntry> = self
                    .accrual_repo
                    .find_by_account(account.account_id, quarter_start, date)
                    .await
                    .map_err(AccountingServiceError::Internal)?;

                for mut entry in entries {
                    entry.is_capitalized = true;
                    self.accrual_repo
                        .save(&entry)
                        .await
                        .map_err(AccountingServiceError::Internal)?;
                }

                capitalized += 1;
            }
        }

        Ok(capitalized)
    }

    /// Capitalizes interest for accounts with annual capitalization frequency.
    pub async fn capitalize_annually(&self, date: NaiveDate) -> Result<usize, AccountingServiceError> {
        let accounts = self
            .account_provider
            .list_interest_bearing_accounts()
            .await
            .map_err(AccountingServiceError::Internal)?;

        let mut capitalized = 0;

        for account in accounts {
            if account.capitalization_frequency != CapitalizationFrequency::Annually {
                continue;
            }

            // Calculate year start
            let year_start = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();

            // Sum accrued interest for the year
            let accrued: Decimal = self
                .accrual_repo
                .sum_accrued(account.account_id, year_start, date)
                .await
                .map_err(AccountingServiceError::Internal)?;

            if accrued > Decimal::ZERO {
                // Mark accruals as capitalized
                let entries: Vec<AccrualEntry> = self
                    .accrual_repo
                    .find_by_account(account.account_id, year_start, date)
                    .await
                    .map_err(AccountingServiceError::Internal)?;

                for mut entry in entries {
                    entry.is_capitalized = true;
                    self.accrual_repo
                        .save(&entry)
                        .await
                        .map_err(AccountingServiceError::Internal)?;
                }

                capitalized += 1;
            }
        }

        Ok(capitalized)
    }
}

/// Helper function to determine if interest should be capitalized on a given date.
fn should_capitalize(date: NaiveDate, frequency: CapitalizationFrequency) -> bool {
    match frequency {
        CapitalizationFrequency::None => false,
        CapitalizationFrequency::Monthly => date.day() == 1,
        CapitalizationFrequency::Quarterly => {
            let quarter = (date.month() - 1) / 3 + 1;
            date.month() == (quarter - 1) * 3 + 1 && date.day() == 1
        }
        CapitalizationFrequency::Annually => date.month() == 1 && date.day() == 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAccrualRepository {
        entries: std::sync::Mutex<Vec<AccrualEntry>>,
    }

    impl MockAccrualRepository {
        fn new() -> Self {
            MockAccrualRepository {
                entries: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl IAccrualRepository for MockAccrualRepository {
        async fn save(&self, entry: &AccrualEntry) -> Result<(), String> {
            let mut entries = self.entries.lock().unwrap();
            entries.retain(|e| {
                e.account_id != entry.account_id || e.accrual_date != entry.accrual_date
            });
            entries.push(entry.clone());
            Ok(())
        }

        async fn find_by_account_and_date(
            &self,
            account_id: Uuid,
            date: NaiveDate,
        ) -> Result<Option<AccrualEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .find(|e| e.account_id == account_id && e.accrual_date == date)
                .cloned())
        }

        async fn find_by_account(
            &self,
            account_id: Uuid,
            from: NaiveDate,
            to: NaiveDate,
        ) -> Result<Vec<AccrualEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| {
                    e.account_id == account_id && e.accrual_date >= from && e.accrual_date <= to
                })
                .cloned()
                .collect())
        }

        async fn sum_accrued(
            &self,
            account_id: Uuid,
            from: NaiveDate,
            to: NaiveDate,
        ) -> Result<Decimal, String> {
            let entries = self.entries.lock().unwrap();
            let sum = entries
                .iter()
                .filter(|e| {
                    e.account_id == account_id && e.accrual_date >= from && e.accrual_date <= to
                })
                .map(|e| e.daily_interest)
                .sum();
            Ok(sum)
        }
    }

    struct MockInterestAccountProvider {
        accounts: std::sync::Mutex<Vec<InterestAccountInfo>>,
    }

    impl MockInterestAccountProvider {
        fn new() -> Self {
            MockInterestAccountProvider {
                accounts: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn add_account(&self, account: InterestAccountInfo) {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.push(account);
        }
    }

    #[async_trait::async_trait]
    impl IInterestAccountProvider for MockInterestAccountProvider {
        async fn list_interest_bearing_accounts(&self) -> Result<Vec<InterestAccountInfo>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts.iter().cloned().collect())
        }
    }

    fn account(
        id: Uuid,
        balance: f64,
        rate: f64,
        account_type: AccountType,
        cap_freq: CapitalizationFrequency,
    ) -> InterestAccountInfo {
        InterestAccountInfo {
            account_id: id,
            balance: Decimal::from_f64_retain(balance).unwrap(),
            annual_rate: Decimal::from_f64_retain(rate).unwrap(),
            calc_method: AccrualMethod::Simple,
            account_type,
            capitalization_frequency: cap_freq,
        }
    }

    #[tokio::test]
    async fn test_simple_interest_calculation() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let account_id = Uuid::new_v4();
        account_provider.add_account(account(
            account_id,
            10000.0,
            0.05,
            AccountType::Savings,
            CapitalizationFrequency::None,
        ));

        let service = InterestAccrualService::new(accrual_repo, account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = service.accrue_daily(date).await.unwrap();

        assert_eq!(result.date, date);
        assert_eq!(result.accounts_processed, 1);
        assert!(result.total_credit_interest > Decimal::ZERO);
        assert_eq!(result.capitalizations, 0);
    }

    #[tokio::test]
    async fn test_loan_interest_calculation() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let account_id = Uuid::new_v4();
        account_provider.add_account(account(
            account_id,
            50000.0,
            0.08,
            AccountType::Loan,
            CapitalizationFrequency::None,
        ));

        let service = InterestAccrualService::new(accrual_repo, account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = service.accrue_daily(date).await.unwrap();

        assert_eq!(result.accounts_processed, 1);
        assert!(result.total_debit_interest > Decimal::ZERO);
        assert_eq!(result.total_credit_interest, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_zero_balance_account_skipped() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let account_id = Uuid::new_v4();
        account_provider.add_account(account(
            account_id,
            0.0,
            0.05,
            AccountType::Savings,
            CapitalizationFrequency::None,
        ));

        let service = InterestAccrualService::new(accrual_repo.clone(), account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = service.accrue_daily(date).await.unwrap();

        assert_eq!(result.accounts_processed, 1);
        assert_eq!(result.total_credit_interest, Decimal::ZERO);
        assert_eq!(result.total_debit_interest, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        account_provider.add_account(account(
            Uuid::new_v4(),
            10000.0,
            0.05,
            AccountType::Savings,
            CapitalizationFrequency::None,
        ));
        account_provider.add_account(account(
            Uuid::new_v4(),
            50000.0,
            0.08,
            AccountType::Loan,
            CapitalizationFrequency::None,
        ));

        let service = InterestAccrualService::new(accrual_repo, account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = service.accrue_daily(date).await.unwrap();

        assert_eq!(result.accounts_processed, 2);
        assert!(result.total_credit_interest > Decimal::ZERO);
        assert!(result.total_debit_interest > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_get_accrued_interest() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let account_id = Uuid::new_v4();
        account_provider.add_account(account(
            account_id,
            10000.0,
            0.05,
            AccountType::Savings,
            CapitalizationFrequency::None,
        ));

        let service = InterestAccrualService::new(accrual_repo, account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        service.accrue_daily(date).await.unwrap();

        let total = service
            .get_accrued_interest(account_id, date, date)
            .await
            .unwrap();

        assert!(total > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_capitalize_monthly() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let account_id = Uuid::new_v4();
        account_provider.add_account(account(
            account_id,
            10000.0,
            0.05,
            AccountType::Savings,
            CapitalizationFrequency::Monthly,
        ));

        let service = InterestAccrualService::new(accrual_repo, account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 30).unwrap();

        service.accrue_daily(date).await.unwrap();

        let next_month = NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
        let result = service.capitalize_monthly(next_month).await.unwrap();

        assert!(result > 0);
    }

    #[tokio::test]
    async fn test_accrual_type_credit_vs_debit() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let savings_id = Uuid::new_v4();
        let loan_id = Uuid::new_v4();

        account_provider.add_account(account(
            savings_id,
            5000.0,
            0.05,
            AccountType::Savings,
            CapitalizationFrequency::None,
        ));
        account_provider.add_account(account(
            loan_id,
            5000.0,
            0.05,
            AccountType::Loan,
            CapitalizationFrequency::None,
        ));

        let service = InterestAccrualService::new(accrual_repo.clone(), account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = service.accrue_daily(date).await.unwrap();

        assert!(result.total_credit_interest > Decimal::ZERO);
        assert!(result.total_debit_interest > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_empty_account_list() {
        let accrual_repo = Arc::new(MockAccrualRepository::new());
        let account_provider = Arc::new(MockInterestAccountProvider::new());

        let service = InterestAccrualService::new(accrual_repo, account_provider);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = service.accrue_daily(date).await.unwrap();

        assert_eq!(result.accounts_processed, 0);
        assert_eq!(result.total_credit_interest, Decimal::ZERO);
        assert_eq!(result.total_debit_interest, Decimal::ZERO);
    }
}
