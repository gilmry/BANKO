use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReportId(Uuid);

impl ReportId {
    pub fn new() -> Self {
        ReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        ReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemplateId(Uuid);

impl TemplateId {
    pub fn new() -> Self {
        TemplateId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        TemplateId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TemplateId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TemplateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportType {
    Weekly,
    Monthly,
    Quarterly,
    Annual,
    AdHoc,
}

impl ReportType {
    pub fn as_str(&self) -> &str {
        match self {
            ReportType::Weekly => "Weekly",
            ReportType::Monthly => "Monthly",
            ReportType::Quarterly => "Quarterly",
            ReportType::Annual => "Annual",
            ReportType::AdHoc => "AdHoc",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Weekly" => Ok(ReportType::Weekly),
            "Monthly" => Ok(ReportType::Monthly),
            "Quarterly" => Ok(ReportType::Quarterly),
            "Annual" => Ok(ReportType::Annual),
            "AdHoc" => Ok(ReportType::AdHoc),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown report type: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportStatus {
    Draft,
    Generated,
    Validated,
    Submitted,
    Acknowledged,
    Rejected,
}

impl ReportStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ReportStatus::Draft => "Draft",
            ReportStatus::Generated => "Generated",
            ReportStatus::Validated => "Validated",
            ReportStatus::Submitted => "Submitted",
            ReportStatus::Acknowledged => "Acknowledged",
            ReportStatus::Rejected => "Rejected",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Draft" => Ok(ReportStatus::Draft),
            "Generated" => Ok(ReportStatus::Generated),
            "Validated" => Ok(ReportStatus::Validated),
            "Submitted" => Ok(ReportStatus::Submitted),
            "Acknowledged" => Ok(ReportStatus::Acknowledged),
            "Rejected" => Ok(ReportStatus::Rejected),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown report status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportFrequency {
    Weekly,
    Monthly,
    Quarterly,
    Annual,
}

impl ReportFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            ReportFrequency::Weekly => "Weekly",
            ReportFrequency::Monthly => "Monthly",
            ReportFrequency::Quarterly => "Quarterly",
            ReportFrequency::Annual => "Annual",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Weekly" => Ok(ReportFrequency::Weekly),
            "Monthly" => Ok(ReportFrequency::Monthly),
            "Quarterly" => Ok(ReportFrequency::Quarterly),
            "Annual" => Ok(ReportFrequency::Annual),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown report frequency: {s}"
            ))),
        }
    }
}

// --- Aggregates ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryReport {
    report_id: ReportId,
    report_type: ReportType,
    period_start: NaiveDate,
    period_end: NaiveDate,
    template_id: TemplateId,
    template_version: String,
    data: String,
    status: ReportStatus,
    generated_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
    acknowledged_at: Option<DateTime<Utc>>,
    rejection_reason: Option<String>,
    generated_by: Uuid,
}

impl RegulatoryReport {
    pub fn new(
        report_type: ReportType,
        period_start: NaiveDate,
        period_end: NaiveDate,
        template_id: TemplateId,
        template_version: String,
        data: String,
        generated_by: Uuid,
    ) -> Result<Self, DomainError> {
        if period_end <= period_start {
            return Err(DomainError::InvalidReport(
                "period_end must be after period_start".to_string(),
            ));
        }
        if template_version.is_empty() {
            return Err(DomainError::InvalidReport(
                "template_version is required".to_string(),
            ));
        }
        if data.is_empty() {
            return Err(DomainError::InvalidReport(
                "data is required".to_string(),
            ));
        }

        Ok(RegulatoryReport {
            report_id: ReportId::new(),
            report_type,
            period_start,
            period_end,
            template_id,
            template_version,
            data,
            status: ReportStatus::Generated,
            generated_at: Utc::now(),
            submitted_at: None,
            acknowledged_at: None,
            rejection_reason: None,
            generated_by,
        })
    }

    /// Reconstruct from persistence (no validation).
    pub fn from_raw(
        report_id: ReportId,
        report_type: ReportType,
        period_start: NaiveDate,
        period_end: NaiveDate,
        template_id: TemplateId,
        template_version: String,
        data: String,
        status: ReportStatus,
        generated_at: DateTime<Utc>,
        submitted_at: Option<DateTime<Utc>>,
        acknowledged_at: Option<DateTime<Utc>>,
        rejection_reason: Option<String>,
        generated_by: Uuid,
    ) -> Self {
        RegulatoryReport {
            report_id,
            report_type,
            period_start,
            period_end,
            template_id,
            template_version,
            data,
            status,
            generated_at,
            submitted_at,
            acknowledged_at,
            rejection_reason,
            generated_by,
        }
    }

    // --- State transitions ---

    pub fn validate(&mut self) -> Result<(), DomainError> {
        if self.status != ReportStatus::Generated {
            return Err(DomainError::InvalidReport(format!(
                "Cannot validate report in status {}",
                self.status.as_str()
            )));
        }
        self.status = ReportStatus::Validated;
        Ok(())
    }

    pub fn submit(&mut self) -> Result<(), DomainError> {
        if self.status != ReportStatus::Validated {
            return Err(DomainError::InvalidReport(format!(
                "Cannot submit report in status {}",
                self.status.as_str()
            )));
        }
        self.status = ReportStatus::Submitted;
        self.submitted_at = Some(Utc::now());
        Ok(())
    }

    pub fn acknowledge(&mut self) -> Result<(), DomainError> {
        if self.status != ReportStatus::Submitted {
            return Err(DomainError::InvalidReport(format!(
                "Cannot acknowledge report in status {}",
                self.status.as_str()
            )));
        }
        self.status = ReportStatus::Acknowledged;
        self.acknowledged_at = Some(Utc::now());
        Ok(())
    }

    pub fn reject(&mut self, reason: String) -> Result<(), DomainError> {
        if self.status != ReportStatus::Submitted {
            return Err(DomainError::InvalidReport(format!(
                "Cannot reject report in status {}",
                self.status.as_str()
            )));
        }
        self.status = ReportStatus::Rejected;
        self.rejection_reason = Some(reason);
        Ok(())
    }

    // --- Accessors ---

    pub fn report_id(&self) -> &ReportId {
        &self.report_id
    }
    pub fn report_type(&self) -> ReportType {
        self.report_type
    }
    pub fn period_start(&self) -> NaiveDate {
        self.period_start
    }
    pub fn period_end(&self) -> NaiveDate {
        self.period_end
    }
    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }
    pub fn template_version(&self) -> &str {
        &self.template_version
    }
    pub fn data(&self) -> &str {
        &self.data
    }
    pub fn status(&self) -> ReportStatus {
        self.status
    }
    pub fn generated_at(&self) -> DateTime<Utc> {
        self.generated_at
    }
    pub fn submitted_at(&self) -> Option<DateTime<Utc>> {
        self.submitted_at
    }
    pub fn acknowledged_at(&self) -> Option<DateTime<Utc>> {
        self.acknowledged_at
    }
    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }
    pub fn generated_by(&self) -> Uuid {
        self.generated_by
    }
}

// --- ReportTemplate ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    template_id: TemplateId,
    name: String,
    report_type: ReportType,
    version: String,
    definition: String,
    is_active: bool,
}

impl ReportTemplate {
    pub fn new(
        name: String,
        report_type: ReportType,
        version: String,
        definition: String,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidReportTemplate(
                "name is required".to_string(),
            ));
        }
        if version.is_empty() {
            return Err(DomainError::InvalidReportTemplate(
                "version is required".to_string(),
            ));
        }
        if definition.is_empty() {
            return Err(DomainError::InvalidReportTemplate(
                "definition is required".to_string(),
            ));
        }

        Ok(ReportTemplate {
            template_id: TemplateId::new(),
            name,
            report_type,
            version,
            definition,
            is_active: true,
        })
    }

    pub fn from_raw(
        template_id: TemplateId,
        name: String,
        report_type: ReportType,
        version: String,
        definition: String,
        is_active: bool,
    ) -> Self {
        ReportTemplate {
            template_id,
            name,
            report_type,
            version,
            definition,
            is_active,
        }
    }

    // --- Accessors ---

    pub fn template_id(&self) -> &TemplateId {
        &self.template_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn report_type(&self) -> ReportType {
        self.report_type
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn definition(&self) -> &str {
        &self.definition
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

// --- IFRS 9 Report Data (GOV-08 prep) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ifrs9ReportData {
    pub stage1_count: i64,
    pub stage1_ecl: f64,
    pub stage2_count: i64,
    pub stage2_ecl: f64,
    pub stage3_count: i64,
    pub stage3_ecl: f64,
    pub total_ecl: f64,
}

impl Ifrs9ReportData {
    pub fn new(
        stage1_count: i64,
        stage1_ecl: f64,
        stage2_count: i64,
        stage2_ecl: f64,
        stage3_count: i64,
        stage3_ecl: f64,
    ) -> Self {
        Ifrs9ReportData {
            stage1_count,
            stage1_ecl,
            stage2_count,
            stage2_ecl,
            stage3_count,
            stage3_ecl,
            total_ecl: stage1_ecl + stage2_ecl + stage3_ecl,
        }
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use uuid::Uuid;

    fn make_report() -> RegulatoryReport {
        RegulatoryReport::new(
            ReportType::Monthly,
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            TemplateId::new(),
            "1.0".to_string(),
            r#"{"sections":["balance_sheet"]}"#.to_string(),
            Uuid::new_v4(),
        )
        .unwrap()
    }

    #[test]
    fn test_report_lifecycle_happy_path() {
        let mut report = make_report();
        assert_eq!(report.status(), ReportStatus::Generated);

        report.validate().unwrap();
        assert_eq!(report.status(), ReportStatus::Validated);

        report.submit().unwrap();
        assert_eq!(report.status(), ReportStatus::Submitted);
        assert!(report.submitted_at().is_some());

        report.acknowledge().unwrap();
        assert_eq!(report.status(), ReportStatus::Acknowledged);
        assert!(report.acknowledged_at().is_some());
    }

    #[test]
    fn test_report_rejection() {
        let mut report = make_report();
        report.validate().unwrap();
        report.submit().unwrap();

        report.reject("Data inconsistency".to_string()).unwrap();
        assert_eq!(report.status(), ReportStatus::Rejected);
        assert_eq!(report.rejection_reason(), Some("Data inconsistency"));
    }

    #[test]
    fn test_invalid_transition_validate_from_submitted() {
        let mut report = make_report();
        report.validate().unwrap();
        report.submit().unwrap();

        let err = report.validate().unwrap_err();
        assert!(err.to_string().contains("Cannot validate"));
    }

    #[test]
    fn test_invalid_transition_submit_from_generated() {
        let mut report = make_report();
        let err = report.submit().unwrap_err();
        assert!(err.to_string().contains("Cannot submit"));
    }

    #[test]
    fn test_invalid_transition_acknowledge_from_generated() {
        let report = make_report();
        let mut r = report;
        let err = r.acknowledge().unwrap_err();
        assert!(err.to_string().contains("Cannot acknowledge"));
    }

    #[test]
    fn test_invalid_transition_reject_from_validated() {
        let mut report = make_report();
        report.validate().unwrap();
        let err = report.reject("reason".to_string()).unwrap_err();
        assert!(err.to_string().contains("Cannot reject"));
    }

    #[test]
    fn test_invalid_period() {
        let result = RegulatoryReport::new(
            ReportType::Monthly,
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            TemplateId::new(),
            "1.0".to_string(),
            r#"{"data":true}"#.to_string(),
            Uuid::new_v4(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_template_creation() {
        let template = ReportTemplate::new(
            "Monthly BCT Report".to_string(),
            ReportType::Monthly,
            "1.0".to_string(),
            r#"{"sections":["balance_sheet"]}"#.to_string(),
        )
        .unwrap();

        assert_eq!(template.name(), "Monthly BCT Report");
        assert_eq!(template.report_type(), ReportType::Monthly);
        assert_eq!(template.version(), "1.0");
        assert!(template.is_active());
    }

    #[test]
    fn test_template_empty_name_rejected() {
        let result = ReportTemplate::new(
            "".to_string(),
            ReportType::Weekly,
            "1.0".to_string(),
            "{}".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ifrs9_report_data() {
        let data = Ifrs9ReportData::new(100, 50000.0, 20, 30000.0, 5, 80000.0);
        assert_eq!(data.stage1_count, 100);
        assert_eq!(data.stage2_count, 20);
        assert_eq!(data.stage3_count, 5);
        assert!((data.total_ecl - 160000.0).abs() < 0.01);
    }

    #[test]
    fn test_report_type_roundtrip() {
        for rt in [
            ReportType::Weekly,
            ReportType::Monthly,
            ReportType::Quarterly,
            ReportType::Annual,
            ReportType::AdHoc,
        ] {
            let s = rt.as_str();
            let parsed = ReportType::from_str_type(s).unwrap();
            assert_eq!(rt, parsed);
        }
    }

    #[test]
    fn test_report_status_roundtrip() {
        for st in [
            ReportStatus::Draft,
            ReportStatus::Generated,
            ReportStatus::Validated,
            ReportStatus::Submitted,
            ReportStatus::Acknowledged,
            ReportStatus::Rejected,
        ] {
            let s = st.as_str();
            let parsed = ReportStatus::from_str_type(s).unwrap();
            assert_eq!(st, parsed);
        }
    }

    #[test]
    fn test_from_raw_reconstruction() {
        let id = ReportId::new();
        let tid = TemplateId::new();
        let gen_by = Uuid::new_v4();
        let now = Utc::now();

        let report = RegulatoryReport::from_raw(
            id.clone(),
            ReportType::Quarterly,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            tid.clone(),
            "2.0".to_string(),
            "{}".to_string(),
            ReportStatus::Submitted,
            now,
            Some(now),
            None,
            None,
            gen_by,
        );

        assert_eq!(report.report_id(), &id);
        assert_eq!(report.template_id(), &tid);
        assert_eq!(report.status(), ReportStatus::Submitted);
        assert_eq!(report.generated_by(), gen_by);
    }
}
