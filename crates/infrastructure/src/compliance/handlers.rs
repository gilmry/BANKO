use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use banko_application::compliance::{
    BiometricVerificationDto, DataPortabilityRequest, ErasureRequest, IBiometricRepository,
    IBreachNotificationRepository, IDataPortabilityRepository, IDpiaRepository, IErasureRepository,
    IInpdpConsentRepository, ISmsiRepository, ITokenVaultRepository, RiskEntry, SmsiControl,
    TokenVaultEntry,
};

// ============================================================
// Request/Response DTOs
// ============================================================

#[derive(Debug, Deserialize)]
pub struct SaveSmsiControlRequest {
    pub control_code: String,
    pub name: String,
    pub theme: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct SmsiControlResponse {
    pub id: String,
    pub control_code: String,
    pub name: String,
    pub theme: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SaveRiskRequest {
    pub identifier: String,
    pub description: String,
    pub score: i32,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct RiskEntryResponse {
    pub id: String,
    pub identifier: String,
    pub description: String,
    pub score: i32,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SaveTokenRequest {
    pub token: String,
    pub masked_pan: String,
}

#[derive(Debug, Serialize)]
pub struct TokenVaultResponse {
    pub id: String,
    pub token: String,
    pub masked_pan: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GrantConsentRequest {
    pub customer_id: String,
    pub purpose: String,
    pub legal_basis: String,
    pub data_categories: Vec<String>,
    pub expiry_days: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ConsentResponse {
    pub id: String,
    pub customer_id: String,
    pub purpose: String,
    pub legal_basis: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SaveDpiaRequest {
    pub title: String,
    pub description: Option<String>,
    pub processing_activity: String,
    pub risk_assessment: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct DpiaResponse {
    pub id: String,
    pub title: String,
    pub processing_activity: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ReportBreachRequest {
    pub breach_type: String,
    pub description: String,
    pub affected_data: Vec<String>,
    pub affected_count: i32,
}

#[derive(Debug, Serialize)]
pub struct BreachResponse {
    pub id: String,
    pub breach_type: String,
    pub description: String,
    pub affected_count: i32,
    pub status: String,
    pub detected_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestDataPortabilityRequest {
    pub customer_id: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DataPortabilityResponse {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub requested_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestErasureRequest {
    pub customer_id: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErasureResponse {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub requested_at: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyBiometricRequest {
    pub customer_id: String,
    pub verification_type: String,
    pub confidence_score: f64,
    pub liveness_check: bool,
    pub document_type: Option<String>,
    pub document_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BiometricVerificationResponse {
    pub id: String,
    pub customer_id: String,
    pub verification_type: String,
    pub status: String,
    pub confidence_score: f64,
    pub verified_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub total_controls: i64,
    pub implemented_controls: i64,
    pub high_risks: i64,
    pub pending_breaches: i64,
    pub active_consents: i64,
    pub data_requests_pending: i64,
}

// ============================================================
// SMSI Controls Handlers
// ============================================================

pub async fn save_smsi_control(
    req: web::Json<SaveSmsiControlRequest>,
    repo: web::Data<Arc<dyn ISmsiRepository>>,
) -> impl Responder {
    let control = SmsiControl {
        id: Uuid::new_v4(),
        control_code: req.control_code.clone(),
        name: req.name.clone(),
        theme: req.theme.clone(),
        description: req.description.clone(),
        status: req.status.clone(),
        evidence: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match repo.save_control(&control).await {
        Ok(_) => {
            let response = SmsiControlResponse {
                id: control.id.to_string(),
                control_code: control.control_code,
                name: control.name,
                theme: control.theme,
                description: control.description,
                status: control.status,
                created_at: control.created_at.to_rfc3339(),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

pub async fn get_smsi_controls(
    repo: web::Data<Arc<dyn ISmsiRepository>>,
) -> impl Responder {
    match repo.list_all_controls().await {
        Ok(controls) => {
            let response: Vec<SmsiControlResponse> = controls
                .into_iter()
                .map(|c| SmsiControlResponse {
                    id: c.id.to_string(),
                    control_code: c.control_code,
                    name: c.name,
                    theme: c.theme,
                    description: c.description,
                    status: c.status,
                    created_at: c.created_at.to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

pub async fn get_smsi_control(
    path: web::Path<String>,
    repo: web::Data<Arc<dyn ISmsiRepository>>,
) -> impl Responder {
    let control_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid control ID format".to_string(),
        }),
    };

    match repo.find_control_by_id(control_id).await {
        Ok(Some(control)) => {
            let response = SmsiControlResponse {
                id: control.id.to_string(),
                control_code: control.control_code,
                name: control.name,
                theme: control.theme,
                description: control.description,
                status: control.status,
                created_at: control.created_at.to_rfc3339(),
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Control not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Risk Register Handlers
// ============================================================

pub async fn save_risk(
    req: web::Json<SaveRiskRequest>,
    repo: web::Data<Arc<dyn ISmsiRepository>>,
) -> impl Responder {
    let risk = RiskEntry {
        id: Uuid::new_v4(),
        identifier: req.identifier.clone(),
        description: req.description.clone(),
        score: req.score,
        status: req.status.clone(),
        mitigations: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match repo.save_risk(&risk).await {
        Ok(_) => {
            let response = RiskEntryResponse {
                id: risk.id.to_string(),
                identifier: risk.identifier,
                description: risk.description,
                score: risk.score,
                status: risk.status,
                created_at: risk.created_at.to_rfc3339(),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

pub async fn list_risks(
    repo: web::Data<Arc<dyn ISmsiRepository>>,
) -> impl Responder {
    match repo.list_all_risks().await {
        Ok(risks) => {
            let response: Vec<RiskEntryResponse> = risks
                .into_iter()
                .map(|r| RiskEntryResponse {
                    id: r.id.to_string(),
                    identifier: r.identifier,
                    description: r.description,
                    score: r.score,
                    status: r.status,
                    created_at: r.created_at.to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

pub async fn list_high_risks(
    repo: web::Data<Arc<dyn ISmsiRepository>>,
) -> impl Responder {
    match repo.list_high_risks().await {
        Ok(risks) => {
            let response: Vec<RiskEntryResponse> = risks
                .into_iter()
                .map(|r| RiskEntryResponse {
                    id: r.id.to_string(),
                    identifier: r.identifier,
                    description: r.description,
                    score: r.score,
                    status: r.status,
                    created_at: r.created_at.to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Token Vault (PCI DSS) Handlers
// ============================================================

pub async fn save_token(
    req: web::Json<SaveTokenRequest>,
    repo: web::Data<Arc<dyn ITokenVaultRepository>>,
) -> impl Responder {
    let token = TokenVaultEntry {
        id: Uuid::new_v4(),
        token: req.token.clone(),
        masked_pan: req.masked_pan.clone(),
        is_active: true,
        created_at: Utc::now(),
        revoked_at: None,
    };

    match repo.save_token(&token).await {
        Ok(_) => {
            let response = TokenVaultResponse {
                id: token.id.to_string(),
                token: token.token,
                masked_pan: token.masked_pan,
                is_active: token.is_active,
                created_at: token.created_at.to_rfc3339(),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

pub async fn list_tokens(
    repo: web::Data<Arc<dyn ITokenVaultRepository>>,
) -> impl Responder {
    match repo.list_active_tokens().await {
        Ok(tokens) => {
            let response: Vec<TokenVaultResponse> = tokens
                .into_iter()
                .map(|t| TokenVaultResponse {
                    id: t.id.to_string(),
                    token: t.token,
                    masked_pan: t.masked_pan,
                    is_active: t.is_active,
                    created_at: t.created_at.to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// INPDP Consent Handlers
// ============================================================

pub async fn grant_consent(
    req: web::Json<GrantConsentRequest>,
    _repo: web::Data<Arc<dyn IInpdpConsentRepository>>,
) -> impl Responder {
    let _customer_id = match Uuid::parse_str(&req.customer_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID format".to_string(),
        }),
    };

    // Note: In a real implementation, this would use the ComplianceService
    // to handle consent creation. For now, we're just logging the request.
    let response = ConsentResponse {
        id: Uuid::new_v4().to_string(),
        customer_id: req.customer_id.clone(),
        purpose: req.purpose.clone(),
        legal_basis: req.legal_basis.clone(),
        status: "active".to_string(),
        created_at: Utc::now().to_rfc3339(),
    };
    HttpResponse::Created().json(response)
}

pub async fn get_consents(
    query: web::Query<std::collections::HashMap<String, String>>,
    repo: web::Data<Arc<dyn IInpdpConsentRepository>>,
) -> impl Responder {
    let customer_id = match query.get("customer_id").and_then(|id| Uuid::parse_str(id).ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Missing or invalid customer_id parameter".to_string(),
        }),
    };

    match repo.find_consents_by_customer(customer_id).await {
        Ok(consents) => {
            let response: Vec<ConsentResponse> = consents
                .into_iter()
                .map(|c| ConsentResponse {
                    id: c.id().as_uuid().to_string(),
                    customer_id: customer_id.to_string(),
                    purpose: c.purpose().as_str().to_string(),
                    legal_basis: c.legal_basis().as_str().to_string(),
                    status: if c.is_valid() { "active" } else { "revoked" }.to_string(),
                    created_at: c.granted_at().unwrap_or_else(Utc::now).to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// DPIA Handlers
// ============================================================

pub async fn save_dpia(
    req: web::Json<SaveDpiaRequest>,
    _repo: web::Data<Arc<dyn IDpiaRepository>>,
) -> impl Responder {
    // Note: In a real implementation, this would use DpiaService
    // to handle DPIA creation with proper domain logic.
    let response = DpiaResponse {
        id: Uuid::new_v4().to_string(),
        title: req.title.clone(),
        processing_activity: req.processing_activity.clone(),
        status: req.status.clone(),
        created_at: Utc::now().to_rfc3339(),
    };
    HttpResponse::Created().json(response)
}

pub async fn list_dpias(
    repo: web::Data<Arc<dyn IDpiaRepository>>,
) -> impl Responder {
    match repo.list_all_dpias().await {
        Ok(dpias) => {
            let response: Vec<DpiaResponse> = dpias
                .into_iter()
                .map(|d| DpiaResponse {
                    id: d.id().as_uuid().to_string(),
                    title: d.title().to_string(),
                    processing_activity: d.processing_activity().to_string(),
                    status: d.status().as_str().to_string(),
                    created_at: d.created_at().to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Breach Notification Handlers
// ============================================================

pub async fn report_breach(
    req: web::Json<ReportBreachRequest>,
    _repo: web::Data<Arc<dyn IBreachNotificationRepository>>,
) -> impl Responder {
    // Note: In a real implementation, this would use BreachNotificationService
    let response = BreachResponse {
        id: Uuid::new_v4().to_string(),
        breach_type: req.breach_type.clone(),
        description: req.description.clone(),
        affected_count: req.affected_count,
        status: "Detected".to_string(),
        detected_at: Utc::now().to_rfc3339(),
    };
    HttpResponse::Created().json(response)
}

pub async fn list_breaches(
    repo: web::Data<Arc<dyn IBreachNotificationRepository>>,
) -> impl Responder {
    match repo.list_all_breaches().await {
        Ok(breaches) => {
            let response: Vec<BreachResponse> = breaches
                .into_iter()
                .map(|b| BreachResponse {
                    id: b.id().as_uuid().to_string(),
                    breach_type: b.breach_type().to_string(),
                    description: b.description().to_string(),
                    affected_count: b.affected_count() as i32,
                    status: b.status().as_str().to_string(),
                    detected_at: b.detected_at().to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

pub async fn get_pending_breaches(
    repo: web::Data<Arc<dyn IBreachNotificationRepository>>,
) -> impl Responder {
    match repo.count_pending_authority_notification().await {
        Ok(count) => HttpResponse::Ok().json(serde_json::json!({
            "pending_count": count
        })),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Data Portability Handlers
// ============================================================

pub async fn request_data_portability(
    req: web::Json<RequestDataPortabilityRequest>,
    repo: web::Data<Arc<dyn IDataPortabilityRepository>>,
) -> impl Responder {
    let customer_id = match Uuid::parse_str(&req.customer_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID format".to_string(),
        }),
    };

    let request = DataPortabilityRequest {
        id: Uuid::new_v4(),
        customer_id,
        status: "Pending".to_string(),
        reason: req.reason.clone(),
        requested_at: Utc::now(),
        completed_at: None,
    };

    match repo.save_request(&request).await {
        Ok(_) => {
            let response = DataPortabilityResponse {
                id: request.id.to_string(),
                customer_id: request.customer_id.to_string(),
                status: request.status,
                requested_at: request.requested_at.to_rfc3339(),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

pub async fn list_portability_requests(
    query: web::Query<std::collections::HashMap<String, String>>,
    repo: web::Data<Arc<dyn IDataPortabilityRepository>>,
) -> impl Responder {
    let customer_id = match query.get("customer_id").and_then(|id| Uuid::parse_str(id).ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Missing or invalid customer_id parameter".to_string(),
        }),
    };

    match repo.find_by_customer(customer_id).await {
        Ok(requests) => {
            let response: Vec<DataPortabilityResponse> = requests
                .into_iter()
                .map(|r| DataPortabilityResponse {
                    id: r.id.to_string(),
                    customer_id: r.customer_id.to_string(),
                    status: r.status,
                    requested_at: r.requested_at.to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Erasure Request Handlers
// ============================================================

pub async fn request_erasure(
    req: web::Json<RequestErasureRequest>,
    repo: web::Data<Arc<dyn IErasureRepository>>,
) -> impl Responder {
    let customer_id = match Uuid::parse_str(&req.customer_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID format".to_string(),
        }),
    };

    let request = ErasureRequest {
        id: Uuid::new_v4(),
        customer_id,
        status: "Pending".to_string(),
        reason: req.reason.clone(),
        requested_at: Utc::now(),
        scheduled_for: None,
    };

    match repo.save_request(&request).await {
        Ok(_) => {
            let response = ErasureResponse {
                id: request.id.to_string(),
                customer_id: request.customer_id.to_string(),
                status: request.status,
                requested_at: request.requested_at.to_rfc3339(),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

pub async fn list_erasure_requests(
    query: web::Query<std::collections::HashMap<String, String>>,
    repo: web::Data<Arc<dyn IErasureRepository>>,
) -> impl Responder {
    let customer_id = match query.get("customer_id").and_then(|id| Uuid::parse_str(id).ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Missing or invalid customer_id parameter".to_string(),
        }),
    };

    match repo.find_by_customer(customer_id).await {
        Ok(requests) => {
            let response: Vec<ErasureResponse> = requests
                .into_iter()
                .map(|r| ErasureResponse {
                    id: r.id.to_string(),
                    customer_id: r.customer_id.to_string(),
                    status: r.status,
                    requested_at: r.requested_at.to_rfc3339(),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Biometric Verification Handlers
// ============================================================

pub async fn verify_biometric(
    req: web::Json<VerifyBiometricRequest>,
    repo: web::Data<Arc<dyn IBiometricRepository>>,
) -> impl Responder {
    let customer_id = match Uuid::parse_str(&req.customer_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid customer ID format".to_string(),
        }),
    };

    let expiry = Utc::now() + chrono::Duration::days(365);
    let verification = BiometricVerificationDto {
        id: Uuid::new_v4(),
        customer_id,
        verification_type: req.verification_type.clone(),
        status: if req.confidence_score >= 0.95 { "verified" } else { "pending" }.to_string(),
        confidence_score: req.confidence_score,
        liveness_check: req.liveness_check,
        document_type: req.document_type.clone(),
        document_number: req.document_number.clone(),
        verified_at: if req.confidence_score >= 0.95 { Some(Utc::now()) } else { None },
        created_at: Utc::now(),
        expires_at: expiry,
    };

    match repo.save_verification(&verification).await {
        Ok(_) => {
            let response = BiometricVerificationResponse {
                id: verification.id.to_string(),
                customer_id: verification.customer_id.to_string(),
                verification_type: verification.verification_type,
                status: verification.status,
                confidence_score: verification.confidence_score,
                verified_at: verification.verified_at.map(|d| d.to_rfc3339()),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { error: e }),
    }
}

pub async fn list_biometric_verifications(
    query: web::Query<std::collections::HashMap<String, String>>,
    repo: web::Data<Arc<dyn IBiometricRepository>>,
) -> impl Responder {
    let customer_id = match query.get("customer_id").and_then(|id| Uuid::parse_str(id).ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Missing or invalid customer_id parameter".to_string(),
        }),
    };

    match repo.find_by_customer(customer_id).await {
        Ok(verifications) => {
            let response: Vec<BiometricVerificationResponse> = verifications
                .into_iter()
                .map(|v| BiometricVerificationResponse {
                    id: v.id.to_string(),
                    customer_id: v.customer_id.to_string(),
                    verification_type: v.verification_type,
                    status: v.status,
                    confidence_score: v.confidence_score,
                    verified_at: v.verified_at.map(|d| d.to_rfc3339()),
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

pub async fn get_verified_biometric(
    query: web::Query<std::collections::HashMap<String, String>>,
    repo: web::Data<Arc<dyn IBiometricRepository>>,
) -> impl Responder {
    let customer_id = match query.get("customer_id").and_then(|id| Uuid::parse_str(id).ok()) {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Missing or invalid customer_id parameter".to_string(),
        }),
    };

    match repo.find_verified_by_customer(customer_id).await {
        Ok(Some(verification)) => {
            let response = BiometricVerificationResponse {
                id: verification.id.to_string(),
                customer_id: verification.customer_id.to_string(),
                verification_type: verification.verification_type,
                status: verification.status,
                confidence_score: verification.confidence_score,
                verified_at: verification.verified_at.map(|d| d.to_rfc3339()),
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "No valid biometric verification found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse { error: e }),
    }
}

// ============================================================
// Compliance Dashboard Handler
// ============================================================

pub async fn get_compliance_dashboard(
    smsi_repo: web::Data<Arc<dyn ISmsiRepository>>,
    breach_repo: web::Data<Arc<dyn IBreachNotificationRepository>>,
    _dpia_repo: web::Data<Arc<dyn IDpiaRepository>>,
) -> impl Responder {
    let total_controls = match smsi_repo.list_all_controls().await {
        Ok(controls) => controls.len() as i64,
        Err(_) => 0,
    };

    let implemented_controls: i64 = smsi_repo.count_by_status("Implemented").await.unwrap_or_default();

    let high_risks = match smsi_repo.list_high_risks().await {
        Ok(risks) => risks.len() as i64,
        Err(_) => 0,
    };

    let pending_breaches: i64 = breach_repo.count_pending_authority_notification().await.unwrap_or_default();

    let active_consents = 0i64; // Would need consent repo to count

    let data_requests_pending = 0i64; // Would need data rights repo to count

    let response = DashboardResponse {
        total_controls,
        implemented_controls,
        high_risks,
        pending_breaches,
        active_consents,
        data_requests_pending,
    };

    HttpResponse::Ok().json(response)
}
