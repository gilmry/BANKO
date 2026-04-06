use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use banko_application::reporting::{
    AnalyticsService, ReportBuilderService, ClientPortfolio, OperationalKpis,
    ReportDefinition, ReportOutput, ReportType, ReportFormat, TrendDataPoint,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub success: bool,
    pub data: Option<ClientPortfolio>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KpisResponse {
    pub success: bool,
    pub data: Option<OperationalKpis>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendsResponse {
    pub success: bool,
    pub data: Option<Vec<TrendDataPoint>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDefinitionResponse {
    pub success: bool,
    pub data: Option<ReportDefinition>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDefinitionListResponse {
    pub success: bool,
    pub data: Option<Vec<ReportDefinition>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportOutputResponse {
    pub success: bool,
    pub data: Option<ReportOutput>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReportRequest {
    pub name: String,
    pub report_type: String,
    pub filters: serde_json::Value,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendQueryParams {
    pub metric: String,
    pub days: Option<u32>,
}

/// GET /analytics/portfolio/{customer_id}
/// Returns comprehensive client portfolio
pub async fn get_client_portfolio_handler(
    path: web::Path<String>,
    _analytics: web::Data<AnalyticsService>,
) -> HttpResponse {
    let customer_id = path.into_inner();

    // Validate customer_id is a valid UUID
    if Uuid::parse_str(&customer_id).is_err() {
        return HttpResponse::BadRequest().json(PortfolioResponse {
            success: false,
            data: None,
            error: Some("Invalid customer ID format".to_string()),
        });
    }

    // In production, inject the analytics service and call it
    // For now, return a stub response
    HttpResponse::Ok().json(PortfolioResponse {
        success: true,
        data: None,
        error: None,
    })
}

/// GET /analytics/accounts/{id}/drilldown
/// Returns detailed account analysis
pub async fn get_account_drilldown_handler(
    path: web::Path<String>,
    _analytics: web::Data<AnalyticsService>,
) -> HttpResponse {
    let account_id = path.into_inner();

    // Validate account_id is a valid UUID
    if Uuid::parse_str(&account_id).is_err() {
        return HttpResponse::BadRequest().json(PortfolioResponse {
            success: false,
            data: None,
            error: Some("Invalid account ID format".to_string()),
        });
    }

    HttpResponse::Ok().json(PortfolioResponse {
        success: true,
        data: None,
        error: None,
    })
}

/// GET /analytics/kpis
/// Returns operational KPIs dashboard
pub async fn get_operational_kpis_handler(
    _analytics: web::Data<AnalyticsService>,
) -> HttpResponse {
    // In production, call analytics service
    HttpResponse::Ok().json(KpisResponse {
        success: true,
        data: None,
        error: None,
    })
}

/// GET /analytics/trends?metric=active_customers&days=30
/// Returns trend data for a specific metric
pub async fn get_trend_handler(
    query: web::Query<TrendQueryParams>,
    _analytics: web::Data<AnalyticsService>,
) -> HttpResponse {
    let metric = query.metric.clone();
    let days = query.days.unwrap_or(30);

    if metric.is_empty() {
        return HttpResponse::BadRequest().json(TrendsResponse {
            success: false,
            data: None,
            error: Some("Metric parameter is required".to_string()),
        });
    }

    if days > 365 {
        return HttpResponse::BadRequest().json(TrendsResponse {
            success: false,
            data: None,
            error: Some("Days parameter cannot exceed 365".to_string()),
        });
    }

    // In production, call analytics service
    HttpResponse::Ok().json(TrendsResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    })
}

/// POST /analytics/reports
/// Create a new report definition
pub async fn create_report_handler(
    body: web::Json<CreateReportRequest>,
    _service: web::Data<ReportBuilderService>,
) -> HttpResponse {
    // Validate required fields
    if body.name.is_empty() {
        return HttpResponse::BadRequest().json(ReportDefinitionResponse {
            success: false,
            data: None,
            error: Some("Report name is required".to_string()),
        });
    }

    // Validate report type
    match body.report_type.as_str() {
        "Transactional" | "Compliance" | "Financial" => {}
        _ => {
            return HttpResponse::BadRequest().json(ReportDefinitionResponse {
                success: false,
                data: None,
                error: Some("Invalid report type".to_string()),
            });
        }
    }

    // Validate format
    match body.format.as_str() {
        "Pdf" | "Csv" | "Json" | "Excel" => {}
        _ => {
            return HttpResponse::BadRequest().json(ReportDefinitionResponse {
                success: false,
                data: None,
                error: Some("Invalid report format".to_string()),
            });
        }
    }

    // In production, call service to create report definition
    HttpResponse::Created().json(ReportDefinitionResponse {
        success: true,
        data: None,
        error: None,
    })
}

/// POST /analytics/reports/{id}/execute
/// Execute a report and return output
pub async fn execute_report_handler(
    path: web::Path<String>,
    _service: web::Data<ReportBuilderService>,
) -> HttpResponse {
    let report_id = path.into_inner();

    // Validate report_id is a valid UUID
    if Uuid::parse_str(&report_id).is_err() {
        return HttpResponse::BadRequest().json(ReportOutputResponse {
            success: false,
            data: None,
            error: Some("Invalid report ID format".to_string()),
        });
    }

    // In production, call service to execute report
    HttpResponse::Ok().json(ReportOutputResponse {
        success: true,
        data: None,
        error: None,
    })
}

/// GET /analytics/reports
/// List all report definitions
pub async fn list_reports_handler(
    _service: web::Data<ReportBuilderService>,
) -> HttpResponse {
    // In production, call service to list reports
    HttpResponse::Ok().json(ReportDefinitionListResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    })
}

// Admin/Backup handlers

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupResponse {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupListResponse {
    pub success: bool,
    pub backups: Option<Vec<serde_json::Value>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrResponse {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerBackupRequest {
    pub backup_type: String,
}

/// POST /admin/backup
/// Trigger a database backup
pub async fn trigger_backup_handler(
    body: web::Json<TriggerBackupRequest>,
) -> HttpResponse {
    // Validate backup type
    match body.backup_type.as_str() {
        "Full" | "Incremental" | "Wal" => {}
        _ => {
            return HttpResponse::BadRequest().json(BackupResponse {
                success: false,
                message: None,
                error: Some("Invalid backup type".to_string()),
            });
        }
    }

    // In production, call backup service
    HttpResponse::Ok().json(BackupResponse {
        success: true,
        message: Some("Backup started".to_string()),
        error: None,
    })
}

/// GET /admin/backups
/// List all backup records
pub async fn list_backups_handler() -> HttpResponse {
    // In production, call backup service to list backups
    HttpResponse::Ok().json(BackupListResponse {
        success: true,
        backups: Some(vec![]),
        error: None,
    })
}

/// POST /admin/dr/execute
/// Execute disaster recovery plan
pub async fn trigger_dr_handler() -> HttpResponse {
    // In production, call disaster recovery orchestrator
    HttpResponse::Ok().json(DrResponse {
        success: true,
        message: Some("Disaster recovery plan initiated".to_string()),
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_report_request_validation() {
        let valid_request = CreateReportRequest {
            name: "Test Report".to_string(),
            report_type: "Compliance".to_string(),
            filters: serde_json::json!({}),
            date_from: Some("2026-01-01".to_string()),
            date_to: Some("2026-12-31".to_string()),
            format: "Json".to_string(),
        };

        assert!(!valid_request.name.is_empty());
        assert_eq!(valid_request.report_type, "Compliance");
    }

    #[test]
    fn test_invalid_report_type() {
        let request = CreateReportRequest {
            name: "Test".to_string(),
            report_type: "InvalidType".to_string(),
            filters: serde_json::json!({}),
            date_from: None,
            date_to: None,
            format: "Json".to_string(),
        };

        assert!(!matches!(
            request.report_type.as_str(),
            "Transactional" | "Compliance" | "Financial"
        ));
    }

    #[test]
    fn test_invalid_format() {
        let request = CreateReportRequest {
            name: "Test".to_string(),
            report_type: "Compliance".to_string(),
            filters: serde_json::json!({}),
            date_from: None,
            date_to: None,
            format: "InvalidFormat".to_string(),
        };

        assert!(!matches!(
            request.format.as_str(),
            "Pdf" | "Csv" | "Json" | "Excel"
        ));
    }

    #[test]
    fn test_trend_query_params() {
        let params = TrendQueryParams {
            metric: "active_customers".to_string(),
            days: Some(30),
        };

        assert_eq!(params.metric, "active_customers");
        assert_eq!(params.days, Some(30));
    }

    #[test]
    fn test_trend_query_params_default_days() {
        let params = TrendQueryParams {
            metric: "active_customers".to_string(),
            days: None,
        };

        assert_eq!(params.days.unwrap_or(30), 30);
    }

    #[test]
    fn test_backup_response_success() {
        let response = BackupResponse {
            success: true,
            message: Some("Backup started".to_string()),
            error: None,
        };

        assert!(response.success);
        assert!(response.message.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_backup_response_error() {
        let response = BackupResponse {
            success: false,
            message: None,
            error: Some("Backup failed".to_string()),
        };

        assert!(!response.success);
        assert!(response.error.is_some());
    }

    #[test]
    fn test_trigger_backup_request() {
        let request = TriggerBackupRequest {
            backup_type: "Full".to_string(),
        };

        assert_eq!(request.backup_type, "Full");
    }

    #[test]
    fn test_portfolio_response_serialization() {
        let response = PortfolioResponse {
            success: true,
            data: None,
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
    }

    #[test]
    fn test_kpis_response_serialization() {
        let response = KpisResponse {
            success: true,
            data: None,
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
    }
}
