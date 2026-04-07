use actix_web::{web, HttpResponse};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::{Encoder, TextEncoder};

pub fn create_prometheus_metrics() -> actix_web_prom::PrometheusMetrics {
    PrometheusMetricsBuilder::new("banko")
        .build()
        .expect("failed to create PrometheusMetrics")
}

pub async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "banko-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn api_info_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "BANKO Core Banking API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Open source core banking platform for Tunisian banks",
        "license": "AGPL-3.0",
        "bounded_contexts": [
            "customer", "account", "credit", "aml", "sanctions",
            "prudential", "accounting", "reporting", "payment", "fx",
            "governance", "identity", "compliance", "reference_data",
            "collateral", "islamic_banking", "cash_management",
            "trade_finance", "insurance", "securities", "data_hub", "arrangement"
        ],
        "endpoints": {
            "health": "/api/v1/health",
            "auth": "/api/v1/auth",
            "customers": "/api/v1/customers",
            "accounts": "/api/v1/accounts",
            "credits": "/api/v1/credits",
            "payments": "/api/v1/payments",
            "aml": "/api/v1/aml",
            "sanctions": "/api/v1/sanctions",
            "accounting": "/api/v1/accounting",
            "reporting": "/api/v1/reporting",
            "governance": "/api/v1/governance",
            "fx": "/api/v1/fx",
            "metrics": "/metrics"
        }
    }))
}

pub async fn metrics_handler() -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    match encoder.encode(&metric_families, &mut Vec::new()) {
        Ok(_buffer) => {
            let buf = Vec::new();
            if encoder.encode(&metric_families, &mut buf.clone()).is_ok() {
                return HttpResponse::Ok()
                    .content_type("text/plain; version=0.0.4")
                    .body(String::from_utf8_lossy(&buf).to_string());
            }
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to encode metrics"
            }))
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to gather metrics"
        })),
    }
}

pub fn configure_metrics_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_handler))
        .route("/metrics", web::get().to(metrics_handler))
        .route("/api/v1/info", web::get().to(api_info_handler));
}