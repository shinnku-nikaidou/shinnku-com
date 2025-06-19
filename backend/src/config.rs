use anyhow::Result;
use redis::{Client, aio::ConnectionManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{fs, sync::OnceCell};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_path: String,
    pub upload_timestamp: u64,
    pub file_size: u64,
}

pub type BucketFiles = Vec<FileInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeValue {
    File(FileInfo),
    Node(TreeNode),
}

pub type TreeNode = HashMap<String, NodeValue>;

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

static REDIS: OnceCell<ConnectionManager> = OnceCell::const_new();

/// Call this from anywhere in your app to get the live connection.
pub async fn redis() -> &'static ConnectionManager {
    REDIS
        .get_or_init(init_connection) // async initialiser
        .await
}

/// Does the heavy lifting once.
async fn init_connection() -> ConnectionManager {
    let settings = load_config("config.toml").await.expect("load config");
    let cfg = &settings.redis;
    let url = match &cfg.password {
        Some(pw) => format!("redis://:{}@{}:{}/{}", pw, cfg.host, cfg.port, cfg.database),
        None => format!("redis://{}:{}/{}", cfg.host, cfg.port, cfg.database),
    };
    let client = Client::open(url).expect("redis url");
    ConnectionManager::new(client).await.expect("connect redis")
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test requires a running Redis instance and a valid config.toml.
    // It checks that the REDIS connection manager can be initialized and used.
    #[tokio::test]
    async fn test_redis_get_set() {
        // Skip if no config.toml is present
        if !std::path::Path::new("config.toml").exists() {
            eprintln!("Skipping redis test: config.toml not found");
            return;
        }
        let conn = redis().await;
        let mut con: ConnectionManager = conn.clone();
        let key = "img:wiki:zh:5406655";
        let res: String = redis::cmd("GET")
            .arg(key)
            .query_async(&mut con)
            .await
            .unwrap();
        println!("Value for {}: {}", key, res);
    }
}
