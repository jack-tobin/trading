/// Trading strategies.

use crate::order::Order;
use crate::portfolio::Portfolio;
use crate::data_loading::DatedStockData;

pub trait Strategy {
    fn on_data(&self, data: Vec<DatedStockData>, portfolio: &Portfolio) -> Option<Order>;
}
