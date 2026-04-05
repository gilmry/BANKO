use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use banko_domain::fx::{ExchangeRate, FxOperation, FxOperationId};

// --- FX Operation Repository ---

#[async_trait]
pub trait IFxRepository: Send + Sync {
    async fn save(&self, op: &FxOperation) -> Result<(), String>;
    async fn find_by_id(&self, id: &FxOperationId) -> Result<Option<FxOperation>, String>;
    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<FxOperation>, String>;
    async fn find_all(
        &self,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FxOperation>, String>;
    async fn count_all(&self, status: Option<&str>) -> Result<i64, String>;
    async fn get_daily_total(
        &self,
        account_id: Uuid,
        currency: &str,
        date: NaiveDate,
    ) -> Result<i64, String>;
}

// --- Exchange Rate Repository ---

#[async_trait]
pub trait IExchangeRateRepository: Send + Sync {
    async fn save(&self, rate: &ExchangeRate) -> Result<(), String>;
    async fn find_current(
        &self,
        source: &str,
        target: &str,
    ) -> Result<Option<ExchangeRate>, String>;
    async fn find_all_current(&self) -> Result<Vec<ExchangeRate>, String>;
    async fn find_history(
        &self,
        source: &str,
        target: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<ExchangeRate>, String>;
}
