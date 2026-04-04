use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub id: String,
    pub customer_id: String,
    pub rib: String,
    pub account_type: String,
    pub balance: f64,
    pub available_balance: f64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovementResponse {
    pub id: String,
    pub account_id: String,
    pub movement_type: String,
    pub amount: f64,
    pub balance_after: f64,
    pub currency: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatementResponse {
    pub account_id: String,
    pub rib: String,
    pub period_from: Option<DateTime<Utc>>,
    pub period_to: Option<DateTime<Utc>>,
    pub opening_balance: f64,
    pub closing_balance: f64,
    pub currency: String,
    pub movements: Vec<MovementResponse>,
}
