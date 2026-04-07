use async_trait::async_trait;

use banko_domain::trade_finance::{
    BankGuarantee, BankGuaranteeId, DocumentaryCollection, DocumentaryCollectionId,
    LetterOfCredit, LetterOfCreditId, TradeFinanceLimit, TradeFinanceLimitId,
};
use banko_domain::shared::CustomerId;

/// Port for letter of credit persistence.
#[async_trait]
pub trait ILetterOfCreditRepository: Send + Sync {
    async fn save(&self, lc: &LetterOfCredit) -> Result<(), String>;
    async fn find_by_id(&self, id: &LetterOfCreditId) -> Result<Option<LetterOfCredit>, String>;
    async fn find_by_applicant(
        &self,
        applicant_id: &CustomerId,
    ) -> Result<Vec<LetterOfCredit>, String>;
    async fn delete(&self, id: &LetterOfCreditId) -> Result<(), String>;
}

/// Port for bank guarantee persistence.
#[async_trait]
pub trait IBankGuaranteeRepository: Send + Sync {
    async fn save(&self, guarantee: &BankGuarantee) -> Result<(), String>;
    async fn find_by_id(
        &self,
        id: &BankGuaranteeId,
    ) -> Result<Option<BankGuarantee>, String>;
    async fn find_by_principal(
        &self,
        principal_id: &CustomerId,
    ) -> Result<Vec<BankGuarantee>, String>;
    async fn delete(&self, id: &BankGuaranteeId) -> Result<(), String>;
}

/// Port for documentary collection persistence.
#[async_trait]
pub trait IDocumentaryCollectionRepository: Send + Sync {
    async fn save(&self, collection: &DocumentaryCollection) -> Result<(), String>;
    async fn find_by_id(
        &self,
        id: &DocumentaryCollectionId,
    ) -> Result<Option<DocumentaryCollection>, String>;
    async fn find_by_exporter(
        &self,
        exporter_id: &CustomerId,
    ) -> Result<Vec<DocumentaryCollection>, String>;
    async fn delete(&self, id: &DocumentaryCollectionId) -> Result<(), String>;
}

/// Port for trade finance limit persistence.
#[async_trait]
pub trait ITradeFinanceLimitRepository: Send + Sync {
    async fn save(&self, limit: &TradeFinanceLimit) -> Result<(), String>;
    async fn find_by_id(&self, id: &TradeFinanceLimitId) -> Result<Option<TradeFinanceLimit>, String>;
    async fn find_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<TradeFinanceLimit>, String>;
    async fn delete(&self, id: &TradeFinanceLimitId) -> Result<(), String>;
}
