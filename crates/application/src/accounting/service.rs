use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use banko_domain::accounting::*;

use super::dto::*;
use super::errors::AccountingServiceError;
use super::ports::{IJournalRepository, ILedgerRepository, IPeriodRepository};

// --- AccountingService (STORY-ACC-02) ---

pub struct AccountingService {
    journal_repo: Arc<dyn IJournalRepository>,
}

impl AccountingService {
    pub fn new(journal_repo: Arc<dyn IJournalRepository>) -> Self {
        AccountingService { journal_repo }
    }

    pub async fn post_entry(
        &self,
        request: CreateEntryRequest,
    ) -> Result<JournalEntryResponse, AccountingServiceError> {
        let journal_code = JournalCode::from_str_value(&request.journal_code)
            .map_err(|e| AccountingServiceError::InvalidEntry(e.to_string()))?;

        let lines: Vec<JournalLine> = request
            .lines
            .into_iter()
            .map(|l| {
                let code = AccountCode::new(&l.account_code)
                    .map_err(|e| AccountingServiceError::InvalidEntry(e.to_string()))?;
                JournalLine::new(code, l.debit, l.credit, l.description)
                    .map_err(|e| AccountingServiceError::InvalidEntry(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut entry =
            JournalEntry::new(journal_code, request.entry_date, request.description, lines)
                .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        entry
            .post()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.journal_repo
            .save(&entry)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(entry_to_response(&entry))
    }

    pub async fn reverse_entry(
        &self,
        entry_id: Uuid,
    ) -> Result<JournalEntryResponse, AccountingServiceError> {
        let id = EntryId::from_uuid(entry_id);
        let mut original = self
            .journal_repo
            .find_by_id(&id)
            .await
            .map_err(AccountingServiceError::Internal)?
            .ok_or(AccountingServiceError::EntryNotFound)?;

        let mut reversal = original
            .create_reversal()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        reversal
            .post()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        original
            .mark_reversed()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.journal_repo
            .save(&original)
            .await
            .map_err(AccountingServiceError::Internal)?;

        self.journal_repo
            .save(&reversal)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(entry_to_response(&reversal))
    }

    pub async fn get_entry(
        &self,
        entry_id: Uuid,
    ) -> Result<JournalEntryResponse, AccountingServiceError> {
        let id = EntryId::from_uuid(entry_id);
        let entry = self
            .journal_repo
            .find_by_id(&id)
            .await
            .map_err(AccountingServiceError::Internal)?
            .ok_or(AccountingServiceError::EntryNotFound)?;

        Ok(entry_to_response(&entry))
    }

    pub async fn list_entries(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<JournalEntryListResponse, AccountingServiceError> {
        let offset = (page - 1) * limit;
        let entries = self
            .journal_repo
            .find_all(offset, limit)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let total = self
            .journal_repo
            .count_all()
            .await
            .map_err(AccountingServiceError::Internal)?;

        let data = entries.iter().map(entry_to_response).collect();

        Ok(JournalEntryListResponse {
            data,
            total,
            page,
            limit,
        })
    }
}

// --- AutoEntryService (STORY-ACC-05) ---

pub struct AutoEntryService {
    journal_repo: Arc<dyn IJournalRepository>,
}

impl AutoEntryService {
    pub fn new(journal_repo: Arc<dyn IJournalRepository>) -> Self {
        AutoEntryService { journal_repo }
    }

    pub async fn on_account_opened(
        &self,
        account_id: Uuid,
        initial_deposit: i64,
    ) -> Result<JournalEntryResponse, AccountingServiceError> {
        if initial_deposit <= 0 {
            return Err(AccountingServiceError::InvalidEntry(
                "Initial deposit must be positive".into(),
            ));
        }

        let lines = vec![
            JournalLine::new(
                AccountCode::new("51").unwrap(),
                initial_deposit,
                0,
                Some("Cash received".into()),
            )
            .unwrap(),
            JournalLine::new(
                AccountCode::new("42").unwrap(),
                0,
                initial_deposit,
                Some("Client deposit".into()),
            )
            .unwrap(),
        ];

        let mut entry = JournalEntry::new(
            JournalCode::CP,
            chrono::Utc::now().date_naive(),
            format!("Account opening deposit - {account_id}"),
            lines,
        )
        .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        entry
            .post()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.journal_repo
            .save(&entry)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(entry_to_response(&entry))
    }

    pub async fn on_loan_disbursed(
        &self,
        loan_id: Uuid,
        amount: i64,
    ) -> Result<JournalEntryResponse, AccountingServiceError> {
        if amount <= 0 {
            return Err(AccountingServiceError::InvalidEntry(
                "Loan amount must be positive".into(),
            ));
        }

        let lines = vec![
            JournalLine::new(
                AccountCode::new("31").unwrap(),
                amount,
                0,
                Some("Loan disbursement".into()),
            )
            .unwrap(),
            JournalLine::new(
                AccountCode::new("51").unwrap(),
                0,
                amount,
                Some("Cash disbursed".into()),
            )
            .unwrap(),
        ];

        let mut entry = JournalEntry::new(
            JournalCode::CP,
            chrono::Utc::now().date_naive(),
            format!("Loan disbursement - {loan_id}"),
            lines,
        )
        .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        entry
            .post()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.journal_repo
            .save(&entry)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(entry_to_response(&entry))
    }

    pub async fn on_interest_calculated(
        &self,
        account_id: Uuid,
        interest: i64,
    ) -> Result<JournalEntryResponse, AccountingServiceError> {
        if interest <= 0 {
            return Err(AccountingServiceError::InvalidEntry(
                "Interest must be positive".into(),
            ));
        }

        let lines = vec![
            JournalLine::new(
                AccountCode::new("31").unwrap(),
                interest,
                0,
                Some("Interest accrual".into()),
            )
            .unwrap(),
            JournalLine::new(
                AccountCode::new("71").unwrap(),
                0,
                interest,
                Some("Interest revenue".into()),
            )
            .unwrap(),
        ];

        let mut entry = JournalEntry::new(
            JournalCode::IN,
            chrono::Utc::now().date_naive(),
            format!("Interest calculation - {account_id}"),
            lines,
        )
        .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        entry
            .post()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.journal_repo
            .save(&entry)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(entry_to_response(&entry))
    }
}

// --- LedgerService (STORY-ACC-06) ---

pub struct LedgerService {
    journal_repo: Arc<dyn IJournalRepository>,
    ledger_repo: Arc<dyn ILedgerRepository>,
}

impl LedgerService {
    pub fn new(
        journal_repo: Arc<dyn IJournalRepository>,
        ledger_repo: Arc<dyn ILedgerRepository>,
    ) -> Self {
        LedgerService {
            journal_repo,
            ledger_repo,
        }
    }

    pub async fn get_general_ledger(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<GeneralLedgerResponse, AccountingServiceError> {
        let entries = self
            .journal_repo
            .find_by_period(from, to)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let data = entries.iter().map(entry_to_response).collect();

        Ok(GeneralLedgerResponse {
            from,
            to,
            entries: data,
        })
    }

    pub async fn get_trial_balance(
        &self,
        as_of: NaiveDate,
    ) -> Result<TrialBalanceResponse, AccountingServiceError> {
        let balances = self
            .ledger_repo
            .get_all_balances(as_of)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let lines: Vec<TrialBalanceLineResponse> = balances
            .iter()
            .map(|b| TrialBalanceLineResponse {
                account_code: b.code.clone(),
                label: b.label.clone(),
                account_type: b.account_type.clone(),
                debit: b.total_debit,
                credit: b.total_credit,
            })
            .collect();

        let total_debit: i64 = lines.iter().map(|l| l.debit).sum();
        let total_credit: i64 = lines.iter().map(|l| l.credit).sum();

        Ok(TrialBalanceResponse {
            as_of,
            lines,
            total_debit,
            total_credit,
            is_balanced: total_debit == total_credit,
        })
    }
}

// --- PeriodService (STORY-ACC-07) ---

pub struct PeriodService {
    period_repo: Arc<dyn IPeriodRepository>,
}

impl PeriodService {
    pub fn new(period_repo: Arc<dyn IPeriodRepository>) -> Self {
        PeriodService { period_repo }
    }

    pub async fn close_period(
        &self,
        period: &str,
    ) -> Result<PeriodClosingResponse, AccountingServiceError> {
        let is_closed = self
            .period_repo
            .is_closed(period)
            .await
            .map_err(AccountingServiceError::Internal)?;

        if is_closed {
            return Err(AccountingServiceError::PeriodAlreadyClosed(
                period.to_string(),
            ));
        }

        self.period_repo
            .close_period(period)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(PeriodClosingResponse {
            period: period.to_string(),
            status: "Closed".into(),
            message: format!("Period {period} closed successfully"),
        })
    }

    pub async fn is_period_closed(&self, period: &str) -> Result<bool, AccountingServiceError> {
        self.period_repo
            .is_closed(period)
            .await
            .map_err(AccountingServiceError::Internal)
    }
}

// --- EclService (STORY-ACC-08) ---

pub struct EclService;

impl EclService {
    pub fn new() -> Self {
        EclService
    }

    /// Classify a loan into ECL stages based on days past due
    pub fn classify_stage(days_past_due: i64) -> EclStage {
        if days_past_due <= 30 {
            EclStage::Stage1
        } else if days_past_due <= 90 {
            EclStage::Stage2
        } else {
            EclStage::Stage3
        }
    }

    /// Calculate ECL for a loan
    pub fn calculate_ecl(
        loan_id: Uuid,
        days_past_due: i64,
        exposure_at_default: i64,
    ) -> ExpectedCreditLoss {
        let stage = Self::classify_stage(days_past_due);
        let (pd, lgd) = match stage {
            EclStage::Stage1 => (0.02, 0.45),
            EclStage::Stage2 => (0.10, 0.55),
            EclStage::Stage3 => (0.50, 0.70),
        };
        ExpectedCreditLoss::new(loan_id, stage, pd, lgd, exposure_at_default)
    }
}

impl Default for EclService {
    fn default() -> Self {
        Self::new()
    }
}

// --- Helper ---

fn entry_to_response(entry: &JournalEntry) -> JournalEntryResponse {
    JournalEntryResponse {
        id: entry.entry_id().to_string(),
        journal_code: entry.journal_code().as_str().to_string(),
        entry_date: entry.entry_date(),
        description: entry.description().to_string(),
        status: entry.status().as_str().to_string(),
        reversal_of: entry.reversal_of().map(|id| id.to_string()),
        total_debit: entry.total_debit(),
        total_credit: entry.total_credit(),
        lines: entry
            .lines()
            .iter()
            .map(|l| JournalLineResponse {
                line_id: l.line_id().to_string(),
                account_code: l.account_code().as_str().to_string(),
                debit: l.debit(),
                credit: l.credit(),
                description: l.description().map(String::from),
            })
            .collect(),
        created_at: entry.created_at(),
        posted_at: entry.posted_at(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounting::ports::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockJournalRepo {
        entries: Mutex<Vec<JournalEntry>>,
    }

    impl MockJournalRepo {
        fn new() -> Self {
            MockJournalRepo {
                entries: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IJournalRepository for MockJournalRepo {
        async fn save(&self, entry: &JournalEntry) -> Result<(), String> {
            let mut entries = self.entries.lock().unwrap();
            entries.retain(|e| e.entry_id() != entry.entry_id());
            entries.push(entry.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &EntryId) -> Result<Option<JournalEntry>, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .find(|e| e.entry_id() == id)
                .cloned())
        }
        async fn find_by_period(
            &self,
            start: NaiveDate,
            end: NaiveDate,
        ) -> Result<Vec<JournalEntry>, String> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .filter(|e| e.entry_date() >= start && e.entry_date() <= end)
                .cloned()
                .collect())
        }
        async fn find_by_account(
            &self,
            _code: &AccountCode,
            _start: NaiveDate,
            _end: NaiveDate,
        ) -> Result<Vec<JournalEntry>, String> {
            Ok(vec![])
        }
        async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<JournalEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self) -> Result<i64, String> {
            Ok(self.entries.lock().unwrap().len() as i64)
        }
    }

    struct MockPeriodRepo {
        closed: Mutex<Vec<String>>,
    }

    impl MockPeriodRepo {
        fn new() -> Self {
            MockPeriodRepo {
                closed: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IPeriodRepository for MockPeriodRepo {
        async fn close_period(&self, period: &str) -> Result<(), String> {
            self.closed.lock().unwrap().push(period.to_string());
            Ok(())
        }
        async fn is_closed(&self, period: &str) -> Result<bool, String> {
            Ok(self.closed.lock().unwrap().contains(&period.to_string()))
        }
        async fn find_closed_periods(&self) -> Result<Vec<String>, String> {
            Ok(self.closed.lock().unwrap().clone())
        }
    }

    #[tokio::test]
    async fn test_post_balanced_entry() {
        let service = AccountingService::new(Arc::new(MockJournalRepo::new()));

        let result = service
            .post_entry(CreateEntryRequest {
                journal_code: "OD".into(),
                entry_date: NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
                description: "Test balanced".into(),
                lines: vec![
                    CreateEntryLineRequest {
                        account_code: "31".into(),
                        debit: 1000,
                        credit: 0,
                        description: None,
                    },
                    CreateEntryLineRequest {
                        account_code: "42".into(),
                        debit: 0,
                        credit: 1000,
                        description: None,
                    },
                ],
            })
            .await
            .unwrap();

        assert_eq!(result.status, "Posted");
        assert_eq!(result.total_debit, 1000);
        assert_eq!(result.total_credit, 1000);
    }

    #[tokio::test]
    async fn test_post_unbalanced_entry_rejected() {
        let service = AccountingService::new(Arc::new(MockJournalRepo::new()));

        let result = service
            .post_entry(CreateEntryRequest {
                journal_code: "OD".into(),
                entry_date: NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
                description: "Unbalanced".into(),
                lines: vec![
                    CreateEntryLineRequest {
                        account_code: "31".into(),
                        debit: 1000,
                        credit: 0,
                        description: None,
                    },
                    CreateEntryLineRequest {
                        account_code: "42".into(),
                        debit: 0,
                        credit: 999,
                        description: None,
                    },
                ],
            })
            .await;

        assert!(matches!(
            result,
            Err(AccountingServiceError::DomainError(_))
        ));
    }

    #[tokio::test]
    async fn test_reverse_entry() {
        let repo = Arc::new(MockJournalRepo::new());
        let service = AccountingService::new(repo.clone());

        let posted = service
            .post_entry(CreateEntryRequest {
                journal_code: "OD".into(),
                entry_date: NaiveDate::from_ymd_opt(2026, 4, 5).unwrap(),
                description: "To reverse".into(),
                lines: vec![
                    CreateEntryLineRequest {
                        account_code: "31".into(),
                        debit: 5000,
                        credit: 0,
                        description: None,
                    },
                    CreateEntryLineRequest {
                        account_code: "42".into(),
                        debit: 0,
                        credit: 5000,
                        description: None,
                    },
                ],
            })
            .await
            .unwrap();

        let entry_id = Uuid::parse_str(&posted.id).unwrap();
        let reversal = service.reverse_entry(entry_id).await.unwrap();

        assert_eq!(reversal.status, "Posted");
        assert_eq!(reversal.reversal_of, Some(posted.id));
        assert_eq!(reversal.total_debit, 5000);
        assert_eq!(reversal.total_credit, 5000);
    }

    #[tokio::test]
    async fn test_auto_entry_account_opened() {
        let service = AutoEntryService::new(Arc::new(MockJournalRepo::new()));
        let result = service
            .on_account_opened(Uuid::new_v4(), 50_000)
            .await
            .unwrap();
        assert_eq!(result.status, "Posted");
        assert_eq!(result.total_debit, 50_000);
        assert_eq!(result.total_credit, 50_000);
    }

    #[tokio::test]
    async fn test_auto_entry_loan_disbursed() {
        let service = AutoEntryService::new(Arc::new(MockJournalRepo::new()));
        let result = service
            .on_loan_disbursed(Uuid::new_v4(), 100_000)
            .await
            .unwrap();
        assert_eq!(result.status, "Posted");
        assert_eq!(result.total_debit, 100_000);
    }

    #[tokio::test]
    async fn test_auto_entry_interest() {
        let service = AutoEntryService::new(Arc::new(MockJournalRepo::new()));
        let result = service
            .on_interest_calculated(Uuid::new_v4(), 2500)
            .await
            .unwrap();
        assert_eq!(result.status, "Posted");
    }

    #[tokio::test]
    async fn test_period_close() {
        let service = PeriodService::new(Arc::new(MockPeriodRepo::new()));
        let result = service.close_period("2026-03").await.unwrap();
        assert_eq!(result.status, "Closed");
    }

    #[tokio::test]
    async fn test_period_double_close_rejected() {
        let service = PeriodService::new(Arc::new(MockPeriodRepo::new()));
        service.close_period("2026-03").await.unwrap();
        let result = service.close_period("2026-03").await;
        assert!(matches!(
            result,
            Err(AccountingServiceError::PeriodAlreadyClosed(_))
        ));
    }

    #[test]
    fn test_ecl_stage_classification() {
        assert_eq!(EclService::classify_stage(0), EclStage::Stage1);
        assert_eq!(EclService::classify_stage(30), EclStage::Stage1);
        assert_eq!(EclService::classify_stage(31), EclStage::Stage2);
        assert_eq!(EclService::classify_stage(90), EclStage::Stage2);
        assert_eq!(EclService::classify_stage(91), EclStage::Stage3);
    }

    #[test]
    fn test_ecl_calculation() {
        let ecl = EclService::calculate_ecl(Uuid::new_v4(), 15, 1_000_000);
        assert_eq!(ecl.stage(), EclStage::Stage1);
        assert_eq!(ecl.ecl_amount(), 9000); // 0.02 * 0.45 * 1M = 9000
    }
}
