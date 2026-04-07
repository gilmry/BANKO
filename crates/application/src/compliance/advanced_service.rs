use std::sync::Arc;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use banko_domain::compliance::{
    AuditStatus, ChangeImpactLevel, ComplianceIncident, ComplianceIncidentId,
    ComplianceRisk, ComplianceTraining, ComplianceTrainingId, GafiRecommendation,
    GafiRecommendationId, GafiStatus, IncidentSeverity, InternalAudit, InternalAuditId,
    RegulatoryChange, RegulatoryChangeId, RiskImpact, RiskMatrixLevel, RiskProbability,
    RiskRating, ThirdPartyAssessment, ThirdPartyAssessmentId, TrainingStatus, WhistleblowerReport,
    WhistleblowerReportId, WhistleblowerStatus,
};

use super::errors::ComplianceError;
use super::ports::{
    IGafiRepository, IInternalAuditRepository, IComplianceRiskRepository,
    IComplianceTrainingRepository, IRegulatoryChangeRepository, IComplianceIncidentRepository,
    IWhistleblowerRepository, IThirdPartyRepository,
};

// ============================================================
// GafiRecommendationService (FR-171)
// ============================================================

pub struct GafiRecommendationService {
    gafi_repo: Arc<dyn IGafiRepository>,
}

impl GafiRecommendationService {
    pub fn new(gafi_repo: Arc<dyn IGafiRepository>) -> Self {
        GafiRecommendationService { gafi_repo }
    }

    pub async fn create_recommendation(
        &self,
        recommendation_number: i32,
        title: String,
        description: String,
        category: String,
        responsible_unit: String,
    ) -> Result<GafiRecommendation, ComplianceError> {
        let recommendation = GafiRecommendation::new(
            recommendation_number,
            title,
            description,
            category,
            responsible_unit,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.gafi_repo
            .save_recommendation(&recommendation)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(recommendation)
    }

    pub async fn update_status(
        &self,
        id: &GafiRecommendationId,
        new_status: GafiStatus,
    ) -> Result<GafiRecommendation, ComplianceError> {
        let mut recommendation = self
            .gafi_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput(
                "GAFI recommendation not found".to_string(),
            ))?;

        recommendation
            .set_status(new_status)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.gafi_repo
            .save_recommendation(&recommendation)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(recommendation)
    }

    pub async fn list_all(&self) -> Result<Vec<GafiRecommendation>, ComplianceError> {
        self.gafi_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    pub async fn get_compliance_status(&self) -> Result<(i64, i64, i64, f64), ComplianceError> {
        let all = self.list_all().await?;

        let total = all.len() as i64;
        let compliant = all
            .iter()
            .filter(|r| r.status() == GafiStatus::Compliant)
            .count() as i64;
        let in_progress = all
            .iter()
            .filter(|r| r.status() == GafiStatus::InProgress)
            .count() as i64;

        let percentage = if total > 0 {
            (compliant as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok((total, compliant, in_progress, percentage))
    }
}

// ============================================================
// InternalAuditService (FR-173)
// ============================================================

pub struct InternalAuditService {
    audit_repo: Arc<dyn IInternalAuditRepository>,
}

impl InternalAuditService {
    pub fn new(audit_repo: Arc<dyn IInternalAuditRepository>) -> Self {
        InternalAuditService { audit_repo }
    }

    pub async fn schedule_audit(
        &self,
        audit_code: String,
        title: String,
        scope: String,
        planned_start_date: DateTime<Utc>,
        planned_end_date: DateTime<Utc>,
        audit_team_lead: String,
    ) -> Result<InternalAudit, ComplianceError> {
        let audit = InternalAudit::new(
            audit_code,
            title,
            scope,
            planned_start_date,
            planned_end_date,
            audit_team_lead,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.audit_repo
            .save_audit(&audit)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(audit)
    }

    pub async fn start_audit(&self, id: &InternalAuditId) -> Result<InternalAudit, ComplianceError> {
        let mut audit = self
            .audit_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Audit not found".to_string()))?;

        audit
            .start_audit()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.audit_repo
            .save_audit(&audit)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(audit)
    }

    pub async fn complete_audit(
        &self,
        id: &InternalAuditId,
        findings_count: i32,
        critical_findings: i32,
    ) -> Result<InternalAudit, ComplianceError> {
        let mut audit = self
            .audit_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Audit not found".to_string()))?;

        audit
            .complete_audit(findings_count, critical_findings)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.audit_repo
            .save_audit(&audit)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(audit)
    }

    pub async fn get_planned_audits(&self) -> Result<Vec<InternalAudit>, ComplianceError> {
        self.audit_repo
            .find_by_status(AuditStatus::Planned)
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    pub async fn get_audit_calendar(&self) -> Result<Vec<InternalAudit>, ComplianceError> {
        self.audit_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)
    }
}

// ============================================================
// ComplianceRiskService (FR-174)
// ============================================================

pub struct ComplianceRiskService {
    risk_repo: Arc<dyn IComplianceRiskRepository>,
}

impl ComplianceRiskService {
    pub fn new(risk_repo: Arc<dyn IComplianceRiskRepository>) -> Self {
        ComplianceRiskService { risk_repo }
    }

    pub async fn create_risk(
        &self,
        risk_code: String,
        title: String,
        description: String,
        category: String,
        probability: RiskProbability,
        impact: RiskImpact,
        owner: String,
    ) -> Result<ComplianceRisk, ComplianceError> {
        let risk = ComplianceRisk::new(
            risk_code, title, description, category, probability, impact, owner,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.risk_repo
            .save_risk(&risk)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(risk)
    }

    pub async fn get_risk_matrix(&self) -> Result<Vec<ComplianceRisk>, ComplianceError> {
        self.risk_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    pub async fn get_critical_risks(&self) -> Result<Vec<ComplianceRisk>, ComplianceError> {
        let all_risks = self.get_risk_matrix().await?;
        let critical: Vec<_> = all_risks
            .into_iter()
            .filter(|r| r.risk_level() == RiskMatrixLevel::Red)
            .collect();
        Ok(critical)
    }

    pub async fn get_risks_by_level(
        &self,
        level: RiskMatrixLevel,
    ) -> Result<Vec<ComplianceRisk>, ComplianceError> {
        let all_risks = self.get_risk_matrix().await?;
        let filtered: Vec<_> = all_risks
            .into_iter()
            .filter(|r| r.risk_level() == level)
            .collect();
        Ok(filtered)
    }

    pub async fn get_risk_distribution(
        &self,
    ) -> Result<(i64, i64, i64, i64), ComplianceError> {
        let all = self.get_risk_matrix().await?;

        let green = all
            .iter()
            .filter(|r| r.risk_level() == RiskMatrixLevel::Green)
            .count() as i64;
        let yellow = all
            .iter()
            .filter(|r| r.risk_level() == RiskMatrixLevel::Yellow)
            .count() as i64;
        let orange = all
            .iter()
            .filter(|r| r.risk_level() == RiskMatrixLevel::Orange)
            .count() as i64;
        let red = all
            .iter()
            .filter(|r| r.risk_level() == RiskMatrixLevel::Red)
            .count() as i64;

        Ok((green, yellow, orange, red))
    }
}

// ============================================================
// ComplianceTrainingService (FR-175)
// ============================================================

pub struct ComplianceTrainingService {
    training_repo: Arc<dyn IComplianceTrainingRepository>,
}

impl ComplianceTrainingService {
    pub fn new(training_repo: Arc<dyn IComplianceTrainingRepository>) -> Self {
        ComplianceTrainingService { training_repo }
    }

    pub async fn schedule_training(
        &self,
        training_code: String,
        title: String,
        description: String,
        training_type: String,
        target_audience: String,
        employee_id: Uuid,
        scheduled_date: DateTime<Utc>,
        validity_period_months: i32,
    ) -> Result<ComplianceTraining, ComplianceError> {
        let training = ComplianceTraining::new(
            training_code,
            title,
            description,
            training_type,
            target_audience,
            employee_id,
            scheduled_date,
            validity_period_months,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.training_repo
            .save_training(&training)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(training)
    }

    pub async fn complete_training(
        &self,
        id: &ComplianceTrainingId,
        score: i32,
        certificate_url: String,
    ) -> Result<ComplianceTraining, ComplianceError> {
        let mut training = self
            .training_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Training not found".to_string()))?;

        training
            .complete_training(score, certificate_url)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.training_repo
            .save_training(&training)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(training)
    }

    pub async fn get_employee_trainings(
        &self,
        employee_id: Uuid,
    ) -> Result<Vec<ComplianceTraining>, ComplianceError> {
        self.training_repo
            .find_by_employee(employee_id)
            .await
            .map_err(ComplianceError::RepositoryError)
    }

    pub async fn get_overdue_trainings(&self) -> Result<Vec<ComplianceTraining>, ComplianceError> {
        let all = self
            .training_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let overdue: Vec<_> = all
            .into_iter()
            .filter(|t| t.status() == TrainingStatus::Overdue)
            .collect();

        Ok(overdue)
    }

    pub async fn get_expiring_trainings(
        &self,
    ) -> Result<Vec<ComplianceTraining>, ComplianceError> {
        let all = self
            .training_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let expiring: Vec<_> = all
            .into_iter()
            .filter(|t| t.status() == TrainingStatus::Expired)
            .collect();

        Ok(expiring)
    }

    pub async fn get_completion_rate(
        &self,
    ) -> Result<f64, ComplianceError> {
        let all = self
            .training_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        if all.is_empty() {
            return Ok(0.0);
        }

        let completed = all
            .iter()
            .filter(|t| t.status() == TrainingStatus::Completed)
            .count() as f64;

        Ok((completed / all.len() as f64) * 100.0)
    }
}

// ============================================================
// RegulatoryChangeService (FR-176)
// ============================================================

pub struct RegulatoryChangeService {
    change_repo: Arc<dyn IRegulatoryChangeRepository>,
}

impl RegulatoryChangeService {
    pub fn new(change_repo: Arc<dyn IRegulatoryChangeRepository>) -> Self {
        RegulatoryChangeService { change_repo }
    }

    pub async fn register_change(
        &self,
        change_code: String,
        title: String,
        source: String,
        description: String,
        effective_date: DateTime<Utc>,
        deadline_for_compliance: DateTime<Utc>,
        impact_level: ChangeImpactLevel,
        action_required: String,
        responsible_unit: String,
    ) -> Result<RegulatoryChange, ComplianceError> {
        let change = RegulatoryChange::new(
            change_code,
            title,
            source,
            description,
            effective_date,
            deadline_for_compliance,
            impact_level,
            action_required,
            responsible_unit,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.change_repo
            .save_change(&change)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(change)
    }

    pub async fn update_progress(
        &self,
        id: &RegulatoryChangeId,
        progress: i32,
    ) -> Result<RegulatoryChange, ComplianceError> {
        let mut change = self
            .change_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Change not found".to_string()))?;

        change
            .update_progress(progress)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.change_repo
            .save_change(&change)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(change)
    }

    pub async fn get_pending_changes(&self) -> Result<Vec<RegulatoryChange>, ComplianceError> {
        let all = self
            .change_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let pending: Vec<_> = all
            .into_iter()
            .filter(|c| c.status() != "Compliant")
            .collect();

        Ok(pending)
    }

    pub async fn get_changes_by_impact(
        &self,
        impact_level: ChangeImpactLevel,
    ) -> Result<Vec<RegulatoryChange>, ComplianceError> {
        let all = self
            .change_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let filtered: Vec<_> = all
            .into_iter()
            .filter(|c| c.impact_level() == impact_level)
            .collect();

        Ok(filtered)
    }
}

// ============================================================
// ComplianceIncidentService (FR-177)
// ============================================================

pub struct ComplianceIncidentService {
    incident_repo: Arc<dyn IComplianceIncidentRepository>,
}

impl ComplianceIncidentService {
    pub fn new(incident_repo: Arc<dyn IComplianceIncidentRepository>) -> Self {
        ComplianceIncidentService { incident_repo }
    }

    pub async fn report_incident(
        &self,
        incident_code: String,
        title: String,
        description: String,
        category: String,
        severity: IncidentSeverity,
    ) -> Result<ComplianceIncident, ComplianceError> {
        let incident = ComplianceIncident::new(
            incident_code,
            title,
            description,
            category,
            severity,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.incident_repo
            .save_incident(&incident)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(incident)
    }

    pub async fn notify_regulator(
        &self,
        id: &ComplianceIncidentId,
    ) -> Result<ComplianceIncident, ComplianceError> {
        let mut incident = self
            .incident_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Incident not found".to_string()))?;

        incident
            .notify_regulator()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.incident_repo
            .save_incident(&incident)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(incident)
    }

    pub async fn close_incident(
        &self,
        id: &ComplianceIncidentId,
        root_cause: String,
        remediation: String,
    ) -> Result<ComplianceIncident, ComplianceError> {
        let mut incident = self
            .incident_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Incident not found".to_string()))?;

        incident
            .close_incident(root_cause, remediation)
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.incident_repo
            .save_incident(&incident)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(incident)
    }

    pub async fn get_critical_incidents(&self) -> Result<Vec<ComplianceIncident>, ComplianceError> {
        let all = self
            .incident_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let critical: Vec<_> = all
            .into_iter()
            .filter(|i| i.severity() == IncidentSeverity::Critical)
            .collect();

        Ok(critical)
    }

    pub async fn get_open_incidents(&self) -> Result<Vec<ComplianceIncident>, ComplianceError> {
        let all = self
            .incident_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let open: Vec<_> = all
            .into_iter()
            .filter(|i| i.closed_at().is_none())
            .collect();

        Ok(open)
    }
}

// ============================================================
// WhistleblowerService (FR-178)
// ============================================================

pub struct WhistleblowerService {
    whistleblower_repo: Arc<dyn IWhistleblowerRepository>,
}

impl WhistleblowerService {
    pub fn new(whistleblower_repo: Arc<dyn IWhistleblowerRepository>) -> Self {
        WhistleblowerService {
            whistleblower_repo,
        }
    }

    pub async fn submit_anonymous_report(
        &self,
        report_code: String,
        issue_category: String,
        description: String,
        received_by: String,
    ) -> Result<WhistleblowerReport, ComplianceError> {
        let report = WhistleblowerReport::new(
            report_code,
            issue_category,
            description,
            true,
            None,
            received_by,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.whistleblower_repo
            .save_report(&report)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(report)
    }

    pub async fn submit_named_report(
        &self,
        report_code: String,
        issue_category: String,
        description: String,
        contact_email_encrypted: String,
        received_by: String,
    ) -> Result<WhistleblowerReport, ComplianceError> {
        let report = WhistleblowerReport::new(
            report_code,
            issue_category,
            description,
            false,
            Some(contact_email_encrypted),
            received_by,
        )
        .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.whistleblower_repo
            .save_report(&report)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(report)
    }

    pub async fn start_investigation(
        &self,
        id: &WhistleblowerReportId,
    ) -> Result<WhistleblowerReport, ComplianceError> {
        let mut report = self
            .whistleblower_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Report not found".to_string()))?;

        report
            .start_investigation()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.whistleblower_repo
            .save_report(&report)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(report)
    }

    pub async fn get_pending_reports(&self) -> Result<Vec<WhistleblowerReport>, ComplianceError> {
        let all = self
            .whistleblower_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let pending: Vec<_> = all
            .into_iter()
            .filter(|r| r.status() == WhistleblowerStatus::Received)
            .collect();

        Ok(pending)
    }

    pub async fn get_under_investigation(
        &self,
    ) -> Result<Vec<WhistleblowerReport>, ComplianceError> {
        let all = self
            .whistleblower_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let investigating: Vec<_> = all
            .into_iter()
            .filter(|r| r.status() == WhistleblowerStatus::UnderInvestigation)
            .collect();

        Ok(investigating)
    }
}

// ============================================================
// ThirdPartyService (FR-179)
// ============================================================

pub struct ThirdPartyService {
    third_party_repo: Arc<dyn IThirdPartyRepository>,
}

impl ThirdPartyService {
    pub fn new(third_party_repo: Arc<dyn IThirdPartyRepository>) -> Self {
        ThirdPartyService { third_party_repo }
    }

    pub async fn create_assessment(
        &self,
        vendor_name: String,
        vendor_code: String,
        vendor_type: String,
        reviewer: String,
    ) -> Result<ThirdPartyAssessment, ComplianceError> {
        let assessment =
            ThirdPartyAssessment::new(vendor_name, vendor_code, vendor_type, reviewer)
                .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.third_party_repo
            .save_assessment(&assessment)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(assessment)
    }

    pub async fn complete_aml_screening(
        &self,
        id: &ThirdPartyAssessmentId,
    ) -> Result<ThirdPartyAssessment, ComplianceError> {
        let mut assessment = self
            .third_party_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Assessment not found".to_string()))?;

        assessment
            .complete_aml_screening()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.third_party_repo
            .save_assessment(&assessment)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(assessment)
    }

    pub async fn approve_vendor(
        &self,
        id: &ThirdPartyAssessmentId,
    ) -> Result<ThirdPartyAssessment, ComplianceError> {
        let mut assessment = self
            .third_party_repo
            .find_by_id(id)
            .await
            .map_err(ComplianceError::RepositoryError)?
            .ok_or(ComplianceError::InvalidInput("Assessment not found".to_string()))?;

        assessment
            .approve_assessment()
            .map_err(|e| ComplianceError::DomainError(e.to_string()))?;

        self.third_party_repo
            .save_assessment(&assessment)
            .await
            .map_err(ComplianceError::RepositoryError)?;

        Ok(assessment)
    }

    pub async fn get_high_risk_vendors(
        &self,
    ) -> Result<Vec<ThirdPartyAssessment>, ComplianceError> {
        let all = self
            .third_party_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let high_risk: Vec<_> = all
            .into_iter()
            .filter(|v| v.risk_rating() == RiskRating::High || v.risk_rating() == RiskRating::VeryHigh)
            .collect();

        Ok(high_risk)
    }

    pub async fn get_vendors_due_for_review(
        &self,
    ) -> Result<Vec<ThirdPartyAssessment>, ComplianceError> {
        let all = self
            .third_party_repo
            .list_all()
            .await
            .map_err(ComplianceError::RepositoryError)?;

        let now = Utc::now();
        let due: Vec<_> = all
            .into_iter()
            .filter(|v| v.next_review_date().is_some() && v.next_review_date().unwrap() <= now)
            .collect();

        Ok(due)
    }
}

// ============================================================
// Unit Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock repositories for testing
    struct MockGafiRepository;

    #[async_trait::async_trait]
    impl IGafiRepository for MockGafiRepository {
        async fn save_recommendation(&self, _: &GafiRecommendation) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, _: &GafiRecommendationId) -> Result<Option<GafiRecommendation>, String> {
            Ok(None)
        }

        async fn list_all(&self) -> Result<Vec<GafiRecommendation>, String> {
            Ok(Vec::new())
        }

        async fn count_by_status(&self, _: GafiStatus) -> Result<i64, String> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_gafi_service_create_recommendation() {
        let repo = Arc::new(MockGafiRepository);
        let service = GafiRecommendationService::new(repo);

        let result = service
            .create_recommendation(
                1,
                "Test".to_string(),
                "Description".to_string(),
                "Category".to_string(),
                "Unit".to_string(),
            )
            .await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_compliance_risk_service_new() {
        struct MockRiskRepository;

        #[async_trait::async_trait]
        impl IComplianceRiskRepository for MockRiskRepository {
            async fn save_risk(&self, _: &ComplianceRisk) -> Result<(), String> {
                Ok(())
            }

            async fn find_by_id(&self, _: &ComplianceRiskId) -> Result<Option<ComplianceRisk>, String> {
                Ok(None)
            }

            async fn list_all(&self) -> Result<Vec<ComplianceRisk>, String> {
                Ok(Vec::new())
            }
        }

        let repo = Arc::new(MockRiskRepository);
        let _service = ComplianceRiskService::new(repo);
    }

    #[test]
    fn test_training_service_new() {
        struct MockTrainingRepository;

        #[async_trait::async_trait]
        impl IComplianceTrainingRepository for MockTrainingRepository {
            async fn save_training(&self, _: &ComplianceTraining) -> Result<(), String> {
                Ok(())
            }

            async fn find_by_id(&self, _: &ComplianceTrainingId) -> Result<Option<ComplianceTraining>, String> {
                Ok(None)
            }

            async fn list_all(&self) -> Result<Vec<ComplianceTraining>, String> {
                Ok(Vec::new())
            }

            async fn find_by_employee(&self, _: Uuid) -> Result<Vec<ComplianceTraining>, String> {
                Ok(Vec::new())
            }
        }

        let repo = Arc::new(MockTrainingRepository);
        let _service = ComplianceTrainingService::new(repo);
    }
}
