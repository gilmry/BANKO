use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::account::IAccountRepository;
use banko_domain::account::{
    Account, AccountId, AccountStatus, AccountType, Movement, MovementId, MovementType,
};
use banko_domain::shared::{Currency, CustomerId, Money, Rib};

pub struct PgAccountRepository {
    pool: PgPool,
}

impl PgAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        PgAccountRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct AccountRow {
    id: Uuid,
    customer_id: Uuid,
    rib: String,
    account_type: String,
    balance: i64,
    available_balance: i64,
    currency: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl AccountRow {
    fn into_domain(self) -> Result<Account, String> {
        let id = AccountId::from_uuid(self.id);
        let customer_id = CustomerId::from_uuid(self.customer_id);
        let rib = Rib::new(&self.rib).map_err(|e| e.to_string())?;
        let account_type =
            AccountType::from_str_type(&self.account_type).map_err(|e| e.to_string())?;
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let balance = Money::from_cents(self.balance, currency);
        let available_balance = Money::from_cents(self.available_balance, currency);
        let status =
            AccountStatus::from_str_status(&self.status).map_err(|e| e.to_string())?;

        Ok(Account::reconstitute(
            id,
            customer_id,
            rib,
            account_type,
            balance,
            available_balance,
            status,
            vec![],
            self.created_at,
            self.updated_at,
        ))
    }
}

#[derive(Debug, sqlx::FromRow)]
struct MovementRow {
    id: Uuid,
    account_id: Uuid,
    movement_type: String,
    amount: i64,
    balance_after: i64,
    currency: String,
    description: String,
    created_at: DateTime<Utc>,
}

impl MovementRow {
    fn into_domain(self) -> Result<Movement, String> {
        let id = MovementId::from_uuid(self.id);
        let account_id = AccountId::from_uuid(self.account_id);
        let movement_type =
            MovementType::from_str_type(&self.movement_type).map_err(|e| e.to_string())?;
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let amount = Money::from_cents(self.amount, currency);
        let balance_after = Money::from_cents(self.balance_after, currency);

        Ok(Movement::reconstitute(
            id,
            account_id,
            movement_type,
            amount,
            balance_after,
            self.description,
            self.created_at,
        ))
    }
}

#[async_trait]
impl IAccountRepository for PgAccountRepository {
    async fn save(&self, account: &Account) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO account.accounts (id, customer_id, rib, account_type, balance, available_balance, currency, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                balance = EXCLUDED.balance,
                available_balance = EXCLUDED.available_balance,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(account.id().as_uuid())
        .bind(account.customer_id().as_uuid())
        .bind(account.rib().as_str())
        .bind(account.account_type().as_str())
        .bind(account.balance().amount_cents())
        .bind(account.available_balance().amount_cents())
        .bind(account.balance().currency().to_string())
        .bind(account.status().as_str())
        .bind(account.created_at())
        .bind(account.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save account error: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, String> {
        let row: Option<AccountRow> = sqlx::query_as(
            "SELECT id, customer_id, rib, account_type, balance, available_balance, currency, status, created_at, updated_at FROM account.accounts WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<Account>, String> {
        let rows: Vec<AccountRow> = sqlx::query_as(
            "SELECT id, customer_id, rib, account_type, balance, available_balance, currency, status, created_at, updated_at FROM account.accounts WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_customer_id error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_by_rib(&self, rib: &Rib) -> Result<Option<Account>, String> {
        let row: Option<AccountRow> = sqlx::query_as(
            "SELECT id, customer_id, rib, account_type, balance, available_balance, currency, status, created_at, updated_at FROM account.accounts WHERE rib = $1",
        )
        .bind(rib.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_rib error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn save_movement(&self, movement: &Movement) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO account.movements (id, account_id, movement_type, amount, balance_after, currency, description, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(movement.id().as_uuid())
        .bind(movement.account_id().as_uuid())
        .bind(movement.movement_type().as_str())
        .bind(movement.amount().amount_cents())
        .bind(movement.balance_after().amount_cents())
        .bind(movement.amount().currency().to_string())
        .bind(movement.description())
        .bind(movement.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save_movement error: {e}"))?;

        Ok(())
    }

    async fn find_movements_by_account(
        &self,
        account_id: &AccountId,
        limit: i64,
    ) -> Result<Vec<Movement>, String> {
        let rows: Vec<MovementRow> = sqlx::query_as(
            "SELECT id, account_id, movement_type, amount, balance_after, currency, description, created_at FROM account.movements WHERE account_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(account_id.as_uuid())
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_movements error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_movements_by_account_and_period(
        &self,
        account_id: &AccountId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<Vec<Movement>, String> {
        let rows: Vec<MovementRow> = sqlx::query_as(
            r#"
            SELECT id, account_id, movement_type, amount, balance_after, currency, description, created_at
            FROM account.movements
            WHERE account_id = $1
              AND ($2::timestamptz IS NULL OR created_at >= $2)
              AND ($3::timestamptz IS NULL OR created_at <= $3)
            ORDER BY created_at ASC
            "#,
        )
        .bind(account_id.as_uuid())
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_movements_by_period error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn delete(&self, id: &AccountId) -> Result<(), String> {
        sqlx::query("DELETE FROM account.accounts WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB delete error: {e}"))?;

        Ok(())
    }
}
