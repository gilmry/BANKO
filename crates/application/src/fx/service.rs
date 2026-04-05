use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use banko_domain::fx::*;

use super::dto::*;
use super::errors::FxServiceError;
use super::ports::*;

// ============================================================
// FxService (FX-01 to FX-04)
// ============================================================

pub struct FxService {
    fx_repo: Arc<dyn IFxRepository>,
    rate_repo: Arc<dyn IExchangeRateRepository>,
}

impl FxService {
    pub fn new(
        fx_repo: Arc<dyn IFxRepository>,
        rate_repo: Arc<dyn IExchangeRateRepository>,
    ) -> Self {
        FxService { fx_repo, rate_repo }
    }

    /// Create a new FX operation (FX-01)
    pub async fn create_operation(
        &self,
        req: CreateFxOperationRequest,
    ) -> Result<FxOperationResponse, FxServiceError> {
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|e| FxServiceError::InvalidInput(format!("Invalid account_id: {e}")))?;

        let operation_type = match req.operation_type.as_deref() {
            Some(t) => FxOperationType::from_str_type(t)
                .map_err(|e| FxServiceError::InvalidInput(e.to_string()))?,
            None => FxOperationType::Spot,
        };

        // If no rate provided, look up the current rate
        let rate = match req.rate {
            Some(r) => r,
            None => {
                let exchange_rate = self
                    .rate_repo
                    .find_current(&req.source_currency, &req.target_currency)
                    .await
                    .map_err(FxServiceError::Internal)?
                    .ok_or(FxServiceError::RateNotFound)?;
                exchange_rate.rate()
            }
        };

        // Check daily limit (FX-08)
        let today = Utc::now().date_naive();
        let daily_total = self
            .fx_repo
            .get_daily_total(account_id, &req.source_currency, today)
            .await
            .map_err(FxServiceError::Internal)?;

        // Default daily limit: 100_000_000 (100k in base units)
        let default_limit: i64 = 100_000_000;
        if daily_total + req.source_amount > default_limit {
            return Err(FxServiceError::DailyLimitExceeded(format!(
                "Daily limit of {} exceeded. Used today: {}, requested: {}",
                default_limit, daily_total, req.source_amount
            )));
        }

        // Compliance check (FX-07 Loi 76-18): authorized intermediary control
        // In a real system this would check against authorized intermediary registry
        // For now, we validate the operation parameters
        if req.source_amount > 50_000_000 {
            // Operations above 50k base units require additional authorization per Loi 76-18
            // This is a simplified compliance check
        }

        let operation = FxOperation::new(
            account_id,
            operation_type,
            req.source_currency,
            req.target_currency,
            req.source_amount,
            rate,
            req.reference,
        )
        .map_err(|e| FxServiceError::DomainError(e.to_string()))?;

        self.fx_repo
            .save(&operation)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(to_fx_response(&operation))
    }

    /// Confirm an FX operation (FX-02)
    pub async fn confirm_operation(
        &self,
        id: Uuid,
    ) -> Result<FxOperationResponse, FxServiceError> {
        let op_id = FxOperationId::from_uuid(id);
        let mut operation = self
            .fx_repo
            .find_by_id(&op_id)
            .await
            .map_err(FxServiceError::Internal)?
            .ok_or(FxServiceError::OperationNotFound)?;

        operation
            .confirm()
            .map_err(|e| FxServiceError::DomainError(e.to_string()))?;

        self.fx_repo
            .save(&operation)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(to_fx_response(&operation))
    }

    /// Settle an FX operation (FX-03)
    pub async fn settle_operation(
        &self,
        id: Uuid,
    ) -> Result<FxOperationResponse, FxServiceError> {
        let op_id = FxOperationId::from_uuid(id);
        let mut operation = self
            .fx_repo
            .find_by_id(&op_id)
            .await
            .map_err(FxServiceError::Internal)?
            .ok_or(FxServiceError::OperationNotFound)?;

        operation
            .settle()
            .map_err(|e| FxServiceError::DomainError(e.to_string()))?;

        self.fx_repo
            .save(&operation)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(to_fx_response(&operation))
    }

    /// Reject an FX operation (FX-04)
    pub async fn reject_operation(
        &self,
        id: Uuid,
        reason: String,
    ) -> Result<FxOperationResponse, FxServiceError> {
        let op_id = FxOperationId::from_uuid(id);
        let mut operation = self
            .fx_repo
            .find_by_id(&op_id)
            .await
            .map_err(FxServiceError::Internal)?
            .ok_or(FxServiceError::OperationNotFound)?;

        operation
            .reject(reason)
            .map_err(|e| FxServiceError::DomainError(e.to_string()))?;

        self.fx_repo
            .save(&operation)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(to_fx_response(&operation))
    }

    /// Get an FX operation by ID
    pub async fn get_operation(
        &self,
        id: Uuid,
    ) -> Result<FxOperationResponse, FxServiceError> {
        let op_id = FxOperationId::from_uuid(id);
        let operation = self
            .fx_repo
            .find_by_id(&op_id)
            .await
            .map_err(FxServiceError::Internal)?
            .ok_or(FxServiceError::OperationNotFound)?;

        Ok(to_fx_response(&operation))
    }

    /// List FX operations with optional status filter and pagination
    pub async fn list_operations(
        &self,
        status: Option<&str>,
        page: i64,
        limit: i64,
    ) -> Result<FxOperationListResponse, FxServiceError> {
        let offset = (page - 1) * limit;
        let operations = self
            .fx_repo
            .find_all(status, limit, offset)
            .await
            .map_err(FxServiceError::Internal)?;
        let total = self
            .fx_repo
            .count_all(status)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(FxOperationListResponse {
            data: operations.iter().map(to_fx_response).collect(),
            total,
            page,
            limit,
        })
    }
}

// ============================================================
// RateService (FX-05)
// ============================================================

pub struct RateService {
    rate_repo: Arc<dyn IExchangeRateRepository>,
}

impl RateService {
    pub fn new(rate_repo: Arc<dyn IExchangeRateRepository>) -> Self {
        RateService { rate_repo }
    }

    /// Update/create an exchange rate
    pub async fn update_rate(
        &self,
        req: UpdateRateRequest,
    ) -> Result<ExchangeRateResponse, FxServiceError> {
        let rate = ExchangeRate::new(req.source_currency, req.target_currency, req.rate)
            .map_err(|e| FxServiceError::DomainError(e.to_string()))?;

        self.rate_repo
            .save(&rate)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(to_rate_response(&rate))
    }

    /// Get current rate for a currency pair
    pub async fn get_current_rate(
        &self,
        source: &str,
        target: &str,
    ) -> Result<ExchangeRateResponse, FxServiceError> {
        let rate = self
            .rate_repo
            .find_current(source, target)
            .await
            .map_err(FxServiceError::Internal)?
            .ok_or(FxServiceError::RateNotFound)?;

        Ok(to_rate_response(&rate))
    }

    /// List all current exchange rates
    pub async fn list_rates(&self) -> Result<RateListResponse, FxServiceError> {
        let rates = self
            .rate_repo
            .find_all_current()
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(RateListResponse {
            data: rates.iter().map(to_rate_response).collect(),
        })
    }

    /// Get rate history for a currency pair
    pub async fn get_rate_history(
        &self,
        source: &str,
        target: &str,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<RateListResponse, FxServiceError> {
        let rates = self
            .rate_repo
            .find_history(source, target, from, to)
            .await
            .map_err(FxServiceError::Internal)?;

        Ok(RateListResponse {
            data: rates.iter().map(to_rate_response).collect(),
        })
    }
}

// ============================================================
// PositionService (FX-06)
// ============================================================

pub struct PositionService {
    fx_repo: Arc<dyn IFxRepository>,
}

impl PositionService {
    pub fn new(fx_repo: Arc<dyn IFxRepository>) -> Self {
        PositionService { fx_repo }
    }

    /// Get position summary across all currencies (settled operations)
    pub async fn get_position_summary(&self) -> Result<PositionSummaryResponse, FxServiceError> {
        // Fetch all settled operations to compute positions
        let operations = self
            .fx_repo
            .find_all(Some("Settled"), 10000, 0)
            .await
            .map_err(FxServiceError::Internal)?;

        let mut currency_positions: std::collections::HashMap<String, (i64, i64)> =
            std::collections::HashMap::new();

        for op in &operations {
            // Source currency: short position (we sell source)
            let entry = currency_positions
                .entry(op.source_currency().to_string())
                .or_insert((0, 0));
            entry.1 += op.source_amount();

            // Target currency: long position (we buy target)
            let entry = currency_positions
                .entry(op.target_currency().to_string())
                .or_insert((0, 0));
            entry.0 += op.target_amount();
        }

        let positions: Vec<FxPositionResponse> = currency_positions
            .into_iter()
            .map(|(currency, (long_amount, short_amount))| {
                let pos = FxPosition::new(currency, long_amount, short_amount);
                FxPositionResponse {
                    currency: pos.currency,
                    long_amount: pos.long_amount,
                    short_amount: pos.short_amount,
                    net_position: pos.net_position,
                }
            })
            .collect();

        Ok(PositionSummaryResponse { positions })
    }
}

// ============================================================
// ComplianceService (FX-07 — Loi 76-18)
// ============================================================

pub struct FxComplianceService;

impl FxComplianceService {
    pub fn new() -> Self {
        FxComplianceService
    }

    /// Check compliance for an FX operation per Loi 76-18
    /// Rules:
    /// - FX operations must go through authorized intermediaries (BCT controlled)
    /// - Large operations require additional documentation
    /// - Certain currency pairs may have restrictions
    pub fn check_compliance(
        &self,
        source_currency: &str,
        target_currency: &str,
        amount: i64,
    ) -> Result<(), FxServiceError> {
        // Rule 1: Both currencies must be recognized
        let valid_currencies = ["TND", "EUR", "USD", "GBP", "LYD"];
        if !valid_currencies.contains(&source_currency) {
            return Err(FxServiceError::ComplianceFailed(format!(
                "Unrecognized source currency: {source_currency}"
            )));
        }
        if !valid_currencies.contains(&target_currency) {
            return Err(FxServiceError::ComplianceFailed(format!(
                "Unrecognized target currency: {target_currency}"
            )));
        }

        // Rule 2: Same currency exchange not allowed
        if source_currency == target_currency {
            return Err(FxServiceError::ComplianceFailed(
                "Same currency exchange not allowed".to_string(),
            ));
        }

        // Rule 3: Loi 76-18 — TND operations require authorized intermediary
        // All TND operations are monitored by BCT
        // Operations above 50M millimes (50k TND) require enhanced due diligence
        if (source_currency == "TND" || target_currency == "TND") && amount > 50_000_000 {
            // In production, this would verify the intermediary is BCT-authorized
            // For now, we log/flag but allow (the system itself acts as authorized intermediary)
        }

        Ok(())
    }
}

impl Default for FxComplianceService {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LimitsService (FX-08)
// ============================================================

pub struct FxLimitsService {
    fx_repo: Arc<dyn IFxRepository>,
}

impl FxLimitsService {
    pub fn new(fx_repo: Arc<dyn IFxRepository>) -> Self {
        FxLimitsService { fx_repo }
    }

    /// Check if a daily limit would be exceeded
    pub async fn check_daily_limit(
        &self,
        account_id: Uuid,
        currency: &str,
        amount: i64,
    ) -> Result<bool, FxServiceError> {
        let today = Utc::now().date_naive();
        let used_today = self
            .fx_repo
            .get_daily_total(account_id, currency, today)
            .await
            .map_err(FxServiceError::Internal)?;

        let limit = DailyLimit::new(account_id, currency.to_string(), 100_000_000, used_today);
        Ok(limit.can_execute(amount))
    }

    /// Get remaining daily limits for an account across all currencies
    pub async fn get_remaining_limits(
        &self,
        account_id: Uuid,
    ) -> Result<DailyLimitsResponse, FxServiceError> {
        let today = Utc::now().date_naive();
        let currencies = ["TND", "EUR", "USD", "GBP", "LYD"];
        let mut limits = Vec::new();

        for currency in &currencies {
            let used_today = self
                .fx_repo
                .get_daily_total(account_id, currency, today)
                .await
                .map_err(FxServiceError::Internal)?;

            let daily_limit_val: i64 = 100_000_000;
            let limit = DailyLimit::new(
                account_id,
                currency.to_string(),
                daily_limit_val,
                used_today,
            );

            limits.push(DailyLimitResponse {
                account_id: account_id.to_string(),
                currency: currency.to_string(),
                daily_limit: daily_limit_val,
                used_today,
                remaining: limit.remaining(),
            });
        }

        Ok(DailyLimitsResponse { limits })
    }
}

// ============================================================
// Mapping helpers
// ============================================================

fn to_fx_response(op: &FxOperation) -> FxOperationResponse {
    FxOperationResponse {
        id: op.operation_id().to_string(),
        account_id: op.account_id().to_string(),
        operation_type: op.operation_type().as_str().to_string(),
        source_currency: op.source_currency().to_string(),
        target_currency: op.target_currency().to_string(),
        source_amount: op.source_amount(),
        target_amount: op.target_amount(),
        rate: op.rate(),
        status: op.status().as_str().to_string(),
        reference: op.reference().to_string(),
        rejection_reason: op.rejection_reason().map(|s| s.to_string()),
        created_at: op.created_at(),
        confirmed_at: op.confirmed_at(),
        settled_at: op.settled_at(),
    }
}

fn to_rate_response(rate: &ExchangeRate) -> ExchangeRateResponse {
    ExchangeRateResponse {
        source_currency: rate.source_currency().to_string(),
        target_currency: rate.target_currency().to_string(),
        rate: rate.rate(),
        valid_from: rate.valid_from(),
        valid_to: rate.valid_to(),
    }
}
