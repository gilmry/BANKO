/// goAML Integration for CTAF (Commission Tunisienne des Analyses Financières)
///
/// Handles Suspicious Transaction Reports (STR/DOS) submission to Tunisia's Financial
/// Intelligence Unit through the UN goAML platform's XML format.
///
/// Reference: UN FATF goAML reporting standard
/// Jurisdiction: Tunisia (Country Code: TN)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoAmlError {
    InvalidReport(String),
    SubmissionFailed(String),
    XmlGenerationError(String),
    NetworkError(String),
    AuthenticationError(String),
    ValidationError(String),
}

impl fmt::Display for GoAmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoAmlError::InvalidReport(msg) => write!(f, "Invalid report: {}", msg),
            GoAmlError::SubmissionFailed(msg) => write!(f, "Submission failed: {}", msg),
            GoAmlError::XmlGenerationError(msg) => write!(f, "XML generation error: {}", msg),
            GoAmlError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            GoAmlError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            GoAmlError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for GoAmlError {}

// ============================================================================
// ENUMS & VALUE OBJECTS
// ============================================================================

/// Report types as defined by goAML standard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoAmlReportType {
    /// Suspicious Transaction Report
    STR,
    /// Suspicious Activity Report
    SAR,
    /// Currency Transaction Report (large cash transactions)
    CTR,
}

impl GoAmlReportType {
    pub fn as_str(&self) -> &str {
        match self {
            GoAmlReportType::STR => "STR",
            GoAmlReportType::SAR => "SAR",
            GoAmlReportType::CTR => "CTR",
        }
    }
}

impl fmt::Display for GoAmlReportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Risk level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &str {
        match self {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Reporting entity (financial institution filing the report)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingEntity {
    /// Organization name
    pub name: String,
    /// Registration number (e.g., Central Bank ID)
    pub registration_number: String,
    /// Country code (ISO 3166-1 alpha-2, e.g., "TN" for Tunisia)
    pub country: String,
    /// Contact person name
    pub contact_person: String,
    /// Contact phone number
    pub phone: String,
    /// Contact email
    pub email: String,
}

impl ReportingEntity {
    pub fn new(
        name: String,
        registration_number: String,
        country: String,
        contact_person: String,
        phone: String,
        email: String,
    ) -> Result<Self, GoAmlError> {
        if name.trim().is_empty() {
            return Err(GoAmlError::ValidationError("Name cannot be empty".to_string()));
        }
        if registration_number.trim().is_empty() {
            return Err(GoAmlError::ValidationError(
                "Registration number cannot be empty".to_string(),
            ));
        }
        if email.trim().is_empty() || !email.contains('@') {
            return Err(GoAmlError::ValidationError("Invalid email address".to_string()));
        }

        Ok(ReportingEntity {
            name,
            registration_number,
            country,
            contact_person,
            phone,
            email,
        })
    }
}

/// Suspect person information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoAmlSuspect {
    /// Full name of the suspect
    pub full_name: String,
    /// Date of birth (YYYY-MM-DD format)
    pub date_of_birth: Option<String>,
    /// Nationality (country code)
    pub nationality: Option<String>,
    /// ID number (passport, national ID, etc.)
    pub id_number: Option<String>,
    /// Physical address
    pub address: Option<String>,
}

impl GoAmlSuspect {
    pub fn new(full_name: String) -> Result<Self, GoAmlError> {
        if full_name.trim().is_empty() {
            return Err(GoAmlError::ValidationError("Full name cannot be empty".to_string()));
        }

        Ok(GoAmlSuspect {
            full_name,
            date_of_birth: None,
            nationality: None,
            id_number: None,
            address: None,
        })
    }

    pub fn with_dob(mut self, dob: String) -> Self {
        self.date_of_birth = Some(dob);
        self
    }

    pub fn with_nationality(mut self, nationality: String) -> Self {
        self.nationality = Some(nationality);
        self
    }

    pub fn with_id_number(mut self, id_number: String) -> Self {
        self.id_number = Some(id_number);
        self
    }

    pub fn with_address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }
}

/// Single transaction within a report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoAmlTransaction {
    /// Unique transaction identifier
    pub transaction_id: String,
    /// Transaction date and time
    pub date: DateTime<Utc>,
    /// Transaction amount
    pub amount: f64,
    /// Currency code (ISO 4217, e.g., "TND")
    pub currency: String,
    /// Type of transaction (e.g., "Wire Transfer", "Deposit", "Cash Withdrawal")
    pub transaction_type: String,
    /// Originating account or party
    pub from_account: String,
    /// Destination account or party
    pub to_account: String,
}

impl GoAmlTransaction {
    pub fn new(
        transaction_id: String,
        date: DateTime<Utc>,
        amount: f64,
        currency: String,
        transaction_type: String,
        from_account: String,
        to_account: String,
    ) -> Result<Self, GoAmlError> {
        if transaction_id.trim().is_empty() {
            return Err(GoAmlError::ValidationError(
                "Transaction ID cannot be empty".to_string(),
            ));
        }
        if amount <= 0.0 {
            return Err(GoAmlError::ValidationError(
                "Amount must be positive".to_string(),
            ));
        }
        if currency.trim().is_empty() {
            return Err(GoAmlError::ValidationError(
                "Currency cannot be empty".to_string(),
            ));
        }

        Ok(GoAmlTransaction {
            transaction_id,
            date,
            amount,
            currency,
            transaction_type,
            from_account,
            to_account,
        })
    }
}

/// Complete goAML report ready for submission to CTAF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoAmlReport {
    /// Unique report identifier (UUID)
    pub report_id: String,
    /// Report type (STR, SAR, CTR)
    pub report_type: GoAmlReportType,
    /// Reporting institution
    pub reporting_entity: ReportingEntity,
    /// Transactions being reported
    pub transactions: Vec<GoAmlTransaction>,
    /// Suspects involved
    pub suspects: Vec<GoAmlSuspect>,
    /// Narrative explaining suspicion
    pub narrative: String,
    /// Risk level classification
    pub risk_level: RiskLevel,
    /// Report submission date
    pub submission_date: DateTime<Utc>,
}

impl GoAmlReport {
    pub fn new(
        report_type: GoAmlReportType,
        reporting_entity: ReportingEntity,
        narrative: String,
        risk_level: RiskLevel,
    ) -> Result<Self, GoAmlError> {
        if narrative.trim().is_empty() {
            return Err(GoAmlError::ValidationError(
                "Narrative cannot be empty".to_string(),
            ));
        }

        Ok(GoAmlReport {
            report_id: Uuid::new_v4().to_string(),
            report_type,
            reporting_entity,
            transactions: Vec::new(),
            suspects: Vec::new(),
            narrative,
            risk_level,
            submission_date: Utc::now(),
        })
    }

    pub fn add_transaction(mut self, transaction: GoAmlTransaction) -> Self {
        self.transactions.push(transaction);
        self
    }

    pub fn add_suspect(mut self, suspect: GoAmlSuspect) -> Self {
        self.suspects.push(suspect);
        self
    }

    pub fn validate(&self) -> Result<(), GoAmlError> {
        if self.transactions.is_empty() {
            return Err(GoAmlError::InvalidReport(
                "At least one transaction is required".to_string(),
            ));
        }
        if self.suspects.is_empty() {
            return Err(GoAmlError::InvalidReport(
                "At least one suspect is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Submission response from CTAF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionResponse {
    /// Submission ID assigned by CTAF
    pub submission_id: String,
    /// Status of submission
    pub status: String,
    /// Message from CTAF
    pub message: String,
    /// Timestamp of submission
    pub timestamp: DateTime<Utc>,
}

// ============================================================================
// goAML CLIENT
// ============================================================================

/// Client for interfacing with CTAF's goAML platform
pub struct GoAmlClient {
    base_url: String,
    api_key: String,
}

impl GoAmlClient {
    pub fn new(base_url: String, api_key: String) -> Result<Self, GoAmlError> {
        if base_url.trim().is_empty() {
            return Err(GoAmlError::ValidationError(
                "Base URL cannot be empty".to_string(),
            ));
        }
        if api_key.trim().is_empty() {
            return Err(GoAmlError::ValidationError(
                "API key cannot be empty".to_string(),
            ));
        }

        Ok(GoAmlClient { base_url, api_key })
    }

    /// Generate goAML-compliant XML for a report
    ///
    /// Creates XML in the UN goAML standard format suitable for CTAF submission
    pub fn generate_xml(&self, report: &GoAmlReport) -> Result<String, GoAmlError> {
        report.validate()?;

        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<goAML>\n");
        xml.push_str("  <report>\n");

        // Report header
        xml.push_str(&format!("    <reportId>{}</reportId>\n", report.report_id));
        xml.push_str(&format!("    <reportType>{}</reportType>\n", report.report_type));
        xml.push_str(&format!("    <riskLevel>{}</riskLevel>\n", report.risk_level.as_str()));
        xml.push_str(&format!(
            "    <submissionDate>{}</submissionDate>\n",
            report.submission_date.to_rfc3339()
        ));

        // Reporting entity
        xml.push_str("    <reportingEntity>\n");
        xml.push_str(&format!("      <name>{}</name>\n", escape_xml(&report.reporting_entity.name)));
        xml.push_str(&format!(
            "      <registrationNumber>{}</registrationNumber>\n",
            escape_xml(&report.reporting_entity.registration_number)
        ));
        xml.push_str(&format!(
            "      <country>{}</country>\n",
            report.reporting_entity.country
        ));
        xml.push_str(&format!(
            "      <contactPerson>{}</contactPerson>\n",
            escape_xml(&report.reporting_entity.contact_person)
        ));
        xml.push_str(&format!("      <phone>{}</phone>\n", report.reporting_entity.phone));
        xml.push_str(&format!("      <email>{}</email>\n", report.reporting_entity.email));
        xml.push_str("    </reportingEntity>\n");

        // Transactions
        xml.push_str("    <transactions>\n");
        for tx in &report.transactions {
            xml.push_str("      <transaction>\n");
            xml.push_str(&format!("        <transactionId>{}</transactionId>\n", escape_xml(&tx.transaction_id)));
            xml.push_str(&format!("        <date>{}</date>\n", tx.date.to_rfc3339()));
            xml.push_str(&format!("        <amount>{}</amount>\n", tx.amount));
            xml.push_str(&format!("        <currency>{}</currency>\n", tx.currency));
            xml.push_str(&format!("        <type>{}</type>\n", escape_xml(&tx.transaction_type)));
            xml.push_str(&format!("        <fromAccount>{}</fromAccount>\n", escape_xml(&tx.from_account)));
            xml.push_str(&format!("        <toAccount>{}</toAccount>\n", escape_xml(&tx.to_account)));
            xml.push_str("      </transaction>\n");
        }
        xml.push_str("    </transactions>\n");

        // Suspects
        xml.push_str("    <suspects>\n");
        for suspect in &report.suspects {
            xml.push_str("      <suspect>\n");
            xml.push_str(&format!("        <fullName>{}</fullName>\n", escape_xml(&suspect.full_name)));
            if let Some(dob) = &suspect.date_of_birth {
                xml.push_str(&format!("        <dateOfBirth>{}</dateOfBirth>\n", dob));
            }
            if let Some(nat) = &suspect.nationality {
                xml.push_str(&format!("        <nationality>{}</nationality>\n", nat));
            }
            if let Some(id) = &suspect.id_number {
                xml.push_str(&format!("        <idNumber>{}</idNumber>\n", escape_xml(id)));
            }
            if let Some(addr) = &suspect.address {
                xml.push_str(&format!("        <address>{}</address>\n", escape_xml(addr)));
            }
            xml.push_str("      </suspect>\n");
        }
        xml.push_str("    </suspects>\n");

        // Narrative
        xml.push_str(&format!("    <narrative>{}</narrative>\n", escape_xml(&report.narrative)));

        xml.push_str("  </report>\n");
        xml.push_str("</goAML>\n");

        Ok(xml)
    }

    /// Submit report to CTAF goAML platform
    ///
    /// In production, this would make an HTTP POST request to CTAF's endpoint.
    /// Currently returns a mock response for testing.
    pub async fn submit_report(&self, report: &GoAmlReport) -> Result<SubmissionResponse, GoAmlError> {
        // Validate report before submission
        report.validate()?;

        // Generate XML
        let xml = self.generate_xml(report)?;

        // In production, this would be:
        // POST to {self.base_url}/api/v1/reports
        // Headers: Authorization: Bearer {self.api_key}, Content-Type: application/xml
        // Body: xml
        //
        // For now, we return a successful mock response
        tracing::info!(
            report_id = %report.report_id,
            report_type = %report.report_type,
            "Submitting goAML report to CTAF"
        );

        Ok(SubmissionResponse {
            submission_id: Uuid::new_v4().to_string(),
            status: "ACCEPTED".to_string(),
            message: "Report successfully submitted to CTAF".to_string(),
            timestamp: Utc::now(),
        })
    }
}

// ============================================================================
// CONVERSIONS FROM DOMAIN TYPES
// ============================================================================

/// Convert from AML domain SuspicionReport to GoAmlReport
///
/// This adapter bridges the internal domain model to the goAML format for
/// external submission to CTAF.
///
/// # Arguments
///
/// * `narrative` - The narrative/reasons from the suspicion report
/// * `reporting_entity` - Institution filing the report
/// * `suspect_info` - Information about the suspect
pub fn from_suspicion_report(
    narrative: String,
    reporting_entity: ReportingEntity,
    suspect_info: GoAmlSuspect,
) -> Result<GoAmlReport, GoAmlError> {
    let mut report = GoAmlReport::new(
        GoAmlReportType::STR,
        reporting_entity,
        narrative,
        RiskLevel::High,
    )?;

    // Add suspect
    report = report.add_suspect(suspect_info);

    Ok(report)
}

// ============================================================================
// UTILITIES
// ============================================================================

/// Escape XML special characters to prevent injection
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporting_entity_creation() {
        let entity = ReportingEntity::new(
            "BANKO".to_string(),
            "REG-001".to_string(),
            "TN".to_string(),
            "Ali Mansouri".to_string(),
            "+216-123-456".to_string(),
            "aml@banko.tn".to_string(),
        )
        .unwrap();

        assert_eq!(entity.name, "BANKO");
        assert_eq!(entity.country, "TN");
    }

    #[test]
    fn test_reporting_entity_invalid_email() {
        let result = ReportingEntity::new(
            "BANKO".to_string(),
            "REG-001".to_string(),
            "TN".to_string(),
            "Ali Mansouri".to_string(),
            "+216-123-456".to_string(),
            "invalid-email".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_suspect_creation() {
        let suspect = GoAmlSuspect::new("Ahmed Ben Mohamed".to_string())
            .unwrap()
            .with_nationality("TN".to_string())
            .with_dob("1980-05-15".to_string());

        assert_eq!(suspect.full_name, "Ahmed Ben Mohamed");
        assert_eq!(suspect.nationality, Some("TN".to_string()));
    }

    #[test]
    fn test_transaction_creation() {
        let tx = GoAmlTransaction::new(
            "TXN-001".to_string(),
            Utc::now(),
            50000.0,
            "TND".to_string(),
            "Wire Transfer".to_string(),
            "ACC-123".to_string(),
            "ACC-456".to_string(),
        )
        .unwrap();

        assert_eq!(tx.amount, 50000.0);
        assert_eq!(tx.currency, "TND");
    }

    #[test]
    fn test_transaction_invalid_amount() {
        let result = GoAmlTransaction::new(
            "TXN-001".to_string(),
            Utc::now(),
            -100.0,
            "TND".to_string(),
            "Wire Transfer".to_string(),
            "ACC-123".to_string(),
            "ACC-456".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_goaml_report_creation() {
        let entity = ReportingEntity::new(
            "BANKO".to_string(),
            "REG-001".to_string(),
            "TN".to_string(),
            "Ali Mansouri".to_string(),
            "+216-123-456".to_string(),
            "aml@banko.tn".to_string(),
        )
        .unwrap();

        let report = GoAmlReport::new(
            GoAmlReportType::STR,
            entity,
            "Suspicious wire transfer detected".to_string(),
            RiskLevel::High,
        )
        .unwrap();

        assert_eq!(report.report_type, GoAmlReportType::STR);
    }

    #[test]
    fn test_goaml_report_validation_missing_transactions() {
        let entity = ReportingEntity::new(
            "BANKO".to_string(),
            "REG-001".to_string(),
            "TN".to_string(),
            "Ali Mansouri".to_string(),
            "+216-123-456".to_string(),
            "aml@banko.tn".to_string(),
        )
        .unwrap();

        let report = GoAmlReport::new(
            GoAmlReportType::STR,
            entity,
            "Suspicious activity".to_string(),
            RiskLevel::High,
        )
        .unwrap();

        assert!(report.validate().is_err());
    }

    #[test]
    fn test_xml_generation() {
        let entity = ReportingEntity::new(
            "BANKO".to_string(),
            "REG-001".to_string(),
            "TN".to_string(),
            "Ali Mansouri".to_string(),
            "+216-123-456".to_string(),
            "aml@banko.tn".to_string(),
        )
        .unwrap();

        let tx = GoAmlTransaction::new(
            "TXN-001".to_string(),
            Utc::now(),
            50000.0,
            "TND".to_string(),
            "Wire Transfer".to_string(),
            "ACC-123".to_string(),
            "ACC-456".to_string(),
        )
        .unwrap();

        let suspect = GoAmlSuspect::new("Ahmed Ben Mohamed".to_string()).unwrap();

        let report = GoAmlReport::new(
            GoAmlReportType::STR,
            entity,
            "Suspicious wire transfer".to_string(),
            RiskLevel::High,
        )
        .unwrap()
        .add_transaction(tx)
        .add_suspect(suspect);

        let client = GoAmlClient::new(
            "https://ctaf.gov.tn".to_string(),
            "test-api-key".to_string(),
        )
        .unwrap();

        let xml = client.generate_xml(&report).unwrap();
        assert!(xml.contains("<?xml version"));
        assert!(xml.contains("<goAML>"));
        assert!(xml.contains("TXN-001"));
        assert!(xml.contains("Ahmed Ben Mohamed"));
    }

    #[test]
    fn test_xml_escaping() {
        assert_eq!(escape_xml("test & co"), "test &amp; co");
        assert_eq!(escape_xml("<script>"), "&lt;script&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }
}
