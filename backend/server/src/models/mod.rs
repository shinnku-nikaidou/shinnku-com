use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    pub file_path: String,
    pub upload_timestamp: u64,
    pub file_size: u64,
}

pub type BucketFiles = Vec<FileInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeType {
    File(FileInfo),
    Node(TreeNode),
}

pub type TreeNode = HashMap<String, NodeType>;
