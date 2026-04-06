// Product infrastructure module
// This module contains the database repositories for the Product bounded context.
// Repositories will be implemented when PostgreSQL integration is available.

pub mod repositories;

pub use repositories::{ProductRepository, PricingGridRepository};
