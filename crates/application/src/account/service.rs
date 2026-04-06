use std::sync::Arc;

use chrono::{DateTime, Utc};

use banko_domain::account::{Account, AccountId, AccountType, Movement};
use banko_domain::shared::{CustomerId, Money, Rib};

use super::dto::{AccountResponse, MovementResponse, StatementResponse};
use super::errors::AccountServiceError;
use super::ports::{IAccountRepository, IKycVerifier};

pub struct AccountService {
    account_repo: Arc<dyn IAccountRepository>,
    kyc_verifier: Arc<dyn IKycVerifier>,
}

impl AccountService {
    pub fn new(
        account_repo: Arc<dyn IAccountRepository>,
        kyc_verifier: Arc<dyn IKycVerifier>,
    ) -> Self {
        AccountService {
            account_repo,
            kyc_verifier,
        }
    }

    /// Open a new account for a customer.
    pub async fn open_account(
        &self,
        customer_id: CustomerId,
        account_type: AccountType,
    ) -> Result<Account, AccountServiceError> {
        // Check KYC via verifier port
        let kyc_validated = self
            .kyc_verifier
            .is_kyc_validated(&customer_id)
            .await
            .map_err(AccountServiceError::Internal)?;

        // Generate a random RIB
        let rib = Self::generate_rib()?;

        // Create account (domain enforces KYC invariant)
        let account =
            Account::new(customer_id, rib, account_type, kyc_validated).map_err(|e| match e {
                banko_domain::shared::DomainError::KycNotValidated => {
                    AccountServiceError::KycNotValidated
                }
                other => AccountServiceError::DomainError(other.to_string()),
            })?;

        // Persist
        self.account_repo
            .save(&account)
            .await
            .map_err(AccountServiceError::Internal)?;

        Ok(account)
    }

    /// Find an account by its ID.
    pub async fn find_by_id(&self, id: &AccountId) -> Result<Account, AccountServiceError> {
        self.account_repo
            .find_by_id(id)
            .await
            .map_err(AccountServiceError::Internal)?
            .ok_or(AccountServiceError::AccountNotFound)
    }

    /// List accounts for a customer.
    pub async fn list_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<Account>, AccountServiceError> {
        self.account_repo
            .find_by_customer_id(customer_id)
            .await
            .map_err(AccountServiceError::Internal)
    }

    /// Deposit money into an account.
    pub async fn deposit(
        &self,
        account_id: &AccountId,
        amount: Money,
        description: &str,
    ) -> Result<Movement, AccountServiceError> {
        let mut account = self.find_by_id(account_id).await?;

        let movement = account.deposit(amount, description).map_err(|e| match e {
            banko_domain::shared::DomainError::AccountClosed => AccountServiceError::AccountClosed,
            banko_domain::shared::DomainError::AccountSuspended => {
                AccountServiceError::AccountSuspended
            }
            other => AccountServiceError::DomainError(other.to_string()),
        })?;

        self.account_repo
            .save(&account)
            .await
            .map_err(AccountServiceError::Internal)?;
        self.account_repo
            .save_movement(&movement)
            .await
            .map_err(AccountServiceError::Internal)?;

        Ok(movement)
    }

    /// Withdraw money from an account.
    pub async fn withdraw(
        &self,
        account_id: &AccountId,
        amount: Money,
        description: &str,
    ) -> Result<Movement, AccountServiceError> {
        let mut account = self.find_by_id(account_id).await?;

        let movement = account.withdraw(amount, description).map_err(|e| match e {
            banko_domain::shared::DomainError::InsufficientFunds => {
                AccountServiceError::InsufficientFunds
            }
            banko_domain::shared::DomainError::AccountClosed => AccountServiceError::AccountClosed,
            banko_domain::shared::DomainError::AccountSuspended => {
                AccountServiceError::AccountSuspended
            }
            other => AccountServiceError::DomainError(other.to_string()),
        })?;

        self.account_repo
            .save(&account)
            .await
            .map_err(AccountServiceError::Internal)?;
        self.account_repo
            .save_movement(&movement)
            .await
            .map_err(AccountServiceError::Internal)?;

        Ok(movement)
    }

    /// Close an account.
    pub async fn close_account(&self, account_id: &AccountId) -> Result<(), AccountServiceError> {
        let mut account = self.find_by_id(account_id).await?;

        account
            .close()
            .map_err(|e| AccountServiceError::DomainError(e.to_string()))?;

        self.account_repo
            .save(&account)
            .await
            .map_err(AccountServiceError::Internal)?;

        Ok(())
    }

    /// Freeze (suspend) an account.
    pub async fn freeze_account(&self, account_id: &AccountId) -> Result<(), AccountServiceError> {
        let mut account = self.find_by_id(account_id).await?;
        account.freeze();
        self.account_repo
            .save(&account)
            .await
            .map_err(AccountServiceError::Internal)?;
        Ok(())
    }

    /// Unfreeze (reactivate) an account.
    pub async fn unfreeze_account(
        &self,
        account_id: &AccountId,
    ) -> Result<(), AccountServiceError> {
        let mut account = self.find_by_id(account_id).await?;
        account.unfreeze();
        self.account_repo
            .save(&account)
            .await
            .map_err(AccountServiceError::Internal)?;
        Ok(())
    }

    /// Get account statement for a period.
    pub async fn get_statement(
        &self,
        account_id: &AccountId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<StatementResponse, AccountServiceError> {
        let account = self.find_by_id(account_id).await?;

        let movements = self
            .account_repo
            .find_movements_by_account_and_period(account_id, from, to)
            .await
            .map_err(AccountServiceError::Internal)?;

        let opening_balance = if let Some(first) = movements.first() {
            // Opening balance = first movement's balance_after minus/plus first movement's amount
            match first.movement_type() {
                banko_domain::account::MovementType::Deposit => {
                    first.balance_after().amount() - first.amount().amount()
                }
                banko_domain::account::MovementType::Withdrawal => {
                    first.balance_after().amount() + first.amount().amount()
                }
            }
        } else {
            account.balance().amount()
        };

        let closing_balance = movements
            .last()
            .map(|m| m.balance_after().amount())
            .unwrap_or(account.balance().amount());

        let movement_responses: Vec<MovementResponse> =
            movements.iter().map(Self::movement_to_response).collect();

        Ok(StatementResponse {
            account_id: account.id().to_string(),
            rib: account.rib().as_str().to_string(),
            period_from: from,
            period_to: to,
            opening_balance,
            closing_balance,
            currency: account.balance().currency().to_string(),
            movements: movement_responses,
        })
    }

    /// Get movements for an account with a limit.
    pub async fn list_movements(
        &self,
        account_id: &AccountId,
        limit: i64,
    ) -> Result<Vec<Movement>, AccountServiceError> {
        // Verify account exists
        let _account = self.find_by_id(account_id).await?;

        self.account_repo
            .find_movements_by_account(account_id, limit)
            .await
            .map_err(AccountServiceError::Internal)
    }

    // --- Helpers ---

    fn generate_rib() -> Result<Rib, AccountServiceError> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut digits = String::with_capacity(20);
        // Bank code: "01"
        digits.push_str("01");
        // Branch code: "001"
        digits.push_str("001");
        // 13 random digits
        for _ in 0..13 {
            digits.push_str(&rng.gen_range(0..10).to_string());
        }
        // Check digits placeholder: "00"
        digits.push_str("00");

        Rib::new(&digits).map_err(|e| AccountServiceError::Internal(e.to_string()))
    }

    pub fn account_to_response(account: &Account) -> AccountResponse {
        AccountResponse {
            id: account.id().to_string(),
            customer_id: account.customer_id().to_string(),
            rib: account.rib().as_str().to_string(),
            account_type: account.account_type().as_str().to_string(),
            balance: account.balance().amount(),
            available_balance: account.available_balance().amount(),
            currency: account.balance().currency().to_string(),
            status: account.status().as_str().to_string(),
            created_at: account.created_at(),
            updated_at: account.updated_at(),
        }
    }

    pub fn movement_to_response(movement: &Movement) -> MovementResponse {
        MovementResponse {
            id: movement.id().to_string(),
            account_id: movement.account_id().to_string(),
            movement_type: movement.movement_type().as_str().to_string(),
            amount: movement.amount().amount(),
            balance_after: movement.balance_after().amount(),
            currency: movement.amount().currency().to_string(),
            description: movement.description().to_string(),
            created_at: movement.created_at(),
        }
    }

    /// Export statement as CSV format.
    /// Returns a CSV string with header and one row per movement, plus opening/closing balance rows.
    /// Format: "Date,Type,Description,Debit,Credit,Balance"
    pub async fn export_statement_csv(
        &self,
        account_id: &AccountId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<String, AccountServiceError> {
        let statement = self.get_statement(account_id, from, to).await?;

        let mut csv = String::from("Date,Type,Description,Debit,Credit,Balance\n");

        // Opening balance row
        csv.push_str(&format!(
            "{},{},{},{},{:.2},{:.2}\n",
            from.map(|d| d.to_rfc3339())
                .unwrap_or_else(|| "Start".to_string()),
            "Opening",
            "Opening Balance",
            "",
            "",
            statement.opening_balance
        ));

        // Movement rows
        for movement in &statement.movements {
            let (debit, credit) = if movement.movement_type == "Withdrawal" {
                (format!("{:.2}", movement.amount), String::new())
            } else {
                (String::new(), format!("{:.2}", movement.amount))
            };

            csv.push_str(&format!(
                "{},{},{},{},{},{:.2}\n",
                movement.created_at.to_rfc3339(),
                movement.movement_type,
                movement.description,
                debit,
                credit,
                movement.balance_after
            ));
        }

        // Closing balance row
        csv.push_str(&format!(
            "{},{},{},{},{:.2},{:.2}\n",
            to.map(|d| d.to_rfc3339())
                .unwrap_or_else(|| "End".to_string()),
            "Closing",
            "Closing Balance",
            "",
            "",
            statement.closing_balance
        ));

        Ok(csv)
    }

    /// Export statement as JSON format.
    /// Returns the complete statement serialized as a JSON string.
    pub async fn export_statement_json(
        &self,
        account_id: &AccountId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<String, AccountServiceError> {
        let statement = self.get_statement(account_id, from, to).await?;
        serde_json::to_string(&statement)
            .map_err(|e| AccountServiceError::Internal(format!("JSON serialization error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::{DateTime, Utc};

    use banko_domain::account::{Account, AccountId, AccountStatus, Movement};
    use banko_domain::shared::{Currency, CustomerId, Money, Rib};

    use super::*;

    // --- Mock Account Repository ---

    struct MockAccountRepository {
        accounts: Mutex<Vec<Account>>,
        movements: Mutex<Vec<Movement>>,
    }

    impl MockAccountRepository {
        fn new() -> Self {
            MockAccountRepository {
                accounts: Mutex::new(Vec::new()),
                movements: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAccountRepository for MockAccountRepository {
        async fn save(&self, account: &Account) -> Result<(), String> {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.retain(|a| a.id() != account.id());
            accounts.push(account.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts.iter().find(|a| a.id() == id).cloned())
        }

        async fn find_by_customer_id(
            &self,
            customer_id: &CustomerId,
        ) -> Result<Vec<Account>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts
                .iter()
                .filter(|a| a.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn find_by_rib(&self, rib: &Rib) -> Result<Option<Account>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts.iter().find(|a| a.rib() == rib).cloned())
        }

        async fn save_movement(&self, movement: &Movement) -> Result<(), String> {
            let mut movements = self.movements.lock().unwrap();
            movements.push(movement.clone());
            Ok(())
        }

        async fn find_movements_by_account(
            &self,
            account_id: &AccountId,
            limit: i64,
        ) -> Result<Vec<Movement>, String> {
            let movements = self.movements.lock().unwrap();
            Ok(movements
                .iter()
                .filter(|m| m.account_id() == account_id)
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn find_movements_by_account_and_period(
            &self,
            account_id: &AccountId,
            from: Option<DateTime<Utc>>,
            to: Option<DateTime<Utc>>,
        ) -> Result<Vec<Movement>, String> {
            let movements = self.movements.lock().unwrap();
            Ok(movements
                .iter()
                .filter(|m| {
                    m.account_id() == account_id
                        && from.is_none_or(|f| m.created_at() >= f)
                        && to.is_none_or(|t| m.created_at() <= t)
                })
                .cloned()
                .collect())
        }

        async fn delete(&self, id: &AccountId) -> Result<(), String> {
            let mut accounts = self.accounts.lock().unwrap();
            accounts.retain(|a| a.id() != id);
            Ok(())
        }
    }

    // --- Mock KYC Verifier ---

    struct MockKycVerifier {
        validated: bool,
    }

    impl MockKycVerifier {
        fn new(validated: bool) -> Self {
            MockKycVerifier { validated }
        }
    }

    #[async_trait]
    impl IKycVerifier for MockKycVerifier {
        async fn is_kyc_validated(&self, _customer_id: &CustomerId) -> Result<bool, String> {
            Ok(self.validated)
        }
    }

    fn make_service(kyc_validated: bool) -> AccountService {
        AccountService::new(
            Arc::new(MockAccountRepository::new()),
            Arc::new(MockKycVerifier::new(kyc_validated)),
        )
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    // --- Open account ---

    #[tokio::test]
    async fn test_open_account_success() {
        let service = make_service(true);
        let customer_id = CustomerId::new();
        let result = service
            .open_account(customer_id.clone(), AccountType::Current)
            .await;
        assert!(result.is_ok());
        let account = result.unwrap();
        assert_eq!(account.customer_id(), &customer_id);
        assert_eq!(account.account_type(), AccountType::Current);
        assert_eq!(account.status(), AccountStatus::Active);
    }

    #[tokio::test]
    async fn test_open_account_kyc_not_validated() {
        let service = make_service(false);
        let customer_id = CustomerId::new();
        let result = service
            .open_account(customer_id, AccountType::Current)
            .await;
        assert!(matches!(result, Err(AccountServiceError::KycNotValidated)));
    }

    // --- Find by ID ---

    #[tokio::test]
    async fn test_find_by_id_success() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        let found = service.find_by_id(account.id()).await.unwrap();
        assert_eq!(found.id(), account.id());
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let service = make_service(true);
        let result = service.find_by_id(&AccountId::new()).await;
        assert!(matches!(result, Err(AccountServiceError::AccountNotFound)));
    }

    // --- List by customer ---

    #[tokio::test]
    async fn test_list_by_customer() {
        let service = make_service(true);
        let customer_id = CustomerId::new();
        service
            .open_account(customer_id.clone(), AccountType::Current)
            .await
            .unwrap();
        service
            .open_account(customer_id.clone(), AccountType::Savings)
            .await
            .unwrap();
        let accounts = service.list_by_customer(&customer_id).await.unwrap();
        assert_eq!(accounts.len(), 2);
    }

    // --- Deposit ---

    #[tokio::test]
    async fn test_deposit_success() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        let movement = service
            .deposit(account.id(), tnd(1000.0), "Test deposit")
            .await
            .unwrap();
        assert_eq!(movement.amount().amount(), 1000.0);

        let updated = service.find_by_id(account.id()).await.unwrap();
        assert_eq!(updated.balance().amount(), 1000.0);
    }

    // --- Withdraw ---

    #[tokio::test]
    async fn test_withdraw_success() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(1000.0), "Deposit")
            .await
            .unwrap();
        let movement = service
            .withdraw(account.id(), tnd(400.0), "Withdrawal")
            .await
            .unwrap();
        assert_eq!(movement.amount().amount(), 400.0);

        let updated = service.find_by_id(account.id()).await.unwrap();
        assert_eq!(updated.balance().amount(), 600.0);
    }

    #[tokio::test]
    async fn test_withdraw_insufficient_funds() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(100.0), "Deposit")
            .await
            .unwrap();
        let result = service
            .withdraw(account.id(), tnd(200.0), "Overdraft")
            .await;
        assert!(matches!(
            result,
            Err(AccountServiceError::InsufficientFunds)
        ));
    }

    // --- Close ---

    #[tokio::test]
    async fn test_close_account_success() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        let result = service.close_account(account.id()).await;
        assert!(result.is_ok());

        let closed = service.find_by_id(account.id()).await.unwrap();
        assert_eq!(closed.status(), AccountStatus::Closed);
    }

    #[tokio::test]
    async fn test_close_account_non_zero_balance() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(100.0), "Deposit")
            .await
            .unwrap();
        let result = service.close_account(account.id()).await;
        assert!(matches!(result, Err(AccountServiceError::DomainError(_))));
    }

    // --- Freeze / Unfreeze ---

    #[tokio::test]
    async fn test_freeze_account() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service.freeze_account(account.id()).await.unwrap();

        let frozen = service.find_by_id(account.id()).await.unwrap();
        assert_eq!(frozen.status(), AccountStatus::Suspended);
    }

    #[tokio::test]
    async fn test_unfreeze_account() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(1000.0), "Deposit")
            .await
            .unwrap();
        service.freeze_account(account.id()).await.unwrap();
        service.unfreeze_account(account.id()).await.unwrap();

        let unfrozen = service.find_by_id(account.id()).await.unwrap();
        assert_eq!(unfrozen.status(), AccountStatus::Active);
        assert_eq!(unfrozen.available_balance().amount(), 1000.0);
    }

    // --- Statement ---

    #[tokio::test]
    async fn test_get_statement() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(1000.0), "Deposit 1")
            .await
            .unwrap();
        service
            .withdraw(account.id(), tnd(200.0), "Withdrawal 1")
            .await
            .unwrap();

        let statement = service
            .get_statement(account.id(), None, None)
            .await
            .unwrap();
        assert_eq!(statement.movements.len(), 2);
        assert_eq!(statement.account_id, account.id().to_string());
    }

    // --- List movements ---

    #[tokio::test]
    async fn test_list_movements() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(500.0), "Dep1")
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(300.0), "Dep2")
            .await
            .unwrap();

        let movements = service.list_movements(account.id(), 10).await.unwrap();
        assert_eq!(movements.len(), 2);
    }

    // --- CSV Export ---

    #[tokio::test]
    async fn test_export_statement_csv() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(1000.0), "Deposit 1")
            .await
            .unwrap();
        service
            .withdraw(account.id(), tnd(200.0), "Withdrawal 1")
            .await
            .unwrap();

        let csv = service
            .export_statement_csv(account.id(), None, None)
            .await
            .unwrap();

        assert!(csv.contains("Date,Type,Description,Debit,Credit,Balance"));
        assert!(csv.contains("Opening"));
        assert!(csv.contains("Closing"));
        assert!(csv.contains("Deposit 1"));
        assert!(csv.contains("Withdrawal 1"));
        assert!(csv.contains("1000.00"));
        assert!(csv.contains("200.00"));
    }

    // --- JSON Export ---

    #[tokio::test]
    async fn test_export_statement_json() {
        let service = make_service(true);
        let account = service
            .open_account(CustomerId::new(), AccountType::Current)
            .await
            .unwrap();
        service
            .deposit(account.id(), tnd(500.0), "Deposit")
            .await
            .unwrap();

        let json = service
            .export_statement_json(account.id(), None, None)
            .await
            .unwrap();

        assert!(json.contains("account_id"));
        assert!(json.contains("opening_balance"));
        assert!(json.contains("closing_balance"));
        assert!(json.contains("movements"));

        // Parse JSON to verify it's valid
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["account_id"].is_string());
        assert!(parsed["movements"].is_array());
    }
}
