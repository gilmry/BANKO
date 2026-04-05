use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use banko_domain::aml::{
    Alert, AlertStatus, AssetFreeze, Investigation, InvestigationStatus, RiskLevel,
    SuspicionReport, Transaction, TransactionId,
};

// --- Transaction Repository ---

#[async_trait]
pub trait ITransactionRepository: Send + Sync {
    async fn save(&self, tx: &Transaction) -> Result<(), String>;
    async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, String>;
    async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<Transaction>, String>;
    async fn find_by_date_range(
        &self,
        account_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Transaction>, String>;
    async fn find_all(
        &self,
        account_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>, String>;
    async fn count_all(&self, account_id: Option<Uuid>) -> Result<i64, String>;
}

// --- Alert Repository ---

#[async_trait]
pub trait IAlertRepository: Send + Sync {
    async fn save(&self, alert: &Alert) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Alert>, String>;
    async fn find_by_transaction_id(&self, tx_id: &TransactionId) -> Result<Vec<Alert>, String>;
    async fn find_by_status(&self, status: AlertStatus) -> Result<Vec<Alert>, String>;
    async fn find_all(
        &self,
        status: Option<AlertStatus>,
        risk_level: Option<RiskLevel>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Alert>, String>;
    async fn count_by_status(&self, status: Option<AlertStatus>) -> Result<i64, String>;
}

// --- Investigation Repository ---

#[async_trait]
pub trait IInvestigationRepository: Send + Sync {
    async fn save(&self, investigation: &Investigation) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Investigation>, String>;
    async fn find_by_alert_id(&self, alert_id: Uuid) -> Result<Option<Investigation>, String>;
    async fn find_by_status(
        &self,
        status: InvestigationStatus,
    ) -> Result<Vec<Investigation>, String>;
    async fn find_all(
        &self,
        status: Option<InvestigationStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Investigation>, String>;
    async fn count_all(&self, status: Option<InvestigationStatus>) -> Result<i64, String>;
}

// --- AML Scenario (pluggable detection) ---

/// Pluggable AML scenario interface.
/// Each scenario evaluates a transaction (optionally with history) and may produce an Alert.
pub trait IAmlScenario: Send + Sync {
    fn name(&self) -> &str;
    fn evaluate(&self, transaction: &Transaction, history: &[Transaction]) -> Option<Alert>;
}

// --- Suspicion Report Repository ---

#[async_trait]
pub trait ISuspicionReportRepository: Send + Sync {
    async fn save(&self, report: &SuspicionReport) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SuspicionReport>, String>;
    async fn find_by_investigation_id(
        &self,
        investigation_id: Uuid,
    ) -> Result<Option<SuspicionReport>, String>;
}

// --- Asset Freeze Repository ---

#[async_trait]
pub trait IAssetFreezeRepository: Send + Sync {
    async fn save(&self, freeze: &AssetFreeze) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AssetFreeze>, String>;
    async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<AssetFreeze>, String>;
    async fn find_active_by_account_id(
        &self,
        account_id: Uuid,
    ) -> Result<Option<AssetFreeze>, String>;
}
