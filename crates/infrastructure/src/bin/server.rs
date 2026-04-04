use actix_web::{web, App, HttpServer};

use banko_infrastructure::config::JwtConfig;
use banko_infrastructure::web::routes::{configure_api_routes, configure_auth_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();

    let jwt_config = JwtConfig::from_env();

    tracing::info!("Starting BANKO API server on 0.0.0.0:8080");

    // NOTE: Full wiring requires a PostgreSQL connection.
    // Real wiring with PgPool, PgUserRepository, BcryptPasswordHasher, etc.
    // will be done when Docker + PostgreSQL is running.

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jwt_config.clone()))
            .configure(configure_auth_routes)
            .configure(configure_api_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
