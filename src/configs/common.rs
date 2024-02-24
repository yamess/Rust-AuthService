use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub app_host: String,
    pub app_port: u16,
    pub app_workers: Option<usize>,
    pub log_folder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    pub log_level: String,
    pub log_folder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_pool_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub logger: LogConfig,
    pub database: DatabaseConfig,
}

impl ApplicationConfig {
    pub fn new() -> Arc<ApplicationConfig> {
        let server_config = envy::from_env::<ServerConfig>().unwrap();
        let log_config = envy::from_env::<LogConfig>().unwrap();
        let database_config = envy::from_env::<DatabaseConfig>().unwrap();
        Arc::new(Self {
            server: server_config,
            logger: log_config,
            database: database_config,
        })
    }
}
