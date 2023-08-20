

use std::env;
use std::error::Error;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn get(key: String) -> Result<Self, Box<dyn Error>> {
        // Assumes that the config key is stored as an environment variable.
        let result = env::var(key)?;
        Ok(
            Self {
                api_key: result,
            }
        )
    }
}
