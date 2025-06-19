use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
