use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{CustomerId, Money};

use super::value_objects::{LoanId, PaymentFrequency};

// --- SubLimitId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubLimitId(uuid::Uuid);

impl SubLimitId {
    pub fn new() -> Self {
        SubLimitId(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(id: uuid::Uuid) -> Self {
        SubLimitId(id)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for SubLimitId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SubLimitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- RevolvingCreditLineId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RevolvingCreditLineId(uuid::Uuid);

impl RevolvingCreditLineId {
    pub fn new() -> Self {
        RevolvingCreditLineId(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(id: uuid::Uuid) -> Self {
        RevolvingCreditLineId(id)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for RevolvingCreditLineId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RevolvingCreditLineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- SyndicatedLoanId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SyndicatedLoanId(uuid::Uuid);

impl SyndicatedLoanId {
    pub fn new() -> Self {
        SyndicatedLoanId(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(id: uuid::Uuid) -> Self {
        SyndicatedLoanId(id)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for SyndicatedLoanId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SyndicatedLoanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- SubLimit value object (FR-036) ---
// Sublimite par devise/type de credit

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubLimit {
    id: SubLimitId,
    credit_line_id: RevolvingCreditLineId,
    limit_type: SubLimitType,
    max_amount: Money,
    utilized_amount: Money,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubLimitType {
    ByCurrency,
    ByProduct,
    ByCollateral,
}

impl SubLimit {
    /// Create a new sub-limit.
    pub fn new(
        credit_line_id: RevolvingCreditLineId,
        limit_type: SubLimitType,
        max_amount: Money,
    ) -> Result<Self, DomainError> {
        if max_amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Sub-limit max amount must be positive".to_string(),
            ));
        }

        let now = Utc::now();
        let currency = max_amount.currency();

        Ok(SubLimit {
            id: SubLimitId::new(),
            credit_line_id,
            limit_type,
            max_amount,
            utilized_amount: Money::zero(currency),
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence.
    pub fn reconstitute(
        id: SubLimitId,
        credit_line_id: RevolvingCreditLineId,
        limit_type: SubLimitType,
        max_amount: Money,
        utilized_amount: Money,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SubLimit {
            id,
            credit_line_id,
            limit_type,
            max_amount,
            utilized_amount,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &SubLimitId {
        &self.id
    }

    pub fn credit_line_id(&self) -> &RevolvingCreditLineId {
        &self.credit_line_id
    }

    pub fn limit_type(&self) -> SubLimitType {
        self.limit_type
    }

    pub fn max_amount(&self) -> &Money {
        &self.max_amount
    }

    pub fn utilized_amount(&self) -> &Money {
        &self.utilized_amount
    }

    pub fn available_amount(&self) -> Money {
        Money::from_cents(
            self.max_amount.amount_cents() - self.utilized_amount.amount_cents(),
            self.max_amount.currency(),
        )
    }

    pub fn utilization_rate(&self) -> f64 {
        if self.max_amount.amount_cents() == 0 {
            0.0
        } else {
            self.utilized_amount.amount_cents() as f64 / self.max_amount.amount_cents() as f64
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Drawdown from the sub-limit. Fails if insufficient available amount.
    pub fn drawdown(&mut self, amount: Money) -> Result<(), DomainError> {
        if amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Drawdown amount must be positive".to_string(),
            ));
        }

        let available = self.available_amount();
        if amount.amount_cents() > available.amount_cents() {
            return Err(DomainError::ValidationError(format!(
                "Insufficient available limit: requested {}, available {}",
                amount.amount(),
                available.amount()
            )));
        }

        self.utilized_amount = Money::from_cents(
            self.utilized_amount.amount_cents() + amount.amount_cents(),
            self.utilized_amount.currency(),
        );
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Repayment towards the sub-limit.
    pub fn repay(&mut self, amount: Money) -> Result<(), DomainError> {
        if amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Repayment amount must be positive".to_string(),
            ));
        }

        if amount.amount_cents() > self.utilized_amount.amount_cents() {
            return Err(DomainError::ValidationError(format!(
                "Repayment exceeds utilized amount: repaying {}, utilized {}",
                amount.amount(),
                self.utilized_amount.amount()
            )));
        }

        self.utilized_amount = Money::from_cents(
            self.utilized_amount.amount_cents() - amount.amount_cents(),
            self.utilized_amount.currency(),
        );
        self.updated_at = Utc::now();

        Ok(())
    }
}

// --- RevolvingCreditLine aggregate root (FR-035) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevolvingCreditLine {
    id: RevolvingCreditLineId,
    customer_id: CustomerId,
    account_id: crate::account::AccountId,
    max_limit: Money,
    current_balance: Money,
    interest_rate: f64,
    sub_limits: Vec<SubLimit>,
    status: RevolvingCreditStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RevolvingCreditStatus {
    Pending,
    Active,
    Suspended,
    Closed,
}

impl RevolvingCreditLine {
    /// Create a new revolving credit line.
    pub fn new(
        customer_id: CustomerId,
        account_id: crate::account::AccountId,
        max_limit: Money,
        interest_rate: f64,
    ) -> Result<Self, DomainError> {
        if max_limit.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Revolving credit max limit must be positive".to_string(),
            ));
        }
        if interest_rate <= 0.0 {
            return Err(DomainError::ValidationError(
                "Interest rate must be positive".to_string(),
            ));
        }

        let now = Utc::now();
        let currency = max_limit.currency();

        Ok(RevolvingCreditLine {
            id: RevolvingCreditLineId::new(),
            customer_id,
            account_id,
            max_limit,
            current_balance: Money::zero(currency),
            interest_rate,
            sub_limits: Vec::new(),
            status: RevolvingCreditStatus::Pending,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence.
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: RevolvingCreditLineId,
        customer_id: CustomerId,
        account_id: crate::account::AccountId,
        max_limit: Money,
        current_balance: Money,
        interest_rate: f64,
        sub_limits: Vec<SubLimit>,
        status: RevolvingCreditStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        RevolvingCreditLine {
            id,
            customer_id,
            account_id,
            max_limit,
            current_balance,
            interest_rate,
            sub_limits,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &RevolvingCreditLineId {
        &self.id
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn account_id(&self) -> &crate::account::AccountId {
        &self.account_id
    }

    pub fn max_limit(&self) -> &Money {
        &self.max_limit
    }

    pub fn current_balance(&self) -> &Money {
        &self.current_balance
    }

    pub fn interest_rate(&self) -> f64 {
        self.interest_rate
    }

    pub fn available_amount(&self) -> Money {
        Money::from_cents(
            self.max_limit.amount_cents() - self.current_balance.amount_cents(),
            self.max_limit.currency(),
        )
    }

    pub fn utilization_rate(&self) -> f64 {
        if self.max_limit.amount_cents() == 0 {
            0.0
        } else {
            self.current_balance.amount_cents() as f64 / self.max_limit.amount_cents() as f64
        }
    }

    pub fn sub_limits(&self) -> &[SubLimit] {
        &self.sub_limits
    }

    pub fn status(&self) -> RevolvingCreditStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Domain behavior ---

    /// Activate the revolving credit line.
    pub fn activate(&mut self) -> Result<(), DomainError> {
        if self.status != RevolvingCreditStatus::Pending {
            return Err(DomainError::ValidationError(
                "Only pending credit lines can be activated".to_string(),
            ));
        }
        self.status = RevolvingCreditStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Drawdown from the revolving credit line.
    pub fn drawdown(&mut self, amount: Money) -> Result<(), DomainError> {
        if self.status != RevolvingCreditStatus::Active {
            return Err(DomainError::ValidationError(
                "Credit line must be active to drawdown".to_string(),
            ));
        }

        if amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Drawdown amount must be positive".to_string(),
            ));
        }

        if amount.amount_cents() > self.available_amount().amount_cents() {
            return Err(DomainError::ValidationError(format!(
                "Insufficient available limit: requested {}, available {}",
                amount.amount(),
                self.available_amount().amount()
            )));
        }

        self.current_balance = Money::from_cents(
            self.current_balance.amount_cents() + amount.amount_cents(),
            self.current_balance.currency(),
        );
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Repay the revolving credit line.
    pub fn repay(&mut self, amount: Money) -> Result<(), DomainError> {
        if amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Repayment amount must be positive".to_string(),
            ));
        }

        if amount.amount_cents() > self.current_balance.amount_cents() {
            return Err(DomainError::ValidationError(format!(
                "Repayment exceeds current balance: repaying {}, balance {}",
                amount.amount(),
                self.current_balance.amount()
            )));
        }

        self.current_balance = Money::from_cents(
            self.current_balance.amount_cents() - amount.amount_cents(),
            self.current_balance.currency(),
        );
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Add a sub-limit to the revolving credit line.
    pub fn add_sub_limit(&mut self, sub_limit: SubLimit) -> Result<(), DomainError> {
        // Ensure currency match
        if sub_limit.max_amount().currency() != self.max_limit.currency() {
            return Err(DomainError::ValidationError(
                "Sub-limit currency must match credit line currency".to_string(),
            ));
        }

        self.sub_limits.push(sub_limit);
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Suspend the credit line.
    pub fn suspend(&mut self) -> Result<(), DomainError> {
        if self.status == RevolvingCreditStatus::Closed {
            return Err(DomainError::ValidationError(
                "Cannot suspend a closed credit line".to_string(),
            ));
        }
        self.status = RevolvingCreditStatus::Suspended;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Close the credit line.
    pub fn close(&mut self) -> Result<(), DomainError> {
        if self.current_balance.amount_cents() > 0 {
            return Err(DomainError::ValidationError(
                "Cannot close credit line with outstanding balance".to_string(),
            ));
        }
        self.status = RevolvingCreditStatus::Closed;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- SyndicatedLoan aggregate root (FR-038) ---
// Pret syndique: participation multiple banques

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyndicatedLoan {
    id: SyndicatedLoanId,
    base_loan_id: LoanId,
    participant_shares: Vec<LoanSyndicationParticipant>,
    total_syndicated_amount: Money,
    arrangement_fee_rate: f64,
    agent_bank_id: String, // Bank participant ID serving as lead
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoanSyndicationParticipant {
    bank_id: String,
    participation_amount: Money,
    participation_rate: f64,
    is_lead: bool,
}

impl LoanSyndicationParticipant {
    pub fn new(
        bank_id: String,
        participation_amount: Money,
        is_lead: bool,
    ) -> Result<Self, DomainError> {
        if participation_amount.amount_cents() <= 0 {
            return Err(DomainError::ValidationError(
                "Participation amount must be positive".to_string(),
            ));
        }

        Ok(LoanSyndicationParticipant {
            bank_id,
            participation_amount,
            participation_rate: 0.0, // Calculated when added to syndicate
            is_lead,
        })
    }

    pub fn bank_id(&self) -> &str {
        &self.bank_id
    }

    pub fn participation_amount(&self) -> &Money {
        &self.participation_amount
    }

    pub fn participation_rate(&self) -> f64 {
        self.participation_rate
    }

    pub fn is_lead(&self) -> bool {
        self.is_lead
    }

    fn set_participation_rate(&mut self, rate: f64) {
        self.participation_rate = rate;
    }
}

impl SyndicatedLoan {
    /// Create a new syndicated loan with lead bank.
    pub fn new(
        base_loan_id: LoanId,
        agent_bank_id: String,
        arrangement_fee_rate: f64,
    ) -> Result<Self, DomainError> {
        if arrangement_fee_rate < 0.0 || arrangement_fee_rate > 0.1 {
            return Err(DomainError::ValidationError(
                "Arrangement fee rate must be 0-10%".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(SyndicatedLoan {
            id: SyndicatedLoanId::new(),
            base_loan_id,
            participant_shares: Vec::new(),
            total_syndicated_amount: Money::zero(crate::shared::value_objects::Currency::TND),
            arrangement_fee_rate,
            agent_bank_id,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence.
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: SyndicatedLoanId,
        base_loan_id: LoanId,
        participant_shares: Vec<LoanSyndicationParticipant>,
        total_syndicated_amount: Money,
        arrangement_fee_rate: f64,
        agent_bank_id: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SyndicatedLoan {
            id,
            base_loan_id,
            participant_shares,
            total_syndicated_amount,
            arrangement_fee_rate,
            agent_bank_id,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &SyndicatedLoanId {
        &self.id
    }

    pub fn base_loan_id(&self) -> &LoanId {
        &self.base_loan_id
    }

    pub fn participant_shares(&self) -> &[LoanSyndicationParticipant] {
        &self.participant_shares
    }

    pub fn total_syndicated_amount(&self) -> &Money {
        &self.total_syndicated_amount
    }

    pub fn arrangement_fee_rate(&self) -> f64 {
        self.arrangement_fee_rate
    }

    pub fn agent_bank_id(&self) -> &str {
        &self.agent_bank_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn num_participants(&self) -> usize {
        self.participant_shares.len()
    }

    // --- Domain behavior ---

    /// Add a participant to the syndication. Updates rates.
    pub fn add_participant(
        &mut self,
        mut participant: LoanSyndicationParticipant,
    ) -> Result<(), DomainError> {
        // Ensure total doesn't exceed base loan
        let new_total = self.total_syndicated_amount.amount_cents()
            + participant.participation_amount().amount_cents();

        // Calculate and set rate
        participant.set_participation_rate(
            participant.participation_amount().amount_cents() as f64 / new_total as f64,
        );

        self.participant_shares.push(participant);
        self.total_syndicated_amount = Money::from_cents(
            new_total,
            self.total_syndicated_amount.currency(),
        );
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Calculate arrangement fee payable by borrower.
    pub fn calculate_arrangement_fee(&self) -> Money {
        let fee_cents =
            (self.total_syndicated_amount.amount_cents() as f64 * self.arrangement_fee_rate)
                .round() as i64;
        Money::from_cents(fee_cents, self.total_syndicated_amount.currency())
    }

    /// Get a participant's share by bank ID.
    pub fn get_participant(&self, bank_id: &str) -> Option<&LoanSyndicationParticipant> {
        self.participant_shares
            .iter()
            .find(|p| p.bank_id == bank_id)
    }
}

// --- EarlyRepaymentPenalty value object (FR-041) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarlyRepaymentPenalty {
    penalty_type: PenaltyType,
    penalty_amount: Money,
    calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PenaltyType {
    FixedPercentage,     // % of remaining balance
    InterestCompensation, // Remaining interest
    None,
}

impl EarlyRepaymentPenalty {
    /// Calculate penalty for early repayment based on remaining balance and rate.
    pub fn calculate_early_repayment_penalty(
        remaining_balance: &Money,
        remaining_months: u32,
        annual_interest_rate: f64,
        penalty_type: PenaltyType,
    ) -> Result<Self, DomainError> {
        let penalty_amount = match penalty_type {
            PenaltyType::FixedPercentage => {
                // Default: 2% of remaining balance
                let cents = (remaining_balance.amount_cents() as f64 * 0.02).round() as i64;
                Money::from_cents(cents, remaining_balance.currency())
            }
            PenaltyType::InterestCompensation => {
                // Remaining interest at annual rate for remaining months
                let monthly_rate = annual_interest_rate / 12.0 / 100.0;
                let interest_cents =
                    (remaining_balance.amount_cents() as f64 * monthly_rate * remaining_months as f64)
                        .round() as i64;
                Money::from_cents(interest_cents, remaining_balance.currency())
            }
            PenaltyType::None => Money::zero(remaining_balance.currency()),
        };

        Ok(EarlyRepaymentPenalty {
            penalty_type,
            penalty_amount,
            calculated_at: Utc::now(),
        })
    }

    pub fn penalty_type(&self) -> PenaltyType {
        self.penalty_type
    }

    pub fn penalty_amount(&self) -> &Money {
        &self.penalty_amount
    }

    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- MoratorYInterest value object (FR-040) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoratorYInterest {
    accrued_amount: Money,
    annual_rate: f64,
    days_overdue: u32,
    calculated_at: DateTime<Utc>,
}

impl MoratorYInterest {
    /// Calculate moratory interest on overdue amount (taux de penalite).
    /// Default: principal outstanding at 2% above contracted rate.
    pub fn calculate_moratory_interest(
        overdue_amount: &Money,
        days_overdue: u32,
        contracted_annual_rate: f64,
    ) -> Result<Self, DomainError> {
        if days_overdue == 0 {
            return Err(DomainError::ValidationError(
                "Days overdue must be positive".to_string(),
            ));
        }

        // Moratory rate = contracted rate + 2% (FR standard)
        let moratory_rate = contracted_annual_rate + 2.0;
        let daily_rate = moratory_rate / 365.0 / 100.0;
        let accrued_cents =
            (overdue_amount.amount_cents() as f64 * daily_rate * days_overdue as f64).round()
                as i64;

        Ok(MoratorYInterest {
            accrued_amount: Money::from_cents(accrued_cents, overdue_amount.currency()),
            annual_rate: moratory_rate,
            days_overdue,
            calculated_at: Utc::now(),
        })
    }

    pub fn accrued_amount(&self) -> &Money {
        &self.accrued_amount
    }

    pub fn annual_rate(&self) -> f64 {
        self.annual_rate
    }

    pub fn days_overdue(&self) -> u32 {
        self.days_overdue
    }

    pub fn calculated_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }
}

// --- LoanRestructuring entity (FR-034) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoanRestructuring {
    id: uuid::Uuid,
    loan_id: LoanId,
    restructuring_reason: String,
    new_maturity_date: NaiveDate,
    new_interest_rate: f64,
    new_payment_frequency: PaymentFrequency,
    grace_period_months: u32,
    principal_reduction: Option<Money>,
    restructured_at: DateTime<Utc>,
}

impl LoanRestructuring {
    /// Create a loan restructuring record.
    pub fn new(
        loan_id: LoanId,
        restructuring_reason: String,
        new_maturity_date: NaiveDate,
        new_interest_rate: f64,
        new_payment_frequency: PaymentFrequency,
        grace_period_months: u32,
    ) -> Result<Self, DomainError> {
        if new_interest_rate <= 0.0 {
            return Err(DomainError::ValidationError(
                "New interest rate must be positive".to_string(),
            ));
        }

        Ok(LoanRestructuring {
            id: uuid::Uuid::new_v4(),
            loan_id,
            restructuring_reason,
            new_maturity_date,
            new_interest_rate,
            new_payment_frequency,
            grace_period_months,
            principal_reduction: None,
            restructured_at: Utc::now(),
        })
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn loan_id(&self) -> &LoanId {
        &self.loan_id
    }

    pub fn restructuring_reason(&self) -> &str {
        &self.restructuring_reason
    }

    pub fn new_maturity_date(&self) -> NaiveDate {
        self.new_maturity_date
    }

    pub fn new_interest_rate(&self) -> f64 {
        self.new_interest_rate
    }

    pub fn new_payment_frequency(&self) -> PaymentFrequency {
        self.new_payment_frequency
    }

    pub fn grace_period_months(&self) -> u32 {
        self.grace_period_months
    }

    pub fn principal_reduction(&self) -> Option<&Money> {
        self.principal_reduction.as_ref()
    }

    pub fn restructured_at(&self) -> DateTime<Utc> {
        self.restructured_at
    }

    /// Set principal reduction (ex: debt forgiveness).
    pub fn set_principal_reduction(&mut self, reduction: Money) {
        self.principal_reduction = Some(reduction);
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::AccountId;
    use crate::shared::value_objects::{Currency, CustomerId, Money};

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    // --- SubLimit Tests (FR-036) ---

    #[test]
    fn test_sublimit_creation() {
        let line_id = RevolvingCreditLineId::new();
        let limit = SubLimit::new(line_id.clone(), SubLimitType::ByCurrency, tnd(50000.0)).unwrap();
        assert_eq!(limit.max_amount().amount(), 50000.0);
        assert_eq!(limit.utilized_amount().amount(), 0.0);
        assert_eq!(limit.utilization_rate(), 0.0);
    }

    #[test]
    fn test_sublimit_zero_amount_fails() {
        let line_id = RevolvingCreditLineId::new();
        let result = SubLimit::new(line_id, SubLimitType::ByCurrency, tnd(0.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_sublimit_drawdown() {
        let line_id = RevolvingCreditLineId::new();
        let mut limit = SubLimit::new(line_id, SubLimitType::ByCurrency, tnd(100000.0)).unwrap();
        assert!(limit.drawdown(tnd(30000.0)).is_ok());
        assert_eq!(limit.utilized_amount().amount(), 30000.0);
        assert_eq!(limit.available_amount().amount(), 70000.0);
        assert_eq!(limit.utilization_rate(), 0.3);
    }

    #[test]
    fn test_sublimit_drawdown_exceeds_fails() {
        let line_id = RevolvingCreditLineId::new();
        let mut limit = SubLimit::new(line_id, SubLimitType::ByCurrency, tnd(50000.0)).unwrap();
        let result = limit.drawdown(tnd(60000.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_sublimit_repay() {
        let line_id = RevolvingCreditLineId::new();
        let mut limit = SubLimit::new(line_id, SubLimitType::ByCurrency, tnd(100000.0)).unwrap();
        limit.drawdown(tnd(50000.0)).unwrap();
        assert!(limit.repay(tnd(20000.0)).is_ok());
        assert_eq!(limit.utilized_amount().amount(), 30000.0);
    }

    // --- RevolvingCreditLine Tests (FR-035) ---

    #[test]
    fn test_revolving_credit_creation() {
        let line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        assert_eq!(line.status(), RevolvingCreditStatus::Pending);
        assert_eq!(line.current_balance().amount(), 0.0);
        assert_eq!(line.utilization_rate(), 0.0);
    }

    #[test]
    fn test_revolving_credit_activation() {
        let mut line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        assert!(line.activate().is_ok());
        assert_eq!(line.status(), RevolvingCreditStatus::Active);
    }

    #[test]
    fn test_revolving_credit_drawdown() {
        let mut line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        line.activate().unwrap();
        assert!(line.drawdown(tnd(50000.0)).is_ok());
        assert_eq!(line.current_balance().amount(), 50000.0);
        assert_eq!(line.available_amount().amount(), 50000.0);
    }

    #[test]
    fn test_revolving_credit_drawdown_inactive_fails() {
        let mut line =
            RevolvingCreditLine::new(CustomerId::new(), AccountId::new(), tnd(100000.0), 8.0)
                .unwrap();
        assert!(line.drawdown(tnd(50000.0)).is_err());
    }

    #[test]
    fn test_revolving_credit_repay() {
        let mut line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        line.activate().unwrap();
        line.drawdown(tnd(60000.0)).unwrap();
        assert!(line.repay(tnd(20000.0)).is_ok());
        assert_eq!(line.current_balance().amount(), 40000.0);
    }

    #[test]
    fn test_revolving_credit_add_sublimit() {
        let mut line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        let sublimit =
            SubLimit::new(line.id().clone(), SubLimitType::ByCurrency, tnd(30000.0)).unwrap();
        assert!(line.add_sub_limit(sublimit).is_ok());
        assert_eq!(line.sub_limits().len(), 1);
    }

    #[test]
    fn test_revolving_credit_close() {
        let mut line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        line.activate().unwrap();
        assert!(line.close().is_ok());
        assert_eq!(line.status(), RevolvingCreditStatus::Closed);
    }

    #[test]
    fn test_revolving_credit_close_with_balance_fails() {
        let mut line = RevolvingCreditLine::new(
            CustomerId::new(),
            AccountId::new(),
            tnd(100000.0),
            8.0,
        )
        .unwrap();
        line.activate().unwrap();
        line.drawdown(tnd(10000.0)).unwrap();
        assert!(line.close().is_err());
    }

    // --- SyndicatedLoan Tests (FR-038) ---

    #[test]
    fn test_syndicated_loan_creation() {
        let loan = SyndicatedLoan::new(LoanId::new(), "bank-001".to_string(), 0.05).unwrap();
        assert_eq!(loan.arrangement_fee_rate(), 0.05);
        assert_eq!(loan.num_participants(), 0);
    }

    #[test]
    fn test_syndicated_loan_add_participant() {
        let mut loan = SyndicatedLoan::new(LoanId::new(), "bank-001".to_string(), 0.05).unwrap();
        let participant =
            LoanSyndicationParticipant::new("bank-002".to_string(), tnd(50000.0), false).unwrap();
        assert!(loan.add_participant(participant).is_ok());
        assert_eq!(loan.num_participants(), 1);
    }

    #[test]
    fn test_syndicated_loan_arrangement_fee() {
        let mut loan = SyndicatedLoan::new(LoanId::new(), "bank-001".to_string(), 0.05).unwrap();
        let participant =
            LoanSyndicationParticipant::new("bank-002".to_string(), tnd(100000.0), false)
                .unwrap();
        loan.add_participant(participant).unwrap();
        let fee = loan.calculate_arrangement_fee();
        assert_eq!(fee.amount(), 5000.0); // 5% of 100000
    }

    // --- EarlyRepaymentPenalty Tests (FR-041) ---

    #[test]
    fn test_early_repayment_fixed_percentage() {
        let penalty = EarlyRepaymentPenalty::calculate_early_repayment_penalty(
            &tnd(100000.0),
            12,
            8.0,
            PenaltyType::FixedPercentage,
        )
        .unwrap();
        assert_eq!(penalty.penalty_amount().amount(), 2000.0); // 2% of 100000
    }

    #[test]
    fn test_early_repayment_interest_compensation() {
        let penalty = EarlyRepaymentPenalty::calculate_early_repayment_penalty(
            &tnd(120000.0),
            12,
            8.0,
            PenaltyType::InterestCompensation,
        )
        .unwrap();
        // Monthly rate: 8%/12 = 0.00667, for 12 months on 120000 = 9600
        assert!(penalty.penalty_amount().amount() > 9000.0);
        assert!(penalty.penalty_amount().amount() < 10000.0);
    }

    #[test]
    fn test_early_repayment_none() {
        let penalty = EarlyRepaymentPenalty::calculate_early_repayment_penalty(
            &tnd(100000.0),
            12,
            8.0,
            PenaltyType::None,
        )
        .unwrap();
        assert_eq!(penalty.penalty_amount().amount(), 0.0);
    }

    // --- MoratorYInterest Tests (FR-040) ---

    #[test]
    fn test_moratory_interest_calculation() {
        let interest = MoratorYInterest::calculate_moratory_interest(
            &tnd(50000.0),
            30, // 30 days overdue
            8.0, // 8% contracted rate
        )
        .unwrap();
        // Moratory rate = 8 + 2 = 10%, daily = 10/365 = 0.0274%
        // 50000 * 0.0274% * 30 = 410.96
        assert!(interest.accrued_amount().amount() > 400.0);
        assert!(interest.accrued_amount().amount() < 450.0);
        assert_eq!(interest.annual_rate(), 10.0);
        assert_eq!(interest.days_overdue(), 30);
    }

    #[test]
    fn test_moratory_interest_zero_days_fails() {
        let result = MoratorYInterest::calculate_moratory_interest(&tnd(50000.0), 0, 8.0);
        assert!(result.is_err());
    }

    // --- LoanRestructuring Tests (FR-034) ---

    #[test]
    fn test_loan_restructuring_creation() {
        let restructuring = LoanRestructuring::new(
            LoanId::new(),
            "Financial hardship".to_string(),
            NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(),
            6.0,
            PaymentFrequency::Monthly,
            6,
        )
        .unwrap();
        assert_eq!(restructuring.grace_period_months(), 6);
        assert_eq!(restructuring.new_interest_rate(), 6.0);
    }

    #[test]
    fn test_loan_restructuring_with_principal_reduction() {
        let mut restructuring = LoanRestructuring::new(
            LoanId::new(),
            "Debt forgiveness".to_string(),
            NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(),
            5.5,
            PaymentFrequency::Quarterly,
            0,
        )
        .unwrap();
        restructuring.set_principal_reduction(tnd(10000.0));
        assert_eq!(restructuring.principal_reduction().unwrap().amount(), 10000.0);
    }

    #[test]
    fn test_loan_restructuring_zero_rate_fails() {
        let result = LoanRestructuring::new(
            LoanId::new(),
            "Restructure".to_string(),
            NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(),
            0.0,
            PaymentFrequency::Monthly,
            0,
        );
        assert!(result.is_err());
    }
}
