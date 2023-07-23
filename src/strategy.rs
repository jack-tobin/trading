/// Trading strategies.

use polars::series::Series;
use std::io::Result;

use crate::order::Order;

pub trait OnData {
    fn on_data(&self, data: Series) -> Result<Order>;
}

pub struct MACrossoverStrategy;
impl OnData for MACrossoverStrategy {
    fn on_data(&self, data: Series) -> Result<Order> {
        // MA crossover strategy strategy
        // If price is greater than MA over a window, buy or maintain
        // If price is lower than MA over a window, sell or maintain.

        const MA_WINDOW: i32 = 60;

        let order = Order::new("AAPL".to_string(), 100);
        Ok(order)
    }
}

