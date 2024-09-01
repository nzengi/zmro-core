// src/config/settings.rs

use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};

/// Settings struct holds the configuration data for the application.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub cache_url: String,
    pub api_key: String,
    pub evm_node_url: String,
    pub server_url: String,
}

impl Settings {
    /// Creates a new instance of Settings by loading from the configuration files and environment variables.
    ///
    /// # Returns
    ///
    /// * `Result<Settings, ConfigError>` - The loaded configuration settings or an error if loading fails.
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Load the default configuration file
        s.merge(File::with_name("config/default").required(false))?;

        // Load the environment-specific configuration
        let environment = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", environment)).required(false))?;

        // Override settings with environment variables, prefixed with "APP"
        s.merge(Environment::with_prefix("APP").separator("__"))?;

        // Deserialize the configuration into the Settings struct
        s.try_into()
    }
}
