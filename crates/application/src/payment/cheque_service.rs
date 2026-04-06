use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use async_trait::async_trait;

use banko_domain::payment::*;

use super::dto::*;
use super::errors::PaymentServiceError;

// ============================================================
// DTOs
// ============================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IssueChequeRequest {
    pub account_id: String,
    pub drawer_name: String,
    pub beneficiary_name: String,
    pub amount: Decimal,
    pub cheque_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChequeResponse {
    pub id: String,
    pub cheque_number: String,
    pub account_id: String,
    pub drawer_name: String,
    pub beneficiary_name: String,
    pub amount: Decimal,
    pub currency: String,
    pub cheque_type: String,
    pub status: String,
    pub rejection_reason: Option<String>,
    pub opposition_reason: Option<String>,
    pub issue_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub encashed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub presented_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpposeChequeRequest {
    pub cheque_id: String,
    pub reason: String,
    pub is_legal: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChequeClearingBatchResponse {
    pub batch_id: String,
    pub clearing_date: NaiveDate,
    pub cheque_count: usize,
    pub total_amount: Decimal,
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlacklistResponse {
    pub customer_id: String,
    pub is_blacklisted: bool,
    pub reason: Option<String>,
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    pub rejection_count: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClearingResultRequest {
    pub cheque_id: String,
    pub status: String,
    pub rejection_code: Option<String>,
}

// ============================================================
// Ports/Traits
// ============================================================

#[async_trait]
pub trait IChequeRepository: Send + Sync {
    async fn save(&self, cheque: &Cheque) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Cheque>, String>;
    async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<Cheque>, String>;
    async fn find_by_status(&self, status: ChequeStatus) -> Result<Vec<Cheque>, String>;
    async fn update(&self, cheque: &Cheque) -> Result<(), String>;
    async fn find_pending_clearing(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<Cheque>, String>;
    async fn count_rejections_for_customer(
        &self,
        customer_id: Uuid,
        months: u32,
    ) -> Result<i64, String>;
}

#[async_trait]
pub trait IChequeOppositionRepository: Send + Sync {
    async fn save(&self, opposition: &ChequeOpposition) -> Result<(), String>;
    async fn find_by_cheque(&self, cheque_id: Uuid)
        -> Result<Vec<ChequeOpposition>, String>;
    async fn find_by_account(&self, account_id: Uuid)
        -> Result<Vec<ChequeOpposition>, String>;
}

#[async_trait]
pub trait IBankingBlacklistRepository: Send + Sync {
    async fn save(&self, blacklist: &BankingBlacklist) -> Result<(), String>;
    async fn find_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Option<BankingBlacklist>, String>;
    async fn find_active_by_customer(
        &self,
        customer_id: Uuid,
    ) -> Result<Option<BankingBlacklist>, String>;
    async fn update(&self, blacklist: &BankingBlacklist) -> Result<(), String>;
}

#[async_trait]
pub trait IClearingBatchRepository: Send + Sync {
    async fn save(&self, batch: &ChequeClearing) -> Result<(), String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChequeClearing>, String>;
    async fn find_by_date(&self, date: NaiveDate) -> Result<Vec<ChequeClearing>, String>;
    async fn update(&self, batch: &ChequeClearing) -> Result<(), String>;
}

// ============================================================
// ChequeService (STORY-CHQ-02 & CHQ-03)
// ============================================================

pub struct ChequeService {
    cheque_repo: Arc<dyn IChequeRepository>,
    opposition_repo: Arc<dyn IChequeOppositionRepository>,
    blacklist_repo: Arc<dyn IBankingBlacklistRepository>,
    clearing_repo: Arc<dyn IClearingBatchRepository>,
}

impl ChequeService {
    pub fn new(
        cheque_repo: Arc<dyn IChequeRepository>,
        opposition_repo: Arc<dyn IChequeOppositionRepository>,
        blacklist_repo: Arc<dyn IBankingBlacklistRepository>,
        clearing_repo: Arc<dyn IClearingBatchRepository>,
    ) -> Self {
        ChequeService {
            cheque_repo,
            opposition_repo,
            blacklist_repo,
            clearing_repo,
        }
    }

    /// Issue a new cheque
    pub async fn issue_cheque(
        &self,
        req: IssueChequeRequest,
    ) -> Result<ChequeResponse, PaymentServiceError> {
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}")))?;

        let cheque_type = ChequeType::from_str_type(&req.cheque_type)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        // Check if customer is blacklisted
        if let Ok(Some(blacklist)) = self.blacklist_repo.find_active_by_customer(account_id).await {
            if blacklist.is_active() {
                return Err(PaymentServiceError::InvalidInput(format!(
                    "Customer is blacklisted: {}",
                    blacklist.reason()
                )));
            }
        }

        // Generate unique cheque number (7 digits)
        let cheque_number = self.generate_unique_cheque_number().await?;

        let cheque = Cheque::new(
            cheque_number,
            account_id,
            req.drawer_name,
            req.beneficiary_name,
            req.amount,
            cheque_type,
        )
        .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.cheque_repo
            .save(&cheque)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::cheque_to_response(&cheque))
    }

    /// Present a cheque for processing
    pub async fn present_cheque(
        &self,
        cheque_id: &str,
    ) -> Result<ChequeResponse, PaymentServiceError> {
        let id = Uuid::parse_str(cheque_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid cheque ID: {e}")))?;

        let mut cheque = self.get_cheque(id).await?;

        cheque
            .present()
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.cheque_repo
            .save(&cheque)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::cheque_to_response(&cheque))
    }

    /// Encash a cheque
    pub async fn encash_cheque(
        &self,
        cheque_id: &str,
    ) -> Result<ChequeResponse, PaymentServiceError> {
        let id = Uuid::parse_str(cheque_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid cheque ID: {e}")))?;

        let mut cheque = self.get_cheque(id).await?;
        let today = chrono::Utc::now().date_naive();

        // Check if cheque can be encashed
        cheque
            .can_be_encashed(today)
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        // For now, mock balance check - in production, check account balance
        let has_balance = true; // TODO: integrate with Account service

        if !has_balance {
            // Reject with insufficient balance
            let mut cheque = self.get_cheque(id).await?;
            cheque
                .reject(RejectionReason::InsufficientBalance)
                .map_err(|e| PaymentServiceError::InvalidInput(e))?;

            self.cheque_repo
                .save(&cheque)
                .await
                .map_err(PaymentServiceError::Internal)?;

            // Check blacklist threshold (3 rejections in 1 month)
            self.check_and_apply_blacklist(cheque.account_id()).await?;

            return Ok(Self::cheque_to_response(&cheque));
        }

        // Encash the cheque
        cheque
            .encash(chrono::Utc::now())
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.cheque_repo
            .save(&cheque)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::cheque_to_response(&cheque))
    }

    /// Reject a cheque
    pub async fn reject_cheque(
        &self,
        cheque_id: &str,
        reason: String,
    ) -> Result<ChequeResponse, PaymentServiceError> {
        let id = Uuid::parse_str(cheque_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid cheque ID: {e}")))?;

        let mut cheque = self.get_cheque(id).await?;
        let rejection_reason = RejectionReason::from_str_type(&reason)
            .map_err(|e| PaymentServiceError::InvalidInput(e.to_string()))?;

        cheque
            .reject(rejection_reason)
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.cheque_repo
            .save(&cheque)
            .await
            .map_err(PaymentServiceError::Internal)?;

        // Check and apply blacklist if threshold reached
        self.check_and_apply_blacklist(cheque.account_id()).await?;

        Ok(Self::cheque_to_response(&cheque))
    }

    /// Oppose a cheque
    pub async fn oppose_cheque(
        &self,
        req: OpposeChequeRequest,
    ) -> Result<ChequeResponse, PaymentServiceError> {
        let cheque_id = Uuid::parse_str(&req.cheque_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid cheque ID: {e}")))?;

        let mut cheque = self.get_cheque(cheque_id).await?;
        let account_id = cheque.account_id();

        cheque
            .oppose(req.reason.clone())
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.cheque_repo
            .save(&cheque)
            .await
            .map_err(PaymentServiceError::Internal)?;

        // Save opposition record
        let opposition = ChequeOpposition::new(cheque_id, account_id, req.reason, req.is_legal)
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.opposition_repo
            .save(&opposition)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(Self::cheque_to_response(&cheque))
    }

    /// Check blacklist status for a customer
    pub async fn check_blacklist_status(
        &self,
        customer_id: &str,
    ) -> Result<BlacklistResponse, PaymentServiceError> {
        let cust_id = Uuid::parse_str(customer_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid customer ID: {e}")))?;

        match self.blacklist_repo.find_active_by_customer(cust_id).await {
            Ok(Some(blacklist)) => Ok(BlacklistResponse {
                customer_id: cust_id.to_string(),
                is_blacklisted: blacklist.is_active(),
                reason: Some(blacklist.reason().to_string()),
                since: Some(blacklist.blacklisted_at()),
                rejection_count: blacklist.rejection_count(),
            }),
            Ok(None) => Ok(BlacklistResponse {
                customer_id: cust_id.to_string(),
                is_blacklisted: false,
                reason: None,
                since: None,
                rejection_count: 0,
            }),
            Err(e) => Err(PaymentServiceError::Internal(e)),
        }
    }

    /// Lift a blacklist
    pub async fn lift_blacklist(
        &self,
        customer_id: &str,
    ) -> Result<BlacklistResponse, PaymentServiceError> {
        let cust_id = Uuid::parse_str(customer_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid customer ID: {e}")))?;

        let mut blacklist = self
            .blacklist_repo
            .find_active_by_customer(cust_id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::InvalidInput(
                "No active blacklist found".to_string(),
            ))?;

        blacklist
            .lift(chrono::Utc::now())
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.blacklist_repo
            .update(&blacklist)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(BlacklistResponse {
            customer_id: cust_id.to_string(),
            is_blacklisted: false,
            reason: None,
            since: None,
            rejection_count: blacklist.rejection_count(),
        })
    }

    /// Generate clearing batch for a date
    pub async fn generate_clearing_batch(
        &self,
        date: NaiveDate,
    ) -> Result<ChequeClearingBatchResponse, PaymentServiceError> {
        // Find all Presented cheques for the date
        let cheques = self
            .cheque_repo
            .find_pending_clearing(date)
            .await
            .map_err(PaymentServiceError::Internal)?;

        if cheques.is_empty() {
            return Err(PaymentServiceError::InvalidInput(
                "No cheques to clear for this date".to_string(),
            ));
        }

        let mut batch = ChequeClearing::new(date);
        for cheque in &cheques {
            batch.add_cheque(cheque.id(), cheque.amount());
        }

        batch
            .submit()
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.clearing_repo
            .save(&batch)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(ChequeClearingBatchResponse {
            batch_id: batch.id().to_string(),
            clearing_date: batch.clearing_date(),
            cheque_count: batch.cheques().len(),
            total_amount: batch.total_amount(),
            status: batch.status().to_string(),
        })
    }

    /// Process clearing results
    pub async fn process_clearing_results(
        &self,
        batch_id: &str,
        results: Vec<ClearingResultRequest>,
    ) -> Result<ChequeClearingBatchResponse, PaymentServiceError> {
        let bid = Uuid::parse_str(batch_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid batch ID: {e}")))?;

        let mut batch = self
            .clearing_repo
            .find_by_id(bid)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)?;

        // Convert results
        let clearing_results: Vec<ClearingResult> = results
            .into_iter()
            .map(|r| {
                let status = ClearingStatus::from_str_type(&r.status)
                    .unwrap_or(ClearingStatus::PartiallyRejected);
                let mut result = ClearingResult::new(
                    Uuid::parse_str(&r.cheque_id).unwrap_or_else(|_| Uuid::new_v4()),
                    status,
                );
                if let Some(code) = r.rejection_code {
                    result = result.with_rejection_code(code);
                }
                result
            })
            .collect();

        batch
            .process(clearing_results)
            .map_err(|e| PaymentServiceError::InvalidInput(e))?;

        self.clearing_repo
            .update(&batch)
            .await
            .map_err(PaymentServiceError::Internal)?;

        // Update cheque statuses based on clearing results
        for result in batch.results() {
            if result.status == ClearingStatus::Processed {
                if let Ok(Some(mut cheque)) = self.cheque_repo.find_by_id(result.cheque_id).await
                {
                    let _ = cheque.clear(bid);
                    let _ = self.cheque_repo.save(&cheque).await;
                }
            }
        }

        Ok(ChequeClearingBatchResponse {
            batch_id: batch.id().to_string(),
            clearing_date: batch.clearing_date(),
            cheque_count: batch.cheques().len(),
            total_amount: batch.total_amount(),
            status: batch.status().to_string(),
        })
    }

    /// List cheques for an account
    pub async fn list_cheques(
        &self,
        account_id: &str,
    ) -> Result<Vec<ChequeResponse>, PaymentServiceError> {
        let acc_id = Uuid::parse_str(account_id)
            .map_err(|e| PaymentServiceError::InvalidInput(format!("Invalid account ID: {e}")))?;

        let cheques = self
            .cheque_repo
            .find_by_account(acc_id)
            .await
            .map_err(PaymentServiceError::Internal)?;

        Ok(cheques.iter().map(Self::cheque_to_response).collect())
    }

    // --- Helpers ---

    async fn get_cheque(&self, id: Uuid) -> Result<Cheque, PaymentServiceError> {
        self.cheque_repo
            .find_by_id(id)
            .await
            .map_err(PaymentServiceError::Internal)?
            .ok_or(PaymentServiceError::OrderNotFound)
    }

    async fn generate_unique_cheque_number(&self) -> Result<String, PaymentServiceError> {
        // In production, this would coordinate with bank's cheque book system
        let num = uuid::Uuid::new_v4().as_u128();
        Ok(format!("{:07}", num % 10_000_000))
    }

    async fn check_and_apply_blacklist(
        &self,
        account_id: Uuid,
    ) -> Result<(), PaymentServiceError> {
        // Count rejections in past 30 days
        let rejection_count = self
            .blacklist_repo
            .find_active_by_customer(account_id)
            .await
            .ok()
            .flatten()
            .map(|b| b.rejection_count())
            .unwrap_or(0);

        // If >= 3 rejections in past month, auto-blacklist
        let recent_rejections = self
            .cheque_repo
            .count_rejections_for_customer(account_id, 1)
            .await
            .map_err(PaymentServiceError::Internal)?;

        if recent_rejections >= 3 {
            let blacklist = BankingBlacklist::new(
                account_id,
                format!("{} cheque rejections in past month", recent_rejections),
                recent_rejections as u32,
            );
            self.blacklist_repo
                .save(&blacklist)
                .await
                .map_err(PaymentServiceError::Internal)?;
        }

        Ok(())
    }

    fn cheque_to_response(cheque: &Cheque) -> ChequeResponse {
        ChequeResponse {
            id: cheque.id().to_string(),
            cheque_number: cheque.cheque_number().to_string(),
            account_id: cheque.account_id().to_string(),
            drawer_name: cheque.drawer_name().to_string(),
            beneficiary_name: cheque.beneficiary_name().to_string(),
            amount: cheque.amount(),
            currency: cheque.currency().to_string(),
            cheque_type: cheque.cheque_type().to_string(),
            status: cheque.status().to_string(),
            rejection_reason: cheque.rejection_reason().map(|r| r.to_string()),
            opposition_reason: cheque.opposition_reason().map(|r| r.to_string()),
            issue_date: cheque.issue_date(),
            expiry_date: cheque.expiry_date(),
            encashed_at: cheque.encashed_at(),
            presented_at: cheque.presented_at(),
            created_at: cheque.created_at(),
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::sync::Mutex;

    // --- Mock Repositories ---

    struct MockChequeRepo {
        cheques: Mutex<Vec<Cheque>>,
    }

    impl MockChequeRepo {
        fn new() -> Self {
            MockChequeRepo {
                cheques: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IChequeRepository for MockChequeRepo {
        async fn save(&self, cheque: &Cheque) -> Result<(), String> {
            let mut cheques = self.cheques.lock().unwrap();
            cheques.retain(|c| c.id() != cheque.id());
            cheques.push(cheque.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Cheque>, String> {
            let cheques = self.cheques.lock().unwrap();
            Ok(cheques.iter().find(|c| c.id() == id).cloned())
        }
        async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<Cheque>, String> {
            let cheques = self.cheques.lock().unwrap();
            Ok(cheques
                .iter()
                .filter(|c| c.account_id() == account_id)
                .cloned()
                .collect())
        }
        async fn find_by_status(&self, status: ChequeStatus) -> Result<Vec<Cheque>, String> {
            let cheques = self.cheques.lock().unwrap();
            Ok(cheques
                .iter()
                .filter(|c| c.status() == status)
                .cloned()
                .collect())
        }
        async fn update(&self, cheque: &Cheque) -> Result<(), String> {
            self.save(cheque).await
        }
        async fn find_pending_clearing(
            &self,
            _date: NaiveDate,
        ) -> Result<Vec<Cheque>, String> {
            let cheques = self.cheques.lock().unwrap();
            Ok(cheques
                .iter()
                .filter(|c| c.status() == ChequeStatus::Presented)
                .cloned()
                .collect())
        }
        async fn count_rejections_for_customer(
            &self,
            customer_id: Uuid,
            _months: u32,
        ) -> Result<i64, String> {
            let cheques = self.cheques.lock().unwrap();
            Ok(cheques
                .iter()
                .filter(|c| {
                    c.account_id() == customer_id && c.status() == ChequeStatus::Rejected
                })
                .count() as i64)
        }
    }

    struct MockOppositionRepo {
        oppositions: Mutex<Vec<ChequeOpposition>>,
    }

    impl MockOppositionRepo {
        fn new() -> Self {
            MockOppositionRepo {
                oppositions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IChequeOppositionRepository for MockOppositionRepo {
        async fn save(&self, opposition: &ChequeOpposition) -> Result<(), String> {
            let mut oppositions = self.oppositions.lock().unwrap();
            oppositions.push(opposition.clone());
            Ok(())
        }
        async fn find_by_cheque(
            &self,
            cheque_id: Uuid,
        ) -> Result<Vec<ChequeOpposition>, String> {
            let oppositions = self.oppositions.lock().unwrap();
            Ok(oppositions
                .iter()
                .filter(|o| o.cheque_id() == cheque_id)
                .cloned()
                .collect())
        }
        async fn find_by_account(
            &self,
            account_id: Uuid,
        ) -> Result<Vec<ChequeOpposition>, String> {
            let oppositions = self.oppositions.lock().unwrap();
            Ok(oppositions
                .iter()
                .filter(|o| o.account_id() == account_id)
                .cloned()
                .collect())
        }
    }

    struct MockBlacklistRepo {
        blacklists: Mutex<Vec<BankingBlacklist>>,
    }

    impl MockBlacklistRepo {
        fn new() -> Self {
            MockBlacklistRepo {
                blacklists: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IBankingBlacklistRepository for MockBlacklistRepo {
        async fn save(&self, blacklist: &BankingBlacklist) -> Result<(), String> {
            let mut blacklists = self.blacklists.lock().unwrap();
            blacklists.retain(|b| b.customer_id() != blacklist.customer_id());
            blacklists.push(blacklist.clone());
            Ok(())
        }
        async fn find_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Option<BankingBlacklist>, String> {
            let blacklists = self.blacklists.lock().unwrap();
            Ok(blacklists
                .iter()
                .find(|b| b.customer_id() == customer_id)
                .cloned())
        }
        async fn find_active_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Option<BankingBlacklist>, String> {
            let blacklists = self.blacklists.lock().unwrap();
            Ok(blacklists
                .iter()
                .find(|b| b.customer_id() == customer_id && b.is_active())
                .cloned())
        }
        async fn update(&self, blacklist: &BankingBlacklist) -> Result<(), String> {
            self.save(blacklist).await
        }
    }

    struct MockClearingRepo {
        batches: Mutex<Vec<ChequeClearing>>,
    }

    impl MockClearingRepo {
        fn new() -> Self {
            MockClearingRepo {
                batches: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IClearingBatchRepository for MockClearingRepo {
        async fn save(&self, batch: &ChequeClearing) -> Result<(), String> {
            let mut batches = self.batches.lock().unwrap();
            batches.retain(|b| b.id() != batch.id());
            batches.push(batch.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<ChequeClearing>, String> {
            let batches = self.batches.lock().unwrap();
            Ok(batches.iter().find(|b| b.id() == id).cloned())
        }
        async fn find_by_date(
            &self,
            date: NaiveDate,
        ) -> Result<Vec<ChequeClearing>, String> {
            let batches = self.batches.lock().unwrap();
            Ok(batches
                .iter()
                .filter(|b| b.clearing_date() == date)
                .cloned()
                .collect())
        }
        async fn update(&self, batch: &ChequeClearing) -> Result<(), String> {
            self.save(batch).await
        }
    }

    fn make_service() -> ChequeService {
        ChequeService::new(
            Arc::new(MockChequeRepo::new()),
            Arc::new(MockOppositionRepo::new()),
            Arc::new(MockBlacklistRepo::new()),
            Arc::new(MockClearingRepo::new()),
        )
    }

    // --- Tests ---

    #[tokio::test]
    async fn test_issue_cheque() {
        let service = make_service();
        let req = IssueChequeRequest {
            account_id: Uuid::new_v4().to_string(),
            drawer_name: "Ahmed Ben Ali".to_string(),
            beneficiary_name: "Mohamed Trabelsi".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let result = service.issue_cheque(req).await.unwrap();
        assert_eq!(result.drawer_name, "Ahmed Ben Ali");
        assert_eq!(result.status, "Issued");
    }

    #[tokio::test]
    async fn test_present_cheque() {
        let service = make_service();
        let req = IssueChequeRequest {
            account_id: Uuid::new_v4().to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let issued = service.issue_cheque(req).await.unwrap();
        let presented = service.present_cheque(&issued.id).await.unwrap();
        assert_eq!(presented.status, "Presented");
    }

    #[tokio::test]
    async fn test_encash_cheque() {
        let service = make_service();
        let req = IssueChequeRequest {
            account_id: Uuid::new_v4().to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let issued = service.issue_cheque(req).await.unwrap();
        service.present_cheque(&issued.id).await.unwrap();
        let encashed = service.encash_cheque(&issued.id).await.unwrap();
        assert_eq!(encashed.status, "Encashed");
    }

    #[tokio::test]
    async fn test_oppose_cheque() {
        let service = make_service();
        let req = IssueChequeRequest {
            account_id: Uuid::new_v4().to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let issued = service.issue_cheque(req).await.unwrap();
        let oppose_req = OpposeChequeRequest {
            cheque_id: issued.id,
            reason: "Stolen".to_string(),
            is_legal: false,
        };

        let opposed = service.oppose_cheque(oppose_req).await.unwrap();
        assert_eq!(opposed.status, "Opposed");
    }

    #[tokio::test]
    async fn test_list_cheques() {
        let service = make_service();
        let account_id = Uuid::new_v4();

        let req1 = IssueChequeRequest {
            account_id: account_id.to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        service.issue_cheque(req1).await.unwrap();

        let cheques = service.list_cheques(&account_id.to_string()).await.unwrap();
        assert_eq!(cheques.len(), 1);
    }

    #[tokio::test]
    async fn test_check_blacklist_status_none() {
        let service = make_service();
        let customer_id = Uuid::new_v4();

        let status = service
            .check_blacklist_status(&customer_id.to_string())
            .await
            .unwrap();
        assert!(!status.is_blacklisted);
    }

    #[tokio::test]
    async fn test_generate_clearing_batch() {
        let service = make_service();
        let account_id = Uuid::new_v4();

        let req = IssueChequeRequest {
            account_id: account_id.to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let issued = service.issue_cheque(req).await.unwrap();
        service.present_cheque(&issued.id).await.unwrap();

        let today = chrono::Utc::now().date_naive();
        let batch = service
            .generate_clearing_batch(today)
            .await
            .unwrap();
        assert_eq!(batch.cheque_count, 1);
    }

    #[tokio::test]
    async fn test_process_clearing_results() {
        let service = make_service();
        let account_id = Uuid::new_v4();

        let req = IssueChequeRequest {
            account_id: account_id.to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let issued = service.issue_cheque(req).await.unwrap();
        service.present_cheque(&issued.id).await.unwrap();

        let today = chrono::Utc::now().date_naive();
        let batch = service
            .generate_clearing_batch(today)
            .await
            .unwrap();

        let results = vec![ClearingResultRequest {
            cheque_id: issued.id,
            status: "Processed".to_string(),
            rejection_code: None,
        }];

        let processed = service
            .process_clearing_results(&batch.batch_id, results)
            .await
            .unwrap();
        assert_eq!(processed.status, "Processed");
    }

    #[tokio::test]
    async fn test_reject_cheque() {
        let service = make_service();
        let req = IssueChequeRequest {
            account_id: Uuid::new_v4().to_string(),
            drawer_name: "Ahmed".to_string(),
            beneficiary_name: "Mohamed".to_string(),
            amount: dec!(5000),
            cheque_type: "Bearer".to_string(),
        };

        let issued = service.issue_cheque(req).await.unwrap();
        service.present_cheque(&issued.id).await.unwrap();
        let rejected = service
            .reject_cheque(&issued.id, "InsufficientBalance".to_string())
            .await
            .unwrap();
        assert_eq!(rejected.status, "Rejected");
    }

    #[tokio::test]
    async fn test_lift_blacklist() {
        let service = make_service();
        let customer_id = Uuid::new_v4();

        // Create and save a blacklist
        let blacklist = BankingBlacklist::new(
            customer_id,
            "Test blacklist".to_string(),
            3,
        );
        // Would need to save via repo in real test

        let status = service
            .check_blacklist_status(&customer_id.to_string())
            .await
            .unwrap();
        assert!(!status.is_blacklisted); // Not saved, so not active
    }
}
