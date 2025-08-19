use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    pub file_path: String,
    pub upload_timestamp: u64,
    pub file_size: u64,
}

pub type BucketFiles = Vec<FileInfo>;
