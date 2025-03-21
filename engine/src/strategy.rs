/// Trading strategies.

use derive_new::new;
use crate::order::Order;
use crate::portfolio::Portfolio;
use crate::data_loading::{DatedStockData, Metadata};
use std::collections::HashMap;
use std::sync::Once;


pub trait Strategy {
    fn on_data(&self, data: Vec<DatedStockData>, metadata: &Metadata, portfolio: &Portfolio) -> Option<Order>;
}


#[derive(Debug, new)]
pub struct MACrossoverStrategy {
    window: u32,
    long_quantity: i64,
    short_quantity: i64,
}

impl Strategy for MACrossoverStrategy {
    fn on_data(
        &self,
        data: Vec<DatedStockData>,
        metadata: &Metadata,
        portfolio: &Portfolio,
    ) -> Option<Order> {
        // MA crossover strategy strategy
        // If price is greater than avg price over a window, buy or maintain
        // If price is lower than avg price over a window, sell or maintain.
        // Otherwise do nothing.

        let ticker = metadata.symbol.clone();

        let n = data.len();
        let data_subset = if n >= self.window as usize {
            data[(n - self.window as usize)..].to_vec()
        } else {
            return None;
        };

        let prices: Vec<f64> = data_subset.iter()
            .map(|x| x.close)
            .collect();

        let subset_mean = prices.iter().sum::<f64>() / prices.len() as f64;
        let last_price = prices.last()?;

        if (*last_price > subset_mean) & portfolio.is_not_long() {
            Some(Order::new(ticker, self.long_quantity))
        }
        else if (*last_price < subset_mean) & portfolio.is_not_short() {
            Some(Order::new(ticker, self.short_quantity))
        }
        else {
            None
        }
    }
}


type StrategyConstructor = Box<dyn Fn(u32, i64, i64) -> Box<dyn Strategy>>;

pub struct StrategyFactory {
    strategies: HashMap<String, StrategyConstructor>
}

impl StrategyFactory {
    pub fn register(&mut self, name: &str, constructor: StrategyConstructor) {
        self.strategies.insert(name.to_string(), constructor);
    }

    pub fn create(&self, name: &str, window: u32, long_qty: i64, short_qty: i64) -> Option<Box<dyn Strategy>> {
        self.strategies.get(name)
            .map(|constructor| constructor(window, long_qty, short_qty))
    }
}

static mut STRATEGY_FACTORY: Option<StrategyFactory> = None;
static INIT: Once = Once::new();

pub fn get_strategy_factory() -> &'static mut StrategyFactory {
    unsafe {
        INIT.call_once(|| {
            let mut factory = StrategyFactory {
                strategies: HashMap::new()
            };

            factory.register("ma_crossover", Box::new(|w, l, s| {
                Box::new(MACrossoverStrategy::new(w, l, s))
            }));

            STRATEGY_FACTORY = Some(factory);
        });

        STRATEGY_FACTORY.as_mut().unwrap()
    }
}
