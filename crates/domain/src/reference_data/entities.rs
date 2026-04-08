use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;

use super::value_objects::{
    BicCodeVo, CountryCodeVo, CurrencyCodeVo, FeeType, HolidayType, ReferenceDataId,
    RegulatoryClassification, SystemParameterType,
};

// --- Country Code Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryCode {
    id: ReferenceDataId,
    code: CountryCodeVo,
    name_en: String,
    name_fr: String,
    name_ar: String,
    is_sanctioned: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CountryCode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: CountryCodeVo,
        name_en: &str,
        name_fr: &str,
        name_ar: &str,
        is_sanctioned: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        if name_en.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "English name cannot be empty".to_string(),
            ));
        }
        if name_fr.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "French name cannot be empty".to_string(),
            ));
        }
        if name_ar.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Arabic name cannot be empty".to_string(),
            ));
        }

        if let Some(to) = effective_to {
            if to <= effective_from {
                return Err(DomainError::ValidationError(
                    "Effective end date must be after start date".to_string(),
                ));
            }
        }

        Ok(CountryCode {
            id: ReferenceDataId::new(),
            code,
            name_en: name_en.trim().to_string(),
            name_fr: name_fr.trim().to_string(),
            name_ar: name_ar.trim().to_string(),
            is_sanctioned,
            effective_from,
            effective_to,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        code: CountryCodeVo,
        name_en: String,
        name_fr: String,
        name_ar: String,
        is_sanctioned: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        CountryCode {
            id,
            code,
            name_en,
            name_fr,
            name_ar,
            is_sanctioned,
            effective_from,
            effective_to,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn code(&self) -> &CountryCodeVo {
        &self.code
    }

    pub fn name_en(&self) -> &str {
        &self.name_en
    }

    pub fn name_fr(&self) -> &str {
        &self.name_fr
    }

    pub fn name_ar(&self) -> &str {
        &self.name_ar
    }

    pub fn is_sanctioned(&self) -> bool {
        self.is_sanctioned
    }

    pub fn effective_from(&self) -> DateTime<Utc> {
        self.effective_from
    }

    pub fn effective_to(&self) -> Option<DateTime<Utc>> {
        self.effective_to
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.effective_from
            && self.effective_to.is_none_or(|to| now < to)
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Currency Reference Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyReference {
    id: ReferenceDataId,
    code: CurrencyCodeVo,
    name_en: String,
    name_fr: String,
    decimal_places: i32,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CurrencyReference {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: CurrencyCodeVo,
        name_en: &str,
        name_fr: &str,
        decimal_places: i32,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        if name_en.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "English name cannot be empty".to_string(),
            ));
        }
        if name_fr.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "French name cannot be empty".to_string(),
            ));
        }
        if !(0..=8).contains(&decimal_places) {
            return Err(DomainError::ValidationError(
                "Decimal places must be between 0 and 8".to_string(),
            ));
        }

        if let Some(to) = effective_to {
            if to <= effective_from {
                return Err(DomainError::ValidationError(
                    "Effective end date must be after start date".to_string(),
                ));
            }
        }

        Ok(CurrencyReference {
            id: ReferenceDataId::new(),
            code,
            name_en: name_en.trim().to_string(),
            name_fr: name_fr.trim().to_string(),
            decimal_places,
            is_active,
            effective_from,
            effective_to,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        code: CurrencyCodeVo,
        name_en: String,
        name_fr: String,
        decimal_places: i32,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        CurrencyReference {
            id,
            code,
            name_en,
            name_fr,
            decimal_places,
            is_active,
            effective_from,
            effective_to,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn code(&self) -> &CurrencyCodeVo {
        &self.code
    }

    pub fn name_en(&self) -> &str {
        &self.name_en
    }

    pub fn name_fr(&self) -> &str {
        &self.name_fr
    }

    pub fn decimal_places(&self) -> i32 {
        self.decimal_places
    }

    pub fn is_active(&self) -> bool {
        self.is_active && {
            let now = Utc::now();
            now >= self.effective_from && self.effective_to.is_none_or(|to| now < to)
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Bank Code Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCode {
    id: ReferenceDataId,
    bic: BicCodeVo,
    bank_name: String,
    country_code: CountryCodeVo,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BankCode {
    pub fn new(
        bic: BicCodeVo,
        bank_name: &str,
        country_code: CountryCodeVo,
        is_active: bool,
    ) -> Result<Self, DomainError> {
        if bank_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Bank name cannot be empty".to_string(),
            ));
        }

        Ok(BankCode {
            id: ReferenceDataId::new(),
            bic,
            bank_name: bank_name.trim().to_string(),
            country_code,
            is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        bic: BicCodeVo,
        bank_name: String,
        country_code: CountryCodeVo,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        BankCode {
            id,
            bic,
            bank_name,
            country_code,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn bic(&self) -> &BicCodeVo {
        &self.bic
    }

    pub fn bank_name(&self) -> &str {
        &self.bank_name
    }

    pub fn country_code(&self) -> &CountryCodeVo {
        &self.country_code
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Branch Code Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCode {
    id: ReferenceDataId,
    branch_code: String,
    branch_name: String,
    bank_bic: BicCodeVo,
    city: String,
    address: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BranchCode {
    pub fn new(
        branch_code: &str,
        branch_name: &str,
        bank_bic: BicCodeVo,
        city: &str,
        address: &str,
        is_active: bool,
    ) -> Result<Self, DomainError> {
        if branch_code.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Branch code cannot be empty".to_string(),
            ));
        }
        if branch_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Branch name cannot be empty".to_string(),
            ));
        }
        if city.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "City cannot be empty".to_string(),
            ));
        }
        if address.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Address cannot be empty".to_string(),
            ));
        }

        Ok(BranchCode {
            id: ReferenceDataId::new(),
            branch_code: branch_code.trim().to_string(),
            branch_name: branch_name.trim().to_string(),
            bank_bic,
            city: city.trim().to_string(),
            address: address.trim().to_string(),
            is_active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        branch_code: String,
        branch_name: String,
        bank_bic: BicCodeVo,
        city: String,
        address: String,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        BranchCode {
            id,
            branch_code,
            branch_name,
            bank_bic,
            city,
            address,
            is_active,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn branch_code(&self) -> &str {
        &self.branch_code
    }

    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    pub fn bank_bic(&self) -> &BicCodeVo {
        &self.bank_bic
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Holiday Calendar Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayCalendar {
    id: ReferenceDataId,
    holiday_date: DateTime<Utc>,
    holiday_name_en: String,
    holiday_name_fr: String,
    holiday_name_ar: String,
    holiday_type: HolidayType,
    is_banking_holiday: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl HolidayCalendar {
    pub fn new(
        holiday_date: DateTime<Utc>,
        holiday_name_en: &str,
        holiday_name_fr: &str,
        holiday_name_ar: &str,
        holiday_type: HolidayType,
        is_banking_holiday: bool,
    ) -> Result<Self, DomainError> {
        if holiday_name_en.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "English holiday name cannot be empty".to_string(),
            ));
        }
        if holiday_name_fr.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "French holiday name cannot be empty".to_string(),
            ));
        }
        if holiday_name_ar.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Arabic holiday name cannot be empty".to_string(),
            ));
        }

        Ok(HolidayCalendar {
            id: ReferenceDataId::new(),
            holiday_date,
            holiday_name_en: holiday_name_en.trim().to_string(),
            holiday_name_fr: holiday_name_fr.trim().to_string(),
            holiday_name_ar: holiday_name_ar.trim().to_string(),
            holiday_type,
            is_banking_holiday,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        holiday_date: DateTime<Utc>,
        holiday_name_en: String,
        holiday_name_fr: String,
        holiday_name_ar: String,
        holiday_type: HolidayType,
        is_banking_holiday: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        HolidayCalendar {
            id,
            holiday_date,
            holiday_name_en,
            holiday_name_fr,
            holiday_name_ar,
            holiday_type,
            is_banking_holiday,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn holiday_date(&self) -> DateTime<Utc> {
        self.holiday_date
    }

    pub fn holiday_name_en(&self) -> &str {
        &self.holiday_name_en
    }

    pub fn holiday_name_fr(&self) -> &str {
        &self.holiday_name_fr
    }

    pub fn holiday_name_ar(&self) -> &str {
        &self.holiday_name_ar
    }

    pub fn holiday_type(&self) -> HolidayType {
        self.holiday_type
    }

    pub fn is_banking_holiday(&self) -> bool {
        self.is_banking_holiday
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- System Parameter Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemParameter {
    id: ReferenceDataId,
    key: String,
    value: String,
    parameter_type: SystemParameterType,
    category: String,
    description: String,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SystemParameter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        key: &str,
        value: &str,
        parameter_type: SystemParameterType,
        category: &str,
        description: &str,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        if key.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Parameter key cannot be empty".to_string(),
            ));
        }
        if value.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Parameter value cannot be empty".to_string(),
            ));
        }
        if category.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Parameter category cannot be empty".to_string(),
            ));
        }

        // Validate value type
        match parameter_type {
            SystemParameterType::Integer => {
                if value.parse::<i64>().is_err() {
                    return Err(DomainError::ValidationError(
                        "Value is not a valid integer".to_string(),
                    ));
                }
            }
            SystemParameterType::Decimal => {
                if value.parse::<f64>().is_err() {
                    return Err(DomainError::ValidationError(
                        "Value is not a valid decimal".to_string(),
                    ));
                }
            }
            SystemParameterType::Boolean => {
                if !matches!(value.to_lowercase().as_str(), "true" | "false") {
                    return Err(DomainError::ValidationError(
                        "Value must be 'true' or 'false'".to_string(),
                    ));
                }
            }
            SystemParameterType::String => {} // Any string is valid
        }

        if let Some(to) = effective_to {
            if to <= effective_from {
                return Err(DomainError::ValidationError(
                    "Effective end date must be after start date".to_string(),
                ));
            }
        }

        Ok(SystemParameter {
            id: ReferenceDataId::new(),
            key: key.trim().to_string(),
            value: value.trim().to_string(),
            parameter_type,
            category: category.trim().to_string(),
            description: description.trim().to_string(),
            is_active,
            effective_from,
            effective_to,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        key: String,
        value: String,
        parameter_type: SystemParameterType,
        category: String,
        description: String,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SystemParameter {
            id,
            key,
            value,
            parameter_type,
            category,
            description,
            is_active,
            effective_from,
            effective_to,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn parameter_type(&self) -> SystemParameterType {
        self.parameter_type
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_active(&self) -> bool {
        self.is_active && {
            let now = Utc::now();
            now >= self.effective_from && self.effective_to.is_none_or(|to| now < to)
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Regulatory Code Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryCode {
    id: ReferenceDataId,
    code: String,
    description_en: String,
    description_fr: String,
    classification: RegulatoryClassification,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RegulatoryCode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: &str,
        description_en: &str,
        description_fr: &str,
        classification: RegulatoryClassification,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        if code.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Code cannot be empty".to_string(),
            ));
        }
        if description_en.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "English description cannot be empty".to_string(),
            ));
        }
        if description_fr.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "French description cannot be empty".to_string(),
            ));
        }

        if let Some(to) = effective_to {
            if to <= effective_from {
                return Err(DomainError::ValidationError(
                    "Effective end date must be after start date".to_string(),
                ));
            }
        }

        Ok(RegulatoryCode {
            id: ReferenceDataId::new(),
            code: code.trim().to_string(),
            description_en: description_en.trim().to_string(),
            description_fr: description_fr.trim().to_string(),
            classification,
            is_active,
            effective_from,
            effective_to,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        code: String,
        description_en: String,
        description_fr: String,
        classification: RegulatoryClassification,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        RegulatoryCode {
            id,
            code,
            description_en,
            description_fr,
            classification,
            is_active,
            effective_from,
            effective_to,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn description_en(&self) -> &str {
        &self.description_en
    }

    pub fn description_fr(&self) -> &str {
        &self.description_fr
    }

    pub fn classification(&self) -> RegulatoryClassification {
        self.classification
    }

    pub fn is_active(&self) -> bool {
        self.is_active && {
            let now = Utc::now();
            now >= self.effective_from && self.effective_to.is_none_or(|to| now < to)
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// --- Fee Schedule Reference Entity ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeScheduleReference {
    id: ReferenceDataId,
    fee_type: FeeType,
    amount_cents: i64,
    currency_code: CurrencyCodeVo,
    description_en: String,
    description_fr: String,
    is_active: bool,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FeeScheduleReference {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fee_type: FeeType,
        amount_cents: i64,
        currency_code: CurrencyCodeVo,
        description_en: &str,
        description_fr: &str,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        if amount_cents < 0 {
            return Err(DomainError::ValidationError(
                "Fee amount cannot be negative".to_string(),
            ));
        }
        if description_en.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "English description cannot be empty".to_string(),
            ));
        }
        if description_fr.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "French description cannot be empty".to_string(),
            ));
        }

        if let Some(to) = effective_to {
            if to <= effective_from {
                return Err(DomainError::ValidationError(
                    "Effective end date must be after start date".to_string(),
                ));
            }
        }

        Ok(FeeScheduleReference {
            id: ReferenceDataId::new(),
            fee_type,
            amount_cents,
            currency_code,
            description_en: description_en.trim().to_string(),
            description_fr: description_fr.trim().to_string(),
            is_active,
            effective_from,
            effective_to,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ReferenceDataId,
        fee_type: FeeType,
        amount_cents: i64,
        currency_code: CurrencyCodeVo,
        description_en: String,
        description_fr: String,
        is_active: bool,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        FeeScheduleReference {
            id,
            fee_type,
            amount_cents,
            currency_code,
            description_en,
            description_fr,
            is_active,
            effective_from,
            effective_to,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ReferenceDataId {
        &self.id
    }

    pub fn fee_type(&self) -> FeeType {
        self.fee_type
    }

    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    pub fn currency_code(&self) -> &CurrencyCodeVo {
        &self.currency_code
    }

    pub fn description_en(&self) -> &str {
        &self.description_en
    }

    pub fn description_fr(&self) -> &str {
        &self.description_fr
    }

    pub fn is_active(&self) -> bool {
        self.is_active && {
            let now = Utc::now();
            now >= self.effective_from && self.effective_to.is_none_or(|to| now < to)
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_code_new_valid() {
        let code = CountryCodeVo::new("TN", "TUN", "788").unwrap();
        let country = CountryCode::new(
            code,
            "Tunisia",
            "Tunisie",
            "تونس",
            false,
            Utc::now(),
            None,
        );
        assert!(country.is_ok());
    }

    #[test]
    fn test_country_code_is_active() {
        let code = CountryCodeVo::new("TN", "TUN", "788").unwrap();
        let now = Utc::now();
        let country = CountryCode::new(
            code,
            "Tunisia",
            "Tunisie",
            "تونس",
            false,
            now,
            Some(now + chrono::Duration::days(30)),
        )
        .unwrap();
        assert!(country.is_active());
    }

    #[test]
    fn test_currency_reference_new_valid() {
        let code = CurrencyCodeVo::new("USD").unwrap();
        let currency = CurrencyReference::new(
            code,
            "US Dollar",
            "Dollar américain",
            2,
            true,
            Utc::now(),
            None,
        );
        assert!(currency.is_ok());
    }

    #[test]
    fn test_currency_reference_invalid_decimal_places() {
        let code = CurrencyCodeVo::new("USD").unwrap();
        let currency = CurrencyReference::new(
            code,
            "US Dollar",
            "Dollar américain",
            10,
            true,
            Utc::now(),
            None,
        );
        assert!(currency.is_err());
    }

    #[test]
    fn test_bank_code_new_valid() {
        let bic = BicCodeVo::new("BNAFFRPP").unwrap();
        let country = CountryCodeVo::new("FR", "FRA", "250").unwrap();
        let bank = BankCode::new(bic, "BNP Paribas", country, true);
        assert!(bank.is_ok());
    }

    #[test]
    fn test_branch_code_new_valid() {
        let bic = BicCodeVo::new("BNAFFRPP").unwrap();
        let branch = BranchCode::new(
            "001",
            "Main Branch",
            bic,
            "Paris",
            "123 Rue de Rivoli",
            true,
        );
        assert!(branch.is_ok());
    }

    #[test]
    fn test_holiday_calendar_new_valid() {
        let holiday = HolidayCalendar::new(
            Utc::now(),
            "Independence Day",
            "Fête de l'Indépendance",
            "عيد الاستقلال",
            HolidayType::National,
            true,
        );
        assert!(holiday.is_ok());
    }

    #[test]
    fn test_system_parameter_integer_valid() {
        let param = SystemParameter::new(
            "MAX_TRANSFER_AMOUNT",
            "100000",
            SystemParameterType::Integer,
            "Limits",
            "Maximum transfer amount in cents",
            true,
            Utc::now(),
            None,
        );
        assert!(param.is_ok());
    }

    #[test]
    fn test_system_parameter_integer_invalid() {
        let param = SystemParameter::new(
            "MAX_TRANSFER_AMOUNT",
            "not_a_number",
            SystemParameterType::Integer,
            "Limits",
            "Maximum transfer amount",
            true,
            Utc::now(),
            None,
        );
        assert!(param.is_err());
    }

    #[test]
    fn test_system_parameter_boolean_valid() {
        let param = SystemParameter::new(
            "ENABLE_FEATURE_X",
            "true",
            SystemParameterType::Boolean,
            "Features",
            "Enable feature X",
            true,
            Utc::now(),
            None,
        );
        assert!(param.is_ok());
    }

    #[test]
    fn test_regulatory_code_new_valid() {
        let code = RegulatoryCode::new(
            "BCT001",
            "Standard Risk Asset",
            "Actif à Risque Standard",
            RegulatoryClassification::StandardRisk,
            true,
            Utc::now(),
            None,
        );
        assert!(code.is_ok());
    }

    #[test]
    fn test_fee_schedule_reference_new_valid() {
        let currency = CurrencyCodeVo::new("TND").unwrap();
        let fee = FeeScheduleReference::new(
            FeeType::Transaction,
            5000,
            currency,
            "Transaction fee",
            "Frais de transaction",
            true,
            Utc::now(),
            None,
        );
        assert!(fee.is_ok());
    }

    #[test]
    fn test_fee_schedule_negative_amount() {
        let currency = CurrencyCodeVo::new("TND").unwrap();
        let fee = FeeScheduleReference::new(
            FeeType::Transaction,
            -5000,
            currency,
            "Transaction fee",
            "Frais de transaction",
            true,
            Utc::now(),
            None,
        );
        assert!(fee.is_err());
    }
}
