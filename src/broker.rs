///
/// Brokerage objects.
/// May 2023
/// Jack Tobin
///
/// These objects are designed to receive orders from the strategy/user
/// and return back trade confirmations that detailt the amount of the
/// order filled, at what price, the timestamp, etc.
///

use rand_distr::Normal;
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
    /// Normal<f32>
    ///     A normally distributed float.
    ///
    fn market_noise(&self, mean: f32, variance: f32) -> Normal<f32> {
        let stdev: f32 = variance.powf(1.0/2.0);
        let noise = Normal::new(0.0, stdev).unwrap();
        return noise;
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

    pub fn execute(&self, order: Order) -> Confirm {

        return;
    }
}
