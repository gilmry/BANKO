use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Newtypes for Notification IDs
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotificationId(Uuid);

impl NotificationId {
    pub fn new() -> Self {
        NotificationId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        NotificationId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NotificationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NotificationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotificationPreferenceId(Uuid);

impl NotificationPreferenceId {
    pub fn new() -> Self {
        NotificationPreferenceId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        NotificationPreferenceId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NotificationPreferenceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NotificationPreferenceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// Notification Channel Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    Email,
    Sms,
    Push,
}

impl Channel {
    pub fn as_str(&self) -> &str {
        match self {
            Channel::Email => "Email",
            Channel::Sms => "Sms",
            Channel::Push => "Push",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Email" => Ok(Channel::Email),
            "Sms" => Ok(Channel::Sms),
            "Push" => Ok(Channel::Push),
            _ => Err(DomainError::InvalidNotificationChannel(format!(
                "Unknown channel: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// Notification Status Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Queued,
    Sending,
    Sent,
    Delivered,
    Failed,
    Retrying,
}

impl NotificationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            NotificationStatus::Pending => "Pending",
            NotificationStatus::Queued => "Queued",
            NotificationStatus::Sending => "Sending",
            NotificationStatus::Sent => "Sent",
            NotificationStatus::Delivered => "Delivered",
            NotificationStatus::Failed => "Failed",
            NotificationStatus::Retrying => "Retrying",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Pending" => Ok(NotificationStatus::Pending),
            "Queued" => Ok(NotificationStatus::Queued),
            "Sending" => Ok(NotificationStatus::Sending),
            "Sent" => Ok(NotificationStatus::Sent),
            "Delivered" => Ok(NotificationStatus::Delivered),
            "Failed" => Ok(NotificationStatus::Failed),
            "Retrying" => Ok(NotificationStatus::Retrying),
            _ => Err(DomainError::InvalidNotificationStatus(format!(
                "Unknown status: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// Notification Type Enum
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    Transactional,
    Security,
    Regulatory,
    Marketing,
}

impl NotificationType {
    pub fn as_str(&self) -> &str {
        match self {
            NotificationType::Transactional => "Transactional",
            NotificationType::Security => "Security",
            NotificationType::Regulatory => "Regulatory",
            NotificationType::Marketing => "Marketing",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "Transactional" => Ok(NotificationType::Transactional),
            "Security" => Ok(NotificationType::Security),
            "Regulatory" => Ok(NotificationType::Regulatory),
            "Marketing" => Ok(NotificationType::Marketing),
            _ => Err(DomainError::InvalidNotificationType(format!(
                "Unknown notification type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// Notification Aggregate
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    id: NotificationId,
    customer_id: Uuid,
    channel: Channel,
    notification_type: NotificationType,
    template_id: String,
    variables: HashMap<String, String>,
    recipient: String,
    subject: String,
    body: String,
    status: NotificationStatus,
    retry_count: i32,
    max_retries: i32,
    created_at: DateTime<Utc>,
    sent_at: Option<DateTime<Utc>>,
    delivered_at: Option<DateTime<Utc>>,
    error_message: Option<String>,
}

impl Notification {
    /// Create a new Notification with validation
    pub fn new(
        customer_id: Uuid,
        channel: Channel,
        notification_type: NotificationType,
        template_id: String,
        variables: HashMap<String, String>,
        recipient: String,
        subject: String,
        body: String,
    ) -> Result<Self, DomainError> {
        if recipient.trim().is_empty() {
            return Err(DomainError::InvalidNotificationRecipient(
                "Recipient must not be empty".to_string(),
            ));
        }

        if template_id.trim().is_empty() {
            return Err(DomainError::InvalidNotificationTemplate(
                "Template ID must not be empty".to_string(),
            ));
        }

        if subject.trim().is_empty() {
            return Err(DomainError::InvalidNotificationSubject(
                "Subject must not be empty".to_string(),
            ));
        }

        if body.trim().is_empty() {
            return Err(DomainError::InvalidNotificationBody(
                "Body must not be empty".to_string(),
            ));
        }

        Ok(Notification {
            id: NotificationId::new(),
            customer_id,
            channel,
            notification_type,
            template_id,
            variables,
            recipient,
            subject,
            body,
            status: NotificationStatus::Pending,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
            sent_at: None,
            delivered_at: None,
            error_message: None,
        })
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        id: NotificationId,
        customer_id: Uuid,
        channel: Channel,
        notification_type: NotificationType,
        template_id: String,
        variables: HashMap<String, String>,
        recipient: String,
        subject: String,
        body: String,
        status: NotificationStatus,
        retry_count: i32,
        max_retries: i32,
        created_at: DateTime<Utc>,
        sent_at: Option<DateTime<Utc>>,
        delivered_at: Option<DateTime<Utc>>,
        error_message: Option<String>,
    ) -> Self {
        Notification {
            id,
            customer_id,
            channel,
            notification_type,
            template_id,
            variables,
            recipient,
            subject,
            body,
            status,
            retry_count,
            max_retries,
            created_at,
            sent_at,
            delivered_at,
            error_message,
        }
    }

    /// Mark notification as queued
    pub fn mark_queued(&mut self) {
        self.status = NotificationStatus::Queued;
    }

    /// Mark notification as sending
    pub fn mark_sending(&mut self) {
        self.status = NotificationStatus::Sending;
    }

    /// Mark notification as sent
    pub fn mark_sent(&mut self) {
        self.status = NotificationStatus::Sent;
        self.sent_at = Some(Utc::now());
        self.error_message = None;
    }

    /// Mark notification as delivered
    pub fn mark_delivered(&mut self) {
        self.status = NotificationStatus::Delivered;
        self.delivered_at = Some(Utc::now());
        self.error_message = None;
    }

    /// Mark notification as failed with error reason
    pub fn mark_failed(&mut self, reason: String) {
        self.status = NotificationStatus::Failed;
        self.error_message = Some(reason);
    }

    /// Check if notification should be retried
    pub fn should_retry(&self) -> bool {
        self.retry_count < self.max_retries && self.status == NotificationStatus::Failed
    }

    /// Increment retry count and update status to Retrying
    pub fn increment_retry(&mut self) {
        if self.should_retry() {
            self.retry_count += 1;
            self.status = NotificationStatus::Retrying;
            self.error_message = None;
        }
    }

    /// Render template by replacing {{var}} patterns with values from variables map
    pub fn render_template(template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// Check if this notification is within dedup window of another
    pub fn is_dedup_window(&self, other: &Notification, window_secs: i64) -> bool {
        if self.customer_id != other.customer_id {
            return false;
        }

        if self.channel != other.channel {
            return false;
        }

        if self.template_id != other.template_id {
            return false;
        }

        let duration = chrono::Duration::seconds(window_secs);
        let time_diff = self.created_at.signed_duration_since(other.created_at);

        time_diff.abs() <= duration
    }

    // --- Getters ---
    pub fn id(&self) -> &NotificationId {
        &self.id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn notification_type(&self) -> &NotificationType {
        &self.notification_type
    }

    pub fn template_id(&self) -> &str {
        &self.template_id
    }

    pub fn variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    pub fn recipient(&self) -> &str {
        &self.recipient
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn status(&self) -> &NotificationStatus {
        &self.status
    }

    pub fn retry_count(&self) -> i32 {
        self.retry_count
    }

    pub fn max_retries(&self) -> i32 {
        self.max_retries
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn sent_at(&self) -> Option<&DateTime<Utc>> {
        self.sent_at.as_ref()
    }

    pub fn delivered_at(&self) -> Option<&DateTime<Utc>> {
        self.delivered_at.as_ref()
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

// ============================================================
// NotificationPreference Aggregate
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationPreference {
    id: NotificationPreferenceId,
    customer_id: Uuid,
    channel: Channel,
    notification_type: NotificationType,
    opted_in: bool,
    updated_at: DateTime<Utc>,
}

impl NotificationPreference {
    /// Create a new NotificationPreference
    pub fn new(
        customer_id: Uuid,
        channel: Channel,
        notification_type: NotificationType,
        opted_in: bool,
    ) -> Self {
        NotificationPreference {
            id: NotificationPreferenceId::new(),
            customer_id,
            channel,
            notification_type,
            opted_in,
            updated_at: Utc::now(),
        }
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        id: NotificationPreferenceId,
        customer_id: Uuid,
        channel: Channel,
        notification_type: NotificationType,
        opted_in: bool,
        updated_at: DateTime<Utc>,
    ) -> Self {
        NotificationPreference {
            id,
            customer_id,
            channel,
            notification_type,
            opted_in,
            updated_at,
        }
    }

    /// Update the opt-in status
    pub fn set_opted_in(&mut self, opted_in: bool) {
        self.opted_in = opted_in;
        self.updated_at = Utc::now();
    }

    /// Check if notification should be allowed based on type and opt-in
    pub fn allows_notification(&self, notif_type: NotificationType) -> bool {
        // Regulatory and Security notifications are always allowed
        match notif_type {
            NotificationType::Regulatory | NotificationType::Security => true,
            NotificationType::Marketing => self.opted_in,
            NotificationType::Transactional => true,
        }
    }

    // --- Getters ---
    pub fn id(&self) -> &NotificationPreferenceId {
        &self.id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn notification_type(&self) -> &NotificationType {
        &self.notification_type
    }

    pub fn opted_in(&self) -> bool {
        self.opted_in
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let customer_id = Uuid::new_v4();
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());

        let result = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            variables.clone(),
            "john@example.com".to_string(),
            "Welcome".to_string(),
            "Welcome to BANKO".to_string(),
        );

        assert!(result.is_ok());
        let notif = result.unwrap();
        assert_eq!(notif.customer_id(), customer_id);
        assert_eq!(notif.channel(), &Channel::Email);
        assert_eq!(notif.status(), &NotificationStatus::Pending);
        assert_eq!(notif.retry_count(), 0);
        assert_eq!(notif.max_retries(), 3);
    }

    #[test]
    fn test_notification_validation_empty_recipient() {
        let result = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_notification_validation_empty_template_id() {
        let result = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_notification_validation_empty_subject() {
        let result = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "".to_string(),
            "Body".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_notification_validation_empty_body() {
        let result = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_notification_mark_queued() {
        let mut notif = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        notif.mark_queued();
        assert_eq!(notif.status(), &NotificationStatus::Queued);
    }

    #[test]
    fn test_notification_mark_sent() {
        let mut notif = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        notif.mark_sent();
        assert_eq!(notif.status(), &NotificationStatus::Sent);
        assert!(notif.sent_at().is_some());
        assert!(notif.error_message().is_none());
    }

    #[test]
    fn test_notification_mark_delivered() {
        let mut notif = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        notif.mark_delivered();
        assert_eq!(notif.status(), &NotificationStatus::Delivered);
        assert!(notif.delivered_at().is_some());
    }

    #[test]
    fn test_notification_mark_failed() {
        let mut notif = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        notif.mark_failed("SMTP error".to_string());
        assert_eq!(notif.status(), &NotificationStatus::Failed);
        assert_eq!(notif.error_message(), Some("SMTP error"));
    }

    #[test]
    fn test_notification_should_retry() {
        let mut notif = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        // Should not retry when pending
        assert!(!notif.should_retry());

        notif.mark_failed("Error".to_string());
        assert!(notif.should_retry());

        // Exhaust retries
        notif.retry_count = 3;
        assert!(!notif.should_retry());
    }

    #[test]
    fn test_notification_increment_retry() {
        let mut notif = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "test".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        notif.mark_failed("Error".to_string());
        notif.increment_retry();

        assert_eq!(notif.retry_count(), 1);
        assert_eq!(notif.status(), &NotificationStatus::Retrying);
    }

    #[test]
    fn test_render_template() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());
        variables.insert("amount".to_string(), "1000 MAD".to_string());

        let template = "Hello {{name}}, you received {{amount}}";
        let rendered = Notification::render_template(template, &variables);

        assert_eq!(rendered, "Hello Alice, you received 1000 MAD");
    }

    #[test]
    fn test_render_template_no_vars() {
        let template = "No variables here";
        let rendered = Notification::render_template(template, &HashMap::new());
        assert_eq!(rendered, template);
    }

    #[test]
    fn test_is_dedup_window_true() {
        let customer_id = Uuid::new_v4();
        let notif1 = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        let notif2 = Notification::new(
            customer_id,
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        assert!(notif1.is_dedup_window(&notif2, 300));
    }

    #[test]
    fn test_is_dedup_window_false_different_customer() {
        let notif1 = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        let notif2 = Notification::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            "welcome".to_string(),
            HashMap::new(),
            "test@example.com".to_string(),
            "Subject".to_string(),
            "Body".to_string(),
        )
        .unwrap();

        assert!(!notif1.is_dedup_window(&notif2, 300));
    }

    #[test]
    fn test_notification_preference_creation() {
        let customer_id = Uuid::new_v4();
        let pref = NotificationPreference::new(
            customer_id,
            Channel::Email,
            NotificationType::Marketing,
            true,
        );

        assert_eq!(pref.customer_id(), customer_id);
        assert_eq!(pref.channel(), &Channel::Email);
        assert_eq!(pref.notification_type(), &NotificationType::Marketing);
        assert!(pref.opted_in());
    }

    #[test]
    fn test_notification_preference_allows_regulatory() {
        let pref = NotificationPreference::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Regulatory,
            false, // Not opted in
        );

        assert!(pref.allows_notification(NotificationType::Regulatory));
    }

    #[test]
    fn test_notification_preference_allows_security() {
        let pref = NotificationPreference::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Security,
            false,
        );

        assert!(pref.allows_notification(NotificationType::Security));
    }

    #[test]
    fn test_notification_preference_rejects_marketing_when_not_opted() {
        let pref = NotificationPreference::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Marketing,
            false,
        );

        assert!(!pref.allows_notification(NotificationType::Marketing));
    }

    #[test]
    fn test_notification_preference_allows_marketing_when_opted() {
        let pref = NotificationPreference::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Marketing,
            true,
        );

        assert!(pref.allows_notification(NotificationType::Marketing));
    }

    #[test]
    fn test_notification_preference_allows_transactional() {
        let pref = NotificationPreference::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Transactional,
            false,
        );

        assert!(pref.allows_notification(NotificationType::Transactional));
    }

    #[test]
    fn test_notification_preference_set_opted_in() {
        let mut pref = NotificationPreference::new(
            Uuid::new_v4(),
            Channel::Email,
            NotificationType::Marketing,
            false,
        );

        pref.set_opted_in(true);
        assert!(pref.opted_in());
    }
}
