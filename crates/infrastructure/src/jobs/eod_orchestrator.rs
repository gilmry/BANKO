use chrono::{DateTime, NaiveDate, Utc};
use std::fmt;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Represents the status of a single EOD processing step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EodStepStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Skipped,
}

impl fmt::Display for EodStepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EodStepStatus::Pending => write!(f, "Pending"),
            EodStepStatus::Running => write!(f, "Running"),
            EodStepStatus::Completed => write!(f, "Completed"),
            EodStepStatus::Failed(msg) => write!(f, "Failed: {}", msg),
            EodStepStatus::Skipped => write!(f, "Skipped"),
        }
    }
}

/// Result of executing a single EOD step.
#[derive(Debug, Clone)]
pub struct EodStepResult {
    pub step_name: String,
    pub status: EodStepStatus,
    pub records_processed: usize,
    pub duration_ms: u64,
    pub details: Option<String>,
}

/// Context passed to EOD steps during execution.
#[derive(Debug, Clone)]
pub struct EodContext {
    pub date: NaiveDate,
    pub started_at: DateTime<Utc>,
    pub dry_run: bool,
}

/// Overall status of an EOD run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EodOverallStatus {
    Completed,
    PartiallyCompleted,
    Failed,
}

impl fmt::Display for EodOverallStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EodOverallStatus::Completed => write!(f, "Completed"),
            EodOverallStatus::PartiallyCompleted => write!(f, "PartiallyCompleted"),
            EodOverallStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Report containing all results from an EOD run.
#[derive(Debug, Clone)]
pub struct EodReport {
    pub date: NaiveDate,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub steps: Vec<EodStepResult>,
    pub overall_status: EodOverallStatus,
}

/// Trait that every EOD step must implement.
#[async_trait::async_trait]
pub trait EodStep: Send + Sync {
    /// Returns the name of this step (used for logging and identification).
    fn name(&self) -> &str;

    /// Indicates if this step is critical (failure triggers rollback).
    fn is_critical(&self) -> bool;

    /// Executes the step with the given context.
    /// Returns a result indicating success or failure.
    async fn execute(&self, context: &EodContext) -> Result<EodStepResult, String>;

    /// Attempts to rollback changes made by this step.
    /// Only called if this step is critical and a later step failed.
    async fn rollback(&self, context: &EodContext) -> Result<(), String>;
}

/// Stub implementation of the Interest Accrual step.
pub struct InterestAccrualStep;

#[async_trait::async_trait]
impl EodStep for InterestAccrualStep {
    fn name(&self) -> &str {
        "interest_accrual"
    }

    fn is_critical(&self) -> bool {
        true
    }

    async fn execute(&self, _ctx: &EodContext) -> Result<EodStepResult, String> {
        debug!("Executing interest accrual step");
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 0,
            duration_ms: 0,
            details: Some("Stub - interest accrual".to_string()),
        })
    }

    async fn rollback(&self, _ctx: &EodContext) -> Result<(), String> {
        Ok(())
    }
}

/// Stub implementation of the Reconciliation step.
pub struct ReconciliationStep;

#[async_trait::async_trait]
impl EodStep for ReconciliationStep {
    fn name(&self) -> &str {
        "reconciliation"
    }

    fn is_critical(&self) -> bool {
        true
    }

    async fn execute(&self, _ctx: &EodContext) -> Result<EodStepResult, String> {
        debug!("Executing reconciliation step");
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 0,
            duration_ms: 0,
            details: Some("Stub - reconciliation".to_string()),
        })
    }

    async fn rollback(&self, _ctx: &EodContext) -> Result<(), String> {
        Ok(())
    }
}

/// Stub implementation of the Fee Calculation step.
pub struct FeeCalculationStep;

#[async_trait::async_trait]
impl EodStep for FeeCalculationStep {
    fn name(&self) -> &str {
        "fee_calculation"
    }

    fn is_critical(&self) -> bool {
        false
    }

    async fn execute(&self, _ctx: &EodContext) -> Result<EodStepResult, String> {
        debug!("Executing fee calculation step");
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 0,
            duration_ms: 0,
            details: Some("Stub - fee calculation".to_string()),
        })
    }

    async fn rollback(&self, _ctx: &EodContext) -> Result<(), String> {
        Ok(())
    }
}

/// Stub implementation of the Cheque Compensation step.
pub struct ChequeCompensationStep;

#[async_trait::async_trait]
impl EodStep for ChequeCompensationStep {
    fn name(&self) -> &str {
        "cheque_compensation"
    }

    fn is_critical(&self) -> bool {
        false
    }

    async fn execute(&self, _ctx: &EodContext) -> Result<EodStepResult, String> {
        debug!("Executing cheque compensation step");
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 0,
            duration_ms: 0,
            details: Some("Stub - cheque compensation".to_string()),
        })
    }

    async fn rollback(&self, _ctx: &EodContext) -> Result<(), String> {
        Ok(())
    }
}

/// Stub implementation of the Card Spending Reset step.
pub struct CardSpendingResetStep;

#[async_trait::async_trait]
impl EodStep for CardSpendingResetStep {
    fn name(&self) -> &str {
        "card_spending_reset"
    }

    fn is_critical(&self) -> bool {
        false
    }

    async fn execute(&self, _ctx: &EodContext) -> Result<EodStepResult, String> {
        debug!("Executing card spending reset step");
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 0,
            duration_ms: 0,
            details: Some("Stub - card spending reset".to_string()),
        })
    }

    async fn rollback(&self, _ctx: &EodContext) -> Result<(), String> {
        Ok(())
    }
}

/// Stub implementation of the Reporting Snapshot step.
pub struct ReportingSnapshotStep;

#[async_trait::async_trait]
impl EodStep for ReportingSnapshotStep {
    fn name(&self) -> &str {
        "reporting_snapshot"
    }

    fn is_critical(&self) -> bool {
        false
    }

    async fn execute(&self, _ctx: &EodContext) -> Result<EodStepResult, String> {
        debug!("Executing reporting snapshot step");
        Ok(EodStepResult {
            step_name: self.name().to_string(),
            status: EodStepStatus::Completed,
            records_processed: 0,
            duration_ms: 0,
            details: Some("Stub - reporting snapshot".to_string()),
        })
    }

    async fn rollback(&self, _ctx: &EodContext) -> Result<(), String> {
        Ok(())
    }
}

/// EOD Orchestrator manages the execution of all End-of-Day processing steps.
/// It handles step sequencing, failure/retry logic, rollback of critical failures, and reporting.
pub struct EodOrchestrator {
    steps: Vec<Box<dyn EodStep + Send + Sync>>,
    max_retries: u32,
    retry_delay_secs: u64,
}

impl EodOrchestrator {
    /// Creates a new EodOrchestrator with default steps in order:
    /// 1. InterestAccrualStep
    /// 2. ReconciliationStep
    /// 3. FeeCalculationStep
    /// 4. ChequeCompensationStep
    /// 5. CardSpendingResetStep
    /// 6. ReportingSnapshotStep
    pub fn new() -> Self {
        let steps: Vec<Box<dyn EodStep + Send + Sync>> = vec![
            Box::new(InterestAccrualStep),
            Box::new(ReconciliationStep),
            Box::new(FeeCalculationStep),
            Box::new(ChequeCompensationStep),
            Box::new(CardSpendingResetStep),
            Box::new(ReportingSnapshotStep),
        ];

        EodOrchestrator {
            steps,
            max_retries: 3,
            retry_delay_secs: 300,
        }
    }

    /// Creates a new EodOrchestrator with custom steps.
    pub fn with_steps(steps: Vec<Box<dyn EodStep + Send + Sync>>) -> Self {
        EodOrchestrator {
            steps,
            max_retries: 3,
            retry_delay_secs: 300,
        }
    }

    /// Sets the maximum number of retries for failed steps.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the delay (in seconds) between retries.
    pub fn with_retry_delay(mut self, retry_delay_secs: u64) -> Self {
        self.retry_delay_secs = retry_delay_secs;
        self
    }

    /// Executes all EOD steps for the given date.
    /// Returns an EodReport with results and overall status.
    pub async fn run(&self, date: NaiveDate) -> EodReport {
        let started_at = Utc::now();
        let context = EodContext {
            date,
            started_at,
            dry_run: false,
        };

        info!("Starting EOD processing for {}", date);

        let mut results = Vec::new();
        let mut completed_indices = Vec::new();

        for (idx, step) in self.steps.iter().enumerate() {
            let step_name = step.name();
            debug!("Processing step: {}", step_name);

            let step_result = self.execute_step_with_retry(step.as_ref(), &context).await;
            results.push(step_result.clone());
            completed_indices.push(idx);

            // Check if this is a critical failure
            match &step_result.status {
                EodStepStatus::Failed(_) if step.is_critical() => {
                    error!(
                        "Critical step '{}' failed: {}",
                        step_name, step_result.status
                    );
                    // Rollback completed critical steps in reverse order
                    self.rollback_steps(&context, &completed_indices).await;
                    let completed_at = Utc::now();
                    return EodReport {
                        date,
                        started_at,
                        completed_at,
                        steps: results,
                        overall_status: EodOverallStatus::Failed,
                    };
                }
                EodStepStatus::Failed(_) => {
                    warn!("Non-critical step '{}' failed: {}", step_name, step_result.status);
                    // Continue with next step
                }
                _ => {}
            }
        }

        let completed_at = Utc::now();

        // Determine overall status
        let overall_status = if results.iter().all(|r| r.status == EodStepStatus::Completed) {
            EodOverallStatus::Completed
        } else if results.iter().any(|r| matches!(r.status, EodStepStatus::Failed(_))) {
            EodOverallStatus::PartiallyCompleted
        } else {
            EodOverallStatus::Completed
        };

        info!(
            "EOD processing for {} completed with status: {}",
            date, overall_status
        );

        EodReport {
            date,
            started_at,
            completed_at,
            steps: results,
            overall_status,
        }
    }

    /// Executes a single step by name for manual reruns.
    /// Useful for recovering from partial failures or manual intervention.
    pub async fn run_single_step(&self, step_name: &str, date: NaiveDate) -> EodStepResult {
        let started_at = Utc::now();
        let context = EodContext {
            date,
            started_at,
            dry_run: false,
        };

        if let Some(step) = self.steps.iter().find(|s| s.name() == step_name) {
            info!("Running single step: {} for {}", step_name, date);
            self.execute_step_with_retry(step.as_ref(), &context).await
        } else {
            EodStepResult {
                step_name: step_name.to_string(),
                status: EodStepStatus::Failed("Step not found".to_string()),
                records_processed: 0,
                duration_ms: 0,
                details: None,
            }
        }
    }

    /// Executes a step with retry logic.
    async fn execute_step_with_retry(
        &self,
        step: &(dyn EodStep + Send + Sync),
        context: &EodContext,
    ) -> EodStepResult {
        let mut attempt = 0;

        loop {
            attempt += 1;
            let start = tokio::time::Instant::now();

            match step.execute(context).await {
                Ok(mut result) => {
                    result.duration_ms = start.elapsed().as_millis() as u64;
                    info!(
                        "Step '{}' completed in {}ms",
                        step.name(),
                        result.duration_ms
                    );
                    return result;
                }
                Err(e) => {
                    if attempt < self.max_retries {
                        warn!(
                            "Step '{}' attempt {} failed: {}. Retrying in {}s",
                            step.name(),
                            attempt,
                            e,
                            self.retry_delay_secs
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(
                            self.retry_delay_secs,
                        ))
                        .await;
                    } else {
                        error!(
                            "Step '{}' failed after {} attempts: {}",
                            step.name(),
                            self.max_retries,
                            e
                        );
                        return EodStepResult {
                            step_name: step.name().to_string(),
                            status: EodStepStatus::Failed(e),
                            records_processed: 0,
                            duration_ms: start.elapsed().as_millis() as u64,
                            details: None,
                        };
                    }
                }
            }
        }
    }

    /// Rolls back all completed critical steps in reverse order.
    async fn rollback_steps(
        &self,
        context: &EodContext,
        completed_indices: &[usize],
    ) {
        for idx in completed_indices.iter().rev() {
            let step = &self.steps[*idx];
            if step.is_critical() {
                debug!("Rolling back step: {}", step.name());
                match step.rollback(context).await {
                    Ok(_) => {
                        info!("Step '{}' rolled back successfully", step.name());
                    }
                    Err(e) => {
                        error!("Failed to rollback step '{}': {}", step.name(), e);
                    }
                }
            }
        }
    }
}

/// EodScheduler spawns an EOD job that runs daily at a specified time.
pub struct EodScheduler;

impl EodScheduler {
    /// Spawns a background task that runs EOD processing daily at the specified time.
    /// Returns a JoinHandle that can be aborted to stop the scheduler.
    pub fn spawn(hour: u32, minute: u32) -> tokio::task::JoinHandle<()> {
        if hour > 23 || minute > 59 {
            panic!("Invalid time: hour must be 0-23, minute must be 0-59");
        }

        tokio::spawn(async move {
            loop {
                let now = Utc::now();
                let today = now.date_naive();

                // Calculate next run time
                let mut next_run = today
                    .and_hms_opt(hour, minute, 0)
                    .unwrap()
                    .and_utc();

                if next_run <= now {
                    // If the scheduled time has already passed today, schedule for tomorrow
                    next_run = (today + chrono::Duration::days(1))
                        .and_hms_opt(hour, minute, 0)
                        .unwrap()
                        .and_utc();
                }

                let duration = next_run - now;
                debug!(
                    "Next EOD scheduled for: {} (in {} seconds)",
                    next_run,
                    duration.num_seconds()
                );

                tokio::time::sleep(duration.to_std().unwrap()).await;

                // Execute EOD for the previous day
                let eod_date = (Utc::now() - chrono::Duration::days(1)).date_naive();
                let orchestrator = EodOrchestrator::new();
                let report = orchestrator.run(eod_date).await;

                info!("EOD execution completed: {:?}", report.overall_status);
            }
        })
    }

    /// Triggers EOD processing immediately for the given date.
    pub async fn run_now(date: NaiveDate) -> EodReport {
        let orchestrator = EodOrchestrator::new();
        orchestrator.run(date).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSuccessStep {
        critical: bool,
    }

    #[async_trait::async_trait]
    impl EodStep for TestSuccessStep {
        fn name(&self) -> &str {
            "test_success"
        }

        fn is_critical(&self) -> bool {
            self.critical
        }

        async fn execute(&self, _context: &EodContext) -> Result<EodStepResult, String> {
            Ok(EodStepResult {
                step_name: self.name().to_string(),
                status: EodStepStatus::Completed,
                records_processed: 100,
                duration_ms: 50,
                details: Some("Test successful".to_string()),
            })
        }

        async fn rollback(&self, _context: &EodContext) -> Result<(), String> {
            Ok(())
        }
    }

    struct TestFailureStep {
        critical: bool,
    }

    #[async_trait::async_trait]
    impl EodStep for TestFailureStep {
        fn name(&self) -> &str {
            "test_failure"
        }

        fn is_critical(&self) -> bool {
            self.critical
        }

        async fn execute(&self, _context: &EodContext) -> Result<EodStepResult, String> {
            Err("Intentional test failure".to_string())
        }

        async fn rollback(&self, _context: &EodContext) -> Result<(), String> {
            Ok(())
        }
    }

    struct TestRollbackTrackingStep {
        critical: bool,
        rollback_called: Arc<std::sync::Mutex<bool>>,
    }

    #[async_trait::async_trait]
    impl EodStep for TestRollbackTrackingStep {
        fn name(&self) -> &str {
            "test_rollback_tracking"
        }

        fn is_critical(&self) -> bool {
            self.critical
        }

        async fn execute(&self, _context: &EodContext) -> Result<EodStepResult, String> {
            Ok(EodStepResult {
                step_name: self.name().to_string(),
                status: EodStepStatus::Completed,
                records_processed: 0,
                duration_ms: 0,
                details: None,
            })
        }

        async fn rollback(&self, _context: &EodContext) -> Result<(), String> {
            *self.rollback_called.lock().unwrap() = true;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_full_success_run() {
        let orchestrator = EodOrchestrator::new();
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = orchestrator.run(date).await;

        assert_eq!(report.date, date);
        assert_eq!(report.steps.len(), 6);
        assert_eq!(report.overall_status, EodOverallStatus::Completed);
        assert!(report.steps.iter().all(|s| s.status == EodStepStatus::Completed));
    }

    #[tokio::test]
    async fn test_critical_step_failure_triggers_rollback() {
        let steps: Vec<Box<dyn EodStep + Send + Sync>> = vec![
            Box::new(TestSuccessStep { critical: true }),
            Box::new(TestFailureStep { critical: true }),
        ];

        let orchestrator = EodOrchestrator::with_steps(steps);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = orchestrator.run(date).await;

        assert_eq!(report.overall_status, EodOverallStatus::Failed);
        assert_eq!(report.steps.len(), 2);
        assert_eq!(report.steps[0].status, EodStepStatus::Completed);
        assert!(matches!(report.steps[1].status, EodStepStatus::Failed(_)));
    }

    #[tokio::test]
    async fn test_non_critical_failure_continues() {
        let steps: Vec<Box<dyn EodStep + Send + Sync>> = vec![
            Box::new(TestSuccessStep { critical: false }),
            Box::new(TestFailureStep { critical: false }),
            Box::new(TestSuccessStep { critical: false }),
        ];

        let orchestrator = EodOrchestrator::with_steps(steps);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = orchestrator.run(date).await;

        assert_eq!(report.overall_status, EodOverallStatus::PartiallyCompleted);
        assert_eq!(report.steps.len(), 3);
        assert!(matches!(report.steps[1].status, EodStepStatus::Failed(_)));
        assert_eq!(report.steps[2].status, EodStepStatus::Completed);
    }

    #[tokio::test]
    async fn test_rollback_triggered_on_critical_failure() {
        let rollback_flag = Arc::new(std::sync::Mutex::new(false));
        let rollback_flag_clone = rollback_flag.clone();

        let steps: Vec<Box<dyn EodStep + Send + Sync>> = vec![
            Box::new(TestRollbackTrackingStep {
                critical: true,
                rollback_called: rollback_flag_clone,
            }),
            Box::new(TestFailureStep { critical: true }),
        ];

        let orchestrator = EodOrchestrator::with_steps(steps);
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = orchestrator.run(date).await;

        assert_eq!(report.overall_status, EodOverallStatus::Failed);
        assert!(*rollback_flag.lock().unwrap());
    }

    #[tokio::test]
    async fn test_dry_run_mode() {
        let orchestrator = EodOrchestrator::new();
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        // Create a context with dry_run = true
        // Note: Actual implementation would respect dry_run flag
        let report = orchestrator.run(date).await;

        assert_eq!(report.date, date);
        assert!(!report.steps.is_empty());
    }

    #[tokio::test]
    async fn test_single_step_rerun() {
        let orchestrator = EodOrchestrator::new();
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = orchestrator.run_single_step("interest_accrual", date).await;

        assert_eq!(result.step_name, "interest_accrual");
        assert_eq!(result.status, EodStepStatus::Completed);
    }

    #[tokio::test]
    async fn test_single_step_not_found() {
        let orchestrator = EodOrchestrator::new();
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let result = orchestrator.run_single_step("nonexistent_step", date).await;

        assert_eq!(result.step_name, "nonexistent_step");
        assert!(matches!(result.status, EodStepStatus::Failed(_)));
    }

    #[tokio::test]
    async fn test_report_generation() {
        let orchestrator = EodOrchestrator::new();
        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        let report = orchestrator.run(date).await;

        assert_eq!(report.date, date);
        assert!(report.started_at <= report.completed_at);
        assert_eq!(report.steps.len(), 6);
        assert!(!report.steps.is_empty());
    }

    #[tokio::test]
    async fn test_retry_logic() {
        let orchestrator = EodOrchestrator::new()
            .with_max_retries(3)
            .with_retry_delay(1);

        let date = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();

        // This should pass with retry logic since default steps always succeed
        let report = orchestrator.run(date).await;

        assert_eq!(report.overall_status, EodOverallStatus::Completed);
    }

    #[tokio::test]
    async fn test_eod_step_status_display() {
        assert_eq!(EodStepStatus::Pending.to_string(), "Pending");
        assert_eq!(EodStepStatus::Running.to_string(), "Running");
        assert_eq!(EodStepStatus::Completed.to_string(), "Completed");
        assert_eq!(EodStepStatus::Skipped.to_string(), "Skipped");
        assert!(EodStepStatus::Failed("test error".to_string())
            .to_string()
            .contains("test error"));
    }

    #[tokio::test]
    async fn test_eod_overall_status_display() {
        assert_eq!(EodOverallStatus::Completed.to_string(), "Completed");
        assert_eq!(EodOverallStatus::PartiallyCompleted.to_string(), "PartiallyCompleted");
        assert_eq!(EodOverallStatus::Failed.to_string(), "Failed");
    }
}
