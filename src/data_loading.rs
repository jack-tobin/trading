///
/// Data loading functions.
///

use crate::config::Config;
use polars::series::Series;
use crate::order::Quote;
use reqwest::blocking::get;
use crate::errors::*;

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

pub struct AlphaVantage;
impl AlphaVantage {
    const BASE_URL: &str = "https://www.alphavantage.co/query";

    // Assumes that the api key is stored as an environment variable called AV_KEY
    fn get_api_key(&self) -> Result<String, DataLoaderError> {
        let config = Config::get("AV_KEY".to_string())?;

        Ok(config.api_key)
    }

    fn get_url(&self, function: String, symbol: String) -> Result<String, DataLoaderError> {
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

    pub fn get_quote(&self, ticker: String, quantity: i64) -> Result<Quote, DataLoaderError> {
        let function = "GLOBAL_QUOTE".to_string();
        let url = self.get_url(function, ticker.clone())?;

        let response = get(url)?.json::<serde_json::Value>()?;

        let price = &response["Global Quote"]["05. price"];
        let price_string: String = serde_json::from_value(price.clone())?;
        let parsed = price_string.parse::<f64>()?;

        let quote = Quote::new(
            ticker.clone(),
            parsed,
            quantity,
        );

        Ok(quote)
    }

    pub fn get_timeseries(
        &self,
        ticker: String,
        interval: Interval,
    ) -> Result<Series, DataLoaderError> {
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

        let url = self.get_url(function.to_string(), ticker.clone())?;

        let response = get(url)?.json::<serde_json::Value>()?;

        println!("{}", response);

        // TODO: Coerce json response to a polars series.

        let series = [1, 2, 3].iter().collect();
        Ok(series)
    }
}
