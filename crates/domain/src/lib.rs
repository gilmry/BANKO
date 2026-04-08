// Domain entities legitimately require many constructor parameters
// to enforce invariants at creation time (DDD aggregate pattern).
#![allow(clippy::too_many_arguments)]

pub mod shared;
pub mod events;
pub mod invariants;

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
pub mod fx;
pub mod governance;
pub mod identity;
pub mod insurance;
pub mod islamic_banking;
pub mod notification;
pub mod payment;
pub mod product;
pub mod prudential;
pub mod reference_data;
pub mod reporting;
pub mod sanctions;
pub mod securities;
pub mod trade_finance;
