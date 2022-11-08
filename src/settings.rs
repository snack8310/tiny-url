use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u32,
    pub ip: String,
}

impl Server {
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size : u32,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        const CURRENT_DIR: &str = "./config/Settings.toml";

        let s = Config::builder().add_source(config::File::with_name(CURRENT_DIR)).build()?;

        s.try_deserialize()
    }
}