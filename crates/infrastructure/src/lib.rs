// Infrastructure handlers may need many parameters for dependency injection.
#![allow(clippy::too_many_arguments)]

pub mod account;
pub mod accounting;
pub mod aml;
pub mod arrangement;
pub mod cash_management;
pub mod collateral;
pub mod compliance;
pub mod config;
pub mod credit;
pub mod customer;
pub mod data_hub;
pub mod database;
pub mod fx;
pub mod governance;
pub mod identity;
pub mod insurance;
pub mod integrations;
pub mod islamic_banking;
pub mod jobs;
pub mod notification;
pub mod payment;
pub mod product;
pub mod prudential;
pub mod reference_data;
pub mod reporting;
pub mod sanctions;
pub mod securities;
pub mod trade_finance;
pub mod web;

#[cfg(test)]
pub(crate) mod test_helpers;
