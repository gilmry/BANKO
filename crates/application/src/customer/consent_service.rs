use std::sync::Arc;

use uuid::Uuid;

use banko_domain::customer::{ConsentPurpose, ConsentRecord};

use super::errors::CustomerServiceError;
use super::ports::IConsentRepository;

pub struct ConsentService {
    repo: Arc<dyn IConsentRepository>,
}

impl ConsentService {
    pub fn new(repo: Arc<dyn IConsentRepository>) -> Self {
        ConsentService { repo }
    }

    /// Grant a consent for a specific purpose.
    pub async fn grant_consent(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<ConsentRecord, CustomerServiceError> {
        let purpose = ConsentPurpose::from_str_purpose(purpose)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        // Check if already active
        let existing = self
            .repo
            .find_by_customer_and_purpose(customer_id, purpose.as_str())
            .await
            .map_err(CustomerServiceError::Internal)?;

        if let Some(ref record) = existing {
            if record.is_active() {
                return Err(CustomerServiceError::Domain(
                    "Consent already active for this purpose".to_string(),
                ));
            }
        }

        let consent = ConsentRecord::grant(customer_id, purpose);
        self.repo
            .save(&consent)
            .await
            .map_err(CustomerServiceError::Internal)?;

        Ok(consent)
    }

    /// Revoke a consent for a specific purpose.
    pub async fn revoke_consent(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<ConsentRecord, CustomerServiceError> {
        let purpose = ConsentPurpose::from_str_purpose(purpose)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        let existing = self
            .repo
            .find_by_customer_and_purpose(customer_id, purpose.as_str())
            .await
            .map_err(CustomerServiceError::Internal)?;

        match existing {
            Some(mut record) => {
                record
                    .revoke()
                    .map_err(|e| CustomerServiceError::Domain(e.to_string()))?;
                self.repo
                    .save(&record)
                    .await
                    .map_err(CustomerServiceError::Internal)?;
                Ok(record)
            }
            None => Err(CustomerServiceError::Domain(
                "Consent not found".to_string(),
            )),
        }
    }

    /// Check if a consent is active for a specific purpose.
    pub async fn check_consent(
        &self,
        customer_id: Uuid,
        purpose: &str,
    ) -> Result<bool, CustomerServiceError> {
        let purpose = ConsentPurpose::from_str_purpose(purpose)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        let existing = self
            .repo
            .find_by_customer_and_purpose(customer_id, purpose.as_str())
            .await
            .map_err(CustomerServiceError::Internal)?;

        Ok(existing.map(|r| r.is_active()).unwrap_or(false))
    }

    /// List all consents for a customer.
    pub async fn list_consents(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<ConsentRecord>, CustomerServiceError> {
        self.repo
            .find_by_customer(customer_id)
            .await
            .map_err(CustomerServiceError::Internal)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use banko_domain::customer::{ConsentPurpose, ConsentRecord};

    use super::super::ports::IConsentRepository;
    use super::*;

    // --- Mock Repository ---

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

    fn make_service() -> ConsentService {
        ConsentService::new(Arc::new(MockConsentRepository::new()))
    }

    #[tokio::test]
    async fn test_grant_consent() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let consent = service.grant_consent(cid, "DataProcessing").await.unwrap();
        assert!(consent.is_active());
        assert_eq!(consent.purpose(), ConsentPurpose::DataProcessing);
    }

    #[tokio::test]
    async fn test_grant_consent_duplicate() {
        let service = make_service();
        let cid = Uuid::new_v4();
        service.grant_consent(cid, "Marketing").await.unwrap();
        let result = service.grant_consent(cid, "Marketing").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_revoke_consent() {
        let service = make_service();
        let cid = Uuid::new_v4();
        service.grant_consent(cid, "Analytics").await.unwrap();
        let revoked = service.revoke_consent(cid, "Analytics").await.unwrap();
        assert!(!revoked.is_active());
    }

    #[tokio::test]
    async fn test_revoke_consent_not_found() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let result = service.revoke_consent(cid, "Marketing").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_consent_active() {
        let service = make_service();
        let cid = Uuid::new_v4();
        service.grant_consent(cid, "Profiling").await.unwrap();
        let active = service.check_consent(cid, "Profiling").await.unwrap();
        assert!(active);
    }

    #[tokio::test]
    async fn test_check_consent_not_found() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let active = service.check_consent(cid, "Profiling").await.unwrap();
        assert!(!active);
    }

    #[tokio::test]
    async fn test_list_consents() {
        let service = make_service();
        let cid = Uuid::new_v4();
        service.grant_consent(cid, "DataProcessing").await.unwrap();
        service.grant_consent(cid, "Marketing").await.unwrap();
        let consents = service.list_consents(cid).await.unwrap();
        assert_eq!(consents.len(), 2);
    }

    #[tokio::test]
    async fn test_invalid_purpose() {
        let service = make_service();
        let cid = Uuid::new_v4();
        let result = service.grant_consent(cid, "InvalidPurpose").await;
        assert!(result.is_err());
    }
}
