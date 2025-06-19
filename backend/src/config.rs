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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub database: u32,
}

pub async fn load_config(path: &str) -> Result<RedisConfig> {
    let raw = fs::read_to_string(path).await?; // non-blocking I/O
    Ok(toml::from_str(&raw)?) // serde-powered parse
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
    // 1️⃣  Read the file
    let cfg = load_config("config.toml").await.expect("valid config");

    // 2️⃣  Build redis:// URL.  The password goes after the ':'.
    let url = match &cfg.password {
        Some(pw) => format!("redis://:{}@{}:{}/{}", pw, cfg.host, cfg.port, cfg.database),
        None => format!("redis://{}:{}/{}", cfg.host, cfg.port, cfg.database),
    };

    // 3️⃣  Open client & wrap it in ConnectionManager (auto-reconnect)
    let client = Client::open(url).expect("redis url");
    ConnectionManager::new(client).await.expect("connect redis")
}
