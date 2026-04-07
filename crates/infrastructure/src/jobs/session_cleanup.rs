use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info};

use banko_application::identity::SessionService;

/// SessionCleanupJob is responsible for periodically removing expired sessions from the database.
/// It runs as a background task and can be gracefully shut down via a cancellation token.
pub struct SessionCleanupJob {
    session_service: Arc<SessionService>,
    interval_secs: u64,
}

impl SessionCleanupJob {
    /// Creates a new SessionCleanupJob with a default interval of 3600 seconds (1 hour).
    pub fn new(session_service: Arc<SessionService>) -> Self {
        SessionCleanupJob {
            session_service,
            interval_secs: 3600, // 1 hour default
        }
    }

    /// Creates a new SessionCleanupJob with a custom interval.
    pub fn with_interval(session_service: Arc<SessionService>, interval_secs: u64) -> Self {
        SessionCleanupJob {
            session_service,
            interval_secs,
        }
    }

    /// Spawns the cleanup job as a background task.
    /// The task will run periodically and clean up expired sessions.
    /// To stop the job, drop the returned JoinHandle or abort it.
    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut cleanup_interval = interval(Duration::from_secs(self.interval_secs));

            loop {
                // Wait for the next interval tick
                cleanup_interval.tick().await;

                debug!("Running session cleanup job");

                match self.session_service.cleanup_expired().await {
                    Ok(removed_count) => {
                        if removed_count > 0 {
                            info!("Session cleanup completed: removed {} expired session(s)", removed_count);
                        } else {
                            debug!("Session cleanup completed: no expired sessions to remove");
                        }
                    }
                    Err(e) => {
                        error!("Session cleanup failed: {}", e);
                    }
                }
            }
        })
    }

    /// Runs a single cleanup cycle synchronously.
    /// Useful for testing or manual triggers.
    pub async fn run_once(&self) -> Result<u64, String> {
        debug!("Running manual session cleanup");
        self.session_service.cleanup_expired().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use banko_domain::identity::{Session, SessionId, UserId};
    use std::sync::Mutex;

    struct MockSessionRepository {
        sessions: Mutex<Vec<Session>>,
    }

    impl MockSessionRepository {
        fn new() -> Self {
            MockSessionRepository {
                sessions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl banko_application::identity::ports::ISessionRepository for MockSessionRepository {
        async fn save(&self, session: &Session) -> Result<(), String> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.retain(|s| s.id() != session.id());
            sessions.push(session.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &SessionId) -> Result<Option<Session>, String> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.iter().find(|s| s.id() == id).cloned())
        }

        async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<Session>, String> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions
                .iter()
                .find(|s| s.token_hash() == token_hash)
                .cloned())
        }

        async fn find_active_by_user_id(&self, user_id: &UserId) -> Result<Vec<Session>, String> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions
                .iter()
                .filter(|s| s.user_id() == user_id && s.is_active())
                .cloned()
                .collect())
        }

        async fn delete_by_id(&self, id: &SessionId) -> Result<(), String> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.retain(|s| s.id() != id);
            Ok(())
        }

        async fn delete_expired(&self) -> Result<u64, String> {
            let mut sessions = self.sessions.lock().unwrap();
            let before = sessions.len();
            sessions.retain(|s| !s.is_expired());
            Ok((before - sessions.len()) as u64)
        }
    }

    #[tokio::test]
    async fn test_run_once() {
        let repo = Arc::new(MockSessionRepository::new());
        let service = SessionService::new(repo);
        let job = SessionCleanupJob::new(Arc::new(service));

        // Should return 0 when no expired sessions exist
        let result = job.run_once().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_run_once_with_expired_sessions() {
        let repo = Arc::new(MockSessionRepository::new());
        let service = Arc::new(SessionService::with_ttl(repo, -10)); // Create with expired TTL
        let job = SessionCleanupJob::new(service.clone());

        // Create an expired session
        let user_id = UserId::new();
        let _session = service
            .create_session(user_id, "test_token".to_string(), None, None)
            .await
            .unwrap();

        // Run cleanup
        let result = job.run_once().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_spawn_task() {
        let repo = Arc::new(MockSessionRepository::new());
        let service = Arc::new(SessionService::new(repo));
        let job = SessionCleanupJob::with_interval(service, 1); // 1 second interval for testing

        let handle = job.spawn();

        // Give the task a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Abort the task
        handle.abort();
    }
}
