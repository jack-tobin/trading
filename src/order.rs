///
/// Orders.
///
/// This contains various objects related to orders, including an Order
/// struct and a Confirm struct.
///

use std::fmt;
use chrono::{Utc};


pub struct Order {
    pub timestamp: i64,
    pub ticker: String,
    pub quantity: i64,
}

impl Order {

    pub fn new(
        ticker: String,
        quantity: i64,
    ) -> Order {

        let dt = Utc::now();
        let timestamp = dt.timestamp();

        Order {
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
            "Order of {} {} contracts submitted at {}.",
            self.quantity,
            self.ticker,
            self.timestamp,
        )
    }
}


pub struct Confirm {
    pub timestamp: i64,
    pub ticker: String,
    pub quantity_filled: i64,
    pub executed_price: f32,
    pub trading_costs: f32,
}

impl Confirm {

    pub fn new(
        ticker: String,
        quantity_filled: i64,
        executed_price: f32,
        trading_costs: f32,
    ) -> Confirm {

        let dt = Utc::now();
        let timestamp = dt.timestamp();

        Confirm {
            timestamp,
            ticker,
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
            "Order of {} {} contracts traded at {} with total fees of {} excuted at {}.",
            self.quantity_filled,
            self.ticker,
            self.executed_price,
            self.trading_costs,
            self.timestamp,
        )
    }
}
