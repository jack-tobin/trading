
use chrono::{DateTime, Utc};
use derive_new::new;

#[allow(dead_code)]
#[derive(Debug, new)]
pub struct Order {
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
    pub ticker: String,
    pub quantity: i64,
}

#[allow(dead_code)]
#[derive(Debug, new)]
pub struct OrderResult {
    pub ticker: String,
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
    pub filled_quantity: i64,
    pub filled_price: f64,
}

#[allow(dead_code)]
#[derive(Debug, new)]
pub struct Confirm {
    pub ticker: String,
    pub executed_timestamp: DateTime<Utc>,
    pub quantity_filled: i64,
    pub executed_price: f64,
    pub trading_costs: f64,
}
