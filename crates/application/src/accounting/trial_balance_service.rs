use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

use banko_domain::accounting::AccountCode;

use super::errors::AccountingServiceError;
use super::ports::{ILedgerRepository, AccountBalanceRow};

// --- Trial Balance Response DTOs ---

#[derive(Debug, Clone)]
pub struct TrialBalanceLineItem {
    pub account_code: String,
    pub account_label: String,
    pub account_type: String,
    pub total_debit: i64,
    pub total_credit: i64,
}

#[derive(Debug, Clone)]
pub struct TrialBalance {
    pub as_of: NaiveDate,
    pub lines: Vec<TrialBalanceLineItem>,
    pub total_debits: i64,
    pub total_credits: i64,
    pub is_balanced: bool,
}

/// Service for computing trial balances (FR-086)
pub struct TrialBalanceService {
    ledger_repo: Arc<dyn ILedgerRepository>,
}

impl TrialBalanceService {
    pub fn new(ledger_repo: Arc<dyn ILedgerRepository>) -> Self {
        TrialBalanceService { ledger_repo }
    }

    /// Compute trial balance as of a given date (FR-086: Balance générale)
    pub async fn compute(
        &self,
        as_of: NaiveDate,
    ) -> Result<TrialBalance, AccountingServiceError> {
        let balances = self
            .ledger_repo
            .get_all_balances(as_of)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let mut lines = Vec::new();
        let mut total_debits: i64 = 0;
        let mut total_credits: i64 = 0;

        for balance in balances {
            lines.push(TrialBalanceLineItem {
                account_code: balance.code,
                account_label: balance.label,
                account_type: balance.account_type,
                total_debit: balance.total_debit,
                total_credit: balance.total_credit,
            });

            total_debits += balance.total_debit;
            total_credits += balance.total_credit;
        }

        let is_balanced = total_debits == total_credits;

        Ok(TrialBalance {
            as_of,
            lines,
            total_debits,
            total_credits,
            is_balanced,
        })
    }

    /// Compute trial balance for a single account
    pub async fn compute_for_account(
        &self,
        account_code: &str,
        as_of: NaiveDate,
    ) -> Result<(i64, i64), AccountingServiceError> {
        let code = AccountCode::new(account_code)
            .map_err(|e| AccountingServiceError::InvalidEntry(e.to_string()))?;

        let (debit, credit) = self
            .ledger_repo
            .get_account_balance(&code, as_of)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok((debit, credit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockLedgerRepository {
        balances: Mutex<Vec<AccountBalanceRow>>,
    }

    impl MockLedgerRepository {
        fn new() -> Self {
            MockLedgerRepository {
                balances: Mutex::new(Vec::new()),
            }
        }

        fn add_balance(&self, code: String, label: String, account_type: String, debit: i64, credit: i64) {
            let mut balances = self.balances.lock().unwrap();
            balances.push(AccountBalanceRow {
                code,
                label,
                account_type,
                total_debit: debit,
                total_credit: credit,
            });
        }
    }

    #[async_trait]
    impl ILedgerRepository for MockLedgerRepository {
        async fn get_account_balance(
            &self,
            code: &AccountCode,
            _as_of: NaiveDate,
        ) -> Result<(i64, i64), String> {
            let balances = self.balances.lock().unwrap();
            let code_str = code.as_str().to_string();
            for balance in balances.iter() {
                if balance.code == code_str {
                    return Ok((balance.total_debit, balance.total_credit));
                }
            }
            Ok((0, 0))
        }

        async fn get_all_balances(&self, _as_of: NaiveDate) -> Result<Vec<AccountBalanceRow>, String> {
            let balances = self.balances.lock().unwrap();
            Ok(balances.iter().cloned().collect())
        }

        async fn save_chart_entry(
            &self,
            _entry: &banko_domain::accounting::LedgerAccount,
        ) -> Result<(), String> {
            Ok(())
        }

        async fn find_chart_entry(
            &self,
            _code: &AccountCode,
        ) -> Result<Option<banko_domain::accounting::LedgerAccount>, String> {
            Ok(None)
        }

        async fn find_all_chart_entries(
            &self,
        ) -> Result<Vec<banko_domain::accounting::LedgerAccount>, String> {
            Ok(Vec::new())
        }
    }

    #[tokio::test]
    async fn test_balanced_trial_balance() {
        let repo = Arc::new(MockLedgerRepository::new());
        repo.add_balance("11".into(), "Capital".into(), "Equity".into(), 100000, 0);
        repo.add_balance("42".into(), "Client deposits".into(), "Liability".into(), 0, 100000);

        let service = TrialBalanceService::new(repo);
        let tb = service
            .compute(NaiveDate::from_ymd_opt(2026, 4, 7).unwrap())
            .await
            .unwrap();

        assert!(tb.is_balanced);
        assert_eq!(tb.total_debits, 100000);
        assert_eq!(tb.total_credits, 100000);
        assert_eq!(tb.lines.len(), 2);
    }

    #[tokio::test]
    async fn test_unbalanced_trial_balance() {
        let repo = Arc::new(MockLedgerRepository::new());
        repo.add_balance("11".into(), "Capital".into(), "Equity".into(), 100000, 0);
        repo.add_balance("42".into(), "Client deposits".into(), "Liability".into(), 0, 50000);

        let service = TrialBalanceService::new(repo);
        let tb = service
            .compute(NaiveDate::from_ymd_opt(2026, 4, 7).unwrap())
            .await
            .unwrap();

        assert!(!tb.is_balanced);
        assert_eq!(tb.total_debits, 100000);
        assert_eq!(tb.total_credits, 50000);
    }

    #[tokio::test]
    async fn test_trial_balance_for_account() {
        let repo = Arc::new(MockLedgerRepository::new());
        repo.add_balance("31".into(), "Loans".into(), "Asset".into(), 500000, 0);

        let service = TrialBalanceService::new(repo);
        let (debit, credit) = service
            .compute_for_account("31", NaiveDate::from_ymd_opt(2026, 4, 7).unwrap())
            .await
            .unwrap();

        assert_eq!(debit, 500000);
        assert_eq!(credit, 0);
    }

    #[tokio::test]
    async fn test_trial_balance_empty_ledger() {
        let repo = Arc::new(MockLedgerRepository::new());
        let service = TrialBalanceService::new(repo);
        let tb = service
            .compute(NaiveDate::from_ymd_opt(2026, 4, 7).unwrap())
            .await
            .unwrap();

        assert!(tb.is_balanced);
        assert_eq!(tb.total_debits, 0);
        assert_eq!(tb.total_credits, 0);
        assert_eq!(tb.lines.len(), 0);
    }
}
