use async_trait::async_trait;
use banko_application::identity::IPasswordHasher;

pub struct BcryptPasswordHasher {
    cost: u32,
}

impl BcryptPasswordHasher {
    pub fn new(cost: u32) -> Self {
        BcryptPasswordHasher { cost }
    }
}

impl Default for BcryptPasswordHasher {
    fn default() -> Self {
        BcryptPasswordHasher { cost: 12 }
    }
}

#[async_trait]
impl IPasswordHasher for BcryptPasswordHasher {
    async fn hash(&self, password: &str) -> Result<String, String> {
        let password = password.to_string();
        let cost = self.cost;
        tokio::task::spawn_blocking(move || {
            bcrypt::hash(password, cost).map_err(|e| format!("Hashing error: {e}"))
        })
        .await
        .map_err(|e| format!("Task join error: {e}"))?
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, String> {
        let password = password.to_string();
        let hash = hash.to_string();
        tokio::task::spawn_blocking(move || {
            bcrypt::verify(password, &hash).map_err(|e| format!("Verify error: {e}"))
        })
        .await
        .map_err(|e| format!("Task join error: {e}"))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_and_verify() {
        let hasher = BcryptPasswordHasher::new(4); // low cost for tests
        let hash = hasher.hash("SecurePass123!").await.unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$2b$"));
        assert_ne!(hash, "SecurePass123!");

        let valid = hasher.verify("SecurePass123!", &hash).await.unwrap();
        assert!(valid);

        let invalid = hasher.verify("WrongPassword1!", &hash).await.unwrap();
        assert!(!invalid);
    }

    #[tokio::test]
    async fn test_hash_not_plain_text() {
        let hasher = BcryptPasswordHasher::new(4);
        let hash = hasher.hash("password").await.unwrap();
        assert_ne!(hash, "password");
        assert!(hash.len() > 20);
    }
}
