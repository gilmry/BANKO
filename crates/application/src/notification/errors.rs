use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Notification not found")]
    NotFound,

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Recipient opted out of {0} notifications")]
    RecipientOptedOut(String),

    #[error("Failed to send notification: {0}")]
    SendFailed(String),

    #[error("Duplicate notification within dedup window")]
    DuplicateNotification,

    #[error("Invalid channel: {0}")]
    InvalidChannel(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
