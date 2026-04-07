use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::data_hub::{
    IDataEntityRepository, IQualityRuleRepository, IDataLineageRepository,
    IDataReconciliationRepository, IMasterDataRepository, IGovernancePolicyRepository,
};
use banko_domain::data_hub::{
    DataEntity, DataEntityId, DataEntityStatus, DataEntityType, DataQualityRule,
    DataQualityRuleId, DataQualityScore, DataLineage, DataLineageId, DataReconciliation,
    DataReconciliationId, MasterDataRecord, MasterDataRecordId, DataGovernancePolicy,
    DataGovernancePolicyId, RuleSeverity, ReconciliationStatus, TransformationType,
    DataClassification,
};

// --- PostgreSQL Repositories ---

pub struct PgDataEntityRepository {
    pool: PgPool,
}

impl PgDataEntityRepository {
    pub fn new(pool: PgPool) -> Self {
        PgDataEntityRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DataEntityRow {
    id: Uuid,
    entity_type: String,
    source_system: String,
    canonical_id: String,
    data_quality_score: i16,
    last_validated_at: DateTime<Utc>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DataEntityRow {
    fn into_domain(self) -> Result<DataEntity, String> {
        let id = DataEntityId::from_uuid(self.id);
        let entity_type = DataEntityType::from_str(&self.entity_type)
            .map_err(|e| e.to_string())?;
        let quality_score = DataQualityScore::new(self.data_quality_score as u8)
            .map_err(|e| e.to_string())?;
        let status = DataEntityStatus::from_str(&self.status)
            .map_err(|e| e.to_string())?;

        Ok(DataEntity::reconstitute(
            id,
            entity_type,
            self.source_system,
            self.canonical_id,
            quality_score,
            self.last_validated_at,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IDataEntityRepository for PgDataEntityRepository {
    async fn save(&self, entity: &DataEntity) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO data_hub_entities (id, entity_type, source_system, canonical_id, data_quality_score, last_validated_at, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                data_quality_score = $5,
                last_validated_at = $6,
                status = $7,
                updated_at = $9
            "#
        )
        .bind(entity.id().as_uuid())
        .bind(entity.entity_type().to_string())
        .bind(entity.source_system())
        .bind(entity.canonical_id())
        .bind(entity.data_quality_score().value() as i16)
        .bind(entity.last_validated_at())
        .bind(entity.status().to_string())
        .bind(entity.created_at())
        .bind(entity.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DataEntityId) -> Result<Option<DataEntity>, String> {
        let row: Option<DataEntityRow> = sqlx::query_as(
            "SELECT id, entity_type, source_system, canonical_id, data_quality_score, last_validated_at, status, created_at, updated_at FROM data_hub_entities WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_canonical_id(&self, canonical_id: &str) -> Result<Vec<DataEntity>, String> {
        let rows: Vec<DataEntityRow> = sqlx::query_as(
            "SELECT id, entity_type, source_system, canonical_id, data_quality_score, last_validated_at, status, created_at, updated_at FROM data_hub_entities WHERE canonical_id = $1"
        )
        .bind(canonical_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataEntity>, String> {
        let rows: Vec<DataEntityRow> = sqlx::query_as(
            "SELECT id, entity_type, source_system, canonical_id, data_quality_score, last_validated_at, status, created_at, updated_at FROM data_hub_entities WHERE entity_type = $1"
        )
        .bind(entity_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_quarantined(&self, limit: i64) -> Result<Vec<DataEntity>, String> {
        let rows: Vec<DataEntityRow> = sqlx::query_as(
            "SELECT id, entity_type, source_system, canonical_id, data_quality_score, last_validated_at, status, created_at, updated_at FROM data_hub_entities WHERE status = $1 LIMIT $2"
        )
        .bind("quarantined")
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &DataEntityId) -> Result<(), String> {
        sqlx::query("DELETE FROM data_hub_entities WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Quality Rule Repository ---

pub struct PgQualityRuleRepository {
    pool: PgPool,
}

impl PgQualityRuleRepository {
    pub fn new(pool: PgPool) -> Self {
        PgQualityRuleRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct QualityRuleRow {
    id: Uuid,
    entity_type: String,
    rule_name: String,
    rule_expression: String,
    severity: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl QualityRuleRow {
    fn into_domain(self) -> Result<DataQualityRule, String> {
        let id = DataQualityRuleId::from_uuid(self.id);
        let entity_type = DataEntityType::from_str(&self.entity_type)
            .map_err(|e| e.to_string())?;
        let severity = match self.severity.as_str() {
            "error" => RuleSeverity::Error,
            "warning" => RuleSeverity::Warning,
            "info" => RuleSeverity::Info,
            _ => return Err("Invalid severity".to_string()),
        };

        Ok(DataQualityRule::reconstitute(
            id,
            entity_type,
            self.rule_name,
            self.rule_expression,
            severity,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IQualityRuleRepository for PgQualityRuleRepository {
    async fn save(&self, rule: &DataQualityRule) -> Result<(), String> {
        let severity_str = match rule.severity() {
            RuleSeverity::Error => "error",
            RuleSeverity::Warning => "warning",
            RuleSeverity::Info => "info",
        };

        sqlx::query(
            r#"
            INSERT INTO data_hub_quality_rules (id, entity_type, rule_name, rule_expression, severity, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                is_active = $6,
                updated_at = $8
            "#
        )
        .bind(rule.id().as_uuid())
        .bind(rule.entity_type().to_string())
        .bind(rule.rule_name())
        .bind(rule.rule_expression())
        .bind(severity_str)
        .bind(rule.is_active())
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DataQualityRuleId) -> Result<Option<DataQualityRule>, String> {
        let row: Option<QualityRuleRow> = sqlx::query_as(
            "SELECT id, entity_type, rule_name, rule_expression, severity, is_active, created_at, updated_at FROM data_hub_quality_rules WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataQualityRule>, String> {
        let rows: Vec<QualityRuleRow> = sqlx::query_as(
            "SELECT id, entity_type, rule_name, rule_expression, severity, is_active, created_at, updated_at FROM data_hub_quality_rules WHERE entity_type = $1"
        )
        .bind(entity_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_active_rules(&self) -> Result<Vec<DataQualityRule>, String> {
        let rows: Vec<QualityRuleRow> = sqlx::query_as(
            "SELECT id, entity_type, rule_name, rule_expression, severity, is_active, created_at, updated_at FROM data_hub_quality_rules WHERE is_active = true"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &DataQualityRuleId) -> Result<(), String> {
        sqlx::query("DELETE FROM data_hub_quality_rules WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Data Lineage Repository ---

pub struct PgDataLineageRepository {
    pool: PgPool,
}

impl PgDataLineageRepository {
    pub fn new(pool: PgPool) -> Self {
        PgDataLineageRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DataLineageRow {
    id: Uuid,
    source_entity_id: Uuid,
    target_entity_id: Uuid,
    transformation_type: String,
    pipeline_name: String,
    last_run_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl DataLineageRow {
    fn into_domain(self) -> Result<DataLineage, String> {
        let id = DataLineageId::from_uuid(self.id);
        let source_entity_id = DataEntityId::from_uuid(self.source_entity_id);
        let target_entity_id = DataEntityId::from_uuid(self.target_entity_id);
        let transformation_type = TransformationType::from_str(&self.transformation_type)
            .map_err(|e| e.to_string())?;

        Ok(DataLineage::reconstitute(
            id,
            source_entity_id,
            target_entity_id,
            transformation_type,
            self.pipeline_name,
            self.last_run_at,
            self.created_at,
        ))
    }
}

#[async_trait]
impl IDataLineageRepository for PgDataLineageRepository {
    async fn save(&self, lineage: &DataLineage) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO data_hub_lineage (id, source_entity_id, target_entity_id, transformation_type, pipeline_name, last_run_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                last_run_at = $6
            "#
        )
        .bind(lineage.id().as_uuid())
        .bind(lineage.source_entity_id().as_uuid())
        .bind(lineage.target_entity_id().as_uuid())
        .bind(lineage.transformation_type().to_string())
        .bind(lineage.pipeline_name())
        .bind(lineage.last_run_at())
        .bind(lineage.id().as_uuid()) // Re-bind created_at
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DataLineageId) -> Result<Option<DataLineage>, String> {
        let row: Option<DataLineageRow> = sqlx::query_as(
            "SELECT id, source_entity_id, target_entity_id, transformation_type, pipeline_name, last_run_at, created_at FROM data_hub_lineage WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_source(&self, source_id: &DataEntityId) -> Result<Vec<DataLineage>, String> {
        let rows: Vec<DataLineageRow> = sqlx::query_as(
            "SELECT id, source_entity_id, target_entity_id, transformation_type, pipeline_name, last_run_at, created_at FROM data_hub_lineage WHERE source_entity_id = $1"
        )
        .bind(source_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_target(&self, target_id: &DataEntityId) -> Result<Vec<DataLineage>, String> {
        let rows: Vec<DataLineageRow> = sqlx::query_as(
            "SELECT id, source_entity_id, target_entity_id, transformation_type, pipeline_name, last_run_at, created_at FROM data_hub_lineage WHERE target_entity_id = $1"
        )
        .bind(target_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_pipeline(&self, pipeline_name: &str) -> Result<Vec<DataLineage>, String> {
        let rows: Vec<DataLineageRow> = sqlx::query_as(
            "SELECT id, source_entity_id, target_entity_id, transformation_type, pipeline_name, last_run_at, created_at FROM data_hub_lineage WHERE pipeline_name = $1"
        )
        .bind(pipeline_name)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &DataLineageId) -> Result<(), String> {
        sqlx::query("DELETE FROM data_hub_lineage WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Data Reconciliation Repository ---

pub struct PgDataReconciliationRepository {
    pool: PgPool,
}

impl PgDataReconciliationRepository {
    pub fn new(pool: PgPool) -> Self {
        PgDataReconciliationRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct DataReconciliationRow {
    id: Uuid,
    entity_type: String,
    source_a: String,
    source_b: String,
    discrepancies_found: Vec<String>,
    status: String,
    scheduled_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl DataReconciliationRow {
    fn into_domain(self) -> Result<DataReconciliation, String> {
        let id = DataReconciliationId::from_uuid(self.id);
        let entity_type = DataEntityType::from_str(&self.entity_type)
            .map_err(|e| e.to_string())?;
        let status = ReconciliationStatus::from_str(&self.status)
            .map_err(|e| e.to_string())?;

        Ok(DataReconciliation::reconstitute(
            id,
            entity_type,
            self.source_a,
            self.source_b,
            self.discrepancies_found,
            status,
            self.scheduled_at,
            self.completed_at,
            self.created_at,
        ))
    }
}

#[async_trait]
impl IDataReconciliationRepository for PgDataReconciliationRepository {
    async fn save(&self, reconciliation: &DataReconciliation) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO data_hub_reconciliation (id, entity_type, source_a, source_b, discrepancies_found, status, scheduled_at, completed_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                discrepancies_found = $5,
                status = $6,
                completed_at = $8
            "#
        )
        .bind(reconciliation.id().as_uuid())
        .bind(reconciliation.entity_type().to_string())
        .bind(reconciliation.source_a())
        .bind(reconciliation.source_b())
        .bind(reconciliation.discrepancies_found())
        .bind(reconciliation.status().to_string())
        .bind(reconciliation.scheduled_at())
        .bind(reconciliation.completed_at())
        .bind(reconciliation.id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DataReconciliationId) -> Result<Option<DataReconciliation>, String> {
        let row: Option<DataReconciliationRow> = sqlx::query_as(
            "SELECT id, entity_type, source_a, source_b, discrepancies_found, status, scheduled_at, completed_at, created_at FROM data_hub_reconciliation WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_pending(&self, limit: i64) -> Result<Vec<DataReconciliation>, String> {
        let rows: Vec<DataReconciliationRow> = sqlx::query_as(
            "SELECT id, entity_type, source_a, source_b, discrepancies_found, status, scheduled_at, completed_at, created_at FROM data_hub_reconciliation WHERE status = $1 LIMIT $2"
        )
        .bind("pending")
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataReconciliation>, String> {
        let rows: Vec<DataReconciliationRow> = sqlx::query_as(
            "SELECT id, entity_type, source_a, source_b, discrepancies_found, status, scheduled_at, completed_at, created_at FROM data_hub_reconciliation WHERE entity_type = $1"
        )
        .bind(entity_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_date_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<DataReconciliation>, String> {
        let rows: Vec<DataReconciliationRow> = sqlx::query_as(
            "SELECT id, entity_type, source_a, source_b, discrepancies_found, status, scheduled_at, completed_at, created_at FROM data_hub_reconciliation WHERE created_at BETWEEN $1 AND $2"
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &DataReconciliationId) -> Result<(), String> {
        sqlx::query("DELETE FROM data_hub_reconciliation WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Master Data Repository ---

pub struct PgMasterDataRepository {
    pool: PgPool,
}

impl PgMasterDataRepository {
    pub fn new(pool: PgPool) -> Self {
        PgMasterDataRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct MasterDataRecordRow {
    id: Uuid,
    entity_type: String,
    canonical_data: serde_json::Value,
    version: i32,
    is_golden_record: bool,
    sources: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MasterDataRecordRow {
    fn into_domain(self) -> Result<MasterDataRecord, String> {
        let id = MasterDataRecordId::from_uuid(self.id);
        let entity_type = DataEntityType::from_str(&self.entity_type)
            .map_err(|e| e.to_string())?;

        Ok(MasterDataRecord::reconstitute(
            id,
            entity_type,
            self.canonical_data,
            self.version as u32,
            self.is_golden_record,
            self.sources,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IMasterDataRepository for PgMasterDataRepository {
    async fn save(&self, record: &MasterDataRecord) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO data_hub_master_data (id, entity_type, canonical_data, version, is_golden_record, sources, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                canonical_data = $3,
                version = $4,
                is_golden_record = $5,
                sources = $6,
                updated_at = $8
            "#
        )
        .bind(record.id().as_uuid())
        .bind(record.entity_type().to_string())
        .bind(record.canonical_data())
        .bind(record.version() as i32)
        .bind(record.is_golden_record())
        .bind(record.sources())
        .bind(record.created_at())
        .bind(record.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &MasterDataRecordId) -> Result<Option<MasterDataRecord>, String> {
        let row: Option<MasterDataRecordRow> = sqlx::query_as(
            "SELECT id, entity_type, canonical_data, version, is_golden_record, sources, created_at, updated_at FROM data_hub_master_data WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_golden_records(&self, entity_type: DataEntityType) -> Result<Vec<MasterDataRecord>, String> {
        let rows: Vec<MasterDataRecordRow> = sqlx::query_as(
            "SELECT id, entity_type, canonical_data, version, is_golden_record, sources, created_at, updated_at FROM data_hub_master_data WHERE entity_type = $1 AND is_golden_record = true"
        )
        .bind(entity_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<MasterDataRecord>, String> {
        let rows: Vec<MasterDataRecordRow> = sqlx::query_as(
            "SELECT id, entity_type, canonical_data, version, is_golden_record, sources, created_at, updated_at FROM data_hub_master_data WHERE entity_type = $1"
        )
        .bind(entity_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_by_source(&self, source: &str) -> Result<Vec<MasterDataRecord>, String> {
        let rows: Vec<MasterDataRecordRow> = sqlx::query_as(
            "SELECT id, entity_type, canonical_data, version, is_golden_record, sources, created_at, updated_at FROM data_hub_master_data WHERE $1 = ANY(sources)"
        )
        .bind(source)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &MasterDataRecordId) -> Result<(), String> {
        sqlx::query("DELETE FROM data_hub_master_data WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// --- Governance Policy Repository ---

pub struct PgGovernancePolicyRepository {
    pool: PgPool,
}

impl PgGovernancePolicyRepository {
    pub fn new(pool: PgPool) -> Self {
        PgGovernancePolicyRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct GovernancePolicyRow {
    id: Uuid,
    name: String,
    description: String,
    entity_types_covered: Vec<String>,
    retention_days: i32,
    classification: String,
    owner_team: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl GovernancePolicyRow {
    fn into_domain(self) -> Result<DataGovernancePolicy, String> {
        let id = DataGovernancePolicyId::from_uuid(self.id);
        let entity_types: Result<Vec<DataEntityType>, _> = self
            .entity_types_covered
            .iter()
            .map(|t| DataEntityType::from_str(t))
            .collect();
        let entity_types = entity_types.map_err(|e| e.to_string())?;
        let classification = DataClassification::from_str(&self.classification)
            .map_err(|e| e.to_string())?;

        Ok(DataGovernancePolicy::reconstitute(
            id,
            self.name,
            self.description,
            entity_types,
            self.retention_days as u32,
            classification,
            self.owner_team,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IGovernancePolicyRepository for PgGovernancePolicyRepository {
    async fn save(&self, policy: &DataGovernancePolicy) -> Result<(), String> {
        let entity_types_strs: Vec<String> = policy
            .entity_types_covered()
            .iter()
            .map(|t| t.to_string())
            .collect();

        sqlx::query(
            r#"
            INSERT INTO data_hub_policies (id, name, description, entity_types_covered, retention_days, classification, owner_team, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                is_active = $8,
                updated_at = $10
            "#
        )
        .bind(policy.id().as_uuid())
        .bind(policy.name())
        .bind(policy.description())
        .bind(entity_types_strs)
        .bind(policy.retention_days() as i32)
        .bind(policy.classification().to_string())
        .bind(policy.owner_team())
        .bind(policy.is_active())
        .bind(policy.created_at())
        .bind(policy.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DataGovernancePolicyId) -> Result<Option<DataGovernancePolicy>, String> {
        let row: Option<GovernancePolicyRow> = sqlx::query_as(
            "SELECT id, name, description, entity_types_covered, retention_days, classification, owner_team, is_active, created_at, updated_at FROM data_hub_policies WHERE id = $1"
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataGovernancePolicy>, String> {
        let rows: Vec<GovernancePolicyRow> = sqlx::query_as(
            "SELECT id, name, description, entity_types_covered, retention_days, classification, owner_team, is_active, created_at, updated_at FROM data_hub_policies WHERE $1 = ANY(entity_types_covered)"
        )
        .bind(entity_type.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn find_active_policies(&self) -> Result<Vec<DataGovernancePolicy>, String> {
        let rows: Vec<GovernancePolicyRow> = sqlx::query_as(
            "SELECT id, name, description, entity_types_covered, retention_days, classification, owner_team, is_active, created_at, updated_at FROM data_hub_policies WHERE is_active = true"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.into_iter()
            .map(|r| r.into_domain())
            .collect()
    }

    async fn delete(&self, id: &DataGovernancePolicyId) -> Result<(), String> {
        sqlx::query("DELETE FROM data_hub_policies WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
