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

use crate::data_loading::{AlphaVantage, Interval, Metadata};
use crate::portfolio::*;
use crate::strategy::*;
use crate::backtest::*;

use env_logger::Builder;
use log::{info, error, LevelFilter};


fn main() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    // Stuff to refactor into Python bindings.
    let ticker: &str = "AAPL";
    let window: u32 = 90;
    let capital: isize = 1_000_000;
    let long_qty: i64 = 100;
    let short_qty: i64 = -100;

    let metadata = Metadata::new(ticker.to_string());

    let loader = AlphaVantage;
    let data = match loader.get_timeseries(&metadata.symbol, &Interval::Day) {
        Ok(data) => data,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let portfolio = Portfolio::new(capital);

    let strategy_factory: &mut StrategyFactory = get_strategy_factory();

    let strategy: Box<dyn Strategy> = strategy_factory.create("ma_crossover", window, long_qty, short_qty)
        .expect("Unable to generate strategy.");
    let mut backtest = Backtest::new(window, portfolio);

    let result = backtest.run(&*strategy, &data, &metadata)
        .expect("Backtesting error.");

    info!("{:?}", result);
}
