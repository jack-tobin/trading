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

use broker::Broker;
use order::{Order, Confirm};

fn main() {
    // Establish our broker with $0.05 trading costs.
    let broker: Broker = Broker::new(0.05);

    // Make an order for +100 AAPL shares.
    let ticker: String = "AAPL".to_string();
    let shares = 100;
    let order = Order::new(ticker.clone(), shares);

    // Instruct broker to execute my order.
    let confirm: Confirm = broker.execute(order)
        .expect("Error in execution.");
    println!("{}", confirm);

    // Load in a time series of data for our strategy.
    let loader = data_loading::AlphaVantage;
    let interval = data_loading::Interval::Day;
    let data = loader._get_timeseries(ticker.clone(), interval)
        .expect("Error downloading data.");

    // Define a strategy and a backtest for that strategy.
    const MA_WINDOW: usize = 30;
    let strategy = strategy::MACrossoverStrategy::new(MA_WINDOW);
    let backtest = backtest::Backtest::new(MA_WINDOW);

    // Run the backtest.
    let backtest_result = backtest.run(&strategy);
    println!("{}", backtest_result);

}
