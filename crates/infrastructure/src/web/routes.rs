use actix_web::web;

use super::metrics;
use super::handlers::{
    account_handlers, accounting_handlers, aml_handlers, auth_handlers, consent_handlers,
    credit_handlers, customer_handlers, data_rights_handlers, fx_handlers, governance_handlers,
    notification_handlers, payment_handlers, cheque_handlers, product_handlers, profile_handlers, prudential_handlers, reporting_handlers,
    retention_handlers, sanctions_handlers, two_factor_handlers, user_handlers, analytics_handlers, mobile_handlers,
};
use crate::payment::recurring_handlers;

use crate::governance::bct_audit_handlers;

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
            .route("/health", web::get().to(metrics::health_handler))
            .route("/profile", web::get().to(profile_handlers::get_profile))
            .route("/users", web::post().to(user_handlers::create_user_handler))
            .route(
                "/users/{id}",
                web::get().to(user_handlers::get_user_handler),
            )
            .route(
                "/users/{id}/roles",
                web::put().to(user_handlers::update_user_roles_handler),
            ),
    );

    // Configure domain-specific routes
    configure_payment_routes(cfg);
    configure_recurring_payment_routes(cfg);
    configure_mobile_routes(cfg);
}

pub fn configure_customer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/customers")
            .route(
                "",
                web::post().to(customer_handlers::create_customer_handler),
            )
            .route("", web::get().to(customer_handlers::list_customers_handler))
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
            )
            // Consent routes (STORY-CONS-01)
            .route(
                "/{id}/consent",
                web::post().to(consent_handlers::grant_consent_handler),
            )
            .route(
                "/{id}/consent",
                web::delete().to(consent_handlers::revoke_consent_handler),
            )
            .route(
                "/{id}/consents",
                web::get().to(consent_handlers::list_consents_handler),
            )
            // Data rights routes (STORY-CONS-02)
            .route(
                "/{id}/data-export",
                web::get().to(data_rights_handlers::data_export_handler),
            )
            .route(
                "/{id}/data-rectification",
                web::put().to(data_rights_handlers::data_rectification_handler),
            )
            .route(
                "/{id}/data-opposition",
                web::post().to(data_rights_handlers::data_opposition_handler),
            )
            .route(
                "/{id}/data-requests",
                web::get().to(data_rights_handlers::list_data_requests_handler),
            ),
    );
}

pub fn configure_account_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/accounts")
            .route("", web::post().to(account_handlers::create_account_handler))
            .route("", web::get().to(account_handlers::list_accounts_handler))
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
            .route("", web::post().to(credit_handlers::create_loan_handler))
            .route("", web::get().to(credit_handlers::list_loans_handler))
            .route("/{id}", web::get().to(credit_handlers::get_loan_handler))
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

pub fn configure_aml_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/transactions")
            .route("", web::post().to(aml_handlers::create_transaction_handler))
            .route("", web::get().to(aml_handlers::list_transactions_handler))
            .route(
                "/{id}",
                web::get().to(aml_handlers::get_transaction_handler),
            ),
    );
    cfg.service(
        web::scope("/api/v1/aml")
            .route("/alerts", web::get().to(aml_handlers::list_alerts_handler))
            .route(
                "/alerts/{id}",
                web::get().to(aml_handlers::get_alert_handler),
            )
            .route(
                "/investigations",
                web::post().to(aml_handlers::open_investigation_handler),
            )
            .route(
                "/investigations/{id}",
                web::get().to(aml_handlers::get_investigation_handler),
            )
            .route(
                "/investigations/{id}/notes",
                web::post().to(aml_handlers::add_note_handler),
            )
            .route(
                "/investigations/{id}/escalate",
                web::post().to(aml_handlers::escalate_investigation_handler),
            )
            .route(
                "/investigations/{id}/close",
                web::post().to(aml_handlers::close_investigation_handler),
            )
            .route(
                "/reports",
                web::post().to(aml_handlers::generate_report_handler),
            )
            .route(
                "/reports/{id}/submit",
                web::post().to(aml_handlers::submit_report_handler),
            )
            .route(
                "/freezes",
                web::post().to(aml_handlers::freeze_account_handler),
            )
            .route(
                "/freezes",
                web::get().to(aml_handlers::list_freezes_handler),
            )
            .route(
                "/freezes/{id}/lift",
                web::post().to(aml_handlers::lift_freeze_handler),
            ),
    );
}

pub fn configure_sanctions_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/sanctions")
            .route(
                "/check",
                web::get().to(sanctions_handlers::screen_name_handler),
            )
            .route(
                "/results",
                web::get().to(sanctions_handlers::list_results_handler),
            )
            .route(
                "/results/{id}",
                web::get().to(sanctions_handlers::get_result_handler),
            )
            .route(
                "/lists",
                web::get().to(sanctions_handlers::list_sanctions_lists_handler),
            )
            .route(
                "/lists/{source}",
                web::get().to(sanctions_handlers::get_list_handler),
            )
            .route(
                "/dashboard",
                web::get().to(sanctions_handlers::dashboard_handler),
            ),
    );
}

pub fn configure_prudential_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/prudential")
            .route(
                "/ratios",
                web::get().to(prudential_handlers::get_ratios_handler),
            )
            .route(
                "/ratios",
                web::post().to(prudential_handlers::calculate_ratios_handler),
            )
            .route(
                "/solvency",
                web::get().to(prudential_handlers::check_solvency_handler),
            )
            .route(
                "/tier1",
                web::get().to(prudential_handlers::check_tier1_handler),
            )
            .route(
                "/credit-deposit",
                web::get().to(prudential_handlers::check_credit_deposit_handler),
            )
            .route(
                "/concentration",
                web::get().to(prudential_handlers::check_concentration_handler),
            )
            .route(
                "/alerts",
                web::get().to(prudential_handlers::get_alerts_handler),
            ),
    );
}

pub fn configure_accounting_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/accounting")
            .route(
                "/entries",
                web::post().to(accounting_handlers::create_entry_handler),
            )
            .route(
                "/entries",
                web::get().to(accounting_handlers::list_entries_handler),
            )
            .route(
                "/entries/{id}",
                web::get().to(accounting_handlers::get_entry_handler),
            )
            .route(
                "/entries/{id}/reverse",
                web::post().to(accounting_handlers::reverse_entry_handler),
            )
            .route(
                "/ledger",
                web::get().to(accounting_handlers::get_ledger_handler),
            )
            .route(
                "/trial-balance",
                web::get().to(accounting_handlers::get_trial_balance_handler),
            )
            .route(
                "/periods/close",
                web::post().to(accounting_handlers::close_period_handler),
            )
            .route(
                "/ecl-staging",
                web::get().to(accounting_handlers::get_ecl_staging_handler),
            ),
    );
}

pub fn configure_governance_routes(cfg: &mut web::ServiceConfig) {
    governance_handlers::configure_governance_routes(cfg);
}

pub fn configure_reporting_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/reporting")
            .route(
                "/forms",
                web::get().to(reporting_handlers::list_reports_handler),
            )
            .route(
                "/forms/{id}",
                web::get().to(reporting_handlers::get_report_handler),
            )
            .route(
                "/generate",
                web::post().to(reporting_handlers::generate_report_handler),
            )
            .route(
                "/forms/{id}/validate",
                web::post().to(reporting_handlers::validate_report_handler),
            )
            .route(
                "/forms/{id}/submit",
                web::post().to(reporting_handlers::submit_report_handler),
            )
            .route(
                "/forms/{id}/acknowledge",
                web::post().to(reporting_handlers::acknowledge_report_handler),
            )
            .route(
                "/templates",
                web::get().to(reporting_handlers::list_templates_handler),
            )
            .route("/ifrs9", web::get().to(reporting_handlers::ifrs9_handler)),
    );
}

pub fn configure_payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/payments")
            .route(
                "/transfers",
                web::post().to(payment_handlers::create_payment_handler),
            )
            .route("", web::get().to(payment_handlers::list_payments_handler))
            .route(
                "/{id}",
                web::get().to(payment_handlers::get_payment_handler),
            )
            .route(
                "/{id}/status",
                web::get().to(payment_handlers::get_payment_status_handler),
            )
            .route(
                "/{id}/screen",
                web::post().to(payment_handlers::screen_payment_handler),
            )
            .route(
                "/{id}/submit",
                web::post().to(payment_handlers::submit_payment_handler),
            )
            .route(
                "/{id}/execute",
                web::post().to(payment_handlers::execute_payment_handler),
            )
            .route(
                "/clearing",
                web::post().to(payment_handlers::run_clearing_handler),
            ),
    );

    // Configure card management routes (STORY-CARD-01 through CARD-06)
    configure_card_routes(cfg);

    // Configure cheque management routes (EPIC-19: CHQ-01 through CHQ-03)
    configure_cheque_routes(cfg);
}

pub fn configure_card_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/cards")
            .route(
                "",
                web::post().to(payment_handlers::request_card_handler),
            )
            .route("", web::get().to(payment_handlers::list_cards_handler))
            .route(
                "/{id}",
                web::get().to(payment_handlers::get_card_handler),
            )
            .route(
                "/{id}/activate",
                web::post().to(payment_handlers::activate_card_handler),
            )
            .route(
                "/{id}/block",
                web::post().to(payment_handlers::block_card_handler),
            )
            .route(
                "/{id}/unblock",
                web::post().to(payment_handlers::unblock_card_handler),
            )
            .route(
                "/{id}",
                web::delete().to(payment_handlers::cancel_card_handler),
            )
            .route(
                "/{id}/limits",
                web::put().to(payment_handlers::set_limits_handler),
            )
            .route(
                "/{id}/transactions",
                web::get().to(payment_handlers::card_transactions_handler),
            )
            .route(
                "/{id}/authorize",
                web::post().to(payment_handlers::authorize_transaction_handler),
            ),
    );
}

pub fn configure_recurring_payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/recurring")
            // Standing Orders (STORY-RECUR-01)
            .service(
                web::scope("/standing-orders")
                    .route(
                        "",
                        web::post().to(recurring_handlers::create_standing_order_handler),
                    )
                    .route(
                        "",
                        web::get().to(recurring_handlers::list_standing_orders_handler),
                    )
                    .route(
                        "/{id}",
                        web::get().to(recurring_handlers::get_standing_order_handler),
                    )
                    .route(
                        "/{id}/suspend",
                        web::post().to(recurring_handlers::suspend_standing_order_handler),
                    )
                    .route(
                        "/{id}/resume",
                        web::post().to(recurring_handlers::resume_standing_order_handler),
                    )
                    .route(
                        "/{id}/cancel",
                        web::post().to(recurring_handlers::cancel_standing_order_handler),
                    ),
            )
            // Direct Debit Mandates (STORY-RECUR-02)
            .service(
                web::scope("/mandates")
                    .route(
                        "",
                        web::post().to(recurring_handlers::create_mandate_handler),
                    )
                    .route(
                        "",
                        web::get().to(recurring_handlers::list_mandates_handler),
                    )
                    .route(
                        "/{id}/sign",
                        web::post().to(recurring_handlers::sign_mandate_handler),
                    )
                    .route(
                        "/{id}/revoke",
                        web::post().to(recurring_handlers::revoke_mandate_handler),
                    ),
            ),
    );
}

pub fn configure_fx_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/fx")
            .route(
                "/operations",
                web::post().to(fx_handlers::create_fx_operation_handler),
            )
            .route(
                "/operations",
                web::get().to(fx_handlers::list_fx_operations_handler),
            )
            .route(
                "/operations/{id}",
                web::get().to(fx_handlers::get_fx_operation_handler),
            )
            .route(
                "/operations/{id}/confirm",
                web::post().to(fx_handlers::confirm_fx_operation_handler),
            )
            .route(
                "/operations/{id}/settle",
                web::post().to(fx_handlers::settle_fx_operation_handler),
            )
            .route(
                "/rates",
                web::get().to(fx_handlers::list_rates_handler),
            )
            .route(
                "/rates",
                web::put().to(fx_handlers::update_rate_handler),
            )
            .route(
                "/positions",
                web::get().to(fx_handlers::get_positions_handler),
            )
            .route(
                "/limits/{account_id}",
                web::get().to(fx_handlers::get_daily_limits_handler),
            ),
    );
}

pub fn configure_retention_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin/retention")
            .route(
                "/run",
                web::post().to(retention_handlers::run_anonymization_handler),
            )
            .route(
                "/status/{customer_id}",
                web::get().to(retention_handlers::check_retention_handler),
            ),
    );
}

pub fn configure_bct_audit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/bct/audit")
            .route(
                "/entries",
                web::get().to(bct_audit_handlers::list_bct_entries_handler),
            )
            .route(
                "/entries/export",
                web::get().to(bct_audit_handlers::export_entries_handler),
            )
            .route(
                "/integrity",
                web::get().to(bct_audit_handlers::integrity_check_handler),
            ),
    );
    cfg.service(
        web::scope("/api/v1/bct/dashboard")
            .route(
                "/stats",
                web::get().to(bct_audit_handlers::dashboard_stats_handler),
            )
            .route(
                "/daily-trend",
                web::get().to(bct_audit_handlers::daily_trend_handler),
            )
            .route(
                "/suspicious",
                web::get().to(bct_audit_handlers::suspicious_handler),
            ),
    );
}

pub fn configure_notification_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/notifications")
            .route(
                "/send",
                web::post().to(notification_handlers::send_notification_handler),
            )
            .route("", web::get().to(notification_handlers::list_notifications_handler))
            .route(
                "/{id}",
                web::get().to(notification_handlers::get_notification_handler),
            )
            .route(
                "/preferences/{customer_id}",
                web::get().to(notification_handlers::get_preferences_handler),
            )
            .route(
                "/preferences",
                web::put().to(notification_handlers::update_preference_handler),
            )
            .route(
                "/templates",
                web::get().to(notification_handlers::list_templates_handler),
            ),
    );
}

pub fn configure_product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/products")
            .route(
                "",
                web::post().to(product_handlers::create_product_handler),
            )
            .route("", web::get().to(product_handlers::list_products_handler))
            .route(
                "/{id}",
                web::get().to(product_handlers::get_product_handler),
            )
            .route(
                "/{id}/activate",
                web::post().to(product_handlers::activate_product_handler),
            )
            .route(
                "/{id}/suspend",
                web::post().to(product_handlers::suspend_product_handler),
            )
            .route(
                "/pricing/calculate",
                web::post().to(product_handlers::calculate_price_handler),
            )
            .route(
                "/eligibility/check",
                web::post().to(product_handlers::check_eligibility_handler),
            )
            .route(
                "/eligibility/eligible",
                web::post().to(product_handlers::get_eligible_products_handler),
            )
            .route(
                "/interest/daily",
                web::post().to(product_handlers::calculate_daily_interest_handler),
            )
            .route(
                "/interest/maturity",
                web::post().to(product_handlers::calculate_maturity_handler),
            ),
    );
}

pub fn configure_admin_pricing_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin/pricing-grids")
            .route(
                "",
                web::post().to(product_handlers::create_pricing_grid_handler),
            ),
    );
}

pub fn configure_cheque_routes(cfg: &mut web::ServiceConfig) {
    cheque_handlers::configure_cheque_routes(cfg);
}

pub fn configure_analytics_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/analytics")
            .route(
                "/portfolio/{customer_id}",
                web::get().to(analytics_handlers::get_client_portfolio_handler),
            )
            .route(
                "/accounts/{id}/drilldown",
                web::get().to(analytics_handlers::get_account_drilldown_handler),
            )
            .route(
                "/kpis",
                web::get().to(analytics_handlers::get_operational_kpis_handler),
            )
            .route(
                "/trends",
                web::get().to(analytics_handlers::get_trend_handler),
            )
            .route(
                "/reports",
                web::post().to(analytics_handlers::create_report_handler),
            )
            .route(
                "/reports/{id}/execute",
                web::post().to(analytics_handlers::execute_report_handler),
            )
            .route(
                "/reports",
                web::get().to(analytics_handlers::list_reports_handler),
            ),
    );
}

pub fn configure_admin_backup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            .route(
                "/backup",
                web::post().to(analytics_handlers::trigger_backup_handler),
            )
            .route(
                "/backups",
                web::get().to(analytics_handlers::list_backups_handler),
            )
            .route(
                "/dr/execute",
                web::post().to(analytics_handlers::trigger_dr_handler),
            ),
    );
}

pub fn configure_mobile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/mobile")
            // Mobile Auth: Device Management
            .route(
                "/devices",
                web::post().to(mobile_handlers::register_device_handler),
            )
            .route(
                "/devices",
                web::get().to(mobile_handlers::list_devices_handler),
            )
            .route(
                "/devices/{id}",
                web::delete().to(mobile_handlers::deactivate_device_handler),
            )
            .route(
                "/devices/{id}/biometric",
                web::post().to(mobile_handlers::enable_biometric_handler),
            )
            .route(
                "/devices/{id}/pin",
                web::post().to(mobile_handlers::set_pin_handler),
            )
            // Mobile Auth: Session Management
            .route(
                "/auth/login",
                web::post().to(mobile_handlers::login_mobile_handler),
            )
            .route(
                "/auth/refresh",
                web::post().to(mobile_handlers::refresh_session_handler),
            )
            // Mobile Account: Dashboard & Sync
            .route(
                "/dashboard",
                web::get().to(mobile_handlers::get_dashboard_handler),
            )
            .route(
                "/offline-cache",
                web::get().to(mobile_handlers::get_offline_data_handler),
            )
            .route(
                "/sync",
                web::post().to(mobile_handlers::sync_handler),
            )
            // Mobile Payments: Transfers & QR
            .route(
                "/payments/transfer",
                web::post().to(mobile_handlers::quick_transfer_handler),
            )
            .route(
                "/payments/beneficiaries",
                web::get().to(mobile_handlers::frequent_beneficiaries_handler),
            )
            .route(
                "/payments/scan-qr",
                web::post().to(mobile_handlers::scan_qr_handler),
            ),
    );
}
