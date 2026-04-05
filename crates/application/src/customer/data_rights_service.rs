use std::sync::Arc;

use uuid::Uuid;

use banko_domain::customer::{DataRequestId, DataRequestType, DataRightsRequest};

use super::errors::CustomerServiceError;
use super::ports::{IConsentRepository, IDataRightsRepository};

pub struct DataRightsService {
    repo: Arc<dyn IDataRightsRepository>,
    consent_repo: Arc<dyn IConsentRepository>,
}

impl DataRightsService {
    pub fn new(
        repo: Arc<dyn IDataRightsRepository>,
        consent_repo: Arc<dyn IConsentRepository>,
    ) -> Self {
        DataRightsService { repo, consent_repo }
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

    fn make_service() -> DataRightsService {
        DataRightsService::new(
            Arc::new(MockDataRightsRepository::new()),
            Arc::new(MockConsentRepository::new()),
        )
    }

    fn make_service_with_consent(consent_repo: Arc<dyn IConsentRepository>) -> DataRightsService {
        DataRightsService::new(Arc::new(MockDataRightsRepository::new()), consent_repo)
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
