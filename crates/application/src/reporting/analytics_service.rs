use std::sync::Arc;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use super::ports::*;
use super::errors::ReportingServiceError;

// ============================================================
// Data Structures for Client Portfolio (STORY-BI-01)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionDirection {
    Credit,
    Debit,
}

impl std::fmt::Display for TransactionDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionDirection::Credit => write!(f, "Credit"),
            TransactionDirection::Debit => write!(f, "Debit"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub id: String,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub direction: TransactionDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    pub account_id: String,
    pub account_type: String,
    pub balance: Decimal,
    pub currency: String,
    pub interest_earned_ytd: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSummary {
    pub card_id: String,
    pub masked_pan: String,
    pub card_type: String,
    pub status: String,
    pub monthly_spent: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanSummary {
    pub loan_id: String,
    pub principal: Decimal,
    pub outstanding: Decimal,
    pub next_payment_date: NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPortfolio {
    pub customer_id: String,
    pub total_balance_tnd: Decimal,
    pub accounts: Vec<AccountSummary>,
    pub cards: Vec<CardSummary>,
    pub loans: Vec<LoanSummary>,
    pub fees_ytd: Decimal,
    pub recent_transactions: Vec<TransactionSummary>,
    pub consolidated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDrilldown {
    pub account_id: String,
    pub balance: Decimal,
    pub currency: String,
    pub interest_earned_ytd: Decimal,
    pub fees_paid_ytd: Decimal,
    pub last_10_transactions: Vec<TransactionSummary>,
    pub generated_at: DateTime<Utc>,
}

// ============================================================
// Data Structures for Operational KPIs (STORY-BI-02)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalKpis {
    pub total_customers: i64,
    pub active_customers_30d: i64,
    pub new_customers_30d: i64,
    pub attrition_rate: Decimal,
    pub total_accounts: i64,
    pub total_balance_tnd: Decimal,
    pub total_loans: i64,
    pub loans_outstanding_tnd: Decimal,
    pub npl_ratio: Decimal,
    pub fraud_alerts_today: i64,
    pub transactions_blocked_today: i64,
    pub blocked_amount_today: Decimal,
    pub aml_open_investigations: i64,
    pub compliance_score: Decimal,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDataPoint {
    pub date: NaiveDate,
    pub value: Decimal,
}

// ============================================================
// Data Structures for Configurable Report Builder (STORY-BI-03)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportType {
    Transactional,
    Compliance,
    Financial,
}

impl std::fmt::Display for ReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportType::Transactional => write!(f, "Transactional"),
            ReportType::Compliance => write!(f, "Compliance"),
            ReportType::Financial => write!(f, "Financial"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFormat {
    Pdf,
    Csv,
    Json,
    Excel,
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportFormat::Pdf => write!(f, "Pdf"),
            ReportFormat::Csv => write!(f, "Csv"),
            ReportFormat::Json => write!(f, "Json"),
            ReportFormat::Excel => write!(f, "Excel"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDefinition {
    pub id: String,
    pub name: String,
    pub report_type: ReportType,
    pub filters: serde_json::Value,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub format: ReportFormat,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportOutput {
    pub data: serde_json::Value,
    pub row_count: usize,
    pub generated_at: DateTime<Utc>,
}

// ============================================================
// Service Ports for Data Providers (STORY-BI-01, BI-02)
// ============================================================

use async_trait::async_trait;

#[async_trait]
pub trait IPortfolioDataProvider: Send + Sync {
    async fn get_customer_accounts(
        &self,
        customer_id: &str,
    ) -> Result<Vec<AccountSummary>, String>;

    async fn get_customer_cards(
        &self,
        customer_id: &str,
    ) -> Result<Vec<CardSummary>, String>;

    async fn get_customer_loans(
        &self,
        customer_id: &str,
    ) -> Result<Vec<LoanSummary>, String>;

    async fn get_recent_transactions(
        &self,
        customer_id: &str,
        limit: usize,
    ) -> Result<Vec<TransactionSummary>, String>;

    async fn get_ytd_fees(
        &self,
        customer_id: &str,
    ) -> Result<Decimal, String>;

    async fn get_total_balance(
        &self,
        customer_id: &str,
    ) -> Result<Decimal, String>;
}

#[async_trait]
pub trait IKpiDataProvider: Send + Sync {
    async fn count_total_customers(&self) -> Result<i64, String>;
    async fn count_active_customers_30d(&self) -> Result<i64, String>;
    async fn count_new_customers_30d(&self) -> Result<i64, String>;
    async fn get_attrition_rate(&self) -> Result<Decimal, String>;
    async fn count_total_accounts(&self) -> Result<i64, String>;
    async fn get_total_balance_tnd(&self) -> Result<Decimal, String>;
    async fn count_total_loans(&self) -> Result<i64, String>;
    async fn get_loans_outstanding_tnd(&self) -> Result<Decimal, String>;
    async fn get_npl_ratio(&self) -> Result<Decimal, String>;
    async fn count_fraud_alerts_today(&self) -> Result<i64, String>;
    async fn count_transactions_blocked_today(&self) -> Result<i64, String>;
    async fn get_blocked_amount_today(&self) -> Result<Decimal, String>;
    async fn count_aml_open_investigations(&self) -> Result<i64, String>;
    async fn get_compliance_score(&self) -> Result<Decimal, String>;
    async fn get_metric_trend(
        &self,
        metric: &str,
        days: u32,
    ) -> Result<Vec<TrendDataPoint>, String>;
}

#[async_trait]
pub trait IReportDefinitionRepository: Send + Sync {
    async fn save(&self, report: &ReportDefinition) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<ReportDefinition>, String>;
    async fn list_all(&self) -> Result<Vec<ReportDefinition>, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

// ============================================================
// AnalyticsService (STORY-BI-01 & BI-02)
// ============================================================

pub struct AnalyticsService {
    portfolio_provider: Arc<dyn IPortfolioDataProvider>,
    kpi_provider: Arc<dyn IKpiDataProvider>,
}

impl AnalyticsService {
    pub fn new(
        portfolio_provider: Arc<dyn IPortfolioDataProvider>,
        kpi_provider: Arc<dyn IKpiDataProvider>,
    ) -> Self {
        AnalyticsService {
            portfolio_provider,
            kpi_provider,
        }
    }

    /// Get comprehensive client portfolio including accounts, cards, loans, and recent transactions
    pub async fn get_client_portfolio(
        &self,
        customer_id: &str,
    ) -> Result<ClientPortfolio, ReportingServiceError> {
        let accounts = self
            .portfolio_provider
            .get_customer_accounts(customer_id)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let cards = self
            .portfolio_provider
            .get_customer_cards(customer_id)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let loans = self
            .portfolio_provider
            .get_customer_loans(customer_id)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let recent_transactions = self
            .portfolio_provider
            .get_recent_transactions(customer_id, 50)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let fees_ytd: Decimal = self
            .portfolio_provider
            .get_ytd_fees(customer_id)
            .await
            .map_err(ReportingServiceError::Internal)?;

        let total_balance_tnd: Decimal = self
            .portfolio_provider
            .get_total_balance(customer_id)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(ClientPortfolio {
            customer_id: customer_id.to_string(),
            total_balance_tnd,
            accounts,
            cards,
            loans,
            fees_ytd,
            recent_transactions,
            consolidated_at: Utc::now(),
        })
    }

    /// Get detailed drilldown for a specific account including interest, fees, and transaction history
    pub async fn get_account_drilldown(
        &self,
        account_id: &str,
    ) -> Result<AccountDrilldown, ReportingServiceError> {
        // In production, these would be separate queries to dedicated repositories
        // For now, returning stub data with proper structure
        let account_drilldown = AccountDrilldown {
            account_id: account_id.to_string(),
            balance: Decimal::new(5000000, 2), // 50,000 TND
            currency: "TND".to_string(),
            interest_earned_ytd: Decimal::new(150000, 2), // 1,500 TND
            fees_paid_ytd: Decimal::new(50000, 2), // 500 TND
            last_10_transactions: vec![],
            generated_at: Utc::now(),
        };

        Ok(account_drilldown)
    }

    /// Get operational KPIs dashboard metrics
    pub async fn get_operational_kpis(&self) -> Result<OperationalKpis, ReportingServiceError> {
        let total_customers: i64 = self
            .kpi_provider
            .count_total_customers()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let active_customers_30d: i64 = self
            .kpi_provider
            .count_active_customers_30d()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let new_customers_30d: i64 = self
            .kpi_provider
            .count_new_customers_30d()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let attrition_rate: Decimal = self
            .kpi_provider
            .get_attrition_rate()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let total_accounts: i64 = self
            .kpi_provider
            .count_total_accounts()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let total_balance_tnd: Decimal = self
            .kpi_provider
            .get_total_balance_tnd()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let total_loans: i64 = self
            .kpi_provider
            .count_total_loans()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let loans_outstanding_tnd: Decimal = self
            .kpi_provider
            .get_loans_outstanding_tnd()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let npl_ratio: Decimal = self
            .kpi_provider
            .get_npl_ratio()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let fraud_alerts_today: i64 = self
            .kpi_provider
            .count_fraud_alerts_today()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let transactions_blocked_today: i64 = self
            .kpi_provider
            .count_transactions_blocked_today()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let blocked_amount_today: Decimal = self
            .kpi_provider
            .get_blocked_amount_today()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let aml_open_investigations: i64 = self
            .kpi_provider
            .count_aml_open_investigations()
            .await
            .map_err(ReportingServiceError::Internal)?;
        let compliance_score: Decimal = self
            .kpi_provider
            .get_compliance_score()
            .await
            .map_err(ReportingServiceError::Internal)?;

        let kpis = OperationalKpis {
            total_customers,
            active_customers_30d,
            new_customers_30d,
            attrition_rate,
            total_accounts,
            total_balance_tnd,
            total_loans,
            loans_outstanding_tnd,
            npl_ratio,
            fraud_alerts_today,
            transactions_blocked_today,
            blocked_amount_today,
            aml_open_investigations,
            compliance_score,
            computed_at: Utc::now(),
        };

        Ok(kpis)
    }

    /// Get trend data for a specific metric over time
    pub async fn get_trend_data(
        &self,
        metric: &str,
        days: u32,
    ) -> Result<Vec<TrendDataPoint>, ReportingServiceError> {
        self.kpi_provider
            .get_metric_trend(metric, days)
            .await
            .map_err(ReportingServiceError::Internal)
    }
}

// ============================================================
// ReportBuilderService (STORY-BI-03)
// ============================================================

pub struct ReportBuilderService {
    definition_repo: Arc<dyn IReportDefinitionRepository>,
}

impl ReportBuilderService {
    pub fn new(definition_repo: Arc<dyn IReportDefinitionRepository>) -> Self {
        ReportBuilderService { definition_repo }
    }

    /// Create a new report definition
    pub async fn create_report_definition(
        &self,
        name: String,
        report_type: ReportType,
        filters: serde_json::Value,
        date_from: Option<NaiveDate>,
        date_to: Option<NaiveDate>,
        format: ReportFormat,
        created_by: String,
    ) -> Result<ReportDefinition, ReportingServiceError> {
        let definition = ReportDefinition {
            id: Uuid::new_v4().to_string(),
            name,
            report_type,
            filters,
            date_from,
            date_to,
            format,
            created_by,
            created_at: Utc::now(),
        };

        self.definition_repo
            .save(&definition)
            .await
            .map_err(ReportingServiceError::Internal)?;

        Ok(definition)
    }

    /// Execute a report and return output
    pub async fn execute_report(
        &self,
        report_id: &str,
    ) -> Result<ReportOutput, ReportingServiceError> {
        let definition = self
            .definition_repo
            .find_by_id(report_id)
            .await
            .map_err(ReportingServiceError::Internal)?
            .ok_or(ReportingServiceError::InvalidInput(
                "Report definition not found".to_string(),
            ))?;

        // In production, aggregate data based on filters
        let data = serde_json::json!({
            "report_type": definition.report_type.to_string(),
            "filters": definition.filters,
            "date_from": definition.date_from,
            "date_to": definition.date_to,
            "rows": []
        });

        Ok(ReportOutput {
            data,
            row_count: 0,
            generated_at: Utc::now(),
        })
    }

    /// List all report definitions
    pub async fn list_report_definitions(&self) -> Result<Vec<ReportDefinition>, ReportingServiceError> {
        self.definition_repo
            .list_all()
            .await
            .map_err(ReportingServiceError::Internal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPortfolioProvider {
        should_fail: bool,
    }

    #[async_trait]
    impl IPortfolioDataProvider for MockPortfolioProvider {
        async fn get_customer_accounts(
            &self,
            _customer_id: &str,
        ) -> Result<Vec<AccountSummary>, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(vec![AccountSummary {
                    account_id: "ACC001".to_string(),
                    account_type: "Checking".to_string(),
                    balance: Decimal::new(1000000, 2),
                    currency: "TND".to_string(),
                    interest_earned_ytd: Decimal::new(50000, 2),
                }])
            }
        }

        async fn get_customer_cards(
            &self,
            _customer_id: &str,
        ) -> Result<Vec<CardSummary>, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(vec![CardSummary {
                    card_id: "CARD001".to_string(),
                    masked_pan: "****1234".to_string(),
                    card_type: "Visa".to_string(),
                    status: "Active".to_string(),
                    monthly_spent: Decimal::new(500000, 2),
                }])
            }
        }

        async fn get_customer_loans(
            &self,
            _customer_id: &str,
        ) -> Result<Vec<LoanSummary>, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(vec![LoanSummary {
                    loan_id: "LOAN001".to_string(),
                    principal: Decimal::new(5000000, 2),
                    outstanding: Decimal::new(4000000, 2),
                    next_payment_date: NaiveDate::from_ymd_opt(2026, 5, 6).unwrap(),
                    status: "Active".to_string(),
                }])
            }
        }

        async fn get_recent_transactions(
            &self,
            _customer_id: &str,
            _limit: usize,
        ) -> Result<Vec<TransactionSummary>, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(vec![])
            }
        }

        async fn get_ytd_fees(&self, _customer_id: &str) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(25000, 2))
            }
        }

        async fn get_total_balance(&self, _customer_id: &str) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(5000000, 2))
            }
        }
    }

    struct MockKpiProvider {
        should_fail: bool,
    }

    #[async_trait]
    impl IKpiDataProvider for MockKpiProvider {
        async fn count_total_customers(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(10000)
            }
        }

        async fn count_active_customers_30d(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(8500)
            }
        }

        async fn count_new_customers_30d(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(500)
            }
        }

        async fn get_attrition_rate(&self) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(250, 2)) // 2.5%
            }
        }

        async fn count_total_accounts(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(15000)
            }
        }

        async fn get_total_balance_tnd(&self) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(150000000000, 2)) // 1.5B TND
            }
        }

        async fn count_total_loans(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(3000)
            }
        }

        async fn get_loans_outstanding_tnd(&self) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(75000000000, 2)) // 750M TND
            }
        }

        async fn get_npl_ratio(&self) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(300, 2)) // 3%
            }
        }

        async fn count_fraud_alerts_today(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(15)
            }
        }

        async fn count_transactions_blocked_today(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(5)
            }
        }

        async fn get_blocked_amount_today(&self) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(50000000, 2)) // 500K TND
            }
        }

        async fn count_aml_open_investigations(&self) -> Result<i64, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(12)
            }
        }

        async fn get_compliance_score(&self) -> Result<Decimal, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                Ok(Decimal::new(9500, 2)) // 95%
            }
        }

        async fn get_metric_trend(
            &self,
            metric: &str,
            days: u32,
        ) -> Result<Vec<TrendDataPoint>, String> {
            if self.should_fail {
                Err("Provider error".to_string())
            } else {
                let mut trends = Vec::new();
                for i in 0..days {
                    trends.push(TrendDataPoint {
                        date: NaiveDate::from_ymd_opt(2026, 4, 6)
                            .unwrap()
                            .pred_opt()
                            .unwrap(),
                        value: Decimal::new(1000 + i as i64 * 10, 0),
                    });
                }
                Ok(trends)
            }
        }
    }

    struct MockReportRepository;

    #[async_trait]
    impl IReportDefinitionRepository for MockReportRepository {
        async fn save(&self, _report: &ReportDefinition) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ReportDefinition>, String> {
            Ok(Some(ReportDefinition {
                id: id.to_string(),
                name: "Test Report".to_string(),
                report_type: ReportType::Compliance,
                filters: serde_json::json!({}),
                date_from: Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
                date_to: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
                format: ReportFormat::Json,
                created_by: "admin".to_string(),
                created_at: Utc::now(),
            }))
        }

        async fn list_all(&self) -> Result<Vec<ReportDefinition>, String> {
            Ok(vec![])
        }

        async fn delete(&self, _id: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_get_client_portfolio() {
        let portfolio_provider = Arc::new(MockPortfolioProvider { should_fail: false });
        let kpi_provider = Arc::new(MockKpiProvider { should_fail: false });
        let analytics = AnalyticsService::new(portfolio_provider, kpi_provider);

        let portfolio = analytics.get_client_portfolio("CUST001").await;
        assert!(portfolio.is_ok());

        let p = portfolio.unwrap();
        assert_eq!(p.customer_id, "CUST001");
        assert!(!p.accounts.is_empty());
        assert!(!p.cards.is_empty());
        assert!(!p.loans.is_empty());
    }

    #[tokio::test]
    async fn test_get_client_portfolio_provider_error() {
        let portfolio_provider = Arc::new(MockPortfolioProvider { should_fail: true });
        let kpi_provider = Arc::new(MockKpiProvider { should_fail: false });
        let analytics = AnalyticsService::new(portfolio_provider, kpi_provider);

        let portfolio = analytics.get_client_portfolio("CUST001").await;
        assert!(portfolio.is_err());
    }

    #[tokio::test]
    async fn test_get_account_drilldown() {
        let portfolio_provider = Arc::new(MockPortfolioProvider { should_fail: false });
        let kpi_provider = Arc::new(MockKpiProvider { should_fail: false });
        let analytics = AnalyticsService::new(portfolio_provider, kpi_provider);

        let drilldown = analytics.get_account_drilldown("ACC001").await;
        assert!(drilldown.is_ok());

        let d = drilldown.unwrap();
        assert_eq!(d.account_id, "ACC001");
        assert_eq!(d.currency, "TND");
    }

    #[tokio::test]
    async fn test_get_operational_kpis() {
        let portfolio_provider = Arc::new(MockPortfolioProvider { should_fail: false });
        let kpi_provider = Arc::new(MockKpiProvider { should_fail: false });
        let analytics = AnalyticsService::new(portfolio_provider, kpi_provider);

        let kpis = analytics.get_operational_kpis().await;
        assert!(kpis.is_ok());

        let k = kpis.unwrap();
        assert_eq!(k.total_customers, 10000);
        assert_eq!(k.total_accounts, 15000);
    }

    #[tokio::test]
    async fn test_get_operational_kpis_provider_error() {
        let portfolio_provider = Arc::new(MockPortfolioProvider { should_fail: false });
        let kpi_provider = Arc::new(MockKpiProvider { should_fail: true });
        let analytics = AnalyticsService::new(portfolio_provider, kpi_provider);

        let kpis = analytics.get_operational_kpis().await;
        assert!(kpis.is_err());
    }

    #[tokio::test]
    async fn test_get_trend_data() {
        let portfolio_provider = Arc::new(MockPortfolioProvider { should_fail: false });
        let kpi_provider = Arc::new(MockKpiProvider { should_fail: false });
        let analytics = AnalyticsService::new(portfolio_provider, kpi_provider);

        let trends = analytics.get_trend_data("active_customers", 30).await;
        assert!(trends.is_ok());
    }

    #[tokio::test]
    async fn test_create_report_definition() {
        let repo = Arc::new(MockReportRepository);
        let service = ReportBuilderService::new(repo);

        let definition = service
            .create_report_definition(
                "Monthly Report".to_string(),
                ReportType::Financial,
                serde_json::json!({}),
                Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
                Some(NaiveDate::from_ymd_opt(2026, 1, 31).unwrap()),
                ReportFormat::Csv,
                "admin".to_string(),
            )
            .await;

        assert!(definition.is_ok());
        let d = definition.unwrap();
        assert_eq!(d.name, "Monthly Report");
        assert_eq!(d.report_type, ReportType::Financial);
    }

    #[tokio::test]
    async fn test_execute_report() {
        let repo = Arc::new(MockReportRepository);
        let service = ReportBuilderService::new(repo);

        let output = service.execute_report("REPORT001").await;
        assert!(output.is_ok());

        let o = output.unwrap();
        assert_eq!(o.row_count, 0);
    }

    #[tokio::test]
    async fn test_list_report_definitions() {
        let repo = Arc::new(MockReportRepository);
        let service = ReportBuilderService::new(repo);

        let definitions = service.list_report_definitions().await;
        assert!(definitions.is_ok());
    }

    #[tokio::test]
    async fn test_report_type_display() {
        assert_eq!(ReportType::Transactional.to_string(), "Transactional");
        assert_eq!(ReportType::Compliance.to_string(), "Compliance");
        assert_eq!(ReportType::Financial.to_string(), "Financial");
    }

    #[tokio::test]
    async fn test_report_format_display() {
        assert_eq!(ReportFormat::Pdf.to_string(), "Pdf");
        assert_eq!(ReportFormat::Csv.to_string(), "Csv");
        assert_eq!(ReportFormat::Json.to_string(), "Json");
        assert_eq!(ReportFormat::Excel.to_string(), "Excel");
    }
}
