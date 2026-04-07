use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use banko_domain::accounting::JournalEntry;

use super::errors::AccountingServiceError;
use super::ports::IJournalRepository;

/// FR-097: Export formats for accounting data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    /// BCT Standard Format (Banque Centrale de Tunisie)
    BCT,
    /// XBRL GL (eXtensible Business Reporting Language - GL)
    XBRL,
    /// CSV Format
    CSV,
    /// JSON Format
    JSON,
}

impl ExportFormat {
    pub fn as_str(&self) -> &str {
        match self {
            ExportFormat::BCT => "BCT",
            ExportFormat::XBRL => "XBRL",
            ExportFormat::CSV => "CSV",
            ExportFormat::JSON => "JSON",
        }
    }

    pub fn file_extension(&self) -> &str {
        match self {
            ExportFormat::BCT => ".bct",
            ExportFormat::XBRL => ".xbrl",
            ExportFormat::CSV => ".csv",
            ExportFormat::JSON => ".json",
        }
    }
}

/// Export request options
#[derive(Debug, Clone)]
pub struct ExportRequest {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub format: ExportFormat,
    pub include_reversals: bool,
}

/// Export result with metadata
#[derive(Debug, Clone)]
pub struct ExportResult {
    pub format: ExportFormat,
    pub entries_count: usize,
    pub period_from: NaiveDate,
    pub period_to: NaiveDate,
    pub content: String,
}

/// Service for exporting accounting data in various formats (FR-097)
pub struct ExportService {
    journal_repo: Arc<dyn IJournalRepository>,
}

impl ExportService {
    pub fn new(journal_repo: Arc<dyn IJournalRepository>) -> Self {
        ExportService { journal_repo }
    }

    /// Export journal entries in the specified format
    pub async fn export(
        &self,
        request: ExportRequest,
    ) -> Result<ExportResult, AccountingServiceError> {
        // Fetch entries for the period
        let entries = self
            .journal_repo
            .find_by_period(request.from, request.to)
            .await
            .map_err(AccountingServiceError::Internal)?;

        let entries_count = entries.len();

        let content = match request.format {
            ExportFormat::BCT => self.export_bct(&entries, request.include_reversals),
            ExportFormat::XBRL => self.export_xbrl(&entries),
            ExportFormat::CSV => self.export_csv(&entries),
            ExportFormat::JSON => self.export_json(&entries)?,
        };

        Ok(ExportResult {
            format: request.format,
            entries_count,
            period_from: request.from,
            period_to: request.to,
            content,
        })
    }

    /// Export in BCT standard format
    fn export_bct(&self, entries: &[JournalEntry], include_reversals: bool) -> String {
        let mut lines = vec![
            "BCT_EXPORT".to_string(),
            format!("VERSION:1.0"),
        ];

        for entry in entries {
            if !include_reversals && entry.status().as_str() == "Reversed" {
                continue;
            }

            let line = format!(
                "{}|{}|{}|{}|{}",
                entry.entry_id(),
                entry.journal_code().as_str(),
                entry.entry_date(),
                entry.description(),
                entry.status().as_str()
            );
            lines.push(line);

            for line_item in entry.lines() {
                let detail = format!(
                    "  LINE|{}|{}|{}|{}",
                    line_item.account_code().as_str(),
                    line_item.debit(),
                    line_item.credit(),
                    line_item.description().unwrap_or("")
                );
                lines.push(detail);
            }
        }

        lines.join("\n")
    }

    /// Export in XBRL format
    fn export_xbrl(&self, entries: &[JournalEntry]) -> String {
        let mut lines = vec![
            r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string(),
            r#"<xbrl xmlns="http://www.xbrl.org/2003/instance">"#.to_string(),
        ];

        for entry in entries {
            lines.push(format!(
                r#"  <context id="entry_{}"><period><instant>{}</instant></period></context>"#,
                entry.entry_id(),
                entry.entry_date()
            ));

            for line_item in entry.lines().iter() {
                let value = if line_item.debit() > 0 {
                    line_item.debit()
                } else {
                    -line_item.credit()
                };

                lines.push(format!(
                    r#"  <item contextRef="entry_{}" unitRef="TND" decimals="2" account="{}">{}</item>"#,
                    entry.entry_id(),
                    line_item.account_code().as_str(),
                    value
                ));
            }
        }

        lines.push("</xbrl>".to_string());
        lines.join("\n")
    }

    /// Export in CSV format
    fn export_csv(&self, entries: &[JournalEntry]) -> String {
        let mut lines = vec![
            "EntryID,JournalCode,EntryDate,Description,Status,AccountCode,Debit,Credit,LineDescription".to_string(),
        ];

        for entry in entries {
            for line_item in entry.lines() {
                let line = format!(
                    "{},{},{},{},{},{},{},{},\"{}\"",
                    entry.entry_id(),
                    entry.journal_code().as_str(),
                    entry.entry_date(),
                    entry.description(),
                    entry.status().as_str(),
                    line_item.account_code().as_str(),
                    line_item.debit(),
                    line_item.credit(),
                    line_item.description().unwrap_or("")
                );
                lines.push(line);
            }
        }

        lines.join("\n")
    }

    /// Export in JSON format
    fn export_json(&self, entries: &[JournalEntry]) -> Result<String, AccountingServiceError> {
        let json_entries: Vec<_> = entries
            .iter()
            .map(|e| {
                serde_json::json!({
                    "id": e.entry_id().to_string(),
                    "journal_code": e.journal_code().as_str(),
                    "entry_date": e.entry_date(),
                    "description": e.description(),
                    "status": e.status().as_str(),
                    "lines": e.lines().iter().map(|l| {
                        serde_json::json!({
                            "account_code": l.account_code().as_str(),
                            "debit": l.debit(),
                            "credit": l.credit(),
                            "description": l.description()
                        })
                    }).collect::<Vec<_>>()
                })
            })
            .collect();

        serde_json::to_string_pretty(&json_entries)
            .map_err(|e| AccountingServiceError::Internal(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockJournalRepository {
        entries: Mutex<Vec<JournalEntry>>,
    }

    impl MockJournalRepository {
        fn new() -> Self {
            MockJournalRepository {
                entries: Mutex::new(Vec::new()),
            }
        }

        fn add_entry(&self, entry: JournalEntry) {
            let mut entries = self.entries.lock().unwrap();
            entries.push(entry);
        }
    }

    #[async_trait]
    impl IJournalRepository for MockJournalRepository {
        async fn save(&self, _entry: &JournalEntry) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(
            &self,
            _id: &banko_domain::accounting::EntryId,
        ) -> Result<Option<JournalEntry>, String> {
            Ok(None)
        }
        async fn find_by_period(
            &self,
            start: NaiveDate,
            end: NaiveDate,
        ) -> Result<Vec<JournalEntry>, String> {
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
        ) -> Result<Vec<JournalEntry>, String> {
            Ok(vec![])
        }
        async fn find_all(&self, _offset: i64, _limit: i64) -> Result<Vec<JournalEntry>, String> {
            Ok(vec![])
        }
        async fn count_all(&self) -> Result<i64, String> {
            Ok(0)
        }
    }

    #[test]
    fn test_export_format_properties() {
        assert_eq!(ExportFormat::BCT.as_str(), "BCT");
        assert_eq!(ExportFormat::BCT.file_extension(), ".bct");
        assert_eq!(ExportFormat::XBRL.as_str(), "XBRL");
        assert_eq!(ExportFormat::XBRL.file_extension(), ".xbrl");
        assert_eq!(ExportFormat::CSV.as_str(), "CSV");
        assert_eq!(ExportFormat::CSV.file_extension(), ".csv");
        assert_eq!(ExportFormat::JSON.as_str(), "JSON");
        assert_eq!(ExportFormat::JSON.file_extension(), ".json");
    }

    #[tokio::test]
    async fn test_export_bct_format() {
        use banko_domain::accounting::{AccountCode, JournalCode, JournalLine};

        let repo = Arc::new(MockJournalRepository::new());

        let lines = vec![
            JournalLine::new(
                AccountCode::new("31").unwrap(),
                1000,
                0,
                None,
            )
            .unwrap(),
            JournalLine::new(
                AccountCode::new("42").unwrap(),
                0,
                1000,
                None,
            )
            .unwrap(),
        ];

        let entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 7).unwrap(),
            "Test export".to_string(),
            lines,
        )
        .unwrap();

        repo.add_entry(entry.clone());

        let service = ExportService::new(repo);
        let request = ExportRequest {
            from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
            format: ExportFormat::BCT,
            include_reversals: true,
        };

        let result = service.export(request).await.unwrap();

        assert_eq!(result.format, ExportFormat::BCT);
        assert_eq!(result.entries_count, 1);
        assert!(result.content.contains("BCT_EXPORT"));
        assert!(result.content.contains("31"));
        assert!(result.content.contains("42"));
    }

    #[tokio::test]
    async fn test_export_csv_format() {
        use banko_domain::accounting::{AccountCode, JournalCode, JournalLine};

        let repo = Arc::new(MockJournalRepository::new());

        let lines = vec![
            JournalLine::new(
                AccountCode::new("31").unwrap(),
                500,
                0,
                Some("Test line".to_string()),
            )
            .unwrap(),
            JournalLine::new(
                AccountCode::new("42").unwrap(),
                0,
                500,
                None,
            )
            .unwrap(),
        ];

        let entry = JournalEntry::new(
            JournalCode::CP,
            NaiveDate::from_ymd_opt(2026, 4, 7).unwrap(),
            "CSV Test".to_string(),
            lines,
        )
        .unwrap();

        repo.add_entry(entry);

        let service = ExportService::new(repo);
        let request = ExportRequest {
            from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
            format: ExportFormat::CSV,
            include_reversals: true,
        };

        let result = service.export(request).await.unwrap();

        assert_eq!(result.format, ExportFormat::CSV);
        assert!(result.content.contains("EntryID,JournalCode"));
        assert!(result.content.contains("31"));
        assert!(result.content.contains("42"));
        assert!(result.content.contains("500"));
    }

    #[tokio::test]
    async fn test_export_json_format() {
        use banko_domain::accounting::{AccountCode, JournalCode, JournalLine};

        let repo = Arc::new(MockJournalRepository::new());

        let lines = vec![
            JournalLine::new(
                AccountCode::new("31").unwrap(),
                250,
                0,
                None,
            )
            .unwrap(),
            JournalLine::new(
                AccountCode::new("42").unwrap(),
                0,
                250,
                None,
            )
            .unwrap(),
        ];

        let entry = JournalEntry::new(
            JournalCode::OD,
            NaiveDate::from_ymd_opt(2026, 4, 7).unwrap(),
            "JSON test".to_string(),
            lines,
        )
        .unwrap();

        repo.add_entry(entry);

        let service = ExportService::new(repo);
        let request = ExportRequest {
            from: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
            format: ExportFormat::JSON,
            include_reversals: true,
        };

        let result = service.export(request).await.unwrap();

        assert_eq!(result.format, ExportFormat::JSON);
        assert!(result.content.contains("\"journal_code\""));
        assert!(result.content.contains("31"));
    }
}
