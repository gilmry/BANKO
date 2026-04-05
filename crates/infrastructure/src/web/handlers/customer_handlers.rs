use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::customer::{
    AddressDto, CreateCustomerRequest, CustomerResponse, CustomerService, CustomerServiceError,
    KycProfileResponse, UpdateKycRequest,
};
use banko_domain::shared::value_objects::CustomerId;

use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct CreateResponse {
    customer_id: String,
    status: String,
}

fn customer_to_response(customer: &banko_domain::customer::Customer) -> CustomerResponse {
    CustomerResponse {
        customer_id: customer.id().to_string(),
        customer_type: customer.customer_type().as_str().to_string(),
        status: customer.status().as_str().to_string(),
        full_name: customer.kyc_profile().full_name().to_string(),
        email: customer.kyc_profile().email().as_str().to_string(),
        phone: customer.kyc_profile().phone().as_str().to_string(),
        risk_score: customer.risk_score().value(),
        risk_level: customer.risk_score().risk_level().to_string(),
        consent: customer.consent().as_str().to_string(),
        created_at: customer.created_at().to_rfc3339(),
        updated_at: customer.updated_at().to_rfc3339(),
    }
}

fn kyc_to_response(customer: &banko_domain::customer::Customer) -> KycProfileResponse {
    let kyc = customer.kyc_profile();
    KycProfileResponse {
        full_name: kyc.full_name().to_string(),
        cin_or_rcs: kyc.cin_or_rcs().to_string(),
        birth_date: kyc.birth_date().map(|d| d.to_string()),
        nationality: kyc.nationality().to_string(),
        profession: kyc.profession().to_string(),
        address: AddressDto {
            street: kyc.address().street().to_string(),
            city: kyc.address().city().to_string(),
            postal_code: kyc.address().postal_code().to_string(),
            country: Some(kyc.address().country().to_string()),
        },
        phone: kyc.phone().as_str().to_string(),
        email: kyc.email().as_str().to_string(),
        pep_status: kyc.pep_status().as_str().to_string(),
        source_of_funds: kyc.source_of_funds().as_str().to_string(),
        sector: kyc.sector().map(|s| s.to_string()),
        submission_date: kyc.submission_date().map(|d| d.to_rfc3339()),
        approval_date: kyc.approval_date().map(|d| d.to_rfc3339()),
        rejection_reason: kyc.rejection_reason().map(|s| s.to_string()),
    }
}

fn map_service_error(err: CustomerServiceError) -> HttpResponse {
    match err {
        CustomerServiceError::CustomerNotFound => HttpResponse::NotFound().json(ErrorResponse {
            error: "Customer not found".to_string(),
        }),
        CustomerServiceError::EmailAlreadyExists(email) => {
            HttpResponse::Conflict().json(ErrorResponse {
                error: format!("Email already registered: {email}"),
            })
        }
        CustomerServiceError::Validation(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        CustomerServiceError::Domain(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        CustomerServiceError::Internal(msg) => {
            tracing::error!("Customer internal error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

/// POST /api/v1/customers
pub async fn create_customer_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
    body: web::Json<CreateCustomerRequest>,
) -> HttpResponse {
    match service.create_customer(body.into_inner()).await {
        Ok(customer_id) => HttpResponse::Created().json(CreateResponse {
            customer_id: customer_id.to_string(),
            status: "Pending".to_string(),
        }),
        Err(err) => map_service_error(err),
    }
}

/// GET /api/v1/customers/{id}
pub async fn get_customer_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match CustomerId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.find_by_id(&id).await {
        Ok(customer) => HttpResponse::Ok().json(customer_to_response(&customer)),
        Err(err) => map_service_error(err),
    }
}

/// GET /api/v1/customers/{id}/kyc
pub async fn get_customer_kyc_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = match CustomerId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.find_by_id(&id).await {
        Ok(customer) => HttpResponse::Ok().json(kyc_to_response(&customer)),
        Err(err) => map_service_error(err),
    }
}

/// GET /api/v1/customers
pub async fn list_customers_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
) -> HttpResponse {
    // Admin only
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ErrorResponse {
            error: "Admin access required".to_string(),
        });
    }

    match service.list_customers().await {
        Ok(customers) => {
            let responses: Vec<CustomerResponse> =
                customers.iter().map(customer_to_response).collect();
            HttpResponse::Ok().json(responses)
        }
        Err(err) => map_service_error(err),
    }
}

/// PUT /api/v1/customers/{id}/kyc
pub async fn update_kyc_handler(
    _auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
    path: web::Path<String>,
    body: web::Json<UpdateKycRequest>,
) -> HttpResponse {
    let id = match CustomerId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.update_kyc(&id, body.into_inner()).await {
        Ok(customer) => HttpResponse::Ok().json(customer_to_response(&customer)),
        Err(err) => map_service_error(err),
    }
}

/// POST /api/v1/customers/{id}/approve
pub async fn approve_kyc_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
    path: web::Path<String>,
) -> HttpResponse {
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ErrorResponse {
            error: "Admin access required".to_string(),
        });
    }

    let id = match CustomerId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.approve_kyc(&id).await {
        Ok(customer) => HttpResponse::Ok().json(customer_to_response(&customer)),
        Err(err) => map_service_error(err),
    }
}

#[derive(Debug, Deserialize)]
pub struct RejectKycRequest {
    pub reason: String,
}

/// POST /api/v1/customers/{id}/reject
pub async fn reject_kyc_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<CustomerService>>,
    path: web::Path<String>,
    body: web::Json<RejectKycRequest>,
) -> HttpResponse {
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ErrorResponse {
            error: "Admin access required".to_string(),
        });
    }

    let id = match CustomerId::parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid customer ID".to_string(),
            })
        }
    };

    match service.reject_kyc(&id, body.reason.clone()).await {
        Ok(customer) => HttpResponse::Ok().json(customer_to_response(&customer)),
        Err(err) => map_service_error(err),
    }
}
