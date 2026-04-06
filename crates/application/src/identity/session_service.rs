use std::sync::Arc;

use banko_domain::identity::{Session, UserId};

use super::ports::ISessionRepository;

const DEFAULT_SESSION_TTL_SECS: i64 = 3600; // 1 hour

pub struct SessionService {
    repo: Arc<dyn ISessionRepository>,
    ttl_secs: i64,
}

impl SessionService {
    pub fn new(repo: Arc<dyn ISessionRepository>) -> Self {
        SessionService {
            repo,
            ttl_secs: DEFAULT_SESSION_TTL_SECS,
        }
    }

    pub fn with_ttl(repo: Arc<dyn ISessionRepository>, ttl_secs: i64) -> Self {
        SessionService { repo, ttl_secs }
    }

    pub async fn create_session(
        &self,
        user_id: UserId,
        token_hash: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session, String> {
        let session = Session::new(user_id, token_hash, self.ttl_secs, ip_address, user_agent);
        self.repo.save(&session).await?;
        Ok(session)
    }

    pub async fn validate_session(&self, token_hash: &str) -> Result<Option<Session>, String> {
        let session = self.repo.find_by_token_hash(token_hash).await?;
        match session {
            Some(s) if s.is_valid() => Ok(Some(s)),
            _ => Ok(None),
        }
    }

    pub async fn logout(&self, token_hash: &str) -> Result<(), String> {
        if let Some(mut session) = self.repo.find_by_token_hash(token_hash).await? {
            session.invalidate();
            self.repo.save(&session).await?;
        }
        Ok(())
    }

    pub async fn get_active_sessions(&self, user_id: &UserId) -> Result<Vec<Session>, String> {
        let sessions = self.repo.find_active_by_user_id(user_id).await?;
        Ok(sessions.into_iter().filter(|s| s.is_valid()).collect())
    }

    pub async fn cleanup_expired(&self) -> Result<u64, String> {
        self.repo.delete_expired().await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use banko_domain::identity::SessionId;

    use super::*;

    struct MockSessionRepo {
        sessions: Mutex<Vec<Session>>,
    }

    impl MockSessionRepo {
        fn new() -> Self {
            MockSessionRepo {
                sessions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ISessionRepository for MockSessionRepo {
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

    fn make_service() -> SessionService {
        SessionService::new(Arc::new(MockSessionRepo::new()))
    }

    fn make_expired_service() -> SessionService {
        SessionService::with_ttl(Arc::new(MockSessionRepo::new()), -10) // already expired
    }

    #[tokio::test]
    async fn test_create_session() {
        let service = make_service();
        let user_id = UserId::new();
        let session = service
            .create_session(
                user_id.clone(),
                "token_hash".to_string(),
                Some("127.0.0.1".to_string()),
                Some("TestAgent".to_string()),
            )
            .await
            .unwrap();
        assert_eq!(session.user_id(), &user_id);
        assert!(session.is_valid());
    }

    #[tokio::test]
    async fn test_validate_session() {
        let service = make_service();
        let user_id = UserId::new();
        service
            .create_session(user_id, "hash123".to_string(), None, None)
            .await
            .unwrap();

        let result = service.validate_session("hash123").await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_validate_expired_session() {
        let service = make_expired_service();
        let user_id = UserId::new();
        service
            .create_session(user_id, "hash_expired".to_string(), None, None)
            .await
            .unwrap();

        let result = service.validate_session("hash_expired").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_validate_nonexistent_session() {
        let service = make_service();
        let result = service.validate_session("no_such_hash").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_logout() {
        let service = make_service();
        let user_id = UserId::new();
        service
            .create_session(user_id, "logout_hash".to_string(), None, None)
            .await
            .unwrap();

        service.logout("logout_hash").await.unwrap();

        let result = service.validate_session("logout_hash").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_active_sessions() {
        let service = make_service();
        let user_id = UserId::new();
        service
            .create_session(user_id.clone(), "s1".to_string(), None, None)
            .await
            .unwrap();
        service
            .create_session(user_id.clone(), "s2".to_string(), None, None)
            .await
            .unwrap();

        let active = service.get_active_sessions(&user_id).await.unwrap();
        assert_eq!(active.len(), 2);
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let service = make_expired_service();
        let user_id = UserId::new();
        service
            .create_session(user_id, "expired1".to_string(), None, None)
            .await
            .unwrap();

        let removed = service.cleanup_expired().await.unwrap();
        assert_eq!(removed, 1);
    }
}
