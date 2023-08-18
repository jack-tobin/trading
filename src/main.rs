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
mod errors;

fn main() {
    let loader = data_loading::AlphaVantage;
    let interval = data_loading::Interval::Day;
    let data = loader.get_timeseries("AAPL".to_string(), interval)
        .expect("Unable to generate data.");

    let init_capital = 1000000;
    let portfolio = portfolio::Portfolio::new(init_capital);

    let ma_window: u64 = 90;
    let long_quantity: i64 = 100;
    let short_quantity: i64 = -100;
    let strategy = strategy::MACrossoverStrategy::new(
        ma_window,
        long_quantity,
        short_quantity
    );
    let mut backtest = backtest::Backtest::new(ma_window, portfolio);

    let (pnl, n_trades) = backtest.run(&strategy, &data)
        .expect("Backtesting error.");

    println!("PnL = {}; N Trades = {}", pnl, n_trades);

}
