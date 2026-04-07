use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;

use banko_domain::islamic_banking::{
    IslamicContractId, IjaraContract, MudarabaContract, MurabahaContract, MusharakaContract,
    ProfitDistribution, ShariaBoardDecision, SukukIssuance, ZakatCalculation,
};
use banko_domain::shared::CustomerId;

/// Port for persisting and retrieving Islamic banking contracts
#[async_trait]
pub trait IIslamicBankingRepository: Send + Sync {
    // --- Murabaha ---
    async fn save_murabaha(&self, contract: &MurabahaContract) -> Result<(), String>;
    async fn find_murabaha(&self, id: IslamicContractId) -> Result<Option<MurabahaContract>, String>;
    async fn find_murabaha_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MurabahaContract>, String>;

    // --- Ijara ---
    async fn save_ijara(&self, contract: &IjaraContract) -> Result<(), String>;
    async fn find_ijara(&self, id: IslamicContractId) -> Result<Option<IjaraContract>, String>;
    async fn find_ijara_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<IjaraContract>, String>;

    // --- Musharaka ---
    async fn save_musharaka(&self, contract: &MusharakaContract) -> Result<(), String>;
    async fn find_musharaka(&self, id: IslamicContractId)
        -> Result<Option<MusharakaContract>, String>;
    async fn find_musharaka_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MusharakaContract>, String>;

    // --- Mudaraba ---
    async fn save_mudaraba(&self, contract: &MudarabaContract) -> Result<(), String>;
    async fn find_mudaraba(&self, id: IslamicContractId)
        -> Result<Option<MudarabaContract>, String>;
    async fn find_mudaraba_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<MudarabaContract>, String>;

    // --- Sukuk ---
    async fn save_sukuk(&self, sukuk: &SukukIssuance) -> Result<(), String>;
    async fn find_sukuk(&self, id: IslamicContractId) -> Result<Option<SukukIssuance>, String>;
    async fn find_sukuk_outstanding(&self) -> Result<Vec<SukukIssuance>, String>;

    // --- Zakat ---
    async fn save_zakat(&self, zakat: &ZakatCalculation) -> Result<(), String>;
    async fn find_zakat(&self, id: IslamicContractId) -> Result<Option<ZakatCalculation>, String>;
    async fn find_zakat_by_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<ZakatCalculation>, String>;
    async fn find_zakat_by_year(
        &self,
        customer_id: &CustomerId,
        year: u32,
    ) -> Result<Option<ZakatCalculation>, String>;

    // --- Sharia Board ---
    async fn save_sharia_decision(&self, decision: &ShariaBoardDecision) -> Result<(), String>;
    async fn find_sharia_decision(
        &self,
        id: IslamicContractId,
    ) -> Result<Option<ShariaBoardDecision>, String>;
    async fn find_sharia_decisions_by_product(
        &self,
        product_type: &str,
    ) -> Result<Vec<ShariaBoardDecision>, String>;

    // --- Profit Distribution ---
    async fn save_profit_distribution(
        &self,
        distribution: &ProfitDistribution,
    ) -> Result<(), String>;
    async fn find_profit_distribution(
        &self,
        id: IslamicContractId,
    ) -> Result<Option<ProfitDistribution>, String>;
    async fn find_profit_distributions_by_period(
        &self,
        period: u32,
    ) -> Result<Vec<ProfitDistribution>, String>;
}

/// Port for verifying Sharia board approval
#[async_trait]
pub trait IShariaApprovalVerifier: Send + Sync {
    async fn is_product_approved(
        &self,
        product_type: &str,
    ) -> Result<bool, String>;

    async fn get_latest_ruling(
        &self,
        product_type: &str,
    ) -> Result<Option<ShariaBoardDecision>, String>;
}

/// Port for validating Islamic products against conditions
#[async_trait]
pub trait IIslamicProductValidator: Send + Sync {
    async fn validate_murabaha(&self, contract: &MurabahaContract) -> Result<bool, String>;
    async fn validate_ijara(&self, contract: &IjaraContract) -> Result<bool, String>;
    async fn validate_musharaka(&self, contract: &MusharakaContract) -> Result<bool, String>;
    async fn validate_mudaraba(&self, contract: &MudarabaContract) -> Result<bool, String>;
    async fn validate_sukuk(&self, sukuk: &SukukIssuance) -> Result<bool, String>;
}

/// Port for asset verification (assets must exist for Ijara, Sukuk)
#[async_trait]
pub trait IAssetVerifier: Send + Sync {
    async fn asset_exists(&self, asset_id: &str) -> Result<bool, String>;
    async fn asset_is_tangible(&self, asset_id: &str) -> Result<bool, String>;
}

/// Port for profit calculations
#[async_trait]
pub trait IProfitCalculator: Send + Sync {
    /// Calculate Murabaha installment amount
    async fn calculate_murabaha_installment(
        &self,
        selling_price: f64,
        installments: u32,
    ) -> Result<f64, String>;

    /// Calculate Ijara rental distribution
    async fn calculate_ijara_distribution(
        &self,
        monthly_rental: f64,
        months: u32,
    ) -> Result<f64, String>;

    /// Calculate Musharaka profit split
    async fn calculate_musharaka_profit(
        &self,
        total_profit: f64,
        profit_sharing_ratio: f64,
    ) -> Result<(f64, f64), String>; // (bank_share, client_share)

    /// Calculate Mudaraba profit split
    async fn calculate_mudaraba_profit(
        &self,
        total_profit: f64,
        profit_sharing_ratio: f64,
    ) -> Result<(f64, f64), String>; // (bank_share, client_share)

    /// Calculate Sukuk coupon payment
    async fn calculate_sukuk_coupon(
        &self,
        total_amount: f64,
        coupon_rate: f64,
    ) -> Result<f64, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_trait_bounds() {
        // This test ensures all ports satisfy Send + Sync
 