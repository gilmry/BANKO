use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::UserId;

/// 2FA state for a user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwoFactorStatus {
    Disabled,
    PendingVerification,
    Enabled,
}

/// TwoFactorAuth aggregate — holds the TOTP secret and status for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorAuth {
    user_id: UserId,
    secret: String,
    status: TwoFactorStatus,
    backup_codes: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TwoFactorAuth {
    pub fn new(user_id: UserId, secret: String) -> Self {
        let now = Utc::now();
        TwoFactorAuth {
            user_id,
            secret,
            status: TwoFactorStatus::PendingVerification,
            backup_codes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn reconstitute(
        user_id: UserId,
        secret: String,
        status: TwoFactorStatus,
        backup_codes: Vec<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        TwoFactorAuth {
            user_id,
            secret,
            status,
            backup_codes,
            created_at,
            updated_at,
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn status(&self) -> &TwoFactorStatus {
        &self.status
    }

    pub fn backup_codes(&self) -> &[String] {
        &self.backup_codes
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn is_enabled(&self) -> bool {
        self.status == TwoFactorStatus::Enabled
    }

    pub fn confirm(&mut self) {
        self.status = TwoFactorStatus::Enabled;
        self.updated_at = Utc::now();
    }

    pub fn disable(&mut self) {
        self.status = TwoFactorStatus::Disabled;
        self.updated_at = Utc::now();
    }

    pub fn set_backup_codes(&mut self, codes: Vec<String>) {
        self.backup_codes = codes;
        self.updated_at = Utc::now();
    }

    pub fn use_backup_code(&mut self, code: &str) -> bool {
        if let Some(pos) = self.backup_codes.iter().position(|c| c == code) {
            self.backup_codes.remove(pos);
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_user_id() -> UserId {
        UserId::new()
    }

    #[test]
    fn test_two_factor_new() {
        let tfa = TwoFactorAuth::new(test_user_id(), "JBSWY3DPEHPK3PXP".to_string());
        assert_eq!(tfa.status(), &TwoFactorStatus::PendingVerification);
        assert_eq!(tfa.secret(), "JBSWY3DPEHPK3PXP");
        assert!(!tfa.is_enabled());
    }

    #[test]
    fn test_two_factor_confirm() {
        let mut tfa = TwoFactorAuth::new(test_user_id(), "JBSWY3DPEHPK3PXP".to_string());
        tfa.confirm();
        assert!(tfa.is_enabled());
        assert_eq!(tfa.status(), &TwoFactorStatus::Enabled);
    }

    #[test]
    fn test_two_factor_disable() {
        let mut tfa = TwoFactorAuth::new(test_user_id(), "JBSWY3DPEHPK3PXP".to_string());
        tfa.confirm();
        tfa.disable();
        assert!(!tfa.is_enabled());
    }

    #[test]
    fn test_backup_codes() {
        let mut tfa = TwoFactorAuth::new(test_user_id(), "secret".to_string());
        tfa.set_backup_codes(vec!["code1".to_string(), "code2".to_string(), "code3".to_string()]);
        assert_eq!(tfa.backup_codes().len(), 3);

        assert!(tfa.use_backup_code("code2"));
        assert_eq!(tfa.backup_codes().len(), 2);

        assert!(!tfa.use_backup_code("code2")); // already used
        assert_eq!(tfa.backup_codes().len(), 2);
    }
}
