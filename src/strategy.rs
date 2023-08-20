/// Trading strategies.

use polars::prelude::*;

use crate::order::Order;
use crate::portfolio::Portfolio;

pub trait Strategy {
    fn on_data(&self, data: Series, portfolio: &Portfolio) -> Option<Order>;
}

pub struct MACrossoverStrategy {
    window: u64,
    long_quantity: i64,
    short_quantity: i64,
}
impl MACrossoverStrategy {
    pub fn new(window: u64, long_quantity: i64, short_quantity: i64) -> Self {
        Self {
            window,
            long_quantity,
            short_quantity,
        }
    }
}
impl Strategy for MACrossoverStrategy {

    fn on_data(
        &self,
        data: Series,
        portfolio: &Portfolio,
    ) -> Option<Order> {
        // MA crossover strategy strategy
        // If price is greater than avg price over a window, buy or maintain
        // If price is lower than avg price over a window, sell or maintain.
        // Otherwise close all positions.

        let n = data.len();
        let data_subset = data.tail(Some(self.window.try_into().ok()?));

        let subset_mean = data_subset
            .f64().ok()?
            .mean()?;

        let last_price = data_subset
            .f64().ok()?
            .get(n - 1)?;

        let order: Order;

        let is_long = portfolio.is_long();
        let is_short = portfolio.is_short();
        if (last_price > subset_mean) & !is_long {
            order = Order::new(data.name().to_string(), self.long_quantity);
            Some(order)
        }
        else if (last_price < subset_mean) & !is_short {
            order = Order::new(data.name().to_string(), self.short_quantity);
            Some(order)
        }
        else {
            None
        }
    }
}

