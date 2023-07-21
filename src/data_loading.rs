///
/// Data loading functions.
///

use crate::config::Config;
extern crate reqwest;
use std::io::Result;
use serde_aux::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Quote {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    quote: f64,
}

pub struct AlphaVantage;

impl AlphaVantage {

    // Get API key
    // use https://www.alphavantage.co/documentation/
    // Assumes that the api key is stored as an environment variable called AV_KEY
    fn get_api_key(&self) -> String {
        let config = Config::get("AV_KEY".to_string())
            .expect("Environment variable AV_KEY not found");

        config.api_key
    }

    // Get a quote for a ticker.
    pub fn get_quote(&self, ticker: String) -> Result<f64> {
        // Get API key.
        let api_key: String = self.get_api_key();

        // Build URL.
        let url_prefix: &str = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&";
        let url_suffix = format!("symbol={}&apikey={}", ticker, api_key);
        let url = format!("{}{}", url_prefix, url_suffix);

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

    pub fn _get_timeseries(&self, _ticker: String) -> Result<f64> {
        Ok(3.2)
    }
}
