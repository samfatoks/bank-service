use ::config::ConfigError;
use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub ledger_name: String,
    pub session_pool_size: u16,
}
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_from_env_ok() {
        dotenv().ok();
        let config_result = Config::from_env();
        assert!(config_result.is_ok());
    }

    #[test]
    fn test_from_env_failure_when_env_vars_not_set() {
        let config_result = Config::from_env();
        assert!(config_result.is_err());
    }
}
