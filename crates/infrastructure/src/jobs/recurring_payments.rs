use chrono::Utc;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info};

use banko_application::payment::RecurringPaymentService;

/// Batch result of a recurring payment execution.
#[derive(Debug, Clone)]
pub struct RecurringPaymentBatchResult {
    pub standing_orders: RecurringPaymentBatchStats,
    pub direct_debits: RecurringPaymentBatchStats,
}

#[derive(Debug, Clone)]
pub struct RecurringPaymentBatchStats {
    pub total: usize,
    pub executed: usize,
    pub failed: usize,
    pub skipped: usize,
}

/// RecurringPaymentScheduler is responsible for periodically executing standing orders
/// and direct debit mandates that are due. It runs as a background task and processes
/// payments on a configurable interval (default 1 hour).
pub struct RecurringPaymentScheduler {
    recurring_payment_service: Arc<RecurringPaymentService>,
    interval_secs: u64,
}

impl RecurringPaymentScheduler {
    /// Creates a new RecurringPaymentScheduler with a default interval of 3600 seconds (1 hour).
    pub fn new(recurring_payment_service: Arc<RecurringPaymentService>) -> Self {
        RecurringPaymentScheduler {
            recurring_payment_service,
            interval_secs: 3600, // 1 hour default
        }
    }

    /// Creates a new RecurringPaymentScheduler with a custom interval.
    pub fn with_interval(recurring_payment_service: Arc<RecurringPaymentService>, interval_secs: u64) -> Self {
        RecurringPaymentScheduler {
            recurring_payment_service,
            interval_secs,
        }
    }

    /// Spawns the recurring payment scheduler as a background task.
    /// The task will run periodically and execute all standing orders and direct debits due today.
    /// To stop the job, drop the returned JoinHandle or abort it.
    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut scheduler_interval = interval(Duration::from_secs(self.interval_secs));

            loop {
                // Wait for the next interval tick
                scheduler_interval.tick().await;

                debug!("Running recurring payment scheduler");

                let today = Utc::now().naive_utc().date();

                match self.run_once(today).await {
                    Ok(result) => {
                        info!(
                            "Recurring payment execution completed: {} standing orders executed, {} failed, {} skipped; {} direct debits executed, {} failed, {} skipped",
                            result.standing_orders.executed,
                            result.standing_orders.failed,
                            result.standing_orders.skipped,
                            result.direct_debits.executed,
                            result.direct_debits.failed,
                            result.direct_debits.skipped,
                        );
                    }
                    Err(e) => {
                        error!("Recurring payment execution failed: {}", e);
                    }
                }
            }
        })
    }

    /// Runs a single recurring payment execution cycle.
    /// Executes all standing orders and direct debits that are due on the given date.
    /// Returns a batch result with statistics for both standing orders and direct debits.
    /// Useful for testing or manual triggers.
    pub async fn run_once(
        &self,
        today: chrono::NaiveDate,
    ) -> Result<RecurringPaymentBatchResult, String> {
        debug!("Running manual recurring payment execution for date: {}", today);

        // Execute standing orders
        let so_result = self
            .recurring_payment_service
            .execute_due_standing_orders(today)
            .await
            .map_err(|e| format!("Failed to execute standing orders: {}", e))?;

        let standing_orders = RecurringPaymentBatchStats {
            total: so_result.total,
            executed: so_result.executed,
            failed: so_result.failed,
            skipped: so_result.skipped,
        };

        // Execute direct debits
        let dd_result = self
            .recurring_payment_service
            .execute_due_debits(today)
            .await
            .map_err(|e| format!("Failed to execute direct debits: {}", e))?;

        let direct_debits = RecurringPaymentBatchStats {
            total: dd_result.total,
            executed: dd_result.executed,
            failed: dd_result.failed,
            skipped: dd_result.skipped,
        };

        Ok(RecurringPaymentBatchResult {
            standing_orders,
            direct_debits,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use banko_application::payment::{
        IStandingOrderRepository, IMandateRepository, IDebitExecutionRepository,
        RecurringPaymentService, BatchExecutionResult,
    };
    use banko_domain::payment::{StandingOrder, DirectDebitMandate, DebitExecution};
    use chrono::NaiveDate;
    use std::sync::Mutex;
    use uuid::Uuid;

    struct MockStandingOrderRepo {
        orders: Mutex<Vec<StandingOrder>>,
    }

    impl MockStandingOrderRepo {
        fn new() -> Self {
            MockStandingOrderRepo {
                orders: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IStandingOrderRepository for MockStandingOrderRepo {
        async fn save(&self, order: &StandingOrder) -> Result<(), String> {
            let mut orders = self.orders.lock().unwrap();
            orders.retain(|o| o.id() != order.id());
            orders.push(order.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders.iter().find(|o| o.id() == id).cloned())
        }

        async fn find_due_today(&self, today: NaiveDate) -> Result<Vec<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| o.is_due_today(today))
                .cloned()
                .collect())
        }

        async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| o.account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn update(&self, order: &StandingOrder) -> Result<(), String> {
            let mut orders = self.orders.lock().unwrap();
            if let Some(existing) = orders.iter_mut().find(|o| o.id() == order.id()) {
                *existing = order.clone();
            }
            Ok(())
        }

        async fn list_active(&self) -> Result<Vec<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders.iter().cloned().collect())
        }
    }

    struct MockMandateRepo {
        mandates: Mutex<Vec<DirectDebitMandate>>,
    }

    impl MockMandateRepo {
        fn new() -> Self {
            MockMandateRepo {
                mandates: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IMandateRepository for MockMandateRepo {
        async fn save(&self, mandate: &DirectDebitMandate) -> Result<(), String> {
            let mut mandates = self.mandates.lock().unwrap();
            mandates.retain(|m| m.id() != mandate.id());
            mandates.push(mandate.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<DirectDebitMandate>, String> {
            let mandates = self.mandates.lock().unwrap();
            Ok(mandates.iter().find(|m| m.id() == id).cloned())
        }

        async fn find_by_debtor(&self, account_id: Uuid) -> Result<Vec<DirectDebitMandate>, String> {
            let mandates = self.mandates.lock().unwrap();
            Ok(mandates
                .iter()
                .filter(|m| m.debtor_account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn find_active_by_creditor(
            &self,
            creditor_id: &str,
        ) -> Result<Vec<DirectDebitMandate>, String> {
            let mandates = self.mandates.lock().unwrap();
            Ok(mandates
                .iter()
                .filter(|m| m.creditor_id() == creditor_id)
                .cloned()
                .collect())
        }

        async fn update(&self, mandate: &DirectDebitMandate) -> Result<(), String> {
            let mut mandates = self.mandates.lock().unwrap();
            if let Some(existing) = mandates.iter_mut().find(|m| m.id() == mandate.id()) {
                *existing = mandate.clone();
            }
            Ok(())
        }
    }

    struct MockDebitExecutionRepo {
        executions: Mutex<Vec<DebitExecution>>,
    }

    impl MockDebitExecutionRepo {
        fn new() -> Self {
            MockDebitExecutionRepo {
                executions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IDebitExecutionRepository for MockDebitExecutionRepo {
        async fn save(&self, execution: &DebitExecution) -> Result<(), String> {
            let mut execs = self.executions.lock().unwrap();
            execs.push(execution.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<DebitExecution>, String> {
            let execs = self.executions.lock().unwrap();
            Ok(execs.iter().find(|e| e.id() == id).cloned())
        }

        async fn find_by_mandate(&self, mandate_id: Uuid) -> Result<Vec<DebitExecution>, String> {
            let execs = self.executions.lock().unwrap();
            Ok(execs
                .iter()
                .filter(|e| e.mandate_id() == mandate_id)
                .cloned()
                .collect())
        }
    }

    #[tokio::test]
    async fn test_run_once_no_due_orders() {
        let so_repo = Arc::new(MockStandingOrderRepo::new());
        let mandate_repo = Arc::new(MockMandateRepo::new());
        let debit_repo = Arc::new(MockDebitExecutionRepo::new());

        let service = Arc::new(RecurringPaymentService::new(
            so_repo,
            mandate_repo,
            debit_repo,
        ));
        let scheduler = RecurringPaymentScheduler::new(service);

        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();
        let result = scheduler.run_once(today).await;

        assert!(result.is_ok());
        let batch = result.unwrap();
        assert_eq!(batch.standing_orders.total, 0);
        assert_eq!(batch.standing_orders.executed, 0);
        assert_eq!(batch.direct_debits.total, 0);
        assert_eq!(batch.direct_debits.executed, 0);
    }

    #[tokio::test]
    async fn test_spawn_task() {
        let so_repo = Arc::new(MockStandingOrderRepo::new());
        let mandate_repo = Arc::new(MockMandateRepo::new());
        let debit_repo = Arc::new(MockDebitExecutionRepo::new());

        let service = Arc::new(RecurringPaymentService::new(
            so_repo,
            mandate_repo,
            debit_repo,
        ));
        let scheduler = RecurringPaymentScheduler::with_interval(service, 1); // 1 second for testing

        let handle = scheduler.spawn();

        // Give the task a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Abort the task
        handle.abort();
    }
}
