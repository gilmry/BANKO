use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use banko_application::islamic_banking::IIslamicBankingRepository;
use banko_domain::islamic_banking::*;
use banko_domain::shared::{Currency, CustomerId, Money};

pub struct PgIslamicBankingRepository {
    pool: PgPool,
}

impl PgIslamicBankingRepository {
    pub fn new(pool: PgPool) -> Self {
        PgIslamicBankingRepository { pool }
    }
}

// --- Murabaha Row ---

#[derive(Debug, sqlx::FromRow)]
struct MurabahaRow {
    id: Uuid,
    customer_id: Uuid,
    cost_price: i64,
    profit_margin: f64,
    selling_price: i64,
    installments: i32,
    asset_description: String,
    delivery_date: DateTime<Utc>,
    status: String,
    currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MurabahaRow {
    fn into_domain(self) -> Result<MurabahaContract, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let cost_price = Money::from_cents(self.cost_price, currency);
        let selling_price = Money::from_cents(self.selling_price, currency);
        let status = MurabahaStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(MurabahaContract::reconstitute(
            IslamicContractId::from_uuid(self.id),
            CustomerId::from_uuid(self.customer_id),
            cost_price,
            self.profit_margin,
            selling_price,
            self.installments as u32,
            self.asset_description,
            self.delivery_date,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Ijara Row ---

#[derive(Debug, sqlx::FromRow)]
struct IjaraRow {
    id: Uuid,
    customer_id: Uuid,
    asset_id: String,
    monthly_rental: i64,
    lease_start: DateTime<Utc>,
    lease_end: DateTime<Utc>,
    purchase_option_price: i64,
    maintenance_responsibility: String,
    status: String,
    currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IjaraRow {
    fn into_domain(self) -> Result<IjaraContract, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let monthly_rental = Money::from_cents(self.monthly_rental, currency);
        let purchase_option_price = Money::from_cents(self.purchase_option_price, currency);
        let status = IjaraStatus::from_str(&self.status).map_err(|e| e.to_string())?;
        let maintenance = MaintenanceResponsibility::from_str(&self.maintenance_responsibility)
            .map_err(|e| e.to_string())?;

        Ok(IjaraContract::reconstitute(
            IslamicContractId::from_uuid(self.id),
            CustomerId::from_uuid(self.customer_id),
            self.asset_id,
            monthly_rental,
            self.lease_start,
            self.lease_end,
            purchase_option_price,
            maintenance,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Musharaka Row ---

#[derive(Debug, sqlx::FromRow)]
struct MusharakaRow {
    id: Uuid,
    customer_id: Uuid,
    total_capital: i64,
    bank_share_pct: f64,
    client_share_pct: f64,
    profit_sharing_ratio: f64,
    diminishing_schedule: String, // JSON string
    status: String,
    currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MusharakaRow {
    fn into_domain(self) -> Result<MusharakaContract, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let total_capital = Money::from_cents(self.total_capital, currency);
        let status = MusharakaStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        // Parse JSON schedule
        let schedule: Vec<(u32, f64)> = serde_json::from_str(&self.diminishing_schedule)
            .map_err(|e| format!("Failed to parse schedule: {}", e))?;

        Ok(MusharakaContract::reconstitute(
            IslamicContractId::from_uuid(self.id),
            CustomerId::from_uuid(self.customer_id),
            total_capital,
            self.bank_share_pct,
            self.client_share_pct,
            self.profit_sharing_ratio,
            schedule,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Mudaraba Row ---

#[derive(Debug, sqlx::FromRow)]
struct MudarabaRow {
    id: Uuid,
    customer_id: Uuid,
    capital_amount: i64,
    profit_sharing_ratio: f64,
    investment_type: String,
    reporting_period: i32,
    status: String,
    currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MudarabaRow {
    fn into_domain(self) -> Result<MudarabaContract, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let capital_amount = Money::from_cents(self.capital_amount, currency);
        let status = MudarabaStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(MudarabaContract::reconstitute(
            IslamicContractId::from_uuid(self.id),
            CustomerId::from_uuid(self.customer_id),
            capital_amount,
            self.profit_sharing_ratio,
            self.investment_type,
            self.reporting_period as u32,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Sukuk Row ---

#[derive(Debug, sqlx::FromRow)]
struct SukukRow {
    id: Uuid,
    denomination: i64,
    total_amount: i64,
    units_issued: i64,
    coupon_rate: f64,
    maturity_date: DateTime<Utc>,
    underlying_asset: String,
    status: String,
    currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SukukRow {
    fn into_domain(self) -> Result<SukukIssuance, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let denomination = Money::from_cents(self.denomination, currency);
        let total_amount = Money::from_cents(self.total_amount, currency);
        let status = SukukStatus::from_str(&self.status).map_err(|e| e.to_string())?;

        Ok(SukukIssuance::reconstitute(
            IslamicContractId::from_uuid(self.id),
            denomination,
            total_amount,
            self.units_issued as u64,
            self.coupon_rate,
            self.maturity_date,
            self.underlying_asset,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Zakat Row ---

#[derive(Debug, sqlx::FromRow)]
struct ZakatRow {
    id: Uuid,
    customer_id: Uuid,
    assessment_year: i32,
    nisab_threshold: i64,
    eligible_wealth: i64,
    zakat_amount: i64,
    payment_status: String,
    currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ZakatRow {
    fn into_domain(self) -> Result<ZakatCalculation, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let nisab_threshold = Money::from_cents(self.nisab_threshold, currency);
        let eligible_wealth = Money::from_cents(self.eligible_wealth, currency);
        let zakat_amount = Money::from_cents(self.zakat_amount, currency);
        let status = ZakatPaymentStatus::from_str(&self.payment_status)
            .map_err(|e| e.to_string())?;

        Ok(ZakatCalculation::reconstitute(
            IslamicContractId::from_uuid(self.id),
            CustomerId::from_uuid(self.customer_id),
            self.assessment_year as u32,
            nisab_threshold,
            eligible_wealth,
            zakat_amount,
            status,
            self.created_at,
            self.updated_at,
        ))
    }
}

// --- Sharia Board Decision Row ---

#[derive(Debug, sqlx::FromRow)]
struct ShariaBoardRow {
    id: Uuid,
    product_type: String,
    ruling: String,
    conditions: String, // JSON array
    board_members: String, // JSON array
    quorum_met: bool,
    decision_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl ShariaBoardRow {
    fn into_domain(self) -> Result<ShariaBoardDecision, String> {
        let product = ProductType::from_str(&self.product_type).map_err(|e| e.to_string())?;
        let ruling = ShariaRuling::from_str(&self.ruling).map_err(|e| e.to_string())?;
        let conditions: Vec<String> =
            serde_json::from_str(&self.conditions).unwrap_or_default();
        let members: Vec<String> = serde_json::from_str(&self.board_members)
            .map_err(|e| format!("Failed to parse members: {}", e))?;

        Ok(ShariaBoardDecision::reconstitute(
            IslamicContractId::from_uuid(self.id),
            product,
            ruling,
            conditions,
            members,
            self.quorum_met,
            self.decision_date,
            self.created_at,
        ))
    }
}

// --- Profit Distribution Row ---

#[derive(Debug, sqlx::FromRow)]
struct ProfitDistributionRow {
    id: Uuid,
    period: i32,
    total_profit: i64,
    depositor_pool_share: i64,
    bank_share: i64,
    per_account_distributions: String, // JSON
    currency: String,
    created_at: DateTime<Utc>,
}

impl ProfitDistributionRow {
    fn into_domain(self) -> Result<ProfitDistribution, String> {
        let currency = Currency::from_code(&self.currency).map_err(|e| e.to_string())?;
        let total_profit = Money::from_cents(self.total_profit, currency);
        let depositor_pool_share = Money::from_cents(self.depositor_pool_share, currency);
        let bank_share = Money::from_cents(self.bank_share, currency);

        // Parse distributions
        let distributions_raw: Vec<(String, i64)> =
            serde_json::from_str(&self.per_account_distributions)
                .map_err(|e| format!("Failed to parse distributions: {}", e))?;

        let distributions: Vec<(CustomerId, Money)> = distributions_raw
            .into_iter()
            .map(|(cust_id, amount)| {
                (
                    CustomerId::from_uuid(Uuid::parse_str(&cust_id).unwrap()),
                    Money::from_cents(amount, currency),
                )
            })
            .collect();

        Ok(ProfitDistribution::reconstitute(
            IslamicContractId::from_uuid(self.id),
            self.period as u32,
            total_profit,
            depositor_pool_share,
            bank_share,
            distributions,
            self.created_at,
        ))
    }
}

// --- Repository Implementation ---

#[async_trait]
impl IIslamicBankingRepository for PgIslamicBankingRepository {
    // --- Murabaha ---
    async fn save_murabaha(&self, contract: &MurabahaContract) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO islamic_banking_murabaha
             (id, customer_id, cost_price, profit_margin, selling_price, installments,
              asset_description, delivery_date, status, currency, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
             ON CONFLICT (id) DO UPDATE SET
             status = $9, updated_at = $12",
        )
        .bind(contract.id().as_uuid())
        .bind(contract.customer_id().as_uuid())
        .bind(contract.cost_price().amount_cents())
        .bind(contract.profit_margin())
        .bind(contract.selling_price().amount_cents())
        .bind(contract.installments() as i32)
        .bind(contract.asset_description())
        .bind(contract.delivery_date())
        .bind(contract.status().as_str())
        .bind(contract.cost_price().currency().to_string())
        .bind(contract.created_at())
        .bind(contract.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Murabaha: {}", e))?;

        Ok(())
    }

    async fn find_murabaha(&self, id: IslamicContractId) -> Result<Option<MurabahaContract>, String> {
        let row = sqlx::query_as::<_, MurabahaRow>(
            "SELECT * FROM islamic_banking_murabaha WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Murabaha: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_murabaha_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MurabahaContract>, String> {
        let rows = sqlx::query_as::<_, MurabahaRow>(
            "SELECT * FROM islamic_banking_murabaha WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Murabaha contracts: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Ijara ---
    async fn save_ijara(&self, contract: &IjaraContract) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO islamic_banking_ijara
             (id, customer_id, asset_id, monthly_rental, lease_start, lease_end,
              purchase_option_price, maintenance_responsibility, status, currency, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
             ON CONFLICT (id) DO UPDATE SET
             status = $9, updated_at = $12",
        )
        .bind(contract.id().as_uuid())
        .bind(contract.customer_id().as_uuid())
        .bind(contract.asset_id())
        .bind(contract.monthly_rental().amount_cents())
        .bind(contract.lease_start())
        .bind(contract.lease_end())
        .bind(contract.purchase_option_price().amount_cents())
        .bind(contract.maintenance_responsibility().as_str())
        .bind(contract.status().as_str())
        .bind(contract.monthly_rental().currency().to_string())
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Ijara: {}", e))?;

        Ok(())
    }

    async fn find_ijara(&self, id: IslamicContractId) -> Result<Option<IjaraContract>, String> {
        let row = sqlx::query_as::<_, IjaraRow>(
            "SELECT * FROM islamic_banking_ijara WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Ijara: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_ijara_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<IjaraContract>, String> {
        let rows = sqlx::query_as::<_, IjaraRow>(
            "SELECT * FROM islamic_banking_ijara WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Ijara contracts: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Musharaka ---
    async fn save_musharaka(&self, contract: &MusharakaContract) -> Result<(), String> {
        let schedule_json = serde_json::to_string(contract.diminishing_schedule())
            .map_err(|e| format!("Failed to serialize schedule: {}", e))?;

        sqlx::query(
            "INSERT INTO islamic_banking_musharaka
             (id, customer_id, total_capital, bank_share_pct, client_share_pct,
              profit_sharing_ratio, diminishing_schedule, status, currency, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             ON CONFLICT (id) DO UPDATE SET
             status = $8, updated_at = $11",
        )
        .bind(contract.id().as_uuid())
        .bind(contract.customer_id().as_uuid())
        .bind(contract.total_capital().amount_cents())
        .bind(contract.bank_share_pct())
        .bind(contract.client_share_pct())
        .bind(contract.profit_sharing_ratio())
        .bind(schedule_json)
        .bind(contract.status().as_str())
        .bind(contract.total_capital().currency().to_string())
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Musharaka: {}", e))?;

        Ok(())
    }

    async fn find_musharaka(
        &self,
        id: IslamicContractId,
    ) -> Result<Option<MusharakaContract>, String> {
        let row = sqlx::query_as::<_, MusharakaRow>(
            "SELECT * FROM islamic_banking_musharaka WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Musharaka: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_musharaka_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MusharakaContract>, String> {
        let rows = sqlx::query_as::<_, MusharakaRow>(
            "SELECT * FROM islamic_banking_musharaka WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Musharaka contracts: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Mudaraba ---
    async fn save_mudaraba(&self, contract: &MudarabaContract) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO islamic_banking_mudaraba
             (id, customer_id, capital_amount, profit_sharing_ratio, investment_type,
              reporting_period, status, currency, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             ON CONFLICT (id) DO UPDATE SET
             status = $7, updated_at = $10",
        )
        .bind(contract.id().as_uuid())
        .bind(contract.customer_id().as_uuid())
        .bind(contract.capital_amount().amount_cents())
        .bind(contract.profit_sharing_ratio())
        .bind(contract.investment_type())
        .bind(contract.reporting_period() as i32)
        .bind(contract.status().as_str())
        .bind(contract.capital_amount().currency().to_string())
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Mudaraba: {}", e))?;

        Ok(())
    }

    async fn find_mudaraba(
        &self,
        id: IslamicContractId,
    ) -> Result<Option<MudarabaContract>, String> {
        let row = sqlx::query_as::<_, MudarabaRow>(
            "SELECT * FROM islamic_banking_mudaraba WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Mudaraba: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_mudaraba_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MudarabaContract>, String> {
        let rows = sqlx::query_as::<_, MudarabaRow>(
            "SELECT * FROM islamic_banking_mudaraba WHERE customer_id = $1 ORDER BY created_at DESC",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Mudaraba contracts: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Sukuk ---
    async fn save_sukuk(&self, sukuk: &SukukIssuance) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO islamic_banking_sukuk
             (id, denomination, total_amount, units_issued, coupon_rate,
              maturity_date, underlying_asset, status, currency, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             ON CONFLICT (id) DO UPDATE SET
             status = $8, updated_at = $11",
        )
        .bind(sukuk.id().as_uuid())
        .bind(sukuk.denomination().amount_cents())
        .bind(sukuk.total_amount().amount_cents())
        .bind(sukuk.units_issued() as i64)
        .bind(sukuk.coupon_rate())
        .bind(sukuk.maturity_date())
        .bind(sukuk.underlying_asset())
        .bind(sukuk.status().as_str())
        .bind(sukuk.denomination().currency().to_string())
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Sukuk: {}", e))?;

        Ok(())
    }

    async fn find_sukuk(&self, id: IslamicContractId) -> Result<Option<SukukIssuance>, String> {
        let row = sqlx::query_as::<_, SukukRow>(
            "SELECT * FROM islamic_banking_sukuk WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Sukuk: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_sukuk_outstanding(&self) -> Result<Vec<SukukIssuance>, String> {
        let rows = sqlx::query_as::<_, SukukRow>(
            "SELECT * FROM islamic_banking_sukuk WHERE status = 'Outstanding' ORDER BY maturity_date ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find outstanding Sukuk: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Zakat ---
    async fn save_zakat(&self, zakat: &ZakatCalculation) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO islamic_banking_zakat
             (id, customer_id, assessment_year, nisab_threshold, eligible_wealth,
              zakat_amount, payment_status, currency, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             ON CONFLICT (id) DO UPDATE SET
             payment_status = $7, updated_at = $10",
        )
        .bind(zakat.id().as_uuid())
        .bind(zakat.customer_id().as_uuid())
        .bind(zakat.assessment_year() as i32)
        .bind(zakat.nisab_threshold().amount_cents())
        .bind(zakat.eligible_wealth().amount_cents())
        .bind(zakat.zakat_amount().amount_cents())
        .bind(zakat.payment_status().as_str())
        .bind(zakat.zakat_amount().currency().to_string())
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Zakat: {}", e))?;

        Ok(())
    }

    async fn find_zakat(&self, id: IslamicContractId) -> Result<Option<ZakatCalculation>, String> {
        let row = sqlx::query_as::<_, ZakatRow>(
            "SELECT * FROM islamic_banking_zakat WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Zakat: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_zakat_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<ZakatCalculation>, String> {
        let rows = sqlx::query_as::<_, ZakatRow>(
            "SELECT * FROM islamic_banking_zakat WHERE customer_id = $1 ORDER BY assessment_year DESC",
        )
        .bind(customer_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Zakat records: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    async fn find_zakat_by_year(
        &self,
        customer_id: &CustomerId,
        year: u32,
    ) -> Result<Option<ZakatCalculation>, String> {
        let row = sqlx::query_as::<_, ZakatRow>(
            "SELECT * FROM islamic_banking_zakat WHERE customer_id = $1 AND assessment_year = $2",
        )
        .bind(customer_id.as_uuid())
        .bind(year as i32)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Zakat: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    // --- Sharia Board ---
    async fn save_sharia_decision(&self, decision: &ShariaBoardDecision) -> Result<(), String> {
        let conditions_json = serde_json::to_string(decision.conditions())
            .map_err(|e| format!("Failed to serialize conditions: {}", e))?;
        let members_json = serde_json::to_string(decision.board_members())
            .map_err(|e| format!("Failed to serialize members: {}", e))?;

        sqlx::query(
            "INSERT INTO islamic_banking_sharia_decisions
             (id, product_type, ruling, conditions, board_members, quorum_met, decision_date, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET quorum_met = $6",
        )
        .bind(decision.id().as_uuid())
        .bind(decision.product_type().as_str())
        .bind(decision.ruling().as_str())
        .bind(conditions_json)
        .bind(members_json)
        .bind(decision.quorum_met())
        .bind(decision.decision_date())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save Sharia decision: {}", e))?;

        Ok(())
    }

    async fn find_sharia_decision(
        &self,
        id: IslamicContractId,
    ) -> Result<Option<ShariaBoardDecision>, String> {
        let row = sqlx::query_as::<_, ShariaBoardRow>(
            "SELECT * FROM islamic_banking_sharia_decisions WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Sharia decision: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_sharia_decisions_by_product(
        &self,
        product_type: &str,
    ) -> Result<Vec<ShariaBoardDecision>, String> {
        let rows = sqlx::query_as::<_, ShariaBoardRow>(
            "SELECT * FROM islamic_banking_sharia_decisions WHERE product_type = $1 ORDER BY decision_date DESC",
        )
        .bind(product_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find Sharia decisions: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }

    // --- Profit Distribution ---
    async fn save_profit_distribution(
        &self,
        distribution: &ProfitDistribution,
    ) -> Result<(), String> {
        let distributions_json: Vec<(String, i64)> = distribution
            .per_account_distributions()
            .iter()
            .map(|(cust_id, money)| (cust_id.as_uuid().to_string(), money.amount_cents()))
            .collect();

        let distributions_str = serde_json::to_string(&distributions_json)
            .map_err(|e| format!("Failed to serialize distributions: {}", e))?;

        sqlx::query(
            "INSERT INTO islamic_banking_profit_distributions
             (id, period, total_profit, depositor_pool_share, bank_share,
              per_account_distributions, currency, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(distribution.id().as_uuid())
        .bind(distribution.period() as i32)
        .bind(distribution.total_profit().amount_cents())
        .bind(distribution.depositor_pool_share().amount_cents())
        .bind(distribution.bank_share().amount_cents())
        .bind(distributions_str)
        .bind(distribution.total_profit().currency().to_string())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save profit distribution: {}", e))?;

        Ok(())
    }

    async fn find_profit_distribution(
        &self,
        id: IslamicContractId,
    ) -> Result<Option<ProfitDistribution>, String> {
        let row = sqlx::query_as::<_, ProfitDistributionRow>(
            "SELECT * FROM islamic_banking_profit_distributions WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find profit distribution: {}", e))?;

        row.map(|r| r.into_domain()).transpose()
    }

    async fn find_profit_distributions_by_period(
        &self,
        period: u32,
    ) -> Result<Vec<ProfitDistribution>, String> {
        let rows = sqlx::query_as::<_, ProfitDistributionRow>(
            "SELECT * FROM islamic_banking_profit_distributions WHERE period = $1",
        )
        .bind(period as i32)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to find distributions: {}", e))?;

        rows.into_iter().map(|r| r.into_domain()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murabaha_row_serialization() {
        // Tests would require database setup
        // This is a compile-time verification
    }
}
