use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


// ============================================================
// CreateProductRequest DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub product_type: String, // "CurrentAccount", etc.
    pub interest_rate: Option<CreateInterestRateDto>,
    pub fees: Vec<CreateFeeDto>,
    pub eligibility: CreateEligibilityDto,
    pub segment_pricing: Option<HashMap<String, Decimal>>,
    pub min_balance: Option<Decimal>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInterestRateDto {
    pub annual_rate: Decimal,
    pub calc_method: String, // "Simple", "Compound", "Daily"
    pub floor_rate: Option<Decimal>,
    pub ceiling_rate: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeeDto {
    pub fee_type: String, // "Monthly", "Transaction", etc.
    pub fixed_amount: Option<Decimal>,
    pub rate: Option<Decimal>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub charged_on: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEligibilityDto {
    pub min_age: Option<u8>,
    pub max_age: Option<u8>,
    pub min_income: Option<Decimal>,
    pub required_segment: Option<String>,
    pub min_credit_score: Option<u32>,
}

// ============================================================
// UpdateProductRequest DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub interest_rate: Option<CreateInterestRateDto>,
    pub fees: Option<Vec<CreateFeeDto>>,
    pub min_balance: Option<Decimal>,
}

// ============================================================
// ProductResponse DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub product_type: String,
    pub status: String,
    pub interest_rate: Option<InterestRateResponse>,
    pub fees: Vec<FeeResponse>,
    pub eligibility: EligibilityResponse,
    pub segment_pricing: HashMap<String, Decimal>,
    pub min_balance: Option<Decimal>,
    pub currency: String,
    pub version: u32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateResponse {
    pub annual_rate: Decimal,
    pub calc_method: String,
    pub floor_rate: Option<Decimal>,
    pub ceiling_rate: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeResponse {
    pub id: String,
    pub fee_type: String,
    pub fixed_amount: Option<Decimal>,
    pub rate: Option<Decimal>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub charged_on: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityResponse {
    pub min_age: Option<u8>,
    pub max_age: Option<u8>,
    pub min_income: Option<Decimal>,
    pub required_segment: Option<String>,
    pub min_credit_score: Option<u32>,
}

// ============================================================
// PriceQuote DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceQuote {
    pub product_id: String,
    pub rate: Decimal,
    pub fees: Decimal,
    pub total_cost: Decimal,
    pub segment_applied: String,
    pub currency: String,
}

// ============================================================
// CreatePricingGridRequest DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePricingGridRequest {
    pub product_id: String,
    pub bands: Vec<CreatePricingBandDto>,
    pub effective_from: String, // ISO 8601 datetime
    pub effective_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePricingBandDto {
    pub min_amount: Decimal,
    pub max_amount: Option<Decimal>,
    pub rate: Decimal,
    pub fees_override: Option<Decimal>,
    pub sort_order: u16,
}

// ============================================================
// PricingGridResponse DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingGridResponse {
    pub id: String,
    pub product_id: String,
    pub bands: Vec<PricingBandResponse>,
    pub effective_from: String,
    pub effective_to: Option<String>,
    pub active: bool,
    pub created_by: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingBandResponse {
    pub id: String,
    pub min_amount: Decimal,
    pub max_amount: Option<Decimal>,
    pub rate: Decimal,
    pub fees_override: Option<Decimal>,
    pub sort_order: u16,
}

// ============================================================
// EligibilityCheckRequest DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityCheckRequest {
    pub product_id: Option<String>,
    pub age: u8,
    pub income: Decimal,
    pub segment: String, // "Standard", "Premium", etc.
    pub credit_score: u32,
}

// ============================================================
// EligibilityCheckResponse DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityCheckResponse {
    pub eligible: bool,
    pub product_id: Option<String>,
    pub reasons: Vec<String>,
}

// ============================================================
// MaturityResult DTO (for interest calculations)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaturityResult {
    pub principal: Decimal,
    pub total_interest: Decimal,
    pub final_amount: Decimal,
    pub currency: String,
}

// ============================================================
// AccrualResult DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccrualResult {
    pub processed: i32,
    pub skipped: i32,
    pub total_interest: Decimal,
}
