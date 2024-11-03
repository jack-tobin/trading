///
/// Data loading functions.
///

use crate::config::Config;
use reqwest::blocking::{Response, get};
use std::error::Error;
use serde_json::Value;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use derive_new::new;
use std::collections::HashMap;


#[allow(dead_code)]
#[derive(Debug, new)]
pub struct Quote {
    pub ticker: String,
    pub quote: f64,
    pub change: f64,
    pub quantity: i64,
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, new)]
pub struct Metadata {
    pub symbol: String,
}


#[allow(dead_code)]
pub enum Interval {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TimeSeriesResponse {
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    ts_data: HashMap<String, StockData>,
}


#[allow(dead_code)]
#[derive(Deserialize)]
struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    output_size: String,
    #[serde(rename = "5. Time Zone")]
    tz: String,
}

#[allow(dead_code)]
#[derive(Debug, new, Clone)]
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
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[derive(Debug)]
pub struct AlphaVantage;
impl AlphaVantage {
    const BASE_URL: &str = "https://www.alphavantage.co/query";
    const CONFIG_KEY: &str = "AV_KEY";

    pub fn get_quote(
        &self,
        ticker: String,
        quantity: i64,
    ) -> Result<Quote, Box<dyn Error>> {
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

    fn get_url(
        &self,
        function: String,
        symbol: String,
    ) -> Result<String, Box<dyn Error>> {
        let api_key = Config::get(Self::CONFIG_KEY.to_string())?;
        let url_suffix = format!(
            "?function={}&symbol={}&apikey={}",
            function,
            symbol,
            api_key,
        );
        let url = [Self::BASE_URL.to_string(), url_suffix].join("");

        Ok(url)
    }

    fn _unpack_json_quote_data(&self, value: &Value) -> Result<f64, Box<dyn Error>> {
        let price_string = value.as_str()
            .ok_or("Failed to convert value to string")?
            .replace("\"", "");
        Ok(price_string.parse::<f64>()?)
    }

    fn _unpack_response_to_json(&self, response: Response) -> Result<Value, Box<dyn Error>> {
        Ok(response.json::<Value>()?)
    }

    pub fn get_timeseries(
        &self,
        ticker: String,
        interval: &Interval,
    ) -> Result<Vec<DatedStockData>, Box<dyn Error>> {
        let function = self._api_function_from_interval(interval)?;
        let url = self.get_url(function, ticker.clone())?;
        let response = get(&url)?;
        if !response.status().is_success() {
            return Err(format!("Failed to get timeseries data: {}", response.status()).into());
        }
        let timeseries = response.json::<TimeSeriesResponse>()?;
        self._unpack_ts_data(timeseries)
    }

    fn _api_function_from_interval(
        &self,
        interval: &Interval,
    ) -> Result<String, Box<dyn Error>> {
        let function = match interval {
            Interval::Minute => "TIME_SERIES_INTRADAY&interval=1min",
            Interval::Hour => "TIME_SERIES_INTRADAY&interval=60min",
            Interval::Day => "TIME_SERIES_DAILY",
            Interval::Week => "TIME_SERIES_WEEKLY",
            Interval::Month => "TIME_SERIES_MONTHLY",
        };
        Ok(function.to_string())
    }

    fn _unpack_ts_data(
        &self,
        ts: TimeSeriesResponse,
    ) -> Result<Vec<DatedStockData>, Box<dyn Error>> {
        let ts_map = ts.ts_data;
        let mut rows: Vec<DatedStockData> = vec![];
        for (date, stock_data) in ts_map.iter() {
            let dated_stockdata = DatedStockData::new(
                date.clone(),
                stock_data.open.replace("\"", "").parse::<f64>()?,
                stock_data.high.replace("\"", "").parse::<f64>()?,
                stock_data.low.replace("\"", "").parse::<f64>()?,
                stock_data.close.replace("\"", "").parse::<f64>()?,
                stock_data.volume.replace("\"", "").parse::<u64>()?,
            );
            rows.push(dated_stockdata);
        }
        Ok(rows)
    }

    fn _ts_key_from_interval(
        &self,
        interval: &Interval,
    ) -> Result<String, Box<dyn Error>> {
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
