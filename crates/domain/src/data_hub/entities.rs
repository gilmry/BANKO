use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use super::value_objects::{
    DataEntityId, DataEntityType, DataQualityRuleId, DataQualityScore, DataLineageId,
    DataReconciliationId, MasterDataRecordId, DataGovernancePolicyId, ReconciliationStatus,
    TransformationType, DataClassification, DataEntityStatus,
};

// --- DataEntity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntity {
    id: DataEntityId,
    entity_type: DataEntityType,
    source_system: String,
    canonical_id: String,
    data_quality_score: DataQualityScore,
    last_validated_at: DateTime<Utc>,
    status: DataEntityStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DataEntity {
    pub fn new(
        entity_type: DataEntityType,
        source_system: &str,
        canonical_id: &str,
        initial_quality_score: u8,
    ) -> Result<Self, DomainError> {
        let quality_score = DataQualityScore::new(initial_quality_score)?;
        let status = if quality_score.is_quarantined() {
            DataEntityStatus::Quarantined
        } else {
            DataEntityStatus::Active
        };

        Ok(DataEntity {
            id: DataEntityId::new(),
            entity_type,
            source_system: source_system.to_string(),
            canonical_id: canonical_id.to_string(),
            data_quality_score: quality_score,
            last_validated_at: Utc::now(),
            status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: DataEntityId,
        entity_type: DataEntityType,
        source_system: String,
        canonical_id: String,
        data_quality_score: DataQualityScore,
        last_validated_at: DateTime<Utc>,
        status: DataEntityStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        DataEntity {
            id,
            entity_type,
            source_system,
            canonical_id,
            data_quality_score,
            last_validated_at,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn update_quality_score(&mut self, new_score: u8) -> Result<(), DomainError> {
        let quality_score = DataQualityScore::new(new_score)?;
        self.data_quality_score = quality_score;
        self.last_validated_at = Utc::now();
        self.status = if self.data_quality_score.is_quarantined() {
            DataEntityStatus::Quarantined
        } else if self.status == DataEntityStatus::Quarantined {
            DataEntityStatus::Active
        } else {
            self.status
        };
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_stale(&mut self) {
        self.status = DataEntityStatus::Stale;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &DataEntityId {
        &self.id
    }

    pub fn entity_type(&self) -> DataEntityType {
        self.entity_type
    }

    pub fn source_system(&self) -> &str {
        &self.source_system
    }

    pub fn canonical_id(&self) -> &str {
        &self.canonical_id
    }

    pub fn data_quality_score(&self) -> &DataQualityScore {
        &self.data_quality_score
    }

    pub fn last_validated_at(&self) -> DateTime<Utc> {
        self.last_validated_at
    }

    pub fn status(&self) -> DataEntityStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- DataQualityRule ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityRule {
    id: DataQualityRuleId,
    entity_type: DataEntityType,
    rule_name: String,
    rule_expression: String,
    severity: RuleSeverity,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleSeverity {
    Error,
    Warning,
    Info,
}

impl DataQualityRule {
    pub fn new(
        entity_type: DataEntityType,
        rule_name: &str,
        rule_expression: &str,
        severity: RuleSeverity,
    ) -> Result<Self, DomainError> {
        if rule_name.is_empty() {
            return Err(DomainError::ValidationError(
                "Rule name cannot be empty".to_string(),
            ));
        }
        if rule_expression.is_empty() {
            return Err(DomainError::ValidationError(
                "Rule expression cannot be empty".to_string(),
            ));
        }

        Ok(DataQualityRule {
            id: DataQualityRuleId::new(),
            entity_type,
            rule_name: rule_name.to_string(),
            rule_expression: rule_expression.to_string(),
            severity,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: DataQualityRuleId,
        entity_type: DataEntityType,
        rule_name: String,
        rule_expression: String,
        severity: RuleSeverity,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        DataQualityRule {
            id,
            entity_type,
            rule_name,
            rule_expression,
            severity,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &DataQualityRuleId {
        &self.id
    }

    pub fn entity_type(&self) -> DataEntityType {
        self.entity_type
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn rule_expression(&self) -> &str {
        &self.rule_expression
    }

    pub fn severity(&self) -> RuleSeverity {
        self.severity
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

// --- DataLineage ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    id: DataLineageId,
    source_entity_id: DataEntityId,
    target_entity_id: DataEntityId,
    transformation_type: TransformationType,
    pipeline_name: String,
    last_run_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl DataLineage {
    pub fn new(
        source_entity_id: DataEntityId,
        target_entity_id: DataEntityId,
        transformation_type: TransformationType,
        pipeline_name: &str,
    ) -> Result<Self, DomainError> {
        if pipeline_name.is_empty() {
            return Err(DomainError::ValidationError(
                "Pipeline name cannot be empty".to_string(),
            ));
        }

        Ok(DataLineage {
            id: DataLineageId::new(),
            source_entity_id,
            target_entity_id,
            transformation_type,
            pipeline_name: pipeline_name.to_string(),
            last_run_at: Utc::now(),
            created_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: DataLineageId,
        source_entity_id: DataEntityId,
        target_entity_id: DataEntityId,
        transformation_type: TransformationType,
        pipeline_name: String,
        last_run_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        DataLineage {
            id,
            source_entity_id,
            target_entity_id,
            transformation_type,
            pipeline_name,
            last_run_at,
            created_at,
        }
    }

    pub fn update_run_timestamp(&mut self) {
        self.last_run_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &DataLineageId {
        &self.id
    }

    pub fn source_entity_id(&self) -> &DataEntityId {
        &self.source_entity_id
    }

    pub fn target_entity_id(&self) -> &DataEntityId {
        &self.target_entity_id
    }

    pub fn transformation_type(&self) -> TransformationType {
        self.transformation_type
    }

    pub fn pipeline_name(&self) -> &str {
        &self.pipeline_name
    }

    pub fn last_run_at(&self) -> DateTime<Utc> {
        self.last_run_at
    }
}

// --- DataReconciliation ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataReconciliation {
    id: DataReconciliationId,
    entity_type: DataEntityType,
    source_a: String,
    source_b: String,
    discrepancies_found: Vec<String>,
    status: ReconciliationStatus,
    scheduled_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl DataReconciliation {
    pub fn new(
        entity_type: DataEntityType,
        source_a: &str,
        source_b: &str,
        scheduled_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if source_a.is_empty() || source_b.is_empty() {
            return Err(DomainError::ValidationError(
                "Sources cannot be empty".to_string(),
            ));
        }

        Ok(DataReconciliation {
            id: DataReconciliationId::new(),
            entity_type,
            source_a: source_a.to_string(),
            source_b: source_b.to_string(),
            discrepancies_found: Vec::new(),
            status: ReconciliationStatus::Pending,
            scheduled_at,
            completed_at: None,
            created_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: DataReconciliationId,
        entity_type: DataEntityType,
        source_a: String,
        source_b: String,
        discrepancies_found: Vec<String>,
        status: ReconciliationStatus,
        scheduled_at: DateTime<Utc>,
        completed_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    ) -> Self {
        DataReconciliation {
            id,
            entity_type,
            source_a,
            source_b,
            discrepancies_found,
            status,
            scheduled_at,
            completed_at,
            created_at,
        }
    }

    pub fn start(&mut self) -> Result<(), DomainError> {
        if self.status != ReconciliationStatus::Pending {
            return Err(DomainError::ValidationError(
                "Can only start pending reconciliations".to_string(),
            ));
        }
        self.status = ReconciliationStatus::InProgress;
        Ok(())
    }

    pub fn add_discrepancy(&mut self, discrepancy: String) {
        self.discrepancies_found.push(discrepancy);
    }

    pub fn resolve(&mut self) -> Result<(), DomainError> {
        if self.status != ReconciliationStatus::InProgress {
            return Err(DomainError::ValidationError(
                "Can only resolve in-progress reconciliations".to_string(),
            ));
        }
        self.status = ReconciliationStatus::Resolved;
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    pub fn escalate(&mut self) -> Result<(), DomainError> {
        if self.status == ReconciliationStatus::Resolved {
            return Err(DomainError::ValidationError(
                "Cannot escalate resolved reconciliation".to_string(),
            ));
        }
        self.status = ReconciliationStatus::Escalated;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &DataReconciliationId {
        &self.id
    }

    pub fn entity_type(&self) -> DataEntityType {
        self.entity_type
    }

    pub fn source_a(&self) -> &str {
        &self.source_a
    }

    pub fn source_b(&self) -> &str {
        &self.source_b
    }

    pub fn discrepancies_found(&self) -> &[String] {
        &self.discrepancies_found
    }

    pub fn status(&self) -> ReconciliationStatus {
        self.status
    }

    pub fn scheduled_at(&self) -> DateTime<Utc> {
        self.scheduled_at
    }

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }
}

// --- MasterDataRecord ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterDataRecord {
    id: MasterDataRecordId,
    entity_type: DataEntityType,
    canonical_data: serde_json::Value,
    version: u32,
    is_golden_record: bool,
    sources: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MasterDataRecord {
    pub fn new(
        entity_type: DataEntityType,
        canonical_data: serde_json::Value,
        sources: Vec<String>,
    ) -> Result<Self, DomainError> {
        if sources.is_empty() {
            return Err(DomainError::ValidationError(
                "At least one source is required".to_string(),
            ));
        }

        Ok(MasterDataRecord {
            id: MasterDataRecordId::new(),
            entity_type,
            canonical_data,
            version: 1,
            is_golden_record: false,
            sources,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: MasterDataRecordId,
        entity_type: DataEntityType,
        canonical_data: serde_json::Value,
        version: u32,
        is_golden_record: bool,
        sources: Vec<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        MasterDataRecord {
            id,
            entity_type,
            canonical_data,
            version,
            is_golden_record,
            sources,
            created_at,
            updated_at,
        }
    }

    pub fn update_data(&mut self, new_data: serde_json::Value) -> Result<(), DomainError> {
        self.canonical_data = new_data;
        self.version += 1;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_as_golden(&mut self) -> Result<(), DomainError> {
        if self.version < 1 {
            return Err(DomainError::ValidationError(
                "Cannot mark unversioned record as golden".to_string(),
            ));
        }
        self.is_golden_record = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn add_source(&mut self, source: String) {
        if !self.sources.contains(&source) {
            self.sources.push(source);
            self.updated_at = Utc::now();
        }
    }

    // Getters
    pub fn id(&self) -> &MasterDataRecordId {
        &self.id
    }

    pub fn entity_type(&self) -> DataEntityType {
        self.entity_type
    }

    pub fn canonical_data(&self) -> &serde_json::Value {
        &self.canonical_data
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn is_golden_record(&self) -> bool {
        self.is_golden_record
    }

    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- DataGovernancePolicy ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGovernancePolicy {
    id: DataGovernancePolicyId,
    name: String,
    description: String,
    entity_types_covered: Vec<DataEntityType>,
    retention_days: u32,
    classification: DataClassification,
    owner_team: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DataGovernancePolicy {
    pub fn new(
        name: &str,
        description: &str,
        entity_types_covered: Vec<DataEntityType>,
        retention_days: u32,
        classification: DataClassification,
        owner_team: &str,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Policy name cannot be empty".to_string(),
            ));
        }
        if entity_types_covered.is_empty() {
            return Err(DomainError::ValidationError(
                "At least one entity type must be covered".to_string(),
            ));
        }
        // Financial data retention: minimum 7 years (365 * 7 = 2555 days)
        if retention_days < 2555 && classification == DataClassification::Restricted {
            return Err(DomainError::ValidationError(
                "Restricted data must be retained for at least 7 years".to_string(),
            ));
        }

        Ok(DataGovernancePolicy {
            id: DataGovernancePolicyId::new(),
            name: name.to_string(),
            description: description.to_string(),
            entity_types_covered,
            retention_days,
            classification,
            owner_team: owner_team.to_string(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: DataGovernancePolicyId,
        name: String,
        description: String,
        entity_types_covered: Vec<DataEntityType>,
        retention_days: u32,
        classification: DataClassification,
        owner_team: String,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        DataGovernancePolicy {
            id,
            name,
            description,
            entity_types_covered,
            retention_days,
            classification,
            owner_team,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &DataGovernancePolicyId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn entity_types_covered(&self) -> &[DataEntityType] {
        &self.entity_types_covered
    }

    pub fn retention_days(&self) -> u32 {
        self.retention_days
    }

    pub fn classification(&self) -> DataClassification {
        self.classification
    }

    pub fn owner_team(&self) -> &str {
        &self.owner_team
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_entity_creation() {
        let entity = DataEntity::new(
            DataEntityType::Customer,
            "CORE_SYSTEM",
            "cust_12345",
            75,
        );
        assert!(entity.is_ok());
        let entity = entity.unwrap();
        assert_eq!(entity.entity_type(), DataEntityType::Customer);
        assert_eq!(entity.source_system(), "CORE_SYSTEM");
        assert_eq!(entity.canonical_id(), "cust_12345");
        assert_eq!(entity.status(), DataEntityStatus::Active);
    }

    #[test]
    fn test_data_entity_quarantine_on_low_quality() {
        let entity = DataEntity::new(
            DataEntityType::Account,
            "LEGACY_SYSTEM",
            "acct_001",
            45,
        );
        assert!(entity.is_ok());
        let entity = entity.unwrap();
        assert_eq!(entity.status(), DataEntityStatus::Quarantined);
    }

    #[test]
    fn test_data_entity_quality_score_update() {
        let mut entity = DataEntity::new(
            DataEntityType::Transaction,
            "PAYMENT_SYS",
            "txn_999",
            30,
        ).unwrap();
        assert_eq!(entity.status(), DataEntityStatus::Quarantined);

        entity.update_quality_score(85).unwrap();
        assert_eq!(entity.status(), DataEntityStatus::Active);
        assert_eq!(entity.data_quality_score().value(), 85);
    }

    #[test]
    fn test_data_entity_mark_stale() {
        let mut entity = DataEntity::new(
            DataEntityType::Customer,
            "CORE",
            "c123",
            80,
        ).unwrap();
        entity.mark_stale();
        assert_eq!(entity.status(), DataEntityStatus::Stale);
    }

    #[test]
    fn test_data_quality_rule_creation() {
        let rule = DataQualityRule::new(
            DataEntityType::Customer,
            "email_format",
            "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
            RuleSeverity::Error,
        );
        assert!(rule.is_ok());
        let rule = rule.unwrap();
        assert!(rule.is_active());
        assert_eq!(rule.rule_name(), "email_format");
    }

    #[test]
    fn test_data_quality_rule_empty_name() {
        let rule = DataQualityRule::new(
            DataEntityType::Account,
            "",
            "some_expression",
            RuleSeverity::Warning,
        );
        assert!(rule.is_err());
    }

    #[test]
    fn test_data_quality_rule_deactivate() {
        let mut rule = DataQualityRule::new(
            DataEntityType::Product,
            "test_rule",
            "test_expr",
            RuleSeverity::Info,
        ).unwrap();
        rule.deactivate();
        assert!(!rule.is_active());
        rule.activate();
        assert!(rule.is_active());
    }

    #[test]
    fn test_data_lineage_creation() {
        let source_id = DataEntityId::new();
        let target_id = DataEntityId::new();
        let lineage = DataLineage::new(
            source_id.clone(),
            target_id.clone(),
            TransformationType::Direct,
            "etl_pipeline_v1",
        );
        assert!(lineage.is_ok());
        let lineage = lineage.unwrap();
        assert_eq!(lineage.source_entity_id(), &source_id);
        assert_eq!(lineage.target_entity_id(), &target_id);
        assert_eq!(lineage.transformation_type(), TransformationType::Direct);
    }

    #[test]
    fn test_data_lineage_empty_pipeline() {
        let lineage = DataLineage::new(
            DataEntityId::new(),
            DataEntityId::new(),
            TransformationType::Aggregated,
            "",
        );
        assert!(lineage.is_err());
    }

    #[test]
    fn test_data_lineage_update_run_timestamp() {
        let mut lineage = DataLineage::new(
            DataEntityId::new(),
            DataEntityId::new(),
            TransformationType::Computed,
            "pipeline",
        ).unwrap();
        let first_run = lineage.last_run_at();
        std::thread::sleep(std::time::Duration::from_millis(10));
        lineage.update_run_timestamp();
        let second_run = lineage.last_run_at();
        assert!(second_run > first_run);
    }

    #[test]
    fn test_data_reconciliation_creation() {
        let now = Utc::now();
        let recon = DataReconciliation::new(
            DataEntityType::Customer,
            "SOURCE_A",
            "SOURCE_B",
            now,
        );
        assert!(recon.is_ok());
        let recon = recon.unwrap();
        assert_eq!(recon.status(), ReconciliationStatus::Pending);
        assert_eq!(recon.source_a(), "SOURCE_A");
        assert_eq!(recon.source_b(), "SOURCE_B");
    }

    #[test]
    fn test_data_reconciliation_workflow() {
        let now = Utc::now();
        let mut recon = DataReconciliation::new(
            DataEntityType::Account,
            "DB1",
            "DB2",
            now,
        ).unwrap();

        assert!(recon.start().is_ok());
        assert_eq!(recon.status(), ReconciliationStatus::InProgress);

        recon.add_discrepancy("Balance mismatch: 1000 vs 900".to_string());
        assert_eq!(recon.discrepancies_found().len(), 1);

        assert!(recon.resolve().is_ok());
        assert_eq!(recon.status(), ReconciliationStatus::Resolved);
        assert!(recon.completed_at().is_some());
    }

    #[test]
    fn test_data_reconciliation_escalate() {
        let mut recon = DataReconciliation::new(
            DataEntityType::Transaction,
            "LEDGER_1",
            "LEDGER_2",
            Utc::now(),
        ).unwrap();
        recon.start().unwrap();
        assert!(recon.escalate().is_ok());
        assert_eq!(recon.status(), ReconciliationStatus::Escalated);
    }

    #[test]
    fn test_data_reconciliation_cannot_start_twice() {
        let mut recon = DataReconciliation::new(
            DataEntityType::Customer,
            "S1",
            "S2",
            Utc::now(),
        ).unwrap();
        assert!(recon.start().is_ok());
        assert!(recon.start().is_err());
    }

    #[test]
    fn test_master_data_record_creation() {
        let data = serde_json::json!({
            "name": "John Doe",
            "email": "john@example.com"
        });
        let sources = vec!["CORE_CRM".to_string(), "FINANCE_SYS".to_string()];
        let record = MasterDataRecord::new(DataEntityType::Customer, data.clone(), sources);
        assert!(record.is_ok());
        let record = record.unwrap();
        assert!(!record.is_golden_record());
        assert_eq!(record.version(), 1);
        assert_eq!(record.sources().len(), 2);
    }

    #[test]
    fn test_master_data_record_no_sources() {
        let data = serde_json::json!({"test": "data"});
        let record = MasterDataRecord::new(DataEntityType::Product, data, vec![]);
        assert!(record.is_err());
    }

    #[test]
    fn test_master_data_record_update_data() {
        let mut record = MasterDataRecord::new(
            DataEntityType::Account,
            serde_json::json!({"balance": 1000}),
            vec!["SYSTEM_1".to_string()],
        ).unwrap();

        let v1 = record.version();
        record.update_data(serde_json::json!({"balance": 2000})).unwrap();
        let v2 = record.version();

        assert_eq!(v2, v1 + 1);
    }

    #[test]
    fn test_master_data_record_mark_as_golden() {
        let mut record = MasterDataRecord::new(
            DataEntityType::Customer,
            serde_json::json!({"data": "value"}),
            vec!["SYS".to_string()],
        ).unwrap();

        record.mark_as_golden().unwrap();
        assert!(record.is_golden_record());
    }

    #[test]
    fn test_master_data_record_add_source() {
        let mut record = MasterDataRecord::new(
            DataEntityType::Transaction,
            serde_json::json!({}),
            vec!["SOURCE_A".to_string()],
        ).unwrap();

        assert_eq!(record.sources().len(), 1);
        record.add_source("SOURCE_B".to_string());
        assert_eq!(record.sources().len(), 2);

        // Adding same source twice should not duplicate
        record.add_source("SOURCE_B".to_string());
        assert_eq!(record.sources().len(), 2);
    }

    #[test]
    fn test_data_governance_policy_creation() {
        let entity_types = vec![DataEntityType::Customer, DataEntityType::Account];
        let policy = DataGovernancePolicy::new(
            "Customer Data Policy",
            "Governs handling of customer data",
            entity_types,
            2555,
            DataClassification::Restricted,
            "Data Governance Team",
        );
        assert!(policy.is_ok());
        let policy = policy.unwrap();
        assert!(policy.is_active());
        assert_eq!(policy.retention_days(), 2555);
    }

    #[test]
    fn test_data_governance_policy_empty_name() {
        let policy = DataGovernancePolicy::new(
            "",
            "description",
            vec![DataEntityType::Customer],
            2555,
            DataClassification::Internal,
            "team",
        );
        assert!(policy.is_err());
    }

    #[test]
    fn test_data_governance_policy_no_entity_types() {
        let policy = DataGovernancePolicy::new(
            "Policy",
            "desc",
            vec![],
            2555,
            DataClassification::Public,
            "team",
        );
        assert!(policy.is_err());
    }

    #[test]
    fn test_data_governance_policy_retention_minimum() {
        // Restricted data must be retained for minimum 7 years
        let policy = DataGovernancePolicy::new(
            "Policy",
            "desc",
            vec![DataEntityType::Customer],
            1000, // Less than 7 years
            DataClassification::Restricted,
            "team",
        );
        assert!(policy.is_err());

        // Public data can have shorter retention
        let policy = DataGovernancePolicy::new(
            "Policy",
            "desc",
            vec![DataEntityType::Product],
            365,
            DataClassification::Public,
            "team",
        );
        assert!(policy.is_ok());
    }

    #[test]
    fn test_data_governance_policy_deactivate() {
        let mut policy = DataGovernancePolicy::new(
            "Policy",
            "desc",
            vec![DataEntityType::Customer],
            2555,
            DataClassification::Confidential,
            "team",
        ).unwrap();

        policy.deactivate();
        assert!(!policy.is_active());
        policy.activate();
        assert!(policy.is_active());
    }

    #[test]
    fn test_data_quality_score_bounds() {
        // Valid scores
        assert!(DataQualityScore::new(0).is_ok());
        assert!(DataQualityScore::new(50).is_ok());
        assert!(DataQualityScore::new(100).is_ok());

        // Invalid scores
        assert!(DataQualityScore::new(101).is_err());
        assert!(DataQualityScore::new(255).is_err());
    }

    #[test]
    fn test_data_quality_score_quarantine_threshold() {
        let low_score = DataQualityScore::new(40).unwrap();
        assert!(low_score.is_quarantined());

        let ok_score = DataQualityScore::new(60).unwrap();
        assert!(!ok_score.is_quarantined());
    }
}
