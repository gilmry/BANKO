use actix_web::web;

use super::handlers::{
    account_handlers, auth_handlers, credit_handlers, customer_handlers, profile_handlers,
    two_factor_handlers, user_handlers,
};

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

pub fn configure_customer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/customers")
            .route(
                "",
                web::post().to(customer_handlers::create_customer_handler),
            )
            .route(
                "",
                web::get().to(customer_handlers::list_customers_handler),
            )
            .route(
                "/{id}",
                web::get().to(customer_handlers::get_customer_handler),
            )
            .route(
                "/{id}/kyc",
                web::get().to(customer_handlers::get_customer_kyc_handler),
            )
            .route(
                "/{id}/kyc",
                web::put().to(customer_handlers::update_kyc_handler),
            )
            .route(
                "/{id}/approve",
                web::post().to(customer_handlers::approve_kyc_handler),
            )
            .route(
                "/{id}/reject",
                web::post().to(customer_handlers::reject_kyc_handler),
            ),
    );
}

pub fn configure_account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/accounts")
            .route(
                "",
                web::post().to(account_handlers::create_account_handler),
            )
            .route(
                "",
                web::get().to(account_handlers::list_accounts_handler),
            )
            .route(
                "/{id}",
                web::get().to(account_handlers::get_account_handler),
            )
            .route(
                "/{id}/movements",
                web::post().to(account_handlers::create_movement_handler),
            )
            .route(
                "/{id}/movements",
                web::get().to(account_handlers::list_movements_handler),
            )
            .route(
                "/{id}/statement",
                web::get().to(account_handlers::get_statement_handler),
            ),
    );
}

pub fn configure_credit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/loans")
            .route(
                "",
                web::post().to(credit_handlers::create_loan_handler),
            )
            .route(
                "",
                web::get().to(credit_handlers::list_loans_handler),
            )
            .route(
                "/{id}",
                web::get().to(credit_handlers::get_loan_handler),
            )
            .route(
                "/{id}/approve",
                web::post().to(credit_handlers::approve_loan_handler),
            )
            .route(
                "/{id}/disburse",
                web::post().to(credit_handlers::disburse_loan_handler),
            )
            .route(
                "/{id}/classify",
                web::post().to(credit_handlers::classify_loan_handler),
            )
            .route(
                "/{id}/payment",
                web::post().to(credit_handlers::record_payment_handler),
            ),
    );
}
