use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use banko_domain::shared::value_objects::CustomerId;

use super::ports::ICustomerRepository;

/// Minimum retention period in years after customer closure (INV-10, Loi 2015-26 art. 125).
const RETENTION_YEARS: u32 = 10;

// --- DTOs ---

#[derive(Debug, Clone, Serialize)]
pub struct RetentionStatus {
    pub customer_id: String,
    pub status: String,
    pub closed_at: Option<String>,
    pub retention_expired: bool,
    pub years_since_closure: Option<f64>,
    pub minimum_years: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnonymizationReport {
    pub count_checked: usize,
    pub count_anonymized: usize,
    pub errors: Vec<String>,
    pub run_at: String,
}

// --- Errors ---

#[derive(Debug, thiserror::Error)]
pub enum RetentionServiceError {
    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Domain error: {0}")]
    Domain(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// --- Service ---

pub struct RetentionService {
    customer_repo: Arc<dyn ICustomerRepository>,
}

impl RetentionService {
    pub fn new(customer_repo: Arc<dyn ICustomerRepository>) -> Self {
        RetentionService { customer_repo }
    }

    /// Check if a customer's retention period (10 years) has expired.
    pub fn is_retention_expired(closed_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        let years_since_closure = (now - closed_at).num_days() as f64 / 365.25;
        years_since_closure >= RETENTION_YEARS as f64
    }

    /// Run anonymization job: find all closed customers where closed_at > 10 years ago,
    /// anonymize them and persist.
    pub async fn run_anonymization_job(
        &self,
    ) -> Result<AnonymizationReport, RetentionServiceError> {
        let now = Utc::now();
        let cutoff = now - chrono::Duration::days((RETENTION_YEARS as i64) * 365 + 3); // slight buffer

        let eligible = self
            .customer_repo
            .find_closed_before(cutoff)
            .await
            .map_err(RetentionServiceError::Internal)?;

        let count_checked = eligible.len();
        let mut count_anonymized = 0;
        let mut errors = Vec::new();

        for mut customer in eligible {
            match customer.anonymize(now) {
                Ok(()) => {
                    if let Err(e) = self.customer_repo.save(&customer).await {
                        errors.push(format!(
                            "Failed to persist anonymized customer {}: {}",
                            customer.id(),
                            e
                        ));
                    } else {
                        count_anonymized += 1;
                        tracing::info!(
                            "INV-10: Anonymized customer {} (closed_at: {:?})",
                            customer.id(),
                            customer.closed_at()
                        );
                    }
                }
                Err(e) => {
                    errors.push(format!(
                        "Failed to anonymize customer {}: {}",
                        customer.id(),
                        e
                    ));
                }
            }
        }

        Ok(AnonymizationReport {
            count_checked,
            count_anonymized,
            errors,
            run_at: now.to_rfc3339(),
        })
    }

    /// Check retention status for a specific customer.
    pub async fn check_retention(
        &self,
        customer_id: Uuid,
    ) -> Result<RetentionStatus, RetentionServiceError> {
        let cid = CustomerId::from_uuid(customer_id);
        let customer = self
            .customer_repo
            .find_by_id(&cid)
            .await
            .map_err(RetentionServiceError::Internal)?
            .ok_or(RetentionServiceError::CustomerNotFound)?;

        let now = Utc::now();
        let (retention_expired, years_since_closure) = match customer.closed_at() {
            Some(closed_at) => {
                let years = (now - closed_at).num_days() as f64 / 365.25;
                (Self::is_retention_expired(closed_at, now), Some(years))
            }
            None => (false, None),
        };

        Ok(RetentionStatus {
            customer_id: customer.id().to_string(),
            status: customer.status().as_str().to_string(),
            closed_at: customer.closed_at().map(|d| d.to_rfc3339()),
            retention_expired,
            years_since_closure,
            minimum_years: RETENTION_YEARS,
        })
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::{Duration, Utc};

    use banko_domain::customer::*;
    use banko_domain::shared::value_objects::*;

    use super::super::ports::ICustomerRepository;
    use super::*;

    // --- Mock Repository ---

    struct MockRetentionRepo {
        customers: Mutex<Vec<Customer>>,
    }

    impl MockRetentionRepo {
        fn new() -> Self {
            MockRetentionRepo {
                customers: Mutex::new(Vec::new()),
            }
        }

        fn with_customers(customers: Vec<Customer>) -> Self {
            MockRetentionRepo {
                customers: Mutex::new(customers),
            }
        }
    }

    #[async_trait]
    impl ICustomerRepository for MockRetentionRepo {
        async fn save(&self, customer: &Customer) -> Result<(), String> {
            let mut customers = self.customers.lock().unwrap();
            customers.retain(|c| c.id() != customer.id());
            customers.push(customer.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &CustomerId) -> Result<Option<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers.iter().find(|c| c.id() == id).cloned())
        }

        async fn find_by_email(&self, _email: &EmailAddress) -> Result<Option<Customer>, String> {
            Ok(None)
        }

        async fn list_all(&self) -> Result<Vec<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers.clone())
        }

        async fn list_by_status(&self, status: &str) -> Result<Vec<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers
                .iter()
                .filter(|c| c.status().as_str() == status)
                .cloned()
                .collect())
        }

        async fn delete(&self, id: &CustomerId) -> Result<(), String> {
            let mut customers = self.customers.lock().unwrap();
            customers.retain(|c| c.id() != id);
            Ok(())
        }

        async fn find_closed_before(&self, before: DateTime<Utc>) -> Result<Vec<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers
                .iter()
                .filter(|c| {
                    c.status() == CustomerStatus::Closed
                        && c.closed_at().map_or(false, |ca| ca < before)
                })
                .cloned()
                .collect())
        }
    }

    fn make_closed_customer(closed_days_ago: i64) -> Customer {
        let closed_at = Utc::now() - Duration::days(closed_days_ago);
        let created_at = closed_at - Duration::days(365);

        let address = Address::new("10 Rue Test", "Tunis", "1000", "Tunisia").unwrap();
        let phone = PhoneNumber::new("+21698123456").unwrap();
        let email_value = format!("test-{}@example.com", uuid::Uuid::new_v4());
        let email = EmailAddress::new(&email_value).unwrap();

        let kyc = KycProfile::reconstitute(
            "Test User".to_string(),
            "12345678".to_string(),
            None,
            "Tunisia".to_string(),
            "Tester".to_string(),
            address,
            phone,
            email,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
            None,
            None,
            None,
        );

        Customer::reconstitute(
            CustomerId::new(),
            CustomerType::Individual,
            kyc,
            vec![],
            RiskScore::new(10).unwrap(),
            CustomerStatus::Closed,
            ConsentStatus::Given,
            created_at,
            closed_at,
            Some(closed_at),
        )
    }

    fn make_active_customer() -> Customer {
        let now = Utc::now();
        let address = Address::new("10 Rue Test", "Tunis", "1000", "Tunisia").unwrap();
        let phone = PhoneNumber::new("+21698123456").unwrap();
        let email_value = format!("active-{}@example.com", uuid::Uuid::new_v4());
        let email = EmailAddress::new(&email_value).unwrap();

        let kyc = KycProfile::reconstitute(
            "Active User".to_string(),
            "87654321".to_string(),
            None,
            "Tunisia".to_string(),
            "Worker".to_string(),
            address,
            phone,
            email,
            PepStatus::No,
            SourceOfFunds::Salary,
            None,
            None,
            None,
            None,
        );

        Customer::reconstitute(
            CustomerId::new(),
            CustomerType::Individual,
            kyc,
            vec![],
            RiskScore::new(10).unwrap(),
            CustomerStatus::Approved,
            ConsentStatus::Given,
            now,
            now,
            None,
        )
    }

    // --- Unit tests ---

    #[test]
    fn test_is_retention_expired_true() {
        let closed_at = Utc::now() - Duration::days(3660); // ~10.02 years
        assert!(RetentionService::is_retention_expired(
            closed_at,
            Utc::now()
        ));
    }

    #[test]
    fn test_is_retention_expired_false() {
        let closed_at = Utc::now() - Duration::days(365 * 5); // 5 years
        assert!(!RetentionService::is_retention_expired(
            closed_at,
            Utc::now()
        ));
    }

    #[test]
    fn test_is_retention_expired_boundary() {
        // Exactly 10 years (3652.5 days ~ 3653)
        let closed_at = Utc::now() - Duration::days(3653);
        assert!(RetentionService::is_retention_expired(
            closed_at,
            Utc::now()
        ));
    }

    // --- Service tests ---

    #[tokio::test]
    async fn test_run_anonymization_job_anonymizes_eligible() {
        let eligible = make_closed_customer(3660); // > 10 years
        let not_eligible = make_closed_customer(365 * 5); // 5 years
        let active = make_active_customer();

        let repo = Arc::new(MockRetentionRepo::with_customers(vec![
            eligible.clone(),
            not_eligible.clone(),
            active.clone(),
        ]));

        let service = RetentionService::new(repo.clone());
        let report = service.run_anonymization_job().await.unwrap();

        assert_eq!(report.count_checked, 1); // only the eligible one found
        assert_eq!(report.count_anonymized, 1);
        assert!(report.errors.is_empty());

        // Verify the eligible customer was anonymized in the repo
        let anon = repo.find_by_id(eligible.id()).await.unwrap().unwrap();
        assert!(anon.is_anonymized());
        assert_eq!(anon.kyc_profile().full_name(), "[ANONYMIZED]");

        // Verify the not-eligible customer was NOT anonymized
        let still_closed = repo.find_by_id(not_eligible.id()).await.unwrap().unwrap();
        assert_eq!(still_closed.status(), CustomerStatus::Closed);
        assert_ne!(still_closed.kyc_profile().full_name(), "[ANONYMIZED]");
    }

    #[tokio::test]
    async fn test_run_anonymization_job_empty() {
        let repo = Arc::new(MockRetentionRepo::new());
        let service = RetentionService::new(repo);
        let report = service.run_anonymization_job().await.unwrap();

        assert_eq!(report.count_checked, 0);
        assert_eq!(report.count_anonymized, 0);
        assert!(report.errors.is_empty());
    }

    #[tokio::test]
    async fn test_check_retention_closed_expired() {
        let customer = make_closed_customer(3660);
        let customer_uuid = *customer.id().as_uuid();
        let repo = Arc::new(MockRetentionRepo::with_customers(vec![customer]));
        let service = RetentionService::new(repo);

        let status = service.check_retention(customer_uuid).await.unwrap();
        assert!(status.retention_expired);
        assert_eq!(status.status, "Closed");
        assert!(status.closed_at.is_some());
        assert!(status.years_since_closure.unwrap() > 10.0);
    }

    #[tokio::test]
    async fn test_check_retention_closed_not_expired() {
        let customer = make_closed_customer(365 * 5);
        let customer_uuid = *customer.id().as_uuid();
        let repo = Arc::new(MockRetentionRepo::with_customers(vec![customer]));
        let service = RetentionService::new(repo);

        let status = service.check_retention(customer_uuid).await.unwrap();
        assert!(!status.retention_expired);
        assert_eq!(status.status, "Closed");
    }

    #[tokio::test]
    async fn test_check_retention_active_customer() {
        let customer = make_active_customer();
        let customer_uuid = *customer.id().as_uuid();
        let repo = Arc::new(MockRetentionRepo::with_customers(vec![customer]));
        let service = RetentionService::new(repo);

        let status = service.check_retention(customer_uuid).await.unwrap();
        assert!(!status.retention_expired);
        assert_eq!(status.status, "Approved");
        assert!(status.closed_at.is_none());
    }

    #[tokio::test]
    async fn test_check_retention_not_found() {
        let repo = Arc::new(MockRetentionRepo::new());
        let service = RetentionService::new(repo);

        let result = service.check_retention(uuid::Uuid::new_v4()).await;
        assert!(matches!(
            result,
            Err(RetentionServiceError::CustomerNotFound)
        ));
    }
}
