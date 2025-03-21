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
use log::{info, LevelFilter};


fn main() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let raw_ticker: &str = "AAPL";
    let window: u32 = 90;
    let capital: isize = 1_000_000;

    let loader = AlphaVantage;
    let metadata = Metadata::new(raw_ticker.to_string());
    let ticker = metadata.symbol.clone();

    let data = loader.get_timeseries(ticker, &Interval::Day)
        .expect("Unable to generate data.");

    let portfolio = Portfolio::new(capital);

    let strategy_factory: &mut StrategyFactory = get_strategy_factory();

    let strategy: Box<dyn Strategy> = strategy_factory.create("ma_crossover", window, 100, -100)
        .expect("Unable to generate strategy.");
    let mut backtest = Backtest::new(window, portfolio);

    let result = backtest.run(&*strategy, &data, &metadata)
        .expect("Backtesting error.");

    info!("{:?}", result);
}
