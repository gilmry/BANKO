use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::credit::{ILoanRepository, IScheduleRepository};
use banko_domain::account::AccountId;
use banko_domain::credit::{
    AssetClass, Installment, InstallmentId, Loan, LoanId, LoanStatus, Provision,
};
use banko_domain::shared::{Currency, CustomerId, Money};

// --- Loan Repository ---

pub struct PgLoanRepository {
    pool: PgPool,
}

impl PgLoanRepository {
    pub fn new(pool: PgPool) -> Self {
        PgLoanRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct LoanRow {
    id: Uuid,
    customer_id: Uuid,
    account_id: Uuid,
    amount: i64,
    interest_rate: f64,
    term_months: i32,
    currency: String,
    asset_class: i32,
    status: String,
    days_past_due: i32,
    disbursement_date: Option<NaiveDate>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
struct ProvisionRow {
    amount: i64,
    rate: f64,
    asset_class: i32,
    currency: String,
}

impl LoanRow {
    fn into_domain(self, provision: Option<ProvisionRow>) -> Result<Loan, String> {
        let id = LoanId::from_uuid(self.id);
        let customer_id = CustomerId::from_uuid(self.customer_id);
        let account_id = AccountId::from_uuid(self.account_id);
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let amount = Money::from_cents(self.amount, currency);
        let asset_class = AssetClass::from_i32(self.asset_class).map_err(|e| e.to_string())?;
        let status = LoanStatus::from_str_status(&self.status).map_err(|e| e.to_string())?;

        let prov = if let Some(p) = provision {
            let p_currency = Currency::from_code(&p.currency).map_err(|e| e.to_string())?;
            let p_class = AssetClass::from_i32(p.asset_class).map_err(|e| e.to_string())?;
            Provision::reconstitute(Money::from_cents(p.amount, p_currency), p.rate, p_class)
        } else {
            Provision::reconstitute(Money::zero(currency), 0.0, asset_class)
        };

        Ok(Loan::reconstitute(
            id,
            customer_id,
            account_id,
            amount,
            self.interest_rate,
            self.term_months as u32,
            asset_class,
            prov,
            None, // Schedule loaded separately if needed
            status,
            self.days_past_due as u32,
            self.disbursement_date,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl ILoanRepository for PgLoanRepository {
    async fn save(&self, loan: &Loan) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO credit.loans (id, customer_id, account_id, amount, interest_rate, term_months, currency, asset_class, status, days_past_due, disbursement_date, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                asset_class = EXCLUDED.asset_class,
                status = EXCLUDED.status,
                days_past_due = EXCLUDED.days_past_due,
                disbursement_date = EXCLUDED.disbursement_date,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(loan.id().as_uuid())
        .bind(loan.customer_id().as_uuid())
        .bind(loan.account_id().as_uuid())
        .bind(loan.amount().amount_cents())
        .bind(loan.interest_rate())
        .bind(loan.term_months() as i32)
        .bind(loan.amount().currency().to_string())
        .bind(loan.asset_class().as_i32())
        .bind(loan.status().as_str())
        .bind(loan.days_past_due() as i32)
        .bind(loan.disbursement_date())
        .bind(loan.created_at())
        .bind(loan.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save loan error: {e}"))?;

        // Upsert provision
        sqlx::query(
            r#"
            INSERT INTO credit.loan_provisions (loan_id, amount, rate, asset_class, currency, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            ON CONFLICT (loan_id) DO UPDATE SET
                amount = EXCLUDED.amount,
                rate = EXCLUDED.rate,
                asset_class = EXCLUDED.asset_class,
                updated_at = NOW()
            "#,
        )
        .bind(loan.id().as_uuid())
        .bind(loan.provision().amount().amount_cents())
        .bind(loan.provision().rate())
        .bind(loan.provision().asset_class().as_i32())
        .bind(loan.provision().amount().currency().to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save provision error: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &LoanId) -> Result<Option<Loan>, String> {
        let row: Option<LoanRow> = sqlx::query_as(
            "SELECT id, customer_id, account_id, amount, interest_rate, term_months, currency, asset_class, status, days_past_due, disbursement_date, created_at, updated_at FROM credit.loans WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_id error: {e}"))?;

        match row {
            Some(r) => {
                let prov: Option<ProvisionRow> = sqlx::query_as(
                    "SELECT amount, rate, asset_class, currency FROM credit.loan_provisions WHERE loan_id = $1 ORDER BY updated_at DESC LIMIT 1",
                )
                .bind(id.as_uuid())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| format!("DB find provision error: {e}"))?;

                Ok(Some(r.into_domain(prov)?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<Loan>, String> {
        let rows: Vec<LoanRow> = sqlx::query_as(
            "SELECT id, customer_id, account_id, amount, interest_rate, term_months, currency, asset_class, status, days_past_due, disbursement_date, created_at, updated_at FROM credit.loans WHERE account_id = $1 ORDER BY created_at DESC",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_account error: {e}"))?;

        let mut loans = Vec::new();
        for r in rows {
            let loan_id = r.id;
            let prov: Option<ProvisionRow> = sqlx::query_as(
                "SELECT amount, rate, asset_class, currency FROM credit.loan_provisions WHERE loan_id = $1 ORDER BY updated_at DESC LIMIT 1",
            )
            .bind(loan_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("DB find provision error: {e}"))?;

            loans.push(r.into_domain(prov)?);
        }
        Ok(loans)
    }

    async fn find_all(
        &self,
        status: Option<LoanStatus>,
        asset_class: Option<AssetClass>,
        account_id: Option<&AccountId>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Loan>, String> {
        let rows: Vec<LoanRow> = sqlx::query_as(
            r#"
            SELECT id, customer_id, account_id, amount, interest_rate, term_months, currency, asset_class, status, days_past_due, disbursement_date, created_at, updated_at
            FROM credit.loans
            WHERE ($1::text IS NULL OR status = $1)
              AND ($2::integer IS NULL OR asset_class = $2)
              AND ($3::uuid IS NULL OR account_id = $3)
            ORDER BY created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(status.map(|s| s.as_str().to_string()))
        .bind(asset_class.map(|c| c.as_i32()))
        .bind(account_id.map(|a| *a.as_uuid()))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_all error: {e}"))?;

        let mut loans = Vec::new();
        for r in rows {
            let loan_id = r.id;
            let prov: Option<ProvisionRow> = sqlx::query_as(
                "SELECT amount, rate, asset_class, currency FROM credit.loan_provisions WHERE loan_id = $1 ORDER BY updated_at DESC LIMIT 1",
            )
            .bind(loan_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("DB find provision error: {e}"))?;

            loans.push(r.into_domain(prov)?);
        }
        Ok(loans)
    }

    async fn count_all(
        &self,
        status: Option<LoanStatus>,
        asset_class: Option<AssetClass>,
        account_id: Option<&AccountId>,
    ) -> Result<i64, String> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM credit.loans
            WHERE ($1::text IS NULL OR status = $1)
              AND ($2::integer IS NULL OR asset_class = $2)
              AND ($3::uuid IS NULL OR account_id = $3)
            "#,
        )
        .bind(status.map(|s| s.as_str().to_string()))
        .bind(asset_class.map(|c| c.as_i32()))
        .bind(account_id.map(|a| *a.as_uuid()))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB count error: {e}"))?;

        Ok(row.0)
    }

    async fn find_active_loans(&self) -> Result<Vec<Loan>, String> {
        let rows: Vec<LoanRow> = sqlx::query_as(
            "SELECT id, customer_id, account_id, amount, interest_rate, term_months, currency, asset_class, status, days_past_due, disbursement_date, created_at, updated_at FROM credit.loans WHERE status = 'Active' ORDER BY created_at",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_active error: {e}"))?;

        let mut loans = Vec::new();
        for r in rows {
            let loan_id = r.id;
            let prov: Option<ProvisionRow> = sqlx::query_as(
                "SELECT amount, rate, asset_class, currency FROM credit.loan_provisions WHERE loan_id = $1 ORDER BY updated_at DESC LIMIT 1",
            )
            .bind(loan_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("DB find provision error: {e}"))?;

            loans.push(r.into_domain(prov)?);
        }
        Ok(loans)
    }

    async fn delete(&self, id: &LoanId) -> Result<(), String> {
        sqlx::query("DELETE FROM credit.loans WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB delete loan error: {e}"))?;

        Ok(())
    }
}

// --- Schedule Repository ---

pub struct PgScheduleRepository {
    pool: PgPool,
}

impl PgScheduleRepository {
    pub fn new(pool: PgPool) -> Self {
        PgScheduleRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct InstallmentRow {
    id: Uuid,
    loan_id: Uuid,
    installment_number: i32,
    due_date: NaiveDate,
    principal_amount: i64,
    interest_amount: i64,
    total_amount: i64,
    remaining_balance: i64,
    currency: String,
    paid: bool,
    paid_date: Option<DateTime<Utc>>,
}

impl InstallmentRow {
    fn into_domain(self) -> Result<Installment, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        Ok(Installment::reconstitute(
            InstallmentId::from_uuid(self.id),
            LoanId::from_uuid(self.loan_id),
            self.installment_number as u32,
            self.due_date,
            Money::from_cents(self.principal_amount, currency),
            Money::from_cents(self.interest_amount, currency),
            Money::from_cents(self.total_amount, currency),
            Money::from_cents(self.remaining_balance, currency),
            self.paid,
            self.paid_date,
        ))
    }
}

#[async_trait]
impl IScheduleRepository for PgScheduleRepository {
    async fn save_installments(&self, installments: &[Installment]) -> Result<(), String> {
        for inst in installments {
            sqlx::query(
                r#"
                INSERT INTO credit.loan_installments (id, loan_id, installment_number, due_date, principal_amount, interest_amount, total_amount, remaining_balance, currency, paid, paid_date)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (id) DO UPDATE SET
                    paid = EXCLUDED.paid,
                    paid_date = EXCLUDED.paid_date
                "#,
            )
            .bind(inst.id().as_uuid())
            .bind(inst.loan_id().as_uuid())
            .bind(inst.installment_number() as i32)
            .bind(inst.due_date())
            .bind(inst.principal_amount().amount_cents())
            .bind(inst.interest_amount().amount_cents())
            .bind(inst.total_amount().amount_cents())
            .bind(inst.remaining_balance().amount_cents())
            .bind(inst.principal_amount().currency().to_string())
            .bind(inst.paid())
            .bind(inst.paid_date())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB save installment error: {e}"))?;
        }
        Ok(())
    }

    async fn find_by_loan_id(&self, loan_id: &LoanId) -> Result<Vec<Installment>, String> {
        let rows: Vec<InstallmentRow> = sqlx::query_as(
            "SELECT id, loan_id, installment_number, due_date, principal_amount, interest_amount, total_amount, remaining_balance, currency, paid, paid_date FROM credit.loan_installments WHERE loan_id = $1 ORDER BY installment_number",
        )
        .bind(loan_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find installments error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn update_installment(&self, installment: &Installment) -> Result<(), String> {
        sqlx::query(
            r#"
            UPDATE credit.loan_installments
            SET paid = $1, paid_date = $2
            WHERE id = $3
            "#,
        )
        .bind(installment.paid())
        .bind(installment.paid_date())
        .bind(installment.id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB update installment error: {e}"))?;

        Ok(())
    }
}
