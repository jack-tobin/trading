/// Trading strategies.

use crate::order::Order;
use crate::portfolio::Portfolio;
use crate::data_loading::{DatedStockData, Metadata};

pub trait Strategy {
    fn on_data(&self, data: Vec<DatedStockData>, metadata: &Metadata, portfolio: &Portfolio) -> Option<Order>;
}
