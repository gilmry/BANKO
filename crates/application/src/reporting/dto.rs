use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// --- Report DTOs ---

#[derive(Debug, Deserialize)]
pub struct GenerateReportRequest {
    pub report_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ReportResponse {
    pub id: String,
    pub report_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub template_id: String,
    pub template_version: String,
    pub data: String,
    pub status: String,
    pub generated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub generated_by: String,
}

#[derive(Debug, Serialize)]
pub struct ReportListResponse {
    pub data: Vec<ReportResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

// --- Template DTOs ---

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: String,
    pub name: String,
    pub report_type: String,
    pub version: String,
    pub definition: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct TemplateListResponse {
    pub data: Vec<TemplateResponse>,
}

// --- IFRS 9 DTO ---

#[derive(Debug, Serialize)]
pub struct Ifrs9Response {
    pub as_of: NaiveDate,
    pub stage1_count: i64,
    pub stage1_ecl: f64,
    pub stage2_count: i64,
    pub stage2_ecl: f64,
    pub stage3_count: i64,
    pub stage3_ecl: f64,
    pub total_ecl: f64,
}
