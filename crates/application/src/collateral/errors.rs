use banko_domain::shared::errors::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub enum CollateralApplicationError {
    // Domain errors
    DomainError(String),

    // Application-specific errors
    RepositoryError(String),

    // Validation errors
    ValidationError(String),

    // Not found errors
    CollateralNotFound,

    // Business logic errors
    LtvComplianceViolation {
        current_ltv: f64,
        max_ltv: f64,
    },

    InsurancePolicyMissing,

    RevaluationDue,

    InvalidStateTransition(String),

    AllocationError(String),
}

impl std::fmt::Display for CollateralApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollateralApplicationError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            CollateralApplicationError::RepositoryError(msg) => {
                write!(f, "Repository error: {}", msg)
            }
            CollateralApplicationError::ValidationError(msg) => {
                write!(f, "Validation error: {}", msg)
            }
            CollateralApplicationError::CollateralNotFound => {
                write!(f, "Collateral not found")
            }
            CollateralApplicationError::LtvComplianceViolation { current_ltv, max_ltv } => {
                write!(
                    f,
                    "LTV compliance violation: {:.2}% exceeds {:.2}%",
                    current_ltv * 100.0,
                    max_ltv * 100.0
                )
            }
            CollateralApplicationError::InsurancePolicyMissing => {
                write!(f, "Insurance policy missing for real estate collateral")
            }
            CollateralApplicationError::RevaluationDue => {
                write!(f, "Collateral revaluation is due")
            }
            CollateralApplicationError::InvalidStateTransition(msg) => {
                write!(f, "Invalid state transition: {}", msg)
            }
            CollateralApplicationError::AllocationError(msg) => {
                write!(f, "Allocation error: {}", msg)
            }
        }
    }
}

impl From<DomainError> for CollateralApplicationError {
    fn from(err: DomainError) -> Self {
        CollateralApplicationError::DomainError(err.to_string())
    }
}

impl From<String> for CollateralApplicationError {
    fn from(err: String) 