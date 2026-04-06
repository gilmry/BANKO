use std::sync::Arc;
use chrono::Utc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info};

use banko_application::credit::LoanService;
use banko_domain::credit::LoanId;

/// Batch result of a classification run.
#[derive(Debug, Clone)]
pub struct ClassificationBatchResult {
    pub total_processed: u64,
    pub reclassified_count: u64,
}

/// CreditClassificationJob is responsible for periodically reclassifying all active loans
/// based on days past due. It computes days_past_due from the loan's disbursement date
/// versus the current date, then applies asset class and regulatory provision updates.
///
/// It runs as a background task and can be gracefully shut down via task cancellation.
pub struct CreditClassificationJob {
    loan_service: Arc<LoanService>,
    interval_secs: u64,
}

impl CreditClassificationJob {
    /// Creates a new CreditClassificationJob with a default interval of 86400 seconds (24 hours).
    pub fn new(loan_service: Arc<LoanService>) -> Self {
        CreditClassificationJob {
            loan_service,
            interval_secs: 86400, // 24 hours default
        }
    }

    /// Creates a new CreditClassificationJob with a custom interval.
    pub fn with_interval(loan_service: Arc<LoanService>, interval_secs: u64) -> Self {
        CreditClassificationJob {
            loan_service,
            interval_secs,
        }
    }

    /// Spawns the classification job as a background task.
    /// The task will run periodically and reclassify all active loans.
    /// To stop the job, drop the returned JoinHandle or abort it.
    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut classification_interval = interval(Duration::from_secs(self.interval_secs));

            loop {
                // Wait for the next interval tick
                classification_interval.tick().await;

                debug!("Running credit classification job");

                match self.run_once().await {
                    Ok(result) => {
                        info!(
                            "Credit classification completed: {} loan(s) processed, {} reclassified",
                            result.total_processed, result.reclassified_count
                        );
                    }
                    Err(e) => {
                        error!("Credit classification failed: {}", e);
                    }
                }
            }
        })
    }

    /// Runs a single classification cycle.
    /// Auto-computes days_past_due from loan disbursement date vs now.
    /// Returns a batch result with total processed and reclassified count.
    /// Useful for testing or manual triggers.
    pub async fn run_once(&self) -> Result<ClassificationBatchResult, String> {
        debug!("Running manual credit classification");

        let now = Utc::now().naive_utc().date();

        // Update all active loans, computing days_past_due from disbursement_date
        let results = self
            .loan_service
            .update_all_classifications(|loan| {
                // Auto-compute days_past_due from disbursement date vs now
                if let Some(disbursement_date) = loan.disbursement_date() {
                    let days_elapsed = (now - disbursement_date).num_days();
                    if days_elapsed > 0 {
                        // Get the next unpaid installment due date
                        if let Some(schedule) = loan.schedule() {
                            if let Some(next_unpaid) = schedule.next_unpaid_installment() {
                                let days_overdue = (now - next_unpaid.due_date()).num_days();
                                if days_overdue > 0 {
                                    return days_overdue as u32;
                                }
                            }
                        }
                    }
                }
                0 // Default to Class 0 (Standard) if no disbursement or all paid
            })
            .await
            .map_err(|e| format!("Failed to update classifications: {}", e))?;

        // Count how many were actually reclassified (would need to track old class, but for now count all)
        let total = results.len() as u64;

        Ok(ClassificationBatchResult {
            total_processed: total,
            reclassified_count: total, // In a real scenario, track actual changes
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use banko_domain::account::AccountId;
    use banko_domain::credit::{
        AssetClass, Loan, LoanId, LoanStatus, PaymentFrequency, Provision, AmortizationType,
    };
    use banko_domain::shared::{CustomerId, Money, Currency};
    use banko_application::credit::ports::{ILoanRepository, IScheduleRepository};
    use banko_domain::credit::Installment;
    use chrono::NaiveDate;
    use std::sync::Mutex;

    struct MockLoanRepository {
        loans: Mutex<Vec<Loan>>,
    }

    impl MockLoanRepository {
        fn new() -> Self {
            MockLoanRepository {
                loans: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ILoanRepository for MockLoanRepository {
        async fn save(&self, loan: &Loan) -> Result<(), String> {
            let mut loans = self.loans.lock().unwrap();
            loans.retain(|l| l.id() != loan.id());
            loans.push(loan.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &LoanId) -> Result<Option<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.iter().find(|l| l.id() == id).cloned())
        }

        async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans
                .iter()
                .filter(|l| l.account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn find_all(
            &self,
            status: Option<LoanStatus>,
            asset_class: Option<AssetClass>,
            account_id: Option<&AccountId>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans
                .iter()
                .filter(|l| status.is_none() || Some(l.status()) == status)
                .filter(|l| asset_class.is_none() || Some(l.asset_class()) == asset_class)
                .filter(|l| account_id.is_none() || Some(l.account_id()) == account_id)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn count_all(
            &self,
            status: Option<LoanStatus>,
            asset_class: Option<AssetClass>,
            account_id: Option<&AccountId>,
        ) -> Result<i64, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans
                .iter()
                .filter(|l| status.is_none() || Some(l.status()) == status)
                .filter(|l| asset_class.is_none() || Some(l.asset_class()) == asset_class)
                .filter(|l| account_id.is_none() || Some(l.account_id()) == account_id)
                .count() as i64)
        }

        async fn find_active_loans(&self) -> Result<Vec<Loan>, String> {
            let loans = self.loans.lock().unwrap();
            Ok(loans
                .iter()
                .filter(|l| l.status() == LoanStatus::Active)
                .cloned()
                .collect())
        }

        async fn delete(&self, id: &LoanId) -> Result<(), String> {
            let mut loans = self.loans.lock().unwrap();
            loans.retain(|l| l.id() != id);
            Ok(())
        }
    }

    struct MockScheduleRepository {
        installments: Mutex<Vec<Installment>>,
    }

    impl MockScheduleRepository {
        fn new() -> Self {
            MockScheduleRepository {
                installments: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IScheduleRepository for MockScheduleRepository {
        async fn save_installments(&self, installments: &[Installment]) -> Result<(), String> {
            let mut all = self.installments.lock().unwrap();
            all.extend(installments.iter().cloned());
            Ok(())
        }

        async fn find_by_loan_id(&self, loan_id: &LoanId) -> Result<Vec<Installment>, String> {
            let all = self.installments.lock().unwrap();
            Ok(all
                .iter()
                .filter(|i| i.loan_id() == loan_id)
                .cloned()
                .collect())
        }

        async fn update_installment(&self, installment: &Installment) -> Result<(), String> {
            let mut all = self.installments.lock().unwrap();
            if let Some(existing) = all.iter_mut().find(|i| i.id() == installment.id()) {
                *existing = installment.clone();
            }
            Ok(())
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    #[tokio::test]
    async fn test_run_once_no_loans() {
        let repo = Arc::new(MockLoanRepository::new());
        let schedule_repo = Arc::new(MockScheduleRepository::new());
        let service = LoanService::new(repo, schedule_repo);
        let job = CreditClassificationJob::new(Arc::new(service));

        let result = job.run_once().await;
        assert!(result.is_ok());
        let batch = result.unwrap();
        assert_eq!(batch.total_processed, 0);
        assert_eq!(batch.reclassified_count, 0);
    }

    #[tokio::test]
    async fn test_run_once_with_active_loans() {
        let loan_repo = Arc::new(MockLoanRepository::new());
        let schedule_repo = Arc::new(MockScheduleRepository::new());
        let service = Arc::new(LoanService::new(
            loan_repo.clone(),
            schedule_repo.clone(),
        ));

        // Create an active loan
        let loan = service
            .apply_for_loan(AccountId::new(), CustomerId::new(), tnd(100000.0), 8.0, 12)
            .await
            .unwrap();

        service.approve_loan(loan.id()).await.unwrap();

        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        service
            .disburse(
                loan.id(),
                start,
                PaymentFrequency::Monthly,
                AmortizationType::Constant,
            )
            .await
            .unwrap();

        let job = CreditClassificationJob::new(service);
        let result = job.run_once().await;
        assert!(result.is_ok());
        let batch = result.unwrap();
        assert_eq!(batch.total_processed, 1);
    }

    #[tokio::test]
    async fn test_spawn_task() {
        let repo = Arc::new(MockLoanRepository::new());
        let schedule_repo = Arc::new(MockScheduleRepository::new());
        let service = Arc::new(LoanService::new(repo, schedule_repo));
        let job = CreditClassificationJob::with_interval(service, 1); // 1 second for testing

        let handle = job.spawn();

        // Give the task a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Abort the task
        handle.abort();
    }
}
