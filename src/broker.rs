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

    /// Create an instance of a Broker.
    ///
    /// Parameters
    /// ----------
    /// trading costs : f32
    ///     Trading costs per contract for this Broker.
    ///
    pub fn new(trading_costs: f32) -> Broker {
        return Broker {
            trading_costs,
        };
    }

    /// Fetch a quote from the broker's pricing database.
    ///
    /// This method pings the broker's database for a quote for a given
    /// ticker for a given number of contracts/shares.
    ///
    /// Parameters
    /// ----------
    /// ticker : &str
    ///     A ticker for a given financial asset, e.g. 'AAPL'
    /// quantity : i64
    ///     The number of shares/contracts desired.
    ///
    /// Returns
    /// -------
    /// f32
    ///     A 32bit float that is the broker's quote for the given trade.
    ///
    pub fn quote(&self, ticker: &str, quantity: i64) -> f32 {
        return 3.0;
    }

    /// Produce market noise.
    ///
    /// This produces a random number normally distributed with given mean and variance.
    ///
    /// Parameters
    /// ----------
    /// mean : f32
    ///     Mean of the distribution
    /// variance : f32
    ///     Variance of the distribution.
    ///
    /// Returns
    /// -------
    /// f32
    ///     A normally distributed float.
    ///
    fn market_noise(&self, mean: f32, variance: f32) -> f32 {
        let stdev: f32 = variance.powf(1.0/2.0);
        let normal = Normal::new(mean, stdev).unwrap();
        normal.sample(&mut rand::thread_rng())
    }

    /// Produce market slippage.
    /// 
    /// This produces a random uniformly distributed number no greater
    /// than a given max slippage amount.
    /// 
    /// Parameters
    /// ----------
    /// max_slippage : i64
    ///     The maximum amount of possible market slippage.
    /// 
    /// Returns
    /// -------
    /// i64
    ///     Returns the number of shares missed by execution.
    ///     
    fn market_slippage(&self, max_slippage: i64) -> i64 {
        let uniform = Uniform::new(0, max_slippage);
        let slippage = uniform.sample(&mut rand::thread_rng());
        return slippage;
    }

    /// Get executed price for a given trade.
    ///
    /// Once the broker goes to market, there is some variability with the price
    /// at which the order will be filled. This method performs the action of
    /// approaching the market with a given quote and then returning with
    /// the order executed at a slightly different price.
    ///
    /// We'll assume that the delta is normally distributed with mean of zero
    /// and unit variance.
    ///
    /// Parameters
    /// ----------
    /// quote : &f32
    ///     A reference to the quoted price of the asset.
    ///
    /// Returns
    /// -------
    /// f32
    ///     The executed price e.g. the quote plus random noise.
    ///
    fn executed_price(&self, quote: &f32) -> f32 {
        let random_noise = self.market_noise(0.0, 1.0);
        return *quote + random_noise;
    }

    /// Get executed quantity for a given trade.
    /// 
    /// Once broker goes to market, there is variability with respect to the
    /// number of shares in the order that the broker is able to fill. This
    /// method performs the action of approaching the market with a given
    /// number of desired shares and then returning with the order executed
    /// at a slightly lower number of shares (amount of shares returned 
    /// will be strictly less than the amount requested.)
    /// 
    /// We'll assume the delta is uniformly distributed between 0 and 25%
    /// of the order size.
    /// 
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

    /// Execute a trade.
    /// 
    /// This executes an order, returning back to the user a trade confirmation
    /// which details the quantity filled and price at which it was filled.
    /// 
    /// Parameters
    /// ----------
    /// order : Order
    ///     Order to execute.
    /// 
    /// Returns
    /// -------
    /// Confirm
    ///     Returns a trade confirmation.
    /// 
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
