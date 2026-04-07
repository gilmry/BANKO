use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use super::errors::AccountingServiceError;
use super::ports::*;

/// Status of account reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationStatus {
    Balanced,
    Variance,
    AutoResolved,
    ManualReviewRequired,
}

impl std::fmt::Display for ReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReconciliationStatus::Balanced => write!(f, "Balanced"),
            ReconciliationStatus::Variance => write!(f, "Variance"),
            ReconciliationStatus::AutoResolved => write!(f, "AutoResolved"),
            ReconciliationStatus::ManualReviewRequired => write!(f, "ManualReviewRequired"),
        }
    }
}

/// Reconciliation details for a single general ledger account.
#[derive(Debug, Clone)]
pub struct AccountReconciliation {
    pub account_code: String,
    pub account_name: String,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
    pub variance: Decimal,
    pub status: ReconciliationStatus,
}

/// Details of an automatically resolved variance.
#[derive(Debug, Clone)]
pub struct AutoResolution {
    pub account_code: String,
    pub variance: Decimal,
    pub resolution_type: String, // e.g., "Rounding"
    pub entry_created: bool,
}

/// Complete reconciliation report for a specific date.
#[derive(Debug, Clone)]
pub struct ReconciliationReport {
    pub id: Uuid,
    pub reconciliation_date: NaiveDate,
    pub accounts: Vec<AccountReconciliation>,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
    pub total_variance: Decimal,
    pub overall_status: ReconciliationStatus,
    pub auto_resolutions: Vec<AutoResolution>,
    pub created_at: DateTime<Utc>,
}

/// Port for persisting and retrieving reconciliation reports.
#[async_trait::async_trait]
pub trait IReconciliationRepository: Send + Sync {
    async fn save(&self, report: &ReconciliationReport) -> Result<(), String>;
    async fn find_by_date(
        &self,
        date: NaiveDate,
    ) -> Result<Option<ReconciliationReport>, String>;
    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<ReconciliationReport>, String>;
    async fn count_all(&self) -> Result<i64, String>;
}

/// Service responsible for reconciling general ledger accounts.
pub struct ReconciliationService {
    ledger_repo: Arc<dyn ILedgerRepository>,
    reconciliation_repo: Arc<dyn IReconciliationRepository>,
}

impl ReconciliationService {
    /// Creates a new ReconciliationService.
    pub fn new(
        ledger_repo: Arc<dyn ILedgerRepository>,
        reconciliation_repo: Arc<dyn IReconciliationRepository>,
    ) -> Self {
        ReconciliationService {
            ledger_repo,
            reconciliation_repo,
        }
    }

    /// Reconciles all general ledger accounts for the specified date.
    /// Returns a reconciliation report with account details and overall status.
    pub async fn reconcile(&self, date: NaiveDate) -> Result<ReconciliationReport, AccountingServiceError> {
        // Rounding tolerance in TND
        let rounding_tolerance = Decimal::from_str_exact("1.00")
            .unwrap_or(Decimal::new(100, 2));

        // Get all account balances from the ledger
        let account_balances = self
            .ledger_repo
            .get_all_balances(date)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let mut accounts = Vec::new();
        let mut total_debits = Decimal::ZERO;
        let mut total_credits = Decimal::ZERO;
        let mut auto_resolutions = Vec::new();

        for balance in account_balances {
            let debits = Decimal::from(balance.total_debit);
            let credits = Decimal::from(balance.total_credit);
            let variance = (debits - credits).abs();

            total_debits += debits;
            total_credits += credits;

            // Determine status based on variance
            let status = if variance == Decimal::ZERO {
                ReconciliationStatus::Balanced
            } else if variance <= rounding_tolerance {
                // Auto-resolve small rounding differences
                auto_resolutions.push(AutoResolution {
                    account_code: balance.code.clone(),
                    variance,
                    resolution_type: "Rounding".to_string(),
                    entry_created: false, // In a real scenario, would create adjustment entry
                });
                ReconciliationStatus::AutoResolved
            } else {
                ReconciliationStatus::ManualReviewRequired
            };

            accounts.push(AccountReconciliation {
                account_code: balance.code,
                account_name: balance.label,
                total_debits: debits,
                total_credits: credits,
                variance,
                status,
            });
        }

        // Calculate overall status
        let overall_status = if accounts.iter().all(|a| a.status == ReconciliationStatus::Balanced)
        {
            ReconciliationStatus::Balanced
        } else if accounts.iter().any(|a| a.status == ReconciliationStatus::ManualReviewRequired) {
            ReconciliationStatus::ManualReviewRequired
        } else if accounts.iter().any(|a| a.status == ReconciliationStatus::AutoResolved) {
            ReconciliationStatus::AutoResolved
        } else {
            ReconciliationStatus::Balanced
        };

        let total_variance = (total_debits - total_credits).abs();

        let report = ReconciliationReport {
            id: Uuid::new_v4(),
            reconciliation_date: date,
            accounts,
            total_debits,
            total_credits,
            total_variance,
            overall_status,
            auto_resolutions,
            created_at: Utc::now(),
        };

        // Persist the report
        self.reconciliation_repo
            .save(&report)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(report)
    }

    /// Retrieves a previously generated reconciliation report for the given date.
    pub async fn get_report(
        &self,
        date: NaiveDate,
    ) -> Result<Option<ReconciliationReport>, AccountingServiceError> {
        self.reconciliation_repo
            .find_by_date(date)
            .await
            .map_err(AccountingServiceError::Internal)
    }

    /// Retrieves all reconciliation reports within a date range.
    pub async fn list_reports(
        &self,
        from: NaiveDate,
        to: NaiveDate,
        limit: i64,
    ) -> Result<Vec<ReconciliationReport>, AccountingServiceError> {
        let all_reports = self
            .reconciliation_repo
            .find_all(0, 10000)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let filtered: Vec<_> = all_reports
            .into_iter()
            .filter(|r| r.reconciliation_date >= from && r.reconciliation_date <= to)
            .take(limit as usize)
            .collect();

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLedgerRepository {
        balances: std::sync::Mutex<Vec<AccountBalanceRow>>,
    }

    impl MockLedgerRepository {
        fn new() -> Self {
            MockLedgerRepository {
                balances: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn add_account(
            &self,
            code: String,
            label: String,
            account_type: String,
            total_debit: i64,
            total_credit: i64,
        ) {
            let mut balances = self.balances.lock().unwrap();
            balances.push(AccountBalanceRow {
                code,
                label,
                account_type,
                total_debit,
                total_credit,
            });
        }
    }

    #[async_trait::async_trait]
    impl ILedgerRepository for MockLedgerRepository {
        async fn get_account_balance(
            &self,
            code: &banko_domain::accounting::AccountCode,
            _as_of: NaiveDate,
        ) -> Result<(i64, i64), String> {
            let balances = self.balances.lock().unwrap();
            let code_str = code.as_str().to_string();
            if let Some(balance) = balances.iter().find(|b| b.code == code_str) {
                Ok((balance.total_debit, balance.total_credit))
            } else {
                Ok((0, 0))
            }
        }

        async fn get_all_balances(
            &self,
            _as_of: NaiveDate,
        ) -> Result<Vec<AccountBalanceRow>, String> {
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
            _code: &banko_domain::accounting::AccountCode,
        ) -> Result<Option<banko_domain::accounting::LedgerAccount>, String> {
            Ok(None)
        }

        async fn find_all_chart_entries(
            &self,
        ) -> Result<Vec<banko_domain::accounting::LedgerAccount>, String> {
            Ok(Vec::new())
        }
    }

    struct MockReconciliationRepository {
        reports: std::sync::Mutex<Vec<ReconciliationReport>>,
    }

    impl MockReconciliationRepository {
        fn new() -> Self {
            MockReconciliationRepository {
                reports: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl IReconciliationRepository for MockReconciliationRepository {
        async fn save(&self, report: &ReconciliationReport) -> Result<(), String> {
            let mut reports = self.reports.lock().unwrap();
            reports.retain(|r| r.reconciliation_date != report.reconciliation_date);
            reports.push(report.clone());
            Ok(())
        }

        async fn find_by_date(
            &self,
            date: NaiveDate,
        ) -> Result<Option<ReconciliationReport>, String> {
            let reports = self.reports.lock().unwrap();
            Ok(reports
                .iter()
                .find(|r| r.reconciliation_date == date)
                .cloned())
        }

        async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<ReconciliationReport>, String> {
            let reports = self.reports.lock().unwrap();
            Ok(reports
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn count_all(&self) -> Result<i64, String> {
            let reports = self.reports.lock().unwrap();
            Ok(reports.len() as i64)
        }
    }

    #[tokio::test]
    async fn test_balanced_accounts() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        // Each account is individually balanced (debits == credits)
        ledger_repo.add_account(
            "11".to_string(),
            "Capital".to_string(),
            "Equity".to_string(),
            10000,
            10000,
        );
        ledger_repo.add_account(
            "42".to_string(),
            "Client deposits".to_string(),
            "Liability".to_string(),
            5000,
            5000,
        );

        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = service.reconcile(date).await.unwrap();

        assert_eq!(report.total_debits, Decimal::from(15000));
        assert_eq!(report.total_credits, Decimal::from(15000));
        assert_eq!(report.total_variance, Decimal::ZERO);
        assert_eq!(report.overall_status, ReconciliationStatus::Balanced);
    }

    #[tokio::test]
    async fn test_variance_detected() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        ledger_repo.add_account(
            "11".to_string(),
            "Capital".to_string(),
            "Equity".to_string(),
            10000,
            0,
        );
        ledger_repo.add_account(
            "42".to_string(),
            "Client deposits".to_string(),
            "Liability".to_string(),
            0,
            9500,
        );

        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = service.reconcile(date).await.unwrap();

        assert!(report.total_variance > Decimal::ZERO);
        assert_eq!(report.overall_status, ReconciliationStatus::ManualReviewRequired);
    }

    #[tokio::test]
    async fn test_small_variance_auto_resolved() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        // Single account with a small variance (1 TND, under rounding tolerance of 1.00)
        ledger_repo.add_account(
            "42".to_string(),
            "Client deposits".to_string(),
            "Liability".to_string(),
            10000,
            9999, // Variance of 1 TND (under rounding tolerance)
        );

        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = service.reconcile(date).await.unwrap();

        assert!(report.auto_resolutions.len() > 0);
        assert_eq!(report.overall_status, ReconciliationStatus::AutoResolved);
    }

    #[tokio::test]
    async fn test_multiple_accounts_mixed_status() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        ledger_repo.add_account(
            "11".to_string(),
            "Capital".to_string(),
            "Equity".to_string(),
            10000,
            10000, // Balanced (variance = 0)
        );
        ledger_repo.add_account(
            "42".to_string(),
            "Client deposits".to_string(),
            "Liability".to_string(),
            5000,
            5000, // Balanced (variance = 0)
        );
        ledger_repo.add_account(
            "43".to_string(),
            "Bank deposits".to_string(),
            "Liability".to_string(),
            5000,
            4999, // Small variance of 1 (auto-resolved, under rounding tolerance)
        );

        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = service.reconcile(date).await.unwrap();

        assert_eq!(report.accounts.len(), 3);
        assert!(report.accounts.iter().any(|a| a.status == ReconciliationStatus::Balanced));
        assert!(report.accounts.iter().any(|a| a.status == ReconciliationStatus::AutoResolved));
    }

    #[tokio::test]
    async fn test_empty_ledger() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = service.reconcile(date).await.unwrap();

        assert_eq!(report.accounts.len(), 0);
        assert_eq!(report.total_debits, Decimal::ZERO);
        assert_eq!(report.total_credits, Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_get_report() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        ledger_repo.add_account(
            "11".to_string(),
            "Capital".to_string(),
            "Equity".to_string(),
            10000,
            10000,
        );

        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let _ = service.reconcile(date).await.unwrap();
        let retrieved = service.get_report(date).await.unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().reconciliation_date, date);
    }

    #[tokio::test]
    async fn test_list_reports() {
        let ledger_repo = Arc::new(MockLedgerRepository::new());
        ledger_repo.add_account(
            "11".to_string(),
            "Capital".to_string(),
            "Equity".to_string(),
            10000,
            10000,
        );

        let reconciliation_repo = Arc::new(MockReconciliationRepository::new());
        let service = ReconciliationService::new(ledger_repo, reconciliation_repo);

        let date1 = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2026, 4, 2).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2026, 4, 3).unwrap();

        let _ = service.reconcile(date1).await.unwrap();
        let _ = service.reconcile(date2).await.unwrap();
        let _ = service.reconcile(date3).await.unwrap();

        let reports = service
            .list_reports(date1, date2, 100)
            .await
            .unwrap();

        assert!(reports.len() >= 2);
    }

    #[tokio::test]
    async fn test_reconciliation_status_display() {
        assert_eq!(ReconciliationStatus::Balanced.to_string(), "Balanced");
        assert_eq!(ReconciliationStatus::Variance.to_string(), "Variance");
        assert_eq!(ReconciliationStatus::AutoResolved.to_string(), "AutoResolved");
        assert_eq!(
            ReconciliationStatus::ManualReviewRequired.to_string(),
            "ManualReviewRequired"
        );
    }
}
