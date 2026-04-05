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

// --- ExpectedCreditLoss (IFRS 9 preparation) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedCreditLoss {
    loan_id: Uuid,
    stage: EclStage,
    probability_of_default: f64,
    loss_given_default: f64,
    exposure_at_default: i64,
    ecl_amount: i64,
    calculated_at: DateTime<Utc>,
}

impl ExpectedCreditLoss {
    pub fn new(
        loan_id: Uuid,
        stage: EclStage,
        probability_of_default: f64,
        loss_given_default: f64,
        exposure_at_default: i64,
    ) -> Self {
        let ecl_amount =
            (probability_of_default * loss_given_default * exposure_at_default as f64) as i64;
        ExpectedCreditLoss {
            loan_id,
            stage,
            probability_of_default,
            loss_given_default,
            exposure_at_default,
            ecl_amount,
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
        self.probability_of_default
    }
    pub fn loss_given_default(&self) -> f64 {
        self.loss_given_default
    }
    pub fn exposure_at_default(&self) -> i64 {
        self.exposure_at_default
    }
    pub fn ecl_amount(&self) -> i64 {
        self.ecl_amount
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
}
