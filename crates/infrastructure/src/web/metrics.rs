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
        "status": "healthy"
    }))
}

pub async fn metrics_handler() -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    match encoder.encode(&metric_families, &mut Vec::new()) {
        Ok(buffer) => {
            let buf = Vec::new();
            if let Ok(_) = encoder.encode(&metric_families, &mut buf.clone()) {
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
        .route("/metrics", web::get().to(metrics_handler));
}
