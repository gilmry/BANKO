use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use super::ports::IPasswordHasher;

/// Mobile platform enum
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MobilePlatform {
    Ios,
    Android,
}

impl MobilePlatform {
    pub fn as_str(&self) -> &str {
        match self {
            MobilePlatform::Ios => "Ios",
            MobilePlatform::Android => "Android",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "Ios" => Some(MobilePlatform::Ios),
            "Android" => Some(MobilePlatform::Android),
            _ => None,
        }
    }
}

/// Device registration with mobile-specific data
#[derive(Debug, Clone)]
pub struct DeviceRegistration {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub platform: MobilePlatform,
    pub push_token: Option<String>,
    pub biometric_enabled: bool,
    pub pin_hash: Option<String>,
    pub registered_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub is_active: bool,
}

/// Mobile session for API access
#[derive(Debug, Clone)]
pub struct MobileSession {
    pub session_id: Uuid,
    pub customer_id: Uuid,
    pub device_id: String,
    pub token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Errors for mobile authentication
#[derive(Debug, thiserror::Error)]
pub enum MobileAuthError {
    #[error("Device limit exceeded (max 5 devices per customer)")]
    DeviceLimitExceeded,

    #[error("Device not found")]
    DeviceNotFound,

    #[error("Invalid PIN")]
    InvalidPin,

    #[error("Invalid biometric token")]
    InvalidBiometric,

    #[error("PIN not set")]
    PinNotSet,

    #[error("Biometric not enabled")]
    BiometricNotEnabled,

    #[error("Device not active")]
    DeviceNotActive,

    #[error("Session expired")]
    SessionExpired,

    #[error("Invalid refresh token")]
    InvalidRefreshToken,

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Port for device repository
#[async_trait]
pub trait IDeviceRepository: Send + Sync {
    async fn save(&self, device: &DeviceRegistration) -> Result<(), String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<DeviceRegistration>, String>;
    async fn find_by_customer(&self, customer_id: &Uuid) -> Result<Vec<DeviceRegistration>, String>;
    async fn find_by_device_id(&self, device_id: &str) -> Result<Option<DeviceRegistration>, String>;
    async fn update(&self, device: &DeviceRegistration) -> Result<(), String>;
    async fn deactivate(&self, id: &Uuid) -> Result<(), String>;
    async fn count_active_by_customer(&self, customer_id: &Uuid) -> Result<usize, String>;
}

/// Port for mobile session repository
#[async_trait]
pub trait IMobileSessionRepository: Send + Sync {
    async fn save(&self, session: &MobileSession) -> Result<(), String>;
    async fn find_by_session_id(&self, session_id: &Uuid) -> Result<Option<MobileSession>, String>;
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<MobileSession>, String>;
    async fn find_by_refresh_token_hash(&self, refresh_token_hash: &str) -> Result<Option<MobileSession>, String>;
    async fn delete_expired(&self) -> Result<u64, String>;
}

/// Mobile Authentication Service
pub struct MobileAuthService {
    device_repo: Arc<dyn IDeviceRepository>,
    session_repo: Arc<dyn IMobileSessionRepository>,
    hasher: Arc<dyn IPasswordHasher>,
}

impl MobileAuthService {
    pub fn new(
        device_repo: Arc<dyn IDeviceRepository>,
        session_repo: Arc<dyn IMobileSessionRepository>,
        hasher: Arc<dyn IPasswordHasher>,
    ) -> Self {
        MobileAuthService {
            device_repo,
            session_repo,
            hasher,
        }
    }

    /// Register a new mobile device (max 5 per customer)
    pub async fn register_device(
        &self,
        customer_id: Uuid,
        device_id: String,
        device_name: String,
        platform: MobilePlatform,
    ) -> Result<DeviceRegistration, MobileAuthError> {
        // Check device limit
        let active_count = self
            .device_repo
            .count_active_by_customer(&customer_id)
            .await
            .map_err(MobileAuthError::Internal)?;

        if active_count >= 5 {
            return Err(MobileAuthError::DeviceLimitExceeded);
        }

        let now = Utc::now();
        let device = DeviceRegistration {
            id: Uuid::new_v4(),
            customer_id,
            device_id,
            device_name,
            platform,
            push_token: None,
            biometric_enabled: false,
            pin_hash: None,
            registered_at: now,
            last_active_at: now,
            is_active: true,
        };

        self.device_repo
            .save(&device)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(device)
    }

    /// Login with PIN or biometric token
    pub async fn login_mobile(
        &self,
        device_id: &str,
        pin_or_biometric: &str,
    ) -> Result<MobileSession, MobileAuthError> {
        // Find device
        let mut device = self
            .device_repo
            .find_by_device_id(device_id)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::DeviceNotFound)?;

        if !device.is_active {
            return Err(MobileAuthError::DeviceNotActive);
        }

        // Validate authentication
        let is_biometric = pin_or_biometric.starts_with("bio_");

        if is_biometric {
            if !device.biometric_enabled {
                return Err(MobileAuthError::BiometricNotEnabled);
            }
            // Biometric token validation (simplified - in production use more robust validation)
            if !pin_or_biometric.starts_with("bio_valid_") {
                return Err(MobileAuthError::InvalidBiometric);
            }
        } else {
            // PIN validation
            let pin_hash = device.pin_hash.as_ref().ok_or(MobileAuthError::PinNotSet)?;
            let valid = self
                .hasher
                .verify(pin_or_biometric, pin_hash)
                .await
                .map_err(MobileAuthError::Internal)?;

            if !valid {
                return Err(MobileAuthError::InvalidPin);
            }
        }

        // Update last active
        device.last_active_at = Utc::now();
        self.device_repo
            .update(&device)
            .await
            .map_err(MobileAuthError::Internal)?;

        // Create session (30 min TTL)
        let session = MobileSession {
            session_id: Uuid::new_v4(),
            customer_id: device.customer_id,
            device_id: device.device_id.clone(),
            token: format!("mobile_{}", Uuid::new_v4()),
            refresh_token: format!("refresh_{}", Uuid::new_v4()),
            expires_at: Utc::now() + chrono::Duration::minutes(30),
            created_at: Utc::now(),
        };

        self.session_repo
            .save(&session)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(session)
    }

    /// Refresh mobile session
    pub async fn refresh_session(&self, refresh_token: &str) -> Result<MobileSession, MobileAuthError> {
        // Find by refresh token hash (hashed version of refresh_token)
        let old_session = self
            .session_repo
            .find_by_refresh_token_hash(refresh_token)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::InvalidRefreshToken)?;

        if old_session.expires_at < Utc::now() {
            return Err(MobileAuthError::SessionExpired);
        }

        // Verify device still exists and is active
        let device = self
            .device_repo
            .find_by_device_id(&old_session.device_id)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::DeviceNotFound)?;

        if !device.is_active {
            return Err(MobileAuthError::DeviceNotActive);
        }

        // Create new session
        let new_session = MobileSession {
            session_id: Uuid::new_v4(),
            customer_id: old_session.customer_id,
            device_id: old_session.device_id,
            token: format!("mobile_{}", Uuid::new_v4()),
            refresh_token: format!("refresh_{}", Uuid::new_v4()),
            expires_at: Utc::now() + chrono::Duration::minutes(30),
            created_at: Utc::now(),
        };

        self.session_repo
            .save(&new_session)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(new_session)
    }

    /// Enable biometric for device
    pub async fn enable_biometric(
        &self,
        device_id: &str,
        biometric_data_hash: &str,
    ) -> Result<(), MobileAuthError> {
        let mut device = self
            .device_repo
            .find_by_device_id(device_id)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::DeviceNotFound)?;

        device.biometric_enabled = true;
        // Store biometric hash as pin_hash (repurposed field)
        device.pin_hash = Some(biometric_data_hash.to_string());

        self.device_repo
            .update(&device)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(())
    }

    /// Set PIN for device
    pub async fn set_pin(&self, device_id: &str, pin: &str) -> Result<(), MobileAuthError> {
        // Validate PIN length
        if pin.len() < 4 || pin.len() > 6 {
            return Err(MobileAuthError::Internal("PIN must be 4-6 digits".to_string()));
        }

        let mut device = self
            .device_repo
            .find_by_device_id(device_id)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::DeviceNotFound)?;

        // Hash PIN using bcrypt
        let pin_hash = self
            .hasher
            .hash(pin)
            .await
            .map_err(MobileAuthError::Internal)?;

        device.pin_hash = Some(pin_hash);

        self.device_repo
            .update(&device)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(())
    }

    /// Deactivate a device
    pub async fn deactivate_device(&self, device_id: &str) -> Result<(), MobileAuthError> {
        let device = self
            .device_repo
            .find_by_device_id(device_id)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::DeviceNotFound)?;

        self.device_repo
            .deactivate(&device.id)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(())
    }

    /// List all active devices for customer
    pub async fn list_devices(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<DeviceRegistration>, MobileAuthError> {
        self.device_repo
            .find_by_customer(&customer_id)
            .await
            .map_err(MobileAuthError::Internal)
    }

    /// Update push token for device
    pub async fn update_push_token(
        &self,
        device_id: &str,
        push_token: &str,
    ) -> Result<(), MobileAuthError> {
        let mut device = self
            .device_repo
            .find_by_device_id(device_id)
            .await
            .map_err(MobileAuthError::Internal)?
            .ok_or(MobileAuthError::DeviceNotFound)?;

        device.push_token = Some(push_token.to_string());

        self.device_repo
            .update(&device)
            .await
            .map_err(MobileAuthError::Internal)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDeviceRepository {
        devices: std::sync::Mutex<Vec<DeviceRegistration>>,
    }

    #[async_trait]
    impl IDeviceRepository for MockDeviceRepository {
        async fn save(&self, device: &DeviceRegistration) -> Result<(), String> {
            let mut devices = self.devices.lock().unwrap();
            devices.push(device.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &Uuid) -> Result<Option<DeviceRegistration>, String> {
            let devices = self.devices.lock().unwrap();
            Ok(devices.iter().find(|d| d.id == *id).cloned())
        }

        async fn find_by_customer(&self, customer_id: &Uuid) -> Result<Vec<DeviceRegistration>, String> {
            let devices = self.devices.lock().unwrap();
            Ok(devices.iter().filter(|d| d.customer_id == *customer_id).cloned().collect())
        }

        async fn find_by_device_id(&self, device_id: &str) -> Result<Option<DeviceRegistration>, String> {
            let devices = self.devices.lock().unwrap();
            Ok(devices.iter().find(|d| d.device_id == device_id).cloned())
        }

        async fn update(&self, device: &DeviceRegistration) -> Result<(), String> {
            let mut devices = self.devices.lock().unwrap();
            if let Some(pos) = devices.iter().position(|d| d.id == device.id) {
                devices[pos] = device.clone();
                Ok(())
            } else {
                Err("Device not found".to_string())
            }
        }

        async fn deactivate(&self, id: &Uuid) -> Result<(), String> {
            let mut devices = self.devices.lock().unwrap();
            if let Some(device) = devices.iter_mut().find(|d| d.id == *id) {
                device.is_active = false;
                Ok(())
            } else {
                Err("Device not found".to_string())
            }
        }

        async fn count_active_by_customer(&self, customer_id: &Uuid) -> Result<usize, String> {
            let devices = self.devices.lock().unwrap();
            Ok(devices
                .iter()
                .filter(|d| d.customer_id == *customer_id && d.is_active)
                .count())
        }
    }

    struct MockSessionRepository {
        sessions: std::sync::Mutex<Vec<MobileSession>>,
    }

    #[async_trait]
    impl IMobileSessionRepository for MockSessionRepository {
        async fn save(&self, session: &MobileSession) -> Result<(), String> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.push(session.clone());
            Ok(())
        }

        async fn find_by_session_id(&self, session_id: &Uuid) -> Result<Option<MobileSession>, String> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.iter().find(|s| s.session_id == *session_id).cloned())
        }

        async fn find_by_token_hash(&self, token_hash: &str) -> Result<Option<MobileSession>, String> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.iter().find(|s| s.token.contains(token_hash)).cloned())
        }

        async fn find_by_refresh_token_hash(&self, refresh_token_hash: &str) -> Result<Option<MobileSession>, String> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.iter().find(|s| s.refresh_token == refresh_token_hash).cloned())
        }

        async fn delete_expired(&self) -> Result<u64, String> {
            let mut sessions = self.sessions.lock().unwrap();
            let before = sessions.len();
            sessions.retain(|s| s.expires_at > Utc::now());
            Ok((before - sessions.len()) as u64)
        }
    }

    struct MockHasher;

    #[async_trait]
    impl IPasswordHasher for MockHasher {
        async fn hash(&self, password: &str) -> Result<String, String> {
            Ok(format!("hashed_{}", password))
        }

        async fn verify(&self, password: &str, hash: &str) -> Result<bool, String> {
            Ok(hash == format!("hashed_{}", password))
        }
    }

    #[tokio::test]
    async fn test_register_device_success() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo, session_repo, hasher);
        let customer_id = Uuid::new_v4();

        let result = service
            .register_device(customer_id, "device123".to_string(), "My Phone".to_string(), MobilePlatform::Ios)
            .await;

        assert!(result.is_ok());
        let device = result.unwrap();
        assert_eq!(device.customer_id, customer_id);
        assert_eq!(device.device_id, "device123");
        assert!(!device.biometric_enabled);
    }

    #[tokio::test]
    async fn test_register_device_limit_exceeded() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo.clone(), session_repo, hasher);
        let customer_id = Uuid::new_v4();

        // Register 5 devices
        for i in 0..5 {
            let _ = service
                .register_device(customer_id, format!("device{}", i), format!("Phone {}", i), MobilePlatform::Ios)
                .await;
        }

        // 6th should fail
        let result = service
            .register_device(customer_id, "device5".to_string(), "Phone 5".to_string(), MobilePlatform::Android)
            .await;

        assert!(matches!(result, Err(MobileAuthError::DeviceLimitExceeded)));
    }

    #[tokio::test]
    async fn test_set_pin() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo.clone(), session_repo, hasher);
        let customer_id = Uuid::new_v4();

        let device = service
            .register_device(customer_id, "device123".to_string(), "My Phone".to_string(), MobilePlatform::Ios)
            .await
            .unwrap();

        let result = service.set_pin("device123", "1234").await;
        assert!(result.is_ok());

        // Verify PIN was set
        let updated_device = device_repo.find_by_device_id("device123").await.unwrap().unwrap();
        assert!(updated_device.pin_hash.is_some());
    }

    #[tokio::test]
    async fn test_login_with_pin() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo, session_repo, hasher);
        let customer_id = Uuid::new_v4();

        service
            .register_device(customer_id, "device123".to_string(), "My Phone".to_string(), MobilePlatform::Ios)
            .await
            .unwrap();

        service.set_pin("device123", "1234").await.unwrap();

        let result = service.login_mobile("device123", "1234").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_with_invalid_pin() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo, session_repo, hasher);
        let customer_id = Uuid::new_v4();

        service
            .register_device(customer_id, "device123".to_string(), "My Phone".to_string(), MobilePlatform::Ios)
            .await
            .unwrap();

        service.set_pin("device123", "1234").await.unwrap();

        let result = service.login_mobile("device123", "9999").await;
        assert!(matches!(result, Err(MobileAuthError::InvalidPin)));
    }

    #[tokio::test]
    async fn test_refresh_session() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo, session_repo, hasher);
        let customer_id = Uuid::new_v4();

        service
            .register_device(customer_id, "device123".to_string(), "My Phone".to_string(), MobilePlatform::Ios)
            .await
            .unwrap();

        service.set_pin("device123", "1234").await.unwrap();

        let session = service.login_mobile("device123", "1234").await.unwrap();
        let refresh_token = session.refresh_token.clone();

        let new_session = service.refresh_session(&refresh_token).await;
        assert!(new_session.is_ok());
    }

    #[tokio::test]
    async fn test_deactivate_device() {
        let device_repo = Arc::new(MockDeviceRepository {
            devices: std::sync::Mutex::new(Vec::new()),
        });
        let session_repo = Arc::new(MockSessionRepository {
            sessions: std::sync::Mutex::new(Vec::new()),
        });
        let hasher = Arc::new(MockHasher);

        let service = MobileAuthService::new(device_repo.clone(), session_repo, hasher);
        let customer_id = Uuid::new_v4();

        service
            .register_device(customer_id, "device123".to_string(), "My Phone".to_string(), MobilePlatform::Ios)
            .await
            .unwrap();

        let result = service.deactivate_device("device123").await;
        assert!(result.is_ok());

        // Verify device is inactive
        let device = device_repo.find_by_device_id("device123").await.unwrap().unwrap();
        assert!(!device.is_active);
    }
}
