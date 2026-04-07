use std::sync::Arc;

use chrono::Utc;

use banko_domain::trade_finance::{
    BankGuarantee, BankGuaranteeId, CollectionStatus, CollectionType, DocumentaryCollection,
    DocumentaryCollectionId, GuaranteeType, LCStatus, LCType, LetterOfCredit, LetterOfCreditId,
    LimitType, TradeFinanceLimit, TradeFinanceLimitId,
};
use banko_domain::shared::{Currency, CustomerId, Money};

use super::dto::*;
use super::errors::TradeFinanceError;
use super::ports::*;

pub struct TradeFinanceService {
    lc_repo: Arc<dyn ILetterOfCreditRepository>,
    guarantee_repo: Arc<dyn IBankGuaranteeRepository>,
    collection_repo: Arc<dyn IDocumentaryCollectionRepository>,
    limit_repo: Arc<dyn ITradeFinanceLimitRepository>,
}

impl TradeFinanceService {
    pub fn new(
        lc_repo: Arc<dyn ILetterOfCreditRepository>,
        guarantee_repo: Arc<dyn IBankGuaranteeRepository>,
        collection_repo: Arc<dyn IDocumentaryCollectionRepository>,
        limit_repo: Arc<dyn ITradeFinanceLimitRepository>,
    ) -> Self {
        TradeFinanceService {
            lc_repo,
            guarantee_repo,
            collection_repo,
            limit_repo,
        }
    }

    // --- Letter of Credit Operations ---

    pub async fn create_letter_of_credit(
        &self,
        request: CreateLetterOfCreditRequest,
    ) -> Result<LetterOfCreditResponse, TradeFinanceError> {
        let lc_type = LCType::from_str(&request.lc_type)
            .map_err(|e| TradeFinanceError::InvalidLcConfiguration(e.to_string()))?;

        let applicant_id = CustomerId::parse(&request.applicant_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| TradeFinanceError::InvalidLcConfiguration(e.to_string()))?;

        let amount = Money::from_f64(request.amount, currency)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let lc = LetterOfCredit::new(
            lc_type,
            applicant_id,
            &request.beneficiary_name,
            &request.issuing_bank,
            &request.advising_bank,
            amount,
            request.issue_date,
            request.expiry_date,
            &request.terms_description,
            request.documents_required,
        )
        .map_err(|e| TradeFinanceError::InvalidLcConfiguration(e.to_string()))?;

        self.lc_repo
            .save(&lc)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.lc_to_response(&lc))
    }

    pub async fn get_letter_of_credit(
        &self,
        id: &str,
    ) -> Result<LetterOfCreditResponse, TradeFinanceError> {
        let lc_id = LetterOfCreditId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let lc = self
            .lc_repo
            .find_by_id(&lc_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::LetterOfCreditNotFound)?;

        if lc.is_expired() {
            return Err(TradeFinanceError::LcExpired);
        }

        Ok(self.lc_to_response(&lc))
    }

    pub async fn issue_letter_of_credit(
        &self,
        id: &str,
    ) -> Result<LetterOfCreditResponse, TradeFinanceError> {
        let lc_id = LetterOfCreditId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let mut lc = self
            .lc_repo
            .find_by_id(&lc_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::LetterOfCreditNotFound)?;

        lc.issue()
            .map_err(|e| TradeFinanceError::InvalidTransition(e.to_string()))?;

        self.lc_repo
            .save(&lc)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.lc_to_response(&lc))
    }

    pub async fn list_letters_of_credit_by_applicant(
        &self,
        applicant_id: &str,
    ) -> Result<Vec<LetterOfCreditResponse>, TradeFinanceError> {
        let app_id = CustomerId::parse(applicant_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let lcs = self
            .lc_repo
            .find_by_applicant(&app_id)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(lcs.iter().map(|lc| self.lc_to_response(lc)).collect())
    }

    // --- Bank Guarantee Operations ---

    pub async fn create_bank_guarantee(
        &self,
        request: CreateBankGuaranteeRequest,
    ) -> Result<BankGuaranteeResponse, TradeFinanceError> {
        let guarantee_type = GuaranteeType::from_str(&request.guarantee_type)
            .map_err(|e| TradeFinanceError::InvalidGuaranteeConfiguration(e.to_string()))?;

        let principal_id = CustomerId::parse(&request.principal_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| TradeFinanceError::InvalidGuaranteeConfiguration(e.to_string()))?;

        let amount = Money::from_f64(request.amount, currency)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let guarantee = BankGuarantee::new(
            guarantee_type,
            principal_id,
            &request.beneficiary_name,
            amount,
            request.issue_date,
            request.expiry_date,
            &request.claim_conditions,
        )
        .map_err(|e| TradeFinanceError::InvalidGuaranteeConfiguration(e.to_string()))?;

        self.guarantee_repo
            .save(&guarantee)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.guarantee_to_response(&guarantee))
    }

    pub async fn get_bank_guarantee(
        &self,
        id: &str,
    ) -> Result<BankGuaranteeResponse, TradeFinanceError> {
        let guarantee_id = BankGuaranteeId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let guarantee = self
            .guarantee_repo
            .find_by_id(&guarantee_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::BankGuaranteeNotFound)?;

        if guarantee.is_expired() {
            return Err(TradeFinanceError::GuaranteeExpired);
        }

        Ok(self.guarantee_to_response(&guarantee))
    }

    pub async fn call_bank_guarantee(
        &self,
        id: &str,
    ) -> Result<BankGuaranteeResponse, TradeFinanceError> {
        let guarantee_id = BankGuaranteeId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let mut guarantee = self
            .guarantee_repo
            .find_by_id(&guarantee_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::BankGuaranteeNotFound)?;

        guarantee
            .call()
            .map_err(|e| TradeFinanceError::InvalidTransition(e.to_string()))?;

        self.guarantee_repo
            .save(&guarantee)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.guarantee_to_response(&guarantee))
    }

    pub async fn list_guarantees_by_principal(
        &self,
        principal_id: &str,
    ) -> Result<Vec<BankGuaranteeResponse>, TradeFinanceError> {
        let prin_id = CustomerId::parse(principal_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let guarantees = self
            .guarantee_repo
            .find_by_principal(&prin_id)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(guarantees
            .iter()
            .map(|g| self.guarantee_to_response(g))
            .collect())
    }

    // --- Documentary Collection Operations ---

    pub async fn create_documentary_collection(
        &self,
        request: CreateDocumentaryCollectionRequest,
    ) -> Result<DocumentaryCollectionResponse, TradeFinanceError> {
        let collection_type = CollectionType::from_str(&request.collection_type)
            .map_err(|e| TradeFinanceError::InvalidCollectionConfiguration(e.to_string()))?;

        let exporter_id = CustomerId::parse(&request.exporter_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| TradeFinanceError::InvalidCollectionConfiguration(e.to_string()))?;

        let amount = Money::from_f64(request.amount, currency)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let collection = DocumentaryCollection::new(
            collection_type,
            exporter_id,
            &request.importer_name,
            amount,
            request.documents,
        )
        .map_err(|e| TradeFinanceError::InvalidCollectionConfiguration(e.to_string()))?;

        self.collection_repo
            .save(&collection)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.collection_to_response(&collection))
    }

    pub async fn get_documentary_collection(
        &self,
        id: &str,
    ) -> Result<DocumentaryCollectionResponse, TradeFinanceError> {
        let collection_id = DocumentaryCollectionId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let collection = self
            .collection_repo
            .find_by_id(&collection_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::DocumentaryCollectionNotFound)?;

        Ok(self.collection_to_response(&collection))
    }

    pub async fn present_collection(
        &self,
        id: &str,
    ) -> Result<DocumentaryCollectionResponse, TradeFinanceError> {
        let collection_id = DocumentaryCollectionId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let mut collection = self
            .collection_repo
            .find_by_id(&collection_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::DocumentaryCollectionNotFound)?;

        collection
            .present()
            .map_err(|e| TradeFinanceError::InvalidTransition(e.to_string()))?;

        self.collection_repo
            .save(&collection)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.collection_to_response(&collection))
    }

    pub async fn list_collections_by_exporter(
        &self,
        exporter_id: &str,
    ) -> Result<Vec<DocumentaryCollectionResponse>, TradeFinanceError> {
        let exp_id = CustomerId::parse(exporter_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let collections = self
            .collection_repo
            .find_by_exporter(&exp_id)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(collections
            .iter()
            .map(|c| self.collection_to_response(c))
            .collect())
    }

    // --- Trade Finance Limit Operations ---

    pub async fn create_trade_finance_limit(
        &self,
        request: CreateTradeFinanceLimitRequest,
    ) -> Result<TradeFinanceLimitResponse, TradeFinanceError> {
        let customer_id = CustomerId::parse(&request.customer_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let limit_type = LimitType::from_str(&request.limit_type)
            .map_err(|e| TradeFinanceError::InvalidLimitConfiguration(e.to_string()))?;

        let currency = Currency::try_from(&request.currency[..])
            .map_err(|e| TradeFinanceError::InvalidLimitConfiguration(e.to_string()))?;

        let total_limit = Money::from_f64(request.total_limit, currency)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let limit = TradeFinanceLimit::new(customer_id, limit_type, total_limit)
            .map_err(|e| TradeFinanceError::InvalidLimitConfiguration(e.to_string()))?;

        self.limit_repo
            .save(&limit)
            .await
            .map_err(TradeFinanceError::Internal)?;

        Ok(self.limit_to_response(&limit)?)
    }

    pub async fn get_trade_finance_limit(
        &self,
        id: &str,
    ) -> Result<TradeFinanceLimitResponse, TradeFinanceError> {
        let limit_id = TradeFinanceLimitId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let limit = self
            .limit_repo
            .find_by_id(&limit_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::TradeFinanceLimitNotFound)?;

        self.limit_to_response(&limit)
    }

    pub async fn utilize_limit(
        &self,
        id: &str,
        amount: f64,
    ) -> Result<TradeFinanceLimitResponse, TradeFinanceError> {
        let limit_id = TradeFinanceLimitId::parse(id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let mut limit = self
            .limit_repo
            .find_by_id(&limit_id)
            .await
            .map_err(TradeFinanceError::Internal)?
            .ok_or(TradeFinanceError::TradeFinanceLimitNotFound)?;

        let amount_money = Money::from_f64(amount, limit.total_limit().currency().clone())
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        limit
            .utilize(&amount_money)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        self.limit_repo
            .save(&limit)
            .await
            .map_err(TradeFinanceError::Internal)?;

        self.limit_to_response(&limit)
    }

    pub async fn list_limits_by_customer(
        &self,
        customer_id: &str,
    ) -> Result<Vec<TradeFinanceLimitResponse>, TradeFinanceError> {
        let cust_id = CustomerId::parse(customer_id)
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        let limits = self
            .limit_repo
            .find_by_customer(&cust_id)
            .await
            .map_err(TradeFinanceError::Internal)?;

        let mut responses = Vec::new();
        for limit in limits {
            responses.push(self.limit_to_response(&limit)?);
        }
        Ok(responses)
    }

    // --- Response Mappers ---

    fn lc_to_response(&self, lc: &LetterOfCredit) -> LetterOfCreditResponse {
        LetterOfCreditResponse {
            id: lc.id().to_string(),
            lc_type: lc.lc_type().to_string(),
            applicant_id: lc.applicant_id().to_string(),
            beneficiary_name: lc.beneficiary_name().to_string(),
            issuing_bank: lc.issuing_bank().to_string(),
            advising_bank: lc.advising_bank().to_string(),
            amount: lc.amount().as_f64(),
            currency: lc.amount().currency().code().to_string(),
            issue_date: lc.issue_date(),
            expiry_date: lc.expiry_date(),
            terms_description: lc.terms_description().to_string(),
            documents_required: lc.documents_required().to_vec(),
            status: lc.status().to_string(),
            is_expired: lc.is_expired(),
            created_at: lc.created_at(),
            updated_at: lc.updated_at(),
        }
    }

    fn guarantee_to_response(&self, guarantee: &BankGuarantee) -> BankGuaranteeResponse {
        BankGuaranteeResponse {
            id: guarantee.id().to_string(),
            guarantee_type: guarantee.guarantee_type().to_string(),
            principal_id: guarantee.principal_id().to_string(),
            beneficiary_name: guarantee.beneficiary_name().to_string(),
            amount: guarantee.amount().as_f64(),
            currency: guarantee.amount().currency().code().to_string(),
            issue_date: guarantee.issue_date(),
            expiry_date: guarantee.expiry_date(),
            claim_conditions: guarantee.claim_conditions().to_string(),
            status: guarantee.status().to_string(),
            is_expired: guarantee.is_expired(),
            created_at: guarantee.created_at(),
            updated_at: guarantee.updated_at(),
        }
    }

    fn collection_to_response(
        &self,
        collection: &DocumentaryCollection,
    ) -> DocumentaryCollectionResponse {
        DocumentaryCollectionResponse {
            id: collection.id().to_string(),
            collection_type: collection.collection_type().to_string(),
            exporter_id: collection.exporter_id().to_string(),
            importer_name: collection.importer_name().to_string(),
            amount: collection.amount().as_f64(),
            currency: collection.amount().currency().code().to_string(),
            documents: collection.documents().to_vec(),
            status: collection.status().to_string(),
            created_at: collection.created_at(),
            updated_at: collection.updated_at(),
        }
    }

    fn limit_to_response(
        &self,
        limit: &TradeFinanceLimit,
    ) -> Result<TradeFinanceLimitResponse, TradeFinanceError> {
        let utilization_rate = limit
            .utilization_rate()
            .map_err(|e| TradeFinanceError::DomainError(e.to_string()))?;

        Ok(TradeFinanceLimitResponse {
            id: limit.id().to_string(),
            customer_id: limit.customer_id().to_string(),
            limit_type: limit.limit_type().to_string(),
            total_limit: limit.total_limit().as_f64(),
            utilized: limit.utilized().as_f64(),
            available: limit.available().as_f64(),
            currency: limit.total_limit().currency().code().to_string(),
            utilization_rate,
            created_at: limit.created_at(),
            updated_at: limit.updated_at(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockLcRepository;
    #[async_trait::async_trait]
    impl ILetterOfCreditRepository for MockLcRepository {
        async fn save(&self, _lc: &LetterOfCredit) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &LetterOfCreditId) -> Result<Option<LetterOfCredit>, String> {
            Ok(None)
        }
        async fn find_by_applicant(
            &self,
            _applicant_id: &CustomerId,
        ) -> Result<Vec<LetterOfCredit>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &LetterOfCreditId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockGuaranteeRepository;
    #[async_trait::async_trait]
    impl IBankGuaranteeRepository for MockGuaranteeRepository {
        async fn save(&self, _guarantee: &BankGuarantee) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(&self, _id: &BankGuaranteeId) -> Result<Option<BankGuarantee>, String> {
            Ok(None)
        }
        async fn find_by_principal(
            &self,
            _principal_id: &CustomerId,
        ) -> Result<Vec<BankGuarantee>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &BankGuaranteeId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockCollectionRepository;
    #[async_trait::async_trait]
    impl IDocumentaryCollectionRepository for MockCollectionRepository {
        async fn save(&self, _collection: &DocumentaryCollection) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(
            &self,
            _id: &DocumentaryCollectionId,
        ) -> Result<Option<DocumentaryCollection>, String> {
            Ok(None)
        }
        async fn find_by_exporter(
            &self,
            _exporter_id: &CustomerId,
        ) -> Result<Vec<DocumentaryCollection>, String> {
            Ok(vec![])
        }
        async fn delete(&self, _id: &DocumentaryCollectionId) -> Result<(), String> {
            Ok(())
        }
    }

    struct MockLimitRepository;
    #[async_trait::async_trait]
    impl ITradeFinanceLimitRepository for MockLimitRepository {
        async fn save(&self, _limit: &TradeFinanceLimit) -> Result<(), String> {
            Ok(())
        }
        async fn find_by_id(
            &self,
            _id: &TradeFinanceLimitId,
        ) -> Result<Option<TradeFinanceLimit>, String> {
            Ok(None)
        }
        async fn find_by_customer(
            &self,
            _cust