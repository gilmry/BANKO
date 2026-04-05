use std::sync::Arc;

use banko_domain::sanctions::*;

use super::dto::*;
use super::errors::SanctionsServiceError;
use super::ports::*;

// ============================================================
// SanctionsScreeningService (SAN-02, SAN-04, SAN-05)
// ============================================================

pub struct SanctionsScreeningService {
    entry_repo: Arc<dyn ISanctionEntryRepository>,
    result_repo: Arc<dyn IScreeningResultRepository>,
    matching: Arc<dyn IMatchingStrategy>,
}

impl SanctionsScreeningService {
    pub fn new(
        entry_repo: Arc<dyn ISanctionEntryRepository>,
        result_repo: Arc<dyn IScreeningResultRepository>,
        matching: Arc<dyn IMatchingStrategy>,
    ) -> Self {
        SanctionsScreeningService {
            entry_repo,
            result_repo,
            matching,
        }
    }

    /// Screen a name against all active sanction entries.
    pub async fn screen_name(
        &self,
        name: &str,
        threshold: Option<u8>,
    ) -> Result<ScreeningResponse, SanctionsServiceError> {
        let threshold = threshold.unwrap_or(HIT_THRESHOLD);

        let entries = self
            .entry_repo
            .find_all_active()
            .await
            .map_err(SanctionsServiceError::Internal)?;

        let match_details = self.matching.screen(name, &entries, threshold);

        let result = ScreeningResult::new(name.to_string(), match_details);

        self.result_repo
            .save(&result)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(Self::result_to_response(&result))
    }

    /// Screen for payment (INV-14). If Hit → error. If PotentialMatch → held.
    pub async fn screen_payment(
        &self,
        counterparty: &str,
    ) -> Result<ScreeningResponse, SanctionsServiceError> {
        let response = self.screen_name(counterparty, None).await?;

        match response.status.as_str() {
            "Hit" => Err(SanctionsServiceError::PaymentBlocked),
            "PotentialMatch" => Err(SanctionsServiceError::PaymentHeld),
            _ => Ok(response),
        }
    }

    pub async fn get_result(
        &self,
        id: &ScreeningResultId,
    ) -> Result<ScreeningResponse, SanctionsServiceError> {
        let result = self
            .result_repo
            .find_by_id(id)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::ResultNotFound)?;

        Ok(Self::result_to_response(&result))
    }

    pub async fn list_results(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<ScreeningListResponse, SanctionsServiceError> {
        let offset = (page - 1) * limit;
        let results = self
            .result_repo
            .find_recent(limit, offset)
            .await
            .map_err(SanctionsServiceError::Internal)?;
        let total = self
            .result_repo
            .count_by_status(None)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(ScreeningListResponse {
            data: results.iter().map(Self::result_to_response).collect(),
            total,
            page,
            limit,
        })
    }

    pub async fn get_screening_stats(
        &self,
    ) -> Result<ScreeningStatsResponse, SanctionsServiceError> {
        let total = self
            .result_repo
            .count_by_status(None)
            .await
            .map_err(SanctionsServiceError::Internal)?;
        let hits = self
            .result_repo
            .count_by_status(Some(ScreeningStatus::Hit))
            .await
            .map_err(SanctionsServiceError::Internal)?;
        let potential = self
            .result_repo
            .count_by_status(Some(ScreeningStatus::PotentialMatch))
            .await
            .map_err(SanctionsServiceError::Internal)?;
        let clear = self
            .result_repo
            .count_by_status(Some(ScreeningStatus::Clear))
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(ScreeningStatsResponse {
            total_screenings: total,
            hits,
            potential_matches: potential,
            clear,
            lists: Vec::new(), // Filled by caller with list repo data
        })
    }

    fn result_to_response(result: &ScreeningResult) -> ScreeningResponse {
        ScreeningResponse {
            id: result.id().to_string(),
            screened_name: result.screened_name().to_string(),
            status: result.status().as_str().to_string(),
            highest_score: result.highest_score(),
            matches: result
                .matched_entries()
                .iter()
                .map(|m| ScreeningMatchResponse {
                    entry_name: m.entry_name.clone(),
                    list_source: m.list_source.as_str().to_string(),
                    score: m.score,
                    matched_fields: m.matched_fields.clone(),
                })
                .collect(),
            screened_at: result.screened_at(),
        }
    }
}

// ============================================================
// ListSyncService (SAN-03, SAN-07)
// ============================================================

pub struct ListSyncService {
    list_repo: Arc<dyn ISanctionListRepository>,
    entry_repo: Arc<dyn ISanctionEntryRepository>,
}

impl ListSyncService {
    pub fn new(
        list_repo: Arc<dyn ISanctionListRepository>,
        entry_repo: Arc<dyn ISanctionEntryRepository>,
    ) -> Self {
        ListSyncService {
            list_repo,
            entry_repo,
        }
    }

    /// Sync a sanctions list: create/update list, add new entries, deactivate removed ones.
    pub async fn sync_list(
        &self,
        source: ListSource,
        version: String,
        new_entries: Vec<SanctionEntry>,
    ) -> Result<ListStatusResponse, SanctionsServiceError> {
        let mut list = self
            .list_repo
            .find_by_source(source)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .unwrap_or_else(|| SanctionList::new(source, version.clone()));

        // Add new entries
        for entry in &new_entries {
            list.add_entry(entry.clone());
        }

        self.list_repo
            .save(&list)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        self.entry_repo
            .save_entries(&new_entries)
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(ListStatusResponse {
            source: source.as_str().to_string(),
            version,
            entry_count: list.entry_count(),
            active_entries: list.active_entries().len(),
            last_updated: list.last_updated(),
        })
    }

    pub async fn get_list_status(
        &self,
        source: ListSource,
    ) -> Result<ListStatusResponse, SanctionsServiceError> {
        let list = self
            .list_repo
            .find_by_source(source)
            .await
            .map_err(SanctionsServiceError::Internal)?
            .ok_or(SanctionsServiceError::ListNotFound)?;

        Ok(ListStatusResponse {
            source: list.source().as_str().to_string(),
            version: list.version().to_string(),
            entry_count: list.entry_count(),
            active_entries: list.active_entries().len(),
            last_updated: list.last_updated(),
        })
    }

    pub async fn get_all_lists_status(
        &self,
    ) -> Result<Vec<ListStatusResponse>, SanctionsServiceError> {
        let lists = self
            .list_repo
            .find_all()
            .await
            .map_err(SanctionsServiceError::Internal)?;

        Ok(lists
            .iter()
            .map(|list| ListStatusResponse {
                source: list.source().as_str().to_string(),
                version: list.version().to_string(),
                entry_count: list.entry_count(),
                active_entries: list.active_entries().len(),
                last_updated: list.last_updated(),
            })
            .collect())
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use banko_domain::sanctions::*;

    use super::*;

    // --- Default matching strategy using domain fuzzy_matcher ---

    struct TestMatchingStrategy;

    impl IMatchingStrategy for TestMatchingStrategy {
        fn screen(
            &self,
            name: &str,
            entries: &[SanctionEntry],
            threshold: u8,
        ) -> Vec<MatchDetail> {
            screen_name(name, entries, threshold)
                .matched_entries()
                .to_vec()
        }
    }

    // --- Mock Repositories ---

    struct MockEntryRepo {
        entries: Mutex<Vec<SanctionEntry>>,
    }

    impl MockEntryRepo {
        fn new() -> Self {
            MockEntryRepo {
                entries: Mutex::new(Vec::new()),
            }
        }

        fn with_entries(entries: Vec<SanctionEntry>) -> Self {
            MockEntryRepo {
                entries: Mutex::new(entries),
            }
        }
    }

    #[async_trait]
    impl ISanctionEntryRepository for MockEntryRepo {
        async fn save_entries(&self, entries: &[SanctionEntry]) -> Result<(), String> {
            let mut all = self.entries.lock().unwrap();
            all.extend(entries.iter().cloned());
            Ok(())
        }
        async fn find_by_id(&self, id: &SanctionEntryId) -> Result<Option<SanctionEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries.iter().find(|e| e.id() == id).cloned())
        }
        async fn find_all_active(&self) -> Result<Vec<SanctionEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries.iter().filter(|e| e.active()).cloned().collect())
        }
        async fn find_by_source(&self, source: ListSource) -> Result<Vec<SanctionEntry>, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .filter(|e| e.list_source() == source)
                .cloned()
                .collect())
        }
        async fn count_active(&self) -> Result<i64, String> {
            let entries = self.entries.lock().unwrap();
            Ok(entries.iter().filter(|e| e.active()).count() as i64)
        }
    }

    struct MockResultRepo {
        results: Mutex<Vec<ScreeningResult>>,
    }

    impl MockResultRepo {
        fn new() -> Self {
            MockResultRepo {
                results: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IScreeningResultRepository for MockResultRepo {
        async fn save(&self, result: &ScreeningResult) -> Result<(), String> {
            let mut results = self.results.lock().unwrap();
            results.push(result.clone());
            Ok(())
        }
        async fn find_by_id(
            &self,
            id: &ScreeningResultId,
        ) -> Result<Option<ScreeningResult>, String> {
            let results = self.results.lock().unwrap();
            Ok(results.iter().find(|r| r.id() == id).cloned())
        }
        async fn find_recent(
            &self,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<ScreeningResult>, String> {
            let results = self.results.lock().unwrap();
            Ok(results
                .iter()
                .rev()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_by_status(
            &self,
            status: Option<ScreeningStatus>,
        ) -> Result<i64, String> {
            let results = self.results.lock().unwrap();
            Ok(results
                .iter()
                .filter(|r| status.is_none() || Some(r.status()) == status)
                .count() as i64)
        }
    }

    struct MockListRepo {
        lists: Mutex<Vec<SanctionList>>,
    }

    impl MockListRepo {
        fn new() -> Self {
            MockListRepo {
                lists: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ISanctionListRepository for MockListRepo {
        async fn save(&self, list: &SanctionList) -> Result<(), String> {
            let mut lists = self.lists.lock().unwrap();
            lists.retain(|l| l.id() != list.id());
            lists.push(list.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &SanctionListId) -> Result<Option<SanctionList>, String> {
            let lists = self.lists.lock().unwrap();
            Ok(lists.iter().find(|l| l.id() == id).cloned())
        }
        async fn find_by_source(&self, source: ListSource) -> Result<Option<SanctionList>, String> {
            let lists = self.lists.lock().unwrap();
            Ok(lists.iter().find(|l| l.source() == source).cloned())
        }
        async fn find_all(&self) -> Result<Vec<SanctionList>, String> {
            let lists = self.lists.lock().unwrap();
            Ok(lists.clone())
        }
    }

    fn make_entry(name: &str) -> SanctionEntry {
        SanctionEntry::new(ListSource::UN, name.to_string(), vec![], None, None).unwrap()
    }

    // --- Screening Service Tests ---

    #[tokio::test]
    async fn test_screen_name_clear() {
        let service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![
                make_entry("Mohammed Ben Ali"),
            ])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        let result = service.screen_name("Pierre Dupont", None).await.unwrap();
        assert_eq!(result.status, "Clear");
        assert_eq!(result.highest_score, 0);
    }

    #[tokio::test]
    async fn test_screen_name_hit() {
        let service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![
                make_entry("Jean Alaoui"),
            ])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        let result = service.screen_name("Jean Alaoui", None).await.unwrap();
        assert_eq!(result.status, "Hit");
        assert_eq!(result.highest_score, 100);
    }

    #[tokio::test]
    async fn test_screen_name_fuzzy_hit() {
        let service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![
                make_entry("Jean Alaoui"),
            ])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        let result = service.screen_name("Jean Alaouie", None).await.unwrap();
        assert_eq!(result.status, "Hit");
        assert!(result.highest_score > 80);
    }

    #[tokio::test]
    async fn test_screen_payment_clear() {
        let service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![
                make_entry("Mohammed Ben Ali"),
            ])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        let result = service.screen_payment("Pierre Dupont").await.unwrap();
        assert_eq!(result.status, "Clear");
    }

    #[tokio::test]
    async fn test_screen_payment_blocked_on_hit() {
        let service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![
                make_entry("Jean Alaoui"),
            ])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        let result = service.screen_payment("Jean Alaoui").await;
        assert!(matches!(result, Err(SanctionsServiceError::PaymentBlocked)));
    }

    // --- List Sync Service Tests ---

    #[tokio::test]
    async fn test_sync_new_list() {
        let service = ListSyncService::new(
            Arc::new(MockListRepo::new()),
            Arc::new(MockEntryRepo::new()),
        );

        let entries = vec![make_entry("Test Person")];
        let status = service
            .sync_list(ListSource::UN, "2026-04".to_string(), entries)
            .await
            .unwrap();

        assert_eq!(status.source, "UN");
        assert_eq!(status.version, "2026-04");
        assert_eq!(status.entry_count, 1);
    }

    #[tokio::test]
    async fn test_get_all_lists_status() {
        let list_repo = Arc::new(MockListRepo::new());
        let entry_repo = Arc::new(MockEntryRepo::new());
        let service = ListSyncService::new(list_repo, entry_repo);

        service
            .sync_list(ListSource::UN, "v1".to_string(), vec![make_entry("A")])
            .await
            .unwrap();
        service
            .sync_list(ListSource::EU, "v1".to_string(), vec![make_entry("B")])
            .await
            .unwrap();

        let statuses = service.get_all_lists_status().await.unwrap();
        assert_eq!(statuses.len(), 2);
    }

    // --- Stats ---

    #[tokio::test]
    async fn test_screening_stats() {
        let service = SanctionsScreeningService::new(
            Arc::new(MockEntryRepo::with_entries(vec![make_entry("Jean Alaoui")])),
            Arc::new(MockResultRepo::new()),
            Arc::new(TestMatchingStrategy),
        );

        service.screen_name("Jean Alaoui", None).await.unwrap(); // Hit
        service.screen_name("Pierre Dupont", None).await.unwrap(); // Clear

        let stats = service.get_screening_stats().await.unwrap();
        assert_eq!(stats.total_screenings, 2);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.clear, 1);
    }
}
