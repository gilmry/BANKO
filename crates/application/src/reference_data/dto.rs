use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- Country Code DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCodeResponse {
    pub id: String,
    pub iso_alpha2: String,
    pub iso_alpha3: String,
    pub iso_numeric: String,
    pub name_en: String,
    pub name_fr: String,
    pub name_ar: String,
    pub is_sanctioned: bool,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCountryCodeRequest {
    pub iso_alpha2: String,
    pub iso_alpha3: String,
    pub iso_numeric: String,
    pub name_en: String,
    pub name_fr: String,
    pub name_ar: String,
    pub is_sanctioned: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCountryCodeRequest {
    pub name_en: Option<String>,
    pub name_fr: Option<String>,
    pub name_ar: Option<String>,
    pub is_sanctioned: Option<bool>,
}

// --- Currency Reference DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyReferenceResponse {
    pub id: String,
    pub code: String,
    pub name_en: String,
    pub name_fr: String,
    pub decimal_places: i32,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCurrencyReferenceRequest {
    pub code: String,
    pub name_en: String,
    pub name_fr: String,
    pub decimal_places: i32,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCurrencyReferenceRequest {
    pub name_en: Option<String>,
    pub name_fr: Option<String>,
    pub decimal_places: Option<i32>,
    pub is_active: Option<bool>,
}

// --- Bank Code DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct BankCodeResponse {
    pub id: String,
    pub bic: String,
    pub bank_name: String,
    pub country_iso_alpha2: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBankCodeRequest {
    pub bic: String,
    pub bank_name: String,
    pub country_iso_alpha2: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBankCodeRequest {
    pub bank_name: Option<String>,
    pub is_active: Option<bool>,
}

// --- Branch Code DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchCodeResponse {
    pub id: String,
    pub branch_code: String,
    pub branch_name: String,
    pub bank_bic: String,
    pub city: String,
    pub address: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBranchCodeRequest {
    pub branch_code: String,
    pub branch_name: String,
    pub bank_bic: String,
    pub city: String,
    pub address: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchCodeRequest {
    pub branch_name: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

// --- Holiday Calendar DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct HolidayCalendarResponse {
    pub id: String,
    pub holiday_date: DateTime<Utc>,
    pub holiday_name_en: String,
    pub holiday_name_fr: String,
    pub holiday_name_ar: String,
    pub holiday_type: String,
    pub is_banking_holiday: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHolidayCalendarRequest {
    pub holiday_date: DateTime<Utc>,
    pub holiday_name_en: String,
    pub holiday_name_fr: String,
    pub holiday_name_ar: String,
    pub holiday_type: String,
    pub is_banking_holiday: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHolidayCalendarRequest {
    pub is_banking_holiday: Option<bool>,
}

// --- System Parameter DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemParameterResponse {
    pub id: String,
    pub key: String,
    pub value: String,
    pub parameter_type: String,
    pub category: String,
    pub description: String,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSystemParameterRequest {
    pub key: String,
    pub value: String,
    pub parameter_type: String,
    pub category: String,
    pub description: String,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSystemParameterRequest {
    pub value: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

// --- Regulatory Code DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct RegulatoryCodeResponse {
    pub id: String,
    pub code: String,
    pub description_en: String,
    pub description_fr: String,
    pub classification: String,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRegulatoryCodeRequest {
    pub code: String,
    pub description_en: String,
    pub description_fr: String,
    pub classification: String,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRegulatoryCodeRequest {
    pub description_en: Option<String>,
    pub description_fr: Option<String>,
    pub is_active: Option<bool>,
}

// --- Fee Schedule DTOs ---

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeScheduleReferenceResponse {
    pub id: String,
    pub fee_type: String,
    pub amount_cents: i64,
    pub currency_code: String,
    pub description_en: String,
    pub description_fr: String,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFeeScheduleRequest {
    pub fee_type: String,
    pub amount_cents: i64,
    pub currency_code: String,
    pub description_en: String,
    pub description_fr: String,
    pub is_active: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeeScheduleRequest {
    pub amount_cents: Option<i64>,
    pub description_en: Option<String>,
    pub description_fr: Option<String>,
    pub is_active: Option<bool>,
}

// --- Pagination DTOs ---

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total_count: i64,
    pub limit: i64,
    pub offset: i64,
}
