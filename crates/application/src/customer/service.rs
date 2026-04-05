use std::sync::Arc;

use banko_domain::customer::{
    Address, Beneficiary, Cin, ConsentStatus, Customer, CustomerType, KycProfile, PepStatus,
    SourceOfFunds,
};
use banko_domain::shared::value_objects::{CustomerId, EmailAddress, PhoneNumber};

use super::dto::{CreateCustomerRequest, UpdateKycRequest};
use super::errors::CustomerServiceError;
use super::ports::{ICustomerRepository, IPepCheckService};
use super::risk_scoring::RiskScoringService;

pub struct CustomerService {
    repo: Arc<dyn ICustomerRepository>,
    pep_checker: Arc<dyn IPepCheckService>,
}

impl CustomerService {
    pub fn new(
        repo: Arc<dyn ICustomerRepository>,
        pep_checker: Arc<dyn IPepCheckService>,
    ) -> Self {
        CustomerService { repo, pep_checker }
    }

    pub async fn create_customer(
        &self,
        req: CreateCustomerRequest,
    ) -> Result<CustomerId, CustomerServiceError> {
        // Parse customer type
        let customer_type = CustomerType::from_str_type(&req.customer_type)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        // Parse consent
        let consent = if req.consent {
            ConsentStatus::Given
        } else {
            ConsentStatus::NotGiven
        };

        // Parse address
        let address = Address::new(
            &req.address.street,
            &req.address.city,
            &req.address.postal_code,
            req.address.country.as_deref().unwrap_or("Tunisia"),
        )
        .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        // Parse phone & email
        let phone = PhoneNumber::new(&req.phone)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;
        let email = EmailAddress::new(&req.email)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        // Check for duplicate email
        let existing = self
            .repo
            .find_by_email(&email)
            .await
            .map_err(CustomerServiceError::Internal)?;
        if existing.is_some() {
            return Err(CustomerServiceError::EmailAlreadyExists(
                req.email.clone(),
            ));
        }

        // Parse PEP and source of funds
        let mut pep_status = req
            .pep_status
            .as_deref()
            .map(PepStatus::from_str_status)
            .transpose()
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?
            .unwrap_or(PepStatus::Unknown);

        let source_of_funds = req
            .source_of_funds
            .as_deref()
            .map(SourceOfFunds::from_str_source)
            .transpose()
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?
            .unwrap_or(SourceOfFunds::Other);

        // STORY-C07: PEP check
        let is_pep = self
            .pep_checker
            .is_pep(&req.full_name)
            .await
            .map_err(CustomerServiceError::Internal)?;
        if is_pep {
            pep_status = PepStatus::Yes;
        }

        // Build KYC profile
        let kyc_profile = match customer_type {
            CustomerType::Individual => {
                let cin_str = req.cin.as_deref().unwrap_or("");
                let cin = Cin::new(cin_str)
                    .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

                let birth_date_str = req.birth_date.as_deref().unwrap_or("");
                let birth_date = chrono::NaiveDate::parse_from_str(birth_date_str, "%Y-%m-%d")
                    .map_err(|e| {
                        CustomerServiceError::Validation(format!("Invalid birth_date: {e}"))
                    })?;

                KycProfile::new_individual(
                    &req.full_name,
                    cin,
                    birth_date,
                    req.nationality.as_deref().unwrap_or("Tunisia"),
                    req.profession.as_deref().unwrap_or(""),
                    address,
                    phone,
                    email,
                    pep_status,
                    source_of_funds,
                )
                .map_err(|e| CustomerServiceError::Domain(e.to_string()))?
            }
            CustomerType::LegalEntity => {
                let rcs = req.registration_number.as_deref().unwrap_or("");
                let sector = req.sector.as_deref().unwrap_or("");

                KycProfile::new_legal_entity(
                    &req.full_name,
                    rcs,
                    sector,
                    address,
                    phone,
                    email,
                    pep_status,
                    source_of_funds,
                )
                .map_err(|e| CustomerServiceError::Domain(e.to_string()))?
            }
        };

        // Parse beneficiaries
        let beneficiaries = match &req.beneficiaries {
            Some(list) => list
                .iter()
                .map(|b| {
                    Beneficiary::new(&b.full_name, b.share_percentage)
                        .map_err(|e| CustomerServiceError::Validation(e.to_string()))
                })
                .collect::<Result<Vec<_>, _>>()?,
            None => vec![],
        };

        // Create customer aggregate
        let mut customer =
            Customer::new(customer_type, kyc_profile, beneficiaries, consent)
                .map_err(|e| CustomerServiceError::Domain(e.to_string()))?;

        // STORY-C08: Auto-calculate risk score
        let risk_score = RiskScoringService::calculate_risk_score(&customer);
        customer.update_risk_score(risk_score);

        let customer_id = customer.id().clone();

        // Persist
        self.repo
            .save(&customer)
            .await
            .map_err(CustomerServiceError::Internal)?;

        Ok(customer_id)
    }

    pub async fn find_by_id(
        &self,
        id: &CustomerId,
    ) -> Result<Customer, CustomerServiceError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(CustomerServiceError::Internal)?
            .ok_or(CustomerServiceError::CustomerNotFound)
    }

    pub async fn approve_kyc(
        &self,
        id: &CustomerId,
    ) -> Result<Customer, CustomerServiceError> {
        let mut customer = self.find_by_id(id).await?;
        customer.approve_kyc();
        self.repo
            .save(&customer)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(customer)
    }

    pub async fn reject_kyc(
        &self,
        id: &CustomerId,
        reason: String,
    ) -> Result<Customer, CustomerServiceError> {
        let mut customer = self.find_by_id(id).await?;
        customer.reject_kyc(reason);
        self.repo
            .save(&customer)
            .await
            .map_err(CustomerServiceError::Internal)?;
        Ok(customer)
    }

    pub async fn update_kyc(
        &self,
        id: &CustomerId,
        req: UpdateKycRequest,
    ) -> Result<Customer, CustomerServiceError> {
        let mut customer = self.find_by_id(id).await?;

        let address = Address::new(
            &req.address.street,
            &req.address.city,
            &req.address.postal_code,
            req.address.country.as_deref().unwrap_or("Tunisia"),
        )
        .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        let phone = PhoneNumber::new(&req.phone)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;
        let email = EmailAddress::new(&req.email)
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

        let pep_status = req
            .pep_status
            .as_deref()
            .map(PepStatus::from_str_status)
            .transpose()
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?
            .unwrap_or(customer.kyc_profile().pep_status());

        let source_of_funds = req
            .source_of_funds
            .as_deref()
            .map(SourceOfFunds::from_str_source)
            .transpose()
            .map_err(|e| CustomerServiceError::Validation(e.to_string()))?
            .unwrap_or(customer.kyc_profile().source_of_funds());

        let kyc_profile = match customer.customer_type() {
            CustomerType::Individual => {
                let cin_str = req.cin.as_deref().unwrap_or(customer.kyc_profile().cin_or_rcs());
                let cin = Cin::new(cin_str)
                    .map_err(|e| CustomerServiceError::Validation(e.to_string()))?;

                let birth_date = if let Some(ref bd) = req.birth_date {
                    chrono::NaiveDate::parse_from_str(bd, "%Y-%m-%d")
                        .map_err(|e| {
                            CustomerServiceError::Validation(format!("Invalid birth_date: {e}"))
                        })?
                } else {
                    customer
                        .kyc_profile()
                        .birth_date()
                        .unwrap_or(chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
                };

                KycProfile::new_individual(
                    &req.full_name,
                    cin,
                    birth_date,
                    req.nationality.as_deref().unwrap_or(customer.kyc_profile().nationality()),
                    req.profession.as_deref().unwrap_or(customer.kyc_profile().profession()),
                    address,
                    phone,
                    email,
                    pep_status,
                    source_of_funds,
                )
                .map_err(|e| CustomerServiceError::Domain(e.to_string()))?
            }
            CustomerType::LegalEntity => {
                let rcs = req
                    .registration_number
                    .as_deref()
                    .unwrap_or(customer.kyc_profile().cin_or_rcs());
                let sector = req
                    .sector
                    .as_deref()
                    .unwrap_or(customer.kyc_profile().sector().unwrap_or(""));

                KycProfile::new_legal_entity(
                    &req.full_name,
                    rcs,
                    sector,
                    address,
                    phone,
                    email,
                    pep_status,
                    source_of_funds,
                )
                .map_err(|e| CustomerServiceError::Domain(e.to_string()))?
            }
        };

        customer.update_kyc(kyc_profile);

        // Recalculate risk score
        let risk_score = RiskScoringService::calculate_risk_score(&customer);
        customer.update_risk_score(risk_score);

        self.repo
            .save(&customer)
            .await
            .map_err(CustomerServiceError::Internal)?;

        Ok(customer)
    }

    pub async fn list_customers(&self) -> Result<Vec<Customer>, CustomerServiceError> {
        self.repo
            .list_all()
            .await
            .map_err(CustomerServiceError::Internal)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use banko_domain::customer::{Customer, CustomerStatus, PepStatus};
    use banko_domain::shared::value_objects::{CustomerId, EmailAddress};

    use super::super::dto::{AddressDto, BeneficiaryDto, CreateCustomerRequest};
    use super::super::ports::{ICustomerRepository, IPepCheckService};
    use super::*;

    // --- Mock Repository ---

    struct MockCustomerRepository {
        customers: Mutex<Vec<Customer>>,
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

        async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<Customer>, String> {
            let customers = self.customers.lock().unwrap();
            Ok(customers
                .iter()
                .find(|c| c.kyc_profile().email() == email)
                .cloned())
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

        async fn find_closed_before(&self, before: chrono::DateTime<chrono::Utc>) -> Result<Vec<Customer>, String> {
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

    // --- Mock PEP Checker ---

    struct MockPepChecker {
        pep_names: Vec<String>,
    }

    impl MockPepChecker {
        fn new() -> Self {
            MockPepChecker {
                pep_names: vec![],
            }
        }

        fn with_pep_names(names: Vec<String>) -> Self {
            MockPepChecker { pep_names: names }
        }
    }

    #[async_trait]
    impl IPepCheckService for MockPepChecker {
        async fn is_pep(&self, full_name: &str) -> Result<bool, String> {
            let name_lower = full_name.to_lowercase();
            Ok(self
                .pep_names
                .iter()
                .any(|n| name_lower.contains(&n.to_lowercase())))
        }
    }

    fn make_service() -> CustomerService {
        CustomerService::new(
            Arc::new(MockCustomerRepository::new()),
            Arc::new(MockPepChecker::new()),
        )
    }

    fn make_service_with_pep(pep_names: Vec<String>) -> CustomerService {
        CustomerService::new(
            Arc::new(MockCustomerRepository::new()),
            Arc::new(MockPepChecker::with_pep_names(pep_names)),
        )
    }

    fn valid_create_request() -> CreateCustomerRequest {
        CreateCustomerRequest {
            customer_type: "Individual".to_string(),
            full_name: "Ahmed Ben Ayed".to_string(),
            cin: Some("12345678".to_string()),
            birth_date: Some("1990-01-15".to_string()),
            nationality: Some("Tunisia".to_string()),
            profession: Some("Banker".to_string()),
            address: AddressDto {
                street: "10 Rue de la Liberte".to_string(),
                city: "Tunis".to_string(),
                postal_code: "1000".to_string(),
                country: Some("Tunisia".to_string()),
            },
            phone: "+21698123456".to_string(),
            email: "ahmed@example.com".to_string(),
            pep_status: Some("No".to_string()),
            source_of_funds: Some("Salary".to_string()),
            consent: true,
            registration_number: None,
            sector: None,
            beneficiaries: None,
        }
    }

    // --- Create tests ---

    #[tokio::test]
    async fn test_create_customer_individual_success() {
        let service = make_service();
        let result = service.create_customer(valid_create_request()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_customer_consent_not_given() {
        let service = make_service();
        let mut req = valid_create_request();
        req.consent = false;
        let result = service.create_customer(req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CustomerServiceError::Domain(msg) => {
                assert!(msg.contains("consent"));
            }
            e => panic!("Expected Domain error, got {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_create_customer_invalid_email() {
        let service = make_service();
        let mut req = valid_create_request();
        req.email = "invalid".to_string();
        let result = service.create_customer(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_customer_duplicate_email() {
        let service = make_service();
        let req = valid_create_request();
        service.create_customer(req).await.unwrap();

        let req2 = valid_create_request();
        let result = service.create_customer(req2).await;
        assert!(matches!(
            result,
            Err(CustomerServiceError::EmailAlreadyExists(_))
        ));
    }

    #[tokio::test]
    async fn test_create_customer_invalid_cin() {
        let service = make_service();
        let mut req = valid_create_request();
        req.cin = Some("123".to_string());
        let result = service.create_customer(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_customer_legal_entity_no_beneficiaries() {
        let service = make_service();
        let mut req = valid_create_request();
        req.customer_type = "LegalEntity".to_string();
        req.full_name = "Banko SA".to_string();
        req.registration_number = Some("RCS-12345".to_string());
        req.sector = Some("Banking".to_string());
        req.cin = None;
        req.birth_date = None;
        req.beneficiaries = None;
        let result = service.create_customer(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_customer_legal_entity_success() {
        let service = make_service();
        let mut req = valid_create_request();
        req.customer_type = "LegalEntity".to_string();
        req.full_name = "Banko SA".to_string();
        req.registration_number = Some("RCS-12345".to_string());
        req.sector = Some("Banking".to_string());
        req.cin = None;
        req.birth_date = None;
        req.beneficiaries = Some(vec![BeneficiaryDto {
            full_name: "Ahmed".to_string(),
            share_percentage: 100.0,
        }]);
        let result = service.create_customer(req).await;
        assert!(result.is_ok());
    }

    // --- Find tests ---

    #[tokio::test]
    async fn test_find_by_id_success() {
        let service = make_service();
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();
        let customer = service.find_by_id(&id).await.unwrap();
        assert_eq!(customer.kyc_profile().full_name(), "Ahmed Ben Ayed");
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let service = make_service();
        let result = service.find_by_id(&CustomerId::new()).await;
        assert!(matches!(result, Err(CustomerServiceError::CustomerNotFound)));
    }

    // --- Approve/Reject tests ---

    #[tokio::test]
    async fn test_approve_kyc() {
        let service = make_service();
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();
        let customer = service.approve_kyc(&id).await.unwrap();
        assert!(customer.is_kyc_validated());
        assert_eq!(customer.status(), CustomerStatus::Approved);
    }

    #[tokio::test]
    async fn test_reject_kyc() {
        let service = make_service();
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();
        let customer = service
            .reject_kyc(&id, "Missing documents".to_string())
            .await
            .unwrap();
        assert_eq!(customer.status(), CustomerStatus::Rejected);
    }

    // --- List tests ---

    #[tokio::test]
    async fn test_list_customers() {
        let service = make_service();
        service
            .create_customer(valid_create_request())
            .await
            .unwrap();

        let mut req2 = valid_create_request();
        req2.email = "another@example.com".to_string();
        service.create_customer(req2).await.unwrap();

        let list = service.list_customers().await.unwrap();
        assert_eq!(list.len(), 2);
    }

    // --- PEP check tests (STORY-C07) ---

    #[tokio::test]
    async fn test_pep_detected_auto_flags() {
        let service =
            make_service_with_pep(vec!["Ahmed Ben Ayed".to_string()]);
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();
        let customer = service.find_by_id(&id).await.unwrap();
        assert_eq!(customer.kyc_profile().pep_status(), PepStatus::Yes);
        // PEP should increase risk score: base 10 + PEP 30 = 40
        assert_eq!(customer.risk_score().value(), 40);
    }

    #[tokio::test]
    async fn test_non_pep_keeps_original_status() {
        let service = make_service();
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();
        let customer = service.find_by_id(&id).await.unwrap();
        assert_eq!(customer.kyc_profile().pep_status(), PepStatus::No);
        assert_eq!(customer.risk_score().value(), 10); // base only
    }

    // --- Risk scoring tests (STORY-C08) ---

    #[tokio::test]
    async fn test_risk_score_auto_calculated() {
        let service = make_service();
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();
        let customer = service.find_by_id(&id).await.unwrap();
        // Base score for a normal individual
        assert!(customer.risk_score().value() >= 10);
    }

    // --- Update KYC tests ---

    #[tokio::test]
    async fn test_update_kyc() {
        let service = make_service();
        let id = service
            .create_customer(valid_create_request())
            .await
            .unwrap();

        let update_req = UpdateKycRequest {
            full_name: "Ahmed Updated".to_string(),
            cin: Some("87654321".to_string()),
            birth_date: Some("1990-01-15".to_string()),
            nationality: Some("Tunisia".to_string()),
            profession: Some("Manager".to_string()),
            address: AddressDto {
                street: "20 Avenue Bourguiba".to_string(),
                city: "Tunis".to_string(),
                postal_code: "1000".to_string(),
                country: Some("Tunisia".to_string()),
            },
            phone: "+21698123456".to_string(),
            email: "ahmed@example.com".to_string(),
            pep_status: None,
            source_of_funds: None,
            registration_number: None,
            sector: None,
        };

        let customer = service.update_kyc(&id, update_req).await.unwrap();
        assert_eq!(customer.kyc_profile().full_name(), "Ahmed Updated");
    }
}
