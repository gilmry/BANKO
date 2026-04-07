pub mod account;
pub mod accounting;
pub mod aml;
pub mod compliance;
pub mod credit;
pub mod customer;
pub mod dto;
pub mod events;
pub mod fx;
pub mod governance;
pub mod identity;
pub mod notification;
pub mod payment;
pub mod ports;
pub mod product;
pub mod prudential;
pub mod reference_data;
pub mod reporting;
pub mod saga;
pub mod sanctions;
pub mod use_cases;

#[cfg(test)]
mod e2e_tests;
