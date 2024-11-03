/// Backtesting

use crate::strategy::Strategy;
use crate::portfolio::{Portfolio, Trade};
use crate::order::Order;
use crate::broker::Broker;
use crate::data_loading::{DatedStockData, Metadata};
use std::error::Error;
use derive_new::new;
use log::{info, warn};


#[allow(dead_code)]
#[derive(Debug, new)]
pub struct BacktestResult {
    pnl: f64,
    n_trades: isize,
}

#[derive(Debug, new)]
pub struct Backtest {
    warm_up_periods: u32,
    portfolio: Portfolio,
    #[new(value = "Broker::new(0.50)")]
    broker: Broker,
    #[new(value = "0.0")]
    pnl: f64,
    #[new(value = "0")]
    n_trades: isize,
}
impl Backtest {
    pub fn run(
        &mut self,
        strategy: &impl Strategy,
        data: &Vec<DatedStockData>,
        metadata: &Metadata,
    ) -> Result<BacktestResult, Box<dyn Error>> {
        let n: u32 = data.len().try_into().unwrap();

        for i in self.warm_up_periods..n {
            let data_slice = data.get(0..i as usize).ok_or("Data slice out of bounds")?;

            match strategy.on_data(data_slice.to_vec(), &metadata, &self.portfolio) {
                Some(order) => {
                    let mut processor = OrderProcessor::new();
                    processor.process(order, &mut self.broker, &mut self.portfolio)?;
                    self.n_trades += 1;
                    self.pnl = self.portfolio.pnl;
                },
                None => (),
            }
        }

        Ok(BacktestResult::new(self.pnl, self.n_trades))
    }
}


#[derive(new)]
pub struct OrderProcessor {
    #[new(value = "0")]
    total_orders_processed: u32,
    #[new(value = "0.0")]
    total_value_processed: f64,
}

impl OrderProcessor {
    pub fn process(&mut self, order: Order, broker: &mut Broker, portfolio: &mut Portfolio) -> Result<(), Box<dyn Error>> {
        info!("Processing order: {:?}", order);

        let quote = broker.quote(order.ticker.clone(), order.quantity)?;
        info!("Received quote at price: ${:.2}", quote.quote);

        let confirm = broker.execute(order)?;
        info!("Order executed: {} shares at ${:.2}", confirm.quantity_filled, confirm.executed_price);

        if confirm.quantity_filled != quote.quantity {
            warn!("Partial fill: requested {} but filled {}",
                  quote.quantity, confirm.quantity_filled);
        }

        let trade = Trade::new(confirm.executed_price, confirm.quantity_filled);

        portfolio.trades.push(trade);
        portfolio.position += confirm.quantity_filled;
        portfolio.pnl += (portfolio.position as f64) * quote.change;
        portfolio.pnl -= confirm.trading_costs;

        self.total_orders_processed += 1;
        self.total_value_processed += confirm.executed_price * confirm.quantity_filled as f64;

        info!("Updated portfolio position: {}", portfolio.position);
        info!("Current P&L: ${:.2}", portfolio.pnl);

        Ok(())
    }
}
