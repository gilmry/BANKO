use actix_web::{web, App, HttpServer};

use banko_infrastructure::config::JwtConfig;
use banko_infrastructure::web::metrics::{create_prometheus_metrics, metrics_handler};
use banko_infrastructure::web::routes::{
    configure_api_routes, configure_auth_routes,
    configure_customer_routes, configure_account_routes,
    configure_credit_routes, configure_aml_routes,
    configure_sanctions_routes, configure_prudential_routes,
    configure_accounting_routes, configure_governance_routes,
    configure_reporting_routes, configure_fx_routes,
    configure_notification_routes, configure_product_routes,
    configure_analytics_routes, configure_admin_backup_routes,
    configure_retention_routes, configure_bct_audit_routes,
    configure_admin_pricing_routes,
    configure_compliance_routes, configure_credit_enhanced_routes,
    configure_prudential_enhanced_routes, configure_payment_enhanced_routes,
    configure_fx_enhanced_routes, configure_governance_enhanced_routes,
    configure_identity_enhanced_routes, configure_reporting_enhanced_routes,
    configure_reference_data_routes,
    // P1 Bounded Contexts
    configure_collateral_routes, configure_islamic_banking_routes,
    configure_cash_management_routes, configure_trade_finance_routes,
    configure_insurance_routes,
    // P2 Bounded Contexts
    configure_securities_routes, configure_data_hub_routes,
    configure_arrangement_routes,
};

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
            // Core bounded contexts
            .configure(configure_customer_routes)
            .configure(configure_account_routes)
            .configure(configure_credit_routes)
            .configure(configure_aml_routes)
            .configure(configure_sanctions_routes)
            .configure(configure_prudential_routes)
            .configure(configure_accounting_routes)
            .configure(configure_governance_routes)
            .configure(configure_reporting_routes)
            .configure(configure_fx_routes)
            .configure(configure_notification_routes)
            .configure(configure_product_routes)
            .configure(configure_analytics_routes)
            .configure(configure_admin_backup_routes)
            .configure(configure_retention_routes)
            .configure(configure_bct_audit_routes)
            .configure(configure_admin_pricing_routes)
            // Enhanced bounded contexts
            .configure(configure_compliance_routes)
            .configure(configure_credit_enhanced_routes)
            .configure(configure_prudential_enhanced_routes)
            .configure(configure_payment_enhanced_routes)
            .configure(configure_fx_enhanced_routes)
            .configure(configure_governance_enhanced_routes)
            .configure(configure_identity_enhanced_routes)
            .configure(configure_reporting_enhanced_routes)
            .configure(configure_reference_data_routes)
            // P1 Bounded Contexts
            .configure(configure_collateral_routes)
            .configure(configure_islamic_banking_routes)
            .configure(configure_cash_management_routes)
            .configure(configure_trade_finance_routes)
            .configure(configure_insurance_routes)
            // P2 Bounded Contexts
            .configure(configure_securities_routes)
            .configure(configure_data_hub_routes)
            .configure(configure_arrangement_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
