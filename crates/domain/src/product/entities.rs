use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ============================================================
// Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductType {
    CurrentAccount,
    SavingsAccount,
    TermDeposit,
    ConsumerLoan,
    MortgageLoan,
    Overdraft,
}

impl ProductType {
    pub fn as_str(&self) -> &str {
        match self {
            ProductType::CurrentAccount => "CurrentAccount",
            ProductType::SavingsAccount => "SavingsAccount",
            ProductType::TermDeposit => "TermDeposit",
            ProductType::ConsumerLoan => "ConsumerLoan",
            ProductType::MortgageLoan => "MortgageLoan",
            ProductType::Overdraft => "Overdraft",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "CurrentAccount" => Ok(ProductType::CurrentAccount),
            "SavingsAccount" => Ok(ProductType::SavingsAccount),
            "TermDeposit" => Ok(ProductType::TermDeposit),
            "ConsumerLoan" => Ok(ProductType::ConsumerLoan),
            "MortgageLoan" => Ok(ProductType::MortgageLoan),
            "Overdraft" => Ok(ProductType::Overdraft),
            _ => Err(format!("Unknown product type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductStatus {
    Draft,
    Active,
    Suspended,
    Discontinued,
}

impl ProductStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ProductStatus::Draft => "Draft",
            ProductStatus::Active => "Active",
            ProductStatus::Suspended => "Suspended",
            ProductStatus::Discontinued => "Discontinued",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Draft" => Ok(ProductStatus::Draft),
            "Active" => Ok(ProductStatus::Active),
            "Suspended" => Ok(ProductStatus::Suspended),
            "Discontinued" => Ok(ProductStatus::Discontinued),
            _ => Err(format!("Unknown product status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeeType {
    Monthly,
    Transaction,
    Setup,
    EarlyWithdrawal,
    Conversion,
    Penalty,
    OverdraftFee,
}

impl FeeType {
    pub fn as_str(&self) -> &str {
        match self {
            FeeType::Monthly => "Monthly",
            FeeType::Transaction => "Transaction",
            FeeType::Setup => "Setup",
            FeeType::EarlyWithdrawal => "EarlyWithdrawal",
            FeeType::Conversion => "Conversion",
            FeeType::Penalty => "Penalty",
            FeeType::OverdraftFee => "OverdraftFee",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Monthly" => Ok(FeeType::Monthly),
            "Transaction" => Ok(FeeType::Transaction),
            "Setup" => Ok(FeeType::Setup),
            "EarlyWithdrawal" => Ok(FeeType::EarlyWithdrawal),
            "Conversion" => Ok(FeeType::Conversion),
            "Penalty" => Ok(FeeType::Penalty),
            "OverdraftFee" => Ok(FeeType::OverdraftFee),
            _ => Err(format!("Unknown fee type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalcMethod {
    Simple,
    Compound,
    Daily,
}

impl CalcMethod {
    pub fn as_str(&self) -> &str {
        match self {
            CalcMethod::Simple => "Simple",
            CalcMethod::Compound => "Compound",
            CalcMethod::Daily => "Daily",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Simple" => Ok(CalcMethod::Simple),
            "Compound" => Ok(CalcMethod::Compound),
            "Daily" => Ok(CalcMethod::Daily),
            _ => Err(format!("Unknown calculation method: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomerSegment {
    Standard,
    Junior,
    Premium,
    VIP,
    Corporate,
}

impl CustomerSegment {
    pub fn as_str(&self) -> &str {
        match self {
            CustomerSegment::Standard => "Standard",
            CustomerSegment::Junior => "Junior",
            CustomerSegment::Premium => "Premium",
            CustomerSegment::VIP => "VIP",
            CustomerSegment::Corporate => "Corporate",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Standard" => Ok(CustomerSegment::Standard),
            "Junior" => Ok(CustomerSegment::Junior),
            "Premium" => Ok(CustomerSegment::Premium),
            "VIP" => Ok(CustomerSegment::VIP),
            "Corporate" => Ok(CustomerSegment::Corporate),
            _ => Err(format!("Unknown customer segment: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    Daily,
    Monthly,
    Quarterly,
    Yearly,
}

impl Frequency {
    pub fn as_str(&self) -> &str {
        match self {
            Frequency::Daily => "Daily",
            Frequency::Monthly => "Monthly",
            Frequency::Quarterly => "Quarterly",
            Frequency::Yearly => "Yearly",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Daily" => Ok(Frequency::Daily),
            "Monthly" => Ok(Frequency::Monthly),
            "Quarterly" => Ok(Frequency::Quarterly),
            "Yearly" => Ok(Frequency::Yearly),
            _ => Err(format!("Unknown frequency: {}", s)),
        }
    }
}

// ============================================================
// InterestRate struct
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRate {
    annual_rate: Decimal,
    calc_method: CalcMethod,
    floor_rate: Option<Decimal>,
    ceiling_rate: Option<Decimal>,
}

impl InterestRate {
    pub fn new(
        annual_rate: Decimal,
        calc_method: CalcMethod,
        floor_rate: Option<Decimal>,
        ceiling_rate: Option<Decimal>,
    ) -> Result<Self, String> {
        // Validate rate is non-negative and <= 100
        if annual_rate < Decimal::ZERO || annual_rate > Decimal::from(100) {
            return Err("Annual rate must be between 0 and 100".to_string());
        }

        // If both floor and ceiling are provided, validate floor <= ceiling
        if let (Some(f), Some(c)) = (floor_rate, ceiling_rate) {
            if f > c {
                return Err("Floor rate cannot exceed ceiling rate".to_string());
            }
        }

        Ok(InterestRate {
            annual_rate,
            calc_method,
            floor_rate,
            ceiling_rate,
        })
    }

    pub fn annual_rate(&self) -> Decimal {
        self.annual_rate
    }

    pub fn calc_method(&self) -> CalcMethod {
        self.calc_method
    }

    pub fn floor_rate(&self) -> Option<Decimal> {
        self.floor_rate
    }

    pub fn ceiling_rate(&self) -> Option<Decimal> {
        self.ceiling_rate
    }

    pub fn calculate_daily_interest(&self, principal: Decimal) -> Decimal {
        let daily_rate = self.annual_rate / Decimal::from(365);
        principal * daily_rate / Decimal::from(100)
    }

    pub fn calculate_compound_monthly(&self, principal: Decimal, months: u32) -> Decimal {
        let monthly_rate = self.annual_rate / Decimal::from(12) / Decimal::from(100);
        let compound_factor = (Decimal::ONE + monthly_rate).pow(months);
        principal * (compound_factor - Decimal::ONE)
    }
}

// ============================================================
// Fee struct
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    id: Uuid,
    fee_type: FeeType,
    fixed_amount: Option<Decimal>,
    rate: Option<Decimal>, // as percentage
    min_amount: Option<Decimal>,
    max_amount: Option<Decimal>,
    charged_on: Option<u8>, // day of month
}

impl Fee {
    pub fn new(
        fee_type: FeeType,
        fixed_amount: Option<Decimal>,
        rate: Option<Decimal>,
        min_amount: Option<Decimal>,
        max_amount: Option<Decimal>,
        charged_on: Option<u8>,
    ) -> Result<Self, String> {
        // At least one of fixed_amount or rate must be set
        if fixed_amount.is_none() && rate.is_none() {
            return Err("At least one of fixed_amount or rate must be set".to_string());
        }

        // If min_amount and max_amount are both set, validate min <= max
        if let (Some(min), Some(max)) = (min_amount, max_amount) {
            if min > max {
                return Err("Min amount cannot exceed max amount".to_string());
            }
        }

        Ok(Fee {
            id: Uuid::new_v4(),
            fee_type,
            fixed_amount,
            rate,
            min_amount,
            max_amount,
            charged_on,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn fee_type(&self) -> FeeType {
        self.fee_type
    }

    pub fn fixed_amount(&self) -> Option<Decimal> {
        self.fixed_amount
    }

    pub fn rate(&self) -> Option<Decimal> {
        self.rate
    }

    pub fn min_amount(&self) -> Option<Decimal> {
        self.min_amount
    }

    pub fn max_amount(&self) -> Option<Decimal> {
        self.max_amount
    }

    pub fn charged_on(&self) -> Option<u8> {
        self.charged_on
    }

    /// Calculate fee for a transaction amount
    pub fn calculate(&self, transaction_amount: Decimal) -> Decimal {
        let fee = if let Some(fixed) = self.fixed_amount {
            fixed
        } else if let Some(rate) = self.rate {
            transaction_amount * rate / Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        // Apply min/max bounds
        let fee = if let Some(min) = self.min_amount {
            if fee < min {
                min
            } else {
                fee
            }
        } else {
            fee
        };

        if let Some(max) = self.max_amount {
            if fee > max {
                max
            } else {
                fee
            }
        } else {
            fee
        }
    }
}

// ============================================================
// EligibilityRule struct
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityRule {
    min_age: Option<u8>,
    max_age: Option<u8>,
    min_income: Option<Decimal>,
    required_segment: Option<CustomerSegment>,
    min_credit_score: Option<u32>,
}

impl EligibilityRule {
    pub fn new(
        min_age: Option<u8>,
        max_age: Option<u8>,
        min_income: Option<Decimal>,
        required_segment: Option<CustomerSegment>,
        min_credit_score: Option<u32>,
    ) -> Self {
        EligibilityRule {
            min_age,
            max_age,
            min_income,
            required_segment,
            min_credit_score,
        }
    }

    pub fn min_age(&self) -> Option<u8> {
        self.min_age
    }

    pub fn max_age(&self) -> Option<u8> {
        self.max_age
    }

    pub fn min_income(&self) -> Option<Decimal> {
        self.min_income
    }

    pub fn required_segment(&self) -> Option<CustomerSegment> {
        self.required_segment
    }

    pub fn min_credit_score(&self) -> Option<u32> {
        self.min_credit_score
    }

    /// Evaluate eligibility and return list of failure reasons
    pub fn evaluate(
        &self,
        age: u8,
        income: Decimal,
        segment: &CustomerSegment,
        credit_score: u32,
    ) -> Result<(), Vec<String>> {
        let mut reasons = Vec::new();

        if let Some(min_age) = self.min_age {
            if age < min_age {
                reasons.push(format!("Minimum age requirement not met: {} years required", min_age));
            }
        }

        if let Some(max_age) = self.max_age {
            if age > max_age {
                reasons.push(format!("Maximum age exceeded: {} years maximum", max_age));
            }
        }

        if let Some(min_income) = self.min_income {
            if income < min_income {
                reasons.push(format!("Minimum income requirement not met: {} TND required", min_income));
            }
        }

        if let Some(required_segment) = self.required_segment {
            if *segment != required_segment {
                reasons.push(format!("Customer segment mismatch: {} required", required_segment.as_str()));
            }
        }

        if let Some(min_score) = self.min_credit_score {
            if credit_score < min_score {
                reasons.push(format!("Minimum credit score not met: {} required", min_score));
            }
        }

        if reasons.is_empty() {
            Ok(())
        } else {
            Err(reasons)
        }
    }
}

// ============================================================
// Product aggregate root
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    id: Uuid,
    name: String,
    product_type: ProductType,
    status: ProductStatus,
    interest_rate: Option<InterestRate>,
    fees: Vec<Fee>,
    eligibility: EligibilityRule,
    segment_pricing: HashMap<CustomerSegment, Decimal>,
    min_balance: Option<Decimal>,
    currency: String,
    version: u32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        name: String,
        product_type: ProductType,
        interest_rate: Option<InterestRate>,
        fees: Vec<Fee>,
        eligibility: EligibilityRule,
        segment_pricing: HashMap<CustomerSegment, Decimal>,
        min_balance: Option<Decimal>,
        currency: String,
    ) -> Result<Self, String> {
        // Validate name is not empty
        if name.trim().is_empty() {
            return Err("Product name cannot be empty".to_string());
        }

        // Validate currency (basic check for 3-letter code)
        if currency.len() != 3 {
            return Err("Currency must be a 3-letter code".to_string());
        }

        let now = Utc::now();

        Ok(Product {
            id: Uuid::new_v4(),
            name,
            product_type,
            status: ProductStatus::Draft,
            interest_rate,
            fees,
            eligibility,
            segment_pricing,
            min_balance,
            currency,
            version: 1,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: Uuid,
        name: String,
        product_type: ProductType,
        status: ProductStatus,
        interest_rate: Option<InterestRate>,
        fees: Vec<Fee>,
        eligibility: EligibilityRule,
        segment_pricing: HashMap<CustomerSegment, Decimal>,
        min_balance: Option<Decimal>,
        currency: String,
        version: u32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Product {
            id,
            name,
            product_type,
            status,
            interest_rate,
            fees,
            eligibility,
            segment_pricing,
            min_balance,
            currency,
            version,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn product_type(&self) -> ProductType {
        self.product_type
    }

    pub fn status(&self) -> ProductStatus {
        self.status
    }

    pub fn interest_rate(&self) -> Option<&InterestRate> {
        self.interest_rate.as_ref()
    }

    pub fn fees(&self) -> &[Fee] {
        &self.fees
    }

    pub fn eligibility(&self) -> &EligibilityRule {
        &self.eligibility
    }

    pub fn segment_pricing(&self) -> &HashMap<CustomerSegment, Decimal> {
        &self.segment_pricing
    }

    pub fn min_balance(&self) -> Option<Decimal> {
        self.min_balance
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    pub fn evaluate_eligibility(
        &self,
        age: u8,
        income: Decimal,
        segment: &CustomerSegment,
        credit_score: u32,
    ) -> Result<(), Vec<String>> {
        self.eligibility.evaluate(age, income, segment, credit_score)
    }

    pub fn get_rate_for_segment(&self, segment: &CustomerSegment) -> Option<Decimal> {
        if let Some(override_rate) = self.segment_pricing.get(segment) {
            Some(*override_rate)
        } else {
            self.interest_rate.as_ref().map(|ir| ir.annual_rate())
        }
    }

    pub fn calculate_total_fees(&self, transaction_amount: Decimal) -> Decimal {
        self.fees.iter()
            .map(|fee| fee.calculate(transaction_amount))
            .sum()
    }

    pub fn activate(&mut self) -> Result<(), String> {
        match self.status {
            ProductStatus::Draft | ProductStatus::Suspended => {
                self.status = ProductStatus::Active;
                self.updated_at = Utc::now();
                self.version += 1;
                Ok(())
            }
            ProductStatus::Discontinued => {
                Err("Cannot activate a discontinued product".to_string())
            }
            ProductStatus::Active => {
                Err("Product is already active".to_string())
            }
        }
    }

    pub fn suspend(&mut self) -> Result<(), String> {
        match self.status {
            ProductStatus::Active => {
                self.status = ProductStatus::Suspended;
                self.updated_at = Utc::now();
                self.version += 1;
                Ok(())
            }
            _ => Err("Only active products can be suspended".to_string()),
        }
    }

    pub fn discontinue(&mut self) -> Result<(), String> {
        match self.status {
            ProductStatus::Draft | ProductStatus::Active | ProductStatus::Suspended => {
                self.status = ProductStatus::Discontinued;
                self.updated_at = Utc::now();
                self.version += 1;
                Ok(())
            }
            ProductStatus::Discontinued => {
                Err("Product is already discontinued".to_string())
            }
        }
    }

    pub fn increment_version(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }
}

// ============================================================
// PricingBand struct
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingBand {
    id: Uuid,
    min_amount: Decimal,
    max_amount: Option<Decimal>,
    rate: Decimal,
    fees_override: Option<Decimal>,
    sort_order: u16,
}

impl PricingBand {
    pub fn new(
        min_amount: Decimal,
        max_amount: Option<Decimal>,
        rate: Decimal,
        fees_override: Option<Decimal>,
        sort_order: u16,
    ) -> Result<Self, String> {
        // Validate min < max if max is provided
        if let Some(max) = max_amount {
            if min_amount >= max {
                return Err("Min amount must be less than max amount".to_string());
            }
        }

        Ok(PricingBand {
            id: Uuid::new_v4(),
            min_amount,
            max_amount,
            rate,
            fees_override,
            sort_order,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn min_amount(&self) -> Decimal {
        self.min_amount
    }

    pub fn max_amount(&self) -> Option<Decimal> {
        self.max_amount
    }

    pub fn rate(&self) -> Decimal {
        self.rate
    }

    pub fn fees_override(&self) -> Option<Decimal> {
        self.fees_override
    }

    pub fn sort_order(&self) -> u16 {
        self.sort_order
    }

    pub fn matches_amount(&self, amount: Decimal) -> bool {
        if amount < self.min_amount {
            return false;
        }
        if let Some(max) = self.max_amount {
            if amount >= max {
                return false;
            }
        }
        true
    }
}

// ============================================================
// PricingGrid aggregate root
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingGrid {
    id: Uuid,
    product_id: Uuid,
    bands: Vec<PricingBand>,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    active: bool,
    created_by: Uuid,
    created_at: DateTime<Utc>,
}

impl PricingGrid {
    pub fn new(
        product_id: Uuid,
        bands: Vec<PricingBand>,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<Self, String> {
        // Validate bands not empty
        if bands.is_empty() {
            return Err("Pricing grid must have at least one band".to_string());
        }

        // Validate bands don't overlap
        let mut sorted_bands = bands.clone();
        sorted_bands.sort_by_key(|b| (b.min_amount.to_u32().unwrap_or(0), b.sort_order));

        for i in 0..sorted_bands.len() - 1 {
            let current_max = sorted_bands[i].max_amount;
            let next_min = sorted_bands[i + 1].min_amount;

            if let Some(max) = current_max {
                if max != next_min {
                    return Err("Pricing bands must not overlap or have gaps".to_string());
                }
            }
        }

        let now = Utc::now();

        Ok(PricingGrid {
            id: Uuid::new_v4(),
            product_id,
            bands,
            effective_from,
            effective_to,
            active: true,
            created_by,
            created_at: now,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn product_id(&self) -> Uuid {
        self.product_id
    }

    pub fn bands(&self) -> &[PricingBand] {
        &self.bands
    }

    pub fn effective_from(&self) -> DateTime<Utc> {
        self.effective_from
    }

    pub fn effective_to(&self) -> Option<DateTime<Utc>> {
        self.effective_to
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn created_by(&self) -> Uuid {
        self.created_by
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_rate_for_amount(&self, amount: Decimal) -> Option<Decimal> {
        self.bands
            .iter()
            .find(|band| band.matches_amount(amount))
            .map(|band| band.rate())
    }

    pub fn is_effective_at(&self, date: DateTime<Utc>) -> bool {
        if !self.active {
            return false;
        }

        if date < self.effective_from {
            return false;
        }

        if let Some(effective_to) = self.effective_to {
            if date >= effective_to {
                return false;
            }
        }

        true
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_basic_interest_rate() -> InterestRate {
        InterestRate::new(Decimal::from(5), CalcMethod::Simple, None, None).unwrap()
    }

    fn create_basic_product(name: &str) -> Product {
        Product::new(
            name.to_string(),
            ProductType::CurrentAccount,
            Some(create_basic_interest_rate()),
            vec![],
            EligibilityRule::new(None, None, None, None, None),
            HashMap::new(),
            None,
            "TND".to_string(),
        )
        .unwrap()
    }

    // --- InterestRate tests ---

    #[test]
    fn test_interest_rate_valid_creation() {
        let rate = InterestRate::new(Decimal::from(5), CalcMethod::Simple, None, None);
        assert!(rate.is_ok());
        assert_eq!(rate.unwrap().annual_rate(), Decimal::from(5));
    }

    #[test]
    fn test_interest_rate_negative_fails() {
        let rate = InterestRate::new(Decimal::from(-1), CalcMethod::Simple, None, None);
        assert!(rate.is_err());
    }

    #[test]
    fn test_interest_rate_exceeds_100_fails() {
        let rate = InterestRate::new(Decimal::from(101), CalcMethod::Simple, None, None);
        assert!(rate.is_err());
    }

    #[test]
    fn test_interest_rate_floor_exceeds_ceiling_fails() {
        let rate = InterestRate::new(
            Decimal::from(5),
            CalcMethod::Simple,
            Some(Decimal::from(6)),
            Some(Decimal::from(5)),
        );
        assert!(rate.is_err());
    }

    #[test]
    fn test_calculate_daily_interest() {
        let rate = InterestRate::new(Decimal::from(36), CalcMethod::Daily, None, None).unwrap();
        let daily = rate.calculate_daily_interest(Decimal::from(10000));
        // 10000 * 36 / 365 / 100 = ~9.86
        assert!(daily > Decimal::ZERO);
    }

    #[test]
    fn test_calculate_compound_monthly() {
        let rate = InterestRate::new(Decimal::from(12), CalcMethod::Compound, None, None).unwrap();
        let interest = rate.calculate_compound_monthly(Decimal::from(10000), 12);
        // Should be roughly 1268.25
        assert!(interest > Decimal::ZERO);
    }

    // --- Fee tests ---

    #[test]
    fn test_fee_fixed_amount_creation() {
        let fee = Fee::new(
            FeeType::Monthly,
            Some(Decimal::from(50)),
            None,
            None,
            None,
            None,
        );
        assert!(fee.is_ok());
    }

    #[test]
    fn test_fee_rate_creation() {
        let fee = Fee::new(
            FeeType::Transaction,
            None,
            Some(Decimal::from(1)),
            None,
            None,
            None,
        );
        assert!(fee.is_ok());
    }

    #[test]
    fn test_fee_no_amount_or_rate_fails() {
        let fee = Fee::new(FeeType::Setup, None, None, None, None, None);
        assert!(fee.is_err());
    }

    #[test]
    fn test_fee_calculate_fixed_amount() {
        let fee = Fee::new(
            FeeType::Monthly,
            Some(Decimal::from(50)),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let calculated = fee.calculate(Decimal::from(1000));
        assert_eq!(calculated, Decimal::from(50));
    }

    #[test]
    fn test_fee_calculate_rate_based() {
        let fee = Fee::new(
            FeeType::Transaction,
            None,
            Some(Decimal::from(1)),
            None,
            None,
            None,
        )
        .unwrap();
        let calculated = fee.calculate(Decimal::from(1000));
        assert_eq!(calculated, Decimal::from(10)); // 1000 * 1 / 100
    }

    #[test]
    fn test_fee_calculate_with_min_bound() {
        let fee = Fee::new(
            FeeType::Transaction,
            None,
            Some(Decimal::from(1)),
            Some(Decimal::from(20)),
            None,
            None,
        )
        .unwrap();
        let calculated = fee.calculate(Decimal::from(100)); // Would be 1, but min is 20
        assert_eq!(calculated, Decimal::from(20));
    }

    #[test]
    fn test_fee_calculate_with_max_bound() {
        let fee = Fee::new(
            FeeType::Transaction,
            None,
            Some(Decimal::from(1)),
            None,
            Some(Decimal::from(50)),
            None,
        )
        .unwrap();
        let calculated = fee.calculate(Decimal::from(10000)); // Would be 100, but max is 50
        assert_eq!(calculated, Decimal::from(50));
    }

    // --- EligibilityRule tests ---

    #[test]
    fn test_eligibility_no_rules_passes() {
        let rule = EligibilityRule::new(None, None, None, None, None);
        let result = rule.evaluate(30, Decimal::from(50000), &CustomerSegment::Standard, 750);
        assert!(result.is_ok());
    }

    #[test]
    fn test_eligibility_age_too_low_fails() {
        let rule = EligibilityRule::new(Some(18), None, None, None, None);
        let result = rule.evaluate(15, Decimal::from(50000), &CustomerSegment::Standard, 750);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].contains("Minimum age requirement"));
    }

    #[test]
    fn test_eligibility_age_too_high_fails() {
        let rule = EligibilityRule::new(None, Some(65), None, None, None);
        let result = rule.evaluate(70, Decimal::from(50000), &CustomerSegment::Standard, 750);
        assert!(result.is_err());
    }

    #[test]
    fn test_eligibility_income_too_low_fails() {
        let rule = EligibilityRule::new(None, None, Some(Decimal::from(30000)), None, None);
        let result = rule.evaluate(30, Decimal::from(20000), &CustomerSegment::Standard, 750);
        assert!(result.is_err());
    }

    #[test]
    fn test_eligibility_segment_mismatch_fails() {
        let rule = EligibilityRule::new(None, None, None, Some(CustomerSegment::Premium), None);
        let result = rule.evaluate(30, Decimal::from(50000), &CustomerSegment::Standard, 750);
        assert!(result.is_err());
    }

    #[test]
    fn test_eligibility_credit_score_too_low_fails() {
        let rule = EligibilityRule::new(None, None, None, None, Some(700));
        let result = rule.evaluate(30, Decimal::from(50000), &CustomerSegment::Standard, 600);
        assert!(result.is_err());
    }

    #[test]
    fn test_eligibility_multiple_failures() {
        let rule = EligibilityRule::new(
            Some(25),
            Some(60),
            Some(Decimal::from(30000)),
            Some(CustomerSegment::Premium),
            Some(700),
        );
        let result = rule.evaluate(20, Decimal::from(20000), &CustomerSegment::Standard, 600);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2);
    }

    // --- Product tests ---

    #[test]
    fn test_product_creation() {
        let product = create_basic_product("Test Account");
        assert_eq!(product.name(), "Test Account");
        assert_eq!(product.status(), ProductStatus::Draft);
        assert_eq!(product.version(), 1);
    }

    #[test]
    fn test_product_empty_name_fails() {
        let result = Product::new(
            "".to_string(),
            ProductType::SavingsAccount,
            None,
            vec![],
            EligibilityRule::new(None, None, None, None, None),
            HashMap::new(),
            None,
            "TND".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_product_invalid_currency_fails() {
        let result = Product::new(
            "Test".to_string(),
            ProductType::SavingsAccount,
            None,
            vec![],
            EligibilityRule::new(None, None, None, None, None),
            HashMap::new(),
            None,
            "INVALID".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_product_activate_from_draft() {
        let mut product = create_basic_product("Account");
        let result = product.activate();
        assert!(result.is_ok());
        assert_eq!(product.status(), ProductStatus::Active);
        assert_eq!(product.version(), 2);
    }

    #[test]
    fn test_product_suspend_active() {
        let mut product = create_basic_product("Account");
        product.activate().unwrap();
        let result = product.suspend();
        assert!(result.is_ok());
        assert_eq!(product.status(), ProductStatus::Suspended);
    }

    #[test]
    fn test_product_discontinue() {
        let mut product = create_basic_product("Account");
        let result = product.discontinue();
        assert!(result.is_ok());
        assert_eq!(product.status(), ProductStatus::Discontinued);
    }

    #[test]
    fn test_product_activate_discontinued_fails() {
        let mut product = create_basic_product("Account");
        product.discontinue().unwrap();
        let result = product.activate();
        assert!(result.is_err());
    }

    #[test]
    fn test_product_get_rate_for_segment_uses_override() {
        let mut segment_pricing = HashMap::new();
        segment_pricing.insert(CustomerSegment::Premium, Decimal::from(3));
        let product = Product::new(
            "Test".to_string(),
            ProductType::SavingsAccount,
            Some(create_basic_interest_rate()),
            vec![],
            EligibilityRule::new(None, None, None, None, None),
            segment_pricing,
            None,
            "TND".to_string(),
        )
        .unwrap();
        let rate = product.get_rate_for_segment(&CustomerSegment::Premium);
        assert_eq!(rate, Some(Decimal::from(3)));
    }

    #[test]
    fn test_product_get_rate_for_segment_uses_default() {
        let product = create_basic_product("Account");
        let rate = product.get_rate_for_segment(&CustomerSegment::Standard);
        assert_eq!(rate, Some(Decimal::from(5)));
    }

    #[test]
    fn test_product_calculate_total_fees() {
        let fee1 = Fee::new(FeeType::Setup, Some(Decimal::from(50)), None, None, None, None).unwrap();
        let fee2 = Fee::new(
            FeeType::Transaction,
            None,
            Some(Decimal::from(1)),
            None,
            None,
            None,
        )
        .unwrap();
        let product = Product::new(
            "Account".to_string(),
            ProductType::CurrentAccount,
            None,
            vec![fee1, fee2],
            EligibilityRule::new(None, None, None, None, None),
            HashMap::new(),
            None,
            "TND".to_string(),
        )
        .unwrap();
        let total = product.calculate_total_fees(Decimal::from(1000));
        // Setup: 50, Transaction: 10 = 60 total
        assert_eq!(total, Decimal::from(60));
    }

    #[test]
    fn test_product_evaluate_eligibility() {
        let rule = EligibilityRule::new(Some(18), None, None, None, None);
        let product = Product::new(
            "Account".to_string(),
            ProductType::CurrentAccount,
            None,
            vec![],
            rule,
            HashMap::new(),
            None,
            "TND".to_string(),
        )
        .unwrap();
        let result = product.evaluate_eligibility(20, Decimal::from(50000), &CustomerSegment::Standard, 750);
        assert!(result.is_ok());
    }

    #[test]
    fn test_product_increment_version() {
        let mut product = create_basic_product("Account");
        let initial_version = product.version();
        product.increment_version();
        assert_eq!(product.version(), initial_version + 1);
    }

    // --- PricingBand tests ---

    #[test]
    fn test_pricing_band_creation() {
        let band = PricingBand::new(
            Decimal::from(0),
            Some(Decimal::from(1000)),
            Decimal::from(5),
            None,
            0,
        );
        assert!(band.is_ok());
    }

    #[test]
    fn test_pricing_band_invalid_range_fails() {
        let band = PricingBand::new(
            Decimal::from(1000),
            Some(Decimal::from(500)),
            Decimal::from(5),
            None,
            0,
        );
        assert!(band.is_err());
    }

    #[test]
    fn test_pricing_band_matches_amount() {
        let band = PricingBand::new(
            Decimal::from(0),
            Some(Decimal::from(1000)),
            Decimal::from(5),
            None,
            0,
        )
        .unwrap();
        assert!(band.matches_amount(Decimal::from(500)));
        assert!(!band.matches_amount(Decimal::from(1000)));
        assert!(!band.matches_amount(Decimal::from(-1)));
    }

    // --- PricingGrid tests ---

    #[test]
    fn test_pricing_grid_creation() {
        let band = PricingBand::new(
            Decimal::from(0),
            None,
            Decimal::from(5),
            None,
            0,
        )
        .unwrap();
        let now = Utc::now();
        let grid = PricingGrid::new(
            Uuid::new_v4(),
            vec![band],
            now,
            None,
            Uuid::new_v4(),
        );
        assert!(grid.is_ok());
    }

    #[test]
    fn test_pricing_grid_empty_bands_fails() {
        let now = Utc::now();
        let grid = PricingGrid::new(Uuid::new_v4(), vec![], now, None, Uuid::new_v4());
        assert!(grid.is_err());
    }

    #[test]
    fn test_pricing_grid_get_rate_for_amount() {
        let band = PricingBand::new(
            Decimal::from(0),
            Some(Decimal::from(1000)),
            Decimal::from(5),
            None,
            0,
        )
        .unwrap();
        let now = Utc::now();
        let grid = PricingGrid::new(
            Uuid::new_v4(),
            vec![band],
            now,
            None,
            Uuid::new_v4(),
        )
        .unwrap();
        let rate = grid.get_rate_for_amount(Decimal::from(500));
        assert_eq!(rate, Some(Decimal::from(5)));
    }

    #[test]
    fn test_pricing_grid_is_effective_at() {
        let band = PricingBand::new(Decimal::from(0), None, Decimal::from(5), None, 0).unwrap();
        let now = Utc::now();
        let grid = PricingGrid::new(
            Uuid::new_v4(),
            vec![band],
            now,
            None,
            Uuid::new_v4(),
        )
        .unwrap();
        assert!(grid.is_effective_at(now));
        assert!(!grid.is_effective_at(now - chrono::Duration::days(1)));
    }
}
