///
/// Basic trading infrastructure.
/// May 2023
/// Jack Tobin
///

mod broker;
mod order;
mod config;
mod data_loading;
mod backtest;
mod strategy;
mod portfolio;

use crate::data_loading::{AlphaVantage, Interval};
use crate::portfolio::*;
use crate::strategy::*;
use crate::backtest::*;
use crate::order::Order;

use derive_new::new;
use polars::prelude::*;


#[derive(Debug, new)]
pub struct MACrossoverStrategy {
    window: u64,
    long_quantity: i64,
    short_quantity: i64,
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
        // Otherwise do nothing.

        let n = data.len();
        let data_subset = data.tail(Some(self.window.try_into().ok()?));

        let subset_mean = data_subset.f64().ok()?.mean()?;
        let last_price = data_subset.f64().ok()?.get(n - 1)?;

        if (last_price > subset_mean) & portfolio.is_not_long() {
            Some(Order::new(data.name().to_string(), self.long_quantity))
        }
        else if (last_price < subset_mean) & portfolio.is_not_short() {
            Some(Order::new(data.name().to_string(), self.short_quantity))
        }
        else {
            None
        }
    }
}


fn main() {
    let loader = AlphaVantage;
    let data = loader.get_timeseries("AAPL".to_string(), Interval::Day)
        .expect("Unable to generate data.");
    println!("{:?}", data);

    let portfolio = Portfolio::new(1_000_000);

    let strategy = MACrossoverStrategy::new(
        90,
        100,
        -100
    );
    let mut backtest = Backtest::new(90, portfolio);

    let data_col = data.column("AAPL").ok()
        .expect("No column exists in df.");

    let result = backtest.run(&strategy, data_col)
        .expect("Backtesting error.");

    println!("{:?}", result);
}
