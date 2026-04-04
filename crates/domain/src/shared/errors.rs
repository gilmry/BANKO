use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid money: {0}")]
    InvalidMoney(String),

    #[error("Invalid currency: {0}")]
    InvalidCurrency(String),

    #[error("Invalid percentage: {0}")]
    InvalidPercentage(String),

    #[error("Invalid RIB: {0}")]
    InvalidRib(String),

    #[error("Invalid BIC: {0}")]
    InvalidBic(String),

    #[error("Invalid email address: {0}")]
    InvalidEmail(String),

    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),

    #[error("Invalid account number: {0}")]
    InvalidAccountNumber(String),

    #[error("Invalid customer ID: {0}")]
    InvalidCustomerId(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid password hash: {0}")]
    InvalidPasswordHash(String),

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Invalid user: {0}")]
    InvalidUser(String),

    #[error("INPDP consent is required")]
    ConsentRequired,

    #[error("Legal entity must have at least one beneficial owner")]
    MissingBeneficiaries,

    #[error("Invalid CIN: {0}")]
    InvalidCin(String),

    #[error("Invalid risk score: {0}")]
    InvalidRiskScore(String),

    #[error("KYC not validated")]
    KycNotValidated,

    #[error("Invalid customer status: {0}")]
    InvalidCustomerStatus(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Account not found")]
    AccountNotFound,

    #[error("Account is closed")]
    AccountClosed,

    #[error("Account is suspended")]
    AccountSuspended,

    #[error("Invalid account type: {0}")]
    InvalidAccountType(String),

    #[error("Invalid movement: {0}")]
    InvalidMovement(String),
}
