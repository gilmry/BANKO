#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use banko_application::identity::{
    IPasswordHasher, ISessionRepository, IUserRepository, SessionService, UserService,
};
use banko_domain::identity::{PasswordHash, Session, SessionId, User, UserId};
use banko_domain::shared::EmailAddress;

pub struct MockUserRepository {
    users: Mutex<Vec<User>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        MockUserRepository {
            users: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl IUserRepository for MockUserRepository {
    async fn save(&self, user: &User) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        users.retain(|u| u.id() != user.id());
        users.push(user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, String> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.id() == id).cloned())
    }

    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>, String> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.email() == email).cloned())
    }

    async fn exists_by_email(&self, email: &EmailAddress) -> Result<bool, String> {
        let users = self.users.lock().unwrap();
        Ok(users.iter().any(|u| u.email() == email))
    }

    async fn delete(&self, id: &UserId) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        users.retain(|u| u.id() != id);
        Ok(())
    }
}

pub struct MockPasswordHasher;

#[async_trait]
impl IPasswordHasher for MockPasswordHasher {
    async fn hash(&self, password: &str) -> Result<String, String> {
        Ok(format!("$2b$12$hashed_{password}_padded_to_be_long_enough"))
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, String> {
        Ok(hash.contains(password))
    }
}

pub fn make_test_user_service() -> Arc<UserService> {
    Arc::new(UserService::new(
        Arc::new(MockUserRepository::new()),
        Arc::new(MockPasswordHasher),
    ))
}

pub fn create_test_user(email: &str) -> User {
    let email = EmailAddress::new(email).unwrap();
    let hash =
        PasswordHash::new("$2b$12$hashed_SecurePass123!_padded_to_be_long_enough".to_string())
            .unwrap();
    User::new(email, hash).unwrap()
}

pub struct MockSessionRepository {
    sessions: Mutex<Vec<Session>>,
}

impl MockSessionRepository {
    pub fn new() -> Self {
        MockSessionRepository {
            sessions: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl ISessionRepository for MockSessionRepository {
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

pub fn make_test_session_service() -> Arc<SessionService> {
    Arc::new(SessionService::new(Arc::new(MockSessionRepository::new())))
}

pub fn make_test_user_service_with_user(user: User) -> Arc<UserService> {
    let repo = MockUserRepository::new();
    {
        let mut users = repo.users.lock().unwrap();
        users.push(user);
    }
    Arc::new(UserService::new(
        Arc::new(repo),
        Arc::new(MockPasswordHasher),
    ))
}
