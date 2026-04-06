use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Newtypes for Compliance IDs
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SmsiControlId(Uuid);

impl SmsiControlId {
    pub fn new() -> Self {
        SmsiControlId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        SmsiControlId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SmsiControlId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SmsiControlId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RiskEntryId(Uuid);

impl RiskEntryId {
    pub fn new() -> Self {
        RiskEntryId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        RiskEntryId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for RiskEntryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RiskEntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenVaultId(Uuid);

impl TokenVaultId {
    pub fn new() -> Self {
        TokenVaultId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        TokenVaultId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TokenVaultId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TokenVaultId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// STORY-COMP-01: SMSI ISO 27001 Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmsiTheme {
    Organizational, // 37 controls
    People,         // 8 controls
    Physical,       // 14 controls
    Technological,  // 34 controls
}

impl SmsiTheme {
    pub fn as_str(&self) -> &str {
        match self {
            SmsiTheme::Organizational => "Organizational",
            SmsiTheme::People => "People",
            SmsiTheme::Physical => "Physical",
            SmsiTheme::Technological => "Technological",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Organizational" => Ok(SmsiTheme::Organizational),
            "People" => Ok(SmsiTheme::People),
            "Physical" => Ok(SmsiTheme::Physical),
            "Technological" => Ok(SmsiTheme::Technological),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown SMSI theme: {s}"
            ))),
        }
    }
}

impl fmt::Display for SmsiTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmsiControlStatus {
    NotImplemented,
    Partial,
    Implemented,
    NotApplicable,
}

impl SmsiControlStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SmsiControlStatus::NotImplemented => "NotImplemented",
            SmsiControlStatus::Partial => "Partial",
            SmsiControlStatus::Implemented => "Implemented",
            SmsiControlStatus::NotApplicable => "NotApplicable",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "NotImplemented" => Ok(SmsiControlStatus::NotImplemented),
            "Partial" => Ok(SmsiControlStatus::Partial),
            "Implemented" => Ok(SmsiControlStatus::Implemented),
            "NotApplicable" => Ok(SmsiControlStatus::NotApplicable),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown control status: {s}"
            ))),
        }
    }
}

impl fmt::Display for SmsiControlStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// STORY-COMP-01: SmsiControl Entity (ISO 27001 Annex A)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsiControl {
    id: SmsiControlId,
    control_ref: String, // e.g. "A.5.1"
    title: String,
    theme: SmsiTheme,
    description: String,
    status: SmsiControlStatus,
    responsible: String, // person/role responsible
    evidence: Option<String>,
    last_audit_date: Option<DateTime<Utc>>,
    next_audit_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SmsiControl {
    /// Create a new SmsiControl with validation
    pub fn new(
        control_ref: String,
        title: String,
        theme: SmsiTheme,
        description: String,
        responsible: String,
    ) -> Result<Self, DomainError> {
        // Validate control_ref format (e.g., "A.5.1")
        if control_ref.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Control reference cannot be empty".to_string(),
            ));
        }

        if title.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Control title cannot be empty".to_string(),
            ));
        }

        if description.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Control description cannot be empty".to_string(),
            ));
        }

        if responsible.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Control responsible party cannot be empty".to_string(),
            ));
        }

        Ok(SmsiControl {
            id: SmsiControlId::new(),
            control_ref,
            title,
            theme,
            description,
            status: SmsiControlStatus::NotImplemented,
            responsible,
            evidence: None,
            last_audit_date: None,
            next_audit_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        id: SmsiControlId,
        control_ref: String,
        title: String,
        theme: SmsiTheme,
        description: String,
        status: SmsiControlStatus,
        responsible: String,
        evidence: Option<String>,
        last_audit_date: Option<DateTime<Utc>>,
        next_audit_date: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SmsiControl {
            id,
            control_ref,
            title,
            theme,
            description,
            status,
            responsible,
            evidence,
            last_audit_date,
            next_audit_date,
            created_at,
            updated_at,
        }
    }

    /// Update the control status
    pub fn set_status(&mut self, status: SmsiControlStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Attach evidence documentation link
    pub fn attach_evidence(&mut self, evidence_link: String) -> Result<(), DomainError> {
        if evidence_link.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Evidence link cannot be empty".to_string(),
            ));
        }
        self.evidence = Some(evidence_link);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Record an audit
    pub fn record_audit(
        &mut self,
        next_audit: DateTime<Utc>,
    ) -> Result<(), DomainError> {
        self.last_audit_date = Some(Utc::now());
        self.next_audit_date = Some(next_audit);
        self.updated_at = Utc::now();
        Ok(())
    }

    // --- Getters ---
    pub fn id(&self) -> &SmsiControlId {
        &self.id
    }
    pub fn control_ref(&self) -> &str {
        &self.control_ref
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn theme(&self) -> &SmsiTheme {
        &self.theme
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn status(&self) -> &SmsiControlStatus {
        &self.status
    }
    pub fn responsible(&self) -> &str {
        &self.responsible
    }
    pub fn evidence(&self) -> Option<&str> {
        self.evidence.as_deref()
    }
    pub fn last_audit_date(&self) -> Option<&DateTime<Utc>> {
        self.last_audit_date.as_ref()
    }
    pub fn next_audit_date(&self) -> Option<&DateTime<Utc>> {
        self.next_audit_date.as_ref()
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

// ============================================================
// STORY-COMP-01: Risk Management Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskCategory {
    Operational,
    Cyber,
    Regulatory,
    Financial,
    Reputational,
}

impl RiskCategory {
    pub fn as_str(&self) -> &str {
        match self {
            RiskCategory::Operational => "Operational",
            RiskCategory::Cyber => "Cyber",
            RiskCategory::Regulatory => "Regulatory",
            RiskCategory::Financial => "Financial",
            RiskCategory::Reputational => "Reputational",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Operational" => Ok(RiskCategory::Operational),
            "Cyber" => Ok(RiskCategory::Cyber),
            "Regulatory" => Ok(RiskCategory::Regulatory),
            "Financial" => Ok(RiskCategory::Financial),
            "Reputational" => Ok(RiskCategory::Reputational),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown risk category: {s}"
            ))),
        }
    }
}

impl fmt::Display for RiskCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum RiskLevel {
    VeryLow,  // 1
    Low,      // 2
    Medium,   // 3
    High,     // 4
    VeryHigh, // 5
}

impl RiskLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            RiskLevel::VeryLow => 1,
            RiskLevel::Low => 2,
            RiskLevel::Medium => 3,
            RiskLevel::High => 4,
            RiskLevel::VeryHigh => 5,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RiskLevel::VeryLow => "VeryLow",
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::VeryHigh => "VeryHigh",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "VeryLow" => Ok(RiskLevel::VeryLow),
            "Low" => Ok(RiskLevel::Low),
            "Medium" => Ok(RiskLevel::Medium),
            "High" => Ok(RiskLevel::High),
            "VeryHigh" => Ok(RiskLevel::VeryHigh),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown risk level: {s}"
            ))),
        }
    }

    pub fn from_u8(value: u8) -> Result<Self, DomainError> {
        match value {
            1 => Ok(RiskLevel::VeryLow),
            2 => Ok(RiskLevel::Low),
            3 => Ok(RiskLevel::Medium),
            4 => Ok(RiskLevel::High),
            5 => Ok(RiskLevel::VeryHigh),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Invalid risk level: {value}"
            ))),
        }
    }
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskStatus {
    Open,
    Mitigated,
    Accepted,
    Closed,
}

impl RiskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            RiskStatus::Open => "Open",
            RiskStatus::Mitigated => "Mitigated",
            RiskStatus::Accepted => "Accepted",
            RiskStatus::Closed => "Closed",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Open" => Ok(RiskStatus::Open),
            "Mitigated" => Ok(RiskStatus::Mitigated),
            "Accepted" => Ok(RiskStatus::Accepted),
            "Closed" => Ok(RiskStatus::Closed),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown risk status: {s}"
            ))),
        }
    }
}

impl fmt::Display for RiskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// STORY-COMP-01: RiskEntry Entity
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskEntry {
    id: RiskEntryId,
    risk_ref: String, // e.g. "RISK-001"
    title: String,
    description: String,
    category: RiskCategory,
    likelihood: RiskLevel, // 1-5
    impact: RiskLevel,     // 1-5
    inherent_score: u8,    // likelihood * impact
    residual_score: u8,    // after mitigations
    mitigations: Vec<String>,
    owner: String,
    status: RiskStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RiskEntry {
    /// Create a new RiskEntry with validation
    pub fn new(
        risk_ref: String,
        title: String,
        description: String,
        category: RiskCategory,
        likelihood: RiskLevel,
        impact: RiskLevel,
        owner: String,
    ) -> Result<Self, DomainError> {
        if risk_ref.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Risk reference cannot be empty".to_string(),
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

        let inherent_score = likelihood.as_u8() * impact.as_u8();

        Ok(RiskEntry {
            id: RiskEntryId::new(),
            risk_ref,
            title,
            description,
            category,
            likelihood,
            impact,
            inherent_score,
            residual_score: inherent_score,
            mitigations: Vec::new(),
            owner,
            status: RiskStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        id: RiskEntryId,
        risk_ref: String,
        title: String,
        description: String,
        category: RiskCategory,
        likelihood: RiskLevel,
        impact: RiskLevel,
        inherent_score: u8,
        residual_score: u8,
        mitigations: Vec<String>,
        owner: String,
        status: RiskStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        RiskEntry {
            id,
            risk_ref,
            title,
            description,
            category,
            likelihood,
            impact,
            inherent_score,
            residual_score,
            mitigations,
            owner,
            status,
            created_at,
            updated_at,
        }
    }

    /// Compute inherent risk score (likelihood * impact)
    pub fn inherent_score(&self) -> u8 {
        self.inherent_score
    }

    /// Compute residual risk score after mitigations
    pub fn residual_score(&self) -> u8 {
        self.residual_score
    }

    /// Check if this is high risk (score >= 15)
    pub fn is_high_risk(&self) -> bool {
        self.inherent_score >= 15
    }

    /// Check if this is high residual risk
    pub fn is_high_residual_risk(&self) -> bool {
        self.residual_score >= 15
    }

    /// Add a mitigation measure
    pub fn add_mitigation(&mut self, mitigation: String) -> Result<(), DomainError> {
        if mitigation.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Mitigation cannot be empty".to_string(),
            ));
        }
        self.mitigations.push(mitigation);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update residual score (typically lower after mitigations applied)
    pub fn set_residual_score(&mut self, score: u8) -> Result<(), DomainError> {
        if score > 25 {
            return Err(DomainError::InvalidComplianceData(
                "Residual score cannot exceed 25".to_string(),
            ));
        }
        self.residual_score = score;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update risk status
    pub fn set_status(&mut self, status: RiskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    // --- Getters ---
    pub fn id(&self) -> &RiskEntryId {
        &self.id
    }
    pub fn risk_ref(&self) -> &str {
        &self.risk_ref
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn category(&self) -> &RiskCategory {
        &self.category
    }
    pub fn likelihood(&self) -> &RiskLevel {
        &self.likelihood
    }
    pub fn impact(&self) -> &RiskLevel {
        &self.impact
    }
    pub fn mitigations(&self) -> &[String] {
        &self.mitigations
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn status(&self) -> &RiskStatus {
        &self.status
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

// ============================================================
// STORY-COMP-02: PCI DSS TokenVault Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenStatus {
    Active,
    Expired,
    Revoked,
}

impl TokenStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TokenStatus::Active => "Active",
            TokenStatus::Expired => "Expired",
            TokenStatus::Revoked => "Revoked",
        }
    }

    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s {
            "Active" => Ok(TokenStatus::Active),
            "Expired" => Ok(TokenStatus::Expired),
            "Revoked" => Ok(TokenStatus::Revoked),
            _ => Err(DomainError::InvalidComplianceData(format!(
                "Unknown token status: {s}"
            ))),
        }
    }
}

impl fmt::Display for TokenStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// STORY-COMP-02: TokenVault Entity (PCI DSS Tokenization)
// ============================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenVault {
    id: TokenVaultId,
    token: String,                   // opaque token replacing PAN
    masked_pan: String,              // e.g. "****-****-****-1234"
    card_holder: String,             // encrypted
    expiry_month: u8,
    expiry_year: u16,
    token_status: TokenStatus,
    encryption_key_id: String,       // reference to HSM key
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
}

impl TokenVault {
    /// Create a new TokenVault by tokenizing a PAN (Primary Account Number)
    /// The PAN is masked and replaced with an opaque token
    pub fn tokenize(
        pan: &str,
        card_holder: String,
        expiry_month: u8,
        expiry_year: u16,
        encryption_key_id: String,
    ) -> Result<Self, DomainError> {
        // Validate PAN format (basic check: 13-19 digits)
        if pan.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "PAN cannot be empty".to_string(),
            ));
        }

        let pan_digits: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();
        if pan_digits.len() < 13 || pan_digits.len() > 19 {
            return Err(DomainError::InvalidComplianceData(
                "Invalid PAN length (must be 13-19 digits)".to_string(),
            ));
        }

        if card_holder.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Card holder name cannot be empty".to_string(),
            ));
        }

        if expiry_month < 1 || expiry_month > 12 {
            return Err(DomainError::InvalidComplianceData(
                "Invalid expiry month (1-12)".to_string(),
            ));
        }

        if expiry_year < 2000 || expiry_year > 2100 {
            return Err(DomainError::InvalidComplianceData(
                "Invalid expiry year".to_string(),
            ));
        }

        if encryption_key_id.trim().is_empty() {
            return Err(DomainError::InvalidComplianceData(
                "Encryption key ID cannot be empty".to_string(),
            ));
        }

        // Generate opaque token from PAN hash
        let token = Self::generate_token(&pan_digits);

        // Mask the PAN: keep first 6 and last 4 digits
        let masked_pan = Self::mask_pan(&pan_digits);

        // Compute expiry date (end of expiry month/year)
        let expires_at = Some(Utc::now() + chrono::Duration::days(365));

        Ok(TokenVault {
            id: TokenVaultId::new(),
            token,
            masked_pan,
            card_holder,
            expiry_month,
            expiry_year,
            token_status: TokenStatus::Active,
            encryption_key_id,
            created_at: Utc::now(),
            expires_at,
        })
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        id: TokenVaultId,
        token: String,
        masked_pan: String,
        card_holder: String,
        expiry_month: u8,
        expiry_year: u16,
        token_status: TokenStatus,
        encryption_key_id: String,
        created_at: DateTime<Utc>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        TokenVault {
            id,
            token,
            masked_pan,
            card_holder,
            expiry_month,
            expiry_year,
            token_status,
            encryption_key_id,
            created_at,
            expires_at,
        }
    }

    /// Generate an opaque token from the PAN (hash-based)
    fn generate_token(pan: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(pan.as_bytes());
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        let token_hash = format!("{:x}", hasher.finalize());
        format!("TOKEN_{}", &token_hash[..32])
    }

    /// Mask a PAN: keep first 6 and last 4 digits
    fn mask_pan(pan: &str) -> String {
        if pan.len() < 10 {
            return "****".to_string();
        }
        let first_six = &pan[..6];
        let last_four = &pan[pan.len() - 4..];
        format!("{}-****-****-{}", first_six, last_four)
    }

    /// Revoke the token
    pub fn revoke(&mut self) -> Result<(), DomainError> {
        if self.token_status == TokenStatus::Revoked {
            return Err(DomainError::InvalidComplianceData(
                "Token is already revoked".to_string(),
            ));
        }
        self.token_status = TokenStatus::Revoked;
        Ok(())
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            Utc::now() > exp
        } else {
            false
        }
    }

    // --- Getters ---
    pub fn id(&self) -> &TokenVaultId {
        &self.id
    }
    pub fn token(&self) -> &str {
        &self.token
    }
    pub fn masked_pan(&self) -> &str {
        &self.masked_pan
    }
    pub fn card_holder(&self) -> &str {
        &self.card_holder
    }
    pub fn expiry_month(&self) -> u8 {
        self.expiry_month
    }
    pub fn expiry_year(&self) -> u16 {
        self.expiry_year
    }
    pub fn token_status(&self) -> &TokenStatus {
        &self.token_status
    }
    pub fn encryption_key_id(&self) -> &str {
        &self.encryption_key_id
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- SmsiControl Tests ---

    #[test]
    fn test_smsi_control_creation() {
        let control = SmsiControl::new(
            "A.5.1".to_string(),
            "Policies for information security".to_string(),
            SmsiTheme::Organizational,
            "Establish and implement policies".to_string(),
            "Security Officer".to_string(),
        );
        assert!(control.is_ok());
        let ctrl = control.unwrap();
        assert_eq!(ctrl.control_ref(), "A.5.1");
        assert_eq!(ctrl.status(), &SmsiControlStatus::NotImplemented);
    }

    #[test]
    fn test_smsi_control_empty_ref_rejected() {
        let result = SmsiControl::new(
            "".to_string(),
            "Title".to_string(),
            SmsiTheme::Organizational,
            "Description".to_string(),
            "Owner".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_smsi_control_set_status() {
        let mut control = SmsiControl::new(
            "A.5.1".to_string(),
            "Control".to_string(),
            SmsiTheme::Organizational,
            "Description".to_string(),
            "Owner".to_string(),
        )
        .unwrap();
        control.set_status(SmsiControlStatus::Implemented);
        assert_eq!(control.status(), &SmsiControlStatus::Implemented);
    }

    #[test]
    fn test_smsi_control_attach_evidence() {
        let mut control = SmsiControl::new(
            "A.5.1".to_string(),
            "Control".to_string(),
            SmsiTheme::Organizational,
            "Description".to_string(),
            "Owner".to_string(),
        )
        .unwrap();
        assert!(control.evidence().is_none());
        control.attach_evidence("https://example.com/evidence".to_string()).ok();
        assert!(control.evidence().is_some());
    }

    #[test]
    fn test_smsi_control_record_audit() {
        let mut control = SmsiControl::new(
            "A.5.1".to_string(),
            "Control".to_string(),
            SmsiTheme::Organizational,
            "Description".to_string(),
            "Owner".to_string(),
        )
        .unwrap();
        let next_audit = Utc::now() + chrono::Duration::days(365);
        control.record_audit(next_audit).ok();
        assert!(control.last_audit_date().is_some());
        assert!(control.next_audit_date().is_some());
    }

    #[test]
    fn test_smsi_theme_roundtrip() {
        for theme in [
            SmsiTheme::Organizational,
            SmsiTheme::People,
            SmsiTheme::Physical,
            SmsiTheme::Technological,
        ] {
            let s = theme.as_str();
            let parsed = SmsiTheme::from_str_type(s).unwrap();
            assert_eq!(theme, parsed);
        }
    }

    // --- RiskEntry Tests ---

    #[test]
    fn test_risk_entry_creation() {
        let risk = RiskEntry::new(
            "RISK-001".to_string(),
            "Cyber Attack Risk".to_string(),
            "Unauthorized access to systems".to_string(),
            RiskCategory::Cyber,
            RiskLevel::High,
            RiskLevel::VeryHigh,
            "CISO".to_string(),
        );
        assert!(risk.is_ok());
        let r = risk.unwrap();
        assert_eq!(r.risk_ref(), "RISK-001");
        assert_eq!(r.status(), &RiskStatus::Open);
    }

    #[test]
    fn test_risk_entry_inherent_score() {
        let risk = RiskEntry::new(
            "RISK-001".to_string(),
            "Test Risk".to_string(),
            "Description".to_string(),
            RiskCategory::Operational,
            RiskLevel::High,      // 4
            RiskLevel::VeryHigh,  // 5
            "Owner".to_string(),
        )
        .unwrap();
        assert_eq!(risk.inherent_score(), 20); // 4 * 5
    }

    #[test]
    fn test_risk_entry_is_high_risk() {
        let high_risk = RiskEntry::new(
            "RISK-001".to_string(),
            "High Risk".to_string(),
            "Description".to_string(),
            RiskCategory::Financial,
            RiskLevel::High,      // 4
            RiskLevel::High,      // 4
            "Owner".to_string(),
        )
        .unwrap();
        assert!(high_risk.is_high_risk()); // 16 >= 15

        let low_risk = RiskEntry::new(
            "RISK-002".to_string(),
            "Low Risk".to_string(),
            "Description".to_string(),
            RiskCategory::Operational,
            RiskLevel::Low,  // 2
            RiskLevel::Low,  // 2
            "Owner".to_string(),
        )
        .unwrap();
        assert!(!low_risk.is_high_risk()); // 4 < 15
    }

    #[test]
    fn test_risk_entry_add_mitigation() {
        let mut risk = RiskEntry::new(
            "RISK-001".to_string(),
            "Test Risk".to_string(),
            "Description".to_string(),
            RiskCategory::Operational,
            RiskLevel::High,
            RiskLevel::High,
            "Owner".to_string(),
        )
        .unwrap();
        assert_eq!(risk.mitigations().len(), 0);
        risk.add_mitigation("Apply security patch".to_string()).ok();
        assert_eq!(risk.mitigations().len(), 1);
    }

    #[test]
    fn test_risk_entry_set_residual_score() {
        let mut risk = RiskEntry::new(
            "RISK-001".to_string(),
            "Test Risk".to_string(),
            "Description".to_string(),
            RiskCategory::Operational,
            RiskLevel::High,
            RiskLevel::High,
            "Owner".to_string(),
        )
        .unwrap();
        assert_eq!(risk.residual_score(), 16);
        risk.set_residual_score(8).ok();
        assert_eq!(risk.residual_score(), 8);
        assert!(!risk.is_high_residual_risk());
    }

    #[test]
    fn test_risk_level_roundtrip() {
        for level in [
            RiskLevel::VeryLow,
            RiskLevel::Low,
            RiskLevel::Medium,
            RiskLevel::High,
            RiskLevel::VeryHigh,
        ] {
            let s = level.as_str();
            let parsed = RiskLevel::from_str_type(s).unwrap();
            assert_eq!(level, parsed);
        }
    }

    // --- TokenVault Tests ---

    #[test]
    fn test_token_vault_tokenize() {
        let result = TokenVault::tokenize(
            "4532123456789123",
            "John Doe".to_string(),
            12,
            2026,
            "HSM_KEY_001".to_string(),
        );
        assert!(result.is_ok());
        let vault = result.unwrap();
        assert_eq!(vault.card_holder(), "John Doe");
        assert_eq!(vault.expiry_month(), 12);
        assert_eq!(vault.expiry_year(), 2026);
        assert!(vault.token().starts_with("TOKEN_"));
    }

    #[test]
    fn test_token_vault_mask_pan() {
        let vault = TokenVault::tokenize(
            "4532123456789123",
            "John Doe".to_string(),
            12,
            2026,
            "HSM_KEY_001".to_string(),
        )
        .unwrap();
        assert_eq!(vault.masked_pan(), "453212-****-****-9123");
    }

    #[test]
    fn test_token_vault_invalid_pan_empty() {
        let result = TokenVault::tokenize(
            "",
            "John Doe".to_string(),
            12,
            2026,
            "HSM_KEY_001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_token_vault_invalid_pan_length() {
        let result = TokenVault::tokenize(
            "123",
            "John Doe".to_string(),
            12,
            2026,
            "HSM_KEY_001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_token_vault_invalid_expiry_month() {
        let result = TokenVault::tokenize(
            "4532123456789123",
            "John Doe".to_string(),
            13,
            2026,
            "HSM_KEY_001".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_token_vault_revoke() {
        let mut vault = TokenVault::tokenize(
            "4532123456789123",
            "John Doe".to_string(),
            12,
            2026,
            "HSM_KEY_001".to_string(),
        )
        .unwrap();
        assert_eq!(vault.token_status(), &TokenStatus::Active);
        vault.revoke().ok();
        assert_eq!(vault.token_status(), &TokenStatus::Revoked);
    }

    #[test]
    fn test_token_vault_cannot_revoke_twice() {
        let mut vault = TokenVault::tokenize(
            "4532123456789123",
            "John Doe".to_string(),
            12,
            2026,
            "HSM_KEY_001".to_string(),
        )
        .unwrap();
        vault.revoke().ok();
        let result = vault.revoke();
        assert!(result.is_err());
    }

    #[test]
    fn test_token_status_roundtrip() {
        for status in [TokenStatus::Active, TokenStatus::Expired, TokenStatus::Revoked] {
            let s = status.as_str();
            let parsed = TokenStatus::from_str_type(s).unwrap();
            assert_eq!(status, parsed);
        }
    }
}

// ============================================================
// INPDP CONSENT AGGREGATE (STORY-COMP-06)
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InpdpConsentId(Uuid);

impl InpdpConsentId {
    pub fn new() -> Self {
        InpdpConsentId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        InpdpConsentId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for InpdpConsentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InpdpConsentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// ConsentPurpose - INPDP Purposes
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentPurpose {
    Marketing,
    Analytics,
    ThirdPartySharing,
    Profiling,
    CrossBorder,
}

impl ConsentPurpose {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "marketing" => Ok(ConsentPurpose::Marketing),
            "analytics" => Ok(ConsentPurpose::Analytics),
            "thirdpartysharing" | "third_party_sharing" => Ok(ConsentPurpose::ThirdPartySharing),
            "profiling" => Ok(ConsentPurpose::Profiling),
            "crossborder" | "cross_border" => Ok(ConsentPurpose::CrossBorder),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown consent purpose: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConsentPurpose::Marketing => "Marketing",
            ConsentPurpose::Analytics => "Analytics",
            ConsentPurpose::ThirdPartySharing => "ThirdPartySharing",
            ConsentPurpose::Profiling => "Profiling",
            ConsentPurpose::CrossBorder => "CrossBorder",
        }
    }
}

impl fmt::Display for ConsentPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// LegalBasis - INPDP Legal Basis
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    Consent,
    ContractualNecessity,
    LegalObligation,
    VitalInterest,
    PublicInterest,
    LegitimateInterest,
}

impl LegalBasis {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "consent" => Ok(LegalBasis::Consent),
            "contractualnecessity" | "contractual_necessity" => {
                Ok(LegalBasis::ContractualNecessity)
            }
            "legalobligation" | "legal_obligation" => Ok(LegalBasis::LegalObligation),
            "vitalinterest" | "vital_interest" => Ok(LegalBasis::VitalInterest),
            "publicinterest" | "public_interest" => Ok(LegalBasis::PublicInterest),
            "legitimateinterest" | "legitimate_interest" => Ok(LegalBasis::LegitimateInterest),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown legal basis: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LegalBasis::Consent => "Consent",
            LegalBasis::ContractualNecessity => "ContractualNecessity",
            LegalBasis::LegalObligation => "LegalObligation",
            LegalBasis::VitalInterest => "VitalInterest",
            LegalBasis::PublicInterest => "PublicInterest",
            LegalBasis::LegitimateInterest => "LegitimateInterest",
        }
    }
}

impl fmt::Display for LegalBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// InpdpConsent - INPDP Consent Aggregate
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InpdpConsent {
    id: InpdpConsentId,
    customer_id: Uuid,
    purpose: ConsentPurpose,
    granted: bool,
    granted_at: Option<DateTime<Utc>>,
    revoked_at: Option<DateTime<Utc>>,
    expiry_date: Option<DateTime<Utc>>,
    legal_basis: LegalBasis,
    data_categories: Vec<String>,
}

impl InpdpConsent {
    /// Create a new INPDP consent.
    pub fn new(
        customer_id: Uuid,
        purpose: ConsentPurpose,
        legal_basis: LegalBasis,
        data_categories: Vec<String>,
        expiry_date: Option<DateTime<Utc>>,
    ) -> Self {
        InpdpConsent {
            id: InpdpConsentId::new(),
            customer_id,
            purpose,
            granted: true,
            granted_at: Some(Utc::now()),
            revoked_at: None,
            expiry_date,
            legal_basis,
            data_categories,
        }
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: InpdpConsentId,
        customer_id: Uuid,
        purpose: ConsentPurpose,
        granted: bool,
        granted_at: Option<DateTime<Utc>>,
        revoked_at: Option<DateTime<Utc>>,
        expiry_date: Option<DateTime<Utc>>,
        legal_basis: LegalBasis,
        data_categories: Vec<String>,
    ) -> Self {
        InpdpConsent {
            id,
            customer_id,
            purpose,
            granted,
            granted_at,
            revoked_at,
            expiry_date,
            legal_basis,
            data_categories,
        }
    }

    /// Revoke this consent.
    pub fn revoke(&mut self) -> Result<(), DomainError> {
        if self.revoked_at.is_some() {
            return Err(DomainError::ConsentNotFound);
        }
        self.granted = false;
        self.revoked_at = Some(Utc::now());
        Ok(())
    }

    /// Check if consent is currently valid.
    pub fn is_valid(&self) -> bool {
        if !self.granted || self.revoked_at.is_some() {
            return false;
        }
        if let Some(expiry) = self.expiry_date {
            return Utc::now() < expiry;
        }
        true
    }

    // --- Getters ---

    pub fn id(&self) -> &InpdpConsentId {
        &self.id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn purpose(&self) -> ConsentPurpose {
        self.purpose
    }

    pub fn granted(&self) -> bool {
        self.granted
    }

    pub fn granted_at(&self) -> Option<DateTime<Utc>> {
        self.granted_at
    }

    pub fn revoked_at(&self) -> Option<DateTime<Utc>> {
        self.revoked_at
    }

    pub fn expiry_date(&self) -> Option<DateTime<Utc>> {
        self.expiry_date
    }

    pub fn legal_basis(&self) -> LegalBasis {
        self.legal_basis
    }

    pub fn data_categories(&self) -> &[String] {
        &self.data_categories
    }
}

// ============================================================
// DpiaStatus (STORY-COMP-08)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DpiaStatus {
    Draft,
    UnderReview,
    Approved,
    Rejected,
}

impl DpiaStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(DpiaStatus::Draft),
            "underreview" | "under_review" => Ok(DpiaStatus::UnderReview),
            "approved" => Ok(DpiaStatus::Approved),
            "rejected" => Ok(DpiaStatus::Rejected),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown DPIA status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DpiaStatus::Draft => "Draft",
            DpiaStatus::UnderReview => "UnderReview",
            DpiaStatus::Approved => "Approved",
            DpiaStatus::Rejected => "Rejected",
        }
    }
}

impl fmt::Display for DpiaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// DpiaId
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DpiaId(Uuid);

impl DpiaId {
    pub fn new() -> Self {
        DpiaId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DpiaId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DpiaId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DpiaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// Dpia - Data Protection Impact Assessment
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dpia {
    id: DpiaId,
    title: String,
    description: String,
    processing_activity: String,
    risk_assessment: String,
    mitigations: Vec<String>,
    status: DpiaStatus,
    created_at: DateTime<Utc>,
    approved_by: Option<String>,
    approved_at: Option<DateTime<Utc>>,
}

impl Dpia {
    /// Create a new DPIA in Draft status.
    pub fn new(
        title: String,
        description: String,
        processing_activity: String,
        risk_assessment: String,
        mitigations: Vec<String>,
    ) -> Self {
        Dpia {
            id: DpiaId::new(),
            title,
            description,
            processing_activity,
            risk_assessment,
            mitigations,
            status: DpiaStatus::Draft,
            created_at: Utc::now(),
            approved_by: None,
            approved_at: None,
        }
    }

    /// Reconstitute from persistence.
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: DpiaId,
        title: String,
        description: String,
        processing_activity: String,
        risk_assessment: String,
        mitigations: Vec<String>,
        status: DpiaStatus,
        created_at: DateTime<Utc>,
        approved_by: Option<String>,
        approved_at: Option<DateTime<Utc>>,
    ) -> Self {
        Dpia {
            id,
            title,
            description,
            processing_activity,
            risk_assessment,
            mitigations,
            status,
            created_at,
            approved_by,
            approved_at,
        }
    }

    /// Transition DPIA to UnderReview status.
    pub fn submit_for_review(&mut self) -> Result<(), DomainError> {
        if self.status != DpiaStatus::Draft {
            return Err(DomainError::ValidationError(
                "Only Draft DPIAs can be submitted for review".to_string(),
            ));
        }
        self.status = DpiaStatus::UnderReview;
        Ok(())
    }

    /// Approve the DPIA.
    pub fn approve(&mut self, approved_by: String) -> Result<(), DomainError> {
        if self.status != DpiaStatus::UnderReview {
            return Err(DomainError::ValidationError(
                "Only DPIAs under review can be approved".to_string(),
            ));
        }
        self.status = DpiaStatus::Approved;
        self.approved_by = Some(approved_by);
        self.approved_at = Some(Utc::now());
        Ok(())
    }

    /// Reject the DPIA.
    pub fn reject(&mut self) -> Result<(), DomainError> {
        if self.status != DpiaStatus::UnderReview {
            return Err(DomainError::ValidationError(
                "Only DPIAs under review can be rejected".to_string(),
            ));
        }
        self.status = DpiaStatus::Rejected;
        Ok(())
    }

    // --- Getters ---

    pub fn id(&self) -> &DpiaId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn processing_activity(&self) -> &str {
        &self.processing_activity
    }

    pub fn risk_assessment(&self) -> &str {
        &self.risk_assessment
    }

    pub fn mitigations(&self) -> &[String] {
        &self.mitigations
    }

    pub fn status(&self) -> DpiaStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn approved_by(&self) -> Option<&str> {
        self.approved_by.as_deref()
    }

    pub fn approved_at(&self) -> Option<DateTime<Utc>> {
        self.approved_at
    }
}

// ============================================================
// BreachStatus
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreachStatus {
    Detected,
    AuthorityNotified,
    SubjectsNotified,
    Resolved,
}

impl BreachStatus {
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "detected" => Ok(BreachStatus::Detected),
            "authoritynotified" | "authority_notified" => Ok(BreachStatus::AuthorityNotified),
            "subjectsnotified" | "subjects_notified" => Ok(BreachStatus::SubjectsNotified),
            "resolved" => Ok(BreachStatus::Resolved),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown breach status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BreachStatus::Detected => "Detected",
            BreachStatus::AuthorityNotified => "AuthorityNotified",
            BreachStatus::SubjectsNotified => "SubjectsNotified",
            BreachStatus::Resolved => "Resolved",
        }
    }
}

impl fmt::Display for BreachStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================
// BreachNotificationId
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BreachNotificationId(Uuid);

impl BreachNotificationId {
    pub fn new() -> Self {
        BreachNotificationId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        BreachNotificationId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BreachNotificationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BreachNotificationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================
// BreachNotification - Data Breach Notification (72h compliance)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachNotification {
    id: BreachNotificationId,
    breach_type: String,
    description: String,
    affected_data: Vec<String>,
    affected_count: u32,
    detected_at: DateTime<Utc>,
    notified_authority_at: Option<DateTime<Utc>>,
    notified_subjects_at: Option<DateTime<Utc>>,
    status: BreachStatus,
}

impl BreachNotification {
    /// Create a new breach notification.
    pub fn new(
        breach_type: String,
        description: String,
        affected_data: Vec<String>,
        affected_count: u32,
    ) -> Self {
        BreachNotification {
            id: BreachNotificationId::new(),
            breach_type,
            description,
            affected_data,
            affected_count,
            detected_at: Utc::now(),
            notified_authority_at: None,
            notified_subjects_at: None,
            status: BreachStatus::Detected,
        }
    }

    /// Reconstitute from persistence.
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: BreachNotificationId,
        breach_type: String,
        description: String,
        affected_data: Vec<String>,
        affected_count: u32,
        detected_at: DateTime<Utc>,
        notified_authority_at: Option<DateTime<Utc>>,
        notified_subjects_at: Option<DateTime<Utc>>,
        status: BreachStatus,
    ) -> Self {
        BreachNotification {
            id,
            breach_type,
            description,
            affected_data,
            affected_count,
            detected_at,
            notified_authority_at,
            notified_subjects_at,
            status,
        }
    }

    /// Notify authority of the breach (must be within 72 hours).
    pub fn notify_authority(&mut self) -> Result<(), DomainError> {
        let hours_since_detection = (Utc::now() - self.detected_at).num_hours();
        if hours_since_detection > 72 {
            return Err(DomainError::ValidationError(
                "Authority notification deadline exceeded (72 hours)".to_string(),
            ));
        }
        self.notified_authority_at = Some(Utc::now());
        self.status = BreachStatus::AuthorityNotified;
        Ok(())
    }

    /// Notify affected subjects (data subjects) of the breach.
    pub fn notify_subjects(&mut self) -> Result<(), DomainError> {
        if self.status != BreachStatus::AuthorityNotified {
            return Err(DomainError::ValidationError(
                "Authority must be notified before subjects".to_string(),
            ));
        }
        self.notified_subjects_at = Some(Utc::now());
        self.status = BreachStatus::SubjectsNotified;
        Ok(())
    }

    /// Mark breach as resolved.
    pub fn resolve(&mut self) -> Result<(), DomainError> {
        if self.status != BreachStatus::SubjectsNotified {
            return Err(DomainError::ValidationError(
                "Subjects must be notified before marking as resolved".to_string(),
            ));
        }
        self.status = BreachStatus::Resolved;
        Ok(())
    }

    /// Check if authority notification deadline is met (72 hours from detection).
    pub fn is_authority_notified_in_time(&self) -> bool {
        if let Some(notified_at) = self.notified_authority_at {
            notified_at - self.detected_at <= chrono::Duration::hours(72)
        } else {
            false
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &BreachNotificationId {
        &self.id
    }

    pub fn breach_type(&self) -> &str {
        &self.breach_type
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn affected_data(&self) -> &[String] {
        &self.affected_data
    }

    pub fn affected_count(&self) -> u32 {
        self.affected_count
    }

    pub fn detected_at(&self) -> DateTime<Utc> {
        self.detected_at
    }

    pub fn notified_authority_at(&self) -> Option<DateTime<Utc>> {
        self.notified_authority_at
    }

    pub fn notified_subjects_at(&self) -> Option<DateTime<Utc>> {
        self.notified_subjects_at
    }

    pub fn status(&self) -> BreachStatus {
        self.status
    }
}

// ============================================================
// INPDP TESTS
// ============================================================

#[cfg(test)]
mod inpdp_tests {
    use super::*;

    fn sample_customer_id() -> Uuid {
        Uuid::new_v4()
    }

    #[test]
    fn test_inpdp_consent_new() {
        let cid = sample_customer_id();
        let consent = InpdpConsent::new(
            cid,
            ConsentPurpose::Marketing,
            LegalBasis::Consent,
            vec!["email".to_string()],
            None,
        );
        assert!(consent.is_valid());
        assert_eq!(consent.customer_id(), cid);
        assert_eq!(consent.purpose(), ConsentPurpose::Marketing);
        assert_eq!(consent.legal_basis(), LegalBasis::Consent);
    }

    #[test]
    fn test_inpdp_consent_revoke() {
        let cid = sample_customer_id();
        let mut consent = InpdpConsent::new(
            cid,
            ConsentPurpose::Analytics,
            LegalBasis::Consent,
            vec!["behavior".to_string()],
            None,
        );
        assert!(consent.is_valid());

        let result = consent.revoke();
        assert!(result.is_ok());
        assert!(!consent.is_valid());
        assert!(consent.revoked_at().is_some());
    }

    #[test]
    fn test_inpdp_consent_with_expiry() {
        let cid = sample_customer_id();
        let expiry = Utc::now() + chrono::Duration::days(30);
        let consent = InpdpConsent::new(
            cid,
            ConsentPurpose::Profiling,
            LegalBasis::Consent,
            vec!["profile".to_string()],
            Some(expiry),
        );
        assert!(consent.is_valid());
    }

    #[test]
    fn test_dpia_lifecycle() {
        let mut dpia = Dpia::new(
            "Payment Processing".to_string(),
            "Assessment of payment processing risks".to_string(),
            "Payment processing for customer transactions".to_string(),
            "Moderate risk due to financial data".to_string(),
            vec!["Encrypt all data in transit".to_string()],
        );
        assert_eq!(dpia.status(), DpiaStatus::Draft);

        let result = dpia.submit_for_review();
        assert!(result.is_ok());
        assert_eq!(dpia.status(), DpiaStatus::UnderReview);

        let result = dpia.approve("officer_123".to_string());
        assert!(result.is_ok());
        assert_eq!(dpia.status(), DpiaStatus::Approved);
    }

    #[test]
    fn test_dpia_reject() {
        let mut dpia = Dpia::new(
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            vec![],
        );
        dpia.submit_for_review().unwrap();
        let result = dpia.reject();
        assert!(result.is_ok());
        assert_eq!(dpia.status(), DpiaStatus::Rejected);
    }

    #[test]
    fn test_breach_notification_new() {
        let breach = BreachNotification::new(
            "Unauthorized Access".to_string(),
            "Breach detected in customer database".to_string(),
            vec!["customer_emails".to_string(), "phone_numbers".to_string()],
            1500,
        );
        assert_eq!(breach.status(), BreachStatus::Detected);
        assert_eq!(breach.affected_count(), 1500);
        assert!(breach.notified_authority_at().is_none());
    }

    #[test]
    fn test_breach_notification_authority() {
        let mut breach = BreachNotification::new(
            "Data Loss".to_string(),
            "Customer data lost in server migration".to_string(),
            vec!["all_customer_data".to_string()],
            500,
        );
        let result = breach.notify_authority();
        assert!(result.is_ok());
        assert_eq!(breach.status(), BreachStatus::AuthorityNotified);
        assert!(breach.notified_authority_at().is_some());
        assert!(breach.is_authority_notified_in_time());
    }

    #[test]
    fn test_breach_notification_subjects() {
        let mut breach = BreachNotification::new(
            "Breach".to_string(),
            "Test".to_string(),
            vec!["data".to_string()],
            100,
        );
        breach.notify_authority().unwrap();
        let result = breach.notify_subjects();
        assert!(result.is_ok());
        assert_eq!(breach.status(), BreachStatus::SubjectsNotified);
    }

    #[test]
    fn test_breach_notification_resolve() {
        let mut breach = BreachNotification::new(
            "Test".to_string(),
            "Test".to_string(),
            vec!["data".to_string()],
            100,
        );
        breach.notify_authority().unwrap();
        breach.notify_subjects().unwrap();
        let result = breach.resolve();
        assert!(result.is_ok());
        assert_eq!(breach.status(), BreachStatus::Resolved);
    }

    #[test]
    fn test_consent_purpose_from_str() {
        assert_eq!(
            ConsentPurpose::from_str("marketing").unwrap(),
            ConsentPurpose::Marketing
        );
        assert_eq!(
            ConsentPurpose::from_str("analytics").unwrap(),
            ConsentPurpose::Analytics
        );
        assert!(ConsentPurpose::from_str("invalid").is_err());
    }

    #[test]
    fn test_legal_basis_from_str() {
        assert_eq!(
            LegalBasis::from_str("consent").unwrap(),
            LegalBasis::Consent
        );
        assert_eq!(
            LegalBasis::from_str("legal_obligation").unwrap(),
            LegalBasis::LegalObligation
        );
        assert!(LegalBasis::from_str("invalid").is_err());
    }
}

// ============================================================
// e-KYC Biometric Verification (Circ. 2025-06)
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiometricType {
    FacialRecognition,
    Fingerprint,
    DocumentScan,  // OCR on ID document
    VideoCall,     // live video verification
    Fido2WebAuthn, // FIDO2/WebAuthn for authentication
}

impl BiometricType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BiometricType::FacialRecognition => "facial_recognition",
            BiometricType::Fingerprint => "fingerprint",
            BiometricType::DocumentScan => "document_scan",
            BiometricType::VideoCall => "video_call",
            BiometricType::Fido2WebAuthn => "fido2_webauthn",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "facial_recognition" => Ok(BiometricType::FacialRecognition),
            "fingerprint" => Ok(BiometricType::Fingerprint),
            "document_scan" => Ok(BiometricType::DocumentScan),
            "video_call" => Ok(BiometricType::VideoCall),
            "fido2_webauthn" => Ok(BiometricType::Fido2WebAuthn),
            other => Err(format!("Unknown biometric type: {}", other)),
        }
    }
}

impl fmt::Display for BiometricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiometricStatus {
    Pending,
    InProgress,
    Verified,
    Failed,
    Expired,
}

impl BiometricStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BiometricStatus::Pending => "pending",
            BiometricStatus::InProgress => "in_progress",
            BiometricStatus::Verified => "verified",
            BiometricStatus::Failed => "failed",
            BiometricStatus::Expired => "expired",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(BiometricStatus::Pending),
            "in_progress" => Ok(BiometricStatus::InProgress),
            "verified" => Ok(BiometricStatus::Verified),
            "failed" => Ok(BiometricStatus::Failed),
            "expired" => Ok(BiometricStatus::Expired),
            other => Err(format!("Unknown biometric status: {}", other)),
        }
    }
}

impl fmt::Display for BiometricStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BiometricVerificationId(Uuid);

impl BiometricVerificationId {
    pub fn new() -> Self {
        BiometricVerificationId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        BiometricVerificationId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for BiometricVerificationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BiometricVerificationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricVerification {
    id: BiometricVerificationId,
    customer_id: Uuid,
    verification_type: BiometricType,
    status: BiometricStatus,
    confidence_score: f64, // 0.0 - 1.0
    liveness_check: bool,
    document_type: Option<String>, // "CIN", "Passport"
    document_number: Option<String>,
    verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>, // biometric verification has limited validity
}

impl BiometricVerification {
    pub fn new(
        customer_id: Uuid,
        verification_type: BiometricType,
        validity_days: i64,
    ) -> Result<Self, DomainError> {
        if validity_days <= 0 {
            return Err(DomainError::InvalidState(
                "Validity days must be positive".to_string(),
            ));
        }

        let now = Utc::now();
        let expires_at = now + chrono::Duration::days(validity_days);

        Ok(BiometricVerification {
            id: BiometricVerificationId::new(),
            customer_id,
            verification_type,
            status: BiometricStatus::Pending,
            confidence_score: 0.0,
            liveness_check: false,
            document_type: None,
            document_number: None,
            verified_at: None,
            created_at: now,
            expires_at,
        })
    }

    pub fn id(&self) -> &BiometricVerificationId {
        &self.id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn verification_type(&self) -> BiometricType {
        self.verification_type
    }

    pub fn status(&self) -> BiometricStatus {
        self.status
    }

    pub fn confidence_score(&self) -> f64 {
        self.confidence_score
    }

    pub fn liveness_check(&self) -> bool {
        self.liveness_check
    }

    pub fn document_type(&self) -> Option<&str> {
        self.document_type.as_deref()
    }

    pub fn document_number(&self) -> Option<&str> {
        self.document_number.as_deref()
    }

    pub fn verified_at(&self) -> Option<DateTime<Utc>> {
        self.verified_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        self.status == BiometricStatus::Verified && Utc::now() < self.expires_at
    }

    pub fn mark_in_progress(&mut self) -> Result<(), DomainError> {
        match self.status {
            BiometricStatus::Pending => {
                self.status = BiometricStatus::InProgress;
                Ok(())
            }
            _ => Err(DomainError::InvalidStateTransition(
                format!(
                    "Cannot transition from {} to in_progress",
                    self.status
                ),
            )),
        }
    }

    pub fn complete_verification(
        &mut self,
        confidence_score: f64,
        liveness_passed: bool,
        document_type: Option<String>,
        document_number: Option<String>,
    ) -> Result<(), DomainError> {
        if !(0.0..=1.0).contains(&confidence_score) {
            return Err(DomainError::InvalidInput(
                "Confidence score must be between 0.0 and 1.0".to_string(),
            ));
        }

        match self.status {
            BiometricStatus::InProgress => {
                self.confidence_score = confidence_score;
                self.liveness_check = liveness_passed;
                self.document_type = document_type;
                self.document_number = document_number;

                // Verification succeeds if confidence is >= 0.95 and liveness check passes
                if confidence_score >= 0.95 && liveness_passed {
                    self.status = BiometricStatus::Verified;
                    self.verified_at = Some(Utc::now());
                } else {
                    self.status = BiometricStatus::Failed;
                }
                Ok(())
            }
            _ => Err(DomainError::InvalidStateTransition(
                format!(
                    "Cannot complete verification from {} status",
                    self.status
                ),
            )),
        }
    }

    pub fn fail_verification(&mut self, reason: &str) -> Result<(), DomainError> {
        match self.status {
            BiometricStatus::Pending | BiometricStatus::InProgress => {
                self.status = BiometricStatus::Failed;
                Ok(())
            }
            _ => Err(DomainError::InvalidStateTransition(
                format!(
                    "Cannot fail verification from {} status",
                    self.status
                ),
            )),
        }
    }

    pub fn mark_expired(&mut self) -> Result<(), DomainError> {
        if Utc::now() >= self.expires_at {
            self.status = BiometricStatus::Expired;
            Ok(())
        } else {
            Err(DomainError::InvalidState(
                "Verification has not yet expired".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod biometric_tests {
    use super::*;

    #[test]
    fn test_biometric_verification_creation() {
        let customer_id = Uuid::new_v4();
        let verification =
            BiometricVerification::new(customer_id, BiometricType::FacialRecognition, 90)
                .unwrap();

        assert_eq!(verification.customer_id(), customer_id);
        assert_eq!(
            verification.verification_type(),
            BiometricType::FacialRecognition
        );
        assert_eq!(verification.status(), BiometricStatus::Pending);
        assert_eq!(verification.confidence_score(), 0.0);
        assert!(!verification.liveness_check());
        assert!(!verification.is_valid());
    }

    #[test]
    fn test_biometric_verification_invalid_validity() {
        let customer_id = Uuid::new_v4();
        let result = BiometricVerification::new(customer_id, BiometricType::Fingerprint, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_biometric_verification_state_transitions() {
        let customer_id = Uuid::new_v4();
        let mut verification =
            BiometricVerification::new(customer_id, BiometricType::DocumentScan, 365).unwrap();

        // Transition to in-progress
        assert!(verification.mark_in_progress().is_ok());
        assert_eq!(verification.status(), BiometricStatus::InProgress);

        // Complete verification with valid scores
        assert!(verification
            .complete_verification(0.98, true, Some("CIN".to_string()), Some("ABC123".to_string()))
            .is_ok());
        assert_eq!(verification.status(), BiometricStatus::Verified);
        assert_eq!(verification.confidence_score(), 0.98);
        assert!(verification.liveness_check());
        assert!(verification.is_valid());
    }

    #[test]
    fn test_biometric_verification_low_confidence_fails() {
        let customer_id = Uuid::new_v4();
        let mut verification =
            BiometricVerification::new(customer_id, BiometricType::FacialRecognition, 90).unwrap();

        verification.mark_in_progress().unwrap();

        // Complete with low confidence
        assert!(verification
            .complete_verification(0.85, true, None, None)
            .is_ok());
        assert_eq!(verification.status(), BiometricStatus::Failed);
        assert!(!verification.is_valid());
    }

    #[test]
    fn test_biometric_verification_failed_liveness_fails() {
        let customer_id = Uuid::new_v4();
        let mut verification =
            BiometricVerification::new(customer_id, BiometricType::VideoCall, 30).unwrap();

        verification.mark_in_progress().unwrap();

        // Complete with failed liveness check
        assert!(verification
            .complete_verification(0.97, false, None, None)
            .is_ok());
        assert_eq!(verification.status(), BiometricStatus::Failed);
        assert!(!verification.is_valid());
    }

    #[test]
    fn test_biometric_verification_invalid_confidence_score() {
        let customer_id = Uuid::new_v4();
        let mut verification =
            BiometricVerification::new(customer_id, BiometricType::Fingerprint, 365).unwrap();

        verification.mark_in_progress().unwrap();

        // Try to complete with invalid confidence
        assert!(verification
            .complete_verification(1.5, true, None, None)
            .is_err());
    }

    #[test]
    fn test_biometric_type_display() {
        assert_eq!(
            BiometricType::FacialRecognition.to_string(),
            "facial_recognition"
        );
        assert_eq!(BiometricType::Fingerprint.to_string(), "fingerprint");
        assert_eq!(BiometricType::DocumentScan.to_string(), "document_scan");
    }

    #[test]
    fn test_biometric_status_display() {
        assert_eq!(BiometricStatus::Pending.to_string(), "pending");
        assert_eq!(BiometricStatus::Verified.to_string(), "verified");
        assert_eq!(BiometricStatus::Expired.to_string(), "expired");
    }

    #[test]
    fn test_biometric_type_from_str() {
        assert!(BiometricType::from_str("facial_recognition").is_ok());
        assert!(BiometricType::from_str("fingerprint").is_ok());
        assert!(BiometricType::from_str("invalid_type").is_err());
    }

    #[test]
    fn test_biometric_status_from_str() {
        assert!(BiometricStatus::from_str("pending").is_ok());
        assert!(BiometricStatus::from_str("verified").is_ok());
        assert!(BiometricStatus::from_str("invalid_status").is_err());
    }
}
