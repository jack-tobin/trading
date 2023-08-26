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
use std::error::Error;
use derive_new::new;


#[derive(Debug, new)]
pub struct Broker {
    trading_costs: f64,
}
impl Broker {
    pub fn quote(&self, ticker: String, quantity: i64) -> Result<Quote, Box<dyn Error>> {
        let av = AlphaVantage;
        let quote = av.get_quote(ticker.clone(), quantity)?;
        Ok(quote)
    }

    fn market_noise(&self, mean: f64, variance: f64) -> Result<f64, Box<dyn Error>> {
        let std_dev: f64 = variance.powf(1.0/2.0);
        let normal = Normal::new(mean, std_dev)?;
        let noise = normal.sample(&mut rand::thread_rng());
        Ok(noise)
    }

    fn market_slippage(&self, max_slippage: i64) -> Result<i64, Box<dyn Error>> {
        let uniform = Uniform::new(0, max_slippage);
        let slippage = uniform.sample(&mut rand::thread_rng());

        Ok(slippage)
    }

    fn executed_price(&self, quote: &Quote) -> Result<f64, Box<dyn Error>> {
        let random_noise = self.market_noise(0.0, 1.0)?;
        let executed_price = quote.quote + random_noise;

        Ok(executed_price)
    }

    fn executed_quantity(&self, quantity_desired: i64) -> Result<i64, Box<dyn Error>> {
        let max_slippage = (0.25 * (quantity_desired as f64)) as i64;
        let slippage = self.market_slippage(max_slippage)?;

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

    fn send_order(&self, quote: Quote) -> Result<OrderResult, Box<dyn std::error::Error>> {
        let amount_filled = self.executed_quantity(quote.quantity)?;
        let price_filled = self.executed_price(&quote)?;

        let result = OrderResult::new(
            quote.ticker.clone(),
            amount_filled,
            price_filled,
        );

        Ok(result)
    }

    pub fn execute(&self, order: Order) -> Result<Confirm, Box<dyn std::error::Error>> {
        let trading_costs = self.trading_costs * (order.quantity as f64);
        let quote = self.quote(order.ticker.clone(), order.quantity)?;
        let result = self.send_order(quote)?;
        println!("Traded {} shares for {}", result.filled_quantity, result.filled_price);

        let confirm = Confirm::new(
            order.ticker.clone(),
            result.timestamp,
            result.filled_quantity,
            result.filled_price,
            trading_costs,
        );

        Ok(confirm)
    }
}
