use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use banko_application::identity::{TwoFactorError, TwoFactorService};
use banko_domain::identity::UserId;

use crate::web::handlers::auth_handlers::ErrorResponse;
use crate::web::middleware::AuthenticatedUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Enable2FAResponse {
    pub secret: String,
    pub qr_code_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Verify2FARequest {
    pub totp_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FAResponse {
    pub message: String,
    pub backup_codes: Vec<String>,
}

pub async fn enable_2fa_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<TwoFactorService>>,
) -> HttpResponse {
    let user_id = match UserId::parse(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            });
        }
    };

    match service.enable(&user_id, &auth_user.email).await {
        Ok((secret, qr_code_url)) => HttpResponse::Ok().json(Enable2FAResponse {
            secret,
            qr_code_url,
        }),
        Err(TwoFactorError::AlreadyEnabled) => HttpResponse::Conflict().json(ErrorResponse {
            error: "2FA is already enabled".to_string(),
        }),
        Err(e) => {
            tracing::error!("2FA enable error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

pub async fn verify_2fa_handler(
    auth_user: AuthenticatedUser,
    service: web::Data<Arc<TwoFactorService>>,
    body: web::Json<Verify2FARequest>,
) -> HttpResponse {
    let user_id = match UserId::parse(&auth_user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            });
        }
    };

    match service.verify_and_activate(&user_id, &body.totp_code).await {
        Ok(backup_codes) => HttpResponse::Ok().json(Verify2FAResponse {
            message: "2FA successfully enabled".to_string(),
            backup_codes,
        }),
        Err(TwoFactorError::InvalidCode) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid TOTP code".to_string(),
        }),
        Err(TwoFactorError::NotEnabled) | Err(TwoFactorError::NotPending) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "2FA setup not initiated. Call /auth/2fa/enable first".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("2FA verify error: {e}");
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::{test, web, App};
    use async_trait::async_trait;

    use banko_application::identity::{ITotpService, ITwoFactorRepository};
    use banko_domain::identity::{TwoFactorAuth, UserId};

    use super::*;
    use crate::config::JwtConfig;

    struct MockTotpService;

    impl ITotpService for MockTotpService {
        fn generate_secret(&self) -> String {
            "JBSWY3DPEHPK3PXP".to_string()
        }

        fn generate_totp_uri(&self, secret: &str, email: &str) -> String {
            format!("otpauth://totp/BANKO:{email}?secret={secret}")
        }

        fn verify_code(&self, _secret: &str, code: &str) -> bool {
            code == "123456"
        }
    }

    struct MockTwoFactorRepo {
        data: Mutex<Vec<TwoFactorAuth>>,
    }

    impl MockTwoFactorRepo {
        fn new() -> Self {
            MockTwoFactorRepo {
                data: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ITwoFactorRepository for MockTwoFactorRepo {
        async fn save(&self, tfa: &TwoFactorAuth) -> Result<(), String> {
            let mut data = self.data.lock().unwrap();
            data.retain(|t| t.user_id() != tfa.user_id());
            data.push(tfa.clone());
            Ok(())
        }

        async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<TwoFactorAuth>, String> {
            let data = self.data.lock().unwrap();
            Ok(data.iter().find(|t| t.user_id() == user_id).cloned())
        }

        async fn delete_by_user_id(&self, user_id: &UserId) -> Result<(), String> {
            let mut data = self.data.lock().unwrap();
            data.retain(|t| t.user_id() != user_id);
            Ok(())
        }
    }

    fn make_2fa_service() -> Arc<TwoFactorService> {
        Arc::new(TwoFactorService::new(
            Arc::new(MockTwoFactorRepo::new()),
            Arc::new(MockTotpService),
        ))
    }

    fn test_jwt_config() -> JwtConfig {
        JwtConfig::new(
            "test-secret-must-be-long-enough-for-jwt".to_string(),
            3600,
            604800,
        )
    }

    fn get_token_for_user(jwt: &JwtConfig, user_id: &str) -> String {
        jwt.generate_access_token(user_id, "test@banko.tn", &["user".to_string()])
            .unwrap()
    }

    #[actix_rt::test]
    async fn test_enable_2fa() {
        let tfa_service = make_2fa_service();
        let jwt = test_jwt_config();
        let user_id = uuid::Uuid::new_v4().to_string();
        let token = get_token_for_user(&jwt, &user_id);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(tfa_service))
                .app_data(web::Data::new(jwt))
                .route("/auth/2fa/enable", web::post().to(enable_2fa_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/2fa/enable")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: Enable2FAResponse = test::read_body_json(resp).await;
        assert_eq!(body.secret, "JBSWY3DPEHPK3PXP");
        assert!(body.qr_code_url.contains("otpauth://totp/BANKO:"));
    }

    #[actix_rt::test]
    async fn test_verify_2fa_valid_code() {
        let tfa_service = make_2fa_service();
        let jwt = test_jwt_config();
        let user_id = uuid::Uuid::new_v4().to_string();
        let token = get_token_for_user(&jwt, &user_id);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(tfa_service))
                .app_data(web::Data::new(jwt))
                .route("/auth/2fa/enable", web::post().to(enable_2fa_handler))
                .route("/auth/2fa/verify", web::post().to(verify_2fa_handler)),
        )
        .await;

        // Enable first
        let req = test::TestRequest::post()
            .uri("/auth/2fa/enable")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        test::call_service(&app, req).await;

        // Verify
        let req = test::TestRequest::post()
            .uri("/auth/2fa/verify")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(serde_json::json!({ "totp_code": "123456" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: Verify2FAResponse = test::read_body_json(resp).await;
        assert_eq!(body.backup_codes.len(), 8);
    }

    #[actix_rt::test]
    async fn test_verify_2fa_invalid_code() {
        let tfa_service = make_2fa_service();
        let jwt = test_jwt_config();
        let user_id = uuid::Uuid::new_v4().to_string();
        let token = get_token_for_user(&jwt, &user_id);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(tfa_service))
                .app_data(web::Data::new(jwt))
                .route("/auth/2fa/enable", web::post().to(enable_2fa_handler))
                .route("/auth/2fa/verify", web::post().to(verify_2fa_handler)),
        )
        .await;

        // Enable first
        let req = test::TestRequest::post()
            .uri("/auth/2fa/enable")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request();
        test::call_service(&app, req).await;

        // Verify with wrong code
        let req = test::TestRequest::post()
            .uri("/auth/2fa/verify")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .set_json(serde_json::json!({ "totp_code": "000000" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_rt::test]
    async fn test_enable_2fa_without_auth_returns_401() {
        let tfa_service = make_2fa_service();
        let jwt = test_jwt_config();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(tfa_service))
                .app_data(web::Data::new(jwt))
                .route("/auth/2fa/enable", web::post().to(enable_2fa_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/auth/2fa/enable")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }
}
