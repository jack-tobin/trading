

use std::env;
use std::error::Error;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn get(key: String) -> Result<Self, Box<dyn Error>> {
        let result = env::var(key)?;
        Ok(
            Self {
                api_key: result,
            }
        )
    }
}
