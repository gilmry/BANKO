use std::sync::Arc;

use banko_domain::data_hub::{
    DataEntity, DataEntityId, DataEntityType, DataGovernancePolicy, DataLineage,
    DataQualityRule, DataReconciliation, MasterDataRecord, RuleSeverity,
    DataClassification, TransformationType,
};

use super::dto::{
    DataEntityRequest, DataEntityResponse, DataQualityRuleRequest, DataQualityRuleResponse,
    DataLineageRequest, DataLineageResponse, DataReconciliationRequest,
    DataReconciliationResponse, MasterDataRecordRequest, MasterDataRecordResponse,
    DataGovernancePolicyRequest, DataGovernancePolicyResponse,
};
use super::errors::DataHubError;
use super::ports::{
    IDataEntityRepository, IQualityRuleRepository, IDataLineageRepository,
    IDataReconciliationRepository, IMasterDataRepository, IGovernancePolicyRepository,
    IDataQualityValidator, IReconciliationEngine,
};

pub struct DataHubService {
    entity_repo: Arc<dyn IDataEntityRepository>,
    rule_repo: Arc<dyn IQualityRuleRepository>,
    lineage_repo: Arc<dyn IDataLineageRepository>,
    reconciliation_repo: Arc<dyn IDataReconciliationRepository>,
    master_data_repo: Arc<dyn IMasterDataRepository>,
    policy_repo: Arc<dyn IGovernancePolicyRepository>,
    _quality_validator: Arc<dyn IDataQualityValidator>,
    reconciliation_engine: Arc<dyn IReconciliationEngine>,
}

impl DataHubService {
    pub fn new(
        entity_repo: Arc<dyn IDataEntityRepository>,
        rule_repo: Arc<dyn IQualityRuleRepository>,
        lineage_repo: Arc<dyn IDataLineageRepository>,
        reconciliation_repo: Arc<dyn IDataReconciliationRepository>,
        master_data_repo: Arc<dyn IMasterDataRepository>,
        policy_repo: Arc<dyn IGovernancePolicyRepository>,
        quality_validator: Arc<dyn IDataQualityValidator>,
        reconciliation_engine: Arc<dyn IReconciliationEngine>,
    ) -> Self {
        DataHubService {
            entity_repo,
            rule_repo,
            lineage_repo,
            reconciliation_repo,
            master_data_repo,
            policy_repo,
            _quality_validator: quality_validator,
            reconciliation_engine,
        }
    }

    // --- Data Entity Operations ---

    pub async fn create_data_entity(
        &self,
        request: DataEntityRequest,
    ) -> Result<DataEntityResponse, DataHubError> {
        let entity_type =
            DataEntityType::from_str(&request.entity_type)
                .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let entity = DataEntity::new(
            entity_type,
            &request.source_system,
            &request.canonical_id,
            request.initial_quality_score,
        )
        .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.entity_repo
            .save(&entity)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.entity_to_response(&entity))
    }

    pub async fn find_data_entity(
        &self,
        id: &str,
    ) -> Result<DataEntityResponse, DataHubError> {
        let entity_id = DataEntityId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let entity = self
            .entity_repo
            .find_by_id(&entity_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::DataEntityNotFound)?;

        Ok(self.entity_to_response(&entity))
    }

    pub async fn update_quality_score(
        &self,
        id: &str,
        new_score: u8,
    ) -> Result<DataEntityResponse, DataHubError> {
        let entity_id = DataEntityId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let mut entity = self
            .entity_repo
            .find_by_id(&entity_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::DataEntityNotFound)?;

        entity
            .update_quality_score(new_score)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.entity_repo
            .save(&entity)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.entity_to_response(&entity))
    }

    pub async fn list_quarantined_entities(
        &self,
        limit: i64,
    ) -> Result<Vec<DataEntityResponse>, DataHubError> {
        let entities = self
            .entity_repo
            .find_quarantined(limit)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(entities
            .into_iter()
            .map(|e| self.entity_to_response(&e))
            .collect())
    }

    // --- Quality Rule Operations ---

    pub async fn create_quality_rule(
        &self,
        request: DataQualityRuleRequest,
    ) -> Result<DataQualityRuleResponse, DataHubError> {
        let entity_type =
            DataEntityType::from_str(&request.entity_type)
                .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let severity = match request.severity.as_str() {
            "error" => RuleSeverity::Error,
            "warning" => RuleSeverity::Warning,
            "info" => RuleSeverity::Info,
            _ => return Err(DataHubError::InvalidInput(
                "Invalid severity".to_string(),
            )),
        };

        let rule = DataQualityRule::new(
            entity_type,
            &request.rule_name,
            &request.rule_expression,
            severity,
        )
        .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.rule_repo
            .save(&rule)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.rule_to_response(&rule))
    }

    pub async fn find_quality_rule(
        &self,
        id: &str,
    ) -> Result<DataQualityRuleResponse, DataHubError> {
        let rule_id = banko_domain::data_hub::DataQualityRuleId::parse(&format!("rule_{}", id))
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let rule = self
            .rule_repo
            .find_by_id(&rule_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::RuleNotFound)?;

        Ok(self.rule_to_response(&rule))
    }

    pub async fn list_active_rules(
        &self,
    ) -> Result<Vec<DataQualityRuleResponse>, DataHubError> {
        let rules = self
            .rule_repo
            .find_active_rules()
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(rules
            .into_iter()
            .map(|r| self.rule_to_response(&r))
            .collect())
    }

    // --- Data Lineage Operations ---

    pub async fn create_data_lineage(
        &self,
        request: DataLineageRequest,
    ) -> Result<DataLineageResponse, DataHubError> {
        let source_id = DataEntityId::parse(&request.source_entity_id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;
        let target_id = DataEntityId::parse(&request.target_entity_id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let transformation_type = TransformationType::from_str(&request.transformation_type)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let lineage = DataLineage::new(source_id, target_id, transformation_type, &request.pipeline_name)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.lineage_repo
            .save(&lineage)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.lineage_to_response(&lineage))
    }

    pub async fn find_data_lineage(
        &self,
        id: &str,
    ) -> Result<DataLineageResponse, DataHubError> {
        let lineage_id = banko_domain::data_hub::DataLineageId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let lineage = self
            .lineage_repo
            .find_by_id(&lineage_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::LineageNotFound)?;

        Ok(self.lineage_to_response(&lineage))
    }

    // --- Data Reconciliation Operations ---

    pub async fn create_reconciliation(
        &self,
        request: DataReconciliationRequest,
    ) -> Result<DataReconciliationResponse, DataHubError> {
        let entity_type =
            DataEntityType::from_str(&request.entity_type)
                .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let reconciliation = DataReconciliation::new(
            entity_type,
            &request.source_a,
            &request.source_b,
            request.scheduled_at,
        )
        .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.reconciliation_repo
            .save(&reconciliation)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.reconciliation_to_response(&reconciliation))
    }

    pub async fn start_reconciliation(
        &self,
        id: &str,
    ) -> Result<DataReconciliationResponse, DataHubError> {
        let recon_id = banko_domain::data_hub::DataReconciliationId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let mut reconciliation = self
            .reconciliation_repo
            .find_by_id(&recon_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::ReconciliationNotFound)?;

        reconciliation
            .start()
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.reconciliation_repo
            .save(&reconciliation)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.reconciliation_to_response(&reconciliation))
    }

    pub async fn execute_reconciliation(
        &self,
        id: &str,
    ) -> Result<DataReconciliationResponse, DataHubError> {
        let recon_id = banko_domain::data_hub::DataReconciliationId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let mut reconciliation = self
            .reconciliation_repo
            .find_by_id(&recon_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::ReconciliationNotFound)?;

        reconciliation
            .start()
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let discrepancies = self
            .reconciliation_engine
            .execute_reconciliation(&recon_id)
            .await
            .map_err(DataHubError::RepositoryError)?;

        for discrepancy in discrepancies {
            reconciliation.add_discrepancy(discrepancy);
        }

        reconciliation
            .resolve()
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.reconciliation_repo
            .save(&reconciliation)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.reconciliation_to_response(&reconciliation))
    }

    // --- Master Data Record Operations ---

    pub async fn create_master_data_record(
        &self,
        request: MasterDataRecordRequest,
    ) -> Result<MasterDataRecordResponse, DataHubError> {
        let entity_type =
            DataEntityType::from_str(&request.entity_type)
                .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let record = MasterDataRecord::new(
            entity_type,
            request.canonical_data,
            request.sources,
        )
        .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.master_data_repo
            .save(&record)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.master_data_to_response(&record))
    }

    pub async fn find_master_data_record(
        &self,
        id: &str,
    ) -> Result<MasterDataRecordResponse, DataHubError> {
        let record_id = banko_domain::data_hub::MasterDataRecordId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let record = self
            .master_data_repo
            .find_by_id(&record_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::MasterDataRecordNotFound)?;

        Ok(self.master_data_to_response(&record))
    }

    pub async fn mark_golden_record(
        &self,
        id: &str,
    ) -> Result<MasterDataRecordResponse, DataHubError> {
        let record_id = banko_domain::data_hub::MasterDataRecordId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let mut record = self
            .master_data_repo
            .find_by_id(&record_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::MasterDataRecordNotFound)?;

        record
            .mark_as_golden()
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.master_data_repo
            .save(&record)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.master_data_to_response(&record))
    }

    // --- Governance Policy Operations ---

    pub async fn create_governance_policy(
        &self,
        request: DataGovernancePolicyRequest,
    ) -> Result<DataGovernancePolicyResponse, DataHubError> {
        let entity_types: Result<Vec<DataEntityType>, _> = request
            .entity_types_covered
            .iter()
            .map(|t| DataEntityType::from_str(t))
            .collect();

        let entity_types = entity_types
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let classification = DataClassification::from_str(&request.classification)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let policy = DataGovernancePolicy::new(
            &request.name,
            &request.description,
            entity_types,
            request.retention_days,
            classification,
            &request.owner_team,
        )
        .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        self.policy_repo
            .save(&policy)
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(self.policy_to_response(&policy))
    }

    pub async fn find_governance_policy(
        &self,
        id: &str,
    ) -> Result<DataGovernancePolicyResponse, DataHubError> {
        let policy_id = banko_domain::data_hub::DataGovernancePolicyId::parse(id)
            .map_err(|e| DataHubError::DomainError(e.to_string()))?;

        let policy = self
            .policy_repo
            .find_by_id(&policy_id)
            .await
            .map_err(DataHubError::RepositoryError)?
            .ok_or(DataHubError::PolicyNotFound)?;

        Ok(self.policy_to_response(&policy))
    }

    pub async fn list_active_policies(
        &self,
    ) -> Result<Vec<DataGovernancePolicyResponse>, DataHubError> {
        let policies = self
            .policy_repo
            .find_active_policies()
            .await
            .map_err(DataHubError::RepositoryError)?;

        Ok(policies
            .into_iter()
            .map(|p| self.policy_to_response(&p))
            .collect())
    }

    // --- Helper Methods ---

    fn entity_to_response(&self, entity: &DataEntity) -> DataEntityResponse {
        DataEntityResponse {
            id: entity.id().to_string(),
            entity_type: entity.entity_type().to_string(),
            source_system: entity.source_system().to_string(),
            canonical_id: entity.canonical_id().to_string(),
            data_quality_score: entity.data_quality_score().value(),
            last_validated_at: entity.last_validated_at(),
            status: entity.status().to_string(),
            created_at: entity.created_at(),
            updated_at: entity.updated_at(),
        }
    }

    fn rule_to_response(&self, rule: &DataQualityRule) -> DataQualityRuleResponse {
        DataQualityRuleResponse {
            id: rule.id().to_string(),
            entity_type: rule.entity_type().to_string(),
            rule_name: rule.rule_name().to_string(),
            rule_expression: rule.rule_expression().to_string(),
            severity: format!("{:?}", rule.severity()).to_lowercase(),
            is_active: rule.is_active(),
        }
    }

    fn lineage_to_response(&self, lineage: &DataLineage) -> DataLineageResponse {
        DataLineageResponse {
            id: lineage.id().to_string(),
            source_entity_id: lineage.source_entity_id().to_string(),
            target_entity_id: lineage.target_entity_id().to_string(),
            transformation_type: lineage.transformation_type().to_string(),
            pipeline_name: lineage.pipeline_name().to_string(),
            last_run_at: lineage.last_run_at(),
        }
    }

    fn reconciliation_to_response(&self, recon: &DataReconciliation) -> DataReconciliationResponse {
        DataReconciliationResponse {
            id: recon.id().to_string(),
            entity_type: recon.entity_type().to_string(),
            source_a: recon.source_a().to_string(),
            source_b: recon.source_b().to_string(),
            discrepancies_found: recon.discrepancies_found().to_vec(),
            status: recon.status().to_string(),
            scheduled_at: recon.scheduled_at(),
            completed_at: recon.completed_at(),
        }
    }

    fn master_data_to_response(&self, record: &MasterDataRecord) -> MasterDataRecordResponse {
        MasterDataRecordResponse {
            id: record.id().to_string(),
            entity_type: record.entity_type().to_string(),
            canonical_data: record.canonical_data().clone(),
            version: record.version(),
            is_golden_record: record.is_golden_record(),
            sources: record.sources().to_vec(),
            created_at: record.created_at(),
            updated_at: record.updated_at(),
        }
    }

    fn policy_to_response(&self, policy: &DataGovernancePolicy) -> DataGovernancePolicyResponse {
        DataGovernancePolicyResponse {
            id: policy.id().to_string(),
            name: policy.name().to_string(),
            description: policy.description().to_string(),
            entity_types_covered: policy
                .entity_types_covered()
                .iter()
                .map(|t| t.to_string())
                .collect(),
            retention_days: policy.retention_days(),
            classification: policy.classification().to_string(),
            owner_team: policy.owner_team().to_string(),
            is_active: policy.is_active(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full service tests require mock implementations of all ports.
    // These tests demonstrate the expected API contracts.

    #[test]
    fn test_service_instantiation() {
        // Service can be created with Arc trait objects
        // (requires mock implementations)
    }
}
