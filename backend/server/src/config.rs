use anyhow::Result;
use redis::{Client, aio::ConnectionManager};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub database: u32,
}

pub async fn load_config(path: &str) -> Result<Settings> {
    let raw = fs::read_to_string(path).await?;
    Ok(toml::from_str::<Settings>(&raw)?)
}

/// Establish a new Redis connection using `config.toml`.
pub async fn connect_redis() -> Result<ConnectionManager> {
    let settings = load_config("config.toml").await?;
    let cfg = &settings.redis;
    let url = match &cfg.password {
        Some(pw) => format!("redis://:{}@{}:{}/{}", pw, cfg.host, cfg.port, cfg.database),
        None => format!("redis://{}:{}/{}", cfg.host, cfg.port, cfg.database),
    };
    let client = Client::open(url)?;
    Ok(ConnectionManager::new(client).await?)
}
