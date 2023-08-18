

use std::env;
use crate::errors::*;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn get(key: String) -> Result<Self, ConfigError> {
        let result = env::var(key)?;
        Ok(
            Self {
                api_key: result,
            }
        )
    }
}
