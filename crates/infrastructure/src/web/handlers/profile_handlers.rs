use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
}

pub async fn get_profile(user: AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(ProfileResponse {
        user_id: user.user_id,
        email: user.email,
        roles: user.roles,
    })
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use super::*;
    use crate::config::JwtConfig;
    use crate::test_helpers::make_test_user_service;
    use crate::web::handlers::auth_handlers::{
        login_handler, register_handler, ErrorResponse, LoginResponse,
    };

    fn test_jwt_config() -> JwtConfig {
        JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            3600,
            604800,
        )
    }

    #[actix_rt::test]
    async fn test_profile_with_valid_token() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt.clone()))
                .route("/auth/register", web::post().to(register_handler))
                .route("/auth/login", web::post().to(login_handler))
                .route("/api/profile", web::get().to(get_profile)),
        )
        .await;

        // Register
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(serde_json::json!({
                "email": "middleware@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        test::call_service(&app, req).await;

        // Login
        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(serde_json::json!({
                "email": "middleware@banko.tn",
                "password": "SecurePass123!"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let login_body: LoginResponse = test::read_body_json(resp).await;
        let token = login_body.access_token;

        // Access profile with token
        let req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: ProfileResponse = test::read_body_json(resp).await;
        assert_eq!(body.email, "middleware@banko.tn");
        assert!(body.roles.contains(&"user".to_string()));
    }

    #[actix_rt::test]
    async fn test_profile_without_token_returns_401() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .route("/api/profile", web::get().to(get_profile)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/profile").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_rt::test]
    async fn test_profile_with_invalid_token_returns_401() {
        let service = make_test_user_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .route("/api/profile", web::get().to(get_profile)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header(("Authorization", "Bearer invalid.token.here"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_rt::test]
    async fn test_profile_with_expired_token_returns_401() {
        let service = make_test_user_service();
        let jwt_normal = test_jwt_config();
        let jwt_expired = JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            -10,
            -10,
        );

        let token = jwt_expired
            .generate_access_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt_normal))
                .route("/api/profile", web::get().to(get_profile)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_rt::test]
    async fn test_profile_with_refresh_token_returns_401() {
        let jwt = test_jwt_config();
        let service = make_test_user_service();

        let token = jwt
            .generate_refresh_token("user-123", "test@banko.tn", &["user".to_string()])
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .app_data(web::Data::new(jwt))
                .route("/api/profile", web::get().to(get_profile)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.error, "Invalid token type");
    }
}
