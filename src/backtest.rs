/// Backtesting

use crate::strategy::Strategy;
use crate::portfolio::{Portfolio, Trade};
use crate::order::{Order, Confirm};
use crate::broker::Broker;
use crate::data_loading::Quote;
use polars::series::Series;
use std::error::Error;
use derive_new::new;

#[allow(dead_code)]
#[derive(Debug, new)]
pub struct BacktestResult {
    pnl: f64,
    n_trades: isize,
}

#[derive(Debug, new)]
pub struct Backtest {
    warm_up_periods: usize,
    portfolio: Portfolio,
    #[new(value = "Broker::new(0.50)")]
    broker: Broker,
    #[new(value = "0.0")]
    pnl: f64,
    #[new(value = "0")]
    n_trades: isize,
}
impl Backtest {
    fn process_order(&mut self, order: Order) -> Result<(), Box<dyn Error>>{
        let quote = self.broker.quote(order.ticker.clone(), order.quantity)?;
        let confirm = self.broker.execute(order)?;
        let _ = self.update_portfolio_with_confirm(confirm, quote)?;
        Ok(())
    }

    fn update_portfolio_with_confirm(&mut self, confirm: Confirm, quote: Quote) -> Result<(), Box<dyn Error>> {
        let trade = Trade::new(confirm.executed_price, confirm.quantity_filled);

        self.n_trades += 1;
        self.portfolio.trades.push(trade);
        self.portfolio.position += confirm.quantity_filled;
        self.portfolio.pnl += (self.portfolio.position as f64) * quote.change;
        self.portfolio.pnl -= confirm.trading_costs;

        Ok(())
    }

    pub fn run(
        &mut self,
        strategy: &impl Strategy,
        data: &Series,
    ) -> Result<BacktestResult, Box<dyn Error>> {
        let n = data.len();

        for i in self.warm_up_periods..n {
            let data_slice = data.head(Some(i));

            match strategy.on_data(data_slice, &self.portfolio) {
                Some(order) => self.process_order(order)?,
                None => (),
            }
        }

        Ok(
            BacktestResult::new(
                self.pnl,
                self.n_trades,
            )
        )
    }
}

