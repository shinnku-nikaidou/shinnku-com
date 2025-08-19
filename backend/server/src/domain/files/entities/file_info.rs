use serde::{Deserialize, Serialize};

/// Domain entity for file information
/// Note: Serialization is kept for practical reasons but domain logic should not depend on it
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_path: String,
    pub upload_timestamp: u64,
    pub file_size: u64,
}

pub type BucketFiles = Vec<FileInfo>;
