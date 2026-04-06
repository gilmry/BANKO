use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_application::governance::AuditService;
use banko_application::identity::{RegisterError, UserService};
use banko_domain::governance::{AuditAction, ResourceType};
use banko_domain::identity::UserId;

use crate::web::handlers::auth_handlers::ErrorResponse;
use crate::web::middleware::{AuthenticatedUser, RequireRole};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub created_at: String,
}

pub async fn create_user_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<UserService>>,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    // Check admin role
    let _guard = match RequireRole::admin(auth_user) {
        Ok(g) => g,
        Err(resp) => return resp,
    };

    // Register the user
    let user_id = match service.register(&body.email, &body.password).await {
        Ok(id) => id,
        Err(RegisterError::EmailTaken) => {
            return HttpResponse::Conflict().json(ErrorResponse {
                error: "Email already registered".to_string(),
            });
        }
        Err(RegisterError::InvalidEmail(msg)) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Invalid email format: {msg}"),
            });
        }
        Err(RegisterError::WeakPassword(msg)) => {
            return HttpResponse::BadRequest().json(ErrorResponse { error: msg });
        }
        Err(RegisterError::Internal(msg)) => {
            tracing::error!("Create user internal error: {msg}");
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            });
        }
    };

    // If roles specified, update them
    if let Some(ref role_strs) = body.roles {
        let roles: Result<Vec<_>, _> = role_strs
            .iter()
            .map(|r| banko_domain::identity::Role::from_str_role(r))
            .collect();

        match roles {
            Ok(roles) => {
                if let Err(e) = service.update_roles(&user_id, roles).await {
                    tracing::error!("Failed to update roles: {e}");
                }
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: format!("Invalid role: {e}"),
                });
            }
        }
    }

    // Fetch the created user to return full profile
    match service.find_by_id(&user_id).await {
        Ok(user) => HttpResponse::Created().json(UserProfileResponse {
            user_id: user.id().to_string(),
            email: user.email().as_str().to_string(),
            roles: user
                .roles()
                .iter()
                .map(|r| r.as_str().to_string())
                .collect(),
            is_active: user.is_active(),
            created_at: user.created_at().to_rfc3339(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn get_user_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<UserService>>,
    path: web::Path<String>,
) -> HttpResponse {
    let target_user_id_str = path.into_inner();

    // Allow own profile or admin
    let is_own = auth_user.user_id == target_user_id_str;
    if !is_own && !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ErrorResponse {
            error: "Forbidden: Admin role required or must be own profile".to_string(),
        });
    }

    let user_id = match UserId::parse(&target_user_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid user ID format".to_string(),
            });
        }
    };

    match service.find_by_id(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserProfileResponse {
            user_id: user.id().to_string(),
            email: user.email().as_str().to_string(),
            roles: user
                .roles()
                .iter()
                .map(|r| r.as_str().to_string())
                .collect(),
            is_active: user.is_active(),
            created_at: user.created_at().to_rfc3339(),
        }),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".to_string(),
        }),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateRolesRequest {
    pub roles: Vec<String>,
}

pub async fn update_user_roles_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<UserService>>,
    audit_service: web::Data<Arc<AuditService>>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateRolesRequest>,
) -> HttpResponse {
    // SuperAdmin only
    let guard = match RequireRole::super_admin(auth_user) {
        Ok(g) => g,
        Err(resp) => return resp,
    };

    let target_user_id_str = path.into_inner();

    // Prevent self role change
    if guard.user.user_id == target_user_id_str {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Cannot modify your own roles".to_string(),
        });
    }

    let user_id = match UserId::parse(&target_user_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid user ID format".to_string(),
            });
        }
    };

    // Parse roles
    let roles: Result<Vec<_>, _> = body
        .roles
        .iter()
        .map(|r| banko_domain::identity::Role::from_str_role(r))
        .collect();

    let roles = match roles {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Invalid role: {e}"),
            });
        }
    };

    if roles.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "At least one role is required".to_string(),
        });
    }

    // Fetch the user before update to capture old roles for audit trail (STORY-ID-08)
    let old_user = match service.find_by_id(&user_id).await {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "User not found".to_string(),
            });
        }
    };
    let old_roles: Vec<String> = old_user.roles().iter().map(|r| r.as_str().to_string()).collect();

    match service.update_roles(&user_id, roles).await {
        Ok(user) => {
            let new_roles: Vec<String> = user.roles().iter().map(|r| r.as_str().to_string()).collect();

            // Extract IP address from request
            let ip_address = req
                .connection_info()
                .peer_addr()
                .map(|s| s.to_string());

            // Prepare audit log changes as JSON
            let changes = serde_json::json!({
                "old_roles": old_roles,
                "new_roles": new_roles,
            })
            .to_string();

            // Log role change to immutable audit trail (STORY-ID-08)
            if let Err(e) = audit_service
                .log_action(
                    Uuid::parse_str(&guard.user.user_id).unwrap_or_else(|_| Uuid::nil()),
                    AuditAction::Update,
                    ResourceType::User,
                    *user.id().as_uuid(),
                    Some(changes),
                    ip_address,
                )
                .await
            {
                tracing::error!("Failed to log role change audit entry: {:?}", e);
            }

            tracing::info!(
                "SuperAdmin {} changed user {} roles to {:?}",
                guard.user.email,
                target_user_id_str,
                user.roles()
            );
            HttpResponse::Ok().json(UserProfileResponse {
                user_id: user.id().to_string(),
                email: user.email().as_str().to_string(),
                roles: user
                    .roles()
                    .iter()
                    .map(|r| r.as_str().to_string())
                    .collect(),
                is_active: user.is_active(),
                created_at: user.created_at().to_rfc3339(),
            })
        }
        Err(e) => {
            tracing::error!("Role update failed: {e}");
            HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("Role update failed: {e}"),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use super::*;
    use crate::config::JwtConfig;
    use crate::test_helpers::{make_test_audit_service, make_test_user_service};
    use crate::web::handlers::auth_handlers::{login_handler, register_handler, LoginResponse};

    fn test_jwt_config() -> JwtConfig {
        JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            3600,
            604800,
        )
    }

    fn configure(cfg: &mut web::ServiceConfig) {
        cfg.route("/auth/register", web::post().to(register_handler))
            .route("/auth/login", web::post().to(login_handler))
            .route("/users", web::post().to(create_user_handler))
            .route("/users/{id}", web::get().to(get_user_handler))
            .route(
                "/users/{id}/roles",
                web::put().to(update_user_roles_handler),
            );
    }

    // Helper to inject audit service into test app
    fn configure_with_audit(cfg: &mut web::ServiceConfig, audit_service: Arc<AuditService>) {
        cfg.route("/auth/register", web::post().to(register_handler))
            .route("/auth/login", web::post().to(login_handler))
            .route("/users", web::post().to(create_user_handler))
            .route("/users/{id}", web::get().to(get_user_handler))
            .route(
                "/users/{id}/roles",
                web::put().to(update_user_roles_handler),
            )
            .app_data(web::Data::new(audit_service));
    }

    async fn get_admin_token(jwt: &JwtConfig) -> String {
        jwt.generate_access_token(
            "admin-123",
            "admin@banko.tn",
            &["admin".to_string(), "user".to_string()],
        )
        .unwrap()
    }

    async fn get_super_admin_token(jwt: &JwtConfig) -> String {
        jwt.generate_access_token(
            "superadmin-123",
            "superadmin@banko.tn",
            &[
                "superadmin".to_string(),
                "admin".to_string(),
                "user".to_string(),
            ],
        )
        .unwrap()
    }

    async fn get_user_token(jwt: &JwtConfig) -> String {
        jwt.generate_access_token("user-123", "user@banko.tn", &["user".to_string()])
            .unwrap()
    }

    #[actix_rt::test]
    async fn test_create_user_as_admin() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let token = get_admin_token(&jwt).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/users")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(serde_json::json!({
                "email": "analyst@banko.tn",
                "password": "SecurePass123!",
                "roles": ["Analyst"]
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);

        let body: UserProfileResponse = test::read_body_json(resp).await;
        assert_eq!(body.email, "analyst@banko.tn");
        assert!(body.roles.contains(&"analyst".to_string()));
    }

    #[actix_rt::test]
    async fn test_create_user_as_non_admin_returns_403() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let token = get_user_token(&jwt).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/users")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(serde_json::json!({
                "email": "hacker@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403);
    }

    #[actix_rt::test]
    async fn test_get_user_as_admin() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt.clone()))
                .configure(configure),
        )
        .await;

        // Create user via admin
        let admin_token = get_admin_token(&jwt).await;
        let req = test::TestRequest::post()
            .uri("/users")
            .insert_header(("Authorization", format!("Bearer {admin_token}")))
            .set_json(serde_json::json!({
                "email": "getme@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let created: UserProfileResponse = test::read_body_json(resp).await;

        // Get user
        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", created.user_id))
            .insert_header(("Authorization", format!("Bearer {admin_token}")))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: UserProfileResponse = test::read_body_json(resp).await;
        assert_eq!(body.email, "getme@banko.tn");
    }

    #[actix_rt::test]
    async fn test_get_user_non_admin_non_self_returns_403() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let user_token = get_user_token(&jwt).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/users/some-other-user-id")
            .insert_header(("Authorization", format!("Bearer {user_token}")))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403);
    }

    #[actix_rt::test]
    async fn test_get_own_profile_as_user() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt.clone()))
                .configure(configure),
        )
        .await;

        // Register + login to get own user_id
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "self@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        test::call_service(&app, req).await;

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "self@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let login_body: LoginResponse = test::read_body_json(resp).await;

        // Decode to get user_id
        let claims = jwt.validate_token(&login_body.access_token).unwrap();

        // Get own profile
        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", claims.sub))
            .insert_header((
                "Authorization",
                format!("Bearer {}", login_body.access_token),
            ))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    // --- Update Roles tests (STORY-ID-08) ---

    #[actix_rt::test]
    async fn test_update_roles_as_super_admin() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let audit_service = make_test_audit_service();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt.clone()))
                .app_data(web::Data::new(audit_service.clone()))
                .configure(configure),
        )
        .await;

        // Create user first
        let admin_token = get_admin_token(&jwt).await;
        let req = test::TestRequest::post()
            .uri("/users")
            .insert_header(("Authorization", format!("Bearer {admin_token}")))
            .set_json(serde_json::json!({
                "email": "rolechange@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let created: UserProfileResponse = test::read_body_json(resp).await;

        // Update roles as super admin
        let sa_token = get_super_admin_token(&jwt).await;
        let req = test::TestRequest::put()
            .uri(&format!("/users/{}/roles", created.user_id))
            .insert_header(("Authorization", format!("Bearer {sa_token}")))
            .set_json(serde_json::json!({
                "roles": ["Analyst", "Compliance"]
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: UserProfileResponse = test::read_body_json(resp).await;
        assert!(body.roles.contains(&"analyst".to_string()));
        assert!(body.roles.contains(&"compliance".to_string()));

        // Verify audit trail entry was created (STORY-ID-08)
        // Note: In a real test with a database, we would query the audit table
        // Here we're just verifying the endpoint completed successfully
    }

    #[actix_rt::test]
    async fn test_update_roles_as_admin_returns_403() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let admin_token = get_admin_token(&jwt).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/users/some-user-id/roles")
            .insert_header(("Authorization", format!("Bearer {admin_token}")))
            .set_json(serde_json::json!({
                "roles": ["Admin"]
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403);
    }

    #[actix_rt::test]
    async fn test_update_roles_as_user_returns_403() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let user_token = get_user_token(&jwt).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::put()
            .uri("/users/some-user-id/roles")
            .insert_header(("Authorization", format!("Bearer {user_token}")))
            .set_json(serde_json::json!({
                "roles": ["Admin"]
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403);
    }

    #[actix_rt::test]
    async fn test_update_roles_invalid_role() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt.clone()))
                .configure(configure),
        )
        .await;

        // Create user first
        let admin_token = get_admin_token(&jwt).await;
        let req = test::TestRequest::post()
            .uri("/users")
            .insert_header(("Authorization", format!("Bearer {admin_token}")))
            .set_json(serde_json::json!({
                "email": "invalidrole@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let created: UserProfileResponse = test::read_body_json(resp).await;

        let sa_token = get_super_admin_token(&jwt).await;
        let req = test::TestRequest::put()
            .uri(&format!("/users/{}/roles", created.user_id))
            .insert_header(("Authorization", format!("Bearer {sa_token}")))
            .set_json(serde_json::json!({
                "roles": ["InvalidRole"]
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_update_own_roles_prevented() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let sa_token = get_super_admin_token(&jwt).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .configure(configure),
        )
        .await;

        // Try to change own roles (superadmin-123 trying to change superadmin-123)
        let req = test::TestRequest::put()
            .uri("/users/superadmin-123/roles")
            .insert_header(("Authorization", format!("Bearer {sa_token}")))
            .set_json(serde_json::json!({
                "roles": ["User"]
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert!(body.error.contains("Cannot modify your own roles"));
    }
}
