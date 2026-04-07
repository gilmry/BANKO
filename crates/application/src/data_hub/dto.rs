use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- DataEntity DTOs ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataEntityRequest {
    pub entity_type: String,
    pub source_system: String,
    pub canonical_id: String,
    pub initial_quality_score: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataEntityResponse {
    pub id: String,
    pub entity_type: String,
    pub source_system: String,
    pub canonical_id: String,
    pub data_quality_score: u8,
    pub last_validated_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQualityScoreRequest {
    pub new_score: u8,
}

// --- Quality Rule DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct DataQualityRuleRequest {
    pub entity_type: String,
    pub rule_name: String,
    pub rule_expression: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataQualityRuleResponse {
    pub id: String,
    pub entity_type: String,
    pub rule_name: String,
    pub rule_expression: String,
    pub severity: String,
    pub is_active: bool,
}

// --- Data Lineage DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct DataLineageRequest {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub transformation_type: String,
    pub pipeline_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataLineageResponse {
    pub id: String,
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub transformation_type: String,
    pub pipeline_name: String,
    pub last_run_at: DateTime<Utc>,
}

// --- Data Reconciliation DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct DataReconciliationRequest {
    pub entity_type: String,
    pub source_a: String,
    pub source_b: String,
    pub scheduled_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataReconciliationResponse {
    pub id: String,
    pub entity_type: String,
    pub source_a: String,
    pub source_b: String,
    pub discrepancies_found: Vec<String>,
    pub status: String,
    pub scheduled_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDiscrepancyRequest {
    pub discrepancy: String,
}

// --- Master Data Record DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataRecordRequest {
    pub entity_type: String,
    pub canonical_data: serde_json::Value,
    pub sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataRecordResponse {
    pub id: String,
    pub entity_type: String,
    pub canonical_data: serde_json::Value,
    pub version: u32,
    pub is_golden_record: bool,
    pub sources: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMasterDataRequest {
    pub canonical_data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSourceRequest {
    pub source: String,
}

// --- Governance Policy DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct DataGovernancePolicyRequest {
    pub name: String,
    pub description: String,
    pub entity_types_covered: Vec<String>,
    pub retention_days: u32,
    pub classification: String,
    pub owner_team: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataGovernancePolicyResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub entity_types_covered: Vec<String>,
    pub retention_days: u32,
    pub classification: String,
    pub owner_team: String,
    pub is_active: bool,
}

// --- Summary/Analytics DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct DataHubSummaryResponse {
    pub total_entities: i64,
    pub quarantined_entities: i64,
    pub stale_entities: i64,
    pub average_quality_score: f64,
    pub golden_records_count: i64,
    pub active_policies: i64,
    pub pending_reconciliations: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityTypeStatsResponse {
    pub entity_type: String,
    pub total_count: i64,
    pub average_quality_score: f64,
    pub quarantined_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataLineageTraceResponse {
    pub entity_id: String,
    pub upstream: Vec<DataLineageResponse>,
    pub downstream: Vec<DataLineageResponse>,
}
