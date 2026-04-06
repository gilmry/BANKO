use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        SessionId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        SessionId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session aggregate — tracks a user's active session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: SessionId,
    user_id: UserId,
    token_hash: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    is_active: bool,
}

impl Session {
    pub fn new(
        user_id: UserId,
        token_hash: String,
        ttl_secs: i64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Session {
            id: SessionId::new(),
            user_id,
            token_hash,
            ip_address,
            user_agent,
            created_at: now,
            expires_at: now + Duration::seconds(ttl_secs),
            is_active: true,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: SessionId,
        user_id: UserId,
        token_hash: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        is_active: bool,
    ) -> Self {
        Session {
            id,
            user_id,
            token_hash,
            ip_address,
            user_agent,
            created_at,
            expires_at,
            is_active,
        }
    }

    pub fn id(&self) -> &SessionId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn token_hash(&self) -> &str {
        &self.token_hash
    }

    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }

    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        self.is_active && !self.is_expired()
    }

    pub fn invalidate(&mut self) {
        self.is_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_session(ttl: i64) -> Session {
        Session::new(
            UserId::new(),
            "token_hash_value".to_string(),
            ttl,
            Some("127.0.0.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        )
    }

    #[test]
    fn test_session_new() {
        let session = test_session(3600);
        assert!(session.is_active());
        assert!(!session.is_expired());
        assert!(session.is_valid());
        assert_eq!(session.ip_address(), Some("127.0.0.1"));
        assert_eq!(session.user_agent(), Some("Mozilla/5.0"));
    }

    #[test]
    fn test_session_expired() {
        let session = test_session(-10); // already expired
        assert!(session.is_expired());
        assert!(!session.is_valid());
    }

    #[test]
    fn test_session_invalidate() {
        let mut session = test_session(3600);
        assert!(session.is_valid());
        session.invalidate();
        assert!(!session.is_active());
        assert!(!session.is_valid());
    }

    #[test]
    fn test_session_id_unique() {
        let s1 = test_session(3600);
        let s2 = test_session(3600);
        assert_ne!(s1.id(), s2.id());
    }

    #[test]
    fn test_session_reconstitute() {
        let id = SessionId::new();
        let user_id = UserId::new();
        let now = Utc::now();
        let session = Session::reconstitute(
            id.clone(),
            user_id.clone(),
            "hash".to_string(),
            Some("10.0.0.1".to_string()),
            None,
            now,
            now + Duration::hours(1),
            true,
        );
        assert_eq!(session.id(), &id);
        assert_eq!(session.user_id(), &user_id);
        assert!(session.is_valid());
    }
}
