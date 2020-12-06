use std::fs;
use serde::Deserialize;
use std::io::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub ledger_name: String,
    pub http_port: u32
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        let contents = fs::read_to_string("Config.toml")?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}