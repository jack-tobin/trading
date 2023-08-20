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

use crate::data_loading::*;
use crate::portfolio::*;
use crate::strategy::*;
use crate::backtest::*;

fn main() {
    let loader = AlphaVantage;
    let data = loader.get_timeseries("AAPL".to_string(), Interval::Day)
        .expect("Unable to generate data.");
    // println!("{}", data);

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

    println!("{}", result);
}
