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
pub struct AuthConfig {
    pub secret_key: String,
    pub token_expire_minutes: i64,
    pub issuer: String,
    pub audience: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub logger: LogConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

impl ApplicationConfig {
    pub fn new() -> Arc<ApplicationConfig> {
        let server_config = envy::from_env::<ServerConfig>().unwrap();
        let log_config = envy::from_env::<LogConfig>().unwrap();
        let database_config = envy::from_env::<DatabaseConfig>().unwrap();
        let auth_config = envy::from_env::<AuthConfig>().unwrap();
        Arc::new(Self {
            server: server_config,
            logger: log_config,
            database: database_config,
            auth: auth_config,
        })
    }
}
