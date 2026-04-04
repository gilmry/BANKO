use std::sync::Arc;

use banko_domain::identity::{TwoFactorAuth, TwoFactorStatus, UserId};

use super::errors::TwoFactorError;
use super::ports::{ITotpService, ITwoFactorRepository};

pub struct TwoFactorService {
    repo: Arc<dyn ITwoFactorRepository>,
    totp: Arc<dyn ITotpService>,
}

impl TwoFactorService {
    pub fn new(repo: Arc<dyn ITwoFactorRepository>, totp: Arc<dyn ITotpService>) -> Self {
        TwoFactorService { repo, totp }
    }

    pub async fn enable(
        &self,
        user_id: &UserId,
        email: &str,
    ) -> Result<(String, String), TwoFactorError> {
        // Check if already enabled
        if let Some(existing) = self
            .repo
            .find_by_user_id(user_id)
            .await
            .map_err(TwoFactorError::Internal)?
        {
            if existing.is_enabled() {
                return Err(TwoFactorError::AlreadyEnabled);
            }
        }

        // Generate secret
        let secret = self.totp.generate_secret();
        let totp_uri = self.totp.generate_totp_uri(&secret, email);

        // Save pending 2FA
        let tfa = TwoFactorAuth::new(user_id.clone(), secret.clone());
        self.repo
            .save(&tfa)
            .await
            .map_err(TwoFactorError::Internal)?;

        Ok((secret, totp_uri))
    }

    pub async fn verify_and_activate(
        &self,
        user_id: &UserId,
        code: &str,
    ) -> Result<Vec<String>, TwoFactorError> {
        let mut tfa = self
            .repo
            .find_by_user_id(user_id)
            .await
            .map_err(TwoFactorError::Internal)?
            .ok_or(TwoFactorError::NotEnabled)?;

        if *tfa.status() != TwoFactorStatus::PendingVerification {
            return Err(TwoFactorError::NotPending);
        }

        // Verify the TOTP code
        if !self.totp.verify_code(tfa.secret(), code) {
            return Err(TwoFactorError::InvalidCode);
        }

        // Generate backup codes
        let backup_codes: Vec<String> = (0..8)
            .map(|_| format!("{:08x}", rand::random::<u32>()))
            .collect();

        tfa.confirm();
        tfa.set_backup_codes(backup_codes.clone());

        self.repo
            .save(&tfa)
            .await
            .map_err(TwoFactorError::Internal)?;

        Ok(backup_codes)
    }

    pub async fn verify_code(
        &self,
        user_id: &UserId,
        code: &str,
    ) -> Result<bool, TwoFactorError> {
        let tfa = self
            .repo
            .find_by_user_id(user_id)
            .await
            .map_err(TwoFactorError::Internal)?
            .ok_or(TwoFactorError::NotEnabled)?;

        if !tfa.is_enabled() {
            return Err(TwoFactorError::NotEnabled);
        }

        Ok(self.totp.verify_code(tfa.secret(), code))
    }

    pub async fn is_enabled(&self, user_id: &UserId) -> Result<bool, TwoFactorError> {
        match self
            .repo
            .find_by_user_id(user_id)
            .await
            .map_err(TwoFactorError::Internal)?
        {
            Some(tfa) => Ok(tfa.is_enabled()),
            None => Ok(false),
        }
    }

    pub async fn disable(&self, user_id: &UserId) -> Result<(), TwoFactorError> {
        self.repo
            .delete_by_user_id(user_id)
            .await
            .map_err(TwoFactorError::Internal)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;

    // Mock TOTP service
    struct MockTotpService {
        valid_code: String,
    }

    impl MockTotpService {
        fn new(valid_code: &str) -> Self {
            MockTotpService {
                valid_code: valid_code.to_string(),
            }
        }
    }

    impl ITotpService for MockTotpService {
        fn generate_secret(&self) -> String {
            "JBSWY3DPEHPK3PXP".to_string()
        }

        fn generate_totp_uri(&self, secret: &str, email: &str) -> String {
            format!("otpauth://totp/BANKO:{email}?secret={secret}&issuer=BANKO")
        }

        fn verify_code(&self, _secret: &str, code: &str) -> bool {
            code == self.valid_code
        }
    }

    // Mock 2FA repository
    struct MockTwoFactorRepo {
        data: Mutex<Vec<TwoFactorAuth>>,
    }

    impl MockTwoFactorRepo {
        fn new() -> Self {
            MockTwoFactorRepo {
                data: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ITwoFactorRepository for MockTwoFactorRepo {
        async fn save(&self, tfa: &TwoFactorAuth) -> Result<(), String> {
            let mut data = self.data.lock().unwrap();
            data.retain(|t| t.user_id() != tfa.user_id());
            data.push(tfa.clone());
            Ok(())
        }

        async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<TwoFactorAuth>, String> {
            let data = self.data.lock().unwrap();
            Ok(data.iter().find(|t| t.user_id() == user_id).cloned())
        }

        async fn delete_by_user_id(&self, user_id: &UserId) -> Result<(), String> {
            let mut data = self.data.lock().unwrap();
            data.retain(|t| t.user_id() != user_id);
            Ok(())
        }
    }

    fn make_service(valid_code: &str) -> TwoFactorService {
        TwoFactorService::new(
            Arc::new(MockTwoFactorRepo::new()),
            Arc::new(MockTotpService::new(valid_code)),
        )
    }

    #[tokio::test]
    async fn test_enable_2fa() {
        let service = make_service("123456");
        let user_id = UserId::new();
        let (secret, uri) = service.enable(&user_id, "test@banko.tn").await.unwrap();
        assert_eq!(secret, "JBSWY3DPEHPK3PXP");
        assert!(uri.contains("otpauth://totp/BANKO:test@banko.tn"));
    }

    #[tokio::test]
    async fn test_verify_and_activate() {
        let service = make_service("123456");
        let user_id = UserId::new();
        service.enable(&user_id, "test@banko.tn").await.unwrap();

        let backup_codes = service.verify_and_activate(&user_id, "123456").await.unwrap();
        assert_eq!(backup_codes.len(), 8);

        assert!(service.is_enabled(&user_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_verify_invalid_code() {
        let service = make_service("123456");
        let user_id = UserId::new();
        service.enable(&user_id, "test@banko.tn").await.unwrap();

        let result = service.verify_and_activate(&user_id, "000000").await;
        assert!(matches!(result, Err(TwoFactorError::InvalidCode)));
    }

    #[tokio::test]
    async fn test_enable_already_enabled() {
        let service = make_service("123456");
        let user_id = UserId::new();
        service.enable(&user_id, "test@banko.tn").await.unwrap();
        service.verify_and_activate(&user_id, "123456").await.unwrap();

        let result = service.enable(&user_id, "test@banko.tn").await;
        assert!(matches!(result, Err(TwoFactorError::AlreadyEnabled)));
    }

    #[tokio::test]
    async fn test_verify_code_when_enabled() {
        let service = make_service("654321");
        let user_id = UserId::new();
        service.enable(&user_id, "test@banko.tn").await.unwrap();
        service.verify_and_activate(&user_id, "654321").await.unwrap();

        assert!(service.verify_code(&user_id, "654321").await.unwrap());
        assert!(!service.verify_code(&user_id, "000000").await.unwrap());
    }

    #[tokio::test]
    async fn test_disable_2fa() {
        let service = make_service("123456");
        let user_id = UserId::new();
        service.enable(&user_id, "test@banko.tn").await.unwrap();
        service.verify_and_activate(&user_id, "123456").await.unwrap();
        assert!(service.is_enabled(&user_id).await.unwrap());

        service.disable(&user_id).await.unwrap();
        assert!(!service.is_enabled(&user_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_is_enabled_when_not_setup() {
        let service = make_service("123456");
        let user_id = UserId::new();
        assert!(!service.is_enabled(&user_id).await.unwrap());
    }
}
