use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{Currency, CustomerId, Money};

use super::errors::IslamicBankingError;
use super::invariants::*;
use super::value_objects::*;

// --- Murabaha Contract (Cost-Plus-Profit Sale Financing) ---

/// Murabaha: Islamic financing via cost-plus-profit sale
/// - Bank buys asset at cost_price
/// - Bank sells to customer at cost_price + profit
/// - Profit margin disclosed upfront (no riba/interest)
/// - Customer pays in agreed installments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MurabahaContract {
    id: IslamicContractId,
    customer_id: CustomerId,
    cost_price: Money,
    profit_margin: f64, // 0.0 to 1.0
    selling_price: Money,
    installments: u32,
    asset_description: String,
    delivery_date: DateTime<Utc>,
    status: MurabahaStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MurabahaContract {
    /// Create a new Murabaha contract with invariant checks
    pub fn new(
        customer_id: CustomerId,
        cost_price: Money,
        profit_margin: f64,
        installments: u32,
        asset_description: String,
        delivery_date: DateTime<Utc>,
    ) -> Result<Self, IslamicBankingError> {
        // Validate cost_price is positive
        if cost_price.is_negative() || cost_price.is_zero() {
            return Err(IslamicBankingError::InvalidMurabahaMargin(
                "cost_price must be positive".to_string(),
            ));
        }

        // Validate profit margin: 0-100%
        if profit_margin < 0.0 || profit_margin > 1.0 {
            return Err(IslamicBankingError::InvalidMurabahaMargin(
                format!("profit_margin must be 0-1.0, got {}", profit_margin),
            ));
        }

        // Validate installments > 0
        if installments == 0 {
            return Err(IslamicBankingError::InvalidMurabahaInstallments);
        }

        // Validate asset description is not empty
        if asset_description.trim().is_empty() {
            return Err(IslamicBankingError::MissingAssetDescription);
        }

        // Calculate selling_price = cost_price * (1 + profit_margin)
        let selling_price = Money::new(
            cost_price.amount() * (1.0 + profit_margin),
            cost_price.currency(),
        )
        .map_err(|_| IslamicBankingError::ProfitMarginMismatch)?;

        Ok(MurabahaContract {
            id: IslamicContractId::new(),
            customer_id,
            cost_price,
            profit_margin,
            selling_price,
            installments,
            asset_description,
            delivery_date,
            status: MurabahaStatus::Proposed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        customer_id: CustomerId,
        cost_price: Money,
        profit_margin: f64,
        selling_price: Money,
        installments: u32,
        asset_description: String,
        delivery_date: DateTime<Utc>,
        status: MurabahaStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        MurabahaContract {
            id,
            customer_id,
            cost_price,
            profit_margin,
            selling_price,
            installments,
            asset_description,
            delivery_date,
            status,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn cost_price(&self) -> &Money {
        &self.cost_price
    }
    pub fn profit_margin(&self) -> f64 {
        self.profit_margin
    }
    pub fn selling_price(&self) -> &Money {
        &self.selling_price
    }
    pub fn installments(&self) -> u32 {
        self.installments
    }
    pub fn asset_description(&self) -> &str {
        &self.asset_description
    }
    pub fn delivery_date(&self) -> DateTime<Utc> {
        self.delivery_date
    }
    pub fn status(&self) -> MurabahaStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // State transitions
    pub fn approve(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MurabahaStatus::Proposed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only approve from Proposed status".to_string(),
            ));
        }
        self.status = MurabahaStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MurabahaStatus::Approved {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only activate from Approved status".to_string(),
            ));
        }
        self.status = MurabahaStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MurabahaStatus::Active {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only complete from Active status".to_string(),
            ));
        }
        self.status = MurabahaStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_defaulted(&mut self) -> Result<(), IslamicBankingError> {
        if self.status == MurabahaStatus::Completed || self.status == MurabahaStatus::Defaulted {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Cannot mark as defaulted from current status".to_string(),
            ));
        }
        self.status = MurabahaStatus::Defaulted;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- Ijara Contract (Islamic Leasing) ---

/// Ijara: Islamic leasing with optional purchase (Ijara wa Iqtina)
/// - Bank owns asset and leases to customer
/// - Monthly rental payments without interest
/// - Optional purchase at end of lease (wa Iqtina)
/// - Asset must exist and be tangible
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IjaraContract {
    id: IslamicContractId,
    customer_id: CustomerId,
    asset_id: String,
    monthly_rental: Money,
    lease_start: DateTime<Utc>,
    lease_end: DateTime<Utc>,
    purchase_option_price: Money,
    maintenance_responsibility: MaintenanceResponsibility,
    status: IjaraStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IjaraContract {
    pub fn new(
        customer_id: CustomerId,
        asset_id: String,
        monthly_rental: Money,
        lease_start: DateTime<Utc>,
        lease_end: DateTime<Utc>,
        purchase_option_price: Money,
        maintenance_responsibility: MaintenanceResponsibility,
    ) -> Result<Self, IslamicBankingError> {
        if asset_id.trim().is_empty() {
            return Err(IslamicBankingError::InvalidIjaraAsset(
                "asset_id cannot be empty".to_string(),
            ));
        }

        if monthly_rental.is_negative() || monthly_rental.is_zero() {
            return Err(IslamicBankingError::InvalidIjaraRental(
                "monthly_rental must be positive".to_string(),
            ));
        }

        if lease_end <= lease_start {
            return Err(IslamicBankingError::InvalidLeasePeriod(
                "lease_end must be after lease_start".to_string(),
            ));
        }

        if purchase_option_price.is_negative() || purchase_option_price.is_zero() {
            return Err(IslamicBankingError::InvalidPurchaseOption);
        }

        Ok(IjaraContract {
            id: IslamicContractId::new(),
            customer_id,
            asset_id,
            monthly_rental,
            lease_start,
            lease_end,
            purchase_option_price,
            maintenance_responsibility,
            status: IjaraStatus::Proposed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        customer_id: CustomerId,
        asset_id: String,
        monthly_rental: Money,
        lease_start: DateTime<Utc>,
        lease_end: DateTime<Utc>,
        purchase_option_price: Money,
        maintenance_responsibility: MaintenanceResponsibility,
        status: IjaraStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        IjaraContract {
            id,
            customer_id,
            asset_id,
            monthly_rental,
            lease_start,
            lease_end,
            purchase_option_price,
            maintenance_responsibility,
            status,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }
    pub fn monthly_rental(&self) -> &Money {
        &self.monthly_rental
    }
    pub fn lease_start(&self) -> DateTime<Utc> {
        self.lease_start
    }
    pub fn lease_end(&self) -> DateTime<Utc> {
        self.lease_end
    }
    pub fn purchase_option_price(&self) -> &Money {
        &self.purchase_option_price
    }
    pub fn maintenance_responsibility(&self) -> MaintenanceResponsibility {
        self.maintenance_responsibility
    }
    pub fn status(&self) -> IjaraStatus {
        self.status
    }

    // State transitions
    pub fn approve(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != IjaraStatus::Proposed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only approve from Proposed status".to_string(),
            ));
        }
        self.status = IjaraStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != IjaraStatus::Approved {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only activate from Approved status".to_string(),
            ));
        }
        self.status = IjaraStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != IjaraStatus::Active {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only complete from Active status".to_string(),
            ));
        }
        self.status = IjaraStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), IslamicBankingError> {
        if self.status == IjaraStatus::Completed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Cannot terminate completed lease".to_string(),
            ));
        }
        self.status = IjaraStatus::Terminated;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- Musharaka Contract (Partnership Financing) ---

/// Musharaka: Islamic partnership with diminishing bank share
/// - Bank and customer jointly invest capital
/// - Profits/losses shared per profit-sharing ratio
/// - Bank share gradually diminishes (diminishing Musharaka)
/// - Both parties participate in management (except Mudaraba)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusharakaContract {
    id: IslamicContractId,
    customer_id: CustomerId,
    total_capital: Money,
    bank_share_pct: f64,      // 0.0 to 100.0
    client_share_pct: f64,     // 0.0 to 100.0
    profit_sharing_ratio: f64, // bank's share of profits
    diminishing_schedule: Vec<(u32, f64)>, // (period, bank_share_pct)
    status: MusharakaStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MusharakaContract {
    pub fn new(
        customer_id: CustomerId,
        total_capital: Money,
        bank_share_pct: f64,
        client_share_pct: f64,
        profit_sharing_ratio: f64,
        diminishing_schedule: Vec<(u32, f64)>,
    ) -> Result<Self, IslamicBankingError> {
        // Validate shares sum to 100%
        if (bank_share_pct + client_share_pct - 100.0).abs() > 0.01 {
            return Err(IslamicBankingError::SharesSumError(
                format!("{}% + {}% != 100%", bank_share_pct, client_share_pct),
            ));
        }

        // Validate each share is between 0 and 100
        if bank_share_pct < 0.0 || bank_share_pct > 100.0 {
            return Err(IslamicBankingError::InvalidSharePercentage(
                format!("bank_share_pct: {}", bank_share_pct),
            ));
        }

        if client_share_pct < 0.0 || client_share_pct > 100.0 {
            return Err(IslamicBankingError::InvalidSharePercentage(
                format!("client_share_pct: {}", client_share_pct),
            ));
        }

        // Validate profit sharing ratio
        if profit_sharing_ratio < 0.0 || profit_sharing_ratio > 1.0 {
            return Err(IslamicBankingError::InvalidProfitRatio);
        }

        // Validate diminishing schedule is not empty
        if diminishing_schedule.is_empty() {
            return Err(IslamicBankingError::EmptyDiminishingSchedule);
        }

        // Validate capital is positive
        if total_capital.is_negative() || total_capital.is_zero() {
            return Err(IslamicBankingError::InvalidCapitalAmount(
                "total_capital must be positive".to_string(),
            ));
        }

        Ok(MusharakaContract {
            id: IslamicContractId::new(),
            customer_id,
            total_capital,
            bank_share_pct,
            client_share_pct,
            profit_sharing_ratio,
            diminishing_schedule,
            status: MusharakaStatus::Proposed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        customer_id: CustomerId,
        total_capital: Money,
        bank_share_pct: f64,
        client_share_pct: f64,
        profit_sharing_ratio: f64,
        diminishing_schedule: Vec<(u32, f64)>,
        status: MusharakaStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        MusharakaContract {
            id,
            customer_id,
            total_capital,
            bank_share_pct,
            client_share_pct,
            profit_sharing_ratio,
            diminishing_schedule,
            status,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn total_capital(&self) -> &Money {
        &self.total_capital
    }
    pub fn bank_share_pct(&self) -> f64 {
        self.bank_share_pct
    }
    pub fn client_share_pct(&self) -> f64 {
        self.client_share_pct
    }
    pub fn profit_sharing_ratio(&self) -> f64 {
        self.profit_sharing_ratio
    }
    pub fn diminishing_schedule(&self) -> &[(u32, f64)] {
        &self.diminishing_schedule
    }
    pub fn status(&self) -> MusharakaStatus {
        self.status
    }

    // State transitions
    pub fn approve(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MusharakaStatus::Proposed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only approve from Proposed status".to_string(),
            ));
        }
        self.status = MusharakaStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MusharakaStatus::Approved {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only activate from Approved status".to_string(),
            ));
        }
        self.status = MusharakaStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MusharakaStatus::Active {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only complete from Active status".to_string(),
            ));
        }
        self.status = MusharakaStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn dissolve(&mut self) -> Result<(), IslamicBankingError> {
        if self.status == MusharakaStatus::Completed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Cannot dissolve completed partnership".to_string(),
            ));
        }
        self.status = MusharakaStatus::Dissolved;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- Mudaraba Contract (Profit-Sharing Investment) ---

/// Mudaraba: Islamic profit-sharing investment
/// - Bank (Rabb al-Mal) provides capital
/// - Customer (Mudarib) manages investment
/// - Profits split by agreed ratio
/// - Losses borne by capital provider (bank)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MudarabaContract {
    id: IslamicContractId,
    customer_id: CustomerId,
    capital_amount: Money,
    profit_sharing_ratio: f64, // bank's share of profits
    investment_type: String,
    reporting_period: u32, // days
    status: MudarabaStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MudarabaContract {
    pub fn new(
        customer_id: CustomerId,
        capital_amount: Money,
        profit_sharing_ratio: f64,
        investment_type: String,
        reporting_period: u32,
    ) -> Result<Self, IslamicBankingError> {
        if capital_amount.is_negative() || capital_amount.is_zero() {
            return Err(IslamicBankingError::InvalidCapitalAmount(
                "capital_amount must be positive".to_string(),
            ));
        }

        if profit_sharing_ratio < 0.0 || profit_sharing_ratio > 1.0 {
            return Err(IslamicBankingError::InvalidMudarabaRatio);
        }

        if investment_type.trim().is_empty() {
            return Err(IslamicBankingError::InvalidInvestmentType);
        }

        if reporting_period == 0 {
            return Err(IslamicBankingError::InvalidDistributionPeriod);
        }

        Ok(MudarabaContract {
            id: IslamicContractId::new(),
            customer_id,
            capital_amount,
            profit_sharing_ratio,
            investment_type,
            reporting_period,
            status: MudarabaStatus::Proposed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        customer_id: CustomerId,
        capital_amount: Money,
        profit_sharing_ratio: f64,
        investment_type: String,
        reporting_period: u32,
        status: MudarabaStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        MudarabaContract {
            id,
            customer_id,
            capital_amount,
            profit_sharing_ratio,
            investment_type,
            reporting_period,
            status,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn capital_amount(&self) -> &Money {
        &self.capital_amount
    }
    pub fn profit_sharing_ratio(&self) -> f64 {
        self.profit_sharing_ratio
    }
    pub fn investment_type(&self) -> &str {
        &self.investment_type
    }
    pub fn reporting_period(&self) -> u32 {
        self.reporting_period
    }
    pub fn status(&self) -> MudarabaStatus {
        self.status
    }

    // State transitions
    pub fn approve(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MudarabaStatus::Proposed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only approve from Proposed status".to_string(),
            ));
        }
        self.status = MudarabaStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MudarabaStatus::Approved {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only activate from Approved status".to_string(),
            ));
        }
        self.status = MudarabaStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != MudarabaStatus::Active {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only complete from Active status".to_string(),
            ));
        }
        self.status = MudarabaStatus::Completed;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), IslamicBankingError> {
        if self.status == MudarabaStatus::Completed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Cannot terminate completed Mudaraba".to_string(),
            ));
        }
        self.status = MudarabaStatus::Terminated;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- Sukuk Issuance (Islamic Bonds) ---

/// Sukuk: Islamic bonds backed by tangible assets
/// - Asset-backed securities (not debt-based)
/// - Investors own share of underlying assets
/// - Coupon = profit rate (not interest)
/// - Tangible asset backing required per Sharia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SukukIssuance {
    id: IslamicContractId,
    denomination: Money,
    total_amount: Money,
    units_issued: u64,
    coupon_rate: f64, // 0.0 to 1.0
    maturity_date: DateTime<Utc>,
    underlying_asset: String,
    status: SukukStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SukukIssuance {
    pub fn new(
        denomination: Money,
        total_amount: Money,
        units_issued: u64,
        coupon_rate: f64,
        maturity_date: DateTime<Utc>,
        underlying_asset: String,
    ) -> Result<Self, IslamicBankingError> {
        if denomination.is_negative() || denomination.is_zero() {
            return Err(IslamicBankingError::InvalidSukukDenomination(
                "denomination must be positive".to_string(),
            ));
        }

        if total_amount.is_negative() || total_amount.is_zero() {
            return Err(IslamicBankingError::InvalidSukukAmount(
                "total_amount must be positive".to_string(),
            ));
        }

        if units_issued == 0 {
            return Err(IslamicBankingError::InvalidSukukAmount(
                "units_issued must be positive".to_string(),
            ));
        }

        if coupon_rate < 0.0 || coupon_rate > 1.0 {
            return Err(IslamicBankingError::InvalidSukukCoupon(
                format!("coupon_rate must be 0-1.0, got {}", coupon_rate),
            ));
        }

        if maturity_date <= Utc::now() {
            return Err(IslamicBankingError::InvalidMaturityDate);
        }

        if underlying_asset.trim().is_empty() {
            return Err(IslamicBankingError::MissingUnderlyingAsset);
        }

        Ok(SukukIssuance {
            id: IslamicContractId::new(),
            denomination,
            total_amount,
            units_issued,
            coupon_rate,
            maturity_date,
            underlying_asset,
            status: SukukStatus::Proposed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        denomination: Money,
        total_amount: Money,
        units_issued: u64,
        coupon_rate: f64,
        maturity_date: DateTime<Utc>,
        underlying_asset: String,
        status: SukukStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SukukIssuance {
            id,
            denomination,
            total_amount,
            units_issued,
            coupon_rate,
            maturity_date,
            underlying_asset,
            status,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn denomination(&self) -> &Money {
        &self.denomination
    }
    pub fn total_amount(&self) -> &Money {
        &self.total_amount
    }
    pub fn units_issued(&self) -> u64 {
        self.units_issued
    }
    pub fn coupon_rate(&self) -> f64 {
        self.coupon_rate
    }
    pub fn maturity_date(&self) -> DateTime<Utc> {
        self.maturity_date
    }
    pub fn underlying_asset(&self) -> &str {
        &self.underlying_asset
    }
    pub fn status(&self) -> SukukStatus {
        self.status
    }

    // State transitions
    pub fn approve(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != SukukStatus::Proposed {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only approve from Proposed status".to_string(),
            ));
        }
        self.status = SukukStatus::Approved;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn issue(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != SukukStatus::Approved {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only issue from Approved status".to_string(),
            ));
        }
        self.status = SukukStatus::Outstanding;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reach_maturity(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != SukukStatus::Outstanding {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only reach maturity from Outstanding status".to_string(),
            ));
        }
        self.status = SukukStatus::MaturityReached;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn redeem(&mut self) -> Result<(), IslamicBankingError> {
        if self.status != SukukStatus::MaturityReached {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Can only redeem from MaturityReached status".to_string(),
            ));
        }
        self.status = SukukStatus::Redeemed;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- Zakat Calculation ---

/// Zakat: 2.5% annual wealth tax (mandatory for Muslims above Nisab)
/// - Calculated on eligible wealth
/// - Nisab threshold per Islamic jurisprudence
/// - Due annually on Hijri calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZakatCalculation {
    id: IslamicContractId,
    customer_id: CustomerId,
    assessment_year: u32,
    nisab_threshold: Money,
    eligible_wealth: Money,
    zakat_amount: Money,
    payment_status: ZakatPaymentStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ZakatCalculation {
    pub fn new(
        customer_id: CustomerId,
        assessment_year: u32,
        nisab_threshold: Money,
        eligible_wealth: Money,
    ) -> Result<Self, IslamicBankingError> {
        if assessment_year == 0 {
            return Err(IslamicBankingError::InvalidAssessmentYear);
        }

        // Calculate zakat: 2.5% of wealth above Nisab
        let zakat_amount = if eligible_wealth.amount() > nisab_threshold.amount() {
            Money::new(
                eligible_wealth.amount() * ZAKAT_RATE,
                eligible_wealth.currency(),
            )
            .map_err(|_| IslamicBankingError::ZakatCalculationError(
                "Failed to calculate zakat amount".to_string(),
            ))?
        } else {
            Money::zero(eligible_wealth.currency())
        };

        Ok(ZakatCalculation {
            id: IslamicContractId::new(),
            customer_id,
            assessment_year,
            nisab_threshold,
            eligible_wealth,
            zakat_amount,
            payment_status: ZakatPaymentStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        customer_id: CustomerId,
        assessment_year: u32,
        nisab_threshold: Money,
        eligible_wealth: Money,
        zakat_amount: Money,
        payment_status: ZakatPaymentStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        ZakatCalculation {
            id,
            customer_id,
            assessment_year,
            nisab_threshold,
            eligible_wealth,
            zakat_amount,
            payment_status,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }
    pub fn assessment_year(&self) -> u32 {
        self.assessment_year
    }
    pub fn nisab_threshold(&self) -> &Money {
        &self.nisab_threshold
    }
    pub fn eligible_wealth(&self) -> &Money {
        &self.eligible_wealth
    }
    pub fn zakat_amount(&self) -> &Money {
        &self.zakat_amount
    }
    pub fn payment_status(&self) -> ZakatPaymentStatus {
        self.payment_status
    }

    // State transitions
    pub fn mark_paid(&mut self) -> Result<(), IslamicBankingError> {
        if self.payment_status == ZakatPaymentStatus::Paid {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Zakat already paid".to_string(),
            ));
        }
        self.payment_status = ZakatPaymentStatus::Paid;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn defer(&mut self) -> Result<(), IslamicBankingError> {
        if self.payment_status == ZakatPaymentStatus::Paid {
            return Err(IslamicBankingError::InvalidContractStatus(
                "Cannot defer paid zakat".to_string(),
            ));
        }
        self.payment_status = ZakatPaymentStatus::Deferred;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_exempt(&mut self) -> Result<(), IslamicBankingError> {
        self.payment_status = ZakatPaymentStatus::Exempt;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- Sharia Board Decision ---

/// ShariaBoardDecision: Governance approval for Islamic products
/// - Minimum 3 board members (INV requirement)
/// - Ruling: Halal, Haram, Makruh, or Conditional
/// - Conditions documented if ruling is Conditional
/// - Quorum required for decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShariaBoardDecision {
    id: IslamicContractId,
    product_type: ProductType,
    ruling: ShariaRuling,
    conditions: Vec<String>,
    board_members: Vec<String>,
    quorum_met: bool,
    decision_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl ShariaBoardDecision {
    pub fn new(
        product_type: ProductType,
        ruling: ShariaRuling,
        conditions: Vec<String>,
        board_members: Vec<String>,
    ) -> Result<Self, IslamicBankingError> {
        // Enforce minimum 3 board members
        if board_members.len() < SHARIA_BOARD_MIN_MEMBERS {
            return Err(IslamicBankingError::InsufficientBoardMembers(
                format!(
                    "Need {} members, got {}",
                    SHARIA_BOARD_MIN_MEMBERS,
                    board_members.len()
                ),
            ));
        }

        // Validate that conditional ruling has conditions
        if ruling == ShariaRuling::Conditional && conditions.is_empty() {
            return Err(IslamicBankingError::ConditionalRulingWithoutConditions);
        }

        // Quorum = majority of board members (> 50%)
        let quorum_met = board_members.len() >= (SHARIA_BOARD_MIN_MEMBERS + 1) / 2;

        Ok(ShariaBoardDecision {
            id: IslamicContractId::new(),
            product_type,
            ruling,
            conditions,
            board_members,
            quorum_met,
            decision_date: Utc::now(),
            created_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        product_type: ProductType,
        ruling: ShariaRuling,
        conditions: Vec<String>,
        board_members: Vec<String>,
        quorum_met: bool,
        decision_date: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        ShariaBoardDecision {
            id,
            product_type,
            ruling,
            conditions,
            board_members,
            quorum_met,
            decision_date,
            created_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn product_type(&self) -> ProductType {
        self.product_type
    }
    pub fn ruling(&self) -> ShariaRuling {
        self.ruling
    }
    pub fn conditions(&self) -> &[String] {
        &self.conditions
    }
    pub fn board_members(&self) -> &[String] {
        &self.board_members
    }
    pub fn quorum_met(&self) -> bool {
        self.quorum_met
    }
    pub fn decision_date(&self) -> DateTime<Utc> {
        self.decision_date
    }
}

// --- Profit Distribution ---

/// ProfitDistribution: Sharia-compliant profit/loss allocation
/// - Periodic distribution to depositors and partners
/// - Bank share deducted first
/// - Remainder distributed to accounts per profit-sharing contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitDistribution {
    id: IslamicContractId,
    period: u32,
    total_profit: Money,
    depositor_pool_share: Money,
    bank_share: Money,
    per_account_distributions: Vec<(CustomerId, Money)>,
    created_at: DateTime<Utc>,
}

impl ProfitDistribution {
    pub fn new(
        period: u32,
        total_profit: Money,
        depositor_pool_share: Money,
        bank_share: Money,
        per_account_distributions: Vec<(CustomerId, Money)>,
    ) -> Result<Self, IslamicBankingError> {
        if total_profit.is_negative() || total_profit.is_zero() {
            return Err(IslamicBankingError::InvalidProfitAmount(
                "total_profit must be positive".to_string(),
            ));
        }

        // Validate that distributions sum <= total_profit
        let total_distributed = per_account_distributions
            .iter()
            .try_fold(Money::zero(total_profit.currency()), |acc, (_, amount)| {
                acc.add(amount)
            })
            .map_err(|_| IslamicBankingError::DistributionMismatch(
                "Cannot sum distributions".to_string(),
            ))?;

        let sum_check = (depositor_pool_share
            .add(&bank_share)
            .map_err(|_| IslamicBankingError::DistributionMismatch(
                "Cannot sum bank and depositor shares".to_string(),
            ))?
            .add(&total_distributed)
            .map_err(|_| IslamicBankingError::DistributionMismatch(
                "Cannot sum all distributions".to_string(),
            ))?);

        if sum_check.amount_cents() > total_profit.amount_cents() {
            return Err(IslamicBankingError::DistributionMismatch(
                "Distributions exceed total profit".to_string(),
            ));
        }

        Ok(ProfitDistribution {
            id: IslamicContractId::new(),
            period,
            total_profit,
            depositor_pool_share,
            bank_share,
            per_account_distributions,
            created_at: Utc::now(),
        })
    }

    pub fn reconstitute(
        id: IslamicContractId,
        period: u32,
        total_profit: Money,
        depositor_pool_share: Money,
        bank_share: Money,
        per_account_distributions: Vec<(CustomerId, Money)>,
        created_at: DateTime<Utc>,
    ) -> Self {
        ProfitDistribution {
            id,
            period,
            total_profit,
            depositor_pool_share,
            bank_share,
            per_account_distributions,
            created_at,
        }
    }

    // Getters
    pub fn id(&self) -> IslamicContractId {
        self.id
    }
    pub fn period(&self) -> u32 {
        self.period
    }
    pub fn total_profit(&self) -> &Money {
        &self.total_profit
    }
    pub fn depositor_pool_share(&self) -> &Money {
        &self.depositor_pool_share
    }
    pub fn bank_share(&self) -> &Money {
        &self.bank_share
    }
    pub fn per_account_distributions(&self) -> &[(CustomerId, Money)] {
        &self.per_account_distributions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_murabaha_creation() {
        let customer_id = CustomerId::new();
        let cost = Money::new(1000.0, Currency::TND).unwrap();
        let contract = MurabahaContract::new(
            customer_id,
            cost,
            0.15, // 15% profit margin
            12,
            "Car Purchase".to_string(),
            Utc::now() + Duration::days(30),
        );
        assert!(contract.is_ok());
        let c = contract.unwrap();
        assert_eq!(c.profit_margin(), 0.15);
        assert_eq!(c.installments(), 12);
    }

    #[test]
    fn test_murabaha_invalid_margin() {
        let customer_id = CustomerId::new();
        let cost = Money::new(1000.0, Currency::TND).unwrap();
        let contract = MurabahaContract::new(
            customer_id,
            cost,
            1.5, // Invalid: > 100%
            12,
            "Car".to_string(),
            Utc::now() + Duration::days(30),
        );
        assert!(contract.is_err());
    }

    #[test]
    fn test_ijara_creation() {
        let customer_id = CustomerId::new();
        let rental = Money::new(500.0, Currency::TND).unwrap();
        let purchase = Money::new(10000.0, Currency::TND).unwrap();
        let now = Utc::now();
        let contract = IjaraContract::new(
            customer_id,
            "ASSET-001".to_string(),
            rental,
            now,
            now + Duration::days(365),
            purchase,
            MaintenanceResponsibility::Lessor,
        );
        assert!(contract.is_ok());
    }

    #[test]
    fn test_ijara_invalid_lease_period() {
        let customer_id = CustomerId::new();
        let rental = Money::new(500.0, Currency::TND).unwrap();
        let purchase = Money::new(10000.0, Currency::TND).unwrap();
        let now = Utc::now();
        let contract = IjaraContract::new(
            customer_id,
            "ASSET-001".to_string(),
            rental,
            now + Duration::days(365),
            now, // End before start
            purchase,
            MaintenanceResponsibility::Lessor,
        );
        assert!(contract.is_err());
    }

    #[test]
    fn test_musharaka_creation() {
        let customer_id = CustomerId::new();
        let capital = Money::new(10000.0, Currency::TND).unwrap();
        let schedule = vec![(1, 60.0), (2, 40.0), (3, 20.0), (4, 0.0)];
        let contract = MusharakaContract::new(
            customer_id,
            capital,
            60.0, // Bank 60%, Client 40%
            40.0,
            0.7, // Bank gets 70% of profits
            schedule,
        );
        assert!(contract.is_ok());
    }

    #[test]
    fn test_musharaka_shares_dont_sum() {
        let customer_id = CustomerId::new();
        let capital = Money::new(10000.0, Currency::TND).unwrap();
        let schedule = vec![(1, 50.0)];
        let contract = MusharakaContract::new(
            customer_id,
            capital,
            60.0, // Bank 60%, Client 30% (only 90%)
            30.0,
            0.7,
            schedule,
        );
        assert!(contract.is_err());
    }

    #[test]
    fn test_mudaraba_creation() {
        let customer_id = CustomerId::new();
        let capital = Money::new(5000.0, Currency::TND).unwrap();
        let contract = MudarabaContract::new(
            customer_id,
            capital,
            0.5, // 50% profit split
            "Real Estate Investment".to_string(),
            30, // 30-day reporting period
        );
        assert!(contract.is_ok());
    }

    #[test]
    fn test_sukuk_creation() {
        let denom = Money::new(1000.0, Currency::TND).unwrap();
        let total = Money::new(1000000.0, Currency::TND).unwrap();
        let contract = SukukIssuance::new(
            denom,
            total,
            1000, // 1000 units
            0.05, // 5% coupon
            Utc::now() + Duration::days(365 * 5),
            "Commercial Real Estate Portfolio".to_string(),
        );
        assert!(contract.is_ok());
    }

    #[test]
    fn test_sukuk_invalid_coupon() {
        let denom = Money::new(1000.0, Currency::TND).unwrap();
        let total = Money::new(1000000.0, Currency::TND).unwrap();
        let contract = SukukIssuance::new(
            denom,
            total,
            1000,
            1.5, // Invalid: > 100%
            Utc::now() + Duration::days(365),
            "Asset".to_string(),
        );
        assert!(contract.is_err());
    }

    #[test]
    fn test_zakat_calculation() {
        let customer_id = CustomerId::new();
        let nisab = Money::new(1000.0, Currency::TND).unwrap();
        let wealth = Money::new(5000.0, Currency::TND).unwrap();
        let zakat = ZakatCalculation::new(customer_id, 2026, nisab, wealth);
        assert!(zakat.is_ok());
        let z = zakat.unwrap();
        assert_eq!(z.zakat_amount().amount(), 5000.0 * ZAKAT_RATE);
    }

    #[test]
    fn test_zakat_below_nisab() {
        let customer_id = CustomerId::new();
        let nisab = Money::new(10000.0, Currency::TND).unwrap();
        let wealth = Money::new(5000.0, Currency::TND).unwrap();
        let zakat = ZakatCalculation::new(customer_id, 2026, nisab, wealth);
        assert!(zakat.is_ok());
        let z = zakat.unwrap();
        assert!(z.zakat_amount().is_zero());
    }

    #[test]
    fn test_sharia_board_decision() {
        let members = vec!["Dr. Ahmed".to_string(), "Dr. Fatima".to_string(), "Dr. Ali".to_string()];
        let decision = ShariaBoardDecision::new(
            ProductType::Murabaha,
            ShariaRuling::Halal,
            vec![],
            members,
        );
        assert!(decision.is_ok());
        let d = decision.unwrap();
        assert!(d.quorum_met());
    }

    #[test]
    fn test_sharia_board_insufficient_members() {
        let members = vec!["Dr. Ahmed".to_string(), "Dr. Fatima".to_string()];
        let decision = ShariaBoardDecision::new(
            ProductType::Murabaha,
            ShariaRuling::Halal,
            vec![],
            members,
        );
        assert!(decision.is_err());
    }

    #[test]
    fn test_sharia_board_conditional_without_conditions() {
        let members = vec![
            "Dr. Ahmed".to_string(),
            "Dr. Fatima".to_string(),
            "Dr. Ali".to_string(),
        ];
        let decision = ShariaBoardDecision::new(
            ProductType::Mudaraba,
            ShariaRuling::Conditional,
            vec![], // Missing conditions
            members,
        );
        assert!(decision.is_err());
    }

    #[test]
    fn test_profit_distribution() {
        let total_profit = Money::new(1000.0, Currency::TND).unwrap();
        let bank_share = Money::new(300.0, Currency::TND).unwrap();
        let depositor_share = Money::new(700.0, Currency::TND).unwrap();
        let distributions = vec![(CustomerId::new(), Money::new(100.0, Currency::TND).unwrap())];

        let dist = ProfitDistribution::new(
            1,
            total_profit,
            depositor_share,
            bank_share,
            distributions,
        );
        assert!(dist.is_ok());
    }

    #[test]
    fn test_murabaha_status_transitions() {
        let customer_id = CustomerId::new();
        let cost = Money::new(1000.0, Currency::TND).unwrap();
        let mut contract = MurabahaContract::new(
            customer_id,
            cost,
            0.15,
            12,
            "Car".to_string(),
            Utc::now() + Duration::days(30),
        )
        .unwrap();

        assert_eq!(contract.status(), MurabahaStatus::Proposed);
        contract.approve().unwrap();
        assert_eq!(contract.status(), MurabahaStatus::Approved);
        contract.activate().unwrap();
        assert_eq!(contract.status(), MurabahaStatus::Active);
        contract.complete().unwrap();
        assert_eq!(contract.status(), MurabahaStatus::Completed);
    }

    #[test]
    fn test_ijara_status_transitions() {
        let customer_id = CustomerId::new();
        let rental = Money::new(500.0, Currency::TND).unwrap();
        let purchase = Money::new(10000.0, Currency::TND).unwrap();
        let now = Utc::now();
        let mut contract = IjaraContract::new(
            customer_id,
            "ASSET-001".to_string(),
            rental,
            now,
            now + Duration::days(365),
            purchase,
            MaintenanceResponsibility::Lessor,
        )
        .unwrap();

        assert_eq!(contract.status(), IjaraStatus::Proposed);
        contract.approve().unwrap();
        assert_eq!(contract.status(), IjaraStatus::Approved);
    }

    #[test]
    fn test_sukuk_status_transitions() {
        let denom = Money::new(1000.0, Currency::TND).unwrap();
        let total = Money::new(1000000.0, Currency::TND).unwrap();
        let mut sukuk = SukukIssuance::new(
            denom,
            total,
            1000,
            0.05,
            Utc::now() + Duration::days(365 * 5),
            "Real Estate".to_string(),
        )
        .unwrap();

        assert_eq!(sukuk.status(), SukukStatus::Proposed);
        sukuk.approve().unwrap();
        assert_eq!(sukuk.status(), SukukStatus::Approved);
        sukuk.issue().unwrap();
        assert_eq!(sukuk.status(), SukukStatus::Outstanding);
    }

    #[test]
    fn test_zakat_status_transitions() {
        let customer_id = CustomerId::new();
        let nisab = Money::new(1000.0, Currency::TND).unwrap();
        let wealth = Money::new(5000.0, Currency::TND).unwrap();
        let mut zakat = ZakatCalculation::new(customer_id, 2026, nisab, wealth).unwrap();

        assert_eq!(zakat.payment_status(), ZakatPaymentStatus::Pending);
        zakat.mark_paid().unwrap();
        assert_eq!(zakat.payment_status(), ZakatPaymentStatus::Paid);
    }

    #[test]
    fn test_currency_consistency() {
        let customer_id = CustomerId::new();
        let cost_tnd = Money::new(1000.0, Currency::TND).unwrap();
        let contract = MurabahaContract::new(
            customer_id,
            cost_tnd,
            0.1,
            12,
            "Asset".to_string(),
            Utc::now() + Duration::days(30),
        );
        assert!(contract.is_ok());
        let c = contract.unwrap();
        assert_eq!(c.selling_price().currency(), Currency::TND);
    }
}
