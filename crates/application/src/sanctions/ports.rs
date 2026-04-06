use async_trait::async_trait;

use banko_domain::sanctions::{
    ListSource, MatchDetail, SanctionEntry, SanctionEntryId, SanctionList, SanctionListId,
    ScreeningResult, ScreeningResultId, ScreeningStatus,
};

// --- Sanction List Repository ---

#[async_trait]
pub trait ISanctionListRepository: Send + Sync {
    async fn save(&self, list: &SanctionList) -> Result<(), String>;
    async fn find_by_id(&self, id: &SanctionListId) -> Result<Option<SanctionList>, String>;
    async fn find_by_source(&self, source: ListSource) -> Result<Option<SanctionList>, String>;
    async fn find_all(&self) -> Result<Vec<SanctionList>, String>;
}

// --- Sanction Entry Repository ---

#[async_trait]
pub trait ISanctionEntryRepository: Send + Sync {
    async fn save_entries(&self, entries: &[SanctionEntry]) -> Result<(), String>;
    async fn find_by_id(&self, id: &SanctionEntryId) -> Result<Option<SanctionEntry>, String>;
    async fn find_all_active(&self) -> Result<Vec<SanctionEntry>, String>;
    async fn find_by_source(&self, source: ListSource) -> Result<Vec<SanctionEntry>, String>;
    async fn count_active(&self) -> Result<i64, String>;
}

// --- Screening Result Repository ---

#[async_trait]
pub trait IScreeningResultRepository: Send + Sync {
    async fn save(&self, result: &ScreeningResult) -> Result<(), String>;
    async fn find_by_id(&self, id: &ScreeningResultId) -> Result<Option<ScreeningResult>, String>;
    async fn find_recent(&self, limit: i64, offset: i64) -> Result<Vec<ScreeningResult>, String>;
    async fn count_by_status(&self, status: Option<ScreeningStatus>) -> Result<i64, String>;
}

// --- Matching Strategy (pluggable) ---

pub trait IMatchingStrategy: Send + Sync {
    fn screen(&self, name: &str, entries: &[SanctionEntry], threshold: u8) -> Vec<MatchDetail>;
}
