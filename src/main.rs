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

use crate::data_loading::{AlphaVantage, Interval, DatedStockData, Metadata};
use crate::portfolio::*;
use crate::strategy::*;
use crate::backtest::*;
use crate::order::Order;

use derive_new::new;
use env_logger::Builder;
use log::{info, LevelFilter};


#[derive(Debug, new)]
pub struct MACrossoverStrategy {
    window: u32,
    long_quantity: i64,
    short_quantity: i64,
}

impl Strategy for MACrossoverStrategy {
    fn on_data(
        &self,
        data: Vec<DatedStockData>,
        metadata: &Metadata,
        portfolio: &Portfolio,
    ) -> Option<Order> {
        // MA crossover strategy strategy
        // If price is greater than avg price over a window, buy or maintain
        // If price is lower than avg price over a window, sell or maintain.
        // Otherwise do nothing.

        let ticker = metadata.symbol.clone();

        let n = data.len();
        let data_subset = if n >= self.window as usize {
            data[(n - self.window as usize)..].to_vec()
        } else {
            return None;
        };

        let prices: Vec<f64> = data_subset.iter()
            .map(|x| x.close)
            .collect();

        let subset_mean = prices.iter().sum::<f64>() / prices.len() as f64;
        let last_price = prices.last()?;

        if (*last_price > subset_mean) & portfolio.is_not_long() {
            Some(Order::new(ticker, self.long_quantity))
        }
        else if (*last_price < subset_mean) & portfolio.is_not_short() {
            Some(Order::new(ticker, self.short_quantity))
        }
        else {
            None
        }
    }
}


fn main() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let loader = AlphaVantage;
    let metadata = Metadata::new("AAPL".to_string());
    let ticker = metadata.symbol.clone();

    let data = loader.get_timeseries(ticker, &Interval::Day)
        .expect("Unable to generate data.");

    let portfolio = Portfolio::new(1_000_000);

    let window: u32 = 90;
    let strategy = MACrossoverStrategy::new(
        window,
        100,
        -100
    );
    let mut backtest = Backtest::new(window, portfolio);

    let result = backtest.run(&strategy, &data, &metadata)
        .expect("Backtesting error.");

    info!("{:?}", result);
}
