// Application services aggregate multiple repository ports (DDD pattern).
#![allow(clippy::too_many_arguments)]

pub mod account;
pub mod accounting;
pub mod aml;
pub mod arrangement;
pub mod cash_management;
pub mod collateral;
pub mod compliance;
pub mod credit;
pub mod customer;
pub mod data_hub;
pub mod dto;
pub mod events;
pub mod fx;
pub mod governance;
pub mod identity;
pub mod insurance;
pub mod islamic_banking;
pub mod notification;
pub mod payment;
pub mod ports;
pub mod product;
pub mod prudential;
pub mod reference_data;
pub mod reporting;
pub mod saga;
pub mod sanctions;
pub mod securities;
pub mod trade_finance;
pub mod use_cases;

#[cfg(test)]
mod e2e_tests;
