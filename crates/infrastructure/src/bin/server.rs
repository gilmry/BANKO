use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use banko_infrastructure::config::JwtConfig;
use banko_infrastructure::jobs::{SessionCleanupJob, CreditClassificationJob};
use banko_infrastructure::web::metrics::{create_prometheus_metrics, metrics_handler};
use banko_infrastructure::web::routes::{configure_api_routes, configure_auth_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();

    let jwt_config = JwtConfig::from_env();

    tracing::info!("Starting BANKO API server on 0.0.0.0:8080");

    // NOTE: Full wiring requires a PostgreSQL connection.
    // Real wiring with PgPool, PgUserRepository, BcryptPasswordHasher, etc.
    // will be done when Docker + PostgreSQL is running.
    //
    // Background jobs would be spawned here when fully wired:
    // SessionCleanupJob:
    // let session_service = Arc::new(SessionService::new(session_repo));
    // let cleanup_job = SessionCleanupJob::new(session_service);
    // let _cleanup_handle = cleanup_job.spawn();
    //
    // CreditClassificationJob (STORY-CR-08):
    // let loan_service = Arc::new(LoanService::new(loan_repo, schedule_repo));
    // let classification_job = CreditClassificationJob::new(loan_service);
    // let _classification_handle = classification_job.spawn();
    //
    // The handles should be stored to allow graceful shutdown of background jobs.

    let prometheus = create_prometheus_metrics();

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .app_data(web::Data::new(jwt_config.clone()))
            .route("/metrics", web::get().to(metrics_handler))
            .configure(configure_auth_routes)
            .configure(configure_api_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
