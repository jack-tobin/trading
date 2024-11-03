

use std::env;
use std::error::Error;
use derive_new::new;

#[derive(Debug, new)]
pub struct Config {}
impl Config {
    pub fn get(key: String) -> Result<String, Box<dyn Error>> {
        // Assumes that the config key is stored as an environment variable.
        Ok(env::var(key)?)
    }
}
