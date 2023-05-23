

use std::env;

pub struct Config {
    pub av_key: String,
}

impl Config {
    pub fn get(key: String) -> Result<Config, &'static str> {
        let result = env::var(key).expect("Key not found.");
        Ok(
            Config {
                av_key: result,
            }
        )
    }
}
