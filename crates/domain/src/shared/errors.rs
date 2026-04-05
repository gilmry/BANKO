use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
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

    // --- Credit errors ---

    #[error("Loan not found")]
    LoanNotFound,

    #[error("Invalid loan status: {0}")]
    InvalidLoanStatus(String),

    #[error("Invalid asset class: {0}")]
    InvalidAssetClass(String),

    #[error("Invalid loan state transition: {0}")]
    InvalidLoanTransition(String),

    #[error("Insufficient provision: {0}")]
    InsufficientProvision(String),

    // --- AML errors ---

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Invalid alert: {0}")]
    InvalidAlert(String),

    #[error("Invalid investigation: {0}")]
    InvalidInvestigation(String),

    #[error("Invalid investigation transition: {0}")]
    InvalidInvestigationTransition(String),

    #[error("AML threshold exceeded")]
    AmlThresholdExceeded,

    #[error("Asset freeze is irrevocable without CTAF authorization")]
    FreezeIrrevocable,

    #[error("Account is frozen")]
    AccountFrozen,

    #[error("Invalid suspicion report: {0}")]
    InvalidSuspicionReport(String),

    // --- Sanctions errors ---

    #[error("Invalid sanction entry: {0}")]
    InvalidSanctionEntry(String),

    #[error("Invalid sanction list: {0}")]
    InvalidSanctionList(String),

    #[error("Invalid screening result: {0}")]
    InvalidScreeningResult(String),

    #[error("Sanctions hit detected: {0}")]
    SanctionsHitDetected(String),

    #[error("Screening required before payment")]
    ScreeningRequired,

    #[error("Sanction entry not found")]
    SanctionEntryNotFound,

    #[error("Sanction list not found")]
    SanctionListNotFound,

    // --- Prudential errors ---

    #[error("Solvency ratio breach: {ratio:.2}% < minimum {minimum:.2}%")]
    SolvencyRatioBreach { ratio: f64, minimum: f64 },

    #[error("Tier 1 ratio breach: {ratio:.2}% < minimum {minimum:.2}%")]
    Tier1RatioBreach { ratio: f64, minimum: f64 },

    #[error("Credit-to-deposit ratio breach: {ratio:.2}% > maximum {maximum:.2}%")]
    CreditToDepositBreach { ratio: f64, maximum: f64 },

    #[error("Concentration breach for beneficiary {beneficiary_id}: {ratio:.2}% > maximum {maximum:.2}%")]
    ConcentrationBreach {
        beneficiary_id: uuid::Uuid,
        ratio: f64,
        maximum: f64,
    },

    #[error("Invalid prudential data: {0}")]
    InvalidPrudentialData(String),

    // --- Accounting errors ---

    #[error("Unbalanced entry: total_debit={total_debit} != total_credit={total_credit}")]
    UnbalancedEntry { total_debit: i64, total_credit: i64 },

    #[error("Invalid account code: {0}")]
    InvalidAccountCode(String),

    #[error("Entry already posted")]
    EntryAlreadyPosted,

    #[error("Entry not posted")]
    EntryNotPosted,

    #[error("Period closed: {period}")]
    PeriodClosed { period: String },

    #[error("Invalid journal entry: {0}")]
    InvalidJournalEntry(String),

    // --- Governance errors ---

    #[error("Invalid audit entry: {0}")]
    InvalidAuditEntry(String),

    #[error("Hash chain integrity violation at entry {entry_id}")]
    HashChainViolation { entry_id: String },

    #[error("Audit entry immutable — cannot modify")]
    AuditEntryImmutable,

    #[error("Invalid committee: {0}")]
    InvalidCommittee(String),

    #[error("Invalid control check: {0}")]
    InvalidControlCheck(String),

    // --- Reporting errors ---

    #[error("Invalid report: {0}")]
    InvalidReport(String),

    #[error("Report not found")]
    ReportNotFound,

    #[error("Report already submitted")]
    ReportAlreadySubmitted,

    #[error("Invalid report template: {0}")]
    InvalidReportTemplate(String),

    // --- Payment errors ---

    #[error("Invalid payment order: {0}")]
    InvalidPaymentOrder(String),

    #[error("Payment order not found")]
    PaymentOrderNotFound,

    #[error("Invalid payment status transition: {0}")]
    InvalidPaymentTransition(String),

    #[error("Sanctions screening required before payment execution")]
    SanctionsScreeningRequired,

    #[error("Payment blocked by sanctions screening: {0}")]
    PaymentBlockedBySanctions(String),

    #[error("Insufficient funds for payment")]
    InsufficientFundsForPayment,

    // --- Retention errors ---

    #[error("Retention period not met: closed at {closed_at}, minimum {minimum_years} years required")]
    RetentionPeriodNotMet {
        closed_at: String,
        minimum_years: u32,
    },

    #[error("Customer already anonymized")]
    CustomerAlreadyAnonymized,

    #[error("Customer not closed — cannot anonymize")]
    CustomerNotClosed,

    // --- Consent errors ---

    #[error("Consent already active for this purpose")]
    ConsentAlreadyActive,

    #[error("Consent not found")]
    ConsentNotFound,

    #[error("Invalid data request: {0}")]
    InvalidDataRequest(String),

    #[error("Data request already completed")]
    DataRequestAlreadyCompleted,
}
