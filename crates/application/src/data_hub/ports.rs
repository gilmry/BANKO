use async_trait::async_trait;
use chrono::{DateTime, Utc};

use banko_domain::data_hub::{
    DataEntity, DataEntityId, DataGovernancePolicy, DataGovernancePolicyId, DataLineage,
    DataLineageId, DataQualityRule, DataQualityRuleId, DataReconciliation,
    DataReconciliationId, MasterDataRecord, MasterDataRecordId, DataEntityType,
};

/// Port for data entity persistence
#[async_trait]
pub trait IDataEntityRepository: Send + Sync {
    async fn save(&self, entity: &DataEntity) -> Result<(), String>;
    async fn find_by_id(&self, id: &DataEntityId) -> Result<Option<DataEntity>, String>;
    async fn find_by_canonical_id(&self, canonical_id: &str) -> Result<Vec<DataEntity>, String>;
    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataEntity>, String>;
    async fn find_quarantined(&self, limit: i64) -> Result<Vec<DataEntity>, String>;
    async fn delete(&self, id: &DataEntityId) -> Result<(), String>;
}

/// Port for quality rule persistence
#[async_trait]
pub trait IQualityRuleRepository: Send + Sync {
    async fn save(&self, rule: &DataQualityRule) -> Result<(), String>;
    async fn find_by_id(&self, id: &DataQualityRuleId) -> Result<Option<DataQualityRule>, String>;
    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataQualityRule>, String>;
    async fn find_active_rules(&self) -> Result<Vec<DataQualityRule>, String>;
    async fn delete(&self, id: &DataQualityRuleId) -> Result<(), String>;
}

/// Port for data lineage persistence
#[async_trait]
pub trait IDataLineageRepository: Send + Sync {
    async fn save(&self, lineage: &DataLineage) -> Result<(), String>;
    async fn find_by_id(&self, id: &DataLineageId) -> Result<Option<DataLineage>, String>;
    async fn find_by_source(&self, source_id: &DataEntityId) -> Result<Vec<DataLineage>, String>;
    async fn find_by_target(&self, target_id: &DataEntityId) -> Result<Vec<DataLineage>, String>;
    async fn find_by_pipeline(&self, pipeline_name: &str) -> Result<Vec<DataLineage>, String>;
    async fn delete(&self, id: &DataLineageId) -> Result<(), String>;
}

/// Port for data reconciliation persistence
#[async_trait]
pub trait IDataReconciliationRepository: Send + Sync {
    async fn save(&self, reconciliation: &DataReconciliation) -> Result<(), String>;
    async fn find_by_id(&self, id: &DataReconciliationId) -> Result<Option<DataReconciliation>, String>;
    async fn find_pending(&self, limit: i64) -> Result<Vec<DataReconciliation>, String>;
    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataReconciliation>, String>;
    async fn find_by_date_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<DataReconciliation>, String>;
    async fn delete(&self, id: &DataReconciliationId) -> Result<(), String>;
}

/// Port for master data record persistence
#[async_trait]
pub trait IMasterDataRepository: Send + Sync {
    async fn save(&self, record: &MasterDataRecord) -> Result<(), String>;
    async fn find_by_id(&self, id: &MasterDataRecordId) -> Result<Option<MasterDataRecord>, String>;
    async fn find_golden_records(&self, entity_type: DataEntityType) -> Result<Vec<MasterDataRecord>, String>;
    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<MasterDataRecord>, String>;
    async fn find_by_source(&self, source: &str) -> Result<Vec<MasterDataRecord>, String>;
    async fn delete(&self, id: &MasterDataRecordId) -> Result<(), String>;
}

/// Port for governance policy persistence
#[async_trait]
pub trait IGovernancePolicyRepository: Send + Sync {
    async fn save(&self, policy: &DataGovernancePolicy) -> Result<(), String>;
    async fn find_by_id(&self, id: &DataGovernancePolicyId) -> Result<Option<DataGovernancePolicy>, String>;
    async fn find_by_entity_type(&self, entity_type: DataEntityType) -> Result<Vec<DataGovernancePolicy>, String>;
    async fn find_active_policies(&self) -> Result<Vec<DataGovernancePolicy>, String>;
    async fn delete(&self, id: &DataGovernancePolicyId) -> Result<(), String>;
}

/// Port for data quality validation
#[async_trait]
pub trait IDataQualityValidator: Send + Sync {
    async fn validate_entity(&self, entity_id: &DataEntityId) -> Result<u8, String>;
    async fn validate_against_rules(
        &self,
        entity_id: &DataEntityId,
        rules: &[DataQualityRule],
    ) -> Result<Vec<String>, String>;
}

/// Port for reconciliation engine
#[async_trait]
pub trait IReconciliationEngine: Send + Sync {
    async fn execute_reconciliation(
        &self,
        reconciliation_id: &DataReconciliationId,
    ) -> Result<Vec<String>, String>;
}
