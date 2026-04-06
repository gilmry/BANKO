use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error, info, warn};

use super::providers::SmtpEmailProvider;

/// Result of processing a batch of notifications
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub processed_count: u64,
    pub sent_count: u64,
    pub failed_count: u64,
}

/// NotificationQueueWorker manages the asynchronous notification delivery queue
///
/// It periodically processes pending notifications in batches with exponential
/// backoff for retries: 2^retry_count * 30 seconds
pub struct NotificationQueueWorker {
    email_provider: Arc<SmtpEmailProvider>,
    interval_duration: Duration,
    batch_size: usize,
}

impl NotificationQueueWorker {
    /// Creates a new notification queue worker with default settings
    ///
    /// Default interval: 30 seconds
    /// Default batch size: 50 notifications
    pub fn new(email_provider: Arc<SmtpEmailProvider>) -> Self {
        NotificationQueueWorker {
            email_provider,
            interval_duration: Duration::from_secs(30),
            batch_size: 50,
        }
    }

    /// Creates a new notification queue worker with custom interval
    pub fn with_interval(
        email_provider: Arc<SmtpEmailProvider>,
        interval_secs: u64,
    ) -> Self {
        NotificationQueueWorker {
            email_provider,
            interval_duration: Duration::from_secs(interval_secs),
            batch_size: 50,
        }
    }

    /// Creates a new notification queue worker with custom settings
    pub fn with_settings(
        email_provider: Arc<SmtpEmailProvider>,
        interval_secs: u64,
        batch_size: usize,
    ) -> Self {
        NotificationQueueWorker {
            email_provider,
            interval_duration: Duration::from_secs(interval_secs),
            batch_size,
        }
    }

    /// Spawns the notification queue worker as a background task
    ///
    /// The task will run periodically and process pending notifications.
    /// To stop the worker, drop the returned JoinHandle or abort it.
    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut processing_interval = interval(self.interval_duration);

            loop {
                // Wait for the next interval tick
                processing_interval.tick().await;

                debug!(
                    batch_size = self.batch_size,
                    "Running notification queue worker"
                );

                match self.run_once().await {
                    Ok(result) => {
                        if result.processed_count > 0 {
                            info!(
                                processed = result.processed_count,
                                sent = result.sent_count,
                                failed = result.failed_count,
                                "Notification queue processing completed"
                            );
                        } else {
                            debug!("No pending notifications to process");
                        }
                    }
                    Err(e) => {
                        error!("Notification queue processing failed: {}", e);
                    }
                }
            }
        })
    }

    /// Runs a single notification processing cycle
    ///
    /// This is useful for testing or manual triggers.
    /// In a real implementation, this would:
    /// 1. Query the database for pending notifications
    /// 2. Attempt to send each notification
    /// 3. Update the database with success/failure status
    /// 4. Implement exponential backoff for retries
    pub async fn run_once(&self) -> Result<ProcessingResult, String> {
        debug!(
            batch_size = self.batch_size,
            "Running manual notification processing cycle"
        );

        // In a real implementation, this would:
        // 1. Query for notifications with status = 'Pending' or 'Retrying'
        // 2. Order by created_at, then by retry_count
        // 3. Limit to batch_size
        // 4. Calculate backoff: if retry_count > 0, check if 2^retry_count * 30 seconds have passed
        // 5. For each notification, call the appropriate provider method
        // 6. Update notification status and error_message
        //
        // For now, return a mock result

        Ok(ProcessingResult {
            processed_count: 0,
            sent_count: 0,
            failed_count: 0,
        })
    }

    /// Calculates the exponential backoff duration for a retry
    ///
    /// Formula: 2^retry_count * 30 seconds
    pub fn calculate_backoff(retry_count: i32) -> Duration {
        let multiplier = 2_u64.pow(retry_count as u32);
        Duration::from_secs(multiplier * 30)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_provider() -> Arc<SmtpEmailProvider> {
        Arc::new(SmtpEmailProvider {
            host: "smtp.test.com".to_string(),
            port: 587,
            username: "user".to_string(),
            password: "pass".to_string(),
            from_address: "test@example.com".to_string(),
            use_tls: true,
        })
    }

    #[tokio::test]
    async fn test_worker_creation() {
        let provider = create_test_provider();
        let worker = NotificationQueueWorker::new(provider);

        assert_eq!(worker.batch_size, 50);
        assert_eq!(worker.interval_duration.as_secs(), 30);
    }

    #[tokio::test]
    async fn test_worker_with_custom_interval() {
        let provider = create_test_provider();
        let worker = NotificationQueueWorker::with_interval(provider, 60);

        assert_eq!(worker.batch_size, 50);
        assert_eq!(worker.interval_duration.as_secs(), 60);
    }

    #[tokio::test]
    async fn test_worker_with_custom_settings() {
        let provider = create_test_provider();
        let worker = NotificationQueueWorker::with_settings(provider, 45, 100);

        assert_eq!(worker.batch_size, 100);
        assert_eq!(worker.interval_duration.as_secs(), 45);
    }

    #[tokio::test]
    async fn test_run_once() {
        let provider = create_test_provider();
        let worker = NotificationQueueWorker::new(provider);

        let result = worker.run_once().await;
        assert!(result.is_ok());

        let res = result.unwrap();
        assert_eq!(res.processed_count, 0);
        assert_eq!(res.sent_count, 0);
        assert_eq!(res.failed_count, 0);
    }

    #[test]
    fn test_exponential_backoff_calculation() {
        // retry_count = 0: 2^0 * 30 = 30 seconds
        assert_eq!(
            NotificationQueueWorker::calculate_backoff(0),
            Duration::from_secs(30)
        );

        // retry_count = 1: 2^1 * 30 = 60 seconds
        assert_eq!(
            NotificationQueueWorker::calculate_backoff(1),
            Duration::from_secs(60)
        );

        // retry_count = 2: 2^2 * 30 = 120 seconds
        assert_eq!(
            NotificationQueueWorker::calculate_backoff(2),
            Duration::from_secs(120)
        );

        // retry_count = 3: 2^3 * 30 = 240 seconds
        assert_eq!(
            NotificationQueueWorker::calculate_backoff(3),
            Duration::from_secs(240)
        );

        // retry_count = 5: 2^5 * 30 = 960 seconds
        assert_eq!(
            NotificationQueueWorker::calculate_backoff(5),
            Duration::from_secs(960)
        );
    }

    #[tokio::test]
    async fn test_spawn_task() {
        let provider = create_test_provider();
        let worker = NotificationQueueWorker::with_interval(provider, 1);

        let handle = worker.spawn();

        // Give the task a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Abort the task
        handle.abort();
    }
}
