use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::web::middleware::AuthenticatedUser;

/// Error response structure
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/// Notification request for sending a notification
#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub customer_id: String,
    pub channel: String,          // Email, Sms, Push
    pub notification_type: String, // Transactional, Security, Regulatory, Marketing
    pub template_id: String,
    pub variables: Option<serde_json::Value>,
    pub recipient: String,
}

/// Notification response
#[derive(Debug, Serialize)]
struct NotificationResponse {
    id: String,
    customer_id: String,
    channel: String,
    notification_type: String,
    template_id: String,
    recipient: String,
    status: String,
    created_at: String,
}

/// List notifications response
#[derive(Debug, Serialize)]
struct NotificationListResponse {
    notifications: Vec<NotificationResponse>,
    total: i64,
}

/// Notification preference update request
#[derive(Debug, Deserialize)]
pub struct UpdatePreferenceRequest {
    pub customer_id: String,
    pub channel: String,          // Email, Sms, Push
    pub notification_type: String, // Transactional, Security, Regulatory, Marketing
    pub opted_in: bool,
}

/// Notification preference response
#[derive(Debug, Serialize)]
struct PreferenceResponse {
    id: String,
    customer_id: String,
    channel: String,
    notification_type: String,
    opted_in: bool,
    updated_at: String,
}

/// Notification template response
#[derive(Debug, Serialize)]
struct TemplateResponse {
    id: String,
    event_type: String,
    channel: String,
    locale: String,
    subject_template: Option<String>,
    body_template: String,
    active: bool,
}

fn map_uuid_error(err: uuid::Error) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse {
        error: format!("Invalid UUID: {}", err),
    })
}

/// POST /api/v1/notifications/send
/// Sends a notification via the specified channel
pub async fn send_notification_handler(
    _auth_user: AuthenticatedUser,
    body: web::Json<SendNotificationRequest>,
) -> HttpResponse {
    // Validate customer_id is a valid UUID
    if let Err(err) = Uuid::parse_str(&body.customer_id) {
        return map_uuid_error(err);
    }

    // Validate channel
    match body.channel.as_str() {
        "Email" | "Sms" | "Push" => {}
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid channel. Must be Email, Sms, or Push".to_string(),
            })
        }
    }

    // Validate notification_type
    match body.notification_type.as_str() {
        "Transactional" | "Security" | "Regulatory" | "Marketing" => {}
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid notification_type. Must be Transactional, Security, Regulatory, or Marketing"
                    .to_string(),
            })
        }
    }

    // Validate recipient is not empty
    if body.recipient.trim().is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Recipient cannot be empty".to_string(),
        });
    }

    // In a real implementation, this would:
    // 1. Create a Notification entity
    // 2. Save it to the database with status = 'Pending'
    // 3. Return the created notification
    //
    // For now, return a mock response
    let notification_id = Uuid::new_v4().to_string();

    let response = NotificationResponse {
        id: notification_id,
        customer_id: body.customer_id.clone(),
        channel: body.channel.clone(),
        notification_type: body.notification_type.clone(),
        template_id: body.template_id.clone(),
        recipient: body.recipient.clone(),
        status: "Pending".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    HttpResponse::Created().json(response)
}

/// GET /api/v1/notifications?customer_id=X
/// Lists all notifications for a customer
pub async fn list_notifications_handler(
    _auth_user: AuthenticatedUser,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let _customer_id = match query.get("customer_id") {
        Some(id) => {
            if let Err(err) = Uuid::parse_str(id) {
                return map_uuid_error(err);
            }
            id.clone()
        }
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "customer_id query parameter is required".to_string(),
            })
        }
    };

    // In a real implementation, this would:
    // 1. Query the database for notifications with matching customer_id
    // 2. Return the list with pagination support
    //
    // For now, return an empty list

    let response = NotificationListResponse {
        notifications: vec![],
        total: 0,
    };

    HttpResponse::Ok().json(response)
}

/// GET /api/v1/notifications/{id}
/// Gets a single notification by ID
pub async fn get_notification_handler(
    _auth_user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let _notification_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(err) => return map_uuid_error(err),
    };

    // In a real implementation, this would:
    // 1. Query the database for the notification with matching ID
    // 2. Return it if found, or 404 if not
    //
    // For now, return a 404

    HttpResponse::NotFound().json(ErrorResponse {
        error: "Notification not found".to_string(),
    })
}

/// GET /api/v1/notifications/preferences/{customer_id}
/// Gets notification preferences for a customer
pub async fn get_preferences_handler(
    _auth_user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let _customer_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(err) => return map_uuid_error(err),
    };

    // In a real implementation, this would:
    // 1. Query the database for preferences with matching customer_id
    // 2. Return the list of preferences
    //
    // For now, return an empty list

    let responses: Vec<PreferenceResponse> = vec![];
    HttpResponse::Ok().json(responses)
}

/// PUT /api/v1/notifications/preferences
/// Updates a notification preference
pub async fn update_preference_handler(
    _auth_user: AuthenticatedUser,
    body: web::Json<UpdatePreferenceRequest>,
) -> HttpResponse {
    // Validate customer_id is a valid UUID
    if let Err(err) = Uuid::parse_str(&body.customer_id) {
        return map_uuid_error(err);
    }

    // Validate channel
    match body.channel.as_str() {
        "Email" | "Sms" | "Push" => {}
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid channel. Must be Email, Sms, or Push".to_string(),
            })
        }
    }

    // Validate notification_type
    match body.notification_type.as_str() {
        "Transactional" | "Security" | "Regulatory" | "Marketing" => {}
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid notification_type. Must be Transactional, Security, Regulatory, or Marketing"
                    .to_string(),
            })
        }
    }

    // In a real implementation, this would:
    // 1. Create or update a notification preference
    // 2. Return the updated preference
    //
    // For now, return a mock response

    let preference_id = Uuid::new_v4().to_string();

    let response = PreferenceResponse {
        id: preference_id,
        customer_id: body.customer_id.clone(),
        channel: body.channel.clone(),
        notification_type: body.notification_type.clone(),
        opted_in: body.opted_in,
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    HttpResponse::Ok().json(response)
}

/// GET /api/v1/notifications/templates
/// Lists all notification templates
pub async fn list_templates_handler(_auth_user: AuthenticatedUser) -> HttpResponse {
    // In a real implementation, this would:
    // 1. Query the database for all active notification templates
    // 2. Return the list, optionally filtered by event_type or channel
    //
    // For now, return an empty list

    let responses: Vec<TemplateResponse> = vec![];
    HttpResponse::Ok().json(responses)
}
