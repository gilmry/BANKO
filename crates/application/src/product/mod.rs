pub mod dto;
pub mod errors;
pub mod ports;
pub mod service;

pub use dto::{
    AccrualResult, CreateEligibilityDto, CreateFeeDto, CreateInterestRateDto,
    CreatePricingBandDto, CreatePricingGridRequest, CreateProductRequest, EligibilityCheckRequest,
    EligibilityCheckResponse, MaturityResult, PriceQuote, PricingBandResponse,
    PricingGridResponse, ProductResponse, UpdateProductRequest,
};
pub use errors::ProductServiceError;
pub use ports::{IPricingGridRepository, IProductRepository};
pub use service::{InterestCalculationService, ProductService};
