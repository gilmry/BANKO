use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use banko_domain::payment::*;

use super::dto::*;
use super::errors::PaymentServiceError;
use super::ports::*;

// ============================================================
// CardService (STORY-CARD-01 through CARD-06)
// ============================================================

pub struct CardService {
    card_repo: Arc<dyn ICardRepository>,
    transaction_repo: Arc<dyn ICardTransactionRepository>,
}

impl CardService {
    pub fn new(
        card_repo: Arc<dyn ICardRepository>,
        transaction_repo: Arc<dyn ICardTransactionRepository>,
    ) -> Self {
        CardService {
            card_repo,
            transaction_repo,
        }
    }

    /// Request a new card (STORY-CARD-01)
    pub async fn request_card(
        &self,
        request: RequestCardRequest,
    ) -> Result<CardResponse, PaymentServiceError> {
        let customer_id = Uuid::parse_str(&request.customer_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid customer ID: {e}")))?;

        let account_id = Uuid::parse_str(&request.account_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}")))?;

        let card_type = CardType::from_str_type(&request.card_type)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        let network = CardNetwork::from_str_type(&request.network)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        let validity_years = request.validity_years.unwrap_or(5);

        let card = Card::new(account_id, customer_id, card_type, network, validity_years);

        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::card_to_response(&card))
    }

    /// Activate a card (STORY-CARD-02)
    pub async fn activate_card(
        &self,
        request: ActivateCardRequest,
    ) -> Result<CardResponse, PaymentServiceError> {
        let card_id = Uuid::parse_str(&request.card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let mut card = self
            .card_repo
            .find_by_id(card_id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        card.activate(&request.activation_code)
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::card_to_response(&card))
    }

    /// Get a card by ID (STORY-CARD-02)
    pub async fn get_card(&self, id: &str) -> Result<CardResponse, PaymentServiceError> {
        let card_id = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let card = self
            .card_repo
            .find_by_id(card_id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        Ok(Self::card_to_response(&card))
    }

    /// List all cards for a customer (STORY-CARD-02)
    pub async fn list_customer_cards(
        &self,
        customer_id: &str,
    ) -> Result<CardListResponse, PaymentServiceError> {
        let cust_id = Uuid::parse_str(customer_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid customer ID: {e}")))?;

        let cards = self
            .card_repo
            .find_by_customer(cust_id)
            .await
            .map_err(PaymentServiceError::Internal)?;

        let total = cards.len();
        let data: Vec<CardResponse> = cards.iter().map(Self::card_to_response).collect();

        Ok(CardListResponse { data, total })
    }

    /// Block a card (STORY-CARD-03)
    pub async fn block_card(&self, card_id: &str) -> Result<(), PaymentServiceError> {
        let id = Uuid::parse_str(card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let mut card = self
            .card_repo
            .find_by_id(id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        card.block()
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    /// Unblock a card (STORY-CARD-03)
    pub async fn unblock_card(&self, card_id: &str) -> Result<(), PaymentServiceError> {
        let id = Uuid::parse_str(card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let mut card = self
            .card_repo
            .find_by_id(id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        card.unblock()
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    /// Cancel a card (STORY-CARD-03)
    pub async fn cancel_card(&self, card_id: &str) -> Result<(), PaymentServiceError> {
        let id = Uuid::parse_str(card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let mut card = self
            .card_repo
            .find_by_id(id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        card.cancel()
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    /// Set card spending limits (STORY-CARD-04)
    pub async fn set_card_limits(
        &self,
        request: SetCardLimitRequest,
    ) -> Result<CardResponse, PaymentServiceError> {
        let card_id = Uuid::parse_str(&request.card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let mut card = self
            .card_repo
            .find_by_id(card_id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        if let Some(daily) = request.daily_limit {
            card.set_daily_limit(daily)
                .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;
        }

        if let Some(monthly) = request.monthly_limit {
            card.set_monthly_limit(monthly)
                .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;
        }

        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::card_to_response(&card))
    }

    /// Get card transactions (STORY-CARD-05)
    pub async fn get_card_transactions(
        &self,
        card_id: &str,
    ) -> Result<CardTransactionListResponse, PaymentServiceError> {
        let id = Uuid::parse_str(card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let transactions = self
            .transaction_repo
            .find_by_card(id)
            .await
            .map_err(PaymentServiceError::Internal)?;

        let total = transactions.len();
        let data: Vec<CardTransactionResponse> =
            transactions.iter().map(Self::transaction_to_response).collect();

        Ok(CardTransactionListResponse { data, total })
    }

    /// Authorize a card transaction (STORY-CARD-05, CARD-06)
    pub async fn authorize_transaction(
        &self,
        request: AuthorizeTransactionRequest,
    ) -> Result<CardTransactionResponse, PaymentServiceError> {
        let card_id = Uuid::parse_str(&request.card_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid card ID: {e}")))?;

        let mut card = self
            .card_repo
            .find_by_id(card_id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        let now = Utc::now();

        // Check if card can transact
        card.can_transact(request.amount, now)
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        // Record the transaction on the card
        card.record_transaction(request.amount);

        // Save the updated card (with new spending)
        self.card_repo
            .save(&card)
            .await
            .map_err(PaymentServiceError::Internal)?;

        // Create and persist the transaction
        let currency = request.currency.unwrap_or_else(|| "TND".to_string());
        let is_contactless = request.is_contactless.unwrap_or(false);
        let is_online = request.is_online.unwrap_or(true);

        let transaction = CardTransaction::new(
            card_id,
            request.amount,
            currency,
            request.merchant_name,
            request.mcc_code,
            is_contactless,
            is_online,
        );

        self.transaction_repo
            .save(&transaction)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::transaction_to_response(&transaction))
    }

    // --- Helpers ---

    fn card_to_response(card: &Card) -> CardResponse {
        CardResponse {
            id: card.id().to_string(),
            account_id: card.account_id().to_string(),
            customer_id: card.customer_id().to_string(),
            card_type: card.card_type().as_str().to_string(),
            network: card.network().as_str().to_string(),
            masked_pan: card.masked_pan().to_string(),
            status: card.status().as_str().to_string(),
            daily_limit: card.daily_limit(),
            monthly_limit: card.monthly_limit(),
            daily_spent: card.daily_spent(),
            monthly_spent: card.monthly_spent(),
            expiry_month: card.expiry_month(),
            expiry_year: card.expiry_year(),
            is_contactless_enabled: card.is_contactless_enabled(),
            created_at: card.created_at(),
            activated_at: card.activated_at(),
            cancelled_at: card.cancelled_at(),
        }
    }

    fn transaction_to_response(transaction: &CardTransaction) -> CardTransactionResponse {
        CardTransactionResponse {
            id: transaction.id().to_string(),
            card_id: transaction.card_id().to_string(),
            amount: transaction.amount(),
            currency: transaction.currency().to_string(),
            merchant_name: transaction.merchant_name().to_string(),
            mcc_code: transaction.mcc_code().to_string(),
            status: transaction.status().as_str().to_string(),
            auth_code: transaction.auth_code().to_string(),
            timestamp: transaction.timestamp(),
            is_contactless: transaction.is_contactless(),
            is_online: transaction.is_online(),
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    // --- Mock Repositories ---

    struct MockCardRepo {
        cards: Mutex<Vec<Card>>,
    }

    impl MockCardRepo {
        fn new() -> Self {
            MockCardRepo {
                cards: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ICardRepository for MockCardRepo {
        async fn save(&self, card: &Card) -> Result<(), String> {
            let mut cards = self.cards.lock().unwrap();
            cards.retain(|c| c.id() != card.id());
            cards.push(card.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<Card>, String> {
            let cards = self.cards.lock().unwrap();
            Ok(cards.iter().find(|c| c.id() == id).cloned())
        }

        async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<Card>, String> {
            let cards = self.cards.lock().unwrap();
            Ok(cards
                .iter()
                .filter(|c| c.account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<Card>, String> {
            let cards = self.cards.lock().unwrap();
            Ok(cards
                .iter()
                .filter(|c| c.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn update(&self, card: &Card) -> Result<(), String> {
            self.save(card).await
        }

        async fn list_active(&self) -> Result<Vec<Card>, String> {
            let cards = self.cards.lock().unwrap();
            Ok(cards
                .iter()
                .filter(|c| c.is_active())
                .cloned()
                .collect())
        }
    }

    struct MockTransactionRepo {
        transactions: Mutex<Vec<CardTransaction>>,
    }

    impl MockTransactionRepo {
        fn new() -> Self {
            MockTransactionRepo {
                transactions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ICardTransactionRepository for MockTransactionRepo {
        async fn save(&self, transaction: &CardTransaction) -> Result<(), String> {
            let mut txns = self.transactions.lock().unwrap();
            txns.push(transaction.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<CardTransaction>, String> {
            let txns = self.transactions.lock().unwrap();
            Ok(txns.iter().find(|t| t.id() == id).cloned())
        }

        async fn find_by_card(&self, card_id: Uuid) -> Result<Vec<CardTransaction>, String> {
            let txns = self.transactions.lock().unwrap();
            Ok(txns
                .iter()
                .filter(|t| t.card_id() == card_id)
                .cloned()
                .collect())
        }

        async fn find_by_card_and_period(
            &self,
            card_id: Uuid,
            from: chrono::NaiveDate,
            to: chrono::NaiveDate,
        ) -> Result<Vec<CardTransaction>, String> {
            let txns = self.transactions.lock().unwrap();
            Ok(txns
                .iter()
                .filter(|t| {
                    t.card_id() == card_id
                        && t.timestamp().date_naive() >= from
                        && t.timestamp().date_naive() <= to
                })
                .cloned()
                .collect())
        }
    }

    fn make_service() -> CardService {
        CardService::new(
            Arc::new(MockCardRepo::new()),
            Arc::new(MockTransactionRepo::new()),
        )
    }

    // --- Tests ---

    #[tokio::test]
    async fn test_request_card() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let response = service.request_card(request).await.unwrap();
        assert_eq!(response.card_type, "Debit");
        assert_eq!(response.network, "Visa");
        assert_eq!(response.status, "Issued");
    }

    #[tokio::test]
    async fn test_activate_card() {
        let service = make_service();
        let cust_id = Uuid::new_v4().to_string();
        let acc_id = Uuid::new_v4().to_string();

        let request = RequestCardRequest {
            customer_id: cust_id,
            account_id: acc_id,
            card_type: "Credit".to_string(),
            network: "Mastercard".to_string(),
            validity_years: None,
        };

        let card_resp = service.request_card(request).await.unwrap();

        let activate = ActivateCardRequest {
            card_id: card_resp.id.clone(),
            activation_code: "123456".to_string(),
        };

        let activated = service.activate_card(activate).await.unwrap();
        assert_eq!(activated.status, "Active");
    }

    #[tokio::test]
    async fn test_get_card() {
        let service = make_service();
        let cust_id = Uuid::new_v4().to_string();
        let acc_id = Uuid::new_v4().to_string();

        let request = RequestCardRequest {
            customer_id: cust_id,
            account_id: acc_id,
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(3),
        };

        let card_resp = service.request_card(request).await.unwrap();
        let fetched = service.get_card(&card_resp.id).await.unwrap();

        assert_eq!(fetched.id, card_resp.id);
        assert_eq!(fetched.card_type, "Debit");
    }

    #[tokio::test]
    async fn test_list_customer_cards() {
        let service = make_service();
        let cust_id = Uuid::new_v4().to_string();
        let acc_id = Uuid::new_v4().to_string();

        for _ in 0..3 {
            let request = RequestCardRequest {
                customer_id: cust_id.clone(),
                account_id: acc_id.clone(),
                card_type: "Debit".to_string(),
                network: "Visa".to_string(),
                validity_years: Some(5),
            };
            service.request_card(request).await.unwrap();
        }

        let list = service.list_customer_cards(&cust_id).await.unwrap();
        assert_eq!(list.total, 3);
        assert_eq!(list.data.len(), 3);
    }

    #[tokio::test]
    async fn test_block_and_unblock_card() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let card_resp = service.request_card(request).await.unwrap();

        let activate = ActivateCardRequest {
            card_id: card_resp.id.clone(),
            activation_code: "123456".to_string(),
        };
        service.activate_card(activate).await.unwrap();

        service.block_card(&card_resp.id).await.unwrap();
        let blocked = service.get_card(&card_resp.id).await.unwrap();
        assert_eq!(blocked.status, "Blocked");

        service.unblock_card(&card_resp.id).await.unwrap();
        let unblocked = service.get_card(&card_resp.id).await.unwrap();
        assert_eq!(unblocked.status, "Active");
    }

    #[tokio::test]
    async fn test_cancel_card() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let card_resp = service.request_card(request).await.unwrap();
        service.cancel_card(&card_resp.id).await.unwrap();

        let cancelled = service.get_card(&card_resp.id).await.unwrap();
        assert_eq!(cancelled.status, "Cancelled");
    }

    #[tokio::test]
    async fn test_set_card_limits() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let card_resp = service.request_card(request).await.unwrap();

        let limit_request = SetCardLimitRequest {
            card_id: card_resp.id.clone(),
            daily_limit: Some(Decimal::new(5000_000, 3)),
            monthly_limit: Some(Decimal::new(100000_000, 3)),
        };

        let updated = service.set_card_limits(limit_request).await.unwrap();
        assert_eq!(updated.daily_limit, Decimal::new(5000_000, 3));
        assert_eq!(updated.monthly_limit, Decimal::new(100000_000, 3));
    }

    #[tokio::test]
    async fn test_authorize_transaction() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let card_resp = service.request_card(request).await.unwrap();

        let activate = ActivateCardRequest {
            card_id: card_resp.id.clone(),
            activation_code: "123456".to_string(),
        };
        service.activate_card(activate).await.unwrap();

        let txn_request = AuthorizeTransactionRequest {
            card_id: card_resp.id.clone(),
            amount: Decimal::new(100_000, 3),
            currency: Some("TND".to_string()),
            merchant_name: "Merchant ABC".to_string(),
            mcc_code: "5411".to_string(),
            is_contactless: Some(false),
            is_online: Some(true),
        };

        let txn_resp = service.authorize_transaction(txn_request).await.unwrap();
        assert_eq!(txn_resp.merchant_name, "Merchant ABC");
        assert_eq!(txn_resp.status, "Authorized");

        let transactions = service.get_card_transactions(&card_resp.id).await.unwrap();
        assert_eq!(transactions.total, 1);
    }

    #[tokio::test]
    async fn test_authorize_transaction_exceeding_limit_fails() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let card_resp = service.request_card(request).await.unwrap();

        let activate = ActivateCardRequest {
            card_id: card_resp.id.clone(),
            activation_code: "123456".to_string(),
        };
        service.activate_card(activate).await.unwrap();

        let txn_request = AuthorizeTransactionRequest {
            card_id: card_resp.id.clone(),
            amount: Decimal::new(3000_000, 3), // Exceeds default 2000 limit
            currency: Some("TND".to_string()),
            merchant_name: "Merchant".to_string(),
            mcc_code: "5411".to_string(),
            is_contactless: None,
            is_online: None,
        };

        assert!(service.authorize_transaction(txn_request).await.is_err());
    }

    #[tokio::test]
    async fn test_get_card_transactions() {
        let service = make_service();
        let request = RequestCardRequest {
            customer_id: Uuid::new_v4().to_string(),
            account_id: Uuid::new_v4().to_string(),
            card_type: "Debit".to_string(),
            network: "Visa".to_string(),
            validity_years: Some(5),
        };

        let card_resp = service.request_card(request).await.unwrap();

        let activate = ActivateCardRequest {
            card_id: card_resp.id.clone(),
            activation_code: "123456".to_string(),
        };
        service.activate_card(activate).await.unwrap();

        for i in 0..2 {
            let txn_request = AuthorizeTransactionRequest {
                card_id: card_resp.id.clone(),
                amount: Decimal::new(100_000, 3),
                currency: Some("TND".to_string()),
                merchant_name: format!("Merchant {}", i),
                mcc_code: "5411".to_string(),
                is_contactless: Some(false),
                is_online: Some(true),
            };
            service.authorize_transaction(txn_request).await.unwrap();
        }

        let transactions = service.get_card_transactions(&card_resp.id).await.unwrap();
        assert_eq!(transactions.total, 2);
    }
}
