use thiserror::Error;

#[derive(Debug, Error)]
pub enum GovernanceServiceError {
    #[error("Audit entry not found")]
    AuditEntryNotFound,

    #[error("Committee not found")]
    CommitteeNotFound,

    #[error("Control check not found")]
    ControlCheckNotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Integrity violation: {0}")]
    IntegrityViolation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
