use actix_web::web;

use super::handlers::{auth_handlers, profile_handlers, two_factor_handlers, user_handlers};

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/register", web::post().to(auth_handlers::register_handler))
            .route("/login", web::post().to(auth_handlers::login_handler))
            .route("/logout", web::post().to(auth_handlers::logout_handler))
            .route(
                "/2fa/enable",
                web::post().to(two_factor_handlers::enable_2fa_handler),
            )
            .route(
                "/2fa/verify",
                web::post().to(two_factor_handlers::verify_2fa_handler),
            ),
    );
}

pub fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/profile", web::get().to(profile_handlers::get_profile))
            .route("/users", web::post().to(user_handlers::create_user_handler))
            .route("/users/{id}", web::get().to(user_handlers::get_user_handler))
            .route(
                "/users/{id}/roles",
                web::put().to(user_handlers::update_user_roles_handler),
            ),
    );
}
