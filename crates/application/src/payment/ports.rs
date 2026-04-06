use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use banko_domain::payment::{
    OrderId, PaymentOrder, PaymentStatus, SwiftMessage, Transfer, TransferId,
    StandingOrder, DirectDebitMandate, DebitExecution,
};

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

// --- Concrete Sanctions Screener Adapter ---
/// Adapter that implements ISanctionsScreener by delegating to Sanctions BC
pub struct SanctionsScreenerAdapter {
    // In production, this would hold references to the Sanctions BC client/service
    // For now, we provide a basic implementation
}

impl SanctionsScreenerAdapter {
    pub fn new() -> Self {
        SanctionsScreenerAdapter {}
    }

    /// Match beneficiary against sanctions lists from Sanctions BC
    fn match_sanctions_list(&self, name: &str, _bic: Option<&str>) -> Option<String> {
        // Placeholder: In production, this would call the Sanctions BC matching logic
        // Simplified implementation: check for known patterns
        let name_lower = name.to_lowercase();

        // Example: Match against known sanctioned entity patterns
        if name_lower.contains("terrorist") || name_lower.contains("sanctioned") {
            Some("UN sanctions list match".to_string())
        } else {
            None
        }
    }
}

impl Default for SanctionsScreenerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ISanctionsScreener for SanctionsScreenerAdapter {
    async fn screen_beneficiary(
        &self,
        name: &str,
        bic: Option<&str>,
    ) -> Result<ScreeningResult, String> {
        // Call the matching logic
        let match_details = self.match_sanctions_list(name, bic);

        Ok(ScreeningResult {
            is_hit: match_details.is_some(),
            match_details,
        })
    }
}

// --- Standing Order Repository (STORY-RECUR-01) ---

#[async_trait]
pub trait IStandingOrderRepository: Send + Sync {
    async fn save(&self, order: &StandingOrder) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<StandingOrder>, String>;
    async fn find_due_today(&self, today: NaiveDate) -> Result<Vec<StandingOrder>, String>;
    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<StandingOrder>, String>;
    async fn update(&self, order: &StandingOrder) -> Result<(), String>;
    async fn list_active(&self) -> Result<Vec<StandingOrder>, String>;
}

// --- Direct Debit Mandate Repository (STORY-RECUR-02) ---

#[async_trait]
pub trait IMandateRepository: Send + Sync {
    async fn save(&self, mandate: &DirectDebitMandate) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DirectDebitMandate>, String>;
    async fn find_by_debtor(&self, account_id: Uuid) -> Result<Vec<DirectDebitMandate>, String>;
    async fn find_active_by_creditor(&self, creditor_id: &str) -> Result<Vec<DirectDebitMandate>, String>;
    async fn update(&self, mandate: &DirectDebitMandate) -> Result<(), String>;
}

// --- Debit Execution Repository (STORY-RECUR-02) ---

#[async_trait]
pub trait IDebitExecutionRepository: Send + Sync {
    async fn save(&self, execution: &DebitExecution) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DebitExecution>, String>;
    async fn find_by_mandate(&self, mandate_id: Uuid) -> Result<Vec<DebitExecution>, String>;
}

// --- Card Repository (STORY-CARD-01 through CARD-06) ---

use banko_domain::payment::{Card, CardTransaction};

#[async_trait]
pub trait ICardRepository: Send + Sync {
    async fn save(&self, card: &Card) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Card>, String>;
    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<Card>, String>;
    async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<Card>, String>;
    async fn update(&self, card: &Card) -> Result<(), String>;
    async fn list_active(&self) -> Result<Vec<Card>, String>;
}

// --- Card Transaction Repository (STORY-CARD-01 through CARD-06) ---

#[async_trait]
pub trait ICardTransactionRepository: Send + Sync {
    async fn save(&self, transaction: &CardTransaction) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CardTransaction>, String>;
    async fn find_by_card(&self, card_id: Uuid) -> Result<Vec<CardTransaction>, String>;
    async fn find_by_card_and_period(
        &self,
        card_id: Uuid,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<CardTransaction>, String>;
}
