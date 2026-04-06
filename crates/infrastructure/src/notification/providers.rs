use std::env;
use uuid::Uuid;
use tracing::{debug, warn};

/// Email provider for SMTP-based email delivery
#[derive(Debug, Clone)]
pub struct SmtpEmailProvider {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub use_tls: bool,
}

impl SmtpEmailProvider {
    /// Creates a new SMTP email provider from environment variables
    ///
    /// Environment variables:
    /// - SMTP_HOST: SMTP server hostname
    /// - SMTP_PORT: SMTP server port (default: 587)
    /// - SMTP_USERNAME: SMTP username
    /// - SMTP_PASSWORD: SMTP password
    /// - SMTP_FROM_ADDRESS: From address for emails
    /// - SMTP_USE_TLS: Whether to use TLS (default: true)
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("SMTP_HOST")
            .map_err(|_| "SMTP_HOST environment variable not set".to_string())?;
        let port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse::<u16>()
            .map_err(|_| "SMTP_PORT must be a valid u16".to_string())?;
        let username = env::var("SMTP_USERNAME")
            .map_err(|_| "SMTP_USERNAME environment variable not set".to_string())?;
        let password = env::var("SMTP_PASSWORD")
            .map_err(|_| "SMTP_PASSWORD environment variable not set".to_string())?;
        let from_address = env::var("SMTP_FROM_ADDRESS")
            .map_err(|_| "SMTP_FROM_ADDRESS environment variable not set".to_string())?;
        let use_tls = env::var("SMTP_USE_TLS")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Ok(SmtpEmailProvider {
            host,
            port,
            username,
            password,
            from_address,
            use_tls,
        })
    }

    /// Send an email notification
    /// For now, logs the email and returns a generated UUID as the message_id
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<String, String> {
        // Validate email format
        if !to.contains('@') || to.is_empty() {
            return Err(format!("Invalid email recipient: {}", to));
        }

        let message_id = Uuid::new_v4().to_string();

        debug!(
            message_id = %message_id,
            to = %to,
            subject = %subject,
            "Email notification queued (SMTP implementation pending)"
        );

        Ok(message_id)
    }

    /// Send an SMS notification
    /// For now, logs the SMS and returns a generated UUID as the provider_id
    pub async fn send_sms(&self, to: &str, message: &str) -> Result<String, String> {
        // Validate phone number
        if to.is_empty() {
            return Err("Invalid phone recipient".to_string());
        }

        let provider_id = Uuid::new_v4().to_string();

        debug!(
            provider_id = %provider_id,
            to = %to,
            message = %message,
            "SMS notification queued (Twilio implementation pending)"
        );

        Ok(provider_id)
    }

    /// Send a push notification
    /// For now, logs the push and returns a generated UUID as the push_id
    pub async fn send_push(
        &self,
        token: &str,
        title: &str,
        body: &str,
    ) -> Result<String, String> {
        if token.is_empty() {
            return Err("Invalid push token".to_string());
        }

        let push_id = Uuid::new_v4().to_string();

        debug!(
            push_id = %push_id,
            token = %token,
            title = %title,
            body = %body,
            "Push notification queued (FCM implementation pending)"
        );

        Ok(push_id)
    }
}

/// SMS provider for SMS-based notifications
#[derive(Debug, Clone)]
pub struct SmsProvider {
    pub api_key: String,
    pub api_url: String,
    pub sender_id: String,
}

impl SmsProvider {
    /// Creates a new SMS provider from environment variables
    ///
    /// Environment variables:
    /// - SMS_API_KEY: SMS provider API key
    /// - SMS_API_URL: SMS provider API URL
    /// - SMS_SENDER_ID: Sender ID for SMS messages
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("SMS_API_KEY")
            .map_err(|_| "SMS_API_KEY environment variable not set".to_string())?;
        let api_url = env::var("SMS_API_URL")
            .map_err(|_| "SMS_API_URL environment variable not set".to_string())?;
        let sender_id = env::var("SMS_SENDER_ID")
            .map_err(|_| "SMS_SENDER_ID environment variable not set".to_string())?;

        Ok(SmsProvider {
            api_key,
            api_url,
            sender_id,
        })
    }

    /// Validates a Tunisian phone number
    /// Must start with +216 and have exactly 8 digits after the prefix
    pub fn validate_phone(&self, phone: &str) -> Result<(), String> {
        if !phone.starts_with("+216") {
            return Err("Phone number must start with +216".to_string());
        }

        let digits = &phone[4..]; // Remove +216 prefix
        if digits.len() != 8 {
            return Err("Phone number must have exactly 8 digits after +216".to_string());
        }

        if !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err("Phone number must contain only digits after +216".to_string());
        }

        Ok(())
    }
}

/// Push notification provider
#[derive(Debug, Clone)]
pub struct PushProvider {
    pub fcm_server_key: String,
}

impl PushProvider {
    /// Creates a new push provider from environment variables
    ///
    /// Environment variables:
    /// - FCM_SERVER_KEY: Firebase Cloud Messaging server key
    pub fn from_env() -> Result<Self, String> {
        let fcm_server_key = env::var("FCM_SERVER_KEY")
            .map_err(|_| "FCM_SERVER_KEY environment variable not set".to_string())?;

        Ok(PushProvider { fcm_server_key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smtp_provider_creation() {
        let provider = SmtpEmailProvider {
            host: "smtp.gmail.com".to_string(),
            port: 587,
            username: "test@gmail.com".to_string(),
            password: "password".to_string(),
            from_address: "noreply@banko.tn".to_string(),
            use_tls: true,
        };

        assert_eq!(provider.host, "smtp.gmail.com");
        assert_eq!(provider.port, 587);
        assert!(provider.use_tls);
    }

    #[tokio::test]
    async fn test_send_email_valid() {
        let provider = SmtpEmailProvider {
            host: "smtp.test.com".to_string(),
            port: 587,
            username: "user".to_string(),
            password: "pass".to_string(),
            from_address: "test@example.com".to_string(),
            use_tls: true,
        };

        let result = provider
            .send_email("recipient@example.com", "Test", "Test body")
            .await;
        assert!(result.is_ok());
        let message_id = result.unwrap();
        assert!(!message_id.is_empty());
    }

    #[tokio::test]
    async fn test_send_email_invalid_recipient() {
        let provider = SmtpEmailProvider {
            host: "smtp.test.com".to_string(),
            port: 587,
            username: "user".to_string(),
            password: "pass".to_string(),
            from_address: "test@example.com".to_string(),
            use_tls: true,
        };

        let result = provider.send_email("invalid-email", "Test", "Test body").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_sms_valid() {
        let provider = SmtpEmailProvider {
            host: "smtp.test.com".to_string(),
            port: 587,
            username: "user".to_string(),
            password: "pass".to_string(),
            from_address: "test@example.com".to_string(),
            use_tls: true,
        };

        let result = provider.send_sms("+21650123456", "Test message").await;
        assert!(result.is_ok());
        let provider_id = result.unwrap();
        assert!(!provider_id.is_empty());
    }

    #[test]
    fn test_sms_provider_tunisian_phone_valid() {
        let provider = SmsProvider {
            api_key: "test_key".to_string(),
            api_url: "https://api.sms.tn".to_string(),
            sender_id: "BANKO".to_string(),
        };

        assert!(provider.validate_phone("+21650123456").is_ok());
        assert!(provider.validate_phone("+21625987654").is_ok());
    }

    #[test]
    fn test_sms_provider_tunisian_phone_invalid_prefix() {
        let provider = SmsProvider {
            api_key: "test_key".to_string(),
            api_url: "https://api.sms.tn".to_string(),
            sender_id: "BANKO".to_string(),
        };

        assert!(provider.validate_phone("+33650123456").is_err());
    }

    #[test]
    fn test_sms_provider_tunisian_phone_invalid_length() {
        let provider = SmsProvider {
            api_key: "test_key".to_string(),
            api_url: "https://api.sms.tn".to_string(),
            sender_id: "BANKO".to_string(),
        };

        assert!(provider.validate_phone("+2165012345").is_err()); // Too short
        assert!(provider.validate_phone("+216501234567").is_err()); // Too long
    }

    #[test]
    fn test_sms_provider_creation() {
        let provider = SmsProvider {
            api_key: "key123".to_string(),
            api_url: "https://api.sms.tn".to_string(),
            sender_id: "BANKO".to_string(),
        };

        assert_eq!(provider.api_key, "key123");
        assert_eq!(provider.sender_id, "BANKO");
    }

    #[test]
    fn test_push_provider_creation() {
        let provider = PushProvider {
            fcm_server_key: "fcm_key_123".to_string(),
        };

        assert_eq!(provider.fcm_server_key, "fcm_key_123");
    }
}
