/// Trading strategies.

use polars::prelude::*;

use crate::order::Order;
use crate::portfolio::Portfolio;

pub trait Strategy {
    fn on_data(&self, data: Series, portfolio: &Portfolio) -> Option<Order>;
}
