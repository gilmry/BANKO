use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

use banko_domain::accounting::{ClosingStatus, PeriodClosing, PeriodType};

use super::errors::AccountingServiceError;
use super::ports::IJournalRepository;

/// Port for persisting period closings (FR-093/094/095)
#[async_trait::async_trait]
pub trait IPeriodClosingRepository: Send + Sync {
    async fn save(&self, closing: &PeriodClosing) -> Result<(), String>;
    async fn find_by_period(&self, period: &str) -> Result<Option<PeriodClosing>, String>;
    async fn find_by_type(&self, period_type: PeriodType) -> Result<Vec<PeriodClosing>, String>;
    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<PeriodClosing>, String>;
}

/// Service for period closings: daily (FR-093), monthly (FR-094), annual (FR-095)
pub struct PeriodClosingService {
    period_repo: Arc<dyn IPeriodClosingRepository>,
    journal_repo: Arc<dyn IJournalRepository>,
}

impl PeriodClosingService {
    pub fn new(
        period_repo: Arc<dyn IPeriodClosingRepository>,
        journal_repo: Arc<dyn IJournalRepository>,
    ) -> Self {
        PeriodClosingService {
            period_repo,
            journal_repo,
        }
    }

    /// Close daily period (FR-093: Clôture journalière)
    pub async fn close_daily(
        &self,
        date: NaiveDate,
    ) -> Result<PeriodClosing, AccountingServiceError> {
        let period = date.format("%Y-%m-%d").to_string();

        // Create period closing record
        let mut closing = PeriodClosing::new(period.clone(), PeriodType::Daily)
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        closing
            .start_closing()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        // Get entries for the day
        let entries = self
            .journal_repo
            .find_by_period(date, date)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let total_debits: i64 = entries.iter().map(|e| e.total_debit()).sum();
        let total_credits: i64 = entries.iter().map(|e| e.total_credit()).sum();
        let entries_count = entries.len() as i64;

        closing
            .complete_closing(total_debits, total_credits, entries_count)
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.period_repo
            .save(&closing)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(closing)
    }

    /// Close monthly period (FR-094: Clôture mensuelle)
    pub async fn close_monthly(
        &self,
        year: i32,
        month: u32,
    ) -> Result<PeriodClosing, AccountingServiceError> {
        if month < 1 || month > 12 {
            return Err(AccountingServiceError::InvalidEntry(
                "Month must be between 1 and 12".to_string(),
            ));
        }

        let period = format!("{:04}-{:02}", year, month);

        // Create period closing record
        let mut closing = PeriodClosing::new(period.clone(), PeriodType::Monthly)
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        closing
            .start_closing()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        // Get entries for the month
        let start = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| AccountingServiceError::InvalidEntry("Invalid date".to_string()))?;

        let end = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
                .ok_or_else(|| AccountingServiceError::InvalidEntry("Invalid date".to_string()))?
                - chrono::Duration::days(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
                .ok_or_else(|| AccountingServiceError::InvalidEntry("Invalid date".to_string()))?
                - chrono::Duration::days(1)
        };

        let entries = self
            .journal_repo
            .find_by_period(start, end)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let total_debits: i64 = entries.iter().map(|e| e.total_debit()).sum();
        let total_credits: i64 = entries.iter().map(|e| e.total_credit()).sum();
        let entries_count = entries.len() as i64;

        closing
            .complete_closing(total_debits, total_credits, entries_count)
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.period_repo
            .save(&closing)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(closing)
    }

    /// Close annual period (FR-095: Clôture annuelle - arrêté des comptes)
    pub async fn close_annual(&self, year: i32) -> Result<PeriodClosing, AccountingServiceError> {
        let period = format!("{:04}", year);

        // Create period closing record
        let mut closing = PeriodClosing::new(period.clone(), PeriodType::Annual)
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        closing
            .start_closing()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        // Get entries for the year
        let start = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| AccountingServiceError::InvalidEntry("Invalid date".to_string()))?;

        let end = NaiveDate::from_ymd_opt(year, 12, 31)
            .ok_or_else(|| AccountingServiceError::InvalidEntry("Invalid date".to_string()))?;

        let entries = self
            .journal_repo
            .find_by_period(start, end)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let total_debits: i64 = entries.iter().map(|e| e.total_debit()).sum();
        let total_credits: i64 = entries.iter().map(|e| e.total_credit()).sum();
        let entries_count = entries.len() as i64;

        closing
            .complete_closing(total_debits, total_credits, entries_count)
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.period_repo
            .save(&closing)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(closing)
    }

    /// Get period closing status
    pub async fn get_closing(
        &self,
        period: &str,
    ) -> Result<Option<PeriodClosing>, AccountingServiceError> {
        self.period_repo
            .find_by_period(period)
            .await
            .map_err(AccountingServiceError::Internal)
    }

    /// Archive a closed period
    pub async fn archive_period(
        &self,
        period: &str,
    ) -> Result<(), AccountingServiceError> {
        let mut closing = self
            .period_repo
            .find_by_period(period)
            .await
            .map_err(AccountingServiceError::Internal)?
            .ok_or(AccountingServiceError::EntryNotFound)?;

        closing
            .archive()
            .map_err(|e| AccountingServiceError::DomainError(e.to_string()))?;

        self.period_repo
            .save(&closing)
            .await
            .map_err(AccountingServiceError::Internal)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockPeriodClosingRepository {
        closings: Mutex<Vec<PeriodClosing>>,
    }

    impl MockPeriodClosingRepository {
        fn new() -> Self {
            MockPeriodClosingRepository {
                closings: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IPeriodClosingRepository for MockPeriodClosingRepository {
        async fn save(&self, closing: &PeriodClosing) -> Result<(), String> {
            let mut closings = self.closings.lock().unwrap();
            closings.retain(|c| c.period() != closing.period());
            closings.push(closing.clone());
            Ok(())
        }

        async fn find_by_period(&self, period: &str) -> Result<Option<PeriodClosing>, String> {
            let closings = self.closings.lock().unwrap();
            Ok(closings.iter().find(|c| c.period() == period).cloned())
        }

        async fn find_by_type(
            &self,
            period_type: PeriodType,
        ) -> Result<Vec<PeriodClosing>, String> {
            let closings = self.closings.lock().unwrap();
            Ok(closings
                .iter()
                .filter(|c| c.period_type() == period_type)
                .cloned()
                .collect())
        }

        async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<PeriodClosing>, String> {
            let closings = self.closings.lock().unwrap();
            Ok(closings
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
    }

    struct MockJournalRepositoryForClosing {
        entries: Mutex<Vec<banko_domain::accounting::JournalEntry>>,
    }

    impl MockJournalRepositoryForClosing {
        fn new() -> Self {
            MockJournalRepositoryForClosing {
                entries: Mutex::new(Vec::new()),
            }
        }

        fn add_entry(&self, entry: banko_domain::accounting::JournalEntry) {
            let mut entries = self.entries.lock().unwrap();
            entries.push(entry);
        }
    }

    #[async_trait]
    impl IJournalRepository for MockJournalRepositoryForClosing {
        async fn save(&self, _entry: &banko_domain::accounting::JournalEntry) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(
            &self,
            _id: &banko_domain::accounting::EntryId,
        ) -> Result<Option<banko_domain::accounting::JournalEntry>, String> {
            Ok(None)
        }
        async fn find_by_period(
            &self,
            start: NaiveDate,
            end: NaiveDate,
        ) -> Result<Vec<banko_domain::accounting::JournalEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| e.entry_date() >= start && e.entry_date() <= end)
                .cloned()
                .collect())
        }
        async fn find_by_account(
            &self,
            _code: &banko_domain::accounting::AccountCode,
            _start: NaiveDate,
            _end: NaiveDate,
        ) -> Result<Vec<banko_domain::accounting::JournalEntry>, String> {
            Ok(vec![])
        }
        async fn find_all(&self, _offset: i64, _limit: i64) -> Result<Vec<banko_domain::accounting::JournalEntry>, String> {
            Ok(vec![])
        }
        async fn count_all(&self) -> Result<i64, String> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_close_daily() {
        let period_repo = Arc::new(MockPeriodClosingRepository::new());
        let journal_repo = Arc::new(MockJournalRepositoryForClosing::new());
        let service = PeriodClosingService::new(period_repo, journal_repo);

        let date = NaiveDate::from_ymd_opt(2026, 4, 7).unwrap();
        let result = service.close_daily(date).await.unwrap();

        assert_eq!(result.period_type(), PeriodType::Daily);
        assert_eq!(result.status(), ClosingStatus::Closed);
        assert!(result.is_balanced());
    }

    #[tokio::test]
    async fn test_close_monthly() {
        let period_repo = Arc::new(MockPeriodClosingRepository::new());
        let journal_repo = Arc::new(MockJournalRepositoryForClosing::new());
        let service = PeriodClosingService::new(period_repo, journal_repo);

        let result = service.close_monthly(2026, 4).await.unwrap();

        assert_eq!(result.period_type(), PeriodType::Monthly);
        assert_eq!(result.status(), ClosingStatus::Closed);
    }

    #[tokio::test]
    async fn test_close_annual() {
        let period_repo = Arc::new(MockPeriodClosingRepository::new());
        let journal_repo = Arc::new(MockJournalRepositoryForClosing::new());
        let service = PeriodClosingService::new(period_repo, journal_repo);

        let result = service.close_annual(2026).await.unwrap();

        assert_eq!(result.period_type(), PeriodType::Annual);
        assert_eq!(result.status(), ClosingStatus::Closed);
    }

    #[tokio::test]
    async fn test_invalid_month() {
        let period_repo = Arc::new(MockPeriodClosingRepository::new());
        let journal_repo = Arc::new(MockJournalRepositoryForClosing::new());
        let service = PeriodClosingService::new(period_repo, journal_repo);

        let result = service.close_monthly(2026, 13).await;
        assert!(result.is_err());
    }
}
