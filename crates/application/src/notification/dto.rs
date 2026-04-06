use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================
// Send Notification Request/Response
// ============================================================

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub customer_id: Uuid,
    pub channel: String,                        // "Email", "Sms", "Push"
    pub notification_type: String,              // "Transactional", "Security", "Regulatory", "Marketing"
    pub template_id: String,
    pub variables: Option<HashMap<String, String>>,
    pub recipient: String,
    pub locale: Option<String>, // Default "fr"
}

#[derive(Debug, Serialize)]
pub struct SendNotificationResponse {
    pub notification_id: String,
    pub status: String,
}

// ============================================================
// Notification Preference Request/Response
// ============================================================

#[derive(Debug, Deserialize)]
pub struct NotificationPreferenceRequest {
    pub customer_id: Uuid,
    pub channel: String,             // "Email", "Sms", "Push"
    pub notification_type: String,   // "Transactional", "Security", "Regulatory", "Marketing"
    pub opted_in: bool,
}

#[derive(Debug, Serialize)]
pub struct PreferenceItem {
    pub id: String,
    pub customer_id: String,
    pub channel: String,
    pub notification_type: String,
    pub opted_in: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NotificationPreferenceResponse {
    pub preferences: Vec<PreferenceItem>,
}

// ============================================================
// Template Info DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfoDto {
    pub id: String,
    pub event_type: String,
    pub channel: String,
    pub locale: String,
    pub subject_template: String,
    pub body_template: String,
}

// ============================================================
// Notification View DTO
// ============================================================

#[derive(Debug, Serialize)]
pub struct NotificationDto {
    pub id: String,
    pub customer_id: String,
    pub channel: String,
    pub notification_type: String,
    pub template_id: String,
    pub variables: HashMap<String, String>,
    pub recipient: String,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub created_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

// ============================================================
// Processing Result DTO
// ============================================================

#[derive(Debug, Serialize)]
pub struct ProcessingResult {
    pub total_processed: usize,
    pub sent: usize,
    pub failed: usize,
    pub skipped: usize,
    pub errors: Vec<ProcessingError>,
}

#[derive(Debug, Serialize)]
pub struct ProcessingError {
    pub notification_id: String,
    pub reason: String,
}

// ============================================================
// Batch Processing DTO
// ============================================================

#[derive(Debug, Serialize)]
pub struct BatchNotificationRequest {
    pub customer_ids: Vec<Uuid>,
    pub channel: String,
    pub notification_type: String,
    pub template_id: String,
    pub variables: Option<HashMap<String, String>>,
    pub locale: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BatchNotificationResponse {
    pub total: usize,
    pub sent: usize,
    pub failed: usize,
}
