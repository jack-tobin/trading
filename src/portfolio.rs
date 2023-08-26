
use derive_new::new;

#[derive(Debug, new)]
pub struct Trade {
    pub price: f64,
    pub quantity: i64,
}

#[derive(Debug, new)]
pub struct Portfolio {
    pub capital: isize,
    #[new(value = "0")]
    pub position: i64,
    #[new(value = "0.0")]
    pub pnl: f64,
    #[new(value = "vec![]")]
    pub trades: Vec<Trade>,
}
impl Portfolio {
    pub fn is_long(&self) -> bool {
        self.position > 0
    }

    pub fn is_short(&self) -> bool {
        self.position < 0
    }

    pub fn is_not_long(&self) -> bool {
        !self.is_long()
    }

    pub fn is_not_short(&self) -> bool {
        !self.is_short()
    }
}