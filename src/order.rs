///
/// Orders.
///
/// This contains various objects related to orders, including an Order
/// struct and a Confirm struct.
///

use chrono::DateTime;

/// Order struct.
///
/// An order object that contains a timestamp signifying the time the order
/// was submitted, and a ticker and quantity signifying the ticker desired
/// and the quantity desired. Quantity can be negative indicating a sale.
///
pub struct Order {
    timestamp: DateTime,
    ticker: &str,
    quantity: i64,
}

/// Confirm.
///
/// A trade confirmation object. This represents the output of a broker
/// transaction and contains a timestamp of when the order was executed,
/// the ticker the order was for, the quantity filled and the execution price
/// at which the order was filled, as well as any explicit trading costs
/// associated with the order.
///
pub struct Confirm {
    timestamp: DateTime,
    ticker: &str,
    quantity_filled: i64,
    executed_price: f32,
    trading_costs: f32,
}
