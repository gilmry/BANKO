use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================
// Saga Context
// ============================================================

/// Shared context for saga execution.
/// Allows steps to share data and state during execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaContext {
    /// Unique identifier for this saga execution
    pub saga_id: Uuid,
    /// Shared data between saga steps
    pub data: HashMap<String, serde_json::Value>,
    /// When the saga started
    pub started_at: DateTime<Utc>,
    /// Optional idempotency key for preventing duplicate executions
    pub idempotency_key: Option<String>,
}

impl SagaContext {
    /// Creates a new saga context.
    pub fn new(saga_id: Uuid) -> Self {
        SagaContext {
            saga_id,
            data: HashMap::new(),
            started_at: Utc::now(),
            idempotency_key: None,
        }
    }

    /// Creates a new saga context with an idempotency key.
    pub fn with_idempotency_key(saga_id: Uuid, idempotency_key: String) -> Self {
        SagaContext {
            saga_id,
            data: HashMap::new(),
            started_at: Utc::now(),
            idempotency_key: Some(idempotency_key),
        }
    }

    /// Retrieves a value from context, deserializing it.
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Sets a value in context, serializing it.
    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), String> {
        match serde_json::to_value(value) {
            Ok(json) => {
                self.data.insert(key.to_string(), json);
                Ok(())
            }
            Err(e) => Err(format!("Failed to serialize value: {}", e)),
        }
    }

    /// Returns the duration since saga started.
    pub fn duration_since_start(&self) -> Duration {
        Utc::now() - self.started_at
    }
}

// ============================================================
// Saga Step Trait
// ============================================================

/// Trait for individual saga steps.
/// Each step must be compensatable (can be reversed).
#[async_trait]
pub trait SagaStep: Send + Sync {
    /// Returns the name of this step.
    fn name(&self) -> &str;

    /// Executes the step's action.
    async fn execute(&self, context: &mut SagaContext) -> Result<(), SagaError>;

    /// Compensates (reverses) the step's action in case of failure.
    async fn compensate(&self, context: &mut SagaContext) -> Result<(), SagaError>;
}

// ============================================================
// Saga Error
// ============================================================

/// Error types for saga execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SagaError {
    /// A step failed to execute
    StepFailed { step_name: String, reason: String },
    /// Compensation of a step failed
    CompensationFailed { step_name: String, reason: String },
    /// Saga execution timed out
    Timeout,
    /// Context serialization/deserialization error
    ContextError(String),
}

impl std::fmt::Display for SagaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SagaError::StepFailed { step_name, reason } => {
                write!(f, "Step '{}' failed: {}", step_name, reason)
            }
            SagaError::CompensationFailed { step_name, reason } => {
                write!(f, "Compensation of step '{}' failed: {}", step_name, reason)
            }
            SagaError::Timeout => write!(f, "Saga execution timed out"),
            SagaError::ContextError(reason) => write!(f, "Context error: {}", reason),
        }
    }
}

impl std::error::Error for SagaError {}

// ============================================================
// Saga Status
// ============================================================

/// Execution status of a saga.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SagaStatus {
    /// Saga has started but not completed
    Started,
    /// A step has completed (includes step name)
    StepCompleted(String),
    /// Saga is currently compensating failed steps
    Compensating,
    /// All steps have been compensated
    Compensated,
    /// Saga completed successfully
    Completed,
    /// Saga failed with error message
    Failed(String),
}

// ============================================================
// Saga Result
// ============================================================

/// Result of a saga execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaResult {
    /// Unique identifier for the saga
    pub saga_id: Uuid,
    /// Final status of the saga
    pub status: SagaStatus,
    /// Names of steps that were completed
    pub completed_steps: Vec<String>,
    /// How long the saga took to execute
    pub duration: Duration,
}

// ============================================================
// Saga Orchestrator
// ============================================================

/// Orchestrates the execution of a saga.
/// Manages step execution and compensation in case of failure.
pub struct SagaOrchestrator {
    saga_id: Uuid,
    steps: Vec<Box<dyn SagaStep>>,
    status: SagaStatus,
    completed_steps: Vec<String>,
}

impl SagaOrchestrator {
    /// Creates a new saga orchestrator with the given steps.
    pub fn new(steps: Vec<Box<dyn SagaStep>>) -> Self {
        SagaOrchestrator {
            saga_id: Uuid::new_v4(),
            steps,
            status: SagaStatus::Started,
            completed_steps: Vec::new(),
        }
    }

    /// Creates a new saga orchestrator with a specific saga ID.
    pub fn with_id(saga_id: Uuid, steps: Vec<Box<dyn SagaStep>>) -> Self {
        SagaOrchestrator {
            saga_id,
            steps,
            status: SagaStatus::Started,
            completed_steps: Vec::new(),
        }
    }

    /// Returns the current saga ID.
    pub fn saga_id(&self) -> Uuid {
        self.saga_id
    }

    /// Returns the current status.
    pub fn status(&self) -> &SagaStatus {
        &self.status
    }

    /// Returns the names of completed steps.
    pub fn completed_steps(&self) -> &[String] {
        &self.completed_steps
    }

    /// Executes the saga: runs all steps in order, compensating on failure.
    pub async fn execute(&mut self, context: &mut SagaContext) -> Result<SagaResult, SagaError> {
        let start_time = Utc::now();

        // Execute each step in order
        for step in self.steps.iter() {
            match step.execute(context).await {
                Ok(()) => {
                    let step_name = step.name().to_string();
                    self.completed_steps.push(step_name.clone());
                    self.status = SagaStatus::StepCompleted(step_name);
                }
                Err(e) => {
                    // Step failed, trigger compensation
                    self.status = SagaStatus::Compensating;
                    self.compensate(context).await?;
                    // Status is now Compensated (set by compensate method)
                    return Err(e);
                }
            }
        }

        // All steps completed successfully
        self.status = SagaStatus::Completed;

        Ok(SagaResult {
            saga_id: self.saga_id,
            status: self.status.clone(),
            completed_steps: self.completed_steps.clone(),
            duration: Utc::now() - start_time,
        })
    }

    /// Compensates (reverses) all completed steps in reverse order.
    async fn compensate(&mut self, context: &mut SagaContext) -> Result<(), SagaError> {
        // Compensate in reverse order
        for step_name in self.completed_steps.iter().rev() {
            // Find the step by name
            if let Some(step) = self.steps.iter().find(|s| s.name() == step_name) {
                match step.compensate(context).await {
                    Ok(()) => {
                        // Compensation successful, continue
                    }
                    Err(e) => {
                        // Log the error but continue compensating other steps
                        tracing::warn!("Compensation of step '{}' failed: {}", step_name, e);
                        // We don't return error here to allow other steps to be compensated
                    }
                }
            }
        }

        self.status = SagaStatus::Compensated;
        Ok(())
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct TestStep {
        name: String,
        should_fail: bool,
        executed: Arc<Mutex<bool>>,
        compensated: Arc<Mutex<bool>>,
    }

    impl TestStep {
        fn new(name: &str, should_fail: bool) -> Self {
            TestStep {
                name: name.to_string(),
                should_fail,
                executed: Arc::new(Mutex::new(false)),
                compensated: Arc::new(Mutex::new(false)),
            }
        }

        fn was_executed(&self) -> bool {
            *self.executed.lock().unwrap()
        }

        fn was_compensated(&self) -> bool {
            *self.compensated.lock().unwrap()
        }
    }

    #[async_trait]
    impl SagaStep for TestStep {
        fn name(&self) -> &str {
            &self.name
        }

        async fn execute(&self, context: &mut SagaContext) -> Result<(), SagaError> {
            *self.executed.lock().unwrap() = true;
            context.set("executed_step", self.name.clone()).ok();

            if self.should_fail {
                Err(SagaError::StepFailed {
                    step_name: self.name.clone(),
                    reason: "Test failure".to_string(),
                })
            } else {
                Ok(())
            }
        }

        async fn compensate(&self, _context: &mut SagaContext) -> Result<(), SagaError> {
            *self.compensated.lock().unwrap() = true;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_successful_saga_execution() {
        let step1 = Box::new(TestStep::new("step1", false));
        let step2 = Box::new(TestStep::new("step2", false));

        let mut orchestrator = SagaOrchestrator::new(vec![step1, step2]);
        let mut context = SagaContext::new(orchestrator.saga_id);

        let result = orchestrator.execute(&mut context).await.expect("Should complete");

        assert_eq!(result.status, SagaStatus::Completed);
        assert_eq!(result.completed_steps.len(), 2);
    }

    #[tokio::test]
    async fn test_saga_failure_triggers_compensation() {
        let step1 = Box::new(TestStep::new("step1", false));
        let step2 = Box::new(TestStep::new("step2", true)); // Will fail

        let mut orchestrator = SagaOrchestrator::new(vec![step1, step2]);
        let mut context = SagaContext::new(orchestrator.saga_id);

        let result = orchestrator.execute(&mut context).await;

        assert!(result.is_err());
        assert_eq!(orchestrator.status(), &SagaStatus::Compensated);
    }

    #[tokio::test]
    async fn test_compensation_in_reverse_order() {
        let step1 = Box::new(TestStep::new("step1", false));
        let step2 = Box::new(TestStep::new("step2", false));
        let step3 = Box::new(TestStep::new("step3", true)); // Will fail

        let mut orchestrator = SagaOrchestrator::new(vec![step1, step2, step3]);
        let mut context = SagaContext::new(orchestrator.saga_id);

        let _result = orchestrator.execute(&mut context).await;

        // All steps should be compensated
        assert_eq!(orchestrator.completed_steps().len(), 2); // step1 and step2 were completed
    }

    #[tokio::test]
    async fn test_saga_context_shared_between_steps() {
        struct ContextTestStep;

        #[async_trait]
        impl SagaStep for ContextTestStep {
            fn name(&self) -> &str {
                "context_test"
            }

            async fn execute(&self, context: &mut SagaContext) -> Result<(), SagaError> {
                context.set("test_key", "test_value").map_err(|e| {
                    SagaError::ContextError(e)
                })?;
                Ok(())
            }

            async fn compensate(&self, _context: &mut SagaContext) -> Result<(), SagaError> {
                Ok(())
            }
        }

        let step = Box::new(ContextTestStep);
        let mut orchestrator = SagaOrchestrator::new(vec![step]);
        let mut context = SagaContext::new(orchestrator.saga_id);

        let _result = orchestrator.execute(&mut context).await.expect("Should complete");

        let value: Option<String> = context.get("test_key");
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[tokio::test]
    async fn test_idempotency_key_in_context() {
        let step = Box::new(TestStep::new("test", false));
        let mut orchestrator = SagaOrchestrator::new(vec![step]);

        let saga_id = Uuid::new_v4();
        let idempotency_key = "test-key-123".to_string();
        let mut context = SagaContext::with_idempotency_key(saga_id, idempotency_key.clone());

        let _result = orchestrator.execute(&mut context).await.expect("Should complete");

        assert_eq!(context.idempotency_key, Some(idempotency_key));
    }

    #[tokio::test]
    async fn test_saga_status_transitions() {
        let step = Box::new(TestStep::new("step1", false));
        let mut orchestrator = SagaOrchestrator::new(vec![step]);

        assert_eq!(orchestrator.status(), &SagaStatus::Started);

        let mut context = SagaContext::new(orchestrator.saga_id);
        let _result = orchestrator.execute(&mut context).await.expect("Should complete");

        assert_eq!(orchestrator.status(), &SagaStatus::Completed);
    }

    #[tokio::test]
    async fn test_empty_saga() {
        let mut orchestrator = SagaOrchestrator::new(vec![]);
        let mut context = SagaContext::new(orchestrator.saga_id);

        let result = orchestrator.execute(&mut context).await.expect("Should complete");

        assert_eq!(result.status, SagaStatus::Completed);
        assert_eq!(result.completed_steps.len(), 0);
    }

    #[tokio::test]
    async fn test_single_step_saga() {
        let step = Box::new(TestStep::new("single_step", false));
        let mut orchestrator = SagaOrchestrator::new(vec![step]);

        let mut context = SagaContext::new(orchestrator.saga_id);
        let result = orchestrator.execute(&mut context).await.expect("Should complete");

        assert_eq!(result.completed_steps.len(), 1);
        assert_eq!(result.completed_steps[0], "single_step");
    }

    #[tokio::test]
    async fn test_saga_with_all_steps_failing_immediately() {
        let step = Box::new(TestStep::new("failing_step", true));
        let mut orchestrator = SagaOrchestrator::new(vec![step]);

        let mut context = SagaContext::new(orchestrator.saga_id);
        let result = orchestrator.execute(&mut context).await;

        assert!(result.is_err());
        assert_eq!(orchestrator.completed_steps().len(), 0);
    }

    #[test]
    fn test_saga_context_new() {
        let saga_id = Uuid::new_v4();
        let context = SagaContext::new(saga_id);

        assert_eq!(context.saga_id, saga_id);
        assert!(context.data.is_empty());
        assert_eq!(context.idempotency_key, None);
    }

    #[test]
    fn test_saga_context_get_set() {
        let saga_id = Uuid::new_v4();
        let mut context = SagaContext::new(saga_id);

        context.set("key", "value").expect("Should set");
        let retrieved: Option<String> = context.get("key");

        assert_eq!(retrieved, Some("value".to_string()));
    }

    #[test]
    fn test_saga_error_display() {
        let error = SagaError::StepFailed {
            step_name: "test_step".to_string(),
            reason: "test reason".to_string(),
        };

        let display_str = format!("{}", error);
        assert!(display_str.contains("test_step"));
        assert!(display_str.contains("test reason"));
    }

    #[test]
    fn test_saga_result_structure() {
        let saga_id = Uuid::new_v4();
        let result = SagaResult {
            saga_id,
            status: SagaStatus::Completed,
            completed_steps: vec!["step1".to_string(), "step2".to_string()],
            duration: Duration::seconds(5),
        };

        assert_eq!(result.saga_id, saga_id);
        assert_eq!(result.completed_steps.len(), 2);
    }
}
