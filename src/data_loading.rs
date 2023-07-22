///
/// Data loading functions.
///

use crate::config::Config;
extern crate reqwest;
use std::io::Result;
use serde_aux::prelude::*;
use polars::series::Series;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Quote {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    quote: f64,
}

#[derive(Debug)]
enum Interval {
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

    // Get API key
    // Assumes that the api key is stored as an environment variable called AV_KEY
    fn get_api_key(&self) -> String {
        let config = Config::get("AV_KEY".to_string())
            .expect("Environment variable AV_KEY not found");

        config.api_key
    }

    // Get a quote for a ticker.
    pub fn get_quote(&self, ticker: &String) -> Result<f64> {
        // Get API key.
        let api_key: String = self.get_api_key();

        // Build URL.
        let url_prefix: &str = "?function=GLOBAL_QUOTE&";
        let url_suffix = format!("symbol={}&apikey={}", ticker, api_key);
        let url = format!("{}{}{}", Self::BASE_URL, url_prefix, url_suffix);

        // Make request.
        let response = reqwest::blocking::get(url)
            .unwrap()
            .json::<serde_json::Value>()
            .expect("Error in coercing response to json.");

        // Extract price from json response.
        let price = &response["Global Quote"]["05. price"];
        let price_string: String = serde_json::from_value(price.clone())
            .expect("Error in parsing price into a String.");
        let parsed = price_string.parse::<f64>()
            .expect("Unable to parse string into f64.");

        let quote = Quote{
            quote: parsed
        };

        Ok(quote.quote)
    }

    pub fn _get_timeseries(
        &self,
        ticker: String,
        interval: Interval,
    ) -> Result<Series> {
        // Get API key.
        let api_key: String = self.get_api_key();

        // Determine function based on frequency:
        let function = match interval {
            Interval::Minute => "TIME_SERIES_INTRADAY&interval=1min",
            Interval::FiveMinute => "TIME_SERIES_INTRADAY&interval=5min",
            Interval::FifteenMinute => "TIME_SERIES_INTRADAY&interval=15min",
            Interval::ThirtyMinute => "TIME_SERIES_INTRADAY&interval=30min",
            Interval::Hour => "TIME_SERIES_INTRADAY&interval=60min",
            Interval::Day => "TIME_SERIES_DAILY",
            Interval::Week => "TIME_SERIES_WEEKLY",
            Interval::Month => "TIME_SERIES_MONTHLY",
            _ => panic!("Invalid interval: {:?}", interval),
        };

        // Build URL.
        let url_prefix = format!("?function={}&", function);
        let url_suffix = format!("symbol={}&apikey={}", ticker, api_key);
        let url = format!("{}{}{}", Self::BASE_URL, url_prefix, url_suffix);

        // Make request.
        let response = reqwest::blocking::get(url)
            .unwrap()
            .json::<serde_json::Value>()
            .expect("Error in coercing response to json.");

        println!("{}", response);

        // Coerce json response to a polars series.

        let series = Series::new("x", [1, 2, 3]);
        Ok(series)
    }
}
