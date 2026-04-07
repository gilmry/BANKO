use chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

use banko_domain::payment::{
    DirectDebitMandate, DebitExecution, Frequency,
    StandingOrder,
};

use super::dto::*;
use super::errors::PaymentServiceError;
use super::ports::*;

// ============================================================
// RecurringPaymentService (STORY-RECUR-01, RECUR-02, RECUR-03)
// ============================================================

pub struct RecurringPaymentService {
    standing_order_repo: Arc<dyn IStandingOrderRepository>,
    mandate_repo: Arc<dyn IMandateRepository>,
    debit_execution_repo: Arc<dyn IDebitExecutionRepository>,
}

impl RecurringPaymentService {
    pub fn new(
        standing_order_repo: Arc<dyn IStandingOrderRepository>,
        mandate_repo: Arc<dyn IMandateRepository>,
        debit_execution_repo: Arc<dyn IDebitExecutionRepository>,
    ) -> Self {
        RecurringPaymentService {
            standing_order_repo,
            mandate_repo,
            debit_execution_repo,
        }
    }

    // --- Standing Order Operations (STORY-RECUR-01) ---

    pub async fn create_standing_order(
        &self,
        req: CreateStandingOrderRequest,
    ) -> Result<StandingOrderResponse, PaymentServiceError> {
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}")))?;

        let frequency = Frequency::from_str(&req.frequency)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        let currency = req.currency.unwrap_or_else(|| "TND".to_string());

        let order = StandingOrder::new(
            account_id,
            req.beneficiary_account,
            req.beneficiary_name,
            req.amount,
            currency,
            frequency,
            req.reference,
            req.start_date,
            req.end_date,
            req.max_executions,
        )
        .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.standing_order_repo
            .save(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::order_to_response(&order))
    }

    pub async fn get_standing_order(
        &self,
        id: &str,
    ) -> Result<StandingOrderResponse, PaymentServiceError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid order ID: {e}")))?;

        let order = self
            .standing_order_repo
            .find_by_id(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        Ok(Self::order_to_response(&order))
    }

    pub async fn list_account_standing_orders(
        &self,
        account_id: &str,
    ) -> Result<StandingOrderListResponse, PaymentServiceError> {
        let uuid = Uuid::parse_str(account_id).map_err(|e| {
            PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}"))
        })?;

        let orders = self
            .standing_order_repo
            .find_by_account(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?;

        let total = orders.len();
        let data = orders.iter().map(Self::order_to_response).collect();

        Ok(StandingOrderListResponse { data, total })
    }

    pub async fn suspend_standing_order(&self, id: &str) -> Result<(), PaymentServiceError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid order ID: {e}")))?;

        let mut order = self
            .standing_order_repo
            .find_by_id(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        order.suspend();

        self.standing_order_repo
            .update(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    pub async fn resume_standing_order(&self, id: &str) -> Result<(), PaymentServiceError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid order ID: {e}")))?;

        let mut order = self
            .standing_order_repo
            .find_by_id(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        order.resume();

        self.standing_order_repo
            .update(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    pub async fn cancel_standing_order(&self, id: &str) -> Result<(), PaymentServiceError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid order ID: {e}")))?;

        let mut order = self
            .standing_order_repo
            .find_by_id(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        order.cancel();

        self.standing_order_repo
            .update(&order)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    // --- Direct Debit Mandate Operations (STORY-RECUR-02) ---

    pub async fn create_mandate(
        &self,
        req: CreateMandateRequest,
    ) -> Result<MandateResponse, PaymentServiceError> {
        let debtor_id = Uuid::parse_str(&req.debtor_account_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}")))?;

        let frequency = Frequency::from_str(&req.frequency)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        let currency = req.currency.unwrap_or_else(|| "TND".to_string());

        let mandate = DirectDebitMandate::new(
            debtor_id,
            req.debtor_name,
            req.creditor_id,
            req.creditor_name,
            req.amount_limit,
            currency,
            frequency,
            req.reference,
            req.expires_at,
        )
        .map_err(|e| PaymentServiceError::DomainError(e.to_string()))?;

        self.mandate_repo
            .save(&mandate)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::mandate_to_response(&mandate))
    }

    pub async fn sign_mandate(&self, id: &str) -> Result<(), PaymentServiceError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid mandate ID: {e}")))?;

        let mut mandate = self
            .mandate_repo
            .find_by_id(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        mandate.sign(chrono::Utc::now());

        self.mandate_repo
            .update(&mandate)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    pub async fn revoke_mandate(&self, id: &str) -> Result<(), PaymentServiceError> {
        let uuid = Uuid::parse_str(id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid mandate ID: {e}")))?;

        let mut mandate = self
            .mandate_repo
            .find_by_id(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        mandate.revoke();

        self.mandate_repo
            .update(&mandate)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(())
    }

    pub async fn list_account_mandates(
        &self,
        debtor_account_id: &str,
    ) -> Result<MandateListResponse, PaymentServiceError> {
        let uuid = Uuid::parse_str(debtor_account_id).map_err(|e| {
            PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}"))
        })?;

        let mandates = self
            .mandate_repo
            .find_by_debtor(uuid)
            .await
            .map_err(PaymentServiceError::Internal)?;

        let total = mandates.len();
        let data = mandates.iter().map(Self::mandate_to_response).collect();

        Ok(MandateListResponse { data, total })
    }

    // --- Batch Execution (STORY-RECUR-02 + RECUR-03) ---

    /// Execute all standing orders due today
    pub async fn execute_due_standing_orders(
        &self,
        today: NaiveDate,
    ) -> Result<BatchExecutionResult, PaymentServiceError> {
        let orders = self
            .standing_order_repo
            .find_due_today(today)
            .await
            .map_err(PaymentServiceError::Internal)?;

        let mut result = BatchExecutionResult {
            total: orders.len(),
            executed: 0,
            failed: 0,
            skipped: 0,
        };

        for mut order in orders {
            // Check if already completed
            if order.is_completed() {
                result.skipped += 1;
                continue;
            }

            // Validate balance (mock check)
            if !self.validate_balance(&order).await {
                result.failed += 1;
                continue;
            }

            // Mark as executed
            order.mark_executed(chrono::Utc::now());

            match self.standing_order_repo.update(&order).await {
                Ok(_) => result.executed += 1,
                Err(_) => result.failed += 1,
            }
        }

        Ok(result)
    }

    /// Execute all direct debits due today
    pub async fn execute_due_debits(
        &self,
        today: NaiveDate,
    ) -> Result<BatchExecutionResult, PaymentServiceError> {
        let mandates = self
            .mandate_repo
            .find_by_debtor(Uuid::nil()) // Placeholder: would need to fetch all debtor mandates
            .await
            .map_err(PaymentServiceError::Internal)?;

        let mut result = BatchExecutionResult {
            total: mandates.len(),
            executed: 0,
            failed: 0,
            skipped: 0,
        };

        for mandate in mandates {
            // Check if mandate can debit
            if !mandate.can_debit(mandate.amount_limit(), today) {
                result.skipped += 1;
                continue;
            }

            // Create debit execution
            let mut exec = DebitExecution::new(mandate.id(), mandate.amount_limit());

            // Validate balance (mock)
            if !self.validate_balance_for_debit(&mandate).await {
                exec.mark_failed("Insufficient balance".to_string());
                let _ = self.debit_execution_repo.save(&exec).await;
                result.failed += 1;
                continue;
            }

            // Mark as executed
            exec.mark_executed();
            self.debit_execution_repo
                .save(&exec)
                .await
                .map_err(PaymentServiceError::Internal)?;

            result.executed += 1;
        }

        Ok(result)
    }

    // --- Private Helpers ---

    fn order_to_response(order: &StandingOrder) -> StandingOrderResponse {
        StandingOrderResponse {
            id: order.id().to_string(),
            account_id: order.account_id().to_string(),
            beneficiary_account: order.beneficiary_account().to_string(),
            beneficiary_name: order.beneficiary_name().to_string(),
            amount: order.amount(),
            currency: order.currency().to_string(),
            frequency: order.frequency().as_str().to_string(),
            reference: order.reference().to_string(),
            start_date: order.start_date(),
            end_date: order.end_date(),
            next_execution_date: order.next_execution_date(),
            status: order.status().as_str().to_string(),
            execution_count: order.execution_count(),
            max_executions: order.max_executions(),
            created_at: order.created_at(),
        }
    }

    fn mandate_to_response(mandate: &DirectDebitMandate) -> MandateResponse {
        MandateResponse {
            id: mandate.id().to_string(),
            debtor_account_id: mandate.debtor_account_id().to_string(),
            debtor_name: mandate.debtor_name().to_string(),
            creditor_id: mandate.creditor_id().to_string(),
            creditor_name: mandate.creditor_name().to_string(),
            amount_limit: mandate.amount_limit(),
            currency: mandate.currency().to_string(),
            frequency: mandate.frequency().as_str().to_string(),
            reference: mandate.reference().to_string(),
            signed_at: mandate.signed_at(),
            expires_at: mandate.expires_at(),
            status: mandate.status().as_str().to_string(),
            created_at: mandate.created_at(),
        }
    }

    async fn validate_balance(&self, _order: &StandingOrder) -> bool {
        // Mock implementation: always return true
        true
    }

    async fn validate_balance_for_debit(&self, _mandate: &DirectDebitMandate) -> bool {
        // Mock implementation: always return true
        true
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockStandingOrderRepo {
        orders: Mutex<Vec<StandingOrder>>,
    }

    impl MockStandingOrderRepo {
        fn new() -> Self {
            MockStandingOrderRepo {
                orders: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IStandingOrderRepository for MockStandingOrderRepo {
        async fn save(&self, order: &StandingOrder) -> Result<(), String> {
            let mut orders = self.orders.lock().unwrap();
            orders.retain(|o| o.id() != order.id());
            orders.push(order.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders.iter().find(|o| o.id() == id).cloned())
        }

        async fn find_due_today(&self, today: NaiveDate) -> Result<Vec<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| o.is_due_today(today))
                .cloned()
                .collect())
        }

        async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| o.account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn update(&self, order: &StandingOrder) -> Result<(), String> {
            let mut orders = self.orders.lock().unwrap();
            if let Some(existing) = orders.iter_mut().find(|o| o.id() == order.id()) {
                *existing = order.clone();
            }
            Ok(())
        }

        async fn list_active(&self) -> Result<Vec<StandingOrder>, String> {
            let orders = self.orders.lock().unwrap();
            Ok(orders
                .iter()
                .filter(|o| o.status() == StandingOrderStatus::Active)
                .cloned()
                .collect())
        }
    }

    struct MockMandateRepo {
        mandates: Mutex<Vec<DirectDebitMandate>>,
    }

    impl MockMandateRepo {
        fn new() -> Self {
            MockMandateRepo {
                mandates: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IMandateRepository for MockMandateRepo {
        async fn save(&self, mandate: &DirectDebitMandate) -> Result<(), String> {
            let mut mandates = self.mandates.lock().unwrap();
            mandates.retain(|m| m.id() != mandate.id());
            mandates.push(mandate.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<DirectDebitMandate>, String> {
            let mandates = self.mandates.lock().unwrap();
            Ok(mandates.iter().find(|m| m.id() == id).cloned())
        }

        async fn find_by_debtor(&self, account_id: Uuid) -> Result<Vec<DirectDebitMandate>, String> {
            let mandates = self.mandates.lock().unwrap();
            Ok(mandates
                .iter()
                .filter(|m| m.debtor_account_id() == account_id)
                .cloned()
                .collect())
        }

        async fn find_active_by_creditor(
            &self,
            creditor_id: &str,
        ) -> Result<Vec<DirectDebitMandate>, String> {
            let mandates = self.mandates.lock().unwrap();
            Ok(mandates
                .iter()
                .filter(|m| m.creditor_id() == creditor_id && m.status() == MandateStatus::Active)
                .cloned()
                .collect())
        }

        async fn update(&self, mandate: &DirectDebitMandate) -> Result<(), String> {
            let mut mandates = self.mandates.lock().unwrap();
            if let Some(existing) = mandates.iter_mut().find(|m| m.id() == mandate.id()) {
                *existing = mandate.clone();
            }
            Ok(())
        }
    }

    struct MockDebitExecutionRepo {
        executions: Mutex<Vec<DebitExecution>>,
    }

    impl MockDebitExecutionRepo {
        fn new() -> Self {
            MockDebitExecutionRepo {
                executions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IDebitExecutionRepository for MockDebitExecutionRepo {
        async fn save(&self, execution: &DebitExecution) -> Result<(), String> {
            let mut execs = self.executions.lock().unwrap();
            execs.push(execution.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<DebitExecution>, String> {
            let execs = self.executions.lock().unwrap();
            Ok(execs.iter().find(|e| e.id() == id).cloned())
        }

        async fn find_by_mandate(&self, mandate_id: Uuid) -> Result<Vec<DebitExecution>, String> {
            let execs = self.executions.lock().unwrap();
            Ok(execs
                .iter()
                .filter(|e| e.mandate_id() == mandate_id)
                .cloned()
                .collect())
        }
    }

    fn make_service() -> RecurringPaymentService {
        RecurringPaymentService::new(
            Arc::new(MockStandingOrderRepo::new()),
            Arc::new(MockMandateRepo::new()),
            Arc::new(MockDebitExecutionRepo::new()),
        )
    }

    #[tokio::test]
    async fn test_create_standing_order() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Loyer".to_string(),
            start_date: start,
            end_date: None,
            max_executions: None,
        };

        let result = service.create_standing_order(req).await.unwrap();
        assert_eq!(result.account_id, account_id.to_string());
        assert_eq!(result.status, "Active");
    }

    #[tokio::test]
    async fn test_get_standing_order() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Loyer".to_string(),
            start_date: start,
            end_date: None,
            max_executions: None,
        };

        let created = service.create_standing_order(req).await.unwrap();
        let fetched = service.get_standing_order(&created.id).await.unwrap();
        assert_eq!(fetched.id, created.id);
    }

    #[tokio::test]
    async fn test_suspend_standing_order() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Loyer".to_string(),
            start_date: start,
            end_date: None,
            max_executions: None,
        };

        let created = service.create_standing_order(req).await.unwrap();
        service.suspend_standing_order(&created.id).await.unwrap();

        let fetched = service.get_standing_order(&created.id).await.unwrap();
        assert_eq!(fetched.status, "Suspended");
    }

    #[tokio::test]
    async fn test_resume_standing_order() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Loyer".to_string(),
            start_date: start,
            end_date: None,
            max_executions: None,
        };

        let created = service.create_standing_order(req).await.unwrap();
        service.suspend_standing_order(&created.id).await.unwrap();
        service.resume_standing_order(&created.id).await.unwrap();

        let fetched = service.get_standing_order(&created.id).await.unwrap();
        assert_eq!(fetched.status, "Active");
    }

    #[tokio::test]
    async fn test_create_mandate() {
        let service = make_service();
        let debtor_id = Uuid::new_v4();

        let req = CreateMandateRequest {
            debtor_account_id: debtor_id.to_string(),
            debtor_name: "Ahmed".to_string(),
            creditor_id: "CRED-001".to_string(),
            creditor_name: "Company".to_string(),
            amount_limit: rust_decimal::Decimal::from(200),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Reference".to_string(),
            expires_at: None,
        };

        let result = service.create_mandate(req).await.unwrap();
        assert_eq!(result.debtor_account_id, debtor_id.to_string());
        assert_eq!(result.status, "PendingSignature");
    }

    #[tokio::test]
    async fn test_sign_mandate() {
        let service = make_service();
        let debtor_id = Uuid::new_v4();

        let req = CreateMandateRequest {
            debtor_account_id: debtor_id.to_string(),
            debtor_name: "Ahmed".to_string(),
            creditor_id: "CRED-001".to_string(),
            creditor_name: "Company".to_string(),
            amount_limit: rust_decimal::Decimal::from(200),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Reference".to_string(),
            expires_at: None,
        };

        let created = service.create_mandate(req).await.unwrap();
        service.sign_mandate(&created.id).await.unwrap();

        let fetched = service.list_account_mandates(&debtor_id.to_string()).await;
        assert!(fetched.is_ok());
    }

    #[tokio::test]
    async fn test_execute_due_standing_orders() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let today = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Daily".to_string(),
            reference: "Loyer".to_string(),
            start_date: today,
            end_date: None,
            max_executions: Some(2),
        };

        service.create_standing_order(req).await.unwrap();
        let result = service.execute_due_standing_orders(today).await.unwrap();
        assert_eq!(result.executed, 1);
    }

    #[tokio::test]
    async fn test_list_account_standing_orders() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Loyer".to_string(),
            start_date: start,
            end_date: None,
            max_executions: None,
        };

        service.create_standing_order(req).await.unwrap();
        let list = service
            .list_account_standing_orders(&account_id.to_string())
            .await
            .unwrap();
        assert_eq!(list.total, 1);
    }

    #[tokio::test]
    async fn test_cancel_standing_order() {
        let service = make_service();
        let account_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 10).unwrap();

        let req = CreateStandingOrderRequest {
            account_id: account_id.to_string(),
            beneficiary_account: "TN1234567890".to_string(),
            beneficiary_name: "Ahmed".to_string(),
            amount: rust_decimal::Decimal::from(500),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Loyer".to_string(),
            start_date: start,
            end_date: None,
            max_executions: None,
        };

        let created = service.create_standing_order(req).await.unwrap();
        service.cancel_standing_order(&created.id).await.unwrap();

        let fetched = service.get_standing_order(&created.id).await.unwrap();
        assert_eq!(fetched.status, "Cancelled");
    }

    #[tokio::test]
    async fn test_revoke_mandate() {
        let service = make_service();
        let debtor_id = Uuid::new_v4();

        let req = CreateMandateRequest {
            debtor_account_id: debtor_id.to_string(),
            debtor_name: "Ahmed".to_string(),
            creditor_id: "CRED-001".to_string(),
            creditor_name: "Company".to_string(),
            amount_limit: rust_decimal::Decimal::from(200),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Reference".to_string(),
            expires_at: None,
        };

        let created = service.create_mandate(req).await.unwrap();
        service.sign_mandate(&created.id).await.unwrap();
        service.revoke_mandate(&created.id).await.unwrap();

        let fetched = service
            .list_account_mandates(&debtor_id.to_string())
            .await
            .unwrap();
        assert_eq!(fetched.total, 1);
    }

    #[tokio::test]
    async fn test_list_account_mandates() {
        let service = make_service();
        let debtor_id = Uuid::new_v4();

        let req = CreateMandateRequest {
            debtor_account_id: debtor_id.to_string(),
            debtor_name: "Ahmed".to_string(),
            creditor_id: "CRED-001".to_string(),
            creditor_name: "Company".to_string(),
            amount_limit: rust_decimal::Decimal::from(200),
            currency: Some("TND".to_string()),
            frequency: "Monthly".to_string(),
            reference: "Reference".to_string(),
            expires_at: None,
        };

        service.create_mandate(req).await.unwrap();
        let list = service
            .list_account_mandates(&debtor_id.to_string())
            .await
            .unwrap();
        assert_eq!(list.total, 1);
    }
}
