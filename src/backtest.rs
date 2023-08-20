/// Backtesting

use crate::strategy::Strategy;
use crate::portfolio::Portfolio;
use crate::order::Order;
use crate::broker::Broker;
use polars::series::Series;
use std::fmt;
use std::error::Error;

pub struct BacktestResult {
    pnl: f64,
    n_trades: isize,
}
impl BacktestResult {
    pub fn new(pnl: f64, n_trades: isize) -> Self {
        Self {
            pnl,
            n_trades,
        }
    }
}
impl fmt::Display for BacktestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Backtest Result:\nPnL = {}\nNum. Trades = {}",
            self.pnl,
            self.n_trades,
        )
    }
}

pub struct Backtest {
    warm_up_periods: u64,
    portfolio: Portfolio,
    pnl: f64,
    n_trades: isize,
}
impl Backtest {
    pub fn new(warm_up_periods: u64, portfolio: Portfolio) -> Self {
        let pnl = 0.0;
        let n_trades = 0;
        Self {
            warm_up_periods,
            portfolio,
            pnl,
            n_trades,
        }
    }

    fn process_order(&mut self, order: Order) -> Result<(), Box<dyn Error>>{
        self.n_trades += 1;

        let broker = Broker::new(0.50);
        let order_result = broker.execute(order)?;
        println!("Traded {} shares for {}", order_result.quantity_filled, order_result.executed_price);

        // Compute PnLs.
        // Portfolio is LIFO. If we buy 100 shares for $10, our investment is $1000
        // if we then sell 50 shares for $12, our PnL is 50 (12 - 10)
        // if we then buy 25 shares for $15, then sell 30 shares for $20, our PnL
        // on that sale is 25 * (20-15) + 5 * (20 - 10)

        self.portfolio.position += order_result.quantity_filled;

        Ok(())
    }

    pub fn run(
        &mut self,
        strategy: &impl Strategy,
        data: &Series,
    ) -> Result<BacktestResult, Box<dyn Error>> {
        let n = data.len().try_into()?;

        for i in self.warm_up_periods..n {
            let data_slice = data.head(Some(i.try_into()?));

            match strategy.on_data(data_slice, &self.portfolio) {
                Some(order) => self.process_order(order)?,
                None => println!("Nothing to do."),
            }
        }
        let result = BacktestResult::new(
            self.pnl,
            self.n_trades,
        );
        Ok(result)
    }
}

