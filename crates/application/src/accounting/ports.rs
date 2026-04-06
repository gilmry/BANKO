use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use banko_domain::accounting::{AccountCode, EntryId, ExpectedCreditLoss, JournalEntry, LedgerAccount, FeeDefinition, FeeCharge, FeeGrid};

#[async_trait]
pub trait IJournalRepository: Send + Sync {
    async fn save(&self, entry: &JournalEntry) -> Result<(), String>;
    async fn find_by_id(&self, id: &EntryId) -> Result<Option<JournalEntry>, String>;
    async fn find_by_period(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<JournalEntry>, String>;
    async fn find_by_account(
        &self,
        code: &AccountCode,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<JournalEntry>, String>;
    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<JournalEntry>, String>;
    async fn count_all(&self) -> Result<i64, String>;
}

#[async_trait]
pub trait ILedgerRepository: Send + Sync {
    /// Returns (total_debit, total_credit) for an account up to a date
    async fn get_account_balance(
        &self,
        code: &AccountCode,
        as_of: NaiveDate,
    ) -> Result<(i64, i64), String>;

    /// Returns all account balances as (code, label, type, debit, credit)
    async fn get_all_balances(&self, as_of: NaiveDate) -> Result<Vec<AccountBalanceRow>, String>;

    async fn save_chart_entry(&self, entry: &LedgerAccount) -> Result<(), String>;
    async fn find_chart_entry(&self, code: &AccountCode) -> Result<Option<LedgerAccount>, String>;
    async fn find_all_chart_entries(&self) -> Result<Vec<LedgerAccount>, String>;
}

#[derive(Debug, Clone)]
pub struct AccountBalanceRow {
    pub code: String,
    pub label: String,
    pub account_type: String,
    pub total_debit: i64,
    pub total_credit: i64,
}

#[async_trait]
pub trait IPeriodRepository: Send + Sync {
    async fn close_period(&self, period: &str) -> Result<(), String>;
    async fn is_closed(&self, period: &str) -> Result<bool, String>;
    async fn find_closed_periods(&self) -> Result<Vec<String>, String>;
}

#[async_trait]
pub trait IEclRepository: Send + Sync {
    /// Save an ECL calculation record
    async fn save(&self, ecl: &ExpectedCreditLoss) -> Result<(), String>;

    /// Find ECL by loan ID
    async fn find_by_loan_id(&self, loan_id: Uuid) -> Result<Option<ExpectedCreditLoss>, String>;

    /// Find all ECL calculations for a period
    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<ExpectedCreditLoss>, String>;

    /// Count total ECL records
    async fn count_all(&self) -> Result<i64, String>;
}

// ==================== Fee Ports ====================

#[async_trait]
pub trait IFeeDefinitionRepository: Send + Sync {
    async fn save(&self, definition: &FeeDefinition) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<FeeDefinition>, String>;
    async fn list_by_product(&self, product_id: Uuid) -> Result<Vec<FeeDefinition>, String>;
    async fn list_all(&self) -> Result<Vec<FeeDefinition>, String>;
}

#[async_trait]
pub trait IFeeChargeRepository: Send + Sync {
    async fn save(&self, charge: &FeeCharge) -> Result<(), String>;
    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<FeeCharge>, String>;
    async fn find_pending(&self, account_id: Uuid) -> Result<Vec<FeeCharge>, String>;
    async fn update_status(&self, charge: &FeeCharge) -> Result<(), String>;
}

#[async_trait]
pub trait IFeeGridRepository: Send + Sync {
    async fn save(&self, grid: &FeeGrid) -> Result<(), String>;
    async fn find_by_segment(&self, segment: &str) -> Result<Option<FeeGrid>, String>;
    async fn find_active_for_segment(
        &self,
        segment: &str,
        date: DateTime<Utc>,
    ) -> Result<Option<FeeGrid>, String>;
    async fn list_all(&self) -> Result<Vec<FeeGrid>, String>;
}

// ==================== Interest Accrual Ports ====================

// Ports for interest accrual and reconciliation are defined in their respective service modules
// to avoid circular dependencies and maintain clear architectural boundaries.
