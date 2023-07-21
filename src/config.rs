

use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn get(key: String) -> Result<Config, &'static str> {
        let result = env::var(key).expect("Key not found.");
        Ok(
            Config {
                api_key: result,
            }
        )
    }
}
