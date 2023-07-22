///
/// Brokerage objects.
/// May 2023
/// Jack Tobin
///
/// These objects are designed to receive orders from the strategy/user
/// and return back trade confirmations that detailt the amount of the
/// order filled, at what price, the timestamp, etc.
///

use rand_distr::{Normal, Uniform, Distribution};
use rand;
use crate::order::{Order, Confirm, OrderResult};
use crate::data_loading::{AlphaVantage, Quote};
use std::io::Result;

pub struct Broker {
    trading_costs: f64,
}

impl Broker {

    pub fn new(trading_costs: f64) -> Broker {
        Broker {
            trading_costs,
        }
    }

    pub fn quote(&self, ticker: &String) -> Result<Quote> {
        let av = AlphaVantage;
        let quote = av.get_quote(ticker)
            .unwrap();

        Ok(quote)
    }

    fn market_noise(&self, mean: f64, variance: f64) -> Result<f64> {
        let std_dev: f64 = variance.powf(1.0/2.0);
        let normal = Normal::new(mean, std_dev)
            .unwrap();

        let noise = normal.sample(&mut rand::thread_rng());

        Ok(noise)
    }

    fn market_slippage(&self, max_slippage: i64) -> Result<i64> {
        let uniform = Uniform::new(0, max_slippage);
        let slippage = uniform.sample(&mut rand::thread_rng());

        Ok(slippage)
    }

    fn executed_price(&self, quote: Quote) -> Result<f64> {
        let random_noise = match self.market_noise(0.0, 1.0) {
            Ok(noise) => noise,
            Err(error) => panic!("Error in computing market noise: {:?}", error),
        };
        let executed_price = quote.quote + random_noise;

        Ok(executed_price)
    }

    fn executed_quantity(&self, quantity_desired: i64) -> Result<i64> {
        let max_slippage = (0.25 * (quantity_desired as f64)) as i64;
        let slippage = match self.market_slippage(max_slippage) {
            Ok(slippage) => slippage,
            Err(error) => panic!("Error in computation of slippage: {:?}", error),
        };

        // If quantity desired is negative, need to add the slippage to the order.
        // Otherwise, subtract it.
        let executed_qty: i64;
        if quantity_desired < 0 {
            executed_qty = quantity_desired + slippage
        } else if quantity_desired > 0 {
            executed_qty = quantity_desired - slippage
        } else {
            executed_qty = 0
        }

        Ok(executed_qty)
    }

    fn send_order(&self, ticker: String, quoted_price: f64, quantity: i64) -> Result<OrderResult> {
        // Unpack executed price and quantity.
        let amount_filled = match self.executed_quantity(quantity) {
            Ok(amount) => amount,
            Err(error) => panic!("Error in executed quantity: {:?}.", error),
        };
        let price_filled = match self.executed_price(quoted_price) {
            Ok(price) => price,
            Err(error) => panic!("Error in executed price: {:?}.", error),
        };

        let result = OrderResult::new(
            ticker,
            amount_filled,
            price_filled,
        );

        Ok(result)
    }

    pub fn execute(&self, order: Order) -> Result<Confirm> {
        // Compute dollar trading costs.
        let trading_costs = self.trading_costs * (order.quantity as f64);

        // Get a quote for the ticker.
        let quote = match self.quote(&order.ticker) {
            Ok(quote) => quote,
            Err(error) => panic!("Error in quote generation: {:?}", error),
        };

        // execute at given quote.
        let result = match self.send_order(
            order.ticker.clone(),
            quote,
            order.quantity
        ) {
            Ok(result) => result,
            Err(error) => panic!("Error in sending order: {:?}", error),
        };

        let confirm = Confirm::new(
            order.ticker,
            result.filled_quantity,
            result.filled_price,
            trading_costs,
        );

        Ok(confirm)
    }
}
