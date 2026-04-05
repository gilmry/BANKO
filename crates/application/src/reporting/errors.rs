use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReportingServiceError {
    #[error("Report not found")]
    ReportNotFound,

    #[error("Template not found")]
    TemplateNotFound,

    #[error("No active template for report type")]
    NoActiveTemplate,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
