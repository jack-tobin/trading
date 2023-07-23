///
/// Orders.
///
/// This contains various objects related to orders, including an Order
/// struct and a Confirm struct.
///

use std::fmt;
use chrono::{DateTime, Utc};


pub struct Quote {
    pub ticker: String,
    pub quote: f64,
    pub quantity: i64,
    pub timestamp: DateTime<Utc>,
}
impl Quote {
    pub fn new(
        ticker: String,
        quote: f64,
        quantity: i64
    ) -> Self {
        let timestamp = Utc::now();
        Self {
            ticker,
            quote,
            quantity,
            timestamp,
        }
    }
}
impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Quote(ticker={}, quote={}, quantity={}, timestamp={})",
            self.ticker,
            self.quote,
            self.quantity,
            self.timestamp,
        )
    }
}


pub struct Order {
    pub timestamp: DateTime<Utc>,
    pub ticker: String,
    pub quantity: i64,
}
impl Order {
    pub fn new(
        ticker: String,
        quantity: i64,
    ) -> Self {
        let timestamp = Utc::now();
        Self {
            timestamp,
            ticker,
            quantity
        }
    }
}
impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Order(ticker={}, quantity={}, timestamp={})",
            self.ticker,
            self.quantity,
            self.timestamp,
        )
    }
}

pub struct OrderResult {
    pub ticker: String,
    pub timestamp: DateTime<Utc>,
    pub filled_quantity: i64,
    pub filled_price: f64,
}
impl OrderResult {
    pub fn new(
        ticker: String,
        filled_quantity: i64,
        filled_price: f64,
    ) -> Self {
        let timestamp = Utc::now();
        Self {
            ticker,
            timestamp,
            filled_quantity,
            filled_price,
        }
    }
}
impl fmt::Display for OrderResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OrderResult(ticker={}, timestamp={}, quantity={}, price={})",
            self.ticker,
            self.timestamp,
            self.filled_quantity,
            self.filled_price,
        )
    }
}



pub struct Confirm {
    pub ticker: String,
    pub executed_timestamp: DateTime<Utc>,
    pub quantity_filled: i64,
    pub executed_price: f64,
    pub trading_costs: f64,
}
impl Confirm {
    pub fn new(
        ticker: String,
        executed_timestamp: DateTime<Utc>,
        quantity_filled: i64,
        executed_price: f64,
        trading_costs: f64,
    ) -> Self {
        Self {
            ticker,
            executed_timestamp,
            quantity_filled,
            executed_price,
            trading_costs,
        }
    }
}
impl fmt::Display for Confirm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Confirm(ticker={}, quantity={}, fill_price={}, costs={}, timestamp={})",
            self.ticker,
            self.quantity_filled,
            self.executed_price,
            self.trading_costs,
            self.executed_timestamp,
        )
    }
}
