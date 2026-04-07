use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::customer::ICustomerRepository;
use banko_domain::customer::{
    Address, Beneficiary, ConsentStatus, Customer, CustomerSegment, CustomerStatus, CustomerType,
    KycProfile, PepStatus, RiskScore, SourceOfFunds,
};
use banko_domain::shared::value_objects::{CustomerId, EmailAddress, PhoneNumber};

pub struct PgCustomerRepository {
    pool: PgPool,
}

impl PgCustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        PgCustomerRepository { pool }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct CustomerRow {
    id: Uuid,
    customer_type: String,
    status: String,
    risk_score: i32,
    consent: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
struct KycProfileRow {
    full_name: String,
    cin_or_rcs: Option<String>,
    birth_date: Option<NaiveDate>,
    nationality: Option<String>,
    profession: Option<String>,
    street: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    pep_status: Option<String>,
    source_of_funds: Option<String>,
    sector: Option<String>,
    submission_date: Option<DateTime<Utc>>,
    approval_date: Option<DateTime<Utc>>,
    rejection_reason: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct BeneficiaryRow {
    id: Uuid,
    full_name: String,
    share_percentage: rust_decimal::Decimal,
}

fn row_to_domain(
    row: CustomerRow,
    kyc_row: KycProfileRow,
    beneficiary_rows: Vec<BeneficiaryRow>,
) -> Result<Customer, String> {
    let id = CustomerId::from_uuid(row.id);
    let customer_type =
        CustomerType::from_str_type(&row.customer_type).map_err(|e| e.to_string())?;
    let status = CustomerStatus::from_str_status(&row.status).map_err(|e| e.to_string())?;
    let risk_score = RiskScore::new(row.risk_score as u8).map_err(|e| e.to_string())?;
    let consent = ConsentStatus::from_str_consent(&row.consent).map_err(|e| e.to_string())?;

    // For anonymized customers, bypass validation since "[ANONYMIZED]" won't pass
    let is_anonymized = status == CustomerStatus::Anonymized;

    let address = if is_anonymized {
        Address::new_unchecked(
            kyc_row.street.as_deref().unwrap_or("[ANONYMIZED]"),
            kyc_row.city.as_deref().unwrap_or("[ANONYMIZED]"),
            kyc_row.postal_code.as_deref().unwrap_or("[ANONYMIZED]"),
            kyc_row.country.as_deref().unwrap_or("[ANONYMIZED]"),
        )
    } else {
        Address::new(
            kyc_row.street.as_deref().unwrap_or(""),
            kyc_row.city.as_deref().unwrap_or(""),
            kyc_row.postal_code.as_deref().unwrap_or(""),
            kyc_row.country.as_deref().unwrap_or("Tunisia"),
        )
        .map_err(|e| e.to_string())?
    };

    let phone = if is_anonymized {
        PhoneNumber::unchecked(kyc_row.phone.as_deref().unwrap_or("[ANONYMIZED]"))
    } else {
        PhoneNumber::new(kyc_row.phone.as_deref().unwrap_or("+21600000000"))
            .map_err(|e| e.to_string())?
    };

    let email = if is_anonymized {
        EmailAddress::unchecked(kyc_row.email.as_deref().unwrap_or("[ANONYMIZED]"))
    } else {
        EmailAddress::new(kyc_row.email.as_deref().unwrap_or("unknown@unknown.com"))
            .map_err(|e| e.to_string())?
    };

    let pep_status = kyc_row
        .pep_status
        .as_deref()
        .map(PepStatus::from_str_status)
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or(PepStatus::Unknown);

    let source_of_funds = kyc_row
        .source_of_funds
        .as_deref()
        .map(SourceOfFunds::from_str_source)
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or(SourceOfFunds::Other);

    let kyc_profile = KycProfile::reconstitute(
        kyc_row.full_name,
        kyc_row.cin_or_rcs.unwrap_or_default(),
        kyc_row.birth_date,
        kyc_row.nationality.unwrap_or_else(|| "Tunisia".to_string()),
        kyc_row.profession.unwrap_or_default(),
        address,
        phone,
        email,
        pep_status,
        source_of_funds,
        kyc_row.sector,
        kyc_row.submission_date,
        kyc_row.approval_date,
        kyc_row.rejection_reason,
    );

    let beneficiaries: Vec<Beneficiary> = beneficiary_rows
        .into_iter()
        .map(|b| {
            use rust_decimal::prelude::ToPrimitive;
            Beneficiary::reconstitute(
                b.id,
                b.full_name,
                b.share_percentage.to_f64().unwrap_or(0.0),
            )
        })
        .collect();

    Ok(Customer::reconstitute(
        id,
        customer_type,
        kyc_profile,
        beneficiaries,
        risk_score,
        status,
        consent,
        CustomerSegment::Retail,  // TODO: persist and load segment from DB
        vec![],                   // TODO: persist and load documents from DB
        row.created_at,
        row.updated_at,
        row.closed_at,
    ))
}

#[async_trait]
impl ICustomerRepository for PgCustomerRepository {
    async fn save(&self, customer: &Customer) -> Result<(), String> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| format!("DB transaction begin error: {e}"))?;

        // Upsert customer
        sqlx::query(
            r#"
            INSERT INTO customer.customers (id, customer_type, status, risk_score, consent, created_at, updated_at, closed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                customer_type = EXCLUDED.customer_type,
                status = EXCLUDED.status,
                risk_score = EXCLUDED.risk_score,
                consent = EXCLUDED.consent,
                updated_at = EXCLUDED.updated_at,
                closed_at = EXCLUDED.closed_at
            "#,
        )
        .bind(customer.id().as_uuid())
        .bind(customer.customer_type().as_str())
        .bind(customer.status().as_str())
        .bind(customer.risk_score().value() as i32)
        .bind(customer.consent().as_str())
        .bind(customer.created_at())
        .bind(customer.updated_at())
        .bind(customer.closed_at())
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("DB save customer error: {e}"))?;

        // Upsert KYC profile
        let kyc = customer.kyc_profile();
        sqlx::query(
            r#"
            INSERT INTO customer.kyc_profiles (customer_id, full_name, cin_or_rcs, birth_date, nationality, profession, street, city, postal_code, country, phone, email, pep_status, source_of_funds, sector, submission_date, approval_date, rejection_reason)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ON CONFLICT (customer_id) DO UPDATE SET
                full_name = EXCLUDED.full_name,
                cin_or_rcs = EXCLUDED.cin_or_rcs,
                birth_date = EXCLUDED.birth_date,
                nationality = EXCLUDED.nationality,
                profession = EXCLUDED.profession,
                street = EXCLUDED.street,
                city = EXCLUDED.city,
                postal_code = EXCLUDED.postal_code,
                country = EXCLUDED.country,
                phone = EXCLUDED.phone,
                email = EXCLUDED.email,
                pep_status = EXCLUDED.pep_status,
                source_of_funds = EXCLUDED.source_of_funds,
                sector = EXCLUDED.sector,
                submission_date = EXCLUDED.submission_date,
                approval_date = EXCLUDED.approval_date,
                rejection_reason = EXCLUDED.rejection_reason
            "#,
        )
        .bind(customer.id().as_uuid())
        .bind(kyc.full_name())
        .bind(kyc.cin_or_rcs())
        .bind(kyc.birth_date())
        .bind(kyc.nationality())
        .bind(kyc.profession())
        .bind(kyc.address().street())
        .bind(kyc.address().city())
        .bind(kyc.address().postal_code())
        .bind(kyc.address().country())
        .bind(kyc.phone().as_str())
        .bind(kyc.email().as_str())
        .bind(kyc.pep_status().as_str())
        .bind(kyc.source_of_funds().as_str())
        .bind(kyc.sector())
        .bind(kyc.submission_date())
        .bind(kyc.approval_date())
        .bind(kyc.rejection_reason())
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("DB save kyc_profile error: {e}"))?;

        // Delete existing beneficiaries and re-insert
        sqlx::query("DELETE FROM customer.beneficiaries WHERE customer_id = $1")
            .bind(customer.id().as_uuid())
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("DB delete beneficiaries error: {e}"))?;

        for b in customer.beneficiaries() {
            sqlx::query(
                r#"
                INSERT INTO customer.beneficiaries (id, customer_id, full_name, share_percentage)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(b.id())
            .bind(customer.id().as_uuid())
            .bind(b.full_name())
            .bind(rust_decimal::Decimal::try_from(b.share_percentage()).unwrap_or_default())
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("DB save beneficiary error: {e}"))?;
        }

        tx.commit()
            .await
            .map_err(|e| format!("DB transaction commit error: {e}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CustomerId) -> Result<Option<Customer>, String> {
        let row: Option<CustomerRow> = sqlx::query_as(
            "SELECT id, customer_type, status, risk_score, consent, created_at, updated_at, closed_at FROM customer.customers WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_id error: {e}"))?;

        match row {
            Some(customer_row) => {
                let kyc_row: KycProfileRow = sqlx::query_as(
                    "SELECT full_name, cin_or_rcs, birth_date, nationality, profession, street, city, postal_code, country, phone, email, pep_status, source_of_funds, sector, submission_date, approval_date, rejection_reason FROM customer.kyc_profiles WHERE customer_id = $1",
                )
                .bind(id.as_uuid())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| format!("DB find kyc_profile error: {e}"))?;

                let beneficiary_rows: Vec<BeneficiaryRow> = sqlx::query_as(
                    "SELECT id, full_name, share_percentage FROM customer.beneficiaries WHERE customer_id = $1",
                )
                .bind(id.as_uuid())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("DB find beneficiaries error: {e}"))?;

                Ok(Some(row_to_domain(
                    customer_row,
                    kyc_row,
                    beneficiary_rows,
                )?))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<Customer>, String> {
        let row: Option<CustomerRow> = sqlx::query_as(
            r#"
            SELECT c.id, c.customer_type, c.status, c.risk_score, c.consent, c.created_at, c.updated_at, c.closed_at
            FROM customer.customers c
            JOIN customer.kyc_profiles k ON k.customer_id = c.id
            WHERE k.email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_by_email error: {e}"))?;

        match row {
            Some(customer_row) => {
                let customer_id = CustomerId::from_uuid(customer_row.id);
                let kyc_row: KycProfileRow = sqlx::query_as(
                    "SELECT full_name, cin_or_rcs, birth_date, nationality, profession, street, city, postal_code, country, phone, email, pep_status, source_of_funds, sector, submission_date, approval_date, rejection_reason FROM customer.kyc_profiles WHERE customer_id = $1",
                )
                .bind(customer_id.as_uuid())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| format!("DB find kyc_profile error: {e}"))?;

                let beneficiary_rows: Vec<BeneficiaryRow> = sqlx::query_as(
                    "SELECT id, full_name, share_percentage FROM customer.beneficiaries WHERE customer_id = $1",
                )
                .bind(customer_id.as_uuid())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("DB find beneficiaries error: {e}"))?;

                Ok(Some(row_to_domain(
                    customer_row,
                    kyc_row,
                    beneficiary_rows,
                )?))
            }
            None => Ok(None),
        }
    }

    async fn list_all(&self) -> Result<Vec<Customer>, String> {
        let rows: Vec<CustomerRow> = sqlx::query_as(
            "SELECT id, customer_type, status, risk_score, consent, created_at, updated_at, closed_at FROM customer.customers ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_all error: {e}"))?;

        let mut customers = Vec::with_capacity(rows.len());
        for customer_row in rows {
            let cid = customer_row.id;
            let kyc_row: KycProfileRow = sqlx::query_as(
                "SELECT full_name, cin_or_rcs, birth_date, nationality, profession, street, city, postal_code, country, phone, email, pep_status, source_of_funds, sector, submission_date, approval_date, rejection_reason FROM customer.kyc_profiles WHERE customer_id = $1",
            )
            .bind(cid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("DB find kyc_profile error: {e}"))?;

            let beneficiary_rows: Vec<BeneficiaryRow> = sqlx::query_as(
                "SELECT id, full_name, share_percentage FROM customer.beneficiaries WHERE customer_id = $1",
            )
            .bind(cid)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("DB find beneficiaries error: {e}"))?;

            customers.push(row_to_domain(customer_row, kyc_row, beneficiary_rows)?);
        }

        Ok(customers)
    }

    async fn list_by_status(&self, status: &str) -> Result<Vec<Customer>, String> {
        let rows: Vec<CustomerRow> = sqlx::query_as(
            "SELECT id, customer_type, status, risk_score, consent, created_at, updated_at, closed_at FROM customer.customers WHERE status = $1 ORDER BY created_at DESC",
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_by_status error: {e}"))?;

        let mut customers = Vec::with_capacity(rows.len());
        for customer_row in rows {
            let cid = customer_row.id;
            let kyc_row: KycProfileRow = sqlx::query_as(
                "SELECT full_name, cin_or_rcs, birth_date, nationality, profession, street, city, postal_code, country, phone, email, pep_status, source_of_funds, sector, submission_date, approval_date, rejection_reason FROM customer.kyc_profiles WHERE customer_id = $1",
            )
            .bind(cid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("DB find kyc_profile error: {e}"))?;

            let beneficiary_rows: Vec<BeneficiaryRow> = sqlx::query_as(
                "SELECT id, full_name, share_percentage FROM customer.beneficiaries WHERE customer_id = $1",
            )
            .bind(cid)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("DB find beneficiaries error: {e}"))?;

            customers.push(row_to_domain(customer_row, kyc_row, beneficiary_rows)?);
        }

        Ok(customers)
    }

    async fn delete(&self, id: &CustomerId) -> Result<(), String> {
        sqlx::query("DELETE FROM customer.customers WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB delete error: {e}"))?;

        Ok(())
    }

    async fn search(
        &self,
        _full_name: Option<&str>,
        _email: Option<&str>,
        _cin_or_rcs: Option<&str>,
        _customer_type: Option<&str>,
        _status: Option<&str>,
        _segment: Option<&str>,
        _risk_score_min: Option<u8>,
        _risk_score_max: Option<u8>,
        _limit: i64,
        _offset: i64,
    ) -> Result<(i64, Vec<Customer>), String> {
        // TODO: Implement multi-criteria search query (FR-014)
        Ok((0, vec![]))
    }

    async fn find_closed_before(&self, before: DateTime<Utc>) -> Result<Vec<Customer>, String> {
        let rows: Vec<CustomerRow> = sqlx::query_as(
            "SELECT id, customer_type, status, risk_score, consent, created_at, updated_at, closed_at FROM customer.customers WHERE status = 'Closed' AND closed_at IS NOT NULL AND closed_at < $1 ORDER BY closed_at ASC",
        )
        .bind(before)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_closed_before error: {e}"))?;

        let mut customers = Vec::with_capacity(rows.len());
        for customer_row in rows {
            let cid = customer_row.id;
            let kyc_row: KycProfileRow = sqlx::query_as(
                "SELECT full_name, cin_or_rcs, birth_date, nationality, profession, street, city, postal_code, country, phone, email, pep_status, source_of_funds, sector, submission_date, approval_date, rejection_reason FROM customer.kyc_profiles WHERE customer_id = $1",
            )
            .bind(cid)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("DB find kyc_profile error: {e}"))?;

            let beneficiary_rows: Vec<BeneficiaryRow> = sqlx::query_as(
                "SELECT id, full_name, share_percentage FROM customer.beneficiaries WHERE customer_id = $1",
            )
            .bind(cid)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("DB find beneficiaries error: {e}"))?;

            customers.push(row_to_domain(customer_row, kyc_row, beneficiary_rows)?);
        }

        Ok(customers)
    }
}
