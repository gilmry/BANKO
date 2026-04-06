use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use banko_domain::prudential::{BreachAlert, Exposure, PrudentialRatio, RatioId, RatioSnapshot};

#[async_trait]
pub trait IPrudentialRepository: Send + Sync {
    async fn save(&self, ratio: &PrudentialRatio) -> Result<(), String>;
    async fn find_by_id(&self, id: &RatioId) -> Result<Option<PrudentialRatio>, String>;
    async fn find_by_institution(
        &self,
        institution_id: Uuid,
    ) -> Result<Option<PrudentialRatio>, String>;
    async fn find_latest(&self, institution_id: Uuid) -> Result<Option<PrudentialRatio>, String>;
    async fn save_snapshot(&self, snapshot: &RatioSnapshot) -> Result<(), String>;
    async fn find_snapshots(
        &self,
        institution_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<RatioSnapshot>, String>;
    async fn save_exposure(&self, ratio_id: &RatioId, exposure: &Exposure) -> Result<(), String>;
    async fn find_exposures(&self, ratio_id: &RatioId) -> Result<Vec<Exposure>, String>;
}

#[async_trait]
pub trait IBreachAlertRepository: Send + Sync {
    async fn save(&self, alert: &BreachAlert) -> Result<(), String>;
    async fn find_active(&self, institution_id: Uuid) -> Result<Vec<BreachAlert>, String>;
    async fn find_all(
        &self,
        institution_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BreachAlert>, String>;
    async fn count_active(&self, institution_id: Option<Uuid>) -> Result<i64, String>;
}
