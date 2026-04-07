use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataHubError {
    #[error("Data entity not found")]
    DataEntityNotFound,

    #[error("Data quality rule not found")]
    RuleNotFound,

    #[error("Data lineage not found")]
    LineageNotFound,

    #[error("Data reconciliation not found")]
    ReconciliationNotFound,

    #[error("Master data record not found")]
    MasterDataRecordNotFound,

    #[error("Governance policy not found")]
    PolicyNotFound,

    #[error("Golden record already exists for canonical_id")]
    GoldenRecordExists,

    #[error("Invalid quality score: {0}")]
    InvalidQualityScore(String),

    #[error("Data quality threshold not met: {0}")]
    QualityThresholdNotMet(String),

    #[error("Reconciliation conflict: {0}")]
    ReconciliationConflict(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
