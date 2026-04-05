use async_trait::async_trait;
use uuid::Uuid;

use banko_domain::payment::{OrderId, PaymentOrder, PaymentStatus, SwiftMessage, Transfer, TransferId};

// --- Payment Order Repository ---

#[async_trait]
pub trait IPaymentRepository: Send + Sync {
    async fn save(&self, order: &PaymentOrder) -> Result<(), String>;
    async fn find_by_id(&self, id: &OrderId) -> Result<Option<PaymentOrder>, String>;
    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<PaymentOrder>, String>;
    async fn find_all(
        &self,
        status: Option<PaymentStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PaymentOrder>, String>;
    async fn count_all(&self, status: Option<PaymentStatus>) -> Result<i64, String>;
}

// --- Transfer Repository ---

#[async_trait]
pub trait ITransferRepository: Send + Sync {
    async fn save(&self, transfer: &Transfer) -> Result<(), String>;
    async fn find_by_id(&self, id: &TransferId) -> Result<Option<Transfer>, String>;
    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Vec<Transfer>, String>;
    async fn find_submitted(&self) -> Result<Vec<Transfer>, String>;
}

// --- SWIFT Message Repository ---

#[async_trait]
pub trait ISwiftMessageRepository: Send + Sync {
    async fn save(&self, message: &SwiftMessage) -> Result<(), String>;
    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<SwiftMessage>, String>;
}

// --- Sanctions Screener Port (delegates to Sanctions BC) ---

pub struct ScreeningResult {
    pub is_hit: bool,
    pub match_details: Option<String>,
}

#[async_trait]
pub trait ISanctionsScreener: Send + Sync {
    async fn screen_beneficiary(
        &self,
        name: &str,
        bic: Option<&str>,
    ) -> Result<ScreeningResult, String>;
}
