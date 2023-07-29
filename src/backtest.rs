/// Backtesting

use crate::strategy::Strategy;
use polars::series::Series;
use std::fmt;
use std::io::Result;


pub struct BacktestResult {
    pnl: f64,
    n_trades: usize,
    drawdown: f64,
}
impl fmt::Display for BacktestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Backtest Result:\nPnL = {}\nNum. Trades = {}\nMax Drawdown = {}",
            self.pnl,
            self.n_trades,
            self.drawdown,
        )
    }
}

pub struct Backtest {
    warm_up_periods: usize
}
impl Backtest {
    pub fn new(warm_up_periods: usize) -> Self {
        Self {
            warm_up_periods,
        }
    }

    pub fn load_data(&self, data: Series) {
        ()
    }

    pub fn run(&self, strategy: &impl Strategy) -> Result<BacktestResult> {

        let pnl: f64 = 0.0;
        let n_trades: usize = 35;
        let drawdown: f64 = -0.10;



        let result = BacktestResult {
            pnl,
            n_trades,
            drawdown
        };
        Ok(result)
    }
}

