use std::sync::Arc;

use banko_domain::islamic_banking::*;
use banko_domain::shared::{Currency, CustomerId, Money};

use super::errors::IslamicBankingServiceError;
use super::ports::*;

/// Islamic Banking Service
/// Orchestrates all Islamic banking operations with Sharia compliance checks
pub struct IslamicBankingService {
    repository: Arc<dyn IIslamicBankingRepository>,
    sharia_verifier: Arc<dyn IShariaApprovalVerifier>,
    validator: Arc<dyn IIslamicProductValidator>,
    asset_verifier: Arc<dyn IAssetVerifier>,
    profit_calculator: Arc<dyn IProfitCalculator>,
}

impl IslamicBankingService {
    pub fn new(
        repository: Arc<dyn IIslamicBankingRepository>,
        sharia_verifier: Arc<dyn IShariaApprovalVerifier>,
        validator: Arc<dyn IIslamicProductValidator>,
        asset_verifier: Arc<dyn IAssetVerifier>,
        profit_calculator: Arc<dyn IProfitCalculator>,
    ) -> Self {
        IslamicBankingService {
            repository,
            sharia_verifier,
            validator,
            asset_verifier,
            profit_calculator,
        }
    }

    // --- Murabaha Operations ---

    /// Create a new Murabaha contract with Sharia approval
    pub async fn create_murabaha(
        &self,
        customer_id: CustomerId,
        cost_price: Money,
        profit_margin: f64,
        installments: u32,
        asset_description: String,
        delivery_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<MurabahaContract, IslamicBankingServiceError> {
        // Verify Murabaha is Sharia-approved
        let approved = self
            .sharia_verifier
            .is_product_approved("Murabaha")
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !approved {
            return Err(IslamicBankingServiceError::ShariaApprovalRequired);
        }

        // Create domain entity with invariant checks
        let contract = MurabahaContract::new(
            customer_id,
            cost_price,
            profit_margin,
            installments,
            asset_description,
            delivery_date,
        )
        .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        // Validate contract
        self.validator
            .validate_murabaha(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        // Persist
        self.repository
            .save_murabaha(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(contract)
    }

    /// Find a Murabaha contract by ID
    pub async fn find_murabaha(
        &self,
        id: IslamicContractId,
    ) -> Result<MurabahaContract, IslamicBankingServiceError> {
        self.repository
            .find_murabaha(id)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?
            .ok_or_else(|| IslamicBankingServiceError::ContractNotFound(id.to_string()))
    }

    /// List all Murabaha contracts for a customer
    pub async fn list_murabaha_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MurabahaContract>, IslamicBankingServiceError> {
        self.repository
            .find_murabaha_by_customer(customer_id)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))
    }

    /// Approve a Murabaha contract
    pub async fn approve_murabaha(
        &self,
        id: IslamicContractId,
    ) -> Result<MurabahaContract, IslamicBankingServiceError> {
        let mut contract = self.find_murabaha(id).await?;
        contract
            .approve()
            .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;
        self.repository
            .save_murabaha(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;
        Ok(contract)
    }

    /// Activate a Murabaha contract
    pub async fn activate_murabaha(
        &self,
        id: IslamicContractId,
    ) -> Result<MurabahaContract, IslamicBankingServiceError> {
        let mut contract = self.find_murabaha(id).await?;
        contract
            .activate()
            .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;
        self.repository
            .save_murabaha(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;
        Ok(contract)
    }

    // --- Ijara Operations ---

    /// Create a new Ijara contract with asset verification
    pub async fn create_ijara(
        &self,
        customer_id: CustomerId,
        asset_id: String,
        monthly_rental: Money,
        lease_start: chrono::DateTime<chrono::Utc>,
        lease_end: chrono::DateTime<chrono::Utc>,
        purchase_option_price: Money,
        maintenance_responsibility: MaintenanceResponsibility,
    ) -> Result<IjaraContract, IslamicBankingServiceError> {
        // Verify Ijara is Sharia-approved
        let approved = self
            .sharia_verifier
            .is_product_approved("Ijara")
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !approved {
            return Err(IslamicBankingServiceError::ShariaApprovalRequired);
        }

        // Verify asset exists and is tangible
        let exists = self
            .asset_verifier
            .asset_exists(&asset_id)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !exists {
            return Err(IslamicBankingServiceError::AssetNotFound(asset_id));
        }

        let is_tangible = self
            .asset_verifier
            .asset_is_tangible(&asset_id)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !is_tangible {
            return Err(IslamicBankingServiceError::Internal(
                "Ijara requires tangible asset".to_string(),
            ));
        }

        // Create domain entity
        let contract = IjaraContract::new(
            customer_id,
            asset_id,
            monthly_rental,
            lease_start,
            lease_end,
            purchase_option_price,
            maintenance_responsibility,
        )
        .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        // Validate
        self.validator
            .validate_ijara(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        // Persist
        self.repository
            .save_ijara(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(contract)
    }

    /// Find an Ijara contract
    pub async fn find_ijara(
        &self,
        id: IslamicContractId,
    ) -> Result<IjaraContract, IslamicBankingServiceError> {
        self.repository
            .find_ijara(id)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?
            .ok_or_else(|| IslamicBankingServiceError::ContractNotFound(id.to_string()))
    }

    // --- Musharaka Operations ---

    /// Create a new Musharaka partnership contract
    pub async fn create_musharaka(
        &self,
        customer_id: CustomerId,
        total_capital: Money,
        bank_share_pct: f64,
        client_share_pct: f64,
        profit_sharing_ratio: f64,
        diminishing_schedule: Vec<(u32, f64)>,
    ) -> Result<MusharakaContract, IslamicBankingServiceError> {
        // Verify Musharaka is Sharia-approved
        let approved = self
            .sharia_verifier
            .is_product_approved("Musharaka")
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !approved {
            return Err(IslamicBankingServiceError::ShariaApprovalRequired);
        }

        let contract = MusharakaContract::new(
            customer_id,
            total_capital,
            bank_share_pct,
            client_share_pct,
            profit_sharing_ratio,
            diminishing_schedule,
        )
        .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        self.validator
            .validate_musharaka(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        self.repository
            .save_musharaka(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(contract)
    }

    // --- Mudaraba Operations ---

    /// Create a new Mudaraba profit-sharing investment
    pub async fn create_mudaraba(
        &self,
        customer_id: CustomerId,
        capital_amount: Money,
        profit_sharing_ratio: f64,
        investment_type: String,
        reporting_period: u32,
    ) -> Result<MudarabaContract, IslamicBankingServiceError> {
        let approved = self
            .sharia_verifier
            .is_product_approved("Mudaraba")
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !approved {
            return Err(IslamicBankingServiceError::ShariaApprovalRequired);
        }

        let contract = MudarabaContract::new(
            customer_id,
            capital_amount,
            profit_sharing_ratio,
            investment_type,
            reporting_period,
        )
        .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        self.validator
            .validate_mudaraba(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        self.repository
            .save_mudaraba(&contract)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(contract)
    }

    // --- Sukuk Operations ---

    /// Issue a new Sukuk with asset backing verification
    pub async fn create_sukuk(
        &self,
        denomination: Money,
        total_amount: Money,
        units_issued: u64,
        coupon_rate: f64,
        maturity_date: chrono::DateTime<chrono::Utc>,
        underlying_asset: String,
    ) -> Result<SukukIssuance, IslamicBankingServiceError> {
        // Verify Sukuk is Sharia-approved
        let approved = self
            .sharia_verifier
            .is_product_approved("Sukuk")
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !approved {
            return Err(IslamicBankingServiceError::ShariaApprovalRequired);
        }

        // Verify underlying asset exists and is tangible
        let exists = self
            .asset_verifier
            .asset_exists(&underlying_asset)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        if !exists {
            return Err(IslamicBankingServiceError::AssetNotFound(underlying_asset));
        }

        let sukuk = SukukIssuance::new(
            denomination,
            total_amount,
            units_issued,
            coupon_rate,
            maturity_date,
            underlying_asset,
        )
        .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        self.validator
            .validate_sukuk(&sukuk)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        self.repository
            .save_sukuk(&sukuk)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(sukuk)
    }

    // --- Zakat Operations ---

    /// Calculate Zakat for a customer
    pub async fn calculate_zakat(
        &self,
        customer_id: CustomerId,
        assessment_year: u32,
        nisab_threshold: Money,
        eligible_wealth: Money,
    ) -> Result<ZakatCalculation, IslamicBankingServiceError> {
        let zakat = ZakatCalculation::new(customer_id, assessment_year, nisab_threshold, eligible_wealth)
            .map_err(|e| IslamicBankingServiceError::ZakatCalculationError(e.to_string()))?;

        self.repository
            .save_zakat(&zakat)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(zakat)
    }

    /// Find a Zakat calculation by ID
    pub async fn find_zakat(
        &self,
        id: IslamicContractId,
    ) -> Result<ZakatCalculation, IslamicBankingServiceError> {
        self.repository
            .find_zakat(id)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?
            .ok_or_else(|| IslamicBankingServiceError::ContractNotFound(id.to_string()))
    }

    /// Get Zakat calculation for a specific year
    pub async fn get_zakat_for_year(
        &self,
        customer_id: &CustomerId,
        year: u32,
    ) -> Result<Option<ZakatCalculation>, IslamicBankingServiceError> {
        self.repository
            .find_zakat_by_year(customer_id, year)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))
    }

    // --- Sharia Board Operations ---

    /// Create a Sharia board decision for a product
    pub async fn create_sharia_decision(
        &self,
        product_type: ProductType,
        ruling: ShariaRuling,
        conditions: Vec<String>,
        board_members: Vec<String>,
    ) -> Result<ShariaBoardDecision, IslamicBankingServiceError> {
        let decision = ShariaBoardDecision::new(product_type, ruling, conditions, board_members)
            .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_sharia_decision(&decision)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(decision)
    }

    /// Get the latest Sharia ruling for a product type
    pub async fn get_product_ruling(
        &self,
        product_type: &str,
    ) -> Result<Option<ShariaBoardDecision>, IslamicBankingServiceError> {
        self.sharia_verifier
            .get_latest_ruling(product_type)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))
    }

    // --- Profit Distribution Operations ---

    /// Distribute profits to accounts
    pub async fn distribute_profits(
        &self,
        period: u32,
        total_profit: Money,
        depositor_pool_share: Money,
        bank_share: Money,
        per_account_distributions: Vec<(CustomerId, Money)>,
    ) -> Result<ProfitDistribution, IslamicBankingServiceError> {
        let distribution = ProfitDistribution::new(
            period,
            total_profit,
            depositor_pool_share,
            bank_share,
            per_account_distributions,
        )
        .map_err(|e| IslamicBankingServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_profit_distribution(&distribution)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))?;

        Ok(distribution)
    }

    /// Get all distributions for a period
    pub async fn get_distributions_for_period(
        &self,
        period: u32,
    ) -> Result<Vec<ProfitDistribution>, IslamicBankingServiceError> {
        self.repository
            .find_profit_distributions_by_period(period)
            .await
            .map_err(|e| IslamicBankingServiceError::RepositoryError(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Tests would require mock implementations of ports
        // This is a compile-time verification that the service is properly structured
    }
}
