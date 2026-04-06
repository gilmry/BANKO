use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use banko_domain::shared::CustomerId;

/// Mobile-friendly account summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MobileAccountSummary {
    pub id: String,
    pub name: String,
    pub balance: Decimal,
    pub currency: String,
    pub account_type: String,
    pub last_tx_amount: Option<Decimal>,
    pub last_tx_date: Option<DateTime<Utc>>,
}

/// Mobile-friendly card summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MobileCardSummary {
    pub id: String,
    pub masked_pan: String,
    pub card_type: String,
    pub status: String,
    pub daily_remaining: Decimal,
}

/// Pending action for customer
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingAction {
    pub action_type: String,
    pub description: String,
    pub action_url: String,
}

/// Mobile dashboard - aggregated view
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MobileDashboard {
    pub customer_name: String,
    pub greeting: String,
    pub total_balance_tnd: Decimal,
    pub accounts: Vec<MobileAccountSummary>,
    pub cards: Vec<MobileCardSummary>,
    pub pending_actions: Vec<PendingAction>,
    pub unread_notifications: u32,
}

/// Transaction for offline sync
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OfflineTransaction {
    pub id: String,
    pub account_id: String,
    pub description: String,
    pub amount: Decimal,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_type: String,
}

/// Offline cache data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OfflineCacheData {
    pub accounts: Vec<MobileAccountSummary>,
    pub recent_transactions: Vec<OfflineTransaction>,
    pub cards: Vec<MobileCardSummary>,
    pub cached_at: DateTime<Utc>,
    pub cache_ttl_hours: i32,
}

/// Balance update for sync
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BalanceUpdate {
    pub account_id: String,
    pub new_balance: Decimal,
    pub currency: String,
    pub updated_at: DateTime<Utc>,
}

/// Notification for sync
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncNotification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub created_at: DateTime<Utc>,
    pub is_read: bool,
}

/// Sync response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncResponse {
    pub new_transactions: Vec<OfflineTransaction>,
    pub balance_updates: Vec<BalanceUpdate>,
    pub notifications: Vec<SyncNotification>,
    pub server_time: DateTime<Utc>,
}

/// Errors for mobile account service
#[derive(Debug, thiserror::Error)]
pub enum MobileAccountError {
    #[error("Customer not found")]
    CustomerNotFound,

    #[error("No accounts found")]
    NoAccountsFound,

    #[error("Invalid locale: {0}")]
    InvalidLocale(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Port for mobile dashboard data provider
#[async_trait]
pub trait IMobileDashboardProvider: Send + Sync {
    async fn get_customer_name(&self, customer_id: &Uuid) -> Result<String, String>;
    async fn get_total_balance(&self, customer_id: &Uuid) -> Result<Decimal, String>;
    async fn get_account_summaries(&self, customer_id: &Uuid) -> Result<Vec<MobileAccountSummary>, String>;
    async fn get_card_summaries(&self, customer_id: &Uuid) -> Result<Vec<MobileCardSummary>, String>;
    async fn get_pending_actions(&self, customer_id: &Uuid) -> Result<Vec<PendingAction>, String>;
    async fn get_unread_notification_count(&self, customer_id: &Uuid) -> Result<u32, String>;
    async fn get_offline_cache_data(&self, customer_id: &Uuid) -> Result<OfflineCacheData, String>;
    async fn get_sync_changes(
        &self,
        customer_id: &Uuid,
        last_sync: DateTime<Utc>,
    ) -> Result<SyncResponse, String>;
}

/// Mobile Account Service
pub struct MobileAccountService {
    dashboard_provider: Arc<dyn IMobileDashboardProvider>,
}

impl MobileAccountService {
    pub fn new(dashboard_provider: Arc<dyn IMobileDashboardProvider>) -> Self {
        MobileAccountService { dashboard_provider }
    }

    /// Get mobile dashboard - optimized single call (reduces HTTP roundtrips)
    pub async fn get_mobile_dashboard(
        &self,
        customer_id: Uuid,
        locale: &str,
    ) -> Result<MobileDashboard, MobileAccountError> {
        // Validate locale
        if !["en", "fr", "ar"].contains(&locale) {
            return Err(MobileAccountError::InvalidLocale(locale.to_string()));
        }

        let customer_name = self
            .dashboard_provider
            .get_customer_name(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))?;

        let total_balance = self
            .dashboard_provider
            .get_total_balance(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))?;

        let accounts = self
            .dashboard_provider
            .get_account_summaries(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))?;

        if accounts.is_empty() {
            return Err(MobileAccountError::NoAccountsFound);
        }

        let cards = self
            .dashboard_provider
            .get_card_summaries(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))?;

        let pending_actions = self
            .dashboard_provider
            .get_pending_actions(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))?;

        let unread_notifications = self
            .dashboard_provider
            .get_unread_notification_count(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))?;

        let greeting = match locale {
            "en" => self.get_greeting_en(&customer_name),
            "fr" => self.get_greeting_fr(&customer_name),
            "ar" => self.get_greeting_ar(&customer_name),
            _ => format!("Hello, {}", customer_name),
        };

        Ok(MobileDashboard {
            customer_name,
            greeting,
            total_balance_tnd: total_balance,
            accounts,
            cards,
            pending_actions,
            unread_notifications,
        })
    }

    /// Get offline cache data (minimal, for offline mode)
    pub async fn get_offline_cache_data(
        &self,
        customer_id: Uuid,
    ) -> Result<OfflineCacheData, MobileAccountError> {
        self.dashboard_provider
            .get_offline_cache_data(&customer_id)
            .await
            .map_err(|e| MobileAccountError::Internal(e))
    }

    /// Sync changes since last sync
    pub async fn sync_changes(
        &self,
        customer_id: Uuid,
        last_sync: DateTime<Utc>,
    ) -> Result<SyncResponse, MobileAccountError> {
        self.dashboard_provider
            .get_sync_changes(&customer_id, last_sync)
            .await
            .map_err(|e| MobileAccountError::Internal(e))
    }

    fn get_greeting_en(&self, name: &str) -> String {
        let hour = Utc::now().hour();
        let greeting = match hour {
            5..=11 => "Good morning",
            12..=17 => "Good afternoon",
            18..=21 => "Good evening",
            _ => "Hello",
        };
        format!("{}, {}!", greeting, name)
    }

    fn get_greeting_fr(&self, name: &str) -> String {
        let hour = Utc::now().hour();
        let greeting = match hour {
            5..=11 => "Bonjour",
            12..=17 => "Bon après-midi",
            18..=21 => "Bonsoir",
            _ => "Bonsoir",
        };
        format!("{}, {}!", greeting, name)
    }

    fn get_greeting_ar(&self, name: &str) -> String {
        let hour = Utc::now().hour();
        let greeting = match hour {
            5..=11 => "صباح الخير",
            12..=17 => "مساء الخير",
            18..=21 => "مساء الخير",
            _ => "السلام عليكم",
        };
        format!("{} {}", greeting, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDashboardProvider {
        customer_name: String,
        total_balance: Decimal,
    }

    #[async_trait]
    impl IMobileDashboardProvider for MockDashboardProvider {
        async fn get_customer_name(&self, _customer_id: &Uuid) -> Result<String, String> {
            Ok(self.customer_name.clone())
        }

        async fn get_total_balance(&self, _customer_id: &Uuid) -> Result<Decimal, String> {
            Ok(self.total_balance)
        }

        async fn get_account_summaries(
            &self,
            _customer_id: &Uuid,
        ) -> Result<Vec<MobileAccountSummary>, String> {
            Ok(vec![MobileAccountSummary {
                id: "acc123".to_string(),
                name: "Checking".to_string(),
                balance: Decimal::new(5000, 2),
                currency: "TND".to_string(),
                account_type: "Checking".to_string(),
                last_tx_amount: Some(Decimal::new(10000, 2)),
                last_tx_date: Some(Utc::now()),
            }])
        }

        async fn get_card_summaries(
            &self,
            _customer_id: &Uuid,
        ) -> Result<Vec<MobileCardSummary>, String> {
            Ok(vec![MobileCardSummary {
                id: "card123".to_string(),
                masked_pan: "****1234".to_string(),
                card_type: "Debit".to_string(),
                status: "Active".to_string(),
                daily_remaining: Decimal::new(50000, 2),
            }])
        }

        async fn get_pending_actions(
            &self,
            _customer_id: &Uuid,
        ) -> Result<Vec<PendingAction>, String> {
            Ok(vec![])
        }

        async fn get_unread_notification_count(&self, _customer_id: &Uuid) -> Result<u32, String> {
            Ok(0)
        }

        async fn get_offline_cache_data(&self, _customer_id: &Uuid) -> Result<OfflineCacheData, String> {
            Ok(OfflineCacheData {
                accounts: vec![],
                recent_transactions: vec![],
                cards: vec![],
                cached_at: Utc::now(),
                cache_ttl_hours: 24,
            })
        }

        async fn get_sync_changes(
            &self,
            _customer_id: &Uuid,
            _last_sync: DateTime<Utc>,
        ) -> Result<SyncResponse, String> {
            Ok(SyncResponse {
                new_transactions: vec![],
                balance_updates: vec![],
                notifications: vec![],
                server_time: Utc::now(),
            })
        }
    }

    #[tokio::test]
    async fn test_get_mobile_dashboard_success() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "John Doe".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        let customer_id = Uuid::new_v4();

        let result = service.get_mobile_dashboard(customer_id, "en").await;
        assert!(result.is_ok());

        let dashboard = result.unwrap();
        assert_eq!(dashboard.customer_name, "John Doe");
        assert_eq!(dashboard.total_balance_tnd, Decimal::new(5000, 2));
        assert!(!dashboard.accounts.is_empty());
    }

    #[tokio::test]
    async fn test_get_mobile_dashboard_invalid_locale() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "John Doe".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        let customer_id = Uuid::new_v4();

        let result = service.get_mobile_dashboard(customer_id, "es").await;
        assert!(matches!(result, Err(MobileAccountError::InvalidLocale(_))));
    }

    #[tokio::test]
    async fn test_greeting_english() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "John".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        assert!(service.get_greeting_en("John").contains("John"));
    }

    #[tokio::test]
    async fn test_greeting_french() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "Jean".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        assert!(service.get_greeting_fr("Jean").contains("Jean"));
    }

    #[tokio::test]
    async fn test_greeting_arabic() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "محمد".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        assert!(service.get_greeting_ar("محمد").contains("محمد"));
    }

    #[tokio::test]
    async fn test_get_offline_cache_data() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "John Doe".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        let customer_id = Uuid::new_v4();

        let result = service.get_offline_cache_data(customer_id).await;
        assert!(result.is_ok());

        let cache = result.unwrap();
        assert_eq!(cache.cache_ttl_hours, 24);
    }

    #[tokio::test]
    async fn test_sync_changes() {
        let provider = Arc::new(MockDashboardProvider {
            customer_name: "John Doe".to_string(),
            total_balance: Decimal::new(5000, 2),
        });

        let service = MobileAccountService::new(provider);
        let customer_id = Uuid::new_v4();
        let last_sync = Utc::now() - chrono::Duration::hours(1);

        let result = service.sync_changes(customer_id, last_sync).await;
        assert!(result.is_ok());
    }
}
