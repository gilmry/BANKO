use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Value Objects / Newtypes
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScheduledReportId(Uuid);

impl ScheduledReportId {
    pub fn new() -> Self {
        ScheduledReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        ScheduledReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ScheduledReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ScheduledReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReportDistributionId(Uuid);

impl ReportDistributionId {
    pub fn new() -> Self {
        ReportDistributionId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        ReportDistributionId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ReportDistributionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReportDistributionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReportArchiveId(Uuid);

impl ReportArchiveId {
    pub fn new() -> Self {
        ReportArchiveId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        ReportArchiveId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ReportArchiveId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReportArchiveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AdHocReportId(Uuid);

impl AdHocReportId {
    pub fn new() -> Self {
        AdHocReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        AdHocReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AdHocReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AdHocReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaxReportId(Uuid);

impl TaxReportId {
    pub fn new() -> Self {
        TaxReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        TaxReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TaxReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaxReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ifrs9ReportId(Uuid);

impl Ifrs9ReportId {
    pub fn new() -> Self {
        Ifrs9ReportId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        Ifrs9ReportId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for Ifrs9ReportId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Ifrs9ReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    Annual,
}

impl ScheduleFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            ScheduleFrequency::Daily => "Daily",
            ScheduleFrequency::Weekly => "Weekly",
            ScheduleFrequency::BiWeekly => "BiWeekly",
            ScheduleFrequency::Monthly => "Monthly",
            ScheduleFrequency::Quarterly => "Quarterly",
            ScheduleFrequency::Annual => "Annual",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Daily" => Ok(ScheduleFrequency::Daily),
            "Weekly" => Ok(ScheduleFrequency::Weekly),
            "BiWeekly" => Ok(ScheduleFrequency::BiWeekly),
            "Monthly" => Ok(ScheduleFrequency::Monthly),
            "Quarterly" => Ok(ScheduleFrequency::Quarterly),
            "Annual" => Ok(ScheduleFrequency::Annual),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown schedule frequency: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DistributionChannel {
    Email,
    Portal,
    Sftp,
    Api,
}

impl DistributionChannel {
    pub fn as_str(&self) -> &str {
        match self {
            DistributionChannel::Email => "Email",
            DistributionChannel::Portal => "Portal",
            DistributionChannel::Sftp => "Sftp",
            DistributionChannel::Api => "Api",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Email" => Ok(DistributionChannel::Email),
            "Portal" => Ok(DistributionChannel::Portal),
            "Sftp" => Ok(DistributionChannel::Sftp),
            "Api" => Ok(DistributionChannel::Api),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown distribution channel: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxReportType {
    Tva,           // Taxe sur la Valeur Ajoutée
    WithholdingTax, // Retenues à la source
    AnnualTaxSummary,
}

impl TaxReportType {
    pub fn as_str(&self) -> &str {
        match self {
            TaxReportType::Tva => "Tva",
            TaxReportType::WithholdingTax => "WithholdingTax",
            TaxReportType::AnnualTaxSummary => "AnnualTaxSummary",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Tva" => Ok(TaxReportType::Tva),
            "WithholdingTax" => Ok(TaxReportType::WithholdingTax),
            "AnnualTaxSummary" => Ok(TaxReportType::AnnualTaxSummary),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown tax report type: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreditStage {
    Stage1,
    Stage2,
    Stage3,
}

impl CreditStage {
    pub fn as_str(&self) -> &str {
        match self {
            CreditStage::Stage1 => "Stage1",
            CreditStage::Stage2 => "Stage2",
            CreditStage::Stage3 => "Stage3",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Stage1" => Ok(CreditStage::Stage1),
            "Stage2" => Ok(CreditStage::Stage2),
            "Stage3" => Ok(CreditStage::Stage3),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown credit stage: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReportFormatType {
    Json,
    Csv,
    Xbrl,
    Xml,
    Excel,
    Pdf,
}

impl ReportFormatType {
    pub fn as_str(&self) -> &str {
        match self {
            ReportFormatType::Json => "Json",
            ReportFormatType::Csv => "Csv",
            ReportFormatType::Xbrl => "Xbrl",
            ReportFormatType::Xml => "Xml",
            ReportFormatType::Excel => "Excel",
            ReportFormatType::Pdf => "Pdf",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Json" => Ok(ReportFormatType::Json),
            "Csv" => Ok(ReportFormatType::Csv),
            "Xbrl" => Ok(ReportFormatType::Xbrl),
            "Xml" => Ok(ReportFormatType::Xml),
            "Excel" => Ok(ReportFormatType::Excel),
            "Pdf" => Ok(ReportFormatType::Pdf),
            _ => Err(DomainError::InvalidReport(format!(
                "Unknown report format: {s}"
            ))),
        }
    }
}

// ============================================================
// ScheduledReport Aggregate (BMAD: Scheduled report generation)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReport {
    scheduled_report_id: ScheduledReportId,
    name: String,
    description: Option<String>,
    report_type: String, // Reference to report type (e.g., "Prudential", "AML", "Accounting")
    frequency: ScheduleFrequency,
    cron_expression: Option<String>, // For advanced scheduling (e.g., "0 9 * * MON")
    next_run: Option<DateTime<Utc>>,
    last_run: Option<DateTime<Utc>>,
    is_active: bool,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ScheduledReport {
    pub fn new(
        name: String,
        description: Option<String>,
        report_type: String,
        frequency: ScheduleFrequency,
        cron_expression: Option<String>,
        next_run: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidReport("name is required".to_string()));
        }
        if report_type.is_empty() {
            return Err(DomainError::InvalidReport("report_type is required".to_string()));
        }

        Ok(ScheduledReport {
            scheduled_report_id: ScheduledReportId::new(),
            name,
            description,
            report_type,
            frequency,
            cron_expression,
            next_run,
            last_run: None,
            is_active: true,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        scheduled_report_id: ScheduledReportId,
        name: String,
        description: Option<String>,
        report_type: String,
        frequency: ScheduleFrequency,
        cron_expression: Option<String>,
        next_run: Option<DateTime<Utc>>,
        last_run: Option<DateTime<Utc>>,
        is_active: bool,
        created_by: Uuid,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ScheduledReport {
            scheduled_report_id,
            name,
            description,
            report_type,
            frequency,
            cron_expression,
            next_run,
            last_run,
            is_active,
            created_by,
            created_at,
            updated_at,
        }
    }

    pub fn mark_executed(&mut self) -> Result<(), DomainError> {
        self.last_run = Some(Utc::now());
        // In production, calculate next_run based on frequency/cron
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.is_active = false;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Accessors
    pub fn scheduled_report_id(&self) -> &ScheduledReportId {
        &self.scheduled_report_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn report_type(&self) -> &str {
        &self.report_type
    }
    pub fn frequency(&self) -> ScheduleFrequency {
        self.frequency
    }
    pub fn cron_expression(&self) -> Option<&str> {
        self.cron_expression.as_deref()
    }
    pub fn next_run(&self) -> Option<DateTime<Utc>> {
        self.next_run
    }
    pub fn last_run(&self) -> Option<DateTime<Utc>> {
        self.last_run
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn created_by(&self) -> Uuid {
        self.created_by
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// ============================================================
// ReportDistribution Aggregate (BMAD: Report distribution)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDistribution {
    distribution_id: ReportDistributionId,
    report_id: String,
    channel: DistributionChannel,
    recipients: Vec<String>, // Email addresses, usernames, or API endpoints
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ReportDistribution {
    pub fn new(
        report_id: String,
        channel: DistributionChannel,
        recipients: Vec<String>,
    ) -> Result<Self, DomainError> {
        if report_id.is_empty() {
            return Err(DomainError::InvalidReport("report_id is required".to_string()));
        }
        if recipients.is_empty() {
            return Err(DomainError::InvalidReport(
                "recipients cannot be empty".to_string(),
            ));
        }

        Ok(ReportDistribution {
            distribution_id: ReportDistributionId::new(),
            report_id,
            channel,
            recipients,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn from_raw(
        distribution_id: ReportDistributionId,
        report_id: String,
        channel: DistributionChannel,
        recipients: Vec<String>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ReportDistribution {
            distribution_id,
            report_id,
            channel,
            recipients,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn add_recipient(&mut self, recipient: String) -> Result<(), DomainError> {
        if recipient.is_empty() {
            return Err(DomainError::InvalidReport(
                "recipient cannot be empty".to_string(),
            ));
        }
        if !self.recipients.contains(&recipient) {
            self.recipients.push(recipient);
            self.updated_at = Utc::now();
        }
        Ok(())
    }

    pub fn remove_recipient(&mut self, recipient: &str) -> Result<(), DomainError> {
        self.recipients.retain(|r| r != recipient);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        self.is_active = false;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Accessors
    pub fn distribution_id(&self) -> &ReportDistributionId {
        &self.distribution_id
    }
    pub fn report_id(&self) -> &str {
        &self.report_id
    }
    pub fn channel(&self) -> DistributionChannel {
        self.channel
    }
    pub fn recipients(&self) -> &[String] {
        &self.recipients
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

// ============================================================
// ReportArchive Aggregate (BMAD: Report archival with retention)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportArchive {
    archive_id: ReportArchiveId,
    report_id: String,
    storage_path: String,
    content_hash: String, // SHA256 hash for integrity verification
    format: ReportFormatType,
    size_bytes: i64,
    created_at: DateTime<Utc>,
    retention_until: DateTime<Utc>, // BMAD: 7 years for BCT reports
}

impl ReportArchive {
    pub fn new(
        report_id: String,
        storage_path: String,
        content_hash: String,
        format: ReportFormatType,
        size_bytes: i64,
        retention_years: i64,
    ) -> Result<Self, DomainError> {
        if report_id.is_empty() {
            return Err(DomainError::InvalidReport("report_id is required".to_string()));
        }
        if storage_path.is_empty() {
            return Err(DomainError::InvalidReport(
                "storage_path is required".to_string(),
            ));
        }
        if content_hash.is_empty() {
            return Err(DomainError::InvalidReport(
                "content_hash is required".to_string(),
            ));
        }
        if size_bytes <= 0 {
            return Err(DomainError::InvalidReport(
                "size_bytes must be positive".to_string(),
            ));
        }
        if retention_years <= 0 {
            return Err(DomainError::InvalidReport(
                "retention_years must be positive".to_string(),
            ));
        }

        let created_at = Utc::now();
        let retention_until = created_at
            + chrono::Duration::days(retention_years * 365)
            + chrono::Duration::days(1); // Add 1 day for safety

        Ok(ReportArchive {
            archive_id: ReportArchiveId::new(),
            report_id,
            storage_path,
            content_hash,
            format,
            size_bytes,
            created_at,
            retention_until,
        })
    }

    pub fn from_raw(
        archive_id: ReportArchiveId,
        report_id: String,
        storage_path: String,
        content_hash: String,
        format: ReportFormatType,
        size_bytes: i64,
        created_at: DateTime<Utc>,
        retention_until: DateTime<Utc>,
    ) -> Self {
        ReportArchive {
            archive_id,
            report_id,
            storage_path,
            content_hash,
            format,
            size_bytes,
            created_at,
            retention_until,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.retention_until
    }

    pub fn days_until_expiry(&self) -> i64 {
        let duration = self.retention_until.signed_duration_since(Utc::now());
        duration.num_days()
    }

    // Accessors
    pub fn archive_id(&self) -> &ReportArchiveId {
        &self.archive_id
    }
    pub fn report_id(&self) -> &str {
        &self.report_id
    }
    pub fn storage_path(&self) -> &str {
        &self.storage_path
    }
    pub fn content_hash(&self) -> &str {
        &self.content_hash
    }
    pub fn format(&self) -> ReportFormatType {
        self.format
    }
    pub fn size_bytes(&self) -> i64 {
        self.size_bytes
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn retention_until(&self) -> DateTime<Utc> {
        self.retention_until
    }
}

// ============================================================
// AdHocReport Aggregate (BMAD: Ad-hoc report builder)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdHocReport {
    adhoc_report_id: AdHocReportId,
    name: String,
    description: Option<String>,
    filters: serde_json::Value,
    columns: Vec<String>,
    format: ReportFormatType,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    executed_at: Option<DateTime<Utc>>,
}

impl AdHocReport {
    pub fn new(
        name: String,
        description: Option<String>,
        filters: serde_json::Value,
        columns: Vec<String>,
        format: ReportFormatType,
        created_by: Uuid,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidReport("name is required".to_string()));
        }
        if columns.is_empty() {
            return Err(DomainError::InvalidReport(
                "columns cannot be empty".to_string(),
            ));
        }

        Ok(AdHocReport {
            adhoc_report_id: AdHocReportId::new(),
            name,
            description,
            filters,
            columns,
            format,
            created_by,
            created_at: Utc::now(),
            executed_at: None,
        })
    }

    pub fn from_raw(
        adhoc_report_id: AdHocReportId,
        name: String,
        description: Option<String>,
        filters: serde_json::Value,
        columns: Vec<String>,
        format: ReportFormatType,
        created_by: Uuid,
        created_at: DateTime<Utc>,
        executed_at: Option<DateTime<Utc>>,
    ) -> Self {
        AdHocReport {
            adhoc_report_id,
            name,
            description,
            filters,
            columns,
            format,
            created_by,
            created_at,
            executed_at,
        }
    }

    pub fn mark_executed(&mut self) -> Result<(), DomainError> {
        self.executed_at = Some(Utc::now());
        Ok(())
    }

    // Accessors
    pub fn adhoc_report_id(&self) -> &AdHocReportId {
        &self.adhoc_report_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn filters(&self) -> &serde_json::Value {
        &self.filters
    }
    pub fn columns(&self) -> &[String] {
        &self.columns
    }
    pub fn format(&self) -> ReportFormatType {
        self.format
    }
    pub fn created_by(&self) -> Uuid {
        self.created_by
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn executed_at(&self) -> Option<DateTime<Utc>> {
        self.executed_at
    }
}

// ============================================================
// TaxReport Aggregate (BMAD: Tax reporting - TVA, withholding tax)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxReport {
    tax_report_id: TaxReportId,
    tax_type: TaxReportType,
    period_start: NaiveDate,
    period_end: NaiveDate,
    total_amount: f64,
    tax_amount: f64,
    details: serde_json::Value,
    generated_by: Uuid,
    created_at: DateTime<Utc>,
}

impl TaxReport {
    pub fn new(
        tax_type: TaxReportType,
        period_start: NaiveDate,
        period_end: NaiveDate,
        total_amount: f64,
        tax_amount: f64,
        details: serde_json::Value,
        generated_by: Uuid,
    ) -> Result<Self, DomainError> {
        if period_end <= period_start {
            return Err(DomainError::InvalidReport(
                "period_end must be after period_start".to_string(),
            ));
        }
        if total_amount < 0.0 {
            return Err(DomainError::InvalidReport(
                "total_amount cannot be negative".to_string(),
            ));
        }
        if tax_amount < 0.0 {
            return Err(DomainError::InvalidReport(
                "tax_amount cannot be negative".to_string(),
            ));
        }

        Ok(TaxReport {
            tax_report_id: TaxReportId::new(),
            tax_type,
            period_start,
            period_end,
            total_amount,
            tax_amount,
            details,
            generated_by,
            created_at: Utc::now(),
        })
    }

    pub fn from_raw(
        tax_report_id: TaxReportId,
        tax_type: TaxReportType,
        period_start: NaiveDate,
        period_end: NaiveDate,
        total_amount: f64,
        tax_amount: f64,
        details: serde_json::Value,
        generated_by: Uuid,
        created_at: DateTime<Utc>,
    ) -> Self {
        TaxReport {
            tax_report_id,
            tax_type,
            period_start,
            period_end,
            total_amount,
            tax_amount,
            details,
            generated_by,
            created_at,
        }
    }

    // Accessors
    pub fn tax_report_id(&self) -> &TaxReportId {
        &self.tax_report_id
    }
    pub fn tax_type(&self) -> TaxReportType {
        self.tax_type
    }
    pub fn period_start(&self) -> NaiveDate {
        self.period_start
    }
    pub fn period_end(&self) -> NaiveDate {
        self.period_end
    }
    pub fn total_amount(&self) -> f64 {
        self.total_amount
    }
    pub fn tax_amount(&self) -> f64 {
        self.tax_amount
    }
    pub fn details(&self) -> &serde_json::Value {
        &self.details
    }
    pub fn generated_by(&self) -> Uuid {
        self.generated_by
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// ============================================================
// Ifrs9Report Aggregate (BMAD: IFRS 9 reporting with staging)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagingAnalysis {
    pub stage: CreditStage,
    pub loan_count: i64,
    pub ecl_amount: f64,
    pub probability_of_default: f64,
    pub loss_given_default: f64,
    pub exposure_at_default: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionMatrix {
    pub from_stage: CreditStage,
    pub to_stage: CreditStage,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ifrs9Report {
    ifrs9_report_id: Ifrs9ReportId,
    as_of: NaiveDate,
    staging_analysis: Vec<StagingAnalysis>,
    transition_matrices: Vec<TransitionMatrix>,
    total_ecl: f64,
    generated_by: Uuid,
    created_at: DateTime<Utc>,
}

impl Ifrs9Report {
    pub fn new(
        as_of: NaiveDate,
        staging_analysis: Vec<StagingAnalysis>,
        transition_matrices: Vec<TransitionMatrix>,
        total_ecl: f64,
        generated_by: Uuid,
    ) -> Result<Self, DomainError> {
        if staging_analysis.is_empty() {
            return Err(DomainError::InvalidReport(
                "staging_analysis cannot be empty".to_string(),
            ));
        }
        if total_ecl < 0.0 {
            return Err(DomainError::InvalidReport(
                "total_ecl cannot be negative".to_string(),
            ));
        }

        Ok(Ifrs9Report {
            ifrs9_report_id: Ifrs9ReportId::new(),
            as_of,
            staging_analysis,
            transition_matrices,
            total_ecl,
            generated_by,
            created_at: Utc::now(),
        })
    }

    pub fn from_raw(
        ifrs9_report_id: Ifrs9ReportId,
        as_of: NaiveDate,
        staging_analysis: Vec<StagingAnalysis>,
        transition_matrices: Vec<TransitionMatrix>,
        total_ecl: f64,
        generated_by: Uuid,
        created_at: DateTime<Utc>,
    ) -> Self {
        Ifrs9Report {
            ifrs9_report_id,
            as_of,
            staging_analysis,
            transition_matrices,
            total_ecl,
            generated_by,
            created_at,
        }
    }

    pub fn get_stage_ecl(&self, stage: CreditStage) -> Option<f64> {
        self.staging_analysis
            .iter()
            .find(|s| s.stage == stage)
            .map(|s| s.ecl_amount)
    }

    pub fn get_stage_count(&self, stage: CreditStage) -> Option<i64> {
        self.staging_analysis
            .iter()
            .find(|s| s.stage == stage)
            .map(|s| s.loan_count)
    }

    // Accessors
    pub fn ifrs9_report_id(&self) -> &Ifrs9ReportId {
        &self.ifrs9_report_id
    }
    pub fn as_of(&self) -> NaiveDate {
        self.as_of
    }
    pub fn staging_analysis(&self) -> &[StagingAnalysis] {
        &self.staging_analysis
    }
    pub fn transition_matrices(&self) -> &[TransitionMatrix] {
        &self.transition_matrices
    }
    pub fn total_ecl(&self) -> f64 {
        self.total_ecl
    }
    pub fn generated_by(&self) -> Uuid {
        self.generated_by
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduled_report_creation() {
        let sr = ScheduledReport::new(
            "Daily Prudential Report".to_string(),
            Some("Daily BCT prudential report".to_string()),
            "Prudential".to_string(),
            ScheduleFrequency::Daily,
            Some("0 9 * * *".to_string()),
            None,
            Uuid::new_v4(),
        )
        .unwrap();

        assert_eq!(sr.name(), "Daily Prudential Report");
        assert_eq!(sr.frequency(), ScheduleFrequency::Daily);
        assert!(sr.is_active());
    }

    #[test]
    fn test_scheduled_report_deactivate() {
        let mut sr = ScheduledReport::new(
            "Test Report".to_string(),
            None,
            "Test".to_string(),
            ScheduleFrequency::Monthly,
            None,
            None,
            Uuid::new_v4(),
        )
        .unwrap();

        sr.deactivate().unwrap();
        assert!(!sr.is_active());
    }

    #[test]
    fn test_report_distribution_creation() {
        let dist = ReportDistribution::new(
            "report-001".to_string(),
            DistributionChannel::Email,
            vec!["compliance@bank.tn".to_string()],
        )
        .unwrap();

        assert_eq!(dist.report_id(), "report-001");
        assert_eq!(dist.channel(), DistributionChannel::Email);
        assert_eq!(dist.recipients().len(), 1);
    }

    #[test]
    fn test_report_distribution_add_recipient() {
        let mut dist = ReportDistribution::new(
            "report-001".to_string(),
            DistributionChannel::Email,
            vec!["compliance@bank.tn".to_string()],
        )
        .unwrap();

        dist.add_recipient("audit@bank.tn".to_string()).unwrap();
        assert_eq!(dist.recipients().len(), 2);
    }

    #[test]
    fn test_report_distribution_remove_recipient() {
        let mut dist = ReportDistribution::new(
            "report-001".to_string(),
            DistributionChannel::Email,
            vec![
                "compliance@bank.tn".to_string(),
                "audit@bank.tn".to_string(),
            ],
        )
        .unwrap();

        dist.remove_recipient("audit@bank.tn").unwrap();
        assert_eq!(dist.recipients().len(), 1);
    }

    #[test]
    fn test_report_archive_creation() {
        let archive = ReportArchive::new(
            "report-001".to_string(),
            "s3://reports/2026-03/report-001.xbrl".to_string(),
            "sha256hash".to_string(),
            ReportFormatType::Xbrl,
            102400,
            7,
        )
        .unwrap();

        assert_eq!(archive.report_id(), "report-001");
        assert!(!archive.is_expired());
        assert!(archive.days_until_expiry() > 2500); // Approx 7 years
    }

    #[test]
    fn test_adhoc_report_creation() {
        let adhoc = AdHocReport::new(
            "Customer Transactions".to_string(),
            Some("Monthly transaction summary".to_string()),
            serde_json::json!({"account_type": "Checking"}),
            vec!["account_id".to_string(), "amount".to_string()],
            ReportFormatType::Csv,
            Uuid::new_v4(),
        )
        .unwrap();

        assert_eq!(adhoc.name(), "Customer Transactions");
        assert_eq!(adhoc.columns().len(), 2);
        assert_eq!(adhoc.format(), ReportFormatType::Csv);
    }

    #[test]
    fn test_tax_report_creation() {
        let tax_report = TaxReport::new(
            TaxReportType::Tva,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            1000000.0,
            180000.0,
            serde_json::json!({"vat_rate": 0.18}),
            Uuid::new_v4(),
        )
        .unwrap();

        assert_eq!(tax_report.tax_type(), TaxReportType::Tva);
        assert!((tax_report.tax_amount() - 180000.0).abs() < 0.01);
    }

    #[test]
    fn test_tax_report_invalid_period() {
        let result = TaxReport::new(
            TaxReportType::Tva,
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            1000000.0,
            180000.0,
            serde_json::json!({}),
            Uuid::new_v4(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_ifrs9_report_creation() {
        let staging_analysis = vec![
            StagingAnalysis {
                stage: CreditStage::Stage1,
                loan_count: 100,
                ecl_amount: 50000.0,
                probability_of_default: 0.01,
                loss_given_default: 0.4,
                exposure_at_default: 5000000.0,
            },
            StagingAnalysis {
                stage: CreditStage::Stage2,
                loan_count: 20,
                ecl_amount: 30000.0,
                probability_of_default: 0.05,
                loss_given_default: 0.5,
                exposure_at_default: 1200000.0,
            },
        ];

        let ifrs9 = Ifrs9Report::new(
            NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
            staging_analysis,
            vec![],
            80000.0,
            Uuid::new_v4(),
        )
        .unwrap();

        assert_eq!(ifrs9.get_stage_count(CreditStage::Stage1), Some(100));
        assert_eq!(ifrs9.get_stage_ecl(CreditStage::Stage2), Some(30000.0));
    }

    #[test]
    fn test_schedule_frequency_roundtrip() {
        for freq in [
            ScheduleFrequency::Daily,
            ScheduleFrequency::Weekly,
            ScheduleFrequency::Monthly,
            ScheduleFrequency::Quarterly,
            ScheduleFrequency::Annual,
        ] {
            let s = freq.as_str();
            let parsed = ScheduleFrequency::from_str_type(s).unwrap();
            assert_eq!(freq, parsed);
        }
    }

    #[test]
    fn test_distribution_channel_roundtrip() {
        for channel in [
            DistributionChannel::Email,
            DistributionChannel::Portal,
            DistributionChannel::Sftp,
            DistributionChannel::Api,
        ] {
            let s = channel.as_str();
            let parsed = DistributionChannel::from_str_type(s).unwrap();
            assert_eq!(channel, parsed);
        }
    }

    #[test]
    fn test_tax_report_type_roundtrip() {
        for tax_type in [
            TaxReportType::Tva,
            TaxReportType::WithholdingTax,
            TaxReportType::AnnualTaxSummary,
        ] {
            let s = tax_type.as_str();
            let parsed = TaxReportType::from_str_type(s).unwrap();
            assert_eq!(tax_type, parsed);
        }
    }

    #[test]
    fn test_credit_stage_roundtrip() {
        for stage in [
            CreditStage::Stage1,
            CreditStage::Stage2,
            CreditStage::Stage3,
        ] {
            let s = stage.as_str();
            let parsed = CreditStage::from_str_type(s).unwrap();
            assert_eq!(stage, parsed);
        }
    }

    #[test]
    fn test_report_format_type_roundtrip() {
        for fmt in [
            ReportFormatType::Json,
            ReportFormatType::Csv,
            ReportFormatType::Xbrl,
            ReportFormatType::Xml,
            ReportFormatType::Excel,
            ReportFormatType::Pdf,
        ] {
            let s = fmt.as_str();
            let parsed = ReportFormatType::from_str_type(s).unwrap();
            assert_eq!(fmt, parsed);
        }
    }
}
