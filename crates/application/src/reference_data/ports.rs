use async_trait::async_trait;
use chrono::{DateTime, Utc};

use banko_domain::reference_data::{
    BankCode, BranchCode, CountryCode, CurrencyReference, FeeScheduleReference, HolidayCalendar,
    ReferenceDataId, RegulatoryCode, SystemParameter,
};

/// Port for reference data persistence -- implemented by infrastructure layer.
#[async_trait]
pub trait IReferenceDataRepository: Send + Sync {
    // --- Country Code Methods ---
    async fn save_country(&self, country: &CountryCode) -> Result<(), String>;
    async fn find_country_by_id(&self, id: &ReferenceDataId) -> Result<Option<CountryCode>, String>;
    async fn find_country_by_iso_alpha2(&self, code: &str) -> Result<Option<CountryCode>, String>;
    async fn find_country_by_iso_alpha3(&self, code: &str) -> Result<Option<CountryCode>, String>;
    async fn list_countries(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CountryCode>, String>;
    async fn list_active_countries(&self) -> Result<Vec<CountryCode>, String>;

    // --- Currency Reference Methods ---
    async fn save_currency(&self, currency: &CurrencyReference) -> Result<(), String>;
    async fn find_currency_by_id(&self, id: &ReferenceDataId) -> Result<Option<CurrencyReference>, String>;
    async fn find_currency_by_code(&self, code: &str) -> Result<Option<CurrencyReference>, String>;
    async fn list_currencies(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CurrencyReference>, String>;
    async fn list_active_currencies(&self) -> Result<Vec<CurrencyReference>, String>;

    // --- Bank Code Methods ---
    async fn save_bank_code(&self, bank: &BankCode) -> Result<(), String>;
    async fn find_bank_code_by_id(&self, id: &ReferenceDataId) -> Result<Option<BankCode>, String>;
    async fn find_bank_code_by_bic(&self, bic: &str) -> Result<Option<BankCode>, String>;
    async fn list_bank_codes(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BankCode>, String>;
    async fn list_active_bank_codes(&self) -> Result<Vec<BankCode>, String>;

    // --- Branch Code Methods ---
    async fn save_branch_code(&self, branch: &BranchCode) -> Result<(), String>;
    async fn find_branch_code_by_id(&self, id: &ReferenceDataId) -> Result<Option<BranchCode>, String>;
    async fn find_branch_code_by_code(&self, code: &str) -> Result<Option<BranchCode>, String>;
    async fn find_branches_by_bank_bic(&self, bic: &str) -> Result<Vec<BranchCode>, String>;
    async fn list_branch_codes(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BranchCode>, String>;
    async fn list_active_branch_codes(&self) -> Result<Vec<BranchCode>, String>;

    // --- Holiday Calendar Methods ---
    async fn save_holiday(&self, holiday: &HolidayCalendar) -> Result<(), String>;
    async fn find_holiday_by_id(&self, id: &ReferenceDataId) -> Result<Option<HolidayCalendar>, String>;
    async fn find_holidays_by_date(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<HolidayCalendar>, String>;
    async fn find_banking_holidays_by_date(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<HolidayCalendar>, String>;
    async fn is_banking_holiday(&self, date: DateTime<Utc>) -> Result<bool, String>;

    // --- System Parameter Methods ---
    async fn save_system_parameter(&self, param: &SystemParameter) -> Result<(), String>;
    async fn find_system_parameter_by_id(&self, id: &ReferenceDataId) -> Result<Option<SystemParameter>, String>;
    async fn find_system_parameter_by_key(&self, key: &str) -> Result<Option<SystemParameter>, String>;
    async fn find_system_parameters_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<SystemParameter>, String>;
    async fn list_active_system_parameters(&self) -> Result<Vec<SystemParameter>, String>;

    // --- Regulatory Code Methods ---
    async fn save_regulatory_code(&self, code: &RegulatoryCode) -> Result<(), String>;
    async fn find_regulatory_code_by_id(&self, id: &ReferenceDataId) -> Result<Option<RegulatoryCode>, String>;
    async fn find_regulatory_code_by_code(&self, code: &str) -> Result<Option<RegulatoryCode>, String>;
    async fn list_regulatory_codes(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RegulatoryCode>, String>;
    async fn list_active_regulatory_codes(&self) -> Result<Vec<RegulatoryCode>, String>;

    // --- Fee Schedule Methods ---
    async fn save_fee_schedule(&self, fee: &FeeScheduleReference) -> Result<(), String>;
    async fn find_fee_schedule_by_id(&self, id: &ReferenceDataId) -> Result<Option<FeeScheduleReference>, String>;
    async fn find_fee_schedules_by_type(
        &self,
        fee_type: &str,
    ) -> Result<Vec<FeeScheduleReference>, String>;
    async fn list_fee_schedules(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FeeScheduleReference>, String>;
    async fn list_active_fee_schedules(&self) -> Result<Vec<FeeScheduleReference>, String>;
}
