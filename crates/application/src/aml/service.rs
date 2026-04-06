use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use banko_domain::account::Account;
use banko_domain::aml::*;
use banko_domain::shared::Money;

use super::dto::*;
use super::errors::AmlServiceError;
use super::ports::*;

// ============================================================
// TransactionMonitoringService (AML-02, AML-04, AML-05)
// ============================================================

pub struct TransactionMonitoringService {
    tx_repo: Arc<dyn ITransactionRepository>,
    alert_repo: Arc<dyn IAlertRepository>,
    scenarios: Vec<Arc<dyn IAmlScenario>>,
}

impl TransactionMonitoringService {
    pub fn new(
        tx_repo: Arc<dyn ITransactionRepository>,
        alert_repo: Arc<dyn IAlertRepository>,
        scenarios: Vec<Arc<dyn IAmlScenario>>,
    ) -> Self {
        TransactionMonitoringService {
            tx_repo,
            alert_repo,
            scenarios,
        }
    }

    /// Record a transaction and run all AML scenarios.
    pub async fn record_transaction(
        &self,
        account_id: Uuid,
        customer_id: Uuid,
        counterparty: String,
        amount: Money,
        transaction_type: TransactionType,
        direction: Direction,
    ) -> Result<(Transaction, Vec<Alert>), AmlServiceError> {
        let tx = Transaction::new(
            account_id,
            customer_id,
            counterparty,
            amount,
            transaction_type,
            direction,
            Utc::now(),
        )
        .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.tx_repo
            .save(&tx)
            .await
            .map_err(AmlServiceError::Internal)?;

        // Fetch recent history for scenario evaluation (24h window)
        let history = self
            .tx_repo
            .find_by_date_range(account_id, Utc::now() - Duration::hours(24), Utc::now())
            .await
            .map_err(AmlServiceError::Internal)?;

        // Run all scenarios
        let mut alerts = Vec::new();
        for scenario in &self.scenarios {
            if let Some(alert) = scenario.evaluate(&tx, &history) {
                self.alert_repo
                    .save(&alert)
                    .await
                    .map_err(AmlServiceError::Internal)?;
                alerts.push(alert);
            }
        }

        Ok((tx, alerts))
    }

    pub async fn get_transaction(
        &self,
        id: &TransactionId,
    ) -> Result<TransactionResponse, AmlServiceError> {
        let tx = self
            .tx_repo
            .find_by_id(id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::TransactionNotFound)?;

        let alerts = self
            .alert_repo
            .find_by_transaction_id(id)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(Self::tx_to_response(&tx, &alerts))
    }

    pub async fn list_transactions(
        &self,
        account_id: Option<Uuid>,
        page: i64,
        limit: i64,
    ) -> Result<TransactionListResponse, AmlServiceError> {
        let offset = (page - 1) * limit;
        let txs = self
            .tx_repo
            .find_all(account_id, limit, offset)
            .await
            .map_err(AmlServiceError::Internal)?;
        let total = self
            .tx_repo
            .count_all(account_id)
            .await
            .map_err(AmlServiceError::Internal)?;

        let mut data = Vec::new();
        for tx in &txs {
            let alerts = self
                .alert_repo
                .find_by_transaction_id(tx.id())
                .await
                .map_err(AmlServiceError::Internal)?;
            data.push(Self::tx_to_response(tx, &alerts));
        }

        Ok(TransactionListResponse {
            data,
            total,
            page,
            limit,
        })
    }

    pub async fn get_alert(&self, id: Uuid) -> Result<AlertResponse, AmlServiceError> {
        let alert = self
            .alert_repo
            .find_by_id(id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::AlertNotFound)?;

        Ok(Self::alert_to_response(&alert))
    }

    pub async fn list_alerts(
        &self,
        status: Option<AlertStatus>,
        risk_level: Option<RiskLevel>,
        page: i64,
        limit: i64,
    ) -> Result<AlertListResponse, AmlServiceError> {
        let offset = (page - 1) * limit;
        let alerts = self
            .alert_repo
            .find_all(status, risk_level, limit, offset)
            .await
            .map_err(AmlServiceError::Internal)?;
        let total = self
            .alert_repo
            .count_by_status(status)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(AlertListResponse {
            data: alerts.iter().map(Self::alert_to_response).collect(),
            total,
            page,
            limit,
        })
    }

    fn tx_to_response(tx: &Transaction, alerts: &[Alert]) -> TransactionResponse {
        TransactionResponse {
            id: tx.id().to_string(),
            account_id: tx.account_id().to_string(),
            customer_id: tx.customer_id().to_string(),
            counterparty: tx.counterparty().to_string(),
            amount: tx.amount().amount(),
            currency: tx.amount().currency().to_string(),
            transaction_type: tx.transaction_type().as_str().to_string(),
            direction: tx.direction().as_str().to_string(),
            timestamp: tx.timestamp(),
            created_at: tx.created_at(),
            alerts: alerts.iter().map(Self::alert_to_response).collect(),
        }
    }

    fn alert_to_response(alert: &Alert) -> AlertResponse {
        AlertResponse {
            id: alert.id().to_string(),
            transaction_id: alert.transaction_id().to_string(),
            risk_level: alert.risk_level().as_str().to_string(),
            reason: alert.reason().to_string(),
            status: alert.status().as_str().to_string(),
            created_at: alert.created_at(),
        }
    }
}

// ============================================================
// InvestigationService (AML-06)
// ============================================================

pub struct InvestigationService {
    investigation_repo: Arc<dyn IInvestigationRepository>,
    alert_repo: Arc<dyn IAlertRepository>,
}

impl InvestigationService {
    pub fn new(
        investigation_repo: Arc<dyn IInvestigationRepository>,
        alert_repo: Arc<dyn IAlertRepository>,
    ) -> Self {
        InvestigationService {
            investigation_repo,
            alert_repo,
        }
    }

    pub async fn open_investigation(
        &self,
        alert_id: Uuid,
        assigned_to: Option<String>,
    ) -> Result<InvestigationResponse, AmlServiceError> {
        // Verify alert exists
        let mut alert = self
            .alert_repo
            .find_by_id(alert_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::AlertNotFound)?;

        alert.mark_under_review();
        self.alert_repo
            .save(&alert)
            .await
            .map_err(AmlServiceError::Internal)?;

        let investigation = Investigation::new(alert_id, assigned_to);
        self.investigation_repo
            .save(&investigation)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(Self::investigation_to_response(&investigation))
    }

    pub async fn add_note(
        &self,
        investigation_id: Uuid,
        note: String,
        author: String,
    ) -> Result<InvestigationResponse, AmlServiceError> {
        let mut investigation = self.get_investigation_entity(investigation_id).await?;

        let inv_note = InvestigationNote::new(note, author)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        investigation
            .add_note(inv_note)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.investigation_repo
            .save(&investigation)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(Self::investigation_to_response(&investigation))
    }

    pub async fn escalate(
        &self,
        investigation_id: Uuid,
    ) -> Result<InvestigationResponse, AmlServiceError> {
        let mut investigation = self.get_investigation_entity(investigation_id).await?;

        investigation
            .escalate()
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.investigation_repo
            .save(&investigation)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(Self::investigation_to_response(&investigation))
    }

    pub async fn close_confirmed(
        &self,
        investigation_id: Uuid,
    ) -> Result<InvestigationResponse, AmlServiceError> {
        let mut investigation = self.get_investigation_entity(investigation_id).await?;

        investigation
            .close_confirmed()
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.investigation_repo
            .save(&investigation)
            .await
            .map_err(AmlServiceError::Internal)?;

        // Update alert status to Confirmed
        if let Ok(Some(mut alert)) = self.alert_repo.find_by_id(investigation.alert_id()).await {
            alert.confirm();
            let _ = self.alert_repo.save(&alert).await;
        }

        Ok(Self::investigation_to_response(&investigation))
    }

    pub async fn close_dismissed(
        &self,
        investigation_id: Uuid,
    ) -> Result<InvestigationResponse, AmlServiceError> {
        let mut investigation = self.get_investigation_entity(investigation_id).await?;

        investigation
            .close_dismissed()
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.investigation_repo
            .save(&investigation)
            .await
            .map_err(AmlServiceError::Internal)?;

        // Update alert status to Dismissed
        if let Ok(Some(mut alert)) = self.alert_repo.find_by_id(investigation.alert_id()).await {
            alert.dismiss();
            let _ = self.alert_repo.save(&alert).await;
        }

        Ok(Self::investigation_to_response(&investigation))
    }

    pub async fn get_investigation(
        &self,
        id: Uuid,
    ) -> Result<InvestigationResponse, AmlServiceError> {
        let investigation = self.get_investigation_entity(id).await?;
        Ok(Self::investigation_to_response(&investigation))
    }

    pub async fn list_investigations(
        &self,
        status: Option<InvestigationStatus>,
        page: i64,
        limit: i64,
    ) -> Result<InvestigationListResponse, AmlServiceError> {
        let offset = (page - 1) * limit;
        let investigations = self
            .investigation_repo
            .find_all(status, limit, offset)
            .await
            .map_err(AmlServiceError::Internal)?;
        let total = self
            .investigation_repo
            .count_all(status)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(InvestigationListResponse {
            data: investigations
                .iter()
                .map(Self::investigation_to_response)
                .collect(),
            total,
            page,
            limit,
        })
    }

    async fn get_investigation_entity(&self, id: Uuid) -> Result<Investigation, AmlServiceError> {
        self.investigation_repo
            .find_by_id(id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::InvestigationNotFound)
    }

    fn investigation_to_response(inv: &Investigation) -> InvestigationResponse {
        InvestigationResponse {
            id: inv.id().to_string(),
            alert_id: inv.alert_id().to_string(),
            status: inv.status().as_str().to_string(),
            assigned_to: inv.assigned_to().map(|s| s.to_string()),
            notes: inv
                .notes()
                .iter()
                .map(|n| InvestigationNoteResponse {
                    note: n.note().to_string(),
                    author: n.author().to_string(),
                    created_at: n.created_at(),
                })
                .collect(),
            created_at: inv.created_at(),
            updated_at: inv.updated_at(),
        }
    }
}

// ============================================================
// DosReportService (AML-07 — DOS generation + CTAF stub)
// ============================================================

pub struct DosReportService {
    report_repo: Arc<dyn ISuspicionReportRepository>,
}

impl DosReportService {
    pub fn new(report_repo: Arc<dyn ISuspicionReportRepository>) -> Self {
        DosReportService { report_repo }
    }

    pub async fn generate_report(
        &self,
        investigation_id: Uuid,
        customer_info: String,
        transaction_details: String,
        reasons: String,
        evidence: Option<String>,
        timeline: Option<String>,
    ) -> Result<SuspicionReportResponse, AmlServiceError> {
        let report = SuspicionReport::new(
            investigation_id,
            customer_info,
            transaction_details,
            reasons,
            evidence,
            timeline,
        )
        .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.report_repo
            .save(&report)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(Self::report_to_response(&report))
    }

    /// Submit report to CTAF (stub — in production would call external API).
    pub async fn submit_to_ctaf(
        &self,
        report_id: Uuid,
    ) -> Result<SuspicionReportResponse, AmlServiceError> {
        let mut report = self
            .report_repo
            .find_by_id(report_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::ReportNotFound)?;

        report
            .submit()
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        // STUB: In production, this would send to CTAF API
        // Log is handled at infrastructure layer

        self.report_repo
            .save(&report)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(Self::report_to_response(&report))
    }

    pub async fn get_report(&self, id: Uuid) -> Result<SuspicionReportResponse, AmlServiceError> {
        let report = self
            .report_repo
            .find_by_id(id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::ReportNotFound)?;

        Ok(Self::report_to_response(&report))
    }

    fn report_to_response(report: &SuspicionReport) -> SuspicionReportResponse {
        SuspicionReportResponse {
            id: report.id().to_string(),
            investigation_id: report.investigation_id().to_string(),
            customer_info: report.customer_info().to_string(),
            transaction_details: report.transaction_details().to_string(),
            reasons: report.reasons().to_string(),
            evidence: report.evidence().map(|s| s.to_string()),
            timeline: report.timeline().map(|s| s.to_string()),
            status: report.status().as_str().to_string(),
            created_at: report.created_at(),
            submitted_at: report.submitted_at(),
        }
    }
}

// ============================================================
// AssetFreezeService (AML-08 — INV-09)
// ============================================================

pub struct AssetFreezeService {
    freeze_repo: Arc<dyn IAssetFreezeRepository>,
    account_port: Option<Arc<dyn IAccountFreezePort>>,
}

impl AssetFreezeService {
    pub fn new(freeze_repo: Arc<dyn IAssetFreezeRepository>) -> Self {
        AssetFreezeService {
            freeze_repo,
            account_port: None,
        }
    }

    /// Create AssetFreezeService with optional account freeze port integration.
    pub fn with_account_port(
        freeze_repo: Arc<dyn IAssetFreezeRepository>,
        account_port: Arc<dyn IAccountFreezePort>,
    ) -> Self {
        AssetFreezeService {
            freeze_repo,
            account_port: Some(account_port),
        }
    }

    /// Freeze account IMMEDIATELY (INV-09).
    /// Creates AssetFreeze record and optionally freezes the account (sets available_balance to 0).
    pub async fn freeze_account(
        &self,
        account_id: Uuid,
        reason: String,
        ordered_by: String,
    ) -> Result<AssetFreezeResponse, AmlServiceError> {
        let freeze = AssetFreeze::freeze(account_id, reason, ordered_by)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.freeze_repo
            .save(&freeze)
            .await
            .map_err(AmlServiceError::Internal)?;

        // If account freeze port is available, freeze the account's available_balance
        if let Some(ref account_port) = self.account_port {
            let _ = account_port
                .freeze_account(account_id)
                .await
                .map_err(|e| AmlServiceError::Internal(e))?;
        }

        Ok(Self::freeze_to_response(&freeze))
    }

    /// Lift freeze — requires CTAF authorization.
    /// Lifts the AssetFreeze record and optionally unfreezes the account (restores available_balance).
    pub async fn lift_freeze(
        &self,
        freeze_id: Uuid,
        lifted_by: String,
    ) -> Result<AssetFreezeResponse, AmlServiceError> {
        let mut freeze = self
            .freeze_repo
            .find_by_id(freeze_id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::FreezeNotFound)?;

        let account_id = freeze.account_id();

        freeze
            .lift(lifted_by)
            .map_err(|e| AmlServiceError::DomainError(e.to_string()))?;

        self.freeze_repo
            .save(&freeze)
            .await
            .map_err(AmlServiceError::Internal)?;

        // If account freeze port is available, unfreeze the account's available_balance
        if let Some(ref account_port) = self.account_port {
            let _ = account_port
                .unfreeze_account(account_id)
                .await
                .map_err(|e| AmlServiceError::Internal(e))?;
        }

        Ok(Self::freeze_to_response(&freeze))
    }

    /// Check if an account currently has an active freeze.
    pub async fn is_account_frozen(&self, account_id: Uuid) -> Result<bool, AmlServiceError> {
        let freeze = self
            .freeze_repo
            .find_active_by_account_id(account_id)
            .await
            .map_err(AmlServiceError::Internal)?;
        Ok(freeze.is_some())
    }

    pub async fn get_freeze(&self, id: Uuid) -> Result<AssetFreezeResponse, AmlServiceError> {
        let freeze = self
            .freeze_repo
            .find_by_id(id)
            .await
            .map_err(AmlServiceError::Internal)?
            .ok_or(AmlServiceError::FreezeNotFound)?;

        Ok(Self::freeze_to_response(&freeze))
    }

    pub async fn list_freezes_for_account(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<AssetFreezeResponse>, AmlServiceError> {
        let freezes = self
            .freeze_repo
            .find_by_account_id(account_id)
            .await
            .map_err(AmlServiceError::Internal)?;

        Ok(freezes.iter().map(Self::freeze_to_response).collect())
    }

    fn freeze_to_response(freeze: &AssetFreeze) -> AssetFreezeResponse {
        AssetFreezeResponse {
            id: freeze.id().to_string(),
            account_id: freeze.account_id().to_string(),
            reason: freeze.reason().to_string(),
            ordered_by: freeze.ordered_by().to_string(),
            status: freeze.status().as_str().to_string(),
            frozen_at: freeze.frozen_at(),
            lifted_at: freeze.lifted_at(),
            lifted_by: freeze.lifted_by().map(|s| s.to_string()),
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    use banko_domain::aml::*;
    use banko_domain::shared::{Currency, Money};

    use super::*;
    use crate::aml::scenarios::ThresholdScenario;

    // --- Mock Repositories ---

    struct MockTransactionRepo {
        txs: Mutex<Vec<Transaction>>,
    }

    impl MockTransactionRepo {
        fn new() -> Self {
            MockTransactionRepo {
                txs: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ITransactionRepository for MockTransactionRepo {
        async fn save(&self, tx: &Transaction) -> Result<(), String> {
            let mut txs = self.txs.lock().unwrap();
            txs.retain(|t| t.id() != tx.id());
            txs.push(tx.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, String> {
            let txs = self.txs.lock().unwrap();
            Ok(txs.iter().find(|t| t.id() == id).cloned())
        }
        async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<Transaction>, String> {
            let txs = self.txs.lock().unwrap();
            Ok(txs
                .iter()
                .filter(|t| t.account_id() == account_id)
                .cloned()
                .collect())
        }
        async fn find_by_date_range(
            &self,
            account_id: Uuid,
            from: DateTime<Utc>,
            to: DateTime<Utc>,
        ) -> Result<Vec<Transaction>, String> {
            let txs = self.txs.lock().unwrap();
            Ok(txs
                .iter()
                .filter(|t| {
                    t.account_id() == account_id && t.timestamp() >= from && t.timestamp() <= to
                })
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            account_id: Option<Uuid>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Transaction>, String> {
            let txs = self.txs.lock().unwrap();
            Ok(txs
                .iter()
                .filter(|t| account_id.is_none() || Some(t.account_id()) == account_id)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self, account_id: Option<Uuid>) -> Result<i64, String> {
            let txs = self.txs.lock().unwrap();
            Ok(txs
                .iter()
                .filter(|t| account_id.is_none() || Some(t.account_id()) == account_id)
                .count() as i64)
        }
    }

    struct MockAlertRepo {
        alerts: Mutex<Vec<Alert>>,
    }

    impl MockAlertRepo {
        fn new() -> Self {
            MockAlertRepo {
                alerts: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAlertRepository for MockAlertRepo {
        async fn save(&self, alert: &Alert) -> Result<(), String> {
            let mut alerts = self.alerts.lock().unwrap();
            alerts.retain(|a| a.id() != alert.id());
            alerts.push(alert.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Alert>, String> {
            let alerts = self.alerts.lock().unwrap();
            Ok(alerts.iter().find(|a| a.id() == id).cloned())
        }
        async fn find_by_transaction_id(
            &self,
            tx_id: &TransactionId,
        ) -> Result<Vec<Alert>, String> {
            let alerts = self.alerts.lock().unwrap();
            Ok(alerts
                .iter()
                .filter(|a| a.transaction_id() == tx_id)
                .cloned()
                .collect())
        }
        async fn find_by_status(&self, status: AlertStatus) -> Result<Vec<Alert>, String> {
            let alerts = self.alerts.lock().unwrap();
            Ok(alerts
                .iter()
                .filter(|a| a.status() == status)
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            status: Option<AlertStatus>,
            _risk_level: Option<RiskLevel>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Alert>, String> {
            let alerts = self.alerts.lock().unwrap();
            Ok(alerts
                .iter()
                .filter(|a| status.is_none() || Some(a.status()) == status)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_by_status(&self, status: Option<AlertStatus>) -> Result<i64, String> {
            let alerts = self.alerts.lock().unwrap();
            Ok(alerts
                .iter()
                .filter(|a| status.is_none() || Some(a.status()) == status)
                .count() as i64)
        }
    }

    struct MockInvestigationRepo {
        investigations: Mutex<Vec<Investigation>>,
    }

    impl MockInvestigationRepo {
        fn new() -> Self {
            MockInvestigationRepo {
                investigations: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IInvestigationRepository for MockInvestigationRepo {
        async fn save(&self, inv: &Investigation) -> Result<(), String> {
            let mut investigations = self.investigations.lock().unwrap();
            investigations.retain(|i| i.id() != inv.id());
            investigations.push(inv.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Investigation>, String> {
            let investigations = self.investigations.lock().unwrap();
            Ok(investigations.iter().find(|i| i.id() == id).cloned())
        }
        async fn find_by_alert_id(&self, alert_id: Uuid) -> Result<Option<Investigation>, String> {
            let investigations = self.investigations.lock().unwrap();
            Ok(investigations
                .iter()
                .find(|i| i.alert_id() == alert_id)
                .cloned())
        }
        async fn find_by_status(
            &self,
            status: InvestigationStatus,
        ) -> Result<Vec<Investigation>, String> {
            let investigations = self.investigations.lock().unwrap();
            Ok(investigations
                .iter()
                .filter(|i| i.status() == status)
                .cloned()
                .collect())
        }
        async fn find_all(
            &self,
            status: Option<InvestigationStatus>,
            limit: i64,
            offset: i64,
        ) -> Result<Vec<Investigation>, String> {
            let investigations = self.investigations.lock().unwrap();
            Ok(investigations
                .iter()
                .filter(|i| status.is_none() || Some(i.status()) == status)
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
        async fn count_all(&self, status: Option<InvestigationStatus>) -> Result<i64, String> {
            let investigations = self.investigations.lock().unwrap();
            Ok(investigations
                .iter()
                .filter(|i| status.is_none() || Some(i.status()) == status)
                .count() as i64)
        }
    }

    struct MockReportRepo {
        reports: Mutex<Vec<SuspicionReport>>,
    }

    impl MockReportRepo {
        fn new() -> Self {
            MockReportRepo {
                reports: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ISuspicionReportRepository for MockReportRepo {
        async fn save(&self, report: &SuspicionReport) -> Result<(), String> {
            let mut reports = self.reports.lock().unwrap();
            reports.retain(|r| r.id() != report.id());
            reports.push(report.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<SuspicionReport>, String> {
            let reports = self.reports.lock().unwrap();
            Ok(reports.iter().find(|r| r.id() == id).cloned())
        }
        async fn find_by_investigation_id(
            &self,
            inv_id: Uuid,
        ) -> Result<Option<SuspicionReport>, String> {
            let reports = self.reports.lock().unwrap();
            Ok(reports
                .iter()
                .find(|r| r.investigation_id() == inv_id)
                .cloned())
        }
    }

    struct MockFreezeRepo {
        freezes: Mutex<Vec<AssetFreeze>>,
    }

    impl MockFreezeRepo {
        fn new() -> Self {
            MockFreezeRepo {
                freezes: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAssetFreezeRepository for MockFreezeRepo {
        async fn save(&self, freeze: &AssetFreeze) -> Result<(), String> {
            let mut freezes = self.freezes.lock().unwrap();
            freezes.retain(|f| f.id() != freeze.id());
            freezes.push(freeze.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: Uuid) -> Result<Option<AssetFreeze>, String> {
            let freezes = self.freezes.lock().unwrap();
            Ok(freezes.iter().find(|f| f.id() == id).cloned())
        }
        async fn find_by_account_id(&self, account_id: Uuid) -> Result<Vec<AssetFreeze>, String> {
            let freezes = self.freezes.lock().unwrap();
            Ok(freezes
                .iter()
                .filter(|f| f.account_id() == account_id)
                .cloned()
                .collect())
        }
        async fn find_active_by_account_id(
            &self,
            account_id: Uuid,
        ) -> Result<Option<AssetFreeze>, String> {
            let freezes = self.freezes.lock().unwrap();
            Ok(freezes
                .iter()
                .find(|f| f.account_id() == account_id && f.is_active())
                .cloned())
        }
    }

    fn tnd(amount: f64) -> Money {
        Money::new(amount, Currency::TND).unwrap()
    }

    // --- Monitoring Service Tests ---

    #[tokio::test]
    async fn test_record_transaction_below_threshold() {
        let service = TransactionMonitoringService::new(
            Arc::new(MockTransactionRepo::new()),
            Arc::new(MockAlertRepo::new()),
            vec![Arc::new(ThresholdScenario)],
        );

        let (tx, alerts) = service
            .record_transaction(
                Uuid::new_v4(),
                Uuid::new_v4(),
                "John".to_string(),
                tnd(1000.0),
                TransactionType::Deposit,
                Direction::Inbound,
            )
            .await
            .unwrap();

        assert_eq!(tx.amount().amount(), 1000.0);
        assert!(alerts.is_empty());
    }

    #[tokio::test]
    async fn test_record_transaction_triggers_alert() {
        let service = TransactionMonitoringService::new(
            Arc::new(MockTransactionRepo::new()),
            Arc::new(MockAlertRepo::new()),
            vec![Arc::new(ThresholdScenario)],
        );

        let (_tx, alerts) = service
            .record_transaction(
                Uuid::new_v4(),
                Uuid::new_v4(),
                "John".to_string(),
                tnd(6000.0),
                TransactionType::Deposit,
                Direction::Inbound,
            )
            .await
            .unwrap();

        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].risk_level(), RiskLevel::Medium);
    }

    #[tokio::test]
    async fn test_record_transaction_invalid_amount() {
        let service = TransactionMonitoringService::new(
            Arc::new(MockTransactionRepo::new()),
            Arc::new(MockAlertRepo::new()),
            vec![],
        );

        let result = service
            .record_transaction(
                Uuid::new_v4(),
                Uuid::new_v4(),
                "John".to_string(),
                tnd(0.0),
                TransactionType::Deposit,
                Direction::Inbound,
            )
            .await;

        assert!(result.is_err());
    }

    // --- Investigation Service Tests ---

    #[tokio::test]
    async fn test_investigation_workflow() {
        let alert_repo = Arc::new(MockAlertRepo::new());
        let alert = Alert::new(
            TransactionId::new(),
            RiskLevel::High,
            "Test alert".to_string(),
        )
        .unwrap();
        let alert_id = alert.id();
        alert_repo.save(&alert).await.unwrap();

        let inv_service =
            InvestigationService::new(Arc::new(MockInvestigationRepo::new()), alert_repo.clone());

        // Open
        let inv = inv_service
            .open_investigation(alert_id, Some("analyst1".to_string()))
            .await
            .unwrap();
        assert_eq!(inv.status, "Open");
        let inv_id = Uuid::parse_str(&inv.id).unwrap();

        // Add note
        let inv = inv_service
            .add_note(inv_id, "Reviewing".to_string(), "analyst1".to_string())
            .await
            .unwrap();
        assert_eq!(inv.status, "InProgress");
        assert_eq!(inv.notes.len(), 1);

        // Escalate
        let inv = inv_service.escalate(inv_id).await.unwrap();
        assert_eq!(inv.status, "Escalated");

        // Close confirmed
        let inv = inv_service.close_confirmed(inv_id).await.unwrap();
        assert_eq!(inv.status, "ClosedConfirmed");
    }

    // --- DOS Report Service Tests ---

    #[tokio::test]
    async fn test_generate_and_submit_report() {
        let report_repo = Arc::new(MockReportRepo::new());
        let service = DosReportService::new(report_repo);

        let report = service
            .generate_report(
                Uuid::new_v4(),
                "Customer X".to_string(),
                "TX details".to_string(),
                "Suspicious transfers".to_string(),
                Some("Evidence docs".to_string()),
                None,
            )
            .await
            .unwrap();

        assert_eq!(report.status, "Draft");
        let report_id = Uuid::parse_str(&report.id).unwrap();

        let submitted = service.submit_to_ctaf(report_id).await.unwrap();
        assert_eq!(submitted.status, "Submitted");
        assert!(submitted.submitted_at.is_some());
    }

    // --- Asset Freeze Service Tests (INV-09) ---

    struct MockAccountFreezePort {
        freezes: Mutex<std::collections::HashMap<Uuid, bool>>,
    }

    impl MockAccountFreezePort {
        fn new() -> Self {
            MockAccountFreezePort {
                freezes: Mutex::new(std::collections::HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl IAccountFreezePort for MockAccountFreezePort {
        async fn find_account_by_id(&self, _account_id: Uuid) -> Result<Option<Account>, String> {
            // For tests, return None — actual implementation will use real Account service
            Ok(None)
        }

        async fn freeze_account(&self, account_id: Uuid) -> Result<Account, String> {
            let mut freezes = self.freezes.lock().unwrap();
            freezes.insert(account_id, true);
            // For tests, return a mock Account (would be real in integration tests)
            Err("Mock freeze — use real Account service in production".to_string())
        }

        async fn unfreeze_account(&self, account_id: Uuid) -> Result<Account, String> {
            let mut freezes = self.freezes.lock().unwrap();
            freezes.insert(account_id, false);
            // For tests, return a mock Account (would be real in integration tests)
            Err("Mock unfreeze — use real Account service in production".to_string())
        }
    }

    #[tokio::test]
    async fn test_freeze_account_immediate() {
        let service = AssetFreezeService::new(Arc::new(MockFreezeRepo::new()));
        let account_id = Uuid::new_v4();

        let freeze = service
            .freeze_account(
                account_id,
                "Suspicious".to_string(),
                "supervisor".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(freeze.status, "Active");

        let is_frozen = service.is_account_frozen(account_id).await.unwrap();
        assert!(is_frozen);
    }

    #[tokio::test]
    async fn test_freeze_and_lift() {
        let service = AssetFreezeService::new(Arc::new(MockFreezeRepo::new()));
        let account_id = Uuid::new_v4();

        let freeze = service
            .freeze_account(
                account_id,
                "Suspicious".to_string(),
                "supervisor".to_string(),
            )
            .await
            .unwrap();

        let freeze_id = Uuid::parse_str(&freeze.id).unwrap();
        let lifted = service
            .lift_freeze(freeze_id, "CTAF_officer".to_string())
            .await
            .unwrap();
        assert_eq!(lifted.status, "Lifted");
        assert_eq!(lifted.lifted_by, Some("CTAF_officer".to_string()));
    }

    #[tokio::test]
    async fn test_freeze_account_with_port_integration() {
        let freeze_repo = Arc::new(MockFreezeRepo::new());
        let account_port = Arc::new(MockAccountFreezePort::new());
        let service = AssetFreezeService::with_account_port(freeze_repo, account_port.clone());
        let account_id = Uuid::new_v4();

        // Freeze account — account port would be called but returns error in mock
        // In integration tests, it would actually freeze the account
        let freeze = service
            .freeze_account(
                account_id,
                "Suspicious".to_string(),
                "supervisor".to_string(),
            )
            .await;

        // Even though account port fails in mock, freeze record should be saved
        assert!(freeze.is_ok());
        let is_frozen = service.is_account_frozen(account_id).await.unwrap();
        assert!(is_frozen);
    }
}
