use std::sync::Arc;

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use banko_domain::customer::{DataRequestId, DataRequestType, DataRightsRequest};
use banko_domain::shared::CustomerId;

use super::dto::DataExportPackage;
use super::errors::CustomerServiceError;
use super::ports::{
    IAccountDataProvider, IConsentRepository, ICustomerRepository, IDataRightsRepository,
};

pub struct DataRightsService {
    repo: Arc<dyn IDataRightsRepository>,
    consent_repo: Arc<dyn IConsentRepository>,
    customer_repo: Arc<dyn ICustomerRepository>,
    account_provider: Arc<dyn IAccountDataProvider>,
}

impl DataRightsService {
    pub fn new(
        repo: Arc<dyn IDataRightsRepository>,
        consent_repo: Arc<dyn IConsentRepository>,
        customer_repo: Arc<dyn ICustomerRepository>,
        account_provider: Arc<dyn IAccountDataProvider>,
    ) -> Self {
        DataRightsService {
            repo,
            consent_repo,
            customer_repo,
            account_provider,
        }
    }

    /// Request a data export (Access right).
    pub async fn request_data_export(
        &self,
        customer_id: Uuid,
    ) -> Result<DataRightsRequest, CustomerServiceError> {
        let request = DataRightsRequest::new(customer_id, DataRequestType::Access, None);
        self.repo
            .save(&request)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(request)
    }

    /// Request data rectification.
    pub async fn request_rectification(
        &self,
        customer_id: Uuid,
        details: String,
    ) -> Result<DataRightsRequest, CustomerServiceError> {
        let request =
            DataRightsRequest::new(customer_id, DataRequestType::Rectification, Some(details));
        self.repo
            .save(&request)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(request)
    }

    /// Request opposition to data processing. Also revokes the related consent if active.
    pub async fn request_opposition(
        &self,
        customer_id: Uuid,
        purpose: String,
    ) -> Result<DataRightsRequest, CustomerServiceError> {
        // Try to revoke consent for the given purpose
        let existing = self
            .consent_repo
            .find_by_customer_and_purpose(customer_id, &purpose)
            .await
            .map_err(CustomerServiceError::Internal)?;

        if let Some(mut consent) = existing {
            if consent.is_active() {
                let _ = consent.revoke();
                self.consent_repo
                    .save(&consent)
                    .await
                    .map_err(CustomerServiceError::Internal)?;
            }
        }

        let request = DataRightsRequest::new(
            customer_id,
            DataRequestType::Opposition,
            Some(format!("Opposition to purpose: {purpose}")),
        );
        self.repo
            .save(&request)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(request)
    }

    /// Get a specific data rights request by ID.
    pub async fn get_request(
        &self,
        request_id: &DataRequestId,
    ) -> Result<DataRightsRequest, CustomerServiceError> {
        self.repo
            .find_by_id(request_id)
            .await
            .map_err(CustomerServiceError::Internal)?
            .ok_or(CustomerServiceError::Domain(
                "Data rights request not found".to_string(),
            ))
    }

    /// List all data rights requests for a customer.
    pub async fn list_requests(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<DataRightsRequest>, CustomerServiceError> {
        self.repo
            .find_by_customer(customer_id)
            .await
            .map_err(CustomerServiceError::Internal)
    }

    /// Mark a request as processing (admin action).
    pub async fn process_request(
        &self,
        request_id: &DataRequestId,
    ) -> Result<DataRightsRequest, CustomerServiceError> {
        let mut request = self.get_request(request_id).await?;
        request
            .process()
            .map_err(|e| CustomerServiceError::Domain(e.to_string()))?;
        self.repo
            .save(&request)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(request)
    }

    /// Complete a request with a response (admin action).
    pub async fn complete_request(
        &self,
        request_id: &DataRequestId,
        response: String,
    ) -> Result<DataRightsRequest, CustomerServiceError> {
        let mut request = self.get_request(request_id).await?;
        request
            .complete(response)
            .map_err(|e| CustomerServiceError::Domain(e.to_string()))?;
        self.repo
            .save(&request)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(request)
    }

    /// Compile all customer data for export (GDPR data portability).
    /// Collects profile, KYC, accounts, transactions, and consents into a single package.
    pub async fn compile_customer_data_export(
        &self,
        customer_id: &CustomerId,
    ) -> Result<DataExportPackage, CustomerServiceError> {
        // Fetch customer profile
        let customer = self
            .customer_repo
            .find_by_id(customer_id)
            .await
            .map_err(CustomerServiceError::Internal)?
            .ok_or(CustomerServiceError::CustomerNotFound)?;

        // Fetch accounts
        let accounts = self
            .account_provider
            .find_accounts_by_customer(customer_id)
            .await
            .map_err(CustomerServiceError::Internal)?;

        // Fetch transactions
        let transactions = self
            .account_provider
            .find_transactions_by_customer(customer_id)
            .await
            .map_err(CustomerServiceError::Internal)?;

        // Fetch consents
        let consents = self
            .consent_repo
            .find_by_customer(*customer_id.as_uuid())
            .await
            .map_err(CustomerServiceError::Internal)?;

        let consents_json = serde_json::to_value(&consents)
            .map_err(|e| CustomerServiceError::Internal(format!("JSON serialization error: {}", e)))?;

        // Build profile JSON
        let kyc_address = customer.kyc_profile().address();
        let profile_json = json!({
            "customer_id": customer.id().to_string(),
            "customer_type": customer.customer_type().as_str(),
            "status": customer.status().as_str(),
            "kyc_profile": {
                "full_name": customer.kyc_profile().full_name(),
                "cin_or_rcs": customer.kyc_profile().cin_or_rcs(),
                "birth_date": customer.kyc_profile().birth_date().map(|d| d.to_string()),
                "nationality": customer.kyc_profile().nationality(),
                "profession": customer.kyc_profile().profession(),
                "address": {
                    "street": kyc_address.street(),
                    "city": kyc_address.city(),
                    "postal_code": kyc_address.postal_code(),
                    "country": kyc_address.country(),
                },
                "phone": customer.kyc_profile().phone().to_string(),
                "email": customer.kyc_profile().email().to_string(),
                "pep_status": customer.kyc_profile().pep_status().as_str(),
                "source_of_funds": customer.kyc_profile().source_of_funds().as_str(),
            },
            "risk_score": customer.risk_score().value(),
            "consent": customer.consent().as_str(),
            "created_at": customer.created_at().to_rfc3339(),
            "updated_at": customer.updated_at().to_rfc3339(),
            "closed_at": customer.closed_at().map(|d| d.to_rfc3339()),
        });

        // Build export package
        let export = DataExportPackage {
            customer_id: customer.id().to_string(),
            customer_type: customer.customer_type().as_str().to_string(),
            profile: profile_json,
            accounts: json!(accounts),
            transactions: json!(transactions),
            consents: consents_json,
            export_date: Utc::now().to_rfc3339(),
        };

        Ok(export)
    }

    /// Anonymize customer data by replacing PII fields with placeholder text.
    /// Keeps structural data intact for regulatory retention purposes.
    pub async fn anonymize_customer_data(
        &self,
        customer_id: &CustomerId,
    ) -> Result<serde_json::Value, CustomerServiceError> {
        let customer = self
            .customer_repo
            .find_by_id(customer_id)
            .await
            .map_err(CustomerServiceError::Internal)?
            .ok_or(CustomerServiceError::CustomerNotFound)?;

        let anonymized = json!({
            "customer_id": customer.id().to_string(),
            "customer_type": customer.customer_type().as_str(),
            "status": customer.status().as_str(),
            "kyc_profile": {
                "full_name": "[ANONYMIZED]",
                "cin_or_rcs": "[ANONYMIZED]",
                "birth_date": "[ANONYMIZED]",
                "nationality": customer.kyc_profile().nationality(), // Keep for regulatory purposes
                "profession": "[ANONYMIZED]",
                "address": {
                    "street": "[ANONYMIZED]",
                    "city": "[ANONYMIZED]",
                    "postal_code": "[ANONYMIZED]",
                    "country": customer.kyc_profile().nationality(), // Keep country for regulatory
                },
                "phone": "[ANONYMIZED]",
                "email": "[ANONYMIZED]",
                "pep_status": customer.kyc_profile().pep_status().as_str(), // Keep for AML/regulatory
                "source_of_funds": customer.kyc_profile().source_of_funds().as_str(), // Keep for regulatory
            },
            "risk_score": customer.risk_score().value(),
            "consent": "[ANONYMIZED]",
            "created_at": customer.created_at().to_rfc3339(),
            "updated_at": customer.updated_at().to_rfc3339(),
            "closed_at": customer.closed_at().map(|d| d.to_rfc3339()),
            "anonymized_at": Utc::now().to_rfc3339(),
        });

        Ok(anonymized)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use banko_domain::customer::{ConsentRecord, DataRequestId, DataRightsRequest};

    use super::super::ports::{IConsentRepository, IDataRightsRepository};
    use super::*;

    // --- Mock Data Rights Repository ---

    struct MockDataRightsRepository {
        requests: Mutex<Vec<DataRightsRequest>>,
    }

    impl MockDataRightsRepository {
        fn new() -> Self {
            MockDataRightsRepository {
                requests: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IDataRightsRepository for MockDataRightsRepository {
        async fn save(&self, request: &DataRightsRequest) -> Result<(), String> {
            let mut requests = self.requests.lock().unwrap();
            requests.retain(|r| r.request_id() != request.request_id());
            requests.push(request.clone());
            Ok(())
        }

        async fn find_by_id(
            &self,
            id: &DataRequestId,
        ) -> Result<Option<DataRightsRequest>, String> {
            let requests = self.requests.lock().unwrap();
            Ok(requests.iter().find(|r| r.request_id() == id).cloned())
        }

        async fn find_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Vec<DataRightsRequest>, String> {
            let requests = self.requests.lock().unwrap();
            Ok(requests
                .iter()
                .filter(|r| r.customer_id() == customer_id)
                .cloned()
                .collect())
        }
    }

    // --- Mock Consent Repository ---

    struct MockConsentRepository {
        consents: Mutex<Vec<ConsentRecord>>,
    }

    impl MockConsentRepository {
        fn new() -> Self {
            MockConsentRepository {
                consents: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IConsentRepository for MockConsentRepository {
        async fn save(&self, consent: &ConsentRecord) -> Result<(), String> {
            let mut consents = self.consents.lock().unwrap();
            consents.retain(|c| c.consent_id() != consent.consent_id());
            consents.push(consent.clone());
            Ok(())
        }

        async fn find_by_customer(&self, customer_id: Uuid) -> Result<Vec<ConsentRecord>, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents
                .iter()
                .filter(|c| c.customer_id() == customer_id)
                .cloned()
                .collect())
        }

        async fn find_by_customer_and_purpose(
            &self,
            customer_id: Uuid,
            purpose: &str,
        ) -> Result<Option<ConsentRecord>, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents
                .iter()
                .filter(|c| c.customer_id() == customer_id && c.purpose().as_str() == purpose)
                .filter(|c| c.is_active())
                .cloned()
                .last())
        }

        async fn find_active_by_customer(
            &self,
            customer_id: Uuid,
        ) -> Result<Vec<ConsentRecord>, String> {
            let consents = self.consents.lock().unwrap();
            Ok(consents
                .iter()
                .filter(|c| c.customer_id() == customer_id && c.is_active())
                .cloned()
                .collect())
        }
    }

    // --- Mock Customer Repository ---

    struct MockCustomerRepository {
        customers: Mutex<Vec<banko_domain::customer::Customer>>,
    }

    impl MockCustomerRepository {
        fn new() -> Self {
            MockCustomerRepository {
                customers: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ICustomerRepository for MockCustomerRepository {
        async fn save(
            &self,
            customer: &banko_domain::customer::Customer,
        ) -> Result<(), String> {
            let mut customers = self.customers.lock().unwrap();
            customers.retain(|c| c.id() != customer.id());
            customers.push(customer.clone());
            Ok(())
        }

        async fn find_by_id(
            &self,
            id: &CustomerId,
        ) -> Result<Option<banko_domain::customer::Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers.iter().find(|c| c.id() == id).cloned())
        }

        async fn find_by_email(
            &self,
            _email: &banko_domain::shared::value_objects::EmailAddress,
        ) -> Result<Option<banko_domain::customer::Customer>, String> {
            Ok(None)
        }

        async fn list_all(&self) -> Result<Vec<banko_domain::customer::Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers.clone())
        }

        async fn list_by_status(
            &self,
            _status: &str,
        ) -> Result<Vec<banko_domain::customer::Customer>, String> {
            Ok(Vec::new())
        }

        async fn delete(&self, id: &CustomerId) -> Result<(), String> {
            let mut customers = self.customers.lock().unwrap();
            customers.retain(|c| c.id() != id);
            Ok(())
        }

        async fn find_closed_before(
            &self,
            _before: chrono::DateTime<Utc>,
        ) -> Result<Vec<banko_domain::customer::Customer>, String> {
            Ok(Vec::new())
        }
        async fn search(
            &self,
            _full_name: Option<&str>,
            _email: Option<&str>,
            _cin_or_rcs: Option<&str>,
            _customer_type: Option<&str>,
            _status: Option<&str>,
            _segment: Option<&str>,
            _risk_score_min: Option<u8>,
            _risk_score_max: Option<u8>,
            _limit: i64,
            _offset: i64,
        ) -> Result<(i64, Vec<banko_domain::customer::Customer>), String> {
            Ok((0, vec![]))
        }
    }

    // --- Mock Account Data Provider ---

    struct MockAccountDataProvider {
        accounts: Mutex<Vec<serde_json::Value>>,
        transactions: Mutex<Vec<serde_json::Value>>,
    }

    impl MockAccountDataProvider {
        fn new() -> Self {
            MockAccountDataProvider {
                accounts: Mutex::new(Vec::new()),
                transactions: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IAccountDataProvider for MockAccountDataProvider {
        async fn find_accounts_by_customer(
            &self,
            _customer_id: &CustomerId,
        ) -> Result<Vec<serde_json::Value>, String> {
            let accounts = self.accounts.lock().unwrap();
            Ok(accounts.clone())
        }

        async fn find_transactions_by_customer(
            &self,
            _customer_id: &CustomerId,
        ) -> Result<Vec<serde_json::Value>, String> {
            let transactions = self.transactions.lock().unwrap();
            Ok(transactions.clone())
        }
    }

    fn make_service() -> DataRightsService {
        DataRightsService::new(
            Arc::new(MockDataRightsRepository::new()),
            Arc::new(MockConsentRepository::new()),
            Arc::new(MockCustomerRepository::new()),
            Arc::new(MockAccountDataProvider::new()),
        )
    }

    fn make_service_with_consent(consent_repo: Arc<dyn IConsentRepository>) -> DataRightsService {
        DataRightsService::new(
            Arc::new(MockDataRightsRepository::new()),
            consent_repo,
            Arc::new(MockCustomerRepository::new()),
            Arc::new(MockAccountDataProvider::new()),
        )
    }

    #[tokio::test]
    async fn test_request_data_export() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let request = service.request_data_export(cid).await.unwrap();
        assert_eq!(
            request.request_type(),
            banko_domain::customer::DataRequestType::Access
        );
        assert_eq!(
            request.status(),
            banko_domain::customer::DataRequestStatus::Pending
        );
    }

    #[tokio::test]
    async fn test_request_rectification() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let request = service
            .request_rectification(cid, "Update my address".to_string())
            .await
            .unwrap();
        assert_eq!(
            request.request_type(),
            banko_domain::customer::DataRequestType::Rectification
        );
        assert_eq!(request.details(), Some("Update my address"));
    }

    #[tokio::test]
    async fn test_request_opposition() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let request = service
            .request_opposition(cid, "Marketing".to_string())
            .await
            .unwrap();
        assert_eq!(
            request.request_type(),
            banko_domain::customer::DataRequestType::Opposition
        );
    }

    #[tokio::test]
    async fn test_request_opposition_revokes_consent() {
        let consent_repo = Arc::new(MockConsentRepository::new());
        let cid = Uuid::new_v4();

        // Grant a consent first
        let consent = ConsentRecord::grant(cid, banko_domain::customer::ConsentPurpose::Marketing);
        consent_repo.save(&consent).await.unwrap();

        let service = make_service_with_consent(consent_repo.clone());
        service
            .request_opposition(cid, "Marketing".to_string())
            .await
            .unwrap();

        // Consent should be revoked
        let active = consent_repo.find_active_by_customer(cid).await.unwrap();
        assert!(active.is_empty());
    }

    #[tokio::test]
    async fn test_get_request() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let request = service.request_data_export(cid).await.unwrap();
        let found = service.get_request(request.request_id()).await.unwrap();
        assert_eq!(found.request_id(), request.request_id());
    }

    #[tokio::test]
    async fn test_get_request_not_found() {
        let service = make_service();
        let result = service.get_request(&DataRequestId::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_requests() {
        let service = make_service();
        let cid = Uuid::new_v4();
        service.request_data_export(cid).await.unwrap();
        service
            .request_rectification(cid, "fix name".to_string())
            .await
            .unwrap();
        let requests = service.list_requests(cid).await.unwrap();
        assert_eq!(requests.len(), 2);
    }

    #[tokio::test]
    async fn test_process_request() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let request = service.request_data_export(cid).await.unwrap();
        let processed = service.process_request(request.request_id()).await.unwrap();
        assert_eq!(
            processed.status(),
            banko_domain::customer::DataRequestStatus::Processing
        );
    }

    #[tokio::test]
    async fn test_complete_request() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let request = service.request_data_export(cid).await.unwrap();
        service.process_request(request.request_id()).await.unwrap();
        let completed = service
            .complete_request(request.request_id(), "Here is your data".to_string())
            .await
            .unwrap();
        assert_eq!(
            completed.status(),
            banko_domain::customer::DataRequestStatus::Completed
        );
        assert_eq!(completed.response(), Some("Here is your data"));
    }
}
