use std::sync::Arc;

use uuid::Uuid;

use banko_domain::payment::*;

use super::dto::*;
use super::errors::PaymentServiceError;
use super::ports::*;

// ============================================================
// PaymentService (PAY-01, PAY-02, PAY-03, PAY-04, PAY-07, PAY-08)
// ============================================================

pub struct PaymentService {
    payment_repo: Arc<dyn IPaymentRepository>,
    screener: Arc<dyn ISanctionsScreener>,
}

impl PaymentService {
    pub fn new(
        payment_repo: Arc<dyn IPaymentRepository>,
        screener: Arc<dyn ISanctionsScreener>,
    ) -> Self {
        PaymentService {
            payment_repo,
            screener,
        }
    }

    /// Create a new payment order. If international/SWIFT, auto-triggers screening.
    pub async fn create_payment(
        &self,
        request: CreatePaymentRequest,
    ) -> Result<PaymentResponse, PaymentServiceError> {
        let sender_account_id = Uuid::parse_str(&request.sender_account_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}")))?;

        let payment_type = PaymentType::from_str_type(&request.payment_type)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        let currency = request.currency.unwrap_or_else(|| "TND".to_string());

        let mut order = PaymentOrder::new(
            sender_account_id,
            request.beneficiary_name,
            request.beneficiary_rib,
            request.beneficiary_bic,
            request.amount,
            currency,
            payment_type,
            request.reference,
            request.description,
        )
        .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        // Auto-trigger screening for international payments (INV-14)
        if order.requires_screening() {
            order.mark_pending_screening();
        }

        self.payment_repo
            .save(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::order_to_response(&order))
    }

    /// Screen a payment order against sanctions lists (INV-14).
    pub async fn screen_payment(
        &self,
        order_id: &str,
    ) -> Result<PaymentResponse, PaymentServiceError> {
        let id = self.parse_order_id(order_id)?;

        let mut order = self.get_order(&id).await?;

        let bic = order.beneficiary_bic().map(|s| s.to_string());
        let result = self
            .screener
            .screen_beneficiary(order.beneficiary_name(), bic.as_deref())
            .await
            .map_err(PaymentServiceError::Internal)?;

        if result.is_hit {
            let reason = result
                .match_details
                .unwrap_or_else(|| "Sanctions hit detected".to_string());
            order.mark_screening_hit(reason);
        } else {
            order.mark_screening_cleared();
        }

        self.payment_repo
            .save(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::order_to_response(&order))
    }

    /// Submit a payment order for execution. INV-14: screening must be cleared for international.
    pub async fn submit_payment(
        &self,
        order_id: &str,
    ) -> Result<PaymentResponse, PaymentServiceError> {
        let id = self.parse_order_id(order_id)?;
        let mut order = self.get_order(&id).await?;

        order
            .submit()
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.payment_repo
            .save(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::order_to_response(&order))
    }

    /// Execute a submitted payment.
    pub async fn execute_payment(
        &self,
        order_id: &str,
    ) -> Result<PaymentResponse, PaymentServiceError> {
        let id = self.parse_order_id(order_id)?;
        let mut order = self.get_order(&id).await?;

        order
            .execute()
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.payment_repo
            .save(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::order_to_response(&order))
    }

    /// Reject a payment order.
    pub async fn reject_payment(
        &self,
        order_id: &str,
        reason: String,
    ) -> Result<PaymentResponse, PaymentServiceError> {
        let id = self.parse_order_id(order_id)?;
        let mut order = self.get_order(&id).await?;

        order.reject(reason);

        self.payment_repo
            .save(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::order_to_response(&order))
    }

    /// Get a single payment order by ID.
    pub async fn get_payment(
        &self,
        order_id: &str,
    ) -> Result<PaymentResponse, PaymentServiceError> {
        let id = self.parse_order_id(order_id)?;
        let order = self.get_order(&id).await?;
        Ok(Self::order_to_response(&order))
    }

    /// Get payment status (lightweight).
    pub async fn get_payment_status(
        &self,
        order_id: &str,
    ) -> Result<PaymentStatusResponse, PaymentServiceError> {
        let id = self.parse_order_id(order_id)?;
        let order = self.get_order(&id).await?;

        Ok(PaymentStatusResponse {
            id: order.order_id().to_string(),
            status: order.status().as_str().to_string(),
            screening_status: order.screening_status().as_str().to_string(),
            submitted_at: order.submitted_at(),
            executed_at: order.executed_at(),
            rejection_reason: order.rejection_reason().map(|s| s.to_string()),
        })
    }

    /// List payments, optionally filtered by status.
    pub async fn list_payments(
        &self,
        account_id: Option<&str>,
        status: Option<&str>,
        page: i64,
        limit: i64,
    ) -> Result<PaymentListResponse, PaymentServiceError> {
        let offset = (page - 1) * limit;

        let payment_status = match status {
            Some(s) => Some(
                PaymentStatus::from_str_type(s)
                    .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?,
            ),
            None => None,
        };

        let (orders, total) = if let Some(acc_id) = account_id {
            let uuid = Uuid::parse_str(acc_id).map_err(|e| {
                PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}"))
            })?;
            let all = self
                .payment_repo
                .find_by_account(uuid)
                .await
                .map_err(PaymentServiceError::Internal)?;
            let total = all.len() as i64;
            let filtered: Vec<PaymentOrder> = all
                .into_iter()
                .filter(|o| payment_status.is_none() || Some(o.status()) == payment_status)
                .skip(offset as usize)
                .take(limit as usize)
                .collect();
            (filtered, total)
        } else {
            let orders = self
                .payment_repo
                .find_all(payment_status, limit, offset)
                .await
                .map_err(PaymentServiceError::Internal)?;
            let total = self
                .payment_repo
                .count_all(payment_status)
                .await
                .map_err(PaymentServiceError::Internal)?;
            (orders, total)
        };

        Ok(PaymentListResponse {
            data: orders.iter().map(Self::order_to_response).collect(),
            total,
            page,
            limit,
        })
    }

    // --- Helpers ---

    fn parse_order_id(&self, order_id: &str) -> Result<OrderId, PaymentServiceError> {
        let uuid = Uuid::parse_str(order_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid order ID: {e}")))?;
        Ok(OrderId::from_uuid(uuid))
    }

    async fn get_order(&self, id: &OrderId) -> Result<PaymentOrder, PaymentServiceError> {
        self.payment_repo
            .find_by_id(id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)
    }

    fn order_to_response(order: &PaymentOrder) -> PaymentResponse {
        PaymentResponse {
            id: order.order_id().to_string(),
            sender_account_id: order.sender_account_id().to_string(),
            beneficiary_name: order.beneficiary_name().to_string(),
            beneficiary_rib: order.beneficiary_rib().map(|s| s.to_string()),
            beneficiary_bic: order.beneficiary_bic().map(|s| s.to_string()),
            amount: order.amount(),
            currency: order.currency().to_string(),
            payment_type: order.payment_type().as_str().to_string(),
            status: order.status().as_str().to_string(),
            screening_status: order.screening_status().as_str().to_string(),
            reference: order.reference().to_string(),
            description: order.description().map(|s| s.to_string()),
            rejection_reason: order.rejection_reason().map(|s| s.to_string()),
            created_at: order.created_at(),
            submitted_at: order.submitted_at(),
            executed_at: order.executed_at(),
        }
    }
}

// ============================================================
// ClearingService (PAY-06)
// ============================================================

pub struct ClearingService {
    payment_repo: Arc<dyn IPaymentRepository>,
}

impl ClearingService {
    pub fn new(payment_repo: Arc<dyn IPaymentRepository>) -> Self {
        ClearingService { payment_repo }
    }

    /// Run clearing batch: process all Submitted payments.
    pub async fn run_clearing_batch(&self) -> Result<ClearingBatchResponse, PaymentServiceError> {
        let orders = self
            .payment_repo
            .find_all(Some(PaymentStatus::Submitted), 1000, 0)
            .await
            .map_err(PaymentServiceError::Internal)?;

        let mut processed = 0;
        let mut cleared = 0;
        let mut failed = 0;

        for mut order in orders {
            processed += 1;
            if let Err(_e) = order.process() {
                failed += 1;
                continue;
            }
            if let Err(_e) = order.clear() {
                failed += 1;
                continue;
            }
            if self.payment_repo.save(&order).await.is_ok() {
                cleared += 1;
            } else {
                failed += 1;
            }
        }

        Ok(ClearingBatchResponse {
            processed,
            cleared,
            failed,
        })
    }
}

// ============================================================
// SwiftService (PAY-05)
// ============================================================

pub struct SwiftService {
    payment_repo: Arc<dyn IPaymentRepository>,
    swift_repo: Arc<dyn ISwiftMessageRepository>,
}

impl SwiftService {
    pub fn new(
        payment_repo: Arc<dyn IPaymentRepository>,
        swift_repo: Arc<dyn ISwiftMessageRepository>,
    ) -> Self {
        SwiftService {
            payment_repo,
            swift_repo,
        }
    }

    /// Generate an MT103 SWIFT message for a payment order.
    pub async fn generate_swift_message(
        &self,
        order_id: &str,
        sender_bic: &str,
    ) -> Result<SwiftMessageResponse, PaymentServiceError> {
        let uuid = Uuid::parse_str(order_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid order ID: {e}")))?;
        let id = OrderId::from_uuid(uuid);

        let order = self
            .payment_repo
            .find_by_id(&id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        let message = SwiftMessage::generate_mt103(&order, sender_bic)
            .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.swift_repo
            .save(&message)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(SwiftMessageResponse {
            message_id: message.message_id().to_string(),
            order_id: message.order_id().to_string(),
            message_type: message.message_type().to_string(),
            sender_bic: message.sender_bic().to_string(),
            receiver_bic: message.receiver_bic().to_string(),
            amount: message.amount(),
            currency: message.currency().to_string(),
            reference: message.reference().to_string(),
            status: message.status().as_str().to_string(),
            created_at: message.created_at(),
        })
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;

    // --- Mock Payment Repository ---

    struct MockPaymentRepo {
        orders: Mutex<Vec<PaymentOrder>>,
    }

    impl MockPaymentRepo {
        fn new() -> Self {
            MockPaymentRepo {
                orders: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IPaymentRepository for MockPaymentRepo {
        async fn save(&self, order: &PaymentOrder) -> Result<(), String> {
            let mut orders = self.orders.lock().unwrap();
            orders.retain(|o| o.order_id() != order.order_id());
            orders.push(order.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &OrderId) -> Result<Option<PaymentOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders.iter().find(|o| o.order_id() == id).cloned())
        }
        async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<PaymentOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| o.sender_account_id() == account_id)
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            status: Option<PaymentStatus>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<PaymentOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| status.is_none() || Some(o.status()) == status)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self, status: Option<PaymentStatus>) -> Result<i64, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| status.is_none() || Some(o.status()) == status)
                .count() as i64)
        }
    }

    // --- Mock SWIFT Repo ---

    struct MockSwiftRepo {
        messages: Mutex<Vec<SwiftMessage>>,
    }

    impl MockSwiftRepo {
        fn new() -> Self {
            MockSwiftRepo {
                messages: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ISwiftMessageRepository for MockSwiftRepo {
        async fn save(&self, message: &SwiftMessage) -> Result<(), String> {
            let mut msgs = self.messages.lock().unwrap();
            msgs.push(message.clone());
            Ok(())
        }
        async fn find_by_order_id(
            &self,
            order_id: &OrderId,
        ) -> Result<Option<SwiftMessage>, String> {
            let msgs = self.messages.lock().unwrap();
            Ok(msgs.iter().find(|m| m.order_id() == order_id).cloned())
        }
    }

    // --- Mock Screener (Clear) ---

    struct MockClearScreener;

    #[async_trait]
    impl ISanctionsScreener for MockClearScreener {
        async fn screen_beneficiary(
            &self,
            _name: &str,
            _bic: Option<&str>,
        ) -> Result<ScreeningResult, String> {
            Ok(ScreeningResult {
                is_hit: false,
                match_details: None,
            })
        }
    }

    // --- Mock Screener (Hit) ---

    struct MockHitScreener;

    #[async_trait]
    impl ISanctionsScreener for MockHitScreener {
        async fn screen_beneficiary(
            &self,
            _name: &str,
            _bic: Option<&str>,
        ) -> Result<ScreeningResult, String> {
            Ok(ScreeningResult {
                is_hit: true,
                match_details: Some("UN sanctions list match".to_string()),
            })
        }
    }

    fn make_service(screener: Arc<dyn ISanctionsScreener>) -> PaymentService {
        PaymentService::new(Arc::new(MockPaymentRepo::new()), screener)
    }

    fn domestic_request() -> CreatePaymentRequest {
        CreatePaymentRequest {
            sender_account_id: Uuid::new_v4().to_string(),
            beneficiary_name: "Ahmed Ben Ali".to_string(),
            beneficiary_rib: Some("01234567890123456789".to_string()),
            beneficiary_bic: None,
            amount: 500_000,
            currency: Some("TND".to_string()),
            payment_type: "Domestic".to_string(),
            reference: "REF-2026-001".to_string(),
            description: Some("Test domestic".to_string()),
        }
    }

    fn international_request() -> CreatePaymentRequest {
        CreatePaymentRequest {
            sender_account_id: Uuid::new_v4().to_string(),
            beneficiary_name: "Pierre Dupont".to_string(),
            beneficiary_rib: None,
            beneficiary_bic: Some("BNPAFRPP".to_string()),
            amount: 100_000,
            currency: Some("EUR".to_string()),
            payment_type: "International".to_string(),
            reference: "REF-INT-001".to_string(),
            description: None,
        }
    }

    // --- Tests ---

    #[tokio::test]
    async fn test_create_domestic_payment() {
        let service = make_service(Arc::new(MockClearScreener));
        let result = service.create_payment(domestic_request()).await.unwrap();
        assert_eq!(result.status, "Draft");
        assert_eq!(result.screening_status, "NotScreened");
        assert_eq!(result.payment_type, "Domestic");
    }

    #[tokio::test]
    async fn test_create_international_payment_triggers_screening() {
        let service = make_service(Arc::new(MockClearScreener));
        let result = service
            .create_payment(international_request())
            .await
            .unwrap();
        // Auto-triggers screening for international
        assert_eq!(result.status, "PendingScreening");
        assert_eq!(result.screening_status, "Pending");
    }

    #[tokio::test]
    async fn test_screen_payment_clear() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service
            .create_payment(international_request())
            .await
            .unwrap();

        let screened = service.screen_payment(&created.id).await.unwrap();
        assert_eq!(screened.screening_status, "Cleared");
        assert_eq!(screened.status, "ScreeningCleared");
    }

    #[tokio::test]
    async fn test_screen_payment_hit_rejects() {
        let service = make_service(Arc::new(MockHitScreener));
        let created = service
            .create_payment(international_request())
            .await
            .unwrap();

        let screened = service.screen_payment(&created.id).await.unwrap();
        assert_eq!(screened.screening_status, "Hit");
        assert_eq!(screened.status, "Rejected");
        assert!(screened.rejection_reason.is_some());
    }

    #[tokio::test]
    async fn test_submit_domestic_without_screening() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service.create_payment(domestic_request()).await.unwrap();

        let submitted = service.submit_payment(&created.id).await.unwrap();
        assert_eq!(submitted.status, "Submitted");
    }

    #[tokio::test]
    async fn test_submit_international_after_screening() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service
            .create_payment(international_request())
            .await
            .unwrap();

        let screened = service.screen_payment(&created.id).await.unwrap();
        assert_eq!(screened.screening_status, "Cleared");

        let submitted = service.submit_payment(&created.id).await.unwrap();
        assert_eq!(submitted.status, "Submitted");
    }

    #[tokio::test]
    async fn test_submit_international_without_screening_fails_inv14() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service
            .create_payment(international_request())
            .await
            .unwrap();

        // Do NOT screen first — INV-14 violation
        let result = service.submit_payment(&created.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_payment() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service.create_payment(domestic_request()).await.unwrap();
        service.submit_payment(&created.id).await.unwrap();

        let executed = service.execute_payment(&created.id).await.unwrap();
        assert_eq!(executed.status, "Executed");
        assert!(executed.executed_at.is_some());
    }

    #[tokio::test]
    async fn test_reject_payment() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service.create_payment(domestic_request()).await.unwrap();

        let rejected = service
            .reject_payment(&created.id, "Compliance issue".to_string())
            .await
            .unwrap();
        assert_eq!(rejected.status, "Rejected");
    }

    #[tokio::test]
    async fn test_get_payment() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service.create_payment(domestic_request()).await.unwrap();

        let fetched = service.get_payment(&created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);
    }

    #[tokio::test]
    async fn test_get_payment_status() {
        let service = make_service(Arc::new(MockClearScreener));
        let created = service.create_payment(domestic_request()).await.unwrap();

        let status = service.get_payment_status(&created.id).await.unwrap();
        assert_eq!(status.id, created.id);
        assert_eq!(status.status, "Draft");
    }

    #[tokio::test]
    async fn test_list_payments() {
        let service = make_service(Arc::new(MockClearScreener));
        service.create_payment(domestic_request()).await.unwrap();
        service.create_payment(domestic_request()).await.unwrap();

        let list = service.list_payments(None, None, 1, 10).await.unwrap();
        assert_eq!(list.data.len(), 2);
        assert_eq!(list.total, 2);
    }

    #[tokio::test]
    async fn test_clearing_batch() {
        let repo = Arc::new(MockPaymentRepo::new());
        let screener: Arc<dyn ISanctionsScreener> = Arc::new(MockClearScreener);
        let service = PaymentService::new(repo.clone(), screener);
        let clearing = ClearingService::new(repo);

        let created = service.create_payment(domestic_request()).await.unwrap();
        service.submit_payment(&created.id).await.unwrap();

        let result = clearing.run_clearing_batch().await.unwrap();
        assert_eq!(result.processed, 1);
        assert_eq!(result.cleared, 1);
        assert_eq!(result.failed, 0);
    }

    #[tokio::test]
    async fn test_swift_message_generation() {
        let payment_repo = Arc::new(MockPaymentRepo::new());
        let swift_repo = Arc::new(MockSwiftRepo::new());
        let screener: Arc<dyn ISanctionsScreener> = Arc::new(MockClearScreener);

        let payment_service = PaymentService::new(payment_repo.clone(), screener);
        let swift_service = SwiftService::new(payment_repo, swift_repo);

        let req = CreatePaymentRequest {
            sender_account_id: Uuid::new_v4().to_string(),
            beneficiary_name: "John Smith".to_string(),
            beneficiary_rib: None,
            beneficiary_bic: Some("CHASUS33".to_string()),
            amount: 5_000_00,
            currency: Some("USD".to_string()),
            payment_type: "Swift".to_string(),
            reference: "REF-SWIFT-001".to_string(),
            description: None,
        };
        let created = payment_service.create_payment(req).await.unwrap();

        let msg = swift_service
            .generate_swift_message(&created.id, "BIATTNTT")
            .await
            .unwrap();
        assert_eq!(msg.message_type, "MT103");
        assert_eq!(msg.sender_bic, "BIATTNTT");
        assert_eq!(msg.receiver_bic, "CHASUS33");
    }
}
