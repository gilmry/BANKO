use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- DataEntityId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataEntityId(Uuid);

impl DataEntityId {
    pub fn new() -> Self {
        DataEntityId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DataEntityId(id)
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Uuid::parse_str(s)
            .map(DataEntityId)
            .map_err(|_| DomainError::ValidationError("Invalid DataEntityId".to_string()))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DataEntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataEntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- DataEntityType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataEntityType {
    Customer,
    Account,
    Transaction,
    Product,
    Arrangement,
    Payment,
    Loan,
    Collateral,
    Beneficiary,
    AuditLog,
    ComplianceRecord,
}

impl DataEntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Customer => "customer",
            Self::Account => "account",
            Self::Transaction => "transaction",
            Self::Product => "product",
            Self::Arrangement => "arrangement",
            Self::Payment => "payment",
            Self::Loan => "loan",
            Self::Collateral => "collateral",
            Self::Beneficiary => "beneficiary",
            Self::AuditLog => "audit_log",
            Self::ComplianceRecord => "compliance_record",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "customer" => Ok(Self::Customer),
            "account" => Ok(Self::Account),
            "transaction" => Ok(Self::Transaction),
            "product" => Ok(Self::Product),
            "arrangement" => Ok(Self::Arrangement),
            "payment" => Ok(Self::Payment),
            "loan" => Ok(Self::Loan),
            "collateral" => Ok(Self::Collateral),
            "beneficiary" => Ok(Self::Beneficiary),
            "audit_log" => Ok(Self::AuditLog),
            "compliance_record" => Ok(Self::ComplianceRecord),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown entity type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for DataEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- DataQualityRuleId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataQualityRuleId(Uuid);

impl DataQualityRuleId {
    pub fn new() -> Self {
        DataQualityRuleId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DataQualityRuleId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DataQualityRuleId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataQualityRuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- DataQualityScore ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataQualityScore(u8);

impl DataQualityScore {
    pub fn new(score: u8) -> Result<Self, DomainError> {
        if score > 100 {
            return Err(DomainError::InvalidPercentage(format!(
                "Quality score must be 0-100, got {}",
                score
            )));
        }
        Ok(DataQualityScore(score))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn is_quarantined(&self) -> bool {
        self.0 < 50
    }
}

impl fmt::Display for DataQualityScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- DataLineageId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataLineageId(Uuid);

impl DataLineageId {
    pub fn new() -> Self {
        DataLineageId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DataLineageId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DataLineageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataLineageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- TransformationType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformationType {
    Direct,
    Computed,
    Aggregated,
    Derived,
}

impl TransformationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Computed => "computed",
            Self::Aggregated => "aggregated",
            Self::Derived => "derived",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "direct" => Ok(Self::Direct),
            "computed" => Ok(Self::Computed),
            "aggregated" => Ok(Self::Aggregated),
            "derived" => Ok(Self::Derived),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown transformation type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for TransformationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- DataReconciliationId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataReconciliationId(Uuid);

impl DataReconciliationId {
    pub fn new() -> Self {
        DataReconciliationId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DataReconciliationId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DataReconciliationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataReconciliationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- ReconciliationStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconciliationStatus {
    Pending,
    InProgress,
    Resolved,
    Escalated,
}

impl ReconciliationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Resolved => "resolved",
            Self::Escalated => "escalated",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "pending" => Ok(Self::Pending),
            "in_progress" => Ok(Self::InProgress),
            "resolved" => Ok(Self::Resolved),
            "escalated" => Ok(Self::Escalated),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown reconciliation status: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for ReconciliationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- MasterDataRecordId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MasterDataRecordId(Uuid);

impl MasterDataRecordId {
    pub fn new() -> Self {
        MasterDataRecordId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        MasterDataRecordId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for MasterDataRecordId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MasterDataRecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- DataGovernancePolicyId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataGovernancePolicyId(Uuid);

impl DataGovernancePolicyId {
    pub fn new() -> Self {
        DataGovernancePolicyId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DataGovernancePolicyId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DataGovernancePolicyId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DataGovernancePolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- DataClassification ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

impl DataClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Confidential => "confidential",
            Self::Restricted => "restricted",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "public" => Ok(Self::Public),
            "internal" => Ok(Self::Internal),
            "confidential" => Ok(Self::Confidential),
            "restricted" => Ok(Self::Restricted),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown classification: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for DataClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- DataEntityStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataEntityStatus {
    Active,
    Stale,
    Quarantined,
}

impl DataEntityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Stale => "stale",
            Self::Quarantined => "quarantined",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s {
            "active" => Ok(Self::Active),
            "stale" => Ok(Self::Stale),
            "quarantined" => Ok(Self::Quarantined),
            _ => Err(DomainError::ValidationError(format!(
                "Unknown entity status: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for DataEntityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
