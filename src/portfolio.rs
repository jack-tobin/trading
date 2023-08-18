
use std::io::Error;

pub struct Trade{
    price: f64,
    quantity: i64,
}

pub struct Portfolio {
    pub initial_capital: isize,
    pub position: i64,
    pub pnl: f64,
    pub trades: Vec<Trade>,
}
impl Portfolio {
    pub fn new(initial_capital: isize) -> Self {
        let position: i64 = 0;
        let pnl: f64 = 0.0;
        let trades: Vec<Trade> = vec![];
        Self {
            initial_capital,
            position,
            pnl,
            trades,
        }
    }

    pub fn is_long(&self) -> Result<bool, Error> {
        Ok(self.position > 0)
    }

    pub fn is_short(&self) -> Result<bool, Error> {
        Ok(self.position < 0)
    }
}