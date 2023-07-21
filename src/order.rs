///
/// Orders.
///
/// This contains various objects related to orders, including an Order
/// struct and a Confirm struct.
///

use std::fmt;
use chrono::Utc;


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
            "Order(ticker={}, quantity={}, timestamp={})",
            self.ticker,
            self.quantity,
            self.timestamp,
        )
    }
}


pub struct Confirm {
    pub timestamp: i64,
    pub ticker: String,
    pub quantity_filled: i64,
    pub executed_price: f64,
    pub trading_costs: f64,
}

impl Confirm {

    pub fn new(
        ticker: String,
        quantity_filled: i64,
        executed_price: f64,
        trading_costs: f64,
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
            "Confirm(ticker={}, quantity={}, fill_price={}, costs={}, timestamp={})",
            self.ticker,
            self.quantity_filled,
            self.executed_price,
            self.trading_costs,
            self.timestamp,
        )
    }
}
