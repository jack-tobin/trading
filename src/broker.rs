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
use crate::order::{Order, Confirm};

pub struct Broker {
    trading_costs: f32,
}

impl Broker {

    pub fn new(trading_costs: f32) -> Broker {
        return Broker {
            trading_costs,
        };
    }

    pub fn quote(&self, ticker: &str, quantity: i64) -> f32 {
        return 3.0;
    }

    fn market_noise(&self, mean: f32, variance: f32) -> f32 {
        let stdev: f32 = variance.powf(1.0/2.0);
        let normal = Normal::new(mean, stdev).unwrap();
        normal.sample(&mut rand::thread_rng())
    }

    fn market_slippage(&self, max_slippage: i64) -> i64 {
        let uniform = Uniform::new(0, max_slippage);
        let slippage = uniform.sample(&mut rand::thread_rng());
        return slippage;
    }

    fn executed_price(&self, quote: &f32) -> f32 {
        let random_noise = self.market_noise(0.0, 1.0);
        return *quote + random_noise;
    }

    fn executed_quantity(&self, quantity_desired: i64) -> i64 {
        let max_slippage = (0.25 * (quantity_desired as f64)) as i64;
        let slippage = self.market_slippage(max_slippage);

        // If quantity desired is negative, need to add the slippage to the order.
        // Otherwise, subtract it.
        if quantity_desired < 0 {
            quantity_desired + slippage
        } else if quantity_desired > 0 {
            quantity_desired - slippage
        } else {
            0
        }
    }

    pub fn execute(&self, order: Order) -> Confirm {

        let quote = self.quote(&order.ticker, order.quantity);
        let executed_price = self.executed_price(&quote);
        let trading_costs = self.trading_costs * order.quantity as f32;
        let amount_filled = self.executed_quantity(order.quantity);

        let confirm = Confirm::new(
            order.ticker,
            amount_filled,
            executed_price,
            trading_costs,
        );

        return confirm;
    }
}
