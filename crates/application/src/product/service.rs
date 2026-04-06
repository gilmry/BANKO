use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use banko_domain::product::{
    CalcMethod, CustomerSegment, EligibilityRule, Fee, FeeType, InterestRate, Product,
    ProductStatus, ProductType, PricingBand, PricingGrid,
};

use super::dto::{
    AccrualResult, CreateEligibilityDto, CreateFeeDto, CreateInterestRateDto,
    CreatePricingBandDto, CreatePricingGridRequest, CreateProductRequest, EligibilityCheckRequest,
    EligibilityCheckResponse, InterestRateResponse, MaturityResult, PriceQuote,
    PricingBandResponse, PricingGridResponse, ProductResponse, UpdateProductRequest, FeeResponse,
    EligibilityResponse,
};
use super::errors::ProductServiceError;
use super::ports::{IPricingGridRepository, IProductRepository};

// ============================================================
// ProductService
// ============================================================

pub struct ProductService {
    product_repo: Arc<dyn IProductRepository>,
    pricing_grid_repo: Arc<dyn IPricingGridRepository>,
}

impl ProductService {
    pub fn new(
        product_repo: Arc<dyn IProductRepository>,
        pricing_grid_repo: Arc<dyn IPricingGridRepository>,
    ) -> Self {
        ProductService {
            product_repo,
            pricing_grid_repo,
        }
    }

    /// Create a new product
    pub async fn create_product(
        &self,
        req: CreateProductRequest,
    ) -> Result<ProductResponse, ProductServiceError> {
        // Parse product type
        let product_type = ProductType::from_str(&req.product_type)
            .map_err(|e| ProductServiceError::InvalidInput(e))?;

        // Parse interest rate if provided
        let interest_rate = if let Some(ir_dto) = req.interest_rate {
            let calc_method = CalcMethod::from_str(&ir_dto.calc_method)
                .map_err(|e| ProductServiceError::InvalidInput(e))?;
            Some(
                InterestRate::new(ir_dto.annual_rate, calc_method, ir_dto.floor_rate, ir_dto.ceiling_rate)
                    .map_err(|e| ProductServiceError::DomainError(e))?,
            )
        } else {
            None
        };

        // Parse fees
        let mut fees = Vec::new();
        for fee_dto in req.fees {
            let fee_type = FeeType::from_str(&fee_dto.fee_type)
                .map_err(|e| ProductServiceError::InvalidInput(e))?;
            let fee = Fee::new(
                fee_type,
                fee_dto.fixed_amount,
                fee_dto.rate,
                fee_dto.min_amount,
                fee_dto.max_amount,
                fee_dto.charged_on,
            )
            .map_err(|e| ProductServiceError::DomainError(e))?;
            fees.push(fee);
        }

        // Parse eligibility
        let eligibility = self.parse_eligibility_dto(&req.eligibility)?;

        // Parse segment pricing
        let mut segment_pricing = HashMap::new();
        if let Some(sp) = req.segment_pricing {
            for (segment_str, rate) in sp.iter() {
                let segment = CustomerSegment::from_str(segment_str)
                    .map_err(|e| ProductServiceError::InvalidInput(e.to_string()))?;
                segment_pricing.insert(segment, *rate);
            }
        }

        // Create product
        let product = Product::new(
            req.name,
            product_type,
            interest_rate,
            fees,
            eligibility,
            segment_pricing,
            req.min_balance,
            req.currency,
        )
        .map_err(|e| ProductServiceError::DomainError(e))?;

        // Save product
        self.product_repo
            .save(&product)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?;

        Ok(self.product_to_response(&product))
    }

    /// Get a product by ID
    pub async fn get_product(&self, id: Uuid) -> Result<ProductResponse, ProductServiceError> {
        let product = self
            .product_repo
            .find_by_id(id)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?
            .ok_or(ProductServiceError::ProductNotFound)?;

        Ok(self.product_to_response(&product))
    }

    /// List all products
    pub async fn list_products(&self) -> Result<Vec<ProductResponse>, ProductServiceError> {
        let products = self
            .product_repo
            .list_all()
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?;

        Ok(products.iter().map(|p| self.product_to_response(p)).collect())
    }

    /// Activate a product
    pub async fn activate_product(&self, id: Uuid) -> Result<ProductResponse, ProductServiceError> {
        let mut product = self
            .product_repo
            .find_by_id(id)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?
            .ok_or(ProductServiceError::ProductNotFound)?;

        product
            .activate()
            .map_err(|e| ProductServiceError::DomainError(e))?;

        self.product_repo
            .update(&product)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?;

        Ok(self.product_to_response(&product))
    }

    /// Suspend a product
    pub async fn suspend_product(&self, id: Uuid) -> Result<ProductResponse, ProductServiceError> {
        let mut product = self
            .product_repo
            .find_by_id(id)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?
            .ok_or(ProductServiceError::ProductNotFound)?;

        product
            .suspend()
            .map_err(|e| ProductServiceError::DomainError(e))?;

        self.product_repo
            .update(&product)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?;

        Ok(self.product_to_response(&product))
    }

    /// Calculate price quote for a customer
    pub async fn calculate_price(
        &self,
        product_id: Uuid,
        customer_segment: &str,
        amount: Decimal,
    ) -> Result<PriceQuote, ProductServiceError> {
        // Get product
        let product = self
            .product_repo
            .find_by_id(product_id)
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?
            .ok_or(ProductServiceError::ProductNotFound)?;

        // Parse segment
        let segment = CustomerSegment::from_str(customer_segment)
            .map_err(|e| ProductServiceError::InvalidInput(e))?;

        // Get rate: first try pricing grid, then segment override, then default
        let rate: Decimal = self
            .get_effective_rate(&product, amount)
            .await
            .or_else(|| product.get_rate_for_segment(&segment))
            .unwrap_or(Decimal::ZERO);

        // Calculate fees
        let fees = product.calculate_total_fees(amount);

        let total_cost = fees; // Cost is fees (rate is for interest earned/paid)

        Ok(PriceQuote {
            product_id: product_id.to_string(),
            rate,
            fees,
            total_cost,
            segment_applied: segment.as_str().to_string(),
            currency: product.currency().to_string(),
        })
    }

    /// Check eligibility for a specific product
    pub async fn check_eligibility(
        &self,
        req: EligibilityCheckRequest,
    ) -> Result<EligibilityCheckResponse, ProductServiceError> {
        // If product_id is provided, check for that product specifically
        if let Some(product_id_str) = &req.product_id {
            let product_id = Uuid::parse_str(product_id_str)
                .map_err(|_| ProductServiceError::InvalidInput("Invalid product ID".to_string()))?;

            let product = self
                .product_repo
                .find_by_id(product_id)
                .await
                .map_err(|e| ProductServiceError::RepositoryError(e))?
                .ok_or(ProductServiceError::ProductNotFound)?;

            let segment = CustomerSegment::from_str(&req.segment)
                .map_err(|e| ProductServiceError::InvalidInput(e))?;

            match product.evaluate_eligibility(req.age, req.income, &segment, req.credit_score) {
                Ok(()) => Ok(EligibilityCheckResponse {
                    eligible: true,
                    product_id: Some(product_id_str.clone()),
                    reasons: vec![],
                }),
                Err(reasons) => Ok(EligibilityCheckResponse {
                    eligible: false,
                    product_id: Some(product_id_str.clone()),
                    reasons,
                }),
            }
        } else {
            // Check all products
            let products = self
                .product_repo
                .list_all()
                .await
                .map_err(|e| ProductServiceError::RepositoryError(e))?;

            let segment = CustomerSegment::from_str(&req.segment)
                .map_err(|e| ProductServiceError::InvalidInput(e))?;

            // Find first eligible product
            for product in products {
                if product.evaluate_eligibility(req.age, req.income, &segment, req.credit_score).is_ok() {
                    return Ok(EligibilityCheckResponse {
                        eligible: true,
                        product_id: Some(product.id().to_string()),
                        reasons: vec![],
                    });
                }
            }

            Ok(EligibilityCheckResponse {
                eligible: false,
                product_id: None,
                reasons: vec!["No eligible products found".to_string()],
            })
        }
    }

    /// Get products eligible for a customer
    pub async fn get_eligible_products(
        &self,
        req: EligibilityCheckRequest,
    ) -> Result<Vec<ProductResponse>, ProductServiceError> {
        let products = self
            .product_repo
            .list_all()
            .await
            .map_err(|e| ProductServiceError::RepositoryError(e))?;

        let segment = CustomerSegment::from_str(&req.segment)
            .map_err(|e| ProductServiceError::InvalidInput(e))?;

        let mut eligible = Vec::new();
        for product in products {
            if product.evaluate_eligibility(req.age, req.income, &segment, req.credit_score).is_ok() {
                eligible.push(self.product_to_response(&product));
            }
        }

        Ok(eligible)
    }

    // --- Private helpers ---

    async fn get_effective_rate(
        &self,
        product: &Product,
        amount: Decimal,
    ) -> Option<Decimal> {
        let now = Utc::now();
        if let Ok(grids) = self
            .pricing_grid_repo
            .find_active_for_product(product.id(), now)
            .await
        {
            for grid in grids {
                if let Some(rate) = grid.get_rate_for_amount(amount) {
                    return Some(rate);
                }
            }
        }
        None
    }

    fn parse_eligibility_dto(
        &self,
        req: &CreateEligibilityDto,
    ) -> Result<EligibilityRule, ProductServiceError> {
        let required_segment = if let Some(segment_str) = &req.required_segment {
            Some(
                CustomerSegment::from_str(segment_str)
                    .map_err(|e| ProductServiceError::InvalidInput(e))?,
            )
        } else {
            None
        };

        Ok(EligibilityRule::new(
            req.min_age,
            req.max_age,
            req.min_income,
            required_segment,
            req.min_credit_score,
        ))
    }

    fn product_to_response(&self, product: &Product) -> ProductResponse {
        let interest_rate = product.interest_rate().map(|ir| InterestRateResponse {
            annual_rate: ir.annual_rate(),
            calc_method: ir.calc_method().as_str().to_string(),
            floor_rate: ir.floor_rate(),
            ceiling_rate: ir.ceiling_rate(),
        });

        let fees = product
            .fees()
            .iter()
            .map(|fee| FeeResponse {
                id: fee.id().to_string(),
                fee_type: fee.fee_type().as_str().to_string(),
                fixed_amount: fee.fixed_amount(),
                rate: fee.rate(),
                min_amount: fee.min_amount(),
                max_amount: fee.max_amount(),
                charged_on: fee.charged_on(),
            })
            .collect();

        let eligibility_rule = product.eligibility();
        let eligibility = EligibilityResponse {
            min_age: eligibility_rule.min_age(),
            max_age: eligibility_rule.max_age(),
            min_income: eligibility_rule.min_income(),
            required_segment: eligibility_rule
                .required_segment()
                .map(|s| s.as_str().to_string()),
            min_credit_score: eligibility_rule.min_credit_score(),
        };

        let segment_pricing = product
            .segment_pricing()
            .iter()
            .map(|(segment, rate)| (segment.as_str().to_string(), *rate))
            .collect();

        ProductResponse {
            id: product.id().to_string(),
            name: product.name().to_string(),
            product_type: product.product_type().as_str().to_string(),
            status: product.status().as_str().to_string(),
            interest_rate,
            fees,
            eligibility,
            segment_pricing,
            min_balance: product.min_balance(),
            currency: product.currency().to_string(),
            version: product.version(),
            created_at: product.created_at().to_rfc3339(),
            updated_at: product.updated_at().to_rfc3339(),
        }
    }
}

// ============================================================
// InterestCalculationService
// ============================================================

pub struct InterestCalculationService;

impl InterestCalculationService {
    /// Calculate daily interest for an account
    pub fn calculate_daily_interest(
        account_balance: Decimal,
        annual_rate: Decimal,
        calc_method: &str,
    ) -> Result<Decimal, String> {
        let method = CalcMethod::from_str(calc_method)?;
        let interest_rate =
            InterestRate::new(annual_rate, method, None, None).map_err(|e| e)?;
        Ok(interest_rate.calculate_daily_interest(account_balance))
    }

    /// Calculate maturity result for a term deposit
    pub fn calculate_term_deposit_maturity(
        principal: Decimal,
        annual_rate: Decimal,
        months: u32,
        currency: &str,
    ) -> Result<MaturityResult, String> {
        let interest_rate = InterestRate::new(annual_rate, CalcMethod::Compound, None, None)?;
        let total_interest = interest_rate.calculate_compound_monthly(principal, months);
        let final_amount = principal + total_interest;

        Ok(MaturityResult {
            principal,
            total_interest,
            final_amount,
            currency: currency.to_string(),
        })
    }

    /// Process interest accrual for multiple accounts (batch operation)
    pub fn process_interest_accrual(
        accounts: Vec<(Decimal, Decimal)>, // (balance, rate) pairs
    ) -> AccrualResult {
        let mut processed = 0;
        let mut total_interest = Decimal::ZERO;

        for (_balance, _rate) in accounts {
            // In real implementation, this would persist to database
            processed += 1;
        }

        AccrualResult {
            processed,
            skipped: 0,
            total_interest,
        }
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock repository for testing
    struct MockProductRepository {
        products: std::sync::Mutex<Vec<Product>>,
    }

    impl MockProductRepository {
        fn new() -> Arc<Self> {
            Arc::new(MockProductRepository {
                products: std::sync::Mutex::new(Vec::new()),
            })
        }
    }

    #[async_trait::async_trait]
    impl IProductRepository for MockProductRepository {
        async fn save(&self, product: &Product) -> Result<(), String> {
            let mut products = self.products.lock().unwrap();
            products.push(product.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, String> {
            let products = self.products.lock().unwrap();
            Ok(products.iter().find(|p| p.id() == id).cloned())
        }

        async fn list_all(&self) -> Result<Vec<Product>, String> {
            let products = self.products.lock().unwrap();
            Ok(products.clone())
        }

        async fn list_active(&self) -> Result<Vec<Product>, String> {
            let products = self.products.lock().unwrap();
            Ok(products
                .iter()
                .filter(|p| p.status() == ProductStatus::Active)
                .cloned()
                .collect())
        }

        async fn find_by_type(&self, product_type: ProductType) -> Result<Vec<Product>, String> {
            let products = self.products.lock().unwrap();
            Ok(products
                .iter()
                .filter(|p| p.product_type() == product_type)
                .cloned()
                .collect())
        }

        async fn update(&self, product: &Product) -> Result<(), String> {
            let mut products = self.products.lock().unwrap();
            if let Some(pos) = products.iter().position(|p| p.id() == product.id()) {
                products[pos] = product.clone();
                Ok(())
            } else {
                Err("Product not found".to_string())
            }
        }
    }

    struct MockPricingGridRepository;

    #[async_trait::async_trait]
    impl IPricingGridRepository for MockPricingGridRepository {
        async fn save(&self, _grid: &PricingGrid) -> Result<(), String> {
            Ok(())
        }

        async fn find_by_product(&self, _product_id: Uuid) -> Result<Vec<PricingGrid>, String> {
            Ok(Vec::new())
        }

        async fn find_active_for_product(
            &self,
            _product_id: Uuid,
            _as_of_date: chrono::DateTime<chrono::Utc>,
        ) -> Result<Vec<PricingGrid>, String> {
            Ok(Vec::new())
        }

        async fn list_all(&self) -> Result<Vec<PricingGrid>, String> {
            Ok(Vec::new())
        }
    }

    fn create_test_service() -> ProductService {
        ProductService {
            product_repo: MockProductRepository::new(),
            pricing_grid_repo: Arc::new(MockPricingGridRepository),
        }
    }

    #[tokio::test]
    async fn test_create_product() {
        let service = create_test_service();
        let req = CreateProductRequest {
            name: "Test Account".to_string(),
            product_type: "CurrentAccount".to_string(),
            interest_rate: Some(CreateInterestRateDto {
                annual_rate: Decimal::from(5),
                calc_method: "Simple".to_string(),
                floor_rate: None,
                ceiling_rate: None,
            }),
            fees: vec![],
            eligibility: CreateEligibilityDto {
                min_age: None,
                max_age: None,
                min_income: None,
                required_segment: None,
                min_credit_score: None,
            },
            segment_pricing: None,
            min_balance: None,
            currency: "TND".to_string(),
        };

        let result = service.create_product(req).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "Test Account");
        assert_eq!(response.status, "Draft");
    }

    #[tokio::test]
    async fn test_activate_product() {
        let service = create_test_service();
        let req = CreateProductRequest {
            name: "Test Account".to_string(),
            product_type: "CurrentAccount".to_string(),
            interest_rate: None,
            fees: vec![],
            eligibility: CreateEligibilityDto {
                min_age: None,
                max_age: None,
                min_income: None,
                required_segment: None,
                min_credit_score: None,
            },
            segment_pricing: None,
            min_balance: None,
            currency: "TND".to_string(),
        };

        let product = service.create_product(req).await.unwrap();
        let product_id = Uuid::parse_str(&product.id).unwrap();

        let activated = service.activate_product(product_id).await;
        assert!(activated.is_ok());
        assert_eq!(activated.unwrap().status, "Active");
    }

    #[tokio::test]
    async fn test_get_product_not_found() {
        let service = create_test_service();
        let result = service.get_product(Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_products() {
        let service = create_test_service();
        let req = CreateProductRequest {
            name: "Test Account".to_string(),
            product_type: "CurrentAccount".to_string(),
            interest_rate: None,
            fees: vec![],
            eligibility: CreateEligibilityDto {
                min_age: None,
                max_age: None,
                min_income: None,
                required_segment: None,
                min_credit_score: None,
            },
            segment_pricing: None,
            min_balance: None,
            currency: "TND".to_string(),
        };

        service.create_product(req).await.unwrap();
        let products = service.list_products().await.unwrap();
        assert_eq!(products.len(), 1);
    }

    #[tokio::test]
    async fn test_calculate_price() {
        let service = create_test_service();
        let req = CreateProductRequest {
            name: "Test Account".to_string(),
            product_type: "CurrentAccount".to_string(),
            interest_rate: Some(CreateInterestRateDto {
                annual_rate: Decimal::from(5),
                calc_method: "Simple".to_string(),
                floor_rate: None,
                ceiling_rate: None,
            }),
            fees: vec![],
            eligibility: CreateEligibilityDto {
                min_age: None,
                max_age: None,
                min_income: None,
                required_segment: None,
                min_credit_score: None,
            },
            segment_pricing: None,
            min_balance: None,
            currency: "TND".to_string(),
        };

        let product = service.create_product(req).await.unwrap();
        let product_id = Uuid::parse_str(&product.id).unwrap();

        let quote = service
            .calculate_price(product_id, "Standard", Decimal::from(1000))
            .await
            .unwrap();

        assert_eq!(quote.rate, Decimal::from(5));
    }

    #[tokio::test]
    async fn test_check_eligibility() {
        let service = create_test_service();
        let req = CreateProductRequest {
            name: "Test Account".to_string(),
            product_type: "CurrentAccount".to_string(),
            interest_rate: None,
            fees: vec![],
            eligibility: CreateEligibilityDto {
                min_age: Some(18),
                max_age: None,
                min_income: None,
                required_segment: None,
                min_credit_score: None,
            },
            segment_pricing: None,
            min_balance: None,
            currency: "TND".to_string(),
        };

        let product = service.create_product(req).await.unwrap();
        let product_id = Uuid::parse_str(&product.id).unwrap();

        let check = EligibilityCheckRequest {
            product_id: Some(product_id.to_string()),
            age: 25,
            income: Decimal::from(50000),
            segment: "Standard".to_string(),
            credit_score: 750,
        };

        let result = service.check_eligibility(check).await.unwrap();
        assert!(result.eligible);
    }

    #[test]
    fn test_calculate_daily_interest() {
        let result = InterestCalculationService::calculate_daily_interest(
            Decimal::from(10000),
            Decimal::from(36),
            "Daily",
        );
        assert!(result.is_ok());
        assert!(result.unwrap() > Decimal::ZERO);
    }

    #[test]
    fn test_calculate_term_deposit_maturity() {
        let result = InterestCalculationService::calculate_term_deposit_maturity(
            Decimal::from(10000),
            Decimal::from(12),
            12,
            "TND",
        );
        assert!(result.is_ok());
        let maturity = result.unwrap();
        assert!(maturity.final_amount > maturity.principal);
    }

    #[test]
    fn test_process_interest_accrual() {
        let accounts = vec![
            (Decimal::from(5000), Decimal::from(5)),
            (Decimal::from(3000), Decimal::from(5)),
        ];
        let result = InterestCalculationService::process_interest_accrual(accounts);
        assert_eq!(result.processed, 2);
        assert_eq!(result.skipped, 0);
    }
}
