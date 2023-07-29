/// Trading strategies.

use polars::series::Series;
use std::io::Result;
use polars::prelude::ChunkAgg;
use polars::prelude::TakeRandom;

use crate::order::Order;

pub trait Strategy {

    fn set_position(&self, position: isize) {
        self.position = position;
    }

    fn on_data(&self, data: Series) -> Option<Order>;

    fn is_long(&self) -> Result<bool> {
        if self.position > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_short(&self) -> Result<bool> {
        if self.position < 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_invested(&self) -> Result<bool> {
        if self.position != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub struct MACrossoverStrategy {
    window: usize,
}
impl MACrossoverStrategy {
    pub fn new(window: usize) -> Self {
        let position: i64 = 0;
        Self {
            window,
        }
    }

    pub fn get_window(&self) -> Option<usize> {
        Some(self.window)
    }
}
impl Strategy for MACrossoverStrategy {
    fn on_data(&self, data: Series) -> Option<Order> {
        // MA crossover strategy strategy
        // If price is greater than avg price over a window, buy or maintain
        // If price is lower than avg price over a window, sell or maintain.

        let n = data.len();
        let data_subset = data.tail(self.get_window());

        let subset_mean = data_subset
            .f64()
            .expect("Unrecognized type")
            .mean()
            .expect("Error getting mean.");

        let last_price = data_subset
            .f64()
            .expect("Unrecognized type.")
            .get(n - 1)
            .expect("Unable to get last item.");

        let order: Order;

        let is_invested = self.is_invested().unwrap();
        if (last_price > subset_mean) & !is_invested {
            order = Order::new("AAPL".to_string(), 100);
            Some(order)
        }
        else if (last_price < subset_mean) & !is_invested {
            order = Order::new("AAPL".to_string(), -100);
            Some(order)
        }
        else {
            None
        }
    }
}

