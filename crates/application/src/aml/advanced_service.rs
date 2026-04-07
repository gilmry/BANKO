use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use banko_domain::aml::*;
use banko_domain::shared::errors::DomainError;

use super::errors::AmlServiceError;
use super::ports::*;

// ============================================================================
// GoAML Submission Service (FR-045)
// ============================================================================

pub struct GoAmlSubmissionService {
    submission_repo: Arc<dyn IGoAmlSubmissionRepository>,
    investigation_repo: Arc<dyn IInvestigationRepository>,
}

impl GoAmlSubmissionService {
    pub fn new(
        submission_repo: Arc<dyn IGoAmlSubmissionRepository>,
        investigation_repo: Arc<dyn IInvestigationRepository>,
    ) -> Self {
        Self {
            submission_repo,
            investigation_repo,
        }
    }

    /// Create a goAML submission from a confirmed investigation
    pub async fn create_submission(
        &self,
        investigation_id: Uuid,
        suspicion_reason: String,
        customer_document_type: String,
        customer_document_value: String,
        suspect_amount: rust_decimal::Decimal,
        xml_payload: String,
    ) -> Result<GoAmlSubmission, AmlServiceError> {
        // Verify investigation exists
        let _investigation = self
            .investigation_repo
            .find_by_id(investigation_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::InvestigationNotFound)?;

        let submission = GoAmlSubmission::new(
            investigation_id,
            suspicion_reason,
            customer_document_type,
            customer_document_value,
            suspect_amount,
            Utc::now(),
            xml_payload,
        )
        .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.submission_repo
            .save(&submission)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(submission)
    }

    /// Submit goAML to CTAF
    pub async fn submit_to_ctaf(
        &self,
        submission_id: &GoAmlSubmissionId,
    ) -> Result<(), AmlServiceError> {
        let mut submission = self
            .submission_repo
            .find_by_id(submission_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::Internal("Submission not found".to_string()))?;

        submission
            .mark_submitted()
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.submission_repo
            .save(&submission)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(())
    }

    /// Mark submission as acknowledged by CTAF
    pub async fn acknowledge_submission(
        &self,
        submission_id: &GoAmlSubmissionId,
        ctaf_reference: String,
    ) -> Result<(), AmlServiceError> {
        let mut submission = self
            .submission_repo
            .find_by_id(submission_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::Internal("Submission not found".to_string()))?;

        submission
            .mark_acknowledged(ctaf_reference)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.submission_repo
            .save(&submission)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(())
    }
}

// ============================================================================
// Travel Rule Service (FR-047)
// ============================================================================

pub struct TravelRuleService {
    message_repo: Arc<dyn ITravelRuleMessageRepository>,
}

impl TravelRuleService {
    pub fn new(message_repo: Arc<dyn ITravelRuleMessageRepository>) -> Self {
        Self { message_repo }
    }

    /// Create a Travel Rule message for cross-border transfer
    pub async fn create_message(
        &self,
        transaction_id: Uuid,
        originator: OriginatorInfo,
        beneficiary: BeneficiaryInfo,
        amount: rust_decimal::Decimal,
        currency: String,
    ) -> Result<TravelRuleMessage, AmlServiceError> {
        let message = TravelRuleMessage::new(
            transaction_id,
            originator,
            beneficiary,
            amount,
            currency,
        )
        .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.message_repo
            .save(&message)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(message)
    }

    /// Get Travel Rule message for a transaction
    pub async fn get_message(
        &self,
        message_id: &TravelRuleMessageId,
    ) -> Result<TravelRuleMessage, AmlServiceError> {
        self.message_repo
            .find_by_id(message_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::Internal("Message not found".to_string()))
    }

    /// Check if Travel Rule message is still valid
    pub async fn validate_message(
        &self,
        message_id: &TravelRuleMessageId,
    ) -> Result<bool, AmlServiceError> {
        let message = self.get_message(message_id).await?;
        Ok(!message.is_expired())
    }
}

// ============================================================================
// Enhanced Due Diligence Service (FR-048)
// ============================================================================

pub struct EddService {
    profile_repo: Arc<dyn IEddProfileRepository>,
}

impl EddService {
    pub fn new(profile_repo: Arc<dyn IEddProfileRepository>) -> Self {
        Self { profile_repo }
    }

    /// Initiate EDD for high-risk customer
    pub async fn initiate_edd(
        &self,
        customer_id: Uuid,
        trigger_reason: String,
    ) -> Result<EddProfile, AmlServiceError> {
        let profile =
            EddProfile::new(customer_id, trigger_reason)
                .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.profile_repo
            .save(&profile)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(profile)
    }

    /// Complete EDD assessment
    pub async fn complete_edd(
        &self,
        profile_id: &EddProfileId,
        risk_assessment: String,
        approved_by: Uuid,
    ) -> Result<(), AmlServiceError> {
        let mut profile = self
            .profile_repo
            .find_by_id(profile_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::Internal("EDD profile not found".to_string()))?;

        profile
            .mark_completed(risk_assessment, approved_by)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.profile_repo
            .save(&profile)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(())
    }

    /// Reject EDD due to high risk
    pub async fn reject_edd(
        &self,
        profile_id: &EddProfileId,
        approved_by: Uuid,
    ) -> Result<(), AmlServiceError> {
        let mut profile = self
            .profile_repo
            .find_by_id(profile_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::Internal("EDD profile not found".to_string()))?;

        profile
            .reject_high_risk(approved_by)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.profile_repo
            .save(&profile)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(())
    }

    /// Get active EDD profiles (reviews pending)
    pub async fn get_active_profiles(
        &self,
    ) -> Result<Vec<EddProfile>, AmlServiceError> {
        self.profile_repo
            .find_by_status(EddStatus::InProgress)
            .await
            .map_err(AmlServiceError::Internal)
    }
}

// ============================================================================
// AML Training Service (FR-051)
// ============================================================================

pub struct AmlTrainingService {
    training_repo: Arc<dyn IAmlTrainingRepository>,
}

impl AmlTrainingService {
    pub fn new(training_repo: Arc<dyn IAmlTrainingRepository>) -> Self {
        Self { training_repo }
    }

    /// Record AML training completion
    pub async fn record_training(
        &self,
        employee_id: Uuid,
        training_type: TrainingType,
        training_version: String,
        provider: String,
    ) -> Result<AmlTrainingRecord, AmlServiceError> {
        let record = AmlTrainingRecord::new(
            employee_id,
            training_type,
            training_version,
            provider,
        )
        .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.training_repo
            .save(&record)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(record)
    }

    /// Get employees with expired training
    pub async fn get_expired_trainings(
        &self,
    ) -> Result<Vec<AmlTrainingRecord>, AmlServiceError> {
        self.training_repo
            .find_expired()
            .await
            .map_err(AmlServiceError::Internal)
    }

    /// Get training records for an employee
    pub async fn get_employee_trainings(
        &self,
        employee_id: Uuid,
    ) -> Result<Vec<AmlTrainingRecord>, AmlServiceError> {
        self.training_repo
            .find_by_employee_id(employee_id)
            .await
            .map_err(AmlServiceError::Internal)
    }

    /// Verify employee has current training
    pub async fn is_trained(&self, employee_id: Uuid) -> Result<bool, AmlServiceError> {
        let trainings = self.get_employee_trainings(employee_id).await?;
        Ok(trainings.iter().any(|t| !t.is_expired()))
    }
}

// ============================================================================
// PEP Continuous Screening Service (FR-053)
// ============================================================================

pub struct PepScreeningScheduleService {
    schedule_repo: Arc<dyn IPepScreeningScheduleRepository>,
}

impl PepScreeningScheduleService {
    pub fn new(schedule_repo: Arc<dyn IPepScreeningScheduleRepository>) -> Self {
        Self { schedule_repo }
    }

    /// Create continuous PEP screening schedule for customer
    pub async fn create_schedule(
        &self,
        customer_id: Uuid,
        pep_list_sources: Vec<String>,
        frequency: ScreeningFrequency,
    ) -> Result<PepScreeningSchedule, AmlServiceError> {
        let schedule = PepScreeningSchedule::new(customer_id, pep_list_sources, frequency)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.schedule_repo
            .save(&schedule)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(schedule)
    }

    /// Get all due screenings
    pub async fn get_due_screenings(&self) -> Result<Vec<PepScreeningSchedule>, AmlServiceError> {
        self.schedule_repo
            .find_active_due()
            .await
            .map_err(AmlServiceError::Internal)
    }

    /// Mark screening as completed
    pub async fn mark_screened(
        &self,
        schedule_id: &PepScreeningScheduleId,
        match_found: bool,
    ) -> Result<(), AmlServiceError> {
        let mut schedule = self
            .schedule_repo
            .find_by_id(schedule_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::Internal(
                "Schedule not found".to_string(),
            ))?;

        schedule
            .mark_screened(match_found)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.schedule_repo
            .save(&schedule)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(())
    }

    /// Get schedules for a customer
    pub async fn get_customer_schedules(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<PepScreeningSchedule>, AmlServiceError> {
        self.schedule_repo
            .find_by_customer_id(customer_id)
            .await
            .map_err(AmlServiceError::Internal)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock repository implementations for testing
    struct MockGoAmlSubmissionRepository;

    #[async_trait::async_trait]
    impl IGoAmlSubmissionRepository for MockGoAmlSubmissionRepository {
        async fn save(&self, _submission: &GoAmlSubmission) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &GoAmlSubmissionId,
        ) -> Result<Option<GoAmlSubmission>, String> {
            Ok(None)
        }

        async fn find_by_investigation_id(
            &self,
            _investigation_id: Uuid,
        ) -> Result<Option<GoAmlSubmission>, String> {
            Ok(None)
        }
    }

    struct MockInvestigationRepository;

    #[async_trait::async_trait]
    impl IInvestigationRepository for MockInvestigationRepository {
        async fn save(&self, _investigation: &Investigation) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _id: Uuid) -> Result<Option<Investigation>, String> {
            // Return a mock investigation
            let inv = Investigation::new(
                Uuid::new_v4(),
                "Test".to_string(),
                Uuid::new_v4(),
            )
            .ok();
            Ok(inv)
        }

        async fn find_by_alert_id(
            &self,
            _alert_id: Uuid,
        ) -> Result<Option<Investigation>, String> {
            Ok(None)
        }

        async fn find_by_status(
            &self,
            _status: InvestigationStatus,
        ) -> Result<Vec<Investigation>, String> {
            Ok(Vec::new())
        }

        async fn find_all(
            &self,
            _status: Option<InvestigationStatus>,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<Investigation>, String> {
            Ok(Vec::new())
        }

        async fn count_all(
            &self,
            _status: Option<InvestigationStatus>,
        ) -> Result<i64, String> {
            Ok(0)
        }
    }

    struct MockTravelRuleRepository;

    #[async_trait::async_trait]
    impl ITravelRuleMessageRepository for MockTravelRuleRepository {
        async fn save(&self, _message: &TravelRuleMessage) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &TravelRuleMessageId,
        ) -> Result<Option<TravelRuleMessage>, String> {
            Ok(None)
        }

        async fn find_by_transaction_id(
            &self,
            _transaction_id: Uuid,
        ) -> Result<Option<TravelRuleMessage>, String> {
            Ok(None)
        }
    }

    struct MockEddRepository;

    #[async_trait::async_trait]
    impl IEddProfileRepository for MockEddRepository {
        async fn save(&self, _profile: &EddProfile) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &EddProfileId,
        ) -> Result<Option<EddProfile>, String> {
            Ok(None)
        }

        async fn find_by_customer_id(
            &self,
            _customer_id: Uuid,
        ) -> Result<Option<EddProfile>, String> {
            Ok(None)
        }

        async fn find_by_status(
            &self,
            _status: EddStatus,
        ) -> Result<Vec<EddProfile>, String> {
            Ok(Vec::new())
        }
    }

    struct MockTrainingRepository;

    #[async_trait::async_trait]
    impl IAmlTrainingRepository for MockTrainingRepository {
        async fn save(&self, _record: &AmlTrainingRecord) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_employee_id(
            &self,
            _employee_id: Uuid,
        ) -> Result<Vec<AmlTrainingRecord>, String> {
            Ok(Vec::new())
        }

        async fn find_expired(&self) -> Result<Vec<AmlTrainingRecord>, String> {
            Ok(Vec::new())
        }
    }

    struct MockPepScheduleRepository;

    #[async_trait::async_trait]
    impl IPepScreeningScheduleRepository for MockPepScheduleRepository {
        async fn save(&self, _schedule: &PepScreeningSchedule) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: &PepScreeningScheduleId,
        ) -> Result<Option<PepScreeningSchedule>, String> {
            Ok(None)
        }

        async fn find_by_customer_id(
            &self,
            _customer_id: Uuid,
        ) -> Result<Vec<PepScreeningSchedule>, String> {
            Ok(Vec::new())
        }

        async fn find_active_due(
            &self,
        ) -> Result<Vec<PepScreeningSchedule>, String> {
            Ok(Vec::new())
        }
    }

    #[tokio::test]
    async fn test_travel_rule_service_creation() {
        let repo = Arc::new(MockTravelRuleRepository);
        let service = TravelRuleService::new(repo);

        let originator = OriginatorInfo::new(
            "Test".to_string(),
            "passport".to_string(),
            "ID123".to_string(),
            "IBAN123".to_string(),
            Uuid::new_v4(),
            "Address".to_string(),
        )
        .unwrap();

        let beneficiary = BeneficiaryInfo::new(
            "Beneficiary".to_string(),
            "ACCOUNT123".to_string(),
            "BIC123".to_string(),
            "US".to_string(),
        )
        .unwrap();

        let result = service
            .create_message(
                Uuid::new_v4(),
                originator,
                beneficiary,
                rust_decimal::Decimal::from(1000),
                "USD".to_string(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_aml_training_service() {
        let repo = Arc::new(MockTrainingRepository);
        let service = AmlTrainingService::new(repo);

        let result = service
            .record_training(
                Uuid::new_v4(),
                TrainingType::Annual,
                "v1".to_string(),
                "Provider".to_string(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pep_screening_schedule_service() {
        let repo = Arc::new(MockPepScheduleRepository);
        let service = PepScreeningScheduleService::new(repo);

        let result = service
            .create_schedule(
                Uuid::new_v4(),
                vec!["UN".to_string()],
                ScreeningFrequency::Quarterly,
            )
            .await;

        assert!(result.is_ok());
    }
}
