

use std::env;
use std::error::Error;
use derive_new::new;

#[derive(Debug, new)]
pub struct Config {
    pub api_key: String,
}
impl Config {
    pub fn get(key: String) -> Result<Self, Box<dyn Error>> {
        // Assumes that the config key is stored as an environment variable.
        Ok(
            Self { api_key: env::var(key)? }
        )
    }
}
