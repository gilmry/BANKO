use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::shared::errors::DomainError;

// ==================== Enums ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeeCategory {
    MonthlyAccountFee,
    TransactionFee,
    SetupFee,
    EarlyWithdrawalFee,
    ConversionFee,
    PenaltyFee,
    OverdraftFee,
    ChequeBookFee,
    CardFee,
    TransferFee,
}

impl FeeCategory {
    pub fn as_str(&self) -> &str {
        match self {
            FeeCategory::MonthlyAccountFee => "MonthlyAccountFee",
            FeeCategory::TransactionFee => "TransactionFee",
            FeeCategory::SetupFee => "SetupFee",
            FeeCategory::EarlyWithdrawalFee => "EarlyWithdrawalFee",
            FeeCategory::ConversionFee => "ConversionFee",
            FeeCategory::PenaltyFee => "PenaltyFee",
            FeeCategory::OverdraftFee => "OverdraftFee",
            FeeCategory::ChequeBookFee => "ChequeBookFee",
            FeeCategory::CardFee => "CardFee",
            FeeCategory::TransferFee => "TransferFee",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "MonthlyAccountFee" => Some(FeeCategory::MonthlyAccountFee),
            "TransactionFee" => Some(FeeCategory::TransactionFee),
            "SetupFee" => Some(FeeCategory::SetupFee),
            "EarlyWithdrawalFee" => Some(FeeCategory::EarlyWithdrawalFee),
            "ConversionFee" => Some(FeeCategory::ConversionFee),
            "PenaltyFee" => Some(FeeCategory::PenaltyFee),
            "OverdraftFee" => Some(FeeCategory::OverdraftFee),
            "ChequeBookFee" => Some(FeeCategory::ChequeBookFee),
            "CardFee" => Some(FeeCategory::CardFee),
            "TransferFee" => Some(FeeCategory::TransferFee),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeeCondition {
    Always,
    BalanceBelow(Decimal),
    TransactionAbove(Decimal),
    MonthDay(u8),
    EndOfMonth,
    OnEvent(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeeStatus {
    Pending,
    Charged,
    Unpaid,
    Waived,
    Reversed,
}

impl FeeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FeeStatus::Pending => "Pending",
            FeeStatus::Charged => "Charged",
            FeeStatus::Unpaid => "Unpaid",
            FeeStatus::Waived => "Waived",
            FeeStatus::Reversed => "Reversed",
        }
    }
}

// ==================== FeeDefinition ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeDefinition {
    id: Uuid,
    name: String,
    category: FeeCategory,
    fixed_amount: Option<Decimal>,
    rate_percent: Option<Decimal>,
    min_amount: Option<Decimal>,
    max_amount: Option<Decimal>,
    condition: FeeCondition,
    applicable_segments: Vec<String>,
    currency: String,
}

impl FeeDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        category: FeeCategory,
        fixed_amount: Option<Decimal>,
        rate_percent: Option<Decimal>,
        min_amount: Option<Decimal>,
        max_amount: Option<Decimal>,
        condition: FeeCondition,
        applicable_segments: Vec<String>,
        currency: String,
    ) -> Result<Self, DomainError> {
        // At least one of fixed_amount or rate_percent must be specified
        if fixed_amount.is_none() && rate_percent.is_none() {
            return Err(DomainError::InvalidMovement(
                "Either fixed_amount or rate_percent must be specified".to_string(),
            ));
        }

        // Validate rate_percent if specified
        if let Some(rate) = rate_percent {
            if rate < Decimal::ZERO {
                return Err(DomainError::InvalidMovement(
                    "rate_percent cannot be negative".to_string(),
                ));
            }
        }

        // Validate fixed_amount if specified
        if let Some(fixed) = fixed_amount {
            if fixed < Decimal::ZERO {
                return Err(DomainError::InvalidMovement(
                    "fixed_amount cannot be negative".to_string(),
                ));
            }
        }

        Ok(FeeDefinition {
            id: Uuid::new_v4(),
            name,
            category,
            fixed_amount,
            rate_percent,
            min_amount,
            max_amount,
            condition,
            applicable_segments,
            currency,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn category(&self) -> FeeCategory {
        self.category
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn condition(&self) -> &FeeCondition {
        &self.condition
    }

    /// Calculate the fee amount based on a transaction amount
    pub fn calculate(&self, transaction_amount: Decimal) -> Decimal {
        let calculated = match self.rate_percent {
            Some(rate) => transaction_amount * rate / Decimal::from(100),
            None => Decimal::ZERO,
        };

        let final_amount = match self.fixed_amount {
            Some(fixed) => {
                if self.rate_percent.is_some() {
                    // Both specified: use max
                    calculated.max(fixed)
                } else {
                    fixed
                }
            }
            None => calculated,
        };

        // Apply min/max bounds
        let mut result = final_amount;

        if let Some(min) = self.min_amount {
            result = result.max(min);
        }

        if let Some(max) = self.max_amount {
            result = result.min(max);
        }

        result
    }

    /// Check if this fee applies to a specific customer segment
    pub fn applies_to_segment(&self, segment: &str) -> bool {
        self.applicable_segments.is_empty() || self.applicable_segments.contains(&segment.to_string())
    }

    /// Check if the condition is met given the current state
    pub fn is_condition_met(
        &self,
        balance: Decimal,
        transaction_amount: Decimal,
        day_of_month: u8,
    ) -> bool {
        match &self.condition {
            FeeCondition::Always => true,
            FeeCondition::BalanceBelow(threshold) => balance < *threshold,
            FeeCondition::TransactionAbove(threshold) => transaction_amount >= *threshold,
            FeeCondition::MonthDay(target_day) => day_of_month == *target_day,
            FeeCondition::EndOfMonth => day_of_month >= 28, // Simplified: 28+ is end of month
            FeeCondition::OnEvent(_) => false, // Event-based, must be explicitly checked
        }
    }
}

// ==================== FeeCharge ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeCharge {
    id: Uuid,
    fee_definition_id: Uuid,
    account_id: Uuid,
    amount: Decimal,
    status: FeeStatus,
    charged_at: DateTime<Utc>,
    description: Option<String>,
}

impl FeeCharge {
    pub fn new(
        fee_definition_id: Uuid,
        account_id: Uuid,
        amount: Decimal,
        description: Option<String>,
    ) -> Result<Self, DomainError> {
        if amount < Decimal::ZERO {
            return Err(DomainError::InvalidMovement(
                "Fee amount cannot be negative".to_string(),
            ));
        }

        Ok(FeeCharge {
            id: Uuid::new_v4(),
            fee_definition_id,
            account_id,
            amount,
            status: FeeStatus::Pending,
            charged_at: Utc::now(),
            description,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn fee_definition_id(&self) -> Uuid {
        self.fee_definition_id
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn status(&self) -> FeeStatus {
        self.status
    }

    pub fn charged_at(&self) -> DateTime<Utc> {
        self.charged_at
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn mark_charged(&mut self) {
        self.status = FeeStatus::Charged;
    }

    pub fn mark_unpaid(&mut self) {
        self.status = FeeStatus::Unpaid;
    }

    pub fn mark_waived(&mut self) {
        self.status = FeeStatus::Waived;
    }

    pub fn mark_reversed(&mut self) {
        self.status = FeeStatus::Reversed;
    }
}

// ==================== FeeGrid ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeGrid {
    id: Uuid,
    name: String,
    segment: String,
    fee_overrides: HashMap<FeeCategory, Decimal>,
    effective_from: DateTime<Utc>,
    effective_to: Option<DateTime<Utc>>,
    active: bool,
}

impl FeeGrid {
    pub fn new(
        name: String,
        segment: String,
        fee_overrides: HashMap<FeeCategory, Decimal>,
        effective_from: DateTime<Utc>,
        effective_to: Option<DateTime<Utc>>,
    ) -> Self {
        FeeGrid {
            id: Uuid::new_v4(),
            name,
            segment,
            fee_overrides,
            effective_from,
            effective_to,
            active: true,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn segment(&self) -> &str {
        &self.segment
    }

    pub fn fee_overrides(&self) -> &HashMap<FeeCategory, Decimal> {
        &self.fee_overrides
    }

    pub fn active(&self) -> bool {
        self.active
    }

    /// Get fee override for a specific category, if exists
    pub fn get_fee_for_category(&self, category: &FeeCategory) -> Option<Decimal> {
        self.fee_overrides.get(category).copied()
    }

    /// Check if this grid is effective at a given date
    pub fn is_effective_at(&self, date: DateTime<Utc>) -> bool {
        self.active
            && date >= self.effective_from
            && self
                .effective_to
                .map(|end| date <= end)
                .unwrap_or(true)
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_category_as_str() {
        assert_eq!(FeeCategory::MonthlyAccountFee.as_str(), "MonthlyAccountFee");
        assert_eq!(FeeCategory::TransactionFee.as_str(), "TransactionFee");
    }

    #[test]
    fn test_fee_category_from_str() {
        assert_eq!(
            FeeCategory::from_str("MonthlyAccountFee"),
            Some(FeeCategory::MonthlyAccountFee)
        );
        assert_eq!(FeeCategory::from_str("INVALID"), None);
    }

    #[test]
    fn test_fee_status_as_str() {
        assert_eq!(FeeStatus::Pending.as_str(), "Pending");
        assert_eq!(FeeStatus::Charged.as_str(), "Charged");
    }

    #[test]
    fn test_fee_definition_new() {
        let def = FeeDefinition::new(
            "Account Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(10)),
            None,
            None,
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        assert_eq!(def.name(), "Account Fee");
        assert_eq!(def.category(), FeeCategory::MonthlyAccountFee);
    }

    #[test]
    fn test_fee_definition_no_amount_fails() {
        let result = FeeDefinition::new(
            "Bad Fee".to_string(),
            FeeCategory::TransactionFee,
            None,
            None,
            None,
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fee_definition_fixed_amount_calculation() {
        let def = FeeDefinition::new(
            "Fixed Fee".to_string(),
            FeeCategory::TransactionFee,
            Some(Decimal::from(5)),
            None,
            None,
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        let fee = def.calculate(Decimal::from(1000));
        assert_eq!(fee, Decimal::from(5));
    }

    #[test]
    fn test_fee_definition_rate_calculation() {
        let def = FeeDefinition::new(
            "Rate Fee".to_string(),
            FeeCategory::TransactionFee,
            None,
            Some(Decimal::from(1)), // 1% rate
            None,
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        let fee = def.calculate(Decimal::from(1000));
        // 1000 * 1% = 10
        assert_eq!(fee, Decimal::from(10));
    }

    #[test]
    fn test_fee_definition_rate_with_min_bound() {
        let def = FeeDefinition::new(
            "Fee with Min".to_string(),
            FeeCategory::TransactionFee,
            None,
            Some(Decimal::from(1)), // 1% rate
            Some(Decimal::from(20)), // min 20
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        // 100 * 1% = 1, but min is 20, so result is 20
        let fee = def.calculate(Decimal::from(100));
        assert_eq!(fee, Decimal::from(20));
    }

    #[test]
    fn test_fee_definition_rate_with_max_bound() {
        let def = FeeDefinition::new(
            "Fee with Max".to_string(),
            FeeCategory::TransactionFee,
            None,
            Some(Decimal::from(10)), // 10% rate
            None,
            Some(Decimal::from(50)), // max 50
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        // 1000 * 10% = 100, but max is 50, so result is 50
        let fee = def.calculate(Decimal::from(1000));
        assert_eq!(fee, Decimal::from(50));
    }

    #[test]
    fn test_fee_definition_fixed_and_rate_max() {
        let def = FeeDefinition::new(
            "Fee with Both".to_string(),
            FeeCategory::TransactionFee,
            Some(Decimal::from(15)),
            Some(Decimal::from(2)), // 2% rate
            None,
            None,
            FeeCondition::Always,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        // 100 * 2% = 2, fixed is 15, so result is max(2, 15) = 15
        let fee = def.calculate(Decimal::from(100));
        assert_eq!(fee, Decimal::from(15));

        // 1000 * 2% = 20, fixed is 15, so result is max(20, 15) = 20
        let fee = def.calculate(Decimal::from(1000));
        assert_eq!(fee, Decimal::from(20));
    }

    #[test]
    fn test_fee_definition_applies_to_segment_empty_list() {
        let def = FeeDefinition::new(
            "General Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(10)),
            None,
            None,
            None,
            FeeCondition::Always,
            vec![], // empty = applies to all
            "TND".to_string(),
        )
        .unwrap();

        assert!(def.applies_to_segment("VIP"));
        assert!(def.applies_to_segment("Standard"));
        assert!(def.applies_to_segment("Junior"));
    }

    #[test]
    fn test_fee_definition_applies_to_segment_specific() {
        let def = FeeDefinition::new(
            "VIP Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(5)),
            None,
            None,
            None,
            FeeCondition::Always,
            vec!["VIP".to_string()],
            "TND".to_string(),
        )
        .unwrap();

        assert!(def.applies_to_segment("VIP"));
        assert!(!def.applies_to_segment("Standard"));
    }

    #[test]
    fn test_fee_condition_always() {
        let cond = FeeCondition::Always;
        assert!(FeeDefinition::new(
            "Test".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(10)),
            None,
            None,
            None,
            cond.clone(),
            vec![],
            "TND".to_string()
        )
        .unwrap()
        .is_condition_met(Decimal::from(1000), Decimal::from(100), 15));
    }

    #[test]
    fn test_fee_condition_balance_below() {
        let cond = FeeCondition::BalanceBelow(Decimal::from(100));
        let def = FeeDefinition::new(
            "Low Balance Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(10)),
            None,
            None,
            None,
            cond,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        assert!(def.is_condition_met(Decimal::from(50), Decimal::from(100), 15));
        assert!(!def.is_condition_met(Decimal::from(150), Decimal::from(100), 15));
    }

    #[test]
    fn test_fee_condition_transaction_above() {
        let cond = FeeCondition::TransactionAbove(Decimal::from(1000));
        let def = FeeDefinition::new(
            "Large Transfer Fee".to_string(),
            FeeCategory::TransferFee,
            Some(Decimal::from(25)),
            None,
            None,
            None,
            cond,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        assert!(!def.is_condition_met(Decimal::from(500), Decimal::from(500), 15));
        assert!(def.is_condition_met(Decimal::from(500), Decimal::from(1000), 15));
        assert!(def.is_condition_met(Decimal::from(500), Decimal::from(2000), 15));
    }

    #[test]
    fn test_fee_condition_month_day() {
        let cond = FeeCondition::MonthDay(15);
        let def = FeeDefinition::new(
            "Mid-Month Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(10)),
            None,
            None,
            None,
            cond,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        assert!(def.is_condition_met(Decimal::from(500), Decimal::from(100), 15));
        assert!(!def.is_condition_met(Decimal::from(500), Decimal::from(100), 16));
    }

    #[test]
    fn test_fee_condition_end_of_month() {
        let cond = FeeCondition::EndOfMonth;
        let def = FeeDefinition::new(
            "End Month Fee".to_string(),
            FeeCategory::MonthlyAccountFee,
            Some(Decimal::from(15)),
            None,
            None,
            None,
            cond,
            vec![],
            "TND".to_string(),
        )
        .unwrap();

        assert!(!def.is_condition_met(Decimal::from(500), Decimal::from(100), 15));
        assert!(def.is_condition_met(Decimal::from(500), Decimal::from(100), 28));
        assert!(def.is_condition_met(Decimal::from(500), Decimal::from(100), 29));
    }

    #[test]
    fn test_fee_charge_new() {
        let fee_def_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let charge = FeeCharge::new(
            fee_def_id,
            account_id,
            Decimal::from(10),
            Some("Monthly account fee".to_string()),
        )
        .unwrap();

        assert_eq!(charge.fee_definition_id(), fee_def_id);
        assert_eq!(charge.account_id(), account_id);
        assert_eq!(charge.amount(), Decimal::from(10));
        assert_eq!(charge.status(), FeeStatus::Pending);
    }

    #[test]
    fn test_fee_charge_negative_amount_fails() {
        let result = FeeCharge::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Decimal::from(-10),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fee_charge_status_transitions() {
        let mut charge = FeeCharge::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Decimal::from(10),
            None,
        )
        .unwrap();

        assert_eq!(charge.status(), FeeStatus::Pending);

        charge.mark_charged();
        assert_eq!(charge.status(), FeeStatus::Charged);

        charge.mark_waived();
        assert_eq!(charge.status(), FeeStatus::Waived);

        charge.mark_reversed();
        assert_eq!(charge.status(), FeeStatus::Reversed);
    }

    #[test]
    fn test_fee_grid_new() {
        let mut overrides = HashMap::new();
        overrides.insert(FeeCategory::MonthlyAccountFee, Decimal::from(8));
        overrides.insert(FeeCategory::TransactionFee, Decimal::from(2));

        let grid = FeeGrid::new(
            "VIP Grid".to_string(),
            "VIP".to_string(),
            overrides,
            Utc::now(),
            None,
        );

        assert_eq!(grid.name(), "VIP Grid");
        assert_eq!(grid.segment(), "VIP");
        assert!(grid.active());
    }

    #[test]
    fn test_fee_grid_get_fee_for_category() {
        let mut overrides = HashMap::new();
        overrides.insert(FeeCategory::MonthlyAccountFee, Decimal::from(8));

        let grid = FeeGrid::new(
            "Test Grid".to_string(),
            "Test".to_string(),
            overrides,
            Utc::now(),
            None,
        );

        assert_eq!(
            grid.get_fee_for_category(&FeeCategory::MonthlyAccountFee),
            Some(Decimal::from(8))
        );
        assert_eq!(
            grid.get_fee_for_category(&FeeCategory::TransactionFee),
            None
        );
    }

    #[test]
    fn test_fee_grid_is_effective_at_no_end() {
        let now = Utc::now();
        let grid = FeeGrid::new(
            "Active Grid".to_string(),
            "Standard".to_string(),
            HashMap::new(),
            now,
            None,
        );

        assert!(grid.is_effective_at(now));
        assert!(grid.is_effective_at(now + chrono::Duration::days(30)));
    }

    #[test]
    fn test_fee_grid_is_effective_at_with_end() {
        let now = Utc::now();
        let end = now + chrono::Duration::days(30);
        let grid = FeeGrid::new(
            "Limited Grid".to_string(),
            "Standard".to_string(),
            HashMap::new(),
            now,
            Some(end),
        );

        assert!(grid.is_effective_at(now));
        assert!(grid.is_effective_at(end));
        assert!(!grid.is_effective_at(end + chrono::Duration::days(1)));
    }

    #[test]
    fn test_fee_grid_deactivate() {
        let mut grid = FeeGrid::new(
            "Grid".to_string(),
            "Test".to_string(),
            HashMap::new(),
            Utc::now(),
            None,
        );

        assert!(grid.active());
        grid.deactivate();
        assert!(!grid.active());
        assert!(!grid.is_effective_at(Utc::now()));
    }
}
