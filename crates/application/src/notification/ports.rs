use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use banko_domain::notification::{Channel, Notification, NotificationPreference, NotificationStatus, NotificationType};

// ============================================================
// INotificationRepository
// ============================================================

#[async_trait]
pub trait INotificationRepository: Send + Sync {
    /// Save a notification
    async fn save(&self, notification: &Notification) -> Result<(), String>;

    /// Find a notification by ID
    async fn find_by_id(&self, id: &str) -> Result<Option<Notification>, String>;

    /// Find pending notifications with limit
    async fn find_pending(&self, limit: usize) -> Result<Vec<Notification>, String>;

    /// Update notification status
    async fn update_status(
        &self,
        id: &str,
        status: NotificationStatus,
    ) -> Result<(), String>;

    /// Find notifications by customer
    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<Notification>, String>;

    /// Find recent notifications for deduplication
    async fn find_recent_by_customer_channel_template(
        &self,
        customer_id: Uuid,
        channel: Channel,
        template_id: &str,
        minutes_back: i64,
    ) -> Result<Vec<Notification>, String>;
}

// ============================================================
// ITemplateRepository
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub id: String,
    pub event_type: String,
    pub channel: String,
    pub locale: String,
    pub subject_template: String,
    pub body_template: String,
}

#[async_trait]
pub trait ITemplateRepository: Send + Sync {
    /// Find a template by ID
    async fn find_by_id(&self, id: &str) -> Result<Option<TemplateInfo>, String>;

    /// Find template by event type and locale (with fallback)
    async fn find_by_event_and_locale(
        &self,
        event_type: &str,
        locale: &str,
    ) -> Result<Option<TemplateInfo>, String>;

    /// List all templates
    async fn list_all(&self) -> Result<Vec<TemplateInfo>, String>;
}

// ============================================================
// INotificationPreferenceRepository
// ============================================================

#[async_trait]
pub trait INotificationPreferenceRepository: Send + Sync {
    /// Find all preferences for a customer
    async fn find_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<NotificationPreference>, String>;

    /// Save or update a preference
    async fn save(&self, preference: &NotificationPreference) -> Result<(), String>;

    /// Find preference by customer, channel, and notification type
    async fn find_by_customer_and_channel_type(
        &self,
        customer_id: Uuid,
        channel: Channel,
        notification_type: NotificationType,
    ) -> Result<Option<NotificationPreference>, String>;
}

// ============================================================
// INotificationSender
// ============================================================

#[async_trait]
pub trait INotificationSender: Send + Sync {
    /// Send an email notification
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), String>;

    /// Send an SMS notification
    async fn send_sms(&self, to: &str, message: &str) -> Result<(), String>;

    /// Send a push notification
    async fn send_push(
        &self,
        token: &str,
        title: &str,
        body: &str,
    ) -> Result<(), String>;
}

// ============================================================
// Mock implementations for testing
// ============================================================

#[cfg(test)]
pub mod mocks {
    use super::*;
    use std::sync::Mutex;

    pub struct MockNotificationRepository {
        pub notifications: Mutex<Vec<Notification>>,
    }

    impl MockNotificationRepository {
        pub fn new() -> Self {
            MockNotificationRepository {
                notifications: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl INotificationRepository for MockNotificationRepository {
        async fn save(&self, notification: &Notification) -> Result<(), String> {
            self.notifications.lock().unwrap().push(notification.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<Notification>, String> {
            let notifs = self.notifications.lock().unwrap();
            Ok(notifs.iter().find(|n| n.id().as_uuid().to_string() == id).cloned())
        }

        async fn find_pending(&self, limit: usize) -> Result<Vec<Notification>, String> {
            let notifs = self.notifications.lock().unwrap();
            Ok(notifs
                .iter()
                .filter(|n| matches!(n.status(), banko_domain::notification::NotificationStatus::Pending | banko_domain::notification::NotificationStatus::Queued))
                .take(limit)
                .cloned()
                .collect())
        }

        async fn update_status(
            &self,
            id: &str,
            status: NotificationStatus,
        ) -> Result<(), String> {
            let mut notifs = self.notifications.lock().unwrap();
            if let Some(notif) = notifs.iter_mut().find(|n| n.id().as_uuid().to_string() == id) {
                // Note: This is a simplification - in real code you'd use proper mutation
                *notif = banko_domain::notification::Notification::from_raw(
                    notif.id().clone(),
                    notif.customer_id(),
                    *notif.channel(),
                    *notif.notification_type(),
                    notif.template_id().to_string(),
                    notif.variables().clone(),
                    notif.recipient().to_string(),
                    notif.subject().to_string(),
                    notif.body().to_string(),
                    status,
                    notif.retry_count(),
                    notif.max_retries(),
                    *notif.created_at(),
                    notif.sent_at().cloned(),
                    notif.delivered_at().cloned(),
                    notif.error_message().map(|s| s.to_string()),
                );
                Ok(())
            } else {
                Err("Notification not found".to_string())
            }
        }

        async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<Notification>, String> {
            let notifs = self.notifications.lock().unwrap();
            Ok(notifs
                .iter()
                .filter(|n| n.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn find_recent_by_customer_channel_template(
            &self,
            customer_id: Uuid,
            channel: Channel,
            template_id: &str,
            minutes_back: i64,
        ) -> Result<Vec<Notification>, String> {
            let notifs = self.notifications.lock().unwrap();
            let cutoff = Utc::now() - chrono::Duration::minutes(minutes_back);
            Ok(notifs
                .iter()
                .filter(|n| {
                    n.customer_id() == customer_id
                        && *n.channel() == channel
                        && n.template_id() == template_id
                        && n.created_at() >= &cutoff
                })
                .cloned()
                .collect())
        }
    }

    pub struct MockTemplateRepository {
        pub templates: Mutex<Vec<TemplateInfo>>,
    }

    impl MockTemplateRepository {
        pub fn new() -> Self {
            MockTemplateRepository {
                templates: Mutex::new(Vec::new()),
            }
        }

        pub fn add_template(&self, template: TemplateInfo) {
            self.templates.lock().unwrap().push(template);
        }
    }

    #[async_trait]
    impl ITemplateRepository for MockTemplateRepository {
        async fn find_by_id(&self, id: &str) -> Result<Option<TemplateInfo>, String> {
            let templates = self.templates.lock().unwrap();
            Ok(templates.iter().find(|t| t.id == id).cloned())
        }

        async fn find_by_event_and_locale(
            &self,
            event_type: &str,
            locale: &str,
        ) -> Result<Option<TemplateInfo>, String> {
            let templates = self.templates.lock().unwrap();
            // Try exact match first
            if let Some(template) = templates
                .iter()
                .find(|t| t.event_type == event_type && t.locale == locale)
            {
                return Ok(Some(template.clone()));
            }
            // Fallback to default locale (fr)
            if locale != "fr" {
                if let Some(template) = templates
                    .iter()
                    .find(|t| t.event_type == event_type && t.locale == "fr")
                {
                    return Ok(Some(template.clone()));
                }
            }
            Ok(None)
        }

        async fn list_all(&self) -> Result<Vec<TemplateInfo>, String> {
            Ok(self.templates.lock().unwrap().clone())
        }
    }

    pub struct MockNotificationPreferenceRepository {
        pub preferences: Mutex<Vec<NotificationPreference>>,
    }

    impl MockNotificationPreferenceRepository {
        pub fn new() -> Self {
            MockNotificationPreferenceRepository {
                preferences: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl INotificationPreferenceRepository for MockNotificationPreferenceRepository {
        async fn find_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Vec<NotificationPreference>, String> {
            let prefs = self.preferences.lock().unwrap();
            Ok(prefs
                .iter()
                .filter(|p| p.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn save(&self, preference: &NotificationPreference) -> Result<(), String> {
            let mut prefs = self.preferences.lock().unwrap();
            // Remove existing if present
            prefs.retain(|p| {
                p.customer_id() != preference.customer_id()
                    || *p.channel() != *preference.channel()
                    || *p.notification_type() != *preference.notification_type()
            });
            prefs.push(preference.clone());
            Ok(())
        }

        async fn find_by_customer_and_channel_type(
            &self,
            customer_id: Uuid,
            channel: Channel,
            notification_type: NotificationType,
        ) -> Result<Option<NotificationPreference>, String> {
            let prefs = self.preferences.lock().unwrap();
            Ok(prefs
                .iter()
                .find(|p| {
                    p.customer_id() == customer_id
                        && *p.channel() == channel
                        && *p.notification_type() == notification_type
                })
                .cloned())
        }
    }

    pub struct MockNotificationSender {
        pub emails_sent: Mutex<Vec<(String, String, String)>>,
        pub sms_sent: Mutex<Vec<(String, String)>>,
        pub push_sent: Mutex<Vec<(String, String, String)>>,
    }

    impl MockNotificationSender {
        pub fn new() -> Self {
            MockNotificationSender {
                emails_sent: Mutex::new(Vec::new()),
                sms_sent: Mutex::new(Vec::new()),
                push_sent: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl INotificationSender for MockNotificationSender {
        async fn send_email(
            &self,
            to: &str,
            subject: &str,
            body: &str,
        ) -> Result<(), String> {
            self.emails_sent.lock().unwrap().push((
                to.to_string(),
                subject.to_string(),
                body.to_string(),
            ));
            Ok(())
        }

        async fn send_sms(&self, to: &str, message: &str) -> Result<(), String> {
            self.sms_sent
                .lock()
                .unwrap()
                .push((to.to_string(), message.to_string()));
            Ok(())
        }

        async fn send_push(
            &self,
            token: &str,
            title: &str,
            body: &str,
        ) -> Result<(), String> {
            self.push_sent.lock().unwrap().push((
                token.to_string(),
                title.to_string(),
                body.to_string(),
            ));
            Ok(())
        }
    }
}
