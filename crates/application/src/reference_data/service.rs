use std::sync::Arc;

use chrono::{DateTime, Utc};

use banko_domain::reference_data::{
    BankCode, BicCodeVo, BranchCode, CountryCode, CountryCodeVo, CurrencyCodeVo,
    CurrencyReference, FeeScheduleReference, FeeType, HolidayCalendar, HolidayType,
    ReferenceDataId, RegulatoryClassification, RegulatoryCode, SystemParameter,
    SystemParameterType,
};

use super::dto::*;
use super::errors::ReferenceDataServiceError;
use super::ports::IReferenceDataRepository;

pub struct ReferenceDataService {
    repository: Arc<dyn IReferenceDataRepository>,
}

impl ReferenceDataService {
    pub fn new(repository: Arc<dyn IReferenceDataRepository>) -> Self {
        ReferenceDataService { repository }
    }

    // --- Country Code Services ---

    pub async fn create_country(
        &self,
        req: CreateCountryCodeRequest,
    ) -> Result<CountryCodeResponse, ReferenceDataServiceError> {
        let code = CountryCodeVo::new(&req.iso_alpha2, &req.iso_alpha3, &req.iso_numeric)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let country = CountryCode::new(
            code,
            &req.name_en,
            &req.name_fr,
            &req.name_ar,
            req.is_sanctioned,
            req.effective_from,
            req.effective_to,
        )
        .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_country(&country)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.country_to_response(&country))
    }

    pub async fn get_country_by_id(
        &self,
        id: &str,
    ) -> Result<CountryCodeResponse, ReferenceDataServiceError> {
        let ref_id = ReferenceDataId::parse(id)
            .map_err(|e| ReferenceDataServiceError::InvalidInput(e.to_string()))?;

        let country = self
            .repository
            .find_country_by_id(&ref_id)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::CountryCodeNotFound(id.to_string()))?;

        Ok(self.country_to_response(&country))
    }

    pub async fn get_country_by_iso_alpha2(
        &self,
        code: &str,
    ) -> Result<CountryCodeResponse, ReferenceDataServiceError> {
        let country = self
            .repository
            .find_country_by_iso_alpha2(code)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::CountryCodeNotFound(code.to_string()))?;

        Ok(self.country_to_response(&country))
    }

    pub async fn get_country_by_iso_alpha3(
        &self,
        code: &str,
    ) -> Result<CountryCodeResponse, ReferenceDataServiceError> {
        let country = self
            .repository
            .find_country_by_iso_alpha3(code)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::CountryCodeNotFound(code.to_string()))?;

        Ok(self.country_to_response(&country))
    }

    pub async fn list_countries(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CountryCodeResponse>, ReferenceDataServiceError> {
        let countries = self
            .repository
            .list_countries(limit, offset)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(countries.iter().map(|c| self.country_to_response(c)).collect())
    }

    pub async fn list_active_countries(
        &self,
    ) -> Result<Vec<CountryCodeResponse>, ReferenceDataServiceError> {
        let countries = self
            .repository
            .list_active_countries()
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(countries.iter().map(|c| self.country_to_response(c)).collect())
    }

    fn country_to_response(&self, country: &CountryCode) -> CountryCodeResponse {
        CountryCodeResponse {
            id: country.id().to_string(),
            iso_alpha2: country.code().iso_alpha2().to_string(),
            iso_alpha3: country.code().iso_alpha3().to_string(),
            iso_numeric: country.code().iso_numeric().to_string(),
            name_en: country.name_en().to_string(),
            name_fr: country.name_fr().to_string(),
            name_ar: country.name_ar().to_string(),
            is_sanctioned: country.is_sanctioned(),
            is_active: country.is_active(),
            effective_from: country.effective_from(),
            effective_to: country.effective_to(),
            created_at: country.created_at(),
            updated_at: country.updated_at(),
        }
    }

    // --- Currency Reference Services ---

    pub async fn create_currency(
        &self,
        req: CreateCurrencyReferenceRequest,
    ) -> Result<CurrencyReferenceResponse, ReferenceDataServiceError> {
        let code = CurrencyCodeVo::new(&req.code)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let currency = CurrencyReference::new(
            code,
            &req.name_en,
            &req.name_fr,
            req.decimal_places,
            req.is_active,
            req.effective_from,
            req.effective_to,
        )
        .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_currency(&currency)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.currency_to_response(&currency))
    }

    pub async fn get_currency_by_code(
        &self,
        code: &str,
    ) -> Result<CurrencyReferenceResponse, ReferenceDataServiceError> {
        let currency = self
            .repository
            .find_currency_by_code(code)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::CurrencyNotFound(code.to_string()))?;

        Ok(self.currency_to_response(&currency))
    }

    pub async fn list_active_currencies(
        &self,
    ) -> Result<Vec<CurrencyReferenceResponse>, ReferenceDataServiceError> {
        let currencies = self
            .repository
            .list_active_currencies()
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(currencies.iter().map(|c| self.currency_to_response(c)).collect())
    }

    fn currency_to_response(&self, currency: &CurrencyReference) -> CurrencyReferenceResponse {
        CurrencyReferenceResponse {
            id: currency.id().to_string(),
            code: currency.code().to_string(),
            name_en: currency.name_en().to_string(),
            name_fr: currency.name_fr().to_string(),
            decimal_places: currency.decimal_places(),
            is_active: currency.is_active(),
            effective_from: currency.created_at(), // Placeholder - would need effective_from from entity
            effective_to: None,
            created_at: currency.created_at(),
            updated_at: currency.updated_at(),
        }
    }

    // --- Bank Code Services ---

    pub async fn create_bank_code(
        &self,
        req: CreateBankCodeRequest,
    ) -> Result<BankCodeResponse, ReferenceDataServiceError> {
        let bic = BicCodeVo::new(&req.bic)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let country = self
            .repository
            .find_country_by_iso_alpha2(&req.country_iso_alpha2)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::CountryCodeNotFound(
                req.country_iso_alpha2.clone(),
            ))?;

        let bank =
            BankCode::new(bic, &req.bank_name, country.code().clone(), req.is_active)
                .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_bank_code(&bank)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.bank_code_to_response(&bank))
    }

    pub async fn get_bank_code_by_bic(
        &self,
        bic: &str,
    ) -> Result<BankCodeResponse, ReferenceDataServiceError> {
        let bank = self
            .repository
            .find_bank_code_by_bic(bic)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::BankCodeNotFound(bic.to_string()))?;

        Ok(self.bank_code_to_response(&bank))
    }

    pub async fn list_active_bank_codes(
        &self,
    ) -> Result<Vec<BankCodeResponse>, ReferenceDataServiceError> {
        let banks = self
            .repository
            .list_active_bank_codes()
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(banks.iter().map(|b| self.bank_code_to_response(b)).collect())
    }

    fn bank_code_to_response(&self, bank: &BankCode) -> BankCodeResponse {
        BankCodeResponse {
            id: bank.id().to_string(),
            bic: bank.bic().to_string(),
            bank_name: bank.bank_name().to_string(),
            country_iso_alpha2: bank.country_code().iso_alpha2().to_string(),
            is_active: bank.is_active(),
            created_at: bank.created_at(),
            updated_at: bank.updated_at(),
        }
    }

    // --- Holiday Services ---

    pub async fn create_holiday(
        &self,
        req: CreateHolidayCalendarRequest,
    ) -> Result<HolidayCalendarResponse, ReferenceDataServiceError> {
        let holiday_type = HolidayType::from_str(&req.holiday_type)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let holiday = HolidayCalendar::new(
            req.holiday_date,
            &req.holiday_name_en,
            &req.holiday_name_fr,
            &req.holiday_name_ar,
            holiday_type,
            req.is_banking_holiday,
        )
        .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_holiday(&holiday)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.holiday_to_response(&holiday))
    }

    pub async fn is_banking_holiday(&self, date: DateTime<Utc>) -> Result<bool, ReferenceDataServiceError> {
        self.repository
            .is_banking_holiday(date)
            .await
            .map_err(ReferenceDataServiceError::Internal)
    }

    pub async fn find_banking_holidays(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<HolidayCalendarResponse>, ReferenceDataServiceError> {
        let holidays = self
            .repository
            .find_banking_holidays_by_date(from, to)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(holidays.iter().map(|h| self.holiday_to_response(h)).collect())
    }

    fn holiday_to_response(&self, holiday: &HolidayCalendar) -> HolidayCalendarResponse {
        HolidayCalendarResponse {
            id: holiday.id().to_string(),
            holiday_date: holiday.holiday_date(),
            holiday_name_en: holiday.holiday_name_en().to_string(),
            holiday_name_fr: holiday.holiday_name_fr().to_string(),
            holiday_name_ar: holiday.holiday_name_ar().to_string(),
            holiday_type: holiday.holiday_type().to_string(),
            is_banking_holiday: holiday.is_banking_holiday(),
            created_at: holiday.created_at(),
            updated_at: holiday.updated_at(),
        }
    }

    // --- System Parameter Services ---

    pub async fn create_system_parameter(
        &self,
        req: CreateSystemParameterRequest,
    ) -> Result<SystemParameterResponse, ReferenceDataServiceError> {
        let param_type = SystemParameterType::from_str(&req.parameter_type)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let param = SystemParameter::new(
            &req.key,
            &req.value,
            param_type,
            &req.category,
            &req.description,
            req.is_active,
            req.effective_from,
            req.effective_to,
        )
        .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_system_parameter(&param)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.system_parameter_to_response(&param))
    }

    pub async fn get_system_parameter_by_key(
        &self,
        key: &str,
    ) -> Result<SystemParameterResponse, ReferenceDataServiceError> {
        let param = self
            .repository
            .find_system_parameter_by_key(key)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::SystemParameterNotFound(key.to_string()))?;

        Ok(self.system_parameter_to_response(&param))
    }

    pub async fn list_system_parameters_by_category(
        &self,
        category: &str,
    ) -> Result<Vec<SystemParameterResponse>, ReferenceDataServiceError> {
        let params = self
            .repository
            .find_system_parameters_by_category(category)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(params.iter().map(|p| self.system_parameter_to_response(p)).collect())
    }

    fn system_parameter_to_response(&self, param: &SystemParameter) -> SystemParameterResponse {
        SystemParameterResponse {
            id: param.id().to_string(),
            key: param.key().to_string(),
            value: param.value().to_string(),
            parameter_type: param.parameter_type().to_string(),
            category: param.category().to_string(),
            description: param.description().to_string(),
            is_active: param.is_active(),
            effective_from: param.created_at(), // Placeholder - would need effective_from from entity
            effective_to: None,
            created_at: param.created_at(),
            updated_at: param.updated_at(),
        }
    }

    // --- Regulatory Code Services ---

    pub async fn create_regulatory_code(
        &self,
        req: CreateRegulatoryCodeRequest,
    ) -> Result<RegulatoryCodeResponse, ReferenceDataServiceError> {
        let classification = RegulatoryClassification::from_str(&req.classification)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let code = RegulatoryCode::new(
            &req.code,
            &req.description_en,
            &req.description_fr,
            classification,
            req.is_active,
            req.effective_from,
            req.effective_to,
        )
        .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_regulatory_code(&code)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.regulatory_code_to_response(&code))
    }

    pub async fn get_regulatory_code_by_code(
        &self,
        code: &str,
    ) -> Result<RegulatoryCodeResponse, ReferenceDataServiceError> {
        let reg_code = self
            .repository
            .find_regulatory_code_by_code(code)
            .await
            .map_err(ReferenceDataServiceError::Internal)?
            .ok_or(ReferenceDataServiceError::RegulatoryCodeNotFound(code.to_string()))?;

        Ok(self.regulatory_code_to_response(&reg_code))
    }

    pub async fn list_active_regulatory_codes(
        &self,
    ) -> Result<Vec<RegulatoryCodeResponse>, ReferenceDataServiceError> {
        let codes = self
            .repository
            .list_active_regulatory_codes()
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(codes.iter().map(|c| self.regulatory_code_to_response(c)).collect())
    }

    fn regulatory_code_to_response(&self, code: &RegulatoryCode) -> RegulatoryCodeResponse {
        RegulatoryCodeResponse {
            id: code.id().to_string(),
            code: code.code().to_string(),
            description_en: code.description_en().to_string(),
            description_fr: code.description_fr().to_string(),
            classification: code.classification().to_string(),
            is_active: code.is_active(),
            effective_from: code.created_at(), // Placeholder
            effective_to: None,
            created_at: code.created_at(),
            updated_at: code.updated_at(),
        }
    }

    // --- Fee Schedule Services ---

    pub async fn create_fee_schedule(
        &self,
        req: CreateFeeScheduleRequest,
    ) -> Result<FeeScheduleReferenceResponse, ReferenceDataServiceError> {
        let fee_type = FeeType::from_str(&req.fee_type)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let currency = CurrencyCodeVo::new(&req.currency_code)
            .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        let fee = FeeScheduleReference::new(
            fee_type,
            req.amount_cents,
            currency,
            &req.description_en,
            &req.description_fr,
            req.is_active,
            req.effective_from,
            req.effective_to,
        )
        .map_err(|e| ReferenceDataServiceError::DomainError(e.to_string()))?;

        self.repository
            .save_fee_schedule(&fee)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(self.fee_schedule_to_response(&fee))
    }

    pub async fn find_fee_schedules_by_type(
        &self,
        fee_type: &str,
    ) -> Result<Vec<FeeScheduleReferenceResponse>, ReferenceDataServiceError> {
        let fees = self
            .repository
            .find_fee_schedules_by_type(fee_type)
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(fees.iter().map(|f| self.fee_schedule_to_response(f)).collect())
    }

    pub async fn list_active_fee_schedules(
        &self,
    ) -> Result<Vec<FeeScheduleReferenceResponse>, ReferenceDataServiceError> {
        let fees = self
            .repository
            .list_active_fee_schedules()
            .await
            .map_err(ReferenceDataServiceError::Internal)?;

        Ok(fees.iter().map(|f| self.fee_schedule_to_response(f)).collect())
    }

    fn fee_schedule_to_response(&self, fee: &FeeScheduleReference) -> FeeScheduleReferenceResponse {
        FeeScheduleReferenceResponse {
            id: fee.id().to_string(),
            fee_type: fee.fee_type().to_string(),
            amount_cents: fee.amount_cents(),
            currency_code: fee.currency_code().to_string(),
            description_en: fee.description_en().to_string(),
            description_fr: fee.description_fr().to_string(),
            is_active: fee.is_active(),
            effective_from: fee.created_at(), // Placeholder
            effective_to: None,
            created_at: fee.created_at(),
            updated_at: fee.updated_at(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock repository for testing
    struct MockReferenceDataRepository;

    #[async_trait::async_trait]
    impl IReferenceDataRepository for MockReferenceDataRepository {
        async fn save_country(&self, _country: &CountryCode) -> Result<(), String> {
            Ok(())
        }

        async fn find_country_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<CountryCode>, String> {
            Ok(None)
        }

        async fn find_country_by_iso_alpha2(&self, _code: &str) -> Result<Option<CountryCode>, String> {
            Ok(None)
        }

        async fn find_country_by_iso_alpha3(&self, _code: &str) -> Result<Option<CountryCode>, String> {
            Ok(None)
        }

        async fn list_countries(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<CountryCode>, String> {
            Ok(vec![])
        }

        async fn list_active_countries(&self) -> Result<Vec<CountryCode>, String> {
            Ok(vec![])
        }

        async fn save_currency(&self, _currency: &CurrencyReference) -> Result<(), String> {
            Ok(())
        }

        async fn find_currency_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<CurrencyReference>, String> {
            Ok(None)
        }

        async fn find_currency_by_code(&self, _code: &str) -> Result<Option<CurrencyReference>, String> {
            Ok(None)
        }

        async fn list_currencies(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<CurrencyReference>, String> {
            Ok(vec![])
        }

        async fn list_active_currencies(&self) -> Result<Vec<CurrencyReference>, String> {
            Ok(vec![])
        }

        async fn save_bank_code(&self, _bank: &BankCode) -> Result<(), String> {
            Ok(())
        }

        async fn find_bank_code_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<BankCode>, String> {
            Ok(None)
        }

        async fn find_bank_code_by_bic(&self, _bic: &str) -> Result<Option<BankCode>, String> {
            Ok(None)
        }

        async fn list_bank_codes(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<BankCode>, String> {
            Ok(vec![])
        }

        async fn list_active_bank_codes(&self) -> Result<Vec<BankCode>, String> {
            Ok(vec![])
        }

        async fn save_branch_code(&self, _branch: &BranchCode) -> Result<(), String> {
            Ok(())
        }

        async fn find_branch_code_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<BranchCode>, String> {
            Ok(None)
        }

        async fn find_branch_code_by_code(&self, _code: &str) -> Result<Option<BranchCode>, String> {
            Ok(None)
        }

        async fn find_branches_by_bank_bic(&self, _bic: &str) -> Result<Vec<BranchCode>, String> {
            Ok(vec![])
        }

        async fn list_branch_codes(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<BranchCode>, String> {
            Ok(vec![])
        }

        async fn list_active_branch_codes(&self) -> Result<Vec<BranchCode>, String> {
            Ok(vec![])
        }

        async fn save_holiday(&self, _holiday: &HolidayCalendar) -> Result<(), String> {
            Ok(())
        }

        async fn find_holiday_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<HolidayCalendar>, String> {
            Ok(None)
        }

        async fn find_holidays_by_date(
            &self,
            _from: DateTime<Utc>,
            _to: DateTime<Utc>,
        ) -> Result<Vec<HolidayCalendar>, String> {
            Ok(vec![])
        }

        async fn find_banking_holidays_by_date(
            &self,
            _from: DateTime<Utc>,
            _to: DateTime<Utc>,
        ) -> Result<Vec<HolidayCalendar>, String> {
            Ok(vec![])
        }

        async fn is_banking_holiday(&self, _date: DateTime<Utc>) -> Result<bool, String> {
            Ok(false)
        }

        async fn save_system_parameter(&self, _param: &SystemParameter) -> Result<(), String> {
            Ok(())
        }

        async fn find_system_parameter_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<SystemParameter>, String> {
            Ok(None)
        }

        async fn find_system_parameter_by_key(&self, _key: &str) -> Result<Option<SystemParameter>, String> {
            Ok(None)
        }

        async fn find_system_parameters_by_category(
            &self,
            _category: &str,
        ) -> Result<Vec<SystemParameter>, String> {
            Ok(vec![])
        }

        async fn list_active_system_parameters(&self) -> Result<Vec<SystemParameter>, String> {
            Ok(vec![])
        }

        async fn save_regulatory_code(&self, _code: &RegulatoryCode) -> Result<(), String> {
            Ok(())
        }

        async fn find_regulatory_code_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<RegulatoryCode>, String> {
            Ok(None)
        }

        async fn find_regulatory_code_by_code(&self, _code: &str) -> Result<Option<RegulatoryCode>, String> {
            Ok(None)
        }

        async fn list_regulatory_codes(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<RegulatoryCode>, String> {
            Ok(vec![])
        }

        async fn list_active_regulatory_codes(&self) -> Result<Vec<RegulatoryCode>, String> {
            Ok(vec![])
        }

        async fn save_fee_schedule(&self, _fee: &FeeScheduleReference) -> Result<(), String> {
            Ok(())
        }

        async fn find_fee_schedule_by_id(
            &self,
            _id: &ReferenceDataId,
        ) -> Result<Option<FeeScheduleReference>, String> {
            Ok(None)
        }

        async fn find_fee_schedules_by_type(&self, _fee_type: &str) -> Result<Vec<FeeScheduleReference>, String> {
            Ok(vec![])
        }

        async fn list_fee_schedules(
            &self,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<FeeScheduleReference>, String> {
            Ok(vec![])
        }

        async fn list_active_fee_schedules(&self) -> Result<Vec<FeeScheduleReference>, String> {
            Ok(vec![])
        }
    }

    #[test]
    fn test_service_creation() {
        let repo = Arc::new(MockReferenceDataRepository);
        let service = ReferenceDataService::new(repo);
        // Service created successfully
        assert!(true);
    }
}
