use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::identity::{LoginError, RegisterError, SessionService, UserService};

use crate::config::JwtConfig;
use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register_handler(
    service: web::Data<Arc<UserService>>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    match service.register(&body.email, &body.password).await {
        Ok(user_id) => HttpResponse::Created().json(RegisterResponse {
            user_id: user_id.to_string(),
            email: body.email.trim().to_lowercase(),
        }),
        Err(RegisterError::EmailTaken) => HttpResponse::Conflict().json(ErrorResponse {
            error: "Email already registered".to_string(),
        }),
        Err(RegisterError::InvalidEmail(msg)) => HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("Invalid email format: {msg}"),
        }),
        Err(RegisterError::WeakPassword(msg)) => {
            HttpResponse::BadRequest().json(ErrorResponse { error: msg })
        }
        Err(RegisterError::Internal(msg)) => {
            tracing::error!("Registration internal error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

pub async fn login_handler(
    service: web::Data<Arc<UserService>>,
    jwt_config: web::Data<JwtConfig>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    match service.login(&body.email, &body.password).await {
        Ok(user) => {
            let roles: Vec<String> = user
                .roles()
                .iter()
                .map(|r| r.as_str().to_string())
                .collect();

            let access_token = match jwt_config.generate_access_token(
                &user.id().to_string(),
                user.email().as_str(),
                &roles,
            ) {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("JWT generation error: {e}");
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Internal server error".to_string(),
                    });
                }
            };

            let refresh_token = match jwt_config.generate_refresh_token(
                &user.id().to_string(),
                user.email().as_str(),
                &roles,
            ) {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("JWT refresh generation error: {e}");
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Internal server error".to_string(),
                    });
                }
            };

            HttpResponse::Ok().json(LoginResponse {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in: 3600,
            })
        }
        Err(LoginError::InvalidCredentials) => HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Invalid credentials".to_string(),
        }),
        Err(LoginError::AccountInactive) => HttpResponse::Forbidden().json(ErrorResponse {
            error: "Account is inactive".to_string(),
        }),
        Err(LoginError::AccountLocked) => HttpResponse::Forbidden().json(ErrorResponse {
            error: "Account is locked".to_string(),
        }),
        Err(LoginError::Internal(msg)) => {
            tracing::error!("Login internal error: {msg}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

pub async fn logout_handler(
    auth_user: AuthenticatedUser,
    session_service: web::Data<Arc<SessionService>>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    // Extract token from Authorization header for session lookup
    if let Some(auth_header) = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // Use a simple hash of the token for session lookup
            let token_hash = format!("{:x}", md5_hash(token.as_bytes()));
            if let Err(e) = session_service.logout(&token_hash).await {
                tracing::error!("Logout error: {e}");
            }
        }
    }

    tracing::info!("User {} logged out", auth_user.email);
    HttpResponse::Ok().json(serde_json::json!({ "message": "Logged out successfully" }))
}

/// Simple hash for session token (not cryptographic — just for lookup).
fn md5_hash(data: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use super::*;
    use crate::test_helpers::{
        create_test_user, make_test_user_service, make_test_user_service_with_user,
    };

    fn test_jwt_config() -> JwtConfig {
        JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            3600,
            604800,
        )
    }

    #[actix_rt::test]
    async fn test_register_success() {
        let service = make_test_user_service();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .route("/auth/register", web::post().to(register_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "test@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);

        let body: RegisterResponse = test::read_body_json(resp).await;
        assert_eq!(body.email, "test@banko.tn");
        assert!(!body.user_id.is_empty());
    }

    #[actix_rt::test]
    async fn test_register_invalid_email() {
        let service = make_test_user_service();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .route("/auth/register", web::post().to(register_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "invalid",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_register_weak_password() {
        let service = make_test_user_service();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .route("/auth/register", web::post().to(register_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "test@banko.tn",
                "password": "weak"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_register_duplicate_email() {
        let service = make_test_user_service();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service.clone()))
                .route("/auth/register", web::post().to(register_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "dup@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        test::call_service(&app, req).await;

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "dup@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 409);
    }

    // --- Login tests ---

    #[actix_rt::test]
    async fn test_login_success() {
        let user = create_test_user("login@banko.tn");
        let service = make_test_user_service_with_user(user);
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .route("/auth/login", web::post().to(login_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "login@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: LoginResponse = test::read_body_json(resp).await;
        assert!(!body.access_token.is_empty());
        assert!(!body.refresh_token.is_empty());
        assert_eq!(body.token_type, "Bearer");
        assert_eq!(body.expires_in, 3600);
    }

    #[actix_rt::test]
    async fn test_login_validates_token_structure() {
        let user = create_test_user("jwt@banko.tn");
        let service = make_test_user_service_with_user(user);
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt.clone()))
                .route("/auth/login", web::post().to(login_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "jwt@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let body: LoginResponse = test::read_body_json(resp).await;

        // Validate the access token has correct claims
        let claims = jwt.validate_token(&body.access_token).unwrap();
        assert_eq!(claims.email, "jwt@banko.tn");
        assert!(claims.roles.contains(&"user".to_string()));
        assert_eq!(claims.token_type, "access");
    }

    #[actix_rt::test]
    async fn test_login_invalid_credentials() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .route("/auth/login", web::post().to(login_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "nobody@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.error, "Invalid credentials");
    }

    #[actix_rt::test]
    async fn test_login_wrong_password() {
        let user = create_test_user("pwd@banko.tn");
        let service = make_test_user_service_with_user(user);
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .route("/auth/login", web::post().to(login_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "pwd@banko.tn",
                "password": "WrongPassword1!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    // --- Logout tests ---

    #[actix_rt::test]
    async fn test_logout_success() {
        let user = create_test_user("logout@banko.tn");
        let service = make_test_user_service_with_user(user);
        let session_service = crate::test_helpers::make_test_session_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(session_service))
                .app_data(web::Data::new(jwt.clone()))
                .route("/auth/login", web::post().to(login_handler))
                .route("/auth/logout", web::post().to(logout_handler)),
        )
        .await;

        // Login first
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "logout@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let body: LoginResponse = test::read_body_json(resp).await;

        // Logout
        let req = test::TestRequest::post()
            .uri("/auth/logout")
            .insert_header(("Authorization", format!("Bearer {}", body.access_token)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_logout_without_token_returns_401() {
        let service = make_test_user_service();
        let session_service = crate::test_helpers::make_test_session_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(session_service))
                .app_data(web::Data::new(jwt))
                .route("/auth/logout", web::post().to(logout_handler)),
        )
        .await;

        let req = test::TestRequest::post().uri("/auth/logout").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }
}
