///
/// Basic trading infrastructure.
/// May 2023
/// Jack Tobin
///

mod broker;
mod order;
mod config;

use broker::Broker;
use order::{Order, Confirm};
use config::get_api_key;

fn main() {
    // Establish our broker with $0.05 trading costs.
    let broker: Broker = Broker::new(0.05);

    // Make an order for +100 AAPL shares.
    let ticker: String = "AAPL".to_string();
    let shares = 100;
    let order = Order::new(ticker, shares);

    // Instruct broker to execute my order.
    let confirm: Confirm = broker.execute(order);

    println!("{}", confirm);

    // Get API key
    get_api_key();
}
