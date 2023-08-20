///
/// Data loading functions.
///

use crate::config::Config;
use crate::order::Quote;
use reqwest::blocking::{Response, get};
use std::error::Error;
use std::io::Cursor;
use polars::prelude::*;
use serde_json::Value;
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub enum Interval {
    Minute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    Hour,
    Day,
    Week,
    Month,
}

#[derive(Deserialize)]
struct TimeSeriesResponse {
    #[serde(alias = "Meta Data")]
    meta_data: MetaData,
    #[serde(alias = "Time Series (Daily)")]
    ts_data: Vec<DatedStockData>,
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

#[derive(Deserialize)]
struct DatedStockData {
    date: String,
    data: StockData,
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

pub struct AlphaVantage;
impl AlphaVantage {
    const BASE_URL: &str = "https://www.alphavantage.co/query";

    pub fn get_quote(&self, ticker: String, quantity: i64) -> Result<Quote, Box<dyn Error>> {
        let function = "GLOBAL_QUOTE".to_string();
        let url = self.get_url(function, ticker.clone())?;
        let response = get(url)?;
        let json = self._unpack_response_to_json(response)?;
        println!("{}", json);
        let quote = self._unpack_json_quote_data(&json["Global Quote"]["05. price"])?;

        Ok(
            Quote::new(
                ticker.clone(),
                quote,
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

    pub fn get_timeseries(&self, ticker: String, interval: Interval) -> Result<DataFrame, Box<dyn Error>> {
        let function = self._api_function_from_interval(interval)?;
        let url = self.get_url(function, ticker.clone())?;
        let response = get(url)?
            .json::<TimeSeriesResponse>()?;

        let results = self.unpack_ts_data(response.ts_data)?;
        Ok(results)
    }

    fn _api_function_from_interval(&self, interval: Interval) -> Result<String, Box<dyn Error>> {
        let function = match interval {
            Interval::Minute => "TIME_SERIES_INTRADAY&interval=1min",
            Interval::FiveMinute => "TIME_SERIES_INTRADAY&interval=5min",
            Interval::FifteenMinute => "TIME_SERIES_INTRADAY&interval=15min",
            Interval::ThirtyMinute => "TIME_SERIES_INTRADAY&interval=30min",
            Interval::Hour => "TIME_SERIES_INTRADAY&interval=60min",
            Interval::Day => "TIME_SERIES_DAILY",
            Interval::Week => "TIME_SERIES_WEEKLY",
            Interval::Month => "TIME_SERIES_MONTHLY",
        };
        Ok(function.to_string())
    }

    fn unpack_ts_data(&self, ts_data: Vec<DatedStockData>) -> Result<DataFrame, Box<dyn Error>> {

        let mut dates: Vec<String> = vec![];
        let mut opens: Vec<f64> = vec![];
        let mut highs: Vec<f64> = vec![];
        let mut lows: Vec<f64> = vec![];
        let mut closes: Vec<f64> = vec![];
        let mut volumes: Vec<u64> = vec![];
        for data in ts_data.iter() {
            dates.push(data.date.clone());
            opens.push(data.data.open);
            highs.push(data.data.high);
            lows.push(data.data.low);
            closes.push(data.data.close);
            volumes.push(data.data.volume);
        }

        let result_df = df!(
            "date" => dates,
            "open" => opens,
            "high" => highs,
            "low"  => lows,
            "close" => closes,
            "volume" => volumes,
        )?;

        Ok(result_df)
    }
}
