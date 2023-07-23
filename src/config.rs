

use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn get(key: String) -> Result<Self, &'static str> {
        let result = env::var(key).expect("Key not found.");
        Ok(
            Self {
                api_key: result,
            }
        )
    }
}
