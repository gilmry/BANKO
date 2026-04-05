use std::sync::Arc;

use banko_domain::identity::{PasswordHash, Role, User, UserId};
use banko_domain::shared::EmailAddress;

use super::errors::{LoginError, RegisterError, UserServiceError};
use super::ports::{IPasswordHasher, IUserRepository};

const MIN_PASSWORD_LENGTH: usize = 12;

pub struct UserService {
    repo: Arc<dyn IUserRepository>,
    hasher: Arc<dyn IPasswordHasher>,
}

impl UserService {
    pub fn new(repo: Arc<dyn IUserRepository>, hasher: Arc<dyn IPasswordHasher>) -> Self {
        UserService { repo, hasher }
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<UserId, RegisterError> {
        // Validate email
        let email =
            EmailAddress::new(email).map_err(|e| RegisterError::InvalidEmail(e.to_string()))?;

        // Validate password strength
        Self::validate_password_strength(password)?;

        // Check email uniqueness
        let exists = self
            .repo
            .exists_by_email(&email)
            .await
            .map_err(RegisterError::Internal)?;
        if exists {
            return Err(RegisterError::EmailTaken);
        }

        // Hash password
        let hash_str = self
            .hasher
            .hash(password)
            .await
            .map_err(RegisterError::Internal)?;
        let password_hash =
            PasswordHash::new(hash_str).map_err(|e| RegisterError::Internal(e.to_string()))?;

        // Create user
        let user =
            User::new(email, password_hash).map_err(|e| RegisterError::Internal(e.to_string()))?;
        let user_id = user.id().clone();

        // Persist
        self.repo
            .save(&user)
            .await
            .map_err(RegisterError::Internal)?;

        Ok(user_id)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<User, LoginError> {
        let email = EmailAddress::new(email).map_err(|_| LoginError::InvalidCredentials)?;

        let user = self
            .repo
            .find_by_email(&email)
            .await
            .map_err(LoginError::Internal)?
            .ok_or(LoginError::InvalidCredentials)?;

        if !user.is_active() {
            return Err(LoginError::AccountInactive);
        }

        let valid = self
            .hasher
            .verify(password, user.password_hash().as_str())
            .await
            .map_err(LoginError::Internal)?;

        if !valid {
            return Err(LoginError::InvalidCredentials);
        }

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &UserId) -> Result<User, UserServiceError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(UserServiceError::Internal)?
            .ok_or(UserServiceError::UserNotFound)
    }

    pub async fn update_roles(
        &self,
        user_id: &UserId,
        roles: Vec<Role>,
    ) -> Result<User, UserServiceError> {
        let mut user = self.find_by_id(user_id).await?;
        user.set_roles(roles)
            .map_err(|e| UserServiceError::InvalidRole(e.to_string()))?;
        self.repo
            .save(&user)
            .await
            .map_err(UserServiceError::Internal)?;
        Ok(user)
    }

    fn validate_password_strength(password: &str) -> Result<(), RegisterError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(RegisterError::WeakPassword(format!(
                "Password must be at least {MIN_PASSWORD_LENGTH} characters"
            )));
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(RegisterError::WeakPassword(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(RegisterError::WeakPassword(
                "Password must contain at least one digit".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use banko_domain::identity::PasswordHash;

    use super::*;

    // --- Mock Repository ---

    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            MockUserRepository {
                users: Mutex::new(Vec::new()),
            }
        }

        fn with_user(user: User) -> Self {
            MockUserRepository {
                users: Mutex::new(vec![user]),
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

    // --- Mock Password Hasher ---

    struct MockPasswordHasher;

    #[async_trait]
    impl IPasswordHasher for MockPasswordHasher {
        async fn hash(&self, password: &str) -> Result<String, String> {
            Ok(format!("$2b$12$hashed_{password}_padded_to_be_long_enough"))
        }

        async fn verify(&self, password: &str, hash: &str) -> Result<bool, String> {
            Ok(hash.contains(password))
        }
    }

    fn make_service() -> UserService {
        UserService::new(
            Arc::new(MockUserRepository::new()),
            Arc::new(MockPasswordHasher),
        )
    }

    fn make_service_with_repo(repo: MockUserRepository) -> UserService {
        UserService::new(Arc::new(repo), Arc::new(MockPasswordHasher))
    }

    fn create_test_user(email: &str) -> User {
        let email = EmailAddress::new(email).unwrap();
        let hash =
            PasswordHash::new("$2b$12$hashed_SecurePass123!_padded_to_be_long_enough".to_string())
                .unwrap();
        User::new(email, hash).unwrap()
    }

    // --- Register tests ---

    #[tokio::test]
    async fn test_register_success() {
        let service = make_service();
        let result = service.register("new@banko.tn", "SecurePass123!").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_invalid_email() {
        let service = make_service();
        let result = service.register("invalid", "SecurePass123!").await;
        assert!(matches!(result, Err(RegisterError::InvalidEmail(_))));
    }

    #[tokio::test]
    async fn test_register_weak_password_too_short() {
        let service = make_service();
        let result = service.register("test@banko.tn", "Short1!").await;
        assert!(matches!(result, Err(RegisterError::WeakPassword(_))));
    }

    #[tokio::test]
    async fn test_register_weak_password_no_uppercase() {
        let service = make_service();
        let result = service.register("test@banko.tn", "nouppercase123!").await;
        assert!(matches!(result, Err(RegisterError::WeakPassword(_))));
    }

    #[tokio::test]
    async fn test_register_weak_password_no_digit() {
        let service = make_service();
        let result = service.register("test@banko.tn", "NoDigitsHere!!!").await;
        assert!(matches!(result, Err(RegisterError::WeakPassword(_))));
    }

    #[tokio::test]
    async fn test_register_email_taken() {
        let repo = MockUserRepository::with_user(create_test_user("existing@banko.tn"));
        let service = make_service_with_repo(repo);
        let result = service
            .register("existing@banko.tn", "SecurePass123!")
            .await;
        assert!(matches!(result, Err(RegisterError::EmailTaken)));
    }

    // --- Login tests ---

    #[tokio::test]
    async fn test_login_success() {
        let repo = MockUserRepository::with_user(create_test_user("user@banko.tn"));
        let service = make_service_with_repo(repo);
        let result = service.login("user@banko.tn", "SecurePass123!").await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email().as_str(), "user@banko.tn");
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let service = make_service();
        let result = service.login("nobody@banko.tn", "SecurePass123!").await;
        assert!(matches!(result, Err(LoginError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let repo = MockUserRepository::with_user(create_test_user("user@banko.tn"));
        let service = make_service_with_repo(repo);
        let result = service.login("user@banko.tn", "WrongPassword1!").await;
        assert!(matches!(result, Err(LoginError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_inactive_account() {
        let mut user = create_test_user("user@banko.tn");
        user.deactivate();
        let repo = MockUserRepository::with_user(user);
        let service = make_service_with_repo(repo);
        let result = service.login("user@banko.tn", "SecurePass123!").await;
        assert!(matches!(result, Err(LoginError::AccountInactive)));
    }

    #[tokio::test]
    async fn test_login_error_no_info_leak() {
        let service = make_service();
        let result = service.login("nobody@banko.tn", "anything").await;
        // Error should say "Invalid credentials", not "User not found"
        let err_msg = format!("{}", result.unwrap_err());
        assert_eq!(err_msg, "Invalid credentials");
        assert!(!err_msg.contains("not found"));
    }

    // --- Find/Update tests ---

    #[tokio::test]
    async fn test_find_by_id_success() {
        let user = create_test_user("find@banko.tn");
        let user_id = user.id().clone();
        let repo = MockUserRepository::with_user(user);
        let service = make_service_with_repo(repo);
        let found = service.find_by_id(&user_id).await.unwrap();
        assert_eq!(found.email().as_str(), "find@banko.tn");
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let service = make_service();
        let result = service.find_by_id(&UserId::new()).await;
        assert!(matches!(result, Err(UserServiceError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_update_roles() {
        let user = create_test_user("roles@banko.tn");
        let user_id = user.id().clone();
        let repo = MockUserRepository::with_user(user);
        let service = make_service_with_repo(repo);
        let updated = service
            .update_roles(&user_id, vec![Role::Admin, Role::Analyst])
            .await
            .unwrap();
        assert!(updated.has_role(&Role::Admin));
        assert!(updated.has_role(&Role::Analyst));
    }

    #[tokio::test]
    async fn test_update_roles_empty_rejected() {
        let user = create_test_user("roles@banko.tn");
        let user_id = user.id().clone();
        let repo = MockUserRepository::with_user(user);
        let service = make_service_with_repo(repo);
        let result = service.update_roles(&user_id, vec![]).await;
        assert!(matches!(result, Err(UserServiceError::InvalidRole(_))));
    }
}
