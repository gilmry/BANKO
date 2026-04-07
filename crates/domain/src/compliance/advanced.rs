use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Newtypes for Advanced Compliance IDs (FR-171 to FR-182)
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GafiRecommendationId(Uuid);

impl GafiRecommendationId {
    pub fn new() -> Self {
        GafiRecommendationId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        GafiRecommendationId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for GafiRecommendationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GafiRecommendationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InternalAuditId(Uuid);

impl InternalAuditId {
    pub fn new() -> Self {
        InternalAuditId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        InternalAuditId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InternalAuditId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InternalAuditId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComplianceRiskId(Uuid);

impl ComplianceRiskId {
    pub fn new() -> Self {
        ComplianceRiskId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ComplianceRiskId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ComplianceRiskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComplianceRiskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComplianceTrainingId(Uuid);

impl ComplianceTrainingId {
    pub fn new() -> Self {
        ComplianceTrainingId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ComplianceTrainingId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ComplianceTrainingId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComplianceTrainingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegulatoryChangeId(Uuid);

impl RegulatoryChangeId {
    pub fn new() -> Self {
        RegulatoryChangeId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        RegulatoryChangeId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for RegulatoryChangeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RegulatoryChangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComplianceIncidentId(Uuid);

impl ComplianceIncidentId {
    pub fn new() -> Self {
        ComplianceIncidentId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ComplianceIncidentId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ComplianceIncidentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComplianceIncidentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhistleblowerReportId(Uuid);

impl WhistleblowerReportId {
    pub fn new() -> Self {
        WhistleblowerReportId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        WhistleblowerReportId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for WhistleblowerReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WhistleblowerReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThirdPartyAssessmentId(Uuid);

impl ThirdPartyAssessmentId {
    pub fn new() -> Self {
        ThirdPartyAssessmentId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        ThirdPartyAssessmentId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ThirdPartyAssessmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ThirdPartyAssessmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// FR-171: GAFI Recommendation Entity (40 Recommendations)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GafiStatus {
    NotStarted,
    InProgress,
    Partially,
    Compliant,
    NotApplicable,
}

impl GafiStatus {
    pub fn as_str(&self) -> &str {
        match self {
            GafiStatus::NotStarted => "NotStarted",
            GafiStatus::InProgress => "InProgress",
            GafiStatus::Partially => "Partially",
            GafiStatus::Compliant => "Compliant",
            GafiStatus::NotApplicable => "NotApplicable",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "NotStarted" => Ok(GafiStatus::NotStarted),
            "InProgress" => Ok(GafiStatus::InProgress),
            "Partially" => Ok(GafiStatus::Partially),
            "Compliant" => Ok(GafiStatus::Compliant),
            "NotApplicable" => Ok(GafiStatus::NotApplicable),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown GAFI status: {s}"
            ))),
        }
    }
}

impl fmt::Display for GafiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GafiRecommendation {
    id: GafiRecommendationId,
    recommendation_number: i32, // 1-40
    title: String,
    description: String,
    category: String, // e.g., "AML/CFT", "TFS", "PEP", "Reporting"
    status: GafiStatus,
    implementation_notes: Option<String>,
    responsible_unit: String,
    deadline: Option<DateTime<Utc>>,
    last_assessment_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl GafiRecommendation {
    pub fn new(
        recommendation_number: i32,
        title: String,
        description: String,
        category: String,
        responsible_unit: String,
    ) -> Result<Self, DomainError> {
        if recommendation_number < 1 || recommendation_number > 40 {
            return Err(DomainError::InvalidComplianceData(
                "GAFI recommendation must be between 1 and 40".to_string(),
            ));
        }
        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "GAFI recommendation title cannot be empty".to_string(),
            ));
        }
        if description.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "GAFI recommendation description cannot be empty".to_string(),
            ));
        }
        if responsible_unit.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "GAFI responsible unit cannot be empty".to_string(),
            ));
        }

        Ok(GafiRecommendation {
            id: GafiRecommendationId::new(),
            recommendation_number,
            title,
            description,
            category,
            status: GafiStatus::NotStarted,
            implementation_notes: None,
            responsible_unit,
            deadline: None,
            last_assessment_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: GafiRecommendationId,
        recommendation_number: i32,
        title: String,
        description: String,
        category: String,
        status: GafiStatus,
        implementation_notes: Option<String>,
        responsible_unit: String,
        deadline: Option<DateTime<Utc>>,
        last_assessment_date: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        GafiRecommendation {
            id,
            recommendation_number,
            title,
            description,
            category,
            status,
            implementation_notes,
            responsible_unit,
            deadline,
            last_assessment_date,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &GafiRecommendationId {
        &self.id
    }

    pub fn recommendation_number(&self) -> i32 {
        self.recommendation_number
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> GafiStatus {
        self.status
    }

    pub fn set_status(&mut self, status: GafiStatus) -> Result<(), DomainError> {
        self.status = status;
        self.last_assessment_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_implementation_notes(&mut self, notes: String) -> Result<(), DomainError> {
        if notes.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Implementation notes cannot be empty".to_string(),
            ));
        }
        self.implementation_notes = Some(notes);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_deadline(&mut self, deadline: DateTime<Utc>) -> Result<(), DomainError> {
        if deadline <= Utc::now() {
            return Err(DomainError::InvalidComplianceData(
                "Deadline must be in the future".to_string(),
            ));
        }
        self.deadline = Some(deadline);
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ============================================================
// FR-173: Internal Audit Entity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditStatus {
    Planned,
    InProgress,
    Completed,
    Rescheduled,
}

impl AuditStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AuditStatus::Planned => "Planned",
            AuditStatus::InProgress => "InProgress",
            AuditStatus::Completed => "Completed",
            AuditStatus::Rescheduled => "Rescheduled",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Planned" => Ok(AuditStatus::Planned),
            "InProgress" => Ok(AuditStatus::InProgress),
            "Completed" => Ok(AuditStatus::Completed),
            "Rescheduled" => Ok(AuditStatus::Rescheduled),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown audit status: {s}"
            ))),
        }
    }
}

impl fmt::Display for AuditStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalAudit {
    id: InternalAuditId,
    audit_code: String,                     // e.g., "AUD-2025-001"
    title: String,
    scope: String,
    status: AuditStatus,
    planned_start_date: DateTime<Utc>,
    planned_end_date: DateTime<Utc>,
    actual_start_date: Option<DateTime<Utc>>,
    actual_end_date: Option<DateTime<Utc>>,
    audit_team_lead: String,
    findings_count: i32,
    critical_findings: i32,
    recommendations: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InternalAudit {
    pub fn new(
        audit_code: String,
        title: String,
        scope: String,
        planned_start_date: DateTime<Utc>,
        planned_end_date: DateTime<Utc>,
        audit_team_lead: String,
    ) -> Result<Self, DomainError> {
        if audit_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Audit code cannot be empty".to_string(),
            ));
        }
        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Audit title cannot be empty".to_string(),
            ));
        }
        if scope.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Audit scope cannot be empty".to_string(),
            ));
        }
        if planned_end_date <= planned_start_date {
            return Err(DomainError::InvalidComplianceData(
                "Audit end date must be after start date".to_string(),
            ));
        }
        if audit_team_lead.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Audit team lead cannot be empty".to_string(),
            ));
        }

        Ok(InternalAudit {
            id: InternalAuditId::new(),
            audit_code,
            title,
            scope,
            status: AuditStatus::Planned,
            planned_start_date,
            planned_end_date,
            actual_start_date: None,
            actual_end_date: None,
            audit_team_lead,
            findings_count: 0,
            critical_findings: 0,
            recommendations: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: InternalAuditId,
        audit_code: String,
        title: String,
        scope: String,
        status: AuditStatus,
        planned_start_date: DateTime<Utc>,
        planned_end_date: DateTime<Utc>,
        actual_start_date: Option<DateTime<Utc>>,
        actual_end_date: Option<DateTime<Utc>>,
        audit_team_lead: String,
        findings_count: i32,
        critical_findings: i32,
        recommendations: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        InternalAudit {
            id,
            audit_code,
            title,
            scope,
            status,
            planned_start_date,
            planned_end_date,
            actual_start_date,
            actual_end_date,
            audit_team_lead,
            findings_count,
            critical_findings,
            recommendations,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &InternalAuditId {
        &self.id
    }

    pub fn start_audit(&mut self) -> Result<(), DomainError> {
        if self.status != AuditStatus::Planned {
            return Err(DomainError::InvalidComplianceData(
                "Audit can only be started from Planned status".to_string(),
            ));
        }
        self.status = AuditStatus::InProgress;
        self.actual_start_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete_audit(&mut self, findings_count: i32, critical_findings: i32) -> Result<(), DomainError> {
        if self.status != AuditStatus::InProgress {
            return Err(DomainError::InvalidComplianceData(
                "Audit can only be completed from InProgress status".to_string(),
            ));
        }
        if critical_findings > findings_count {
            return Err(DomainError::InvalidComplianceData(
                "Critical findings cannot exceed total findings".to_string(),
            ));
        }
        self.status = AuditStatus::Completed;
        self.actual_end_date = Some(Utc::now());
        self.findings_count = findings_count;
        self.critical_findings = critical_findings;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reschedule(&mut self, new_start: DateTime<Utc>, new_end: DateTime<Utc>) -> Result<(), DomainError> {
        if new_end <= new_start {
            return Err(DomainError::InvalidComplianceData(
                "New end date must be after start date".to_string(),
            ));
        }
        if self.status == AuditStatus::Completed {
            return Err(DomainError::InvalidComplianceData(
                "Cannot reschedule a completed audit".to_string(),
            ));
        }
        self.planned_start_date = new_start;
        self.planned_end_date = new_end;
        self.status = AuditStatus::Rescheduled;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_recommendations(&mut self, recommendations: String) -> Result<(), DomainError> {
        if recommendations.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Recommendations cannot be empty".to_string(),
            ));
        }
        self.recommendations = Some(recommendations);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn status(&self) -> AuditStatus {
        self.status
    }
}

// ============================================================
// FR-174: Compliance Risk Matrix Entity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskProbability {
    Remote,       // 1
    Low,          // 2
    Medium,       // 3
    High,         // 4
    VeryHigh,     // 5
}

impl RiskProbability {
    pub fn as_score(&self) -> i32 {
        match self {
            RiskProbability::Remote => 1,
            RiskProbability::Low => 2,
            RiskProbability::Medium => 3,
            RiskProbability::High => 4,
            RiskProbability::VeryHigh => 5,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RiskProbability::Remote => "Remote",
            RiskProbability::Low => "Low",
            RiskProbability::Medium => "Medium",
            RiskProbability::High => "High",
            RiskProbability::VeryHigh => "VeryHigh",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Remote" => Ok(RiskProbability::Remote),
            "Low" => Ok(RiskProbability::Low),
            "Medium" => Ok(RiskProbability::Medium),
            "High" => Ok(RiskProbability::High),
            "VeryHigh" => Ok(RiskProbability::VeryHigh),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown risk probability: {s}"
            ))),
        }
    }
}

impl fmt::Display for RiskProbability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskImpact {
    Minimal,      // 1
    Minor,        // 2
    Moderate,     // 3
    Major,        // 4
    Catastrophic, // 5
}

impl RiskImpact {
    pub fn as_score(&self) -> i32 {
        match self {
            RiskImpact::Minimal => 1,
            RiskImpact::Minor => 2,
            RiskImpact::Moderate => 3,
            RiskImpact::Major => 4,
            RiskImpact::Catastrophic => 5,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RiskImpact::Minimal => "Minimal",
            RiskImpact::Minor => "Minor",
            RiskImpact::Moderate => "Moderate",
            RiskImpact::Major => "Major",
            RiskImpact::Catastrophic => "Catastrophic",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Minimal" => Ok(RiskImpact::Minimal),
            "Minor" => Ok(RiskImpact::Minor),
            "Moderate" => Ok(RiskImpact::Moderate),
            "Major" => Ok(RiskImpact::Major),
            "Catastrophic" => Ok(RiskImpact::Catastrophic),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown risk impact: {s}"
            ))),
        }
    }
}

impl fmt::Display for RiskImpact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskMatrixLevel {
    Green,   // 1-5: Low
    Yellow,  // 6-12: Medium
    Orange,  // 13-19: High
    Red,     // 20-25: Critical
}

impl RiskMatrixLevel {
    pub fn from_score(score: i32) -> Self {
        match score {
            1..=5 => RiskMatrixLevel::Green,
            6..=12 => RiskMatrixLevel::Yellow,
            13..=19 => RiskMatrixLevel::Orange,
            20..=25 => RiskMatrixLevel::Red,
            _ => RiskMatrixLevel::Red,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RiskMatrixLevel::Green => "Green",
            RiskMatrixLevel::Yellow => "Yellow",
            RiskMatrixLevel::Orange => "Orange",
            RiskMatrixLevel::Red => "Red",
        }
    }
}

impl fmt::Display for RiskMatrixLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceRisk {
    id: ComplianceRiskId,
    risk_code: String,
    title: String,
    description: String,
    category: String,
    probability: RiskProbability,
    impact: RiskImpact,
    mitigation_plan: Option<String>,
    owner: String,
    deadline: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ComplianceRisk {
    pub fn new(
        risk_code: String,
        title: String,
        description: String,
        category: String,
        probability: RiskProbability,
        impact: RiskImpact,
        owner: String,
    ) -> Result<Self, DomainError> {
        if risk_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Risk code cannot be empty".to_string(),
            ));
        }
        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Risk title cannot be empty".to_string(),
            ));
        }
        if description.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Risk description cannot be empty".to_string(),
            ));
        }
        if owner.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Risk owner cannot be empty".to_string(),
            ));
        }

        Ok(ComplianceRisk {
            id: ComplianceRiskId::new(),
            risk_code,
            title,
            description,
            category,
            probability,
            impact,
            mitigation_plan: None,
            owner,
            deadline: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: ComplianceRiskId,
        risk_code: String,
        title: String,
        description: String,
        category: String,
        probability: RiskProbability,
        impact: RiskImpact,
        mitigation_plan: Option<String>,
        owner: String,
        deadline: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ComplianceRisk {
            id,
            risk_code,
            title,
            description,
            category,
            probability,
            impact,
            mitigation_plan,
            owner,
            deadline,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ComplianceRiskId {
        &self.id
    }

    pub fn status(&self) -> &str {
        &self.category
    }

    pub fn risk_score(&self) -> i32 {
        self.probability.as_score() * self.impact.as_score()
    }

    pub fn risk_level(&self) -> RiskMatrixLevel {
        RiskMatrixLevel::from_score(self.risk_score())
    }

    pub fn impact_level(&self) -> ChangeImpactLevel {
        match self.impact {
            RiskImpact::Minimal => ChangeImpactLevel::Low,
            RiskImpact::Minor => ChangeImpactLevel::Low,
            RiskImpact::Moderate => ChangeImpactLevel::Medium,
            RiskImpact::Major => ChangeImpactLevel::High,
            RiskImpact::Catastrophic => ChangeImpactLevel::Critical,
        }
    }

    pub fn set_mitigation_plan(&mut self, plan: String) -> Result<(), DomainError> {
        if plan.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Mitigation plan cannot be empty".to_string(),
            ));
        }
        self.mitigation_plan = Some(plan);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_deadline(&mut self, deadline: DateTime<Utc>) -> Result<(), DomainError> {
        if deadline <= Utc::now() {
            return Err(DomainError::InvalidComplianceData(
                "Deadline must be in the future".to_string(),
            ));
        }
        self.deadline = Some(deadline);
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ============================================================
// FR-175: Compliance Training Entity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingStatus {
    Scheduled,
    InProgress,
    Completed,
    Expired,
    Overdue,
}

impl TrainingStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TrainingStatus::Scheduled => "Scheduled",
            TrainingStatus::InProgress => "InProgress",
            TrainingStatus::Completed => "Completed",
            TrainingStatus::Expired => "Expired",
            TrainingStatus::Overdue => "Overdue",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Scheduled" => Ok(TrainingStatus::Scheduled),
            "InProgress" => Ok(TrainingStatus::InProgress),
            "Completed" => Ok(TrainingStatus::Completed),
            "Expired" => Ok(TrainingStatus::Expired),
            "Overdue" => Ok(TrainingStatus::Overdue),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown training status: {s}"
            ))),
        }
    }
}

impl fmt::Display for TrainingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceTraining {
    id: ComplianceTrainingId,
    training_code: String,
    title: String,
    description: String,
    training_type: String, // e.g., "AML", "GDPR", "Sanctions", "PCI-DSS"
    target_audience: String,
    employee_id: Uuid,
    scheduled_date: DateTime<Utc>,
    completion_date: Option<DateTime<Utc>>,
    validity_period_months: i32,
    next_renewal_date: Option<DateTime<Utc>>,
    status: TrainingStatus,
    score: Option<i32>, // 0-100
    certificate_url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ComplianceTraining {
    pub fn new(
        training_code: String,
        title: String,
        description: String,
        training_type: String,
        target_audience: String,
        employee_id: Uuid,
        scheduled_date: DateTime<Utc>,
        validity_period_months: i32,
    ) -> Result<Self, DomainError> {
        if training_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Training code cannot be empty".to_string(),
            ));
        }
        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Training title cannot be empty".to_string(),
            ));
        }
        if validity_period_months < 1 {
            return Err(DomainError::InvalidComplianceData(
                "Training validity period must be at least 1 month".to_string(),
            ));
        }

        Ok(ComplianceTraining {
            id: ComplianceTrainingId::new(),
            training_code,
            title,
            description,
            training_type,
            target_audience,
            employee_id,
            scheduled_date,
            completion_date: None,
            validity_period_months,
            next_renewal_date: None,
            status: TrainingStatus::Scheduled,
            score: None,
            certificate_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: ComplianceTrainingId,
        training_code: String,
        title: String,
        description: String,
        training_type: String,
        target_audience: String,
        employee_id: Uuid,
        scheduled_date: DateTime<Utc>,
        completion_date: Option<DateTime<Utc>>,
        validity_period_months: i32,
        next_renewal_date: Option<DateTime<Utc>>,
        status: TrainingStatus,
        score: Option<i32>,
        certificate_url: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ComplianceTraining {
            id,
            training_code,
            title,
            description,
            training_type,
            target_audience,
            employee_id,
            scheduled_date,
            completion_date,
            validity_period_months,
            next_renewal_date,
            status,
            score,
            certificate_url,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ComplianceTrainingId {
        &self.id
    }

    pub fn complete_training(
        &mut self,
        score: i32,
        certificate_url: String,
    ) -> Result<(), DomainError> {
        if score < 0 || score > 100 {
            return Err(DomainError::InvalidComplianceData(
                "Training score must be between 0 and 100".to_string(),
            ));
        }
        if certificate_url.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Certificate URL cannot be empty".to_string(),
            ));
        }

        self.completion_date = Some(Utc::now());
        self.score = Some(score);
        self.certificate_url = Some(certificate_url);
        self.status = TrainingStatus::Completed;

        let renewal = Utc::now() + chrono::Duration::days(self.validity_period_months as i64 * 30);
        self.next_renewal_date = Some(renewal);

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_as_overdue(&mut self) {
        self.status = TrainingStatus::Overdue;
        self.updated_at = Utc::now();
    }

    pub fn mark_as_expired(&mut self) {
        self.status = TrainingStatus::Expired;
        self.updated_at = Utc::now();
    }

    pub fn status(&self) -> TrainingStatus {
        self.status
    }
}

// ============================================================
// FR-176: Regulatory Change Entity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl ChangeImpactLevel {
    pub fn as_str(&self) -> &str {
        match self {
            ChangeImpactLevel::Low => "Low",
            ChangeImpactLevel::Medium => "Medium",
            ChangeImpactLevel::High => "High",
            ChangeImpactLevel::Critical => "Critical",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Low" => Ok(ChangeImpactLevel::Low),
            "Medium" => Ok(ChangeImpactLevel::Medium),
            "High" => Ok(ChangeImpactLevel::High),
            "Critical" => Ok(ChangeImpactLevel::Critical),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown impact level: {s}"
            ))),
        }
    }
}

impl fmt::Display for ChangeImpactLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryChange {
    id: RegulatoryChangeId,
    change_code: String,
    title: String,
    source: String, // e.g., "BCT Circular 2025", "GAFI Guidance", "EU Directive"
    description: String,
    effective_date: DateTime<Utc>,
    deadline_for_compliance: DateTime<Utc>,
    impact_level: ChangeImpactLevel,
    affected_areas: Vec<String>,
    action_required: String,
    responsible_unit: String,
    status: String, // "Received", "Analyzed", "Implemented", "Compliant"
    implementation_progress: i32, // 0-100%
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RegulatoryChange {
    pub fn new(
        change_code: String,
        title: String,
        source: String,
        description: String,
        effective_date: DateTime<Utc>,
        deadline_for_compliance: DateTime<Utc>,
        impact_level: ChangeImpactLevel,
        action_required: String,
        responsible_unit: String,
    ) -> Result<Self, DomainError> {
        if change_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Change code cannot be empty".to_string(),
            ));
        }
        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Change title cannot be empty".to_string(),
            ));
        }
        if deadline_for_compliance <= effective_date {
            return Err(DomainError::InvalidComplianceData(
                "Compliance deadline must be after effective date".to_string(),
            ));
        }

        Ok(RegulatoryChange {
            id: RegulatoryChangeId::new(),
            change_code,
            title,
            source,
            description,
            effective_date,
            deadline_for_compliance,
            impact_level,
            affected_areas: Vec::new(),
            action_required,
            responsible_unit,
            status: "Received".to_string(),
            implementation_progress: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: RegulatoryChangeId,
        change_code: String,
        title: String,
        source: String,
        description: String,
        effective_date: DateTime<Utc>,
        deadline_for_compliance: DateTime<Utc>,
        impact_level: ChangeImpactLevel,
        affected_areas: Vec<String>,
        action_required: String,
        responsible_unit: String,
        status: String,
        implementation_progress: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        RegulatoryChange {
            id,
            change_code,
            title,
            source,
            description,
            effective_date,
            deadline_for_compliance,
            impact_level,
            affected_areas,
            action_required,
            responsible_unit,
            status,
            implementation_progress,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &RegulatoryChangeId {
        &self.id
    }

    pub fn update_progress(&mut self, progress: i32) -> Result<(), DomainError> {
        if progress < 0 || progress > 100 {
            return Err(DomainError::InvalidComplianceData(
                "Progress must be between 0 and 100".to_string(),
            ));
        }
        self.implementation_progress = progress;

        if progress == 100 {
            self.status = "Compliant".to_string();
        } else if progress > 0 {
            self.status = "Implemented".to_string();
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn add_affected_area(&mut self, area: String) -> Result<(), DomainError> {
        if area.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Affected area cannot be empty".to_string(),
            ));
        }
        if !self.affected_areas.contains(&area) {
            self.affected_areas.push(area);
            self.updated_at = Utc::now();
        }
        Ok(())
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn impact_level(&self) -> ChangeImpactLevel {
        self.impact_level
    }
}

// ============================================================
// FR-177: Compliance Incident Entity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

impl IncidentSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            IncidentSeverity::Minor => "Minor",
            IncidentSeverity::Moderate => "Moderate",
            IncidentSeverity::Major => "Major",
            IncidentSeverity::Critical => "Critical",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Minor" => Ok(IncidentSeverity::Minor),
            "Moderate" => Ok(IncidentSeverity::Moderate),
            "Major" => Ok(IncidentSeverity::Major),
            "Critical" => Ok(IncidentSeverity::Critical),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown incident severity: {s}"
            ))),
        }
    }
}

impl fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceIncident {
    id: ComplianceIncidentId,
    incident_code: String,
    title: String,
    description: String,
    category: String, // e.g., "AML", "KYC", "Data Protection", "Sanctions"
    severity: IncidentSeverity,
    detected_at: DateTime<Utc>,
    reported_at: Option<DateTime<Utc>>,
    closed_at: Option<DateTime<Utc>>,
    root_cause: Option<String>,
    remediation_actions: Option<String>,
    reported_to_regulator: bool,
    regulator_notification_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ComplianceIncident {
    pub fn new(
        incident_code: String,
        title: String,
        description: String,
        category: String,
        severity: IncidentSeverity,
    ) -> Result<Self, DomainError> {
        if incident_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Incident code cannot be empty".to_string(),
            ));
        }
        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Incident title cannot be empty".to_string(),
            ));
        }
        if description.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Incident description cannot be empty".to_string(),
            ));
        }

        Ok(ComplianceIncident {
            id: ComplianceIncidentId::new(),
            incident_code,
            title,
            description,
            category,
            severity,
            detected_at: Utc::now(),
            reported_at: None,
            closed_at: None,
            root_cause: None,
            remediation_actions: None,
            reported_to_regulator: false,
            regulator_notification_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: ComplianceIncidentId,
        incident_code: String,
        title: String,
        description: String,
        category: String,
        severity: IncidentSeverity,
        detected_at: DateTime<Utc>,
        reported_at: Option<DateTime<Utc>>,
        closed_at: Option<DateTime<Utc>>,
        root_cause: Option<String>,
        remediation_actions: Option<String>,
        reported_to_regulator: bool,
        regulator_notification_date: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ComplianceIncident {
            id,
            incident_code,
            title,
            description,
            category,
            severity,
            detected_at,
            reported_at,
            closed_at,
            root_cause,
            remediation_actions,
            reported_to_regulator,
            regulator_notification_date,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ComplianceIncidentId {
        &self.id
    }

    pub fn report_incident(&mut self) -> Result<(), DomainError> {
        self.reported_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn notify_regulator(&mut self) -> Result<(), DomainError> {
        self.reported_to_regulator = true;
        self.regulator_notification_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn close_incident(
        &mut self,
        root_cause: String,
        remediation: String,
    ) -> Result<(), DomainError> {
        if root_cause.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Root cause cannot be empty".to_string(),
            ));
        }
        if remediation.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Remediation actions cannot be empty".to_string(),
            ));
        }

        self.root_cause = Some(root_cause);
        self.remediation_actions = Some(remediation);
        self.closed_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn severity(&self) -> IncidentSeverity {
        self.severity
    }

    pub fn closed_at(&self) -> Option<DateTime<Utc>> {
        self.closed_at
    }
}

// ============================================================
// FR-178: Whistleblower Report Entity (Anonymous)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WhistleblowerStatus {
    Received,
    UnderInvestigation,
    Substantiated,
    NotSubstantiated,
    ActionTaken,
    Closed,
}

impl WhistleblowerStatus {
    pub fn as_str(&self) -> &str {
        match self {
            WhistleblowerStatus::Received => "Received",
            WhistleblowerStatus::UnderInvestigation => "UnderInvestigation",
            WhistleblowerStatus::Substantiated => "Substantiated",
            WhistleblowerStatus::NotSubstantiated => "NotSubstantiated",
            WhistleblowerStatus::ActionTaken => "ActionTaken",
            WhistleblowerStatus::Closed => "Closed",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Received" => Ok(WhistleblowerStatus::Received),
            "UnderInvestigation" => Ok(WhistleblowerStatus::UnderInvestigation),
            "Substantiated" => Ok(WhistleblowerStatus::Substantiated),
            "NotSubstantiated" => Ok(WhistleblowerStatus::NotSubstantiated),
            "ActionTaken" => Ok(WhistleblowerStatus::ActionTaken),
            "Closed" => Ok(WhistleblowerStatus::Closed),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown whistleblower status: {s}"
            ))),
        }
    }
}

impl fmt::Display for WhistleblowerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhistleblowerReport {
    id: WhistleblowerReportId,
    report_code: String,
    issue_category: String, // e.g., "Fraud", "AML Violation", "Sanctions Breach", "Data Privacy"
    description: String,
    is_anonymous: bool,
    contact_email_encrypted: Option<String>, // Encrypted if not anonymous
    report_date: DateTime<Utc>,
    received_by: String, // Name/ID of compliance officer
    status: WhistleblowerStatus,
    investigation_notes: Option<String>,
    actions_taken: Option<String>,
    closure_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl WhistleblowerReport {
    pub fn new(
        report_code: String,
        issue_category: String,
        description: String,
        is_anonymous: bool,
        contact_email_encrypted: Option<String>,
        received_by: String,
    ) -> Result<Self, DomainError> {
        if report_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Report code cannot be empty".to_string(),
            ));
        }
        if issue_category.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Issue category cannot be empty".to_string(),
            ));
        }
        if description.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Description cannot be empty".to_string(),
            ));
        }
        if !is_anonymous && contact_email_encrypted.is_none() {
            return Err(DomainError::InvalidComplianceData(
                "Contact email required for non-anonymous reports".to_string(),
            ));
        }

        Ok(WhistleblowerReport {
            id: WhistleblowerReportId::new(),
            report_code,
            issue_category,
            description,
            is_anonymous,
            contact_email_encrypted,
            report_date: Utc::now(),
            received_by,
            status: WhistleblowerStatus::Received,
            investigation_notes: None,
            actions_taken: None,
            closure_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: WhistleblowerReportId,
        report_code: String,
        issue_category: String,
        description: String,
        is_anonymous: bool,
        contact_email_encrypted: Option<String>,
        report_date: DateTime<Utc>,
        received_by: String,
        status: WhistleblowerStatus,
        investigation_notes: Option<String>,
        actions_taken: Option<String>,
        closure_date: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        WhistleblowerReport {
            id,
            report_code,
            issue_category,
            description,
            is_anonymous,
            contact_email_encrypted,
            report_date,
            received_by,
            status,
            investigation_notes,
            actions_taken,
            closure_date,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &WhistleblowerReportId {
        &self.id
    }

    pub fn start_investigation(&mut self) -> Result<(), DomainError> {
        if self.status != WhistleblowerStatus::Received {
            return Err(DomainError::InvalidComplianceData(
                "Investigation can only start from Received status".to_string(),
            ));
        }
        self.status = WhistleblowerStatus::UnderInvestigation;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn substantiate(&mut self, investigation_notes: String) -> Result<(), DomainError> {
        if investigation_notes.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Investigation notes cannot be empty".to_string(),
            ));
        }
        self.status = WhistleblowerStatus::Substantiated;
        self.investigation_notes = Some(investigation_notes);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn not_substantiate(&mut self, investigation_notes: String) -> Result<(), DomainError> {
        if investigation_notes.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Investigation notes cannot be empty".to_string(),
            ));
        }
        self.status = WhistleblowerStatus::NotSubstantiated;
        self.investigation_notes = Some(investigation_notes);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn close_report(
        &mut self,
        actions_taken: String,
    ) -> Result<(), DomainError> {
        if actions_taken.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Actions taken cannot be empty".to_string(),
            ));
        }
        self.actions_taken = Some(actions_taken);
        self.closure_date = Some(Utc::now());
        self.status = WhistleblowerStatus::Closed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn status(&self) -> WhistleblowerStatus {
        self.status
    }
}

// ============================================================
// FR-179: Third-Party Due Diligence Entity
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssessmentStatus {
    NotStarted,
    InProgress,
    Completed,
    ReviewRequired,
    Approved,
    Rejected,
}

impl AssessmentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AssessmentStatus::NotStarted => "NotStarted",
            AssessmentStatus::InProgress => "InProgress",
            AssessmentStatus::Completed => "Completed",
            AssessmentStatus::ReviewRequired => "ReviewRequired",
            AssessmentStatus::Approved => "Approved",
            AssessmentStatus::Rejected => "Rejected",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "NotStarted" => Ok(AssessmentStatus::NotStarted),
            "InProgress" => Ok(AssessmentStatus::InProgress),
            "Completed" => Ok(AssessmentStatus::Completed),
            "ReviewRequired" => Ok(AssessmentStatus::ReviewRequired),
            "Approved" => Ok(AssessmentStatus::Approved),
            "Rejected" => Ok(AssessmentStatus::Rejected),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown assessment status: {s}"
            ))),
        }
    }
}

impl fmt::Display for AssessmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskRating {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl RiskRating {
    pub fn as_str(&self) -> &str {
        match self {
            RiskRating::Low => "Low",
            RiskRating::Medium => "Medium",
            RiskRating::High => "High",
            RiskRating::VeryHigh => "VeryHigh",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Low" => Ok(RiskRating::Low),
            "Medium" => Ok(RiskRating::Medium),
            "High" => Ok(RiskRating::High),
            "VeryHigh" => Ok(RiskRating::VeryHigh),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown risk rating: {s}"
            ))),
        }
    }
}

impl fmt::Display for RiskRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThirdPartyAssessment {
    id: ThirdPartyAssessmentId,
    vendor_name: String,
    vendor_code: String,
    vendor_type: String, // e.g., "Payment Processor", "Cloud Provider", "Audit Firm"
    aml_screening_done: bool,
    aml_screening_date: Option<DateTime<Utc>>,
    kyc_documentation_complete: bool,
    kyc_verification_date: Option<DateTime<Utc>>,
    contract_in_place: bool,
    sla_review_done: bool,
    cybersecurity_assessment: Option<String>,
    data_protection_assessment: Option<String>,
    risk_rating: RiskRating,
    assessment_status: AssessmentStatus,
    reviewer: String,
    review_date: Option<DateTime<Utc>>,
    next_review_date: Option<DateTime<Utc>>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ThirdPartyAssessment {
    pub fn new(
        vendor_name: String,
        vendor_code: String,
        vendor_type: String,
        reviewer: String,
    ) -> Result<Self, DomainError> {
        if vendor_name.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Vendor name cannot be empty".to_string(),
            ));
        }
        if vendor_code.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Vendor code cannot be empty".to_string(),
            ));
        }
        if vendor_type.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Vendor type cannot be empty".to_string(),
            ));
        }
        if reviewer.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Reviewer cannot be empty".to_string(),
            ));
        }

        Ok(ThirdPartyAssessment {
            id: ThirdPartyAssessmentId::new(),
            vendor_name,
            vendor_code,
            vendor_type,
            aml_screening_done: false,
            aml_screening_date: None,
            kyc_documentation_complete: false,
            kyc_verification_date: None,
            contract_in_place: false,
            sla_review_done: false,
            cybersecurity_assessment: None,
            data_protection_assessment: None,
            risk_rating: RiskRating::Medium,
            assessment_status: AssessmentStatus::NotStarted,
            reviewer,
            review_date: None,
            next_review_date: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        id: ThirdPartyAssessmentId,
        vendor_name: String,
        vendor_code: String,
        vendor_type: String,
        aml_screening_done: bool,
        aml_screening_date: Option<DateTime<Utc>>,
        kyc_documentation_complete: bool,
        kyc_verification_date: Option<DateTime<Utc>>,
        contract_in_place: bool,
        sla_review_done: bool,
        cybersecurity_assessment: Option<String>,
        data_protection_assessment: Option<String>,
        risk_rating: RiskRating,
        assessment_status: AssessmentStatus,
        reviewer: String,
        review_date: Option<DateTime<Utc>>,
        next_review_date: Option<DateTime<Utc>>,
        notes: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ThirdPartyAssessment {
            id,
            vendor_name,
            vendor_code,
            vendor_type,
            aml_screening_done,
            aml_screening_date,
            kyc_documentation_complete,
            kyc_verification_date,
            contract_in_place,
            sla_review_done,
            cybersecurity_assessment,
            data_protection_assessment,
            risk_rating,
            assessment_status,
            reviewer,
            review_date,
            next_review_date,
            notes,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ThirdPartyAssessmentId {
        &self.id
    }

    pub fn complete_aml_screening(&mut self) -> Result<(), DomainError> {
        self.aml_screening_done = true;
        self.aml_screening_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete_kyc(&mut self) -> Result<(), DomainError> {
        self.kyc_documentation_complete = true;
        self.kyc_verification_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_contract_in_place(&mut self) -> Result<(), DomainError> {
        self.contract_in_place = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_sla_reviewed(&mut self) -> Result<(), DomainError> {
        self.sla_review_done = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn approve_assessment(&mut self) -> Result<(), DomainError> {
        if !self.aml_screening_done || !self.kyc_documentation_complete {
            return Err(DomainError::InvalidComplianceData(
                "Cannot approve without completing AML screening and KYC".to_string(),
            ));
        }
        self.assessment_status = AssessmentStatus::Approved;
        self.review_date = Some(Utc::now());

        let next_review = Utc::now() + chrono::Duration::days(365);
        self.next_review_date = Some(next_review);

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reject_assessment(&mut self, reason: String) -> Result<(), DomainError> {
        if reason.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Rejection reason cannot be empty".to_string(),
            ));
        }
        self.assessment_status = AssessmentStatus::Rejected;
        self.notes = Some(reason);
        self.review_date = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_risk_rating(&mut self, rating: RiskRating) -> Result<(), DomainError> {
        self.risk_rating = rating;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn risk_rating(&self) -> RiskRating {
        self.risk_rating
    }

    pub fn next_review_date(&self) -> Option<DateTime<Utc>> {
        self.next_review_date
    }
}

// ============================================================
// Unit Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== GafiRecommendation Tests ==========

    #[test]
    fn test_gafi_recommendation_new_valid() {
        let gafi = GafiRecommendation::new(
            1,
            "AML/CFT Policy".to_string(),
            "Establish comprehensive AML/CFT policies".to_string(),
            "AML/CFT".to_string(),
            "Compliance Unit".to_string(),
        )
        .expect("Should create valid GAFI recommendation");

        assert_eq!(gafi.recommendation_number, 1);
        assert_eq!(gafi.status, GafiStatus::NotStarted);
        assert_eq!(gafi.title, "AML/CFT Policy");
    }

    #[test]
    fn test_gafi_recommendation_invalid_number() {
        let result = GafiRecommendation::new(
            41,
            "Invalid".to_string(),
            "Description".to_string(),
            "AML/CFT".to_string(),
            "Unit".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_gafi_status_transitions() {
        let mut gafi = GafiRecommendation::new(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "AML/CFT".to_string(),
            "Unit".to_string(),
        )
        .unwrap();

        gafi.set_status(GafiStatus::InProgress).unwrap();
        assert_eq!(gafi.status, GafiStatus::InProgress);

        gafi.set_status(GafiStatus::Compliant).unwrap();
        assert_eq!(gafi.status, GafiStatus::Compliant);
    }

    // ========== InternalAudit Tests ==========

    #[test]
    fn test_internal_audit_new_valid() {
        let start = Utc::now();
        let end = start + chrono::Duration::days(30);

        let audit = InternalAudit::new(
            "AUD-2025-001".to_string(),
            "Annual Audit".to_string(),
            "Full compliance review".to_string(),
            start,
            end,
            "John Doe".to_string(),
        )
        .expect("Should create valid audit");

        assert_eq!(audit.status, AuditStatus::Planned);
        assert_eq!(audit.findings_count, 0);
    }

    #[test]
    fn test_internal_audit_invalid_dates() {
        let start = Utc::now();
        let end = start - chrono::Duration::days(1);

        let result = InternalAudit::new(
            "AUD-001".to_string(),
            "Test".to_string(),
            "Scope".to_string(),
            start,
            end,
            "Lead".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_internal_audit_workflow() {
        let start = Utc::now();
        let end = start + chrono::Duration::days(30);

        let mut audit = InternalAudit::new(
            "AUD-2025-001".to_string(),
            "Test Audit".to_string(),
            "Test Scope".to_string(),
            start,
            end,
            "Lead".to_string(),
        )
        .unwrap();

        audit.start_audit().unwrap();
        assert_eq!(audit.status, AuditStatus::InProgress);

        audit.complete_audit(5, 2).unwrap();
        assert_eq!(audit.status, AuditStatus::Completed);
        assert_eq!(audit.findings_count, 5);
        assert_eq!(audit.critical_findings, 2);
    }

    // ========== ComplianceRisk Tests ==========

    #[test]
    fn test_compliance_risk_new_valid() {
        let risk = ComplianceRisk::new(
            "RISK-001".to_string(),
            "AML Risk".to_string(),
            "Potential AML violation".to_string(),
            "AML".to_string(),
            RiskProbability::High,
            RiskImpact::Major,
            "Compliance Officer".to_string(),
        )
        .expect("Should create valid risk");

        assert_eq!(risk.risk_score(), 16); // High (4) * Major (4)
        assert_eq!(risk.risk_level(), RiskMatrixLevel::Orange);
    }

    #[test]
    fn test_risk_matrix_scoring() {
        let risk = ComplianceRisk::new(
            "R1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            "Cat".to_string(),
            RiskProbability::VeryHigh,
            RiskImpact::Catastrophic,
            "Owner".to_string(),
        )
        .unwrap();

        assert_eq!(risk.risk_score(), 25); // 5 * 5
        assert_eq!(risk.risk_level(), RiskMatrixLevel::Red);
    }

    // ========== ComplianceTraining Tests ==========

    #[test]
    fn test_compliance_training_new_valid() {
        let employee_id = Uuid::new_v4();
        let scheduled = Utc::now() + chrono::Duration::days(7);

        let training = ComplianceTraining::new(
            "TRAIN-AML-2025".to_string(),
            "AML Training".to_string(),
            "Annual AML awareness".to_string(),
            "AML".to_string(),
            "All Staff".to_string(),
            employee_id,
            scheduled,
            12,
        )
        .expect("Should create valid training");

        assert_eq!(training.status, TrainingStatus::Scheduled);
        assert_eq!(training.validity_period_months, 12);
    }

    #[test]
    fn test_training_completion() {
        let employee_id = Uuid::new_v4();
        let scheduled = Utc::now();

        let mut training = ComplianceTraining::new(
            "TRAIN-001".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            "AML".to_string(),
            "Staff".to_string(),
            employee_id,
            scheduled,
            12,
        )
        .unwrap();

        training
            .complete_training(95, "https://cert.example.com".to_string())
            .unwrap();

        assert_eq!(training.status, TrainingStatus::Completed);
        assert_eq!(training.score, Some(95));
        assert!(training.next_renewal_date.is_some());
    }

    // ========== RegulatoryChange Tests ==========

    #[test]
    fn test_regulatory_change_new_valid() {
        let now = Utc::now();
        let effective = now + chrono::Duration::days(30);
        let deadline = effective + chrono::Duration::days(90);

        let change = RegulatoryChange::new(
            "BCT-2025-001".to_string(),
            "New AML Requirements".to_string(),
            "BCT Circular 2025".to_string(),
            "Enhanced AML controls required".to_string(),
            effective,
            deadline,
            ChangeImpactLevel::High,
            "Update AML procedures".to_string(),
            "Compliance".to_string(),
        )
        .expect("Should create valid change");

        assert_eq!(change.status, "Received");
        assert_eq!(change.implementation_progress, 0);
    }

    #[test]
    fn test_regulatory_change_progress() {
        let now = Utc::now();
        let effective = now + chrono::Duration::days(30);
        let deadline = effective + chrono::Duration::days(90);

        let mut change = RegulatoryChange::new(
            "BCT-2025-001".to_string(),
            "Test Change".to_string(),
            "Source".to_string(),
            "Description".to_string(),
            effective,
            deadline,
            ChangeImpactLevel::Critical,
            "Action".to_string(),
            "Unit".to_string(),
        )
        .unwrap();

        change.update_progress(50).unwrap();
        assert_eq!(change.implementation_progress, 50);
        assert_eq!(change.status, "Implemented");

        change.update_progress(100).unwrap();
        assert_eq!(change.status, "Compliant");
    }

    // ========== ComplianceIncident Tests ==========

    #[test]
    fn test_compliance_incident_new_valid() {
        let incident = ComplianceIncident::new(
            "INC-2025-001".to_string(),
            "Potential AML Violation".to_string(),
            "Suspicious transaction detected".to_string(),
            "AML".to_string(),
            IncidentSeverity::Major,
        )
        .expect("Should create valid incident");

        assert_eq!(incident.severity(), IncidentSeverity::Major);
        assert!(incident.closed_at().is_none());
    }

    #[test]
    fn test_compliance_incident_workflow() {
        let mut incident = ComplianceIncident::new(
            "INC-001".to_string(),
            "Test".to_string(),
            "Description".to_string(),
            "AML".to_string(),
            IncidentSeverity::Critical,
        )
        .unwrap();

        incident.report_incident().unwrap();
        assert!(incident.reported_at.is_some());

        incident.notify_regulator().unwrap();
        assert!(incident.reported_to_regulator);

        incident
            .close_incident(
                "Root cause identified".to_string(),
                "Remediation completed".to_string(),
            )
            .unwrap();
        assert!(incident.closed_at.is_some());
    }

    // ========== WhistleblowerReport Tests ==========

    #[test]
    fn test_whistleblower_report_anonymous() {
        let report = WhistleblowerReport::new(
            "WB-2025-001".to_string(),
            "Fraud".to_string(),
            "Suspected internal fraud".to_string(),
            true,
            None,
            "Compliance Officer".to_string(),
        )
        .expect("Should create anonymous report");

        assert!(report.is_anonymous);
        assert_eq!(report.status, WhistleblowerStatus::Received);
    }

    #[test]
    fn test_whistleblower_report_non_anonymous() {
        let report = WhistleblowerReport::new(
            "WB-001".to_string(),
            "Fraud".to_string(),
            "Description".to_string(),
            false,
            Some("encrypted_email".to_string()),
            "Officer".to_string(),
        )
        .expect("Should create non-anonymous report");

        assert!(!report.is_anonymous);
        assert!(report.contact_email_encrypted.is_some());
    }

    #[test]
    fn test_whistleblower_report_workflow() {
        let mut report = WhistleblowerReport::new(
            "WB-001".to_string(),
            "AML Violation".to_string(),
            "Description".to_string(),
            true,
            None,
            "Officer".to_string(),
        )
        .unwrap();

        report.start_investigation().unwrap();
        assert_eq!(report.status, WhistleblowerStatus::UnderInvestigation);

        report
            .substantiate("Evidence found".to_string())
            .unwrap();
        assert_eq!(report.status, WhistleblowerStatus::Substantiated);

        report.close_report("Actions taken".to_string()).unwrap();
        assert_eq!(report.status, WhistleblowerStatus::Closed);
    }

    // ========== ThirdPartyAssessment Tests ==========

    #[test]
    fn test_third_party_assessment_new_valid() {
        let assessment = ThirdPartyAssessment::new(
            "Example Payment Corp".to_string(),
            "VENDOR-001".to_string(),
            "Payment Processor".to_string(),
            "Jane Smith".to_string(),
        )
        .expect("Should create valid assessment");

        assert_eq!(assessment.assessment_status, AssessmentStatus::NotStarted);
        assert_eq!(assessment.risk_rating, RiskRating::Medium);
        assert!(!assessment.aml_screening_done);
    }

    #[test]
    fn test_third_party_assessment_workflow() {
        let mut assessment = ThirdPartyAssessment::new(
            "Payment Corp".to_string(),
            "VENDOR-001".to_string(),
            "Payment Processor".to_string(),
            "Reviewer".to_string(),
        )
        .unwrap();

        assessment.complete_aml_screening().unwrap();
        assert!(assessment.aml_screening_done);

        assessment.complete_kyc().unwrap();
        assert!(assessment.kyc_documentation_complete);

        assessment.mark_contract_in_place().unwrap();
        assert!(assessment.contract_in_place);

        assessment.approve_assessment().unwrap();
        assert_eq!(assessment.assessment_status, AssessmentStatus::Approved);
        assert!(assessment.next_review_date.is_some());
    }

    #[test]
    fn test_third_party_assessment_cannot_approve_without_aml() {
        let mut assessment = ThirdPartyAssessment::new(
            "Vendor".to_string(),
            "V-001".to_string(),
            "Type".to_string(),
            "Reviewer".to_string(),
        )
        .unwrap();

        let result = assessment.approve_assessment();
        assert!(result.is_err());
    }

    // ========== Enum Conversion Tests ==========

    #[test]
    fn test_gafi_status_conversion() {
        assert_eq!(GafiStatus::from_str_type("Compliant").unwrap(), GafiStatus::Compliant);
        assert!(GafiStatus::from_str_type("Invalid").is_err());
    }

    #[test]
    fn test_risk_probability_conversion() {
        assert_eq!(RiskProbability::High.as_score(), 4);
        assert_eq!(RiskProbability::from_str_type("High").unwrap(), RiskProbability::High);
    }

    #[test]
    fn test_risk_matrix_level_calculation() {
        assert_eq!(RiskMatrixLevel::from_score(3), RiskMatrixLevel::Green);
        assert_eq!(RiskMatrixLevel::from_score(10), RiskMatrixLevel::Yellow);
        assert_eq!(RiskMatrixLevel::from_score(15), RiskMatrixLevel::Orange);
        assert_eq!(RiskMatrixLevel::from_score(25), RiskMatrixLevel::Red);
    }

    #[test]
    fn test_incident_severity_conversion() {
        assert_eq!(
            IncidentSeverity::from_str_type("Critical").unwrap(),
            IncidentSeverity::Critical
        );
        assert!(IncidentSeverity::from_str_type("Unknown").is_err());
    }
}
