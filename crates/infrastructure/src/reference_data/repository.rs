use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::reference_data::IReferenceDataRepository;
use banko_domain::reference_data::{
    BankCode, BicCodeVo, BranchCode, CountryCode, CountryCodeVo, CurrencyCodeVo,
    CurrencyReference, FeeScheduleReference, FeeType, HolidayCalendar, HolidayType,
    ReferenceDataId, RegulatoryClassification, RegulatoryCode, SystemParameter,
    SystemParameterType,
};

pub struct PgReferenceDataRepository {
    pool: PgPool,
}

impl PgReferenceDataRepository {
    pub fn new(pool: PgPool) -> Self {
        PgReferenceDataRepository { pool }
    }
}

// --- Country Code Row ---

#[derive(Debug, sqlx::FromRow)]
struct CountryCodeRow {
    id: Uuid,
    iso_alpha2: String,
    iso_alpha3: String,
    iso_numeric: String,
    name_en: String,
    name_fr: String,
    name_ar: String,
    is_sanctioned: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CountryCodeRow {
    fn into_domain(self) -> Result<CountryCode, String> {
        let code = CountryCodeVo::new(&self.iso_alpha2, &self.iso_alpha3, &self.iso_numeric)
            .map_err(|e| e.to_string())?;

        Ok(CountryCode::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            code,
            self.name_en,
            self.name_fr,
            self.name_ar,
            self.is_sanctioned,
            self.effective_from,
            self.effective_to,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Currency Reference Row ---

#[derive(Debug, sqlx::FromRow)]
struct CurrencyReferenceRow {
    id: Uuid,
    code: String,
    name_en: String,
    name_fr: String,
    decimal_places: i32,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CurrencyReferenceRow {
    fn into_domain(self) -> Result<CurrencyReference, String> {
        let code = CurrencyCodeVo::new(&self.code).map_err(|e| e.to_string())?;

        Ok(CurrencyReference::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            code,
            self.name_en,
            self.name_fr,
            self.decimal_places,
            self.is_active,
            self.effective_from,
            self.effective_to,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Bank Code Row ---

#[derive(Debug, sqlx::FromRow)]
struct BankCodeRow {
    id: Uuid,
    bic: String,
    bank_name: String,
    country_iso_alpha2: String,
    country_iso_alpha3: String,
    country_iso_numeric: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BankCodeRow {
    fn into_domain(self) -> Result<BankCode, String> {
        let bic = BicCodeVo::new(&self.bic).map_err(|e| e.to_string())?;
        let country =
            CountryCodeVo::new(&self.country_iso_alpha2, &self.country_iso_alpha3, &self.country_iso_numeric)
                .map_err(|e| e.to_string())?;

        Ok(BankCode::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            bic,
            self.bank_name,
            country,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Branch Code Row ---

#[derive(Debug, sqlx::FromRow)]
struct BranchCodeRow {
    id: Uuid,
    branch_code: String,
    branch_name: String,
    bank_bic: String,
    city: String,
    address: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BranchCodeRow {
    fn into_domain(self) -> Result<BranchCode, String> {
        let bic = BicCodeVo::new(&self.bank_bic).map_err(|e| e.to_string())?;

        Ok(BranchCode::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            self.branch_code,
            self.branch_name,
            bic,
            self.city,
            self.address,
            self.is_active,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Holiday Calendar Row ---

#[derive(Debug, sqlx::FromRow)]
struct HolidayCalendarRow {
    id: Uuid,
    holiday_date: DateTime<Utc>,
    holiday_name_en: String,
    holiday_name_fr: String,
    holiday_name_ar: String,
    holiday_type: String,
    is_banking_holiday: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl HolidayCalendarRow {
    fn into_domain(self) -> Result<HolidayCalendar, String> {
        let holiday_type = HolidayType::from_str(&self.holiday_type).map_err(|e| e.to_string())?;

        Ok(HolidayCalendar::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            self.holiday_date,
            self.holiday_name_en,
            self.holiday_name_fr,
            self.holiday_name_ar,
            holiday_type,
            self.is_banking_holiday,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- System Parameter Row ---

#[derive(Debug, sqlx::FromRow)]
struct SystemParameterRow {
    id: Uuid,
    key: String,
    value: String,
    parameter_type: String,
    category: String,
    description: String,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SystemParameterRow {
    fn into_domain(self) -> Result<SystemParameter, String> {
        let param_type = SystemParameterType::from_str(&self.parameter_type)
            .map_err(|e| e.to_string())?;

        Ok(SystemParameter::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            self.key,
            self.value,
            param_type,
            self.category,
            self.description,
            self.is_active,
            self.effective_from,
            self.effective_to,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Regulatory Code Row ---

#[derive(Debug, sqlx::FromRow)]
struct RegulatoryCodeRow {
    id: Uuid,
    code: String,
    description_en: String,
    description_fr: String,
    classification: String,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RegulatoryCodeRow {
    fn into_domain(self) -> Result<RegulatoryCode, String> {
        let classification = RegulatoryClassification::from_str(&self.classification)
            .map_err(|e| e.to_string())?;

        Ok(RegulatoryCode::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            self.code,
            self.description_en,
            self.description_fr,
            classification,
            self.is_active,
            self.effective_from,
            self.effective_to,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Fee Schedule Row ---

#[derive(Debug, sqlx::FromRow)]
struct FeeScheduleRow {
    id: Uuid,
    fee_type: String,
    amount_cents: i64,
    currency_code: String,
    description_en: String,
    description_fr: String,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FeeScheduleRow {
    fn into_domain(self) -> Result<FeeScheduleReference, String> {
        let fee_type = FeeType::from_str(&self.fee_type).map_err(|e| e.to_string())?;
        let currency = CurrencyCodeVo::new(&self.currency_code).map_err(|e| e.to_string())?;

        Ok(FeeScheduleReference::reconstitute(
            ReferenceDataId::from_uuid(self.id),
            fee_type,
            self.amount_cents,
            currency,
            self.description_en,
            self.description_fr,
            self.is_active,
            self.effective_from,
            self.effective_to,
            self.created_at,
            self.updated_at,
        ))
    }
}

#[async_trait]
impl IReferenceDataRepository for PgReferenceDataRepository {
    // --- Country Code Implementation ---

    async fn save_country(&self, country: &CountryCode) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.country_codes
                (id, iso_alpha2, iso_alpha3, iso_numeric, name_en, name_fr, name_ar,
                 is_sanctioned, effective_from, effective_to, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                name_en = EXCLUDED.name_en,
                name_fr = EXCLUDED.name_fr,
                name_ar = EXCLUDED.name_ar,
                is_sanctioned = EXCLUDED.is_sanctioned,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(country.id().as_uuid())
        .bind(country.code().iso_alpha2())
        .bind(country.code().iso_alpha3())
        .bind(country.code().iso_numeric())
        .bind(country.name_en())
        .bind(country.name_fr())
        .bind(country.name_ar())
        .bind(country.is_sanctioned())
        .bind(country.effective_from())
        .bind(country.effective_to())
        .bind(country.created_at())
        .bind(country.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save country error: {e}"))?;

        Ok(())
    }

    async fn find_country_by_id(&self, id: &ReferenceDataId) -> Result<Option<CountryCode>, String> {
        let row: Option<CountryCodeRow> = sqlx::query_as(
            "SELECT id, iso_alpha2, iso_alpha3, iso_numeric, name_en, name_fr, name_ar,
                    is_sanctioned, effective_from, effective_to, created_at, updated_at
             FROM reference_data.country_codes WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_country_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_country_by_iso_alpha2(&self, code: &str) -> Result<Option<CountryCode>, String> {
        let row: Option<CountryCodeRow> = sqlx::query_as(
            "SELECT id, iso_alpha2, iso_alpha3, iso_numeric, name_en, name_fr, name_ar,
                    is_sanctioned, effective_from, effective_to, created_at, updated_at
             FROM reference_data.country_codes WHERE iso_alpha2 = $1",
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_country_by_iso_alpha2 error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_country_by_iso_alpha3(&self, code: &str) -> Result<Option<CountryCode>, String> {
        let row: Option<CountryCodeRow> = sqlx::query_as(
            "SELECT id, iso_alpha2, iso_alpha3, iso_numeric, name_en, name_fr, name_ar,
                    is_sanctioned, effective_from, effective_to, created_at, updated_at
             FROM reference_data.country_codes WHERE iso_alpha3 = $1",
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_country_by_iso_alpha3 error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_countries(&self, limit: i64, offset: i64) -> Result<Vec<CountryCode>, String> {
        let rows: Vec<CountryCodeRow> = sqlx::query_as(
            "SELECT id, iso_alpha2, iso_alpha3, iso_numeric, name_en, name_fr, name_ar,
                    is_sanctioned, effective_from, effective_to, created_at, updated_at
             FROM reference_data.country_codes
             ORDER BY iso_alpha2 ASC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_countries error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_countries(&self) -> Result<Vec<CountryCode>, String> {
        let now = Utc::now();
        let rows: Vec<CountryCodeRow> = sqlx::query_as(
            "SELECT id, iso_alpha2, iso_alpha3, iso_numeric, name_en, name_fr, name_ar,
                    is_sanctioned, effective_from, effective_to, created_at, updated_at
             FROM reference_data.country_codes
             WHERE effective_from <= $1 AND (effective_to IS NULL OR effective_to > $1)
             ORDER BY iso_alpha2 ASC",
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_countries error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Currency Reference Implementation ---

    async fn save_currency(&self, currency: &CurrencyReference) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.currency_references
                (id, code, name_en, name_fr, decimal_places, is_active, effective_from, effective_to, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                name_en = EXCLUDED.name_en,
                name_fr = EXCLUDED.name_fr,
                decimal_places = EXCLUDED.decimal_places,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(currency.id().as_uuid())
        .bind(currency.code().to_string())
        .bind(currency.name_en())
        .bind(currency.name_fr())
        .bind(currency.decimal_places())
        .bind(currency.is_active())
        .bind(currency.created_at())
        .bind(None::<DateTime<Utc>>)
        .bind(currency.created_at())
        .bind(currency.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save currency error: {e}"))?;

        Ok(())
    }

    async fn find_currency_by_id(&self, id: &ReferenceDataId) -> Result<Option<CurrencyReference>, String> {
        let row: Option<CurrencyReferenceRow> = sqlx::query_as(
            "SELECT id, code, name_en, name_fr, decimal_places, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.currency_references WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_currency_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_currency_by_code(&self, code: &str) -> Result<Option<CurrencyReference>, String> {
        let row: Option<CurrencyReferenceRow> = sqlx::query_as(
            "SELECT id, code, name_en, name_fr, decimal_places, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.currency_references WHERE code = $1",
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_currency_by_code error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_currencies(&self, limit: i64, offset: i64) -> Result<Vec<CurrencyReference>, String> {
        let rows: Vec<CurrencyReferenceRow> = sqlx::query_as(
            "SELECT id, code, name_en, name_fr, decimal_places, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.currency_references
             ORDER BY code ASC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_currencies error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_currencies(&self) -> Result<Vec<CurrencyReference>, String> {
        let now = Utc::now();
        let rows: Vec<CurrencyReferenceRow> = sqlx::query_as(
            "SELECT id, code, name_en, name_fr, decimal_places, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.currency_references
             WHERE is_active = true AND effective_from <= $1 AND (effective_to IS NULL OR effective_to > $1)
             ORDER BY code ASC",
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_currencies error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Bank Code Implementation ---

    async fn save_bank_code(&self, bank: &BankCode) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.bank_codes
                (id, bic, bank_name, country_iso_alpha2, country_iso_alpha3, country_iso_numeric, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                bank_name = EXCLUDED.bank_name,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(bank.id().as_uuid())
        .bind(bank.bic().to_string())
        .bind(bank.bank_name())
        .bind(bank.country_code().iso_alpha2())
        .bind(bank.country_code().iso_alpha3())
        .bind(bank.country_code().iso_numeric())
        .bind(bank.is_active())
        .bind(bank.created_at())
        .bind(bank.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save bank_code error: {e}"))?;

        Ok(())
    }

    async fn find_bank_code_by_id(&self, id: &ReferenceDataId) -> Result<Option<BankCode>, String> {
        let row: Option<BankCodeRow> = sqlx::query_as(
            "SELECT id, bic, bank_name, country_iso_alpha2, country_iso_alpha3, country_iso_numeric, is_active, created_at, updated_at
             FROM reference_data.bank_codes WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_bank_code_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_bank_code_by_bic(&self, bic: &str) -> Result<Option<BankCode>, String> {
        let row: Option<BankCodeRow> = sqlx::query_as(
            "SELECT id, bic, bank_name, country_iso_alpha2, country_iso_alpha3, country_iso_numeric, is_active, created_at, updated_at
             FROM reference_data.bank_codes WHERE bic = $1",
        )
        .bind(bic)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_bank_code_by_bic error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_bank_codes(&self, limit: i64, offset: i64) -> Result<Vec<BankCode>, String> {
        let rows: Vec<BankCodeRow> = sqlx::query_as(
            "SELECT id, bic, bank_name, country_iso_alpha2, country_iso_alpha3, country_iso_numeric, is_active, created_at, updated_at
             FROM reference_data.bank_codes
             ORDER BY bic ASC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_bank_codes error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_bank_codes(&self) -> Result<Vec<BankCode>, String> {
        let rows: Vec<BankCodeRow> = sqlx::query_as(
            "SELECT id, bic, bank_name, country_iso_alpha2, country_iso_alpha3, country_iso_numeric, is_active, created_at, updated_at
             FROM reference_data.bank_codes
             WHERE is_active = true
             ORDER BY bic ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_bank_codes error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Branch Code Implementation ---

    async fn save_branch_code(&self, branch: &BranchCode) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.branch_codes
                (id, branch_code, branch_name, bank_bic, city, address, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                branch_name = EXCLUDED.branch_name,
                city = EXCLUDED.city,
                address = EXCLUDED.address,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(branch.id().as_uuid())
        .bind(branch.branch_code())
        .bind(branch.branch_name())
        .bind(branch.bank_bic().to_string())
        .bind(branch.city())
        .bind(branch.address())
        .bind(branch.is_active())
        .bind(branch.created_at())
        .bind(branch.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save branch_code error: {e}"))?;

        Ok(())
    }

    async fn find_branch_code_by_id(&self, id: &ReferenceDataId) -> Result<Option<BranchCode>, String> {
        let row: Option<BranchCodeRow> = sqlx::query_as(
            "SELECT id, branch_code, branch_name, bank_bic, city, address, is_active, created_at, updated_at
             FROM reference_data.branch_codes WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_branch_code_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_branch_code_by_code(&self, code: &str) -> Result<Option<BranchCode>, String> {
        let row: Option<BranchCodeRow> = sqlx::query_as(
            "SELECT id, branch_code, branch_name, bank_bic, city, address, is_active, created_at, updated_at
             FROM reference_data.branch_codes WHERE branch_code = $1",
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_branch_code_by_code error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_branches_by_bank_bic(&self, bic: &str) -> Result<Vec<BranchCode>, String> {
        let rows: Vec<BranchCodeRow> = sqlx::query_as(
            "SELECT id, branch_code, branch_name, bank_bic, city, address, is_active, created_at, updated_at
             FROM reference_data.branch_codes
             WHERE bank_bic = $1
             ORDER BY branch_code ASC",
        )
        .bind(bic)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_branches_by_bank_bic error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_branch_codes(&self, limit: i64, offset: i64) -> Result<Vec<BranchCode>, String> {
        let rows: Vec<BranchCodeRow> = sqlx::query_as(
            "SELECT id, branch_code, branch_name, bank_bic, city, address, is_active, created_at, updated_at
             FROM reference_data.branch_codes
             ORDER BY branch_code ASC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_branch_codes error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_branch_codes(&self) -> Result<Vec<BranchCode>, String> {
        let rows: Vec<BranchCodeRow> = sqlx::query_as(
            "SELECT id, branch_code, branch_name, bank_bic, city, address, is_active, created_at, updated_at
             FROM reference_data.branch_codes
             WHERE is_active = true
             ORDER BY branch_code ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_branch_codes error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Holiday Calendar Implementation ---

    async fn save_holiday(&self, holiday: &HolidayCalendar) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.holiday_calendar
                (id, holiday_date, holiday_name_en, holiday_name_fr, holiday_name_ar, holiday_type, is_banking_holiday, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                is_banking_holiday = EXCLUDED.is_banking_holiday,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(holiday.id().as_uuid())
        .bind(holiday.holiday_date())
        .bind(holiday.holiday_name_en())
        .bind(holiday.holiday_name_fr())
        .bind(holiday.holiday_name_ar())
        .bind(holiday.holiday_type().to_string())
        .bind(holiday.is_banking_holiday())
        .bind(holiday.created_at())
        .bind(holiday.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save holiday error: {e}"))?;

        Ok(())
    }

    async fn find_holiday_by_id(&self, id: &ReferenceDataId) -> Result<Option<HolidayCalendar>, String> {
        let row: Option<HolidayCalendarRow> = sqlx::query_as(
            "SELECT id, holiday_date, holiday_name_en, holiday_name_fr, holiday_name_ar, holiday_type, is_banking_holiday, created_at, updated_at
             FROM reference_data.holiday_calendar WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_holiday_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_holidays_by_date(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<HolidayCalendar>, String> {
        let rows: Vec<HolidayCalendarRow> = sqlx::query_as(
            "SELECT id, holiday_date, holiday_name_en, holiday_name_fr, holiday_name_ar, holiday_type, is_banking_holiday, created_at, updated_at
             FROM reference_data.holiday_calendar
             WHERE holiday_date >= $1 AND holiday_date < $2
             ORDER BY holiday_date ASC",
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_holidays_by_date error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_banking_holidays_by_date(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<HolidayCalendar>, String> {
        let rows: Vec<HolidayCalendarRow> = sqlx::query_as(
            "SELECT id, holiday_date, holiday_name_en, holiday_name_fr, holiday_name_ar, holiday_type, is_banking_holiday, created_at, updated_at
             FROM reference_data.holiday_calendar
             WHERE is_banking_holiday = true AND holiday_date >= $1 AND holiday_date < $2
             ORDER BY holiday_date ASC",
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_banking_holidays_by_date error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn is_banking_holiday(&self, date: DateTime<Utc>) -> Result<bool, String> {
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM reference_data.holiday_calendar
             WHERE is_banking_holiday = true AND DATE(holiday_date) = DATE($1))",
        )
        .bind(date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB is_banking_holiday error: {e}"))?;

        Ok(exists.0)
    }

    // --- System Parameter Implementation ---

    async fn save_system_parameter(&self, param: &SystemParameter) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.system_parameters
                (id, key, value, parameter_type, category, description, is_active, effective_from, effective_to, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                value = EXCLUDED.value,
                description = EXCLUDED.description,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(param.id().as_uuid())
        .bind(param.key())
        .bind(param.value())
        .bind(param.parameter_type().to_string())
        .bind(param.category())
        .bind(param.description())
        .bind(param.is_active())
        .bind(param.created_at())
        .bind(None::<DateTime<Utc>>)
        .bind(param.created_at())
        .bind(param.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save system_parameter error: {e}"))?;

        Ok(())
    }

    async fn find_system_parameter_by_id(&self, id: &ReferenceDataId) -> Result<Option<SystemParameter>, String> {
        let row: Option<SystemParameterRow> = sqlx::query_as(
            "SELECT id, key, value, parameter_type, category, description, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.system_parameters WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_system_parameter_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_system_parameter_by_key(&self, key: &str) -> Result<Option<SystemParameter>, String> {
        let row: Option<SystemParameterRow> = sqlx::query_as(
            "SELECT id, key, value, parameter_type, category, description, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.system_parameters WHERE key = $1",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_system_parameter_by_key error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_system_parameters_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<SystemParameter>, String> {
        let rows: Vec<SystemParameterRow> = sqlx::query_as(
            "SELECT id, key, value, parameter_type, category, description, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.system_parameters
             WHERE category = $1
             ORDER BY key ASC",
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_system_parameters_by_category error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_system_parameters(&self) -> Result<Vec<SystemParameter>, String> {
        let now = Utc::now();
        let rows: Vec<SystemParameterRow> = sqlx::query_as(
            "SELECT id, key, value, parameter_type, category, description, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.system_parameters
             WHERE is_active = true AND effective_from <= $1 AND (effective_to IS NULL OR effective_to > $1)
             ORDER BY category ASC, key ASC",
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_system_parameters error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Regulatory Code Implementation ---

    async fn save_regulatory_code(&self, code: &RegulatoryCode) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.regulatory_codes
                (id, code, description_en, description_fr, classification, is_active, effective_from, effective_to, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                description_en = EXCLUDED.description_en,
                description_fr = EXCLUDED.description_fr,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(code.id().as_uuid())
        .bind(code.code())
        .bind(code.description_en())
        .bind(code.description_fr())
        .bind(code.classification().to_string())
        .bind(code.is_active())
        .bind(code.created_at())
        .bind(None::<DateTime<Utc>>)
        .bind(code.created_at())
        .bind(code.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save regulatory_code error: {e}"))?;

        Ok(())
    }

    async fn find_regulatory_code_by_id(&self, id: &ReferenceDataId) -> Result<Option<RegulatoryCode>, String> {
        let row: Option<RegulatoryCodeRow> = sqlx::query_as(
            "SELECT id, code, description_en, description_fr, classification, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.regulatory_codes WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_regulatory_code_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_regulatory_code_by_code(&self, code: &str) -> Result<Option<RegulatoryCode>, String> {
        let row: Option<RegulatoryCodeRow> = sqlx::query_as(
            "SELECT id, code, description_en, description_fr, classification, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.regulatory_codes WHERE code = $1",
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_regulatory_code_by_code error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn list_regulatory_codes(&self, limit: i64, offset: i64) -> Result<Vec<RegulatoryCode>, String> {
        let rows: Vec<RegulatoryCodeRow> = sqlx::query_as(
            "SELECT id, code, description_en, description_fr, classification, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.regulatory_codes
             ORDER BY code ASC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_regulatory_codes error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_regulatory_codes(&self) -> Result<Vec<RegulatoryCode>, String> {
        let now = Utc::now();
        let rows: Vec<RegulatoryCodeRow> = sqlx::query_as(
            "SELECT id, code, description_en, description_fr, classification, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.regulatory_codes
             WHERE is_active = true AND effective_from <= $1 AND (effective_to IS NULL OR effective_to > $1)
             ORDER BY code ASC",
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_regulatory_codes error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Fee Schedule Implementation ---

    async fn save_fee_schedule(&self, fee: &FeeScheduleReference) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO reference_data.fee_schedule_references
                (id, fee_type, amount_cents, currency_code, description_en, description_fr, is_active, effective_from, effective_to, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                amount_cents = EXCLUDED.amount_cents,
                description_en = EXCLUDED.description_en,
                description_fr = EXCLUDED.description_fr,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(fee.id().as_uuid())
        .bind(fee.fee_type().to_string())
        .bind(fee.amount_cents())
        .bind(fee.currency_code().to_string())
        .bind(fee.description_en())
        .bind(fee.description_fr())
        .bind(fee.is_active())
        .bind(fee.created_at())
        .bind(None::<DateTime<Utc>>)
        .bind(fee.created_at())
        .bind(fee.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("DB save fee_schedule error: {e}"))?;

        Ok(())
    }

    async fn find_fee_schedule_by_id(&self, id: &ReferenceDataId) -> Result<Option<FeeScheduleReference>, String> {
        let row: Option<FeeScheduleRow> = sqlx::query_as(
            "SELECT id, fee_type, amount_cents, currency_code, description_en, description_fr, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.fee_schedule_references WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB find_fee_schedule_by_id error: {e}"))?;

        match row {
            Some(r) => Ok(Some(r.into_domain()?)),
            None => Ok(None),
        }
    }

    async fn find_fee_schedules_by_type(&self, fee_type: &str) -> Result<Vec<FeeScheduleReference>, String> {
        let rows: Vec<FeeScheduleRow> = sqlx::query_as(
            "SELECT id, fee_type, amount_cents, currency_code, description_en, description_fr, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.fee_schedule_references
             WHERE fee_type = $1
             ORDER BY currency_code ASC",
        )
        .bind(fee_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB find_fee_schedules_by_type error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_fee_schedules(&self, limit: i64, offset: i64) -> Result<Vec<FeeScheduleReference>, String> {
        let rows: Vec<FeeScheduleRow> = sqlx::query_as(
            "SELECT id, fee_type, amount_cents, currency_code, description_en, description_fr, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.fee_schedule_references
             ORDER BY fee_type ASC, currency_code ASC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_fee_schedules error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn list_active_fee_schedules(&self) -> Result<Vec<FeeScheduleReference>, String> {
        let now = Utc::now();
        let rows: Vec<FeeScheduleRow> = sqlx::query_as(
            "SELECT id, fee_type, amount_cents, currency_code, description_en, description_fr, is_active, effective_from, effective_to, created_at, updated_at
             FROM reference_data.fee_schedule_references
             WHERE is_active = true AND effective_from <= $1 AND (effective_to IS NULL OR effective_to > $1)
             ORDER BY fee_type ASC, currency_code ASC",
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("DB list_active_fee_schedules error: {e}"))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}
