use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- Value Objects / Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntryId(Uuid);

impl EntryId {
    pub fn new() -> Self {
        EntryId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        EntryId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for EntryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- AccountCode (NCT bank chart: classes 1-7) ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountCode(String);

impl AccountCode {
    pub fn new(code: &str) -> Result<Self, DomainError> {
        let trimmed = code.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidAccountCode(
                "Account code cannot be empty".to_string(),
            ));
        }
        let first = trimmed.chars().next().unwrap();
        if !('1'..='7').contains(&first) {
            return Err(DomainError::InvalidAccountCode(format!(
                "Account code must start with digit 1-7, got: {trimmed}"
            )));
        }
        if !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::InvalidAccountCode(format!(
                "Account code must contain only digits: {trimmed}"
            )));
        }
        Ok(AccountCode(trimmed.to_string()))
    }

    /// Reconstruct from persistence (no validation)
    pub fn from_raw(code: String) -> Self {
        AccountCode(code)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn class(&self) -> u8 {
        self.0.as_bytes()[0] - b'0'
    }
}

impl fmt::Display for AccountCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JournalCode {
    OD, // Opérations Diverses
    CP, // Comptabilité Principale
    VT, // Virements
    IN, // Intérêts
    PR, // Provisions
}

impl JournalCode {
    pub fn as_str(&self) -> &str {
        match self {
            JournalCode::OD => "OD",
            JournalCode::CP => "CP",
            JournalCode::VT => "VT",
            JournalCode::IN => "IN",
            JournalCode::PR => "PR",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "OD" => Ok(JournalCode::OD),
            "CP" => Ok(JournalCode::CP),
            "VT" => Ok(JournalCode::VT),
            "IN" => Ok(JournalCode::IN),
            "PR" => Ok(JournalCode::PR),
            _ => Err(DomainError::InvalidJournalEntry(format!(
                "Unknown journal code: {s}"
            ))),
        }
    }
}

impl fmt::Display for JournalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntryStatus {
    Draft,
    Posted,
    Reversed,
}

impl EntryStatus {
    pub fn as_str(&self) -> &str {
        match self {
            EntryStatus::Draft => "Draft",
            EntryStatus::Posted => "Posted",
            EntryStatus::Reversed => "Reversed",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Draft" => Ok(EntryStatus::Draft),
            "Posted" => Ok(EntryStatus::Posted),
            "Reversed" => Ok(EntryStatus::Reversed),
            _ => Err(DomainError::InvalidJournalEntry(format!(
                "Unknown entry status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountType {
    pub fn as_str(&self) -> &str {
        match self {
            AccountType::Asset => "Asset",
            AccountType::Liability => "Liability",
            AccountType::Equity => "Equity",
            AccountType::Revenue => "Revenue",
            AccountType::Expense => "Expense",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Asset" => Ok(AccountType::Asset),
            "Liability" => Ok(AccountType::Liability),
            "Equity" => Ok(AccountType::Equity),
            "Revenue" => Ok(AccountType::Revenue),
            "Expense" => Ok(AccountType::Expense),
            _ => Err(DomainError::InvalidAccountCode(format!(
                "Unknown account type: {s}"
            ))),
        }
    }
}

// --- ECL Stage (IFRS 9) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EclStage {
    Stage1, // Low risk — 12-month ECL
    Stage2, // Significant increase in credit risk — Lifetime ECL
    Stage3, // Default — Lifetime ECL (credit-impaired)
}

impl EclStage {
    pub fn as_str(&self) -> &str {
        match self {
            EclStage::Stage1 => "Stage1",
            EclStage::Stage2 => "Stage2",
            EclStage::Stage3 => "Stage3",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Stage1" => Ok(EclStage::Stage1),
            "Stage2" => Ok(EclStage::Stage2),
            "Stage3" => Ok(EclStage::Stage3),
            _ => Err(DomainError::InvalidJournalEntry(format!(
                "Unknown ECL stage: {s}"
            ))),
        }
    }
}

// --- JournalLine ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalLine {
    line_id: Uuid,
    account_code: AccountCode,
    debit: i64,
    credit: i64,
    description: Option<String>,
}

impl JournalLine {
    pub fn new(
        account_code: AccountCode,
        debit: i64,
        credit: i64,
        description: Option<String>,
    ) -> Result<Self, DomainError> {
        if debit < 0 || credit < 0 {
            return Err(DomainError::InvalidJournalEntry(
                "Debit and credit must be non-negative".to_string(),
            ));
        }
        if debit == 0 && credit == 0 {
            return Err(DomainError::InvalidJournalEntry(
                "Line must have either debit or credit > 0".to_string(),
            ));
        }
        Ok(JournalLine {
            line_id: Uuid::new_v4(),
            account_code,
            debit,
            credit,
            description,
        })
    }

    /// Reconstruct from persistence
    pub fn from_raw(
        line_id: Uuid,
        account_code: AccountCode,
        debit: i64,
        credit: i64,
        description: Option<String>,
    ) -> Self {
        JournalLine {
            line_id,
            account_code,
            debit,
            credit,
            description,
        }
    }

    pub fn line_id(&self) -> Uuid {
        self.line_id
    }
    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }
    pub fn debit(&self) -> i64 {
        self.debit
    }
    pub fn credit(&self) -> i64 {
        self.credit
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// --- JournalEntry Aggregate Root ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry {
    entry_id: EntryId,
    journal_code: JournalCode,
    entry_date: NaiveDate,
    description: String,
    lines: Vec<JournalLine>,
    status: EntryStatus,
    reversal_of: Option<EntryId>,
    created_at: DateTime<Utc>,
    posted_at: Option<DateTime<Utc>>,
}

impl JournalEntry {
    /// Create a new journal entry, enforcing INV-11 (debit == credit)
    pub fn new(
        journal_code: JournalCode,
        entry_date: NaiveDate,
        description: String,
        lines: Vec<JournalLine>,
    ) -> Result<Self, DomainError> {
        if lines.len() < 2 {
            return Err(DomainError::InvalidJournalEntry(
                "Entry must have at least 2 lines".to_string(),
            ));
        }

        if description.trim().is_empty() {
            return Err(DomainError::InvalidJournalEntry(
                "Description cannot be empty".to_string(),
            ));
        }

        let total_debit: i64 = lines.iter().map(|l| l.debit).sum();
        let total_credit: i64 = lines.iter().map(|l| l.credit).sum();

        // INV-11: Toute écriture comptable est équilibrée
        if total_debit != total_credit {
            return Err(DomainError::UnbalancedEntry {
                total_debit,
                total_credit,
            });
        }

        Ok(JournalEntry {
            entry_id: EntryId::new(),
            journal_code,
            entry_date,
            description,
            lines,
            status: EntryStatus::Draft,
            reversal_of: None,
            created_at: Utc::now(),
            posted_at: None,
        })
    }

    /// Reconstruct from persistence (bypasses validation)
    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        entry_id: EntryId,
        journal_code: JournalCode,
        entry_date: NaiveDate,
        description: String,
        lines: Vec<JournalLine>,
        status: EntryStatus,
        reversal_of: Option<EntryId>,
        created_at: DateTime<Utc>,
        posted_at: Option<DateTime<Utc>>,
    ) -> Self {
        JournalEntry {
            entry_id,
            journal_code,
            entry_date,
            description,
            lines,
            status,
            reversal_of,
            created_at,
            posted_at,
        }
    }

    // --- Getters ---

    pub fn entry_id(&self) -> &EntryId {
        &self.entry_id
    }
    pub fn journal_code(&self) -> JournalCode {
        self.journal_code
    }
    pub fn entry_date(&self) -> NaiveDate {
        self.entry_date
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn lines(&self) -> &[JournalLine] {
        &self.lines
    }
    pub fn status(&self) -> EntryStatus {
        self.status
    }
    pub fn reversal_of(&self) -> Option<&EntryId> {
        self.reversal_of.as_ref()
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn posted_at(&self) -> Option<DateTime<Utc>> {
        self.posted_at
    }

    // --- Calculations ---

    pub fn total_debit(&self) -> i64 {
        self.lines.iter().map(|l| l.debit).sum()
    }

    pub fn total_credit(&self) -> i64 {
        self.lines.iter().map(|l| l.credit).sum()
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debit() == self.total_credit()
    }

    // --- State transitions ---

    /// Transition from Draft → Posted (immutable after this)
    pub fn post(&mut self) -> Result<(), DomainError> {
        if self.status != EntryStatus::Draft {
            return Err(DomainError::EntryAlreadyPosted);
        }
        self.status = EntryStatus::Posted;
        self.posted_at = Some(Utc::now());
        Ok(())
    }

    /// Create a reversal entry (contre-passation): new entry with swapped debit/credit
    pub fn create_reversal(&self) -> Result<JournalEntry, DomainError> {
        if self.status != EntryStatus::Posted {
            return Err(DomainError::EntryNotPosted);
        }

        let reversed_lines: Vec<JournalLine> = self
            .lines
            .iter()
            .map(|line| {
                JournalLine::new(
                    line.account_code.clone(),
                    line.credit, // swap: debit ← credit
                    line.debit,  // swap: credit ← debit
                    Some(format!("Reversal: {}", line.description().unwrap_or(""))),
                )
                .unwrap()
            })
            .collect();

        let mut reversal = JournalEntry::new(
            self.journal_code,
            self.entry_date,
            format!("Reversal of {}", self.entry_id),
            reversed_lines,
        )?;

        reversal.reversal_of = Some(self.entry_id.clone());
        Ok(reversal)
    }

    /// Mark this entry as reversed (called after reversal entry is created)
    pub fn mark_reversed(&mut self) -> Result<(), DomainError> {
        if self.status != EntryStatus::Posted {
            return Err(DomainError::EntryNotPosted);
        }
        self.status = EntryStatus::Reversed;
        Ok(())
    }
}

// --- LedgerAccount (Chart of Accounts entry) ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerAccount {
    account_code: AccountCode,
    label: String,
    account_type: AccountType,
    nct_ref: Option<String>,
}

impl LedgerAccount {
    pub fn new(
        account_code: AccountCode,
        label: String,
        account_type: AccountType,
        nct_ref: Option<String>,
    ) -> Self {
        LedgerAccount {
            account_code,
            label,
            account_type,
            nct_ref,
        }
    }

    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn account_type(&self) -> AccountType {
        self.account_type
    }
    pub fn nct_ref(&self) -> Option<&str> {
        self.nct_ref.as_deref()
    }
}

// --- ChartOfAccounts (FR-082: NCT Tunisian classification) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountClass {
    Class1, // Equity & liabilities
    Class2, // Liabilities
    Class3, // Assets
    Class4, // Liabilities (current)
    Class5, // Assets (current)
    Class6, // Expenses
    Class7, // Revenue
}

impl AccountClass {
    pub fn from_digit(d: u8) -> Result<Self, DomainError> {
        match d {
            1 => Ok(AccountClass::Class1),
            2 => Ok(AccountClass::Class2),
            3 => Ok(AccountClass::Class3),
            4 => Ok(AccountClass::Class4),
            5 => Ok(AccountClass::Class5),
            6 => Ok(AccountClass::Class6),
            7 => Ok(AccountClass::Class7),
            _ => Err(DomainError::InvalidAccountCode(
                "Invalid account class (must be 1-7)".to_string(),
            )),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AccountClass::Class1 => "Class1",
            AccountClass::Class2 => "Class2",
            AccountClass::Class3 => "Class3",
            AccountClass::Class4 => "Class4",
            AccountClass::Class5 => "Class5",
            AccountClass::Class6 => "Class6",
            AccountClass::Class7 => "Class7",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChartOfAccounts {
    account_code: AccountCode,
    label: String,
    account_class: AccountClass,
    account_type: AccountType,
    nct_ref: Option<String>,
    parent_code: Option<AccountCode>,
    is_active: bool,
}

impl ChartOfAccounts {
    pub fn new(
        account_code: AccountCode,
        label: String,
        account_class: AccountClass,
        account_type: AccountType,
        nct_ref: Option<String>,
        parent_code: Option<AccountCode>,
    ) -> Result<Self, DomainError> {
        if label.trim().is_empty() {
            return Err(DomainError::InvalidAccountCode(
                "Account label cannot be empty".to_string(),
            ));
        }

        // Validate that account class matches account code first digit
        let account_class_from_code = AccountClass::from_digit(account_code.class())?;
        if account_class != account_class_from_code {
            return Err(DomainError::InvalidAccountCode(format!(
                "Account class mismatch: code implies {:?}, but {:?} specified",
                account_class_from_code, account_class
            )));
        }

        Ok(ChartOfAccounts {
            account_code,
            label,
            account_class,
            account_type,
            nct_ref,
            parent_code,
            is_active: true,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        account_code: AccountCode,
        label: String,
        account_class: AccountClass,
        account_type: AccountType,
        nct_ref: Option<String>,
        parent_code: Option<AccountCode>,
        is_active: bool,
    ) -> Self {
        ChartOfAccounts {
            account_code,
            label,
            account_class,
            account_type,
            nct_ref,
            parent_code,
            is_active,
        }
    }

    pub fn account_code(&self) -> &AccountCode {
        &self.account_code
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn account_class(&self) -> AccountClass {
        self.account_class
    }
    pub fn account_type(&self) -> AccountType {
        self.account_type
    }
    pub fn nct_ref(&self) -> Option<&str> {
        self.nct_ref.as_deref()
    }
    pub fn parent_code(&self) -> Option<&AccountCode> {
        self.parent_code.as_ref()
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

// --- PeriodClosing (FR-093/094/095: Daily, Monthly, Annual closings) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PeriodType {
    Daily,
    Monthly,
    Annual,
}

impl PeriodType {
    pub fn as_str(&self) -> &str {
        match self {
            PeriodType::Daily => "Daily",
            PeriodType::Monthly => "Monthly",
            PeriodType::Annual => "Annual",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Daily" => Ok(PeriodType::Daily),
            "Monthly" => Ok(PeriodType::Monthly),
            "Annual" => Ok(PeriodType::Annual),
            _ => Err(DomainError::InvalidJournalEntry(format!(
                "Unknown period type: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClosingStatus {
    Open,
    InProgress,
    Closed,
    Archived,
}

impl ClosingStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ClosingStatus::Open => "Open",
            ClosingStatus::InProgress => "InProgress",
            ClosingStatus::Closed => "Closed",
            ClosingStatus::Archived => "Archived",
        }
    }

    pub fn from_str_value(s: &str) -> Result<Self, DomainError> {
        match s {
            "Open" => Ok(ClosingStatus::Open),
            "InProgress" => Ok(ClosingStatus::InProgress),
            "Closed" => Ok(ClosingStatus::Closed),
            "Archived" => Ok(ClosingStatus::Archived),
            _ => Err(DomainError::InvalidJournalEntry(format!(
                "Unknown closing status: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeriodClosing {
    period_id: Uuid,
    period: String, // YYYY, YYYY-MM, or YYYY-MM-DD
    period_type: PeriodType,
    status: ClosingStatus,
    entries_count: i64,
    total_debits: i64,
    total_credits: i64,
    variance: i64,
    closed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl PeriodClosing {
    pub fn new(
        period: String,
        period_type: PeriodType,
    ) -> Result<Self, DomainError> {
        if period.trim().is_empty() {
            return Err(DomainError::InvalidJournalEntry(
                "Period cannot be empty".to_string(),
            ));
        }

        Ok(PeriodClosing {
            period_id: Uuid::new_v4(),
            period,
            period_type,
            status: ClosingStatus::Open,
            entries_count: 0,
            total_debits: 0,
            total_credits: 0,
            variance: 0,
            closed_at: None,
            created_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        period_id: Uuid,
        period: String,
        period_type: PeriodType,
        status: ClosingStatus,
        entries_count: i64,
        total_debits: i64,
        total_credits: i64,
        variance: i64,
        closed_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    ) -> Self {
        PeriodClosing {
            period_id,
            period,
            period_type,
            status,
            entries_count,
            total_debits,
            total_credits,
            variance,
            closed_at,
            created_at,
        }
    }

    // --- Getters ---

    pub fn period_id(&self) -> Uuid {
        self.period_id
    }
    pub fn period(&self) -> &str {
        &self.period
    }
    pub fn period_type(&self) -> PeriodType {
        self.period_type
    }
    pub fn status(&self) -> ClosingStatus {
        self.status
    }
    pub fn entries_count(&self) -> i64 {
        self.entries_count
    }
    pub fn total_debits(&self) -> i64 {
        self.total_debits
    }
    pub fn total_credits(&self) -> i64 {
        self.total_credits
    }
    pub fn variance(&self) -> i64 {
        self.variance
    }
    pub fn closed_at(&self) -> Option<DateTime<Utc>> {
        self.closed_at
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // --- State transitions ---

    pub fn start_closing(&mut self) -> Result<(), DomainError> {
        if self.status != ClosingStatus::Open {
            return Err(DomainError::InvalidJournalEntry(
                "Period must be Open to start closing".to_string(),
            ));
        }
        self.status = ClosingStatus::InProgress;
        Ok(())
    }

    pub fn complete_closing(
        &mut self,
        total_debits: i64,
        total_credits: i64,
        entries_count: i64,
    ) -> Result<(), DomainError> {
        if self.status != ClosingStatus::InProgress {
            return Err(DomainError::InvalidJournalEntry(
                "Period must be InProgress to complete closing".to_string(),
            ));
        }
        self.total_debits = total_debits;
        self.total_credits = total_credits;
        self.entries_count = entries_count;
        self.variance = (total_debits - total_credits).abs();
        self.status = ClosingStatus::Closed;
        self.closed_at = Some(Utc::now());
        Ok(())
    }

    pub fn archive(&mut self) -> Result<(), DomainError> {
        if self.status != ClosingStatus::Closed {
            return Err(DomainError::InvalidJournalEntry(
                "Period must be Closed to archive".to_string(),
            ));
        }
        self.status = ClosingStatus::Archived;
        Ok(())
    }

    pub fn is_balanced(&self) -> bool {
        self.total_debits == self.total_credits
    }
}

// --- DualPosting (FR-090: Dual engine NCT + IFRS 9) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PostingEngine {
    NCT,     // Tunisian accounting standard
    IFRS9,   // IFRS 9 for IFRS compliance
}

impl PostingEngine {
    pub fn as_str(&self) -> &str {
        match self {
            PostingEngine::NCT => "NCT",
            PostingEngine::IFRS9 => "IFRS9",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DualPosting {
    entry_id: EntryId,
    nct_entry: JournalEntry,
    ifrs9_entry: Option<JournalEntry>, // Alternative IFRS 9 posting
    posting_engines: Vec<PostingEngine>,
    created_at: DateTime<Utc>,
}

impl DualPosting {
    pub fn new(
        entry_id: EntryId,
        nct_entry: JournalEntry,
        ifrs9_entry: Option<JournalEntry>,
    ) -> Self {
        let mut engines = vec![PostingEngine::NCT];
        if ifrs9_entry.is_some() {
            engines.push(PostingEngine::IFRS9);
        }

        DualPosting {
            entry_id,
            nct_entry,
            ifrs9_entry,
            posting_engines: engines,
            created_at: Utc::now(),
        }
    }

    pub fn entry_id(&self) -> &EntryId {
        &self.entry_id
    }
    pub fn nct_entry(&self) -> &JournalEntry {
        &self.nct_entry
    }
    pub fn ifrs9_entry(&self) -> Option<&JournalEntry> {
        self.ifrs9_entry.as_ref()
    }
    pub fn posting_engines(&self) -> &[PostingEngine] {
        &self.posting_engines
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn has_dual_posting(&self) -> bool {
        self.posting_engines.len() > 1
    }
}

// --- ExpectedCreditLoss (IFRS 9 preparation) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedCreditLoss {
    loan_id: Uuid,
    stage: EclStage,
    probability_of_default_12m: f64, // 12-month PD (Stage 1)
    probability_of_default_lifetime: f64, // Lifetime PD (Stage 2/3)
    loss_given_default: f64,
    exposure_at_default: i64,
    ecl_amount_12m: i64,
    ecl_amount_lifetime: i64,
    calculated_at: DateTime<Utc>,
}

impl ExpectedCreditLoss {
    /// Create ECL with both 12-month and lifetime PDs (FR-089: IFRS 9)
    pub fn new(
        loan_id: Uuid,
        stage: EclStage,
        probability_of_default: f64,
        loss_given_default: f64,
        exposure_at_default: i64,
    ) -> Self {
        // For backward compatibility, use PD for 12m
        let ecl_amount_12m =
            (probability_of_default * loss_given_default * exposure_at_default as f64) as i64;

        // Lifetime PD is higher (approximation)
        let pd_lifetime = match stage {
            EclStage::Stage1 => probability_of_default * 2.0,
            EclStage::Stage2 => probability_of_default * 3.0,
            EclStage::Stage3 => 1.0,
        };
        let ecl_amount_lifetime =
            (pd_lifetime * loss_given_default * exposure_at_default as f64) as i64;

        ExpectedCreditLoss {
            loan_id,
            stage,
            probability_of_default_12m: probability_of_default,
            probability_of_default_lifetime: pd_lifetime,
            loss_given_default,
            exposure_at_default,
            ecl_amount_12m,
            ecl_amount_lifetime,
            calculated_at: Utc::now(),
        }
    }

    pub fn loan_id(&self) -> Uuid {
        self.loan_id
    }
    pub fn stage(&self) -> EclStage {
        self.stage
    }
    pub fn probability_of_default(&self) -> f64 {
        self.probability_of_default_12m
    }
    pub fn probability_of_default_12m(&self) -> f64 {
        self.probability_of_default_12m
    }
    pub fn probability_of_default_lifetime(&self) -> f64 {
        self.probability_of_default_lifetime
    }
    pub fn loss_given_default(&self) -> f64 {
        self.loss_given_default
    }
    pub fn exposure_at_default(&self) -> i64 {
        self.exposure_at_default
    }
    pub fn ecl_amount(&self) -> i64 {
        // Return 12m ECL for backward compatibility
        self.ecl_amount_12m
    }
    pub fn ecl_amount_12m(&self) -> i64 {
        self.ecl_amount_12m
    }
    pub fn ecl_amount_lifetime(&self) -> i64 {
        self.ecl_amount_lifetime
    }
    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_line(account: &str, debit: i64, credit: i64) -> JournalLine {
        JournalLine::new(AccountCode::new(account).unwrap(), debit, credit, None).unwrap()
    }

    // --- ACC-01: JournalEntry domain invariants ---

    #[test]
    fn test_balanced_entry_valid() {
        let lines = vec![make_line("31", 1000, 0), make_line("42", 0, 1000)];
        let entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Test entry".into(),
            lines,
        )
        .unwrap();

        assert!(entry.is_balanced());
        assert_eq!(entry.total_debit(), 1000);
        assert_eq!(entry.total_credit(), 1000);
        assert_eq!(entry.status(), EntryStatus::Draft);
    }

    #[test]
    fn test_unbalanced_entry_rejected() {
        // INV-11: debit 1000 / credit 999 → error
        let lines = vec![make_line("31", 1000, 0), make_line("42", 0, 999)];
        let result = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Bad entry".into(),
            lines,
        );
        assert!(matches!(
            result,
            Err(DomainError::UnbalancedEntry {
                total_debit: 1000,
                total_credit: 999
            })
        ));
    }

    #[test]
    fn test_entry_requires_minimum_two_lines() {
        let lines = vec![make_line("31", 1000, 0)];
        let result = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Single line".into(),
            lines,
        );
        assert!(matches!(result, Err(DomainError::InvalidJournalEntry(_))));
    }

    #[test]
    fn test_entry_requires_description() {
        let lines = vec![make_line("31", 500, 0), make_line("42", 0, 500)];
        let result = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "  ".into(),
            lines,
        );
        assert!(matches!(result, Err(DomainError::InvalidJournalEntry(_))));
    }

    #[test]
    fn test_post_entry() {
        let lines = vec![make_line("31", 5000, 0), make_line("42", 0, 5000)];
        let mut entry = JournalEntry::new(
            JournalCode::CP,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Posting test".into(),
            lines,
        )
        .unwrap();

        assert_eq!(entry.status(), EntryStatus::Draft);
        entry.post().unwrap();
        assert_eq!(entry.status(), EntryStatus::Posted);
        assert!(entry.posted_at().is_some());
    }

    #[test]
    fn test_double_post_rejected() {
        let lines = vec![make_line("31", 5000, 0), make_line("42", 0, 5000)];
        let mut entry = JournalEntry::new(
            JournalCode::CP,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Double post".into(),
            lines,
        )
        .unwrap();

        entry.post().unwrap();
        let result = entry.post();
        assert!(matches!(result, Err(DomainError::EntryAlreadyPosted)));
    }

    #[test]
    fn test_reversal_creates_swapped_entry() {
        let lines = vec![make_line("31", 1000, 0), make_line("42", 0, 1000)];
        let mut entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Original".into(),
            lines,
        )
        .unwrap();

        entry.post().unwrap();
        let reversal = entry.create_reversal().unwrap();

        // Reversal has swapped amounts
        assert_eq!(reversal.lines()[0].debit(), 0);
        assert_eq!(reversal.lines()[0].credit(), 1000);
        assert_eq!(reversal.lines()[1].debit(), 1000);
        assert_eq!(reversal.lines()[1].credit(), 0);
        assert!(reversal.is_balanced());
        assert_eq!(reversal.reversal_of().unwrap(), entry.entry_id());
    }

    #[test]
    fn test_reversal_requires_posted_status() {
        let lines = vec![make_line("31", 500, 0), make_line("42", 0, 500)];
        let entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Draft reversal".into(),
            lines,
        )
        .unwrap();

        let result = entry.create_reversal();
        assert!(matches!(result, Err(DomainError::EntryNotPosted)));
    }

    #[test]
    fn test_mark_reversed() {
        let lines = vec![make_line("31", 500, 0), make_line("42", 0, 500)];
        let mut entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "To reverse".into(),
            lines,
        )
        .unwrap();

        entry.post().unwrap();
        entry.mark_reversed().unwrap();
        assert_eq!(entry.status(), EntryStatus::Reversed);
    }

    // --- AccountCode validation ---

    #[test]
    fn test_account_code_valid() {
        assert!(AccountCode::new("31").is_ok());
        assert!(AccountCode::new("101").is_ok());
        assert!(AccountCode::new("7100").is_ok());
    }

    #[test]
    fn test_account_code_invalid_class() {
        assert!(AccountCode::new("0123").is_err()); // class 0
        assert!(AccountCode::new("8123").is_err()); // class 8
        assert!(AccountCode::new("9123").is_err()); // class 9
    }

    #[test]
    fn test_account_code_empty() {
        assert!(AccountCode::new("").is_err());
    }

    #[test]
    fn test_account_code_non_digits() {
        assert!(AccountCode::new("31A").is_err());
    }

    #[test]
    fn test_account_code_class() {
        assert_eq!(AccountCode::new("31").unwrap().class(), 3);
        assert_eq!(AccountCode::new("710").unwrap().class(), 7);
    }

    // --- JournalLine ---

    #[test]
    fn test_journal_line_valid_debit() {
        let line = JournalLine::new(
            AccountCode::new("31").unwrap(),
            1000,
            0,
            Some("Test".into()),
        )
        .unwrap();
        assert_eq!(line.debit(), 1000);
        assert_eq!(line.credit(), 0);
    }

    #[test]
    fn test_journal_line_valid_credit() {
        let line = JournalLine::new(AccountCode::new("42").unwrap(), 0, 1000, None).unwrap();
        assert_eq!(line.debit(), 0);
        assert_eq!(line.credit(), 1000);
    }

    #[test]
    fn test_journal_line_zero_both_rejected() {
        let result = JournalLine::new(AccountCode::new("31").unwrap(), 0, 0, None);
        assert!(matches!(result, Err(DomainError::InvalidJournalEntry(_))));
    }

    #[test]
    fn test_journal_line_negative_rejected() {
        let result = JournalLine::new(AccountCode::new("31").unwrap(), -100, 0, None);
        assert!(matches!(result, Err(DomainError::InvalidJournalEntry(_))));
    }

    // --- JournalCode ---

    #[test]
    fn test_journal_code_roundtrip() {
        for jc in [
            JournalCode::OD,
            JournalCode::CP,
            JournalCode::VT,
            JournalCode::IN,
            JournalCode::PR,
        ] {
            assert_eq!(JournalCode::from_str_value(jc.as_str()).unwrap(), jc);
        }
    }

    // --- EntryStatus ---

    #[test]
    fn test_entry_status_roundtrip() {
        for es in [
            EntryStatus::Draft,
            EntryStatus::Posted,
            EntryStatus::Reversed,
        ] {
            assert_eq!(EntryStatus::from_str_value(es.as_str()).unwrap(), es);
        }
    }

    // --- AccountType ---

    #[test]
    fn test_account_type_roundtrip() {
        for at in [
            AccountType::Asset,
            AccountType::Liability,
            AccountType::Equity,
            AccountType::Revenue,
            AccountType::Expense,
        ] {
            assert_eq!(AccountType::from_str_value(at.as_str()).unwrap(), at);
        }
    }

    // --- LedgerAccount ---

    #[test]
    fn test_ledger_account() {
        let acct = LedgerAccount::new(
            AccountCode::new("31").unwrap(),
            "Créances sur la clientèle".into(),
            AccountType::Asset,
            Some("NCT-24".into()),
        );
        assert_eq!(acct.label(), "Créances sur la clientèle");
        assert_eq!(acct.account_type(), AccountType::Asset);
        assert_eq!(acct.nct_ref(), Some("NCT-24"));
    }

    // --- ECL ---

    #[test]
    fn test_ecl_calculation() {
        let ecl = ExpectedCreditLoss::new(
            Uuid::new_v4(),
            EclStage::Stage1,
            0.02,      // 2% PD
            0.45,      // 45% LGD
            1_000_000, // 1M EAD
        );
        // ECL = 0.02 * 0.45 * 1_000_000 = 9000
        assert_eq!(ecl.ecl_amount(), 9000);
        assert_eq!(ecl.stage(), EclStage::Stage1);
    }

    #[test]
    fn test_ecl_stage_roundtrip() {
        for s in [EclStage::Stage1, EclStage::Stage2, EclStage::Stage3] {
            assert_eq!(EclStage::from_str_value(s.as_str()).unwrap(), s);
        }
    }

    // --- Multi-line entry ---

    #[test]
    fn test_multi_line_balanced_entry() {
        let lines = vec![
            make_line("31", 500, 0),
            make_line("32", 300, 0),
            make_line("42", 0, 600),
            make_line("43", 0, 200),
        ];
        let entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
            "Multi-line test".into(),
            lines,
        )
        .unwrap();
        assert!(entry.is_balanced());
        assert_eq!(entry.total_debit(), 800);
        assert_eq!(entry.total_credit(), 800);
    }

    #[test]
    fn test_entry_id_display() {
        let id = EntryId::new();
        let s = format!("{id}");
        assert!(!s.is_empty());
    }

    // --- AccountClass & ChartOfAccounts ---

    #[test]
    fn test_account_class_from_digit() {
        assert_eq!(AccountClass::from_digit(1).unwrap(), AccountClass::Class1);
        assert_eq!(AccountClass::from_digit(7).unwrap(), AccountClass::Class7);
        assert!(AccountClass::from_digit(0).is_err());
        assert!(AccountClass::from_digit(8).is_err());
    }

    #[test]
    fn test_chart_of_accounts_new_valid() {
        let coa = ChartOfAccounts::new(
            AccountCode::new("31").unwrap(),
            "Créances sur la clientèle".into(),
            AccountClass::Class3,
            AccountType::Asset,
            Some("NCT-24".into()),
            None,
        )
        .unwrap();

        assert_eq!(coa.label(), "Créances sur la clientèle");
        assert_eq!(coa.account_class(), AccountClass::Class3);
        assert!(coa.is_active());
    }

    #[test]
    fn test_chart_of_accounts_class_mismatch() {
        // Code 31 implies Class 3, but specifying Class 1 should fail
        let result = ChartOfAccounts::new(
            AccountCode::new("31").unwrap(),
            "Bad class".into(),
            AccountClass::Class1,
            AccountType::Asset,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_chart_of_accounts_empty_label() {
        let result = ChartOfAccounts::new(
            AccountCode::new("31").unwrap(),
            "".into(),
            AccountClass::Class3,
            AccountType::Asset,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_chart_of_accounts_deactivate() {
        let mut coa = ChartOfAccounts::new(
            AccountCode::new("31").unwrap(),
            "Test".into(),
            AccountClass::Class3,
            AccountType::Asset,
            None,
            None,
        )
        .unwrap();

        assert!(coa.is_active());
        coa.deactivate();
        assert!(!coa.is_active());
    }

    // --- PeriodClosing ---

    #[test]
    fn test_period_closing_new_daily() {
        let pc = PeriodClosing::new("2026-04-07".into(), PeriodType::Daily).unwrap();
        assert_eq!(pc.period_type(), PeriodType::Daily);
        assert_eq!(pc.status(), ClosingStatus::Open);
        assert!(pc.is_balanced());
    }

    #[test]
    fn test_period_closing_new_monthly() {
        let pc = PeriodClosing::new("2026-04".into(), PeriodType::Monthly).unwrap();
        assert_eq!(pc.period_type(), PeriodType::Monthly);
    }

    #[test]
    fn test_period_closing_new_annual() {
        let pc = PeriodClosing::new("2026".into(), PeriodType::Annual).unwrap();
        assert_eq!(pc.period_type(), PeriodType::Annual);
    }

    #[test]
    fn test_period_closing_empty_period() {
        let result = PeriodClosing::new("".into(), PeriodType::Daily);
        assert!(result.is_err());
    }

    #[test]
    fn test_period_closing_state_transitions() {
        let mut pc = PeriodClosing::new("2026-04".into(), PeriodType::Monthly).unwrap();

        assert_eq!(pc.status(), ClosingStatus::Open);

        pc.start_closing().unwrap();
        assert_eq!(pc.status(), ClosingStatus::InProgress);

        pc.complete_closing(5000, 5000, 10).unwrap();
        assert_eq!(pc.status(), ClosingStatus::Closed);
        assert!(pc.is_balanced());

        pc.archive().unwrap();
        assert_eq!(pc.status(), ClosingStatus::Archived);
    }

    #[test]
    fn test_period_closing_invalid_transition() {
        let mut pc = PeriodClosing::new("2026-04".into(), PeriodType::Monthly).unwrap();
        let result = pc.complete_closing(5000, 5000, 10);
        assert!(matches!(result, Err(DomainError::InvalidJournalEntry(_))));
    }

    #[test]
    fn test_period_closing_variance_calculation() {
        let mut pc = PeriodClosing::new("2026-04".into(), PeriodType::Monthly).unwrap();
        pc.start_closing().unwrap();
        pc.complete_closing(10000, 9500, 20).unwrap();

        assert_eq!(pc.variance(), 500);
        assert!(!pc.is_balanced());
    }

    #[test]
    fn test_period_type_roundtrip() {
        for pt in [PeriodType::Daily, PeriodType::Monthly, PeriodType::Annual] {
            assert_eq!(PeriodType::from_str_value(pt.as_str()).unwrap(), pt);
        }
    }

    #[test]
    fn test_closing_status_roundtrip() {
        for cs in [
            ClosingStatus::Open,
            ClosingStatus::InProgress,
            ClosingStatus::Closed,
            ClosingStatus::Archived,
        ] {
            assert_eq!(ClosingStatus::from_str_value(cs.as_str()).unwrap(), cs);
        }
    }

    // --- DualPosting ---

    #[test]
    fn test_dual_posting_nct_only() {
        let lines = vec![make_line("31", 1000, 0), make_line("42", 0, 1000)];
        let entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 7).unwrap(),
            "Test dual posting".into(),
            lines,
        )
        .unwrap();

        let entry_id = entry.entry_id().clone();
        let dual = DualPosting::new(entry_id.clone(), entry, None);

        assert_eq!(dual.posting_engines().len(), 1);
        assert_eq!(dual.posting_engines()[0], PostingEngine::NCT);
        assert!(!dual.has_dual_posting());
    }

    #[test]
    fn test_dual_posting_with_ifrs9() {
        let lines1 = vec![make_line("31", 1000, 0), make_line("42", 0, 1000)];
        let entry1 = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 7).unwrap(),
            "NCT posting".into(),
            lines1,
        )
        .unwrap();

        let lines2 = vec![make_line("31", 950, 0), make_line("42", 0, 950), make_line("69", 50, 0), make_line("39", 0, 50)];
        let entry2 = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 7).unwrap(),
            "IFRS 9 posting".into(),
            lines2,
        )
        .unwrap();

        let entry_id = entry1.entry_id().clone();
        let dual = DualPosting::new(entry_id.clone(), entry1, Some(entry2));

        assert_eq!(dual.posting_engines().len(), 2);
        assert!(dual.posting_engines().contains(&PostingEngine::NCT));
        assert!(dual.posting_engines().contains(&PostingEngine::IFRS9));
        assert!(dual.has_dual_posting());
    }

    // --- ECL 12m vs Lifetime ---

    #[test]
    fn test_ecl_12m_vs_lifetime() {
        let ecl = ExpectedCreditLoss::new(
            Uuid::new_v4(),
            EclStage::Stage1,
            0.02,      // 2% PD (12m)
            0.45,      // 45% LGD
            1_000_000, // 1M EAD
        );

        let ecl_12m = ecl.ecl_amount_12m();
        let ecl_lifetime = ecl.ecl_amount_lifetime();

        // Lifetime ECL should be higher than 12m
        assert!(ecl_lifetime > ecl_12m);
        assert_eq!(ecl_12m, 9000); // 0.02 * 0.45 * 1M
        assert!(ecl_lifetime > 9000);
    }

    #[test]
    fn test_ecl_stage2_higher_pd_lifetime() {
        let ecl = ExpectedCreditLoss::new(
            Uuid::new_v4(),
            EclStage::Stage2,
            0.05,
            0.45,
            1_000_000,
        );

        let pd_12m = ecl.probability_of_default_12m();
        let pd_lifetime = ecl.probability_of_default_lifetime();

        assert_eq!(pd_12m, 0.05);
        assert!(pd_lifetime > 0.05);
    }

    #[test]
    fn test_ecl_stage3_pd_100_percent() {
        let ecl = ExpectedCreditLoss::new(
            Uuid::new_v4(),
            EclStage::Stage3,
            1.0,
            0.45,
            1_000_000,
        );

        assert_eq!(ecl.probability_of_default_lifetime(), 1.0);
    }
}
