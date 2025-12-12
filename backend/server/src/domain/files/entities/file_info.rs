use std::sync::Arc;

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

#[derive(Debug, Clone, PartialEq)]
pub struct FileInfoRef(Arc<FileInfo>);

impl FileInfoRef {
    pub fn new(file_info: FileInfo) -> Self {
        Self(Arc::new(file_info))
    }
}

impl From<FileInfo> for FileInfoRef {
    fn from(file_info: FileInfo) -> Self {
        Self::new(file_info)
    }
}

impl serde::Serialize for FileInfoRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for FileInfoRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let file_info = FileInfo::deserialize(deserializer)?;
        Ok(Self::new(file_info))
    }
}
