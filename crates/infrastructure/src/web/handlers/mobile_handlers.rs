use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_application::identity::{DeviceRegistration, MobileAuthError, MobileAuthService, MobilePlatform, MobileSession};
use banko_application::account::MobileAccountService;
use banko_application::payment::{MobilePaymentService, QuickTransferRequest};

use crate::web::middleware::AuthenticatedUser;

// ============================================================
// Mobile Auth DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub device_id: String,
    pub device_name: String,
    pub platform: String,
}

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: String,
    pub device_id: String,
    pub device_name: String,
    pub platform: String,
    pub biometric_enabled: bool,
    pub registered_at: String,
    pub last_active_at: String,
    pub is_active: bool,
}

impl From<DeviceRegistration> for DeviceResponse {
    fn from(device: DeviceRegistration) -> Self {
        DeviceResponse {
            id: device.id.to_string(),
            device_id: device.device_id,
            device_name: device.device_name,
            platform: device.platform.as_str().to_string(),
            biometric_enabled: device.biometric_enabled,
            registered_at: device.registered_at.to_rfc3339(),
            last_active_at: device.last_active_at.to_rfc3339(),
            is_active: device.is_active,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeviceListResponse {
    pub data: Vec<DeviceResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct MobileLoginRequest {
    pub device_id: String,
    pub pin_or_biometric: String,
}

#[derive(Debug, Serialize)]
pub struct MobileSessionResponse {
    pub session_id: String,
    pub token: String,
    pub refresh_token: String,
    pub expires_at: String,
}

impl From<MobileSession> for MobileSessionResponse {
    fn from(session: MobileSession) -> Self {
        MobileSessionResponse {
            session_id: session.session_id.to_string(),
            token: session.token,
            refresh_token: session.refresh_token,
            expires_at: session.expires_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RefreshSessionRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct EnableBiometricRequest {
    pub biometric_data_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPinRequest {
    pub pin: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ============================================================
// Mobile Auth Handlers
// ============================================================

pub async fn register_device_handler(
    service: web::Data<Arc<MobileAuthService>>,
    user: AuthenticatedUser,
    body: web::Json<RegisterDeviceRequest>,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    let platform = match MobilePlatform::parse(&body.platform) {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid platform. Must be 'Ios' or 'Android'".to_string(),
        }),
    };

    match service
        .register_device(customer_id, body.device_id.clone(), body.device_name.clone(), platform)
        .await
    {
        Ok(device) => HttpResponse::Created().json(DeviceResponse::from(device)),
        Err(MobileAuthError::DeviceLimitExceeded) => HttpResponse::Conflict().json(ErrorResponse {
            error: "Device limit exceeded (max 5 devices)".to_string(),
        }),
        Err(MobileAuthError::Internal(msg)) => {
            tracing::error!("Register device error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn list_devices_handler(
    service: web::Data<Arc<MobileAuthService>>,
    user: AuthenticatedUser,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    match service.list_devices(customer_id).await {
        Ok(devices) => {
            let total = devices.len();
            let data: Vec<DeviceResponse> = devices.into_iter().map(DeviceResponse::from).collect();
            HttpResponse::Ok().json(DeviceListResponse { data, total })
        }
        Err(MobileAuthError::Internal(msg)) => {
            tracing::error!("List devices error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn login_mobile_handler(
    service: web::Data<Arc<MobileAuthService>>,
    body: web::Json<MobileLoginRequest>,
) -> HttpResponse {
    match service
        .login_mobile(&body.device_id, &body.pin_or_biometric)
        .await
    {
        Ok(session) => HttpResponse::Ok().json(MobileSessionResponse::from(session)),
        Err(MobileAuthError::DeviceNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Device not found".to_string(),
        }),
        Err(MobileAuthError::InvalidPin) => HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid PIN".to_string(),
        }),
        Err(MobileAuthError::InvalidBiometric) => HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid biometric token".to_string(),
        }),
        Err(MobileAuthError::DeviceNotActive) => HttpResponse::Forbidden().json(ErrorResponse {
            error: "Device is not active".to_string(),
        }),
        Err(MobileAuthError::Internal(msg)) => {
            tracing::error!("Mobile login error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn refresh_session_handler(
    service: web::Data<Arc<MobileAuthService>>,
    body: web::Json<RefreshSessionRequest>,
) -> HttpResponse {
    match service.refresh_session(&body.refresh_token).await {
        Ok(session) => HttpResponse::Ok().json(MobileSessionResponse::from(session)),
        Err(MobileAuthError::InvalidRefreshToken) => HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid refresh token".to_string(),
        }),
        Err(MobileAuthError::SessionExpired) => HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Refresh token expired".to_string(),
        }),
        Err(MobileAuthError::Internal(msg)) => {
            tracing::error!("Refresh session error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn enable_biometric_handler(
    service: web::Data<Arc<MobileAuthService>>,
    path: web::Path<String>,
    body: web::Json<EnableBiometricRequest>,
) -> HttpResponse {
    let device_id = path.into_inner();

    match service
        .enable_biometric(&device_id, &body.biometric_data_hash)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "Biometric enabled" })),
        Err(MobileAuthError::DeviceNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Device not found".to_string(),
        }),
        Err(MobileAuthError::Internal(msg)) => {
            tracing::error!("Enable biometric error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn set_pin_handler(
    service: web::Data<Arc<MobileAuthService>>,
    path: web::Path<String>,
    body: web::Json<SetPinRequest>,
) -> HttpResponse {
    let device_id = path.into_inner();

    match service.set_pin(&device_id, &body.pin).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "PIN set successfully" })),
        Err(MobileAuthError::DeviceNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Device not found".to_string(),
        }),
        Err(MobileAuthError::Internal(msg)) => HttpResponse::BadRequest().json(ErrorResponse {
            error: msg,
        }),
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn deactivate_device_handler(
    service: web::Data<Arc<MobileAuthService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let device_id = path.into_inner();

    match service.deactivate_device(&device_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "Device deactivated" })),
        Err(MobileAuthError::DeviceNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Device not found".to_string(),
        }),
        Err(MobileAuthError::Internal(msg)) => {
            tracing::error!("Deactivate device error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
        _ => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

// ============================================================
// Mobile Account Handlers
// ============================================================

#[derive(Debug, Deserialize)]
pub struct GetDashboardRequest {
    pub locale: Option<String>,
}

pub async fn get_dashboard_handler(
    service: web::Data<Arc<MobileAccountService>>,
    user: AuthenticatedUser,
    query: web::Query<GetDashboardRequest>,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    let locale = query.locale.as_deref().unwrap_or("en");

    match service.get_mobile_dashboard(customer_id, locale).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(e) => {
            tracing::error!("Get dashboard error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

pub async fn get_offline_data_handler(
    service: web::Data<Arc<MobileAccountService>>,
    user: AuthenticatedUser,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    match service.get_offline_cache_data(customer_id).await {
        Ok(cache) => HttpResponse::Ok().json(cache),
        Err(e) => {
            tracing::error!("Get offline data error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SyncRequest {
    pub last_sync: String,
}

pub async fn sync_handler(
    service: web::Data<Arc<MobileAccountService>>,
    user: AuthenticatedUser,
    body: web::Json<SyncRequest>,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    let last_sync = match body.last_sync.parse::<chrono::DateTime<chrono::Utc>>() {
        Ok(dt) => dt,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid last_sync timestamp".to_string(),
        }),
    };

    match service.sync_changes(customer_id, last_sync).await {
        Ok(sync_response) => HttpResponse::Ok().json(sync_response),
        Err(e) => {
            tracing::error!("Sync error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

// ============================================================
// Mobile Payment Handlers
// ============================================================

#[derive(Debug, Deserialize)]
pub struct QuickTransferRequestDto {
    pub from_account_id: String,
    pub to_iban_or_phone: String,
    pub amount: String,
    pub currency: String,
    pub note: Option<String>,
}

pub async fn quick_transfer_handler(
    service: web::Data<Arc<MobilePaymentService>>,
    user: AuthenticatedUser,
    body: web::Json<QuickTransferRequestDto>,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    let from_account_id = match Uuid::parse_str(&body.from_account_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid account ID".to_string(),
        }),
    };

    let amount = match body.amount.parse::<rust_decimal::Decimal>() {
        Ok(a) => a,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid amount".to_string(),
        }),
    };

    let req = QuickTransferRequest {
        from_account_id,
        to_iban_or_phone: body.to_iban_or_phone.clone(),
        amount,
        currency: body.currency.clone(),
        note: body.note.clone(),
    };

    match service.quick_transfer(customer_id, req).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Quick transfer error: {:?}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn frequent_beneficiaries_handler(
    service: web::Data<Arc<MobilePaymentService>>,
    user: AuthenticatedUser,
) -> HttpResponse {
    let customer_id = match Uuid::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID".to_string(),
        }),
    };

    match service.get_frequent_beneficiaries(customer_id).await {
        Ok(beneficiaries) => {
            let response = serde_json::json!({
                "data": beneficiaries,
                "total": beneficiaries.len()
            });
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            tracing::error!("Get beneficiaries error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ScanQrRequest {
    pub qr_data: String,
}

pub async fn scan_qr_handler(
    service: web::Data<Arc<MobilePaymentService>>,
    _user: AuthenticatedUser,
    body: web::Json<ScanQrRequest>,
) -> HttpResponse {
    match service.scan_qr_payment(body.qr_data.clone()).await {
        Ok(qr_info) => HttpResponse::Ok().json(qr_info),
        Err(e) => {
            tracing::warn!("QR scan error: {:?}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}
