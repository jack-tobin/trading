use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyValueError;

mod broker;
mod order;
mod config;
mod data_loading;
mod backtest;
mod strategy;
mod portfolio;

use crate::data_loading::{AlphaVantage, Interval, Metadata};
use crate::portfolio::*;
use crate::strategy::*;
use crate::backtest::*;

#[pyclass]
struct BacktestResult {
    #[pyo3(get)]
    final_capital: f64,
    #[pyo3(get)]
    total_return: f64,
    #[pyo3(get)]
    trades: Vec<PyObject>,
}

#[pyfunction]
fn run_backtest(
    py: Python,
    strategy_type: &str,
    ticker: &str,
    window: u32,
    capital: i64,
    long_qty: i64,
    short_qty: i64,
) -> PyResult<Py<PyDict>> {
    let metadata = Metadata::new(ticker.to_string());
    let loader = AlphaVantage;

    let data = match loader.get_timeseries(&metadata.symbol, &Interval::Day) {
        Ok(data) => data,
        Err(e) => return Err(PyValueError::new_err(format!("Data loading error: {}", e))),
    };

    let portfolio = Portfolio::new(capital as isize);

    let strategy_factory: &mut StrategyFactory = get_strategy_factory();
    let strategy = strategy_factory.create(strategy_type, window, long_qty, short_qty)
        .expect("Unable to generate strategy.");

    let mut backtest = Backtest::new(window, portfolio);
    let result = match backtest.run(&*strategy, &data, &metadata) {
        Ok(r) => r,
        Err(e) => return Err(PyValueError::new_err(format!("Backtest error: {}", e))),
    };

    let result_dict = PyDict::new(py);
    result_dict.set_item("n_trades", result.n_trades)?;

    let trades = PyList::empty(py);
    for trade in &result.portfolio.trades {
        let trade_dict = PyDict::new(py);
        trade_dict.set_item("timestamp", trade.timestamp.to_string())?;
        trade_dict.set_item("ticker", &trade.ticker)?;
        trade_dict.set_item("quantity", trade.quantity)?;
        trade_dict.set_item("price", trade.price)?;
        trades.append(trade_dict)?;
    }
    result_dict.set_item("trades", trades)?;

    Ok(result_dict.into())
}


#[pymodule]
fn trading_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_backtest, m)?)?;
    Ok(())
}
