///
/// Data loading functions.
///

use crate::config::Config;
use reqwest::blocking::{Response, get};
use std::error::Error;
use std::fs;
use serde_json::{Value, from_str};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use derive_new::new;
use std::collections::HashMap;


#[derive(Debug, new)]
pub struct Quote {
    pub ticker: String,
    pub quote: f64,
    pub change: f64,
    pub quantity: i64,
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
}


#[allow(dead_code)]
pub enum Interval {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

#[derive(Deserialize)]
struct TimeSeriesResponse {
    #[serde(alias = "Meta Data")]
    meta_data: Option<HashMap<String, String>>,
    #[serde(flatten, alias = "Time Series (Daily)")]  // TODO: dynamize.
    ts_data: Option<HashMap<String, HashMap<String, StockData>>>,
}

#[derive(Deserialize)]
struct MetaData {
    #[serde(alias = "1. Information")]
    information: String,
    #[serde(alias = "2. Symbol")]
    symbol: String,
    #[serde(alias = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(alias = "4. Output Size")]
    output_size: String,
    #[serde(alias = "5. Time Zone")]
    tz: String,
}


#[derive(Debug, new)]
pub struct DatedStockData {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Deserialize)]
struct StockData {
    #[serde(alias = "1. open")]
    open: f64,
    #[serde(alias = "2. high")]
    high: f64,
    #[serde(alias = "3. low")]
    low: f64,
    #[serde(alias = "4. close")]
    close: f64,
    #[serde(alias = "5. volume")]
    volume: u64,
}

#[derive(Debug)]
pub struct AlphaVantage;
impl AlphaVantage {
    const BASE_URL: &str = "https://www.alphavantage.co/query";

    pub fn get_quote(&self, ticker: String, quantity: i64) -> Result<Quote, Box<dyn Error>> {
        let function = "GLOBAL_QUOTE".to_string();
        let url = self.get_url(function, ticker.clone())?;
        let response = get(url)?;
        let json = self._unpack_response_to_json(response)?;
        let quote = self._unpack_json_quote_data(&json["Global Quote"]["05. price"])?;
        let change = self._unpack_json_quote_data(&json["Global Quote"]["09. change"])?;

        Ok(
            Quote::new(
                ticker.clone(),
                quote,
                change,
                quantity,
            )
        )
    }

    fn get_url(&self, function: String, symbol: String) -> Result<String, Box<dyn Error>> {
        let api_key = self.get_api_key()?;
        let url_suffix = format!(
            "?function={}&symbol={}&apikey={}",
            function,
            symbol,
            api_key,
        );
        let url = [Self::BASE_URL.to_string(), url_suffix].join("");

        Ok(url)
    }

    fn get_api_key(&self) -> Result<String, Box<dyn Error>> {
        let config = Config::get("AV_KEY".to_string())?;
        Ok(config.api_key)
    }

    fn _unpack_json_quote_data(&self, value: &Value) -> Result<f64, Box<dyn Error>> {
        let price_string: String = serde_json::to_string(value)?;
        Ok(price_string.parse::<f64>()?)
    }

    fn _unpack_response_to_json(&self, response: Response) -> Result<Value, Box<dyn Error>> {
        Ok(response.json::<Value>()?)
    }

    pub fn get_timeseries(&self, ticker: String, interval: Interval) -> Result<Vec<DatedStockData>, Box<dyn Error>> {
        // Temp: read and pare json output from file.
        let response = fs::read_to_string("example_json_output.json")?;

        // let function = self._api_function_from_interval(interval)?;
        // let url = self.get_url(function, ticker.clone())?;
        // let response = get(url)?
        //     .text()?;

        let timeseries: TimeSeriesResponse = from_str(response.as_str())?;
        self._unpack_ts_data(timeseries, interval)
    }

    fn _api_function_from_interval(&self, interval: Interval) -> Result<String, Box<dyn Error>> {
        let function = match interval {
            Interval::Minute => "TIME_SERIES_INTRADAY&interval=1min",
            Interval::Hour => "TIME_SERIES_INTRADAY&interval=60min",
            Interval::Day => "TIME_SERIES_DAILY",
            Interval::Week => "TIME_SERIES_WEEKLY",
            Interval::Month => "TIME_SERIES_MONTHLY",
        };
        Ok(function.to_string())
    }

    fn _unpack_ts_data(&self, ts: TimeSeriesResponse, interval: Interval) -> Result<Vec<DatedStockData>, Box<dyn Error>> {
        let ts_key = self._ts_key_from_interval(interval)?;

        let ts_map = ts.ts_data
        .expect("Timeseries object not found.");
        let ts = ts_map.get(ts_key.as_str())
            .expect("Time series object not found.");

        let mut rows: Vec<DatedStockData> = vec![];
        for (date, stock_data) in ts.iter() {
            let dated_stockdata = DatedStockData::new(
                date.clone(),
                stock_data.open,
                stock_data.high,
                stock_data.low,
                stock_data.close,
            stock_data.volume,
            );
            rows.push(dated_stockdata);
        }

        // Assemble into series
        Ok(rows)
    }

    fn _ts_key_from_interval(&self, interval: Interval) -> Result<String, Box<dyn Error>> {
        let key = match interval {
            Interval::Minute => "Time Series (1min)",
            Interval::Hour => "Time Series (60min)",
            Interval::Day => "Time Series (Daily)",
            Interval::Week => "Weekly Time Series",
            Interval::Month => "Monthly Time Series",
        };
        Ok(key.to_string())
    }
}
