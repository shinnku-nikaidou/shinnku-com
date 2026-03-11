use std::sync::Arc;

use crate::domain::files::entities::file_info::FileInfo as DomainFileInfo;
use serde::{Deserialize, Serialize};

/// Serializable version of FileInfo for infrastructure layer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SerializableFileInfo {
    pub file_path: String,
    pub upload_timestamp: u64,
    pub file_size: u64,
}

impl From<SerializableFileInfo> for DomainFileInfo {
    fn from(serializable: SerializableFileInfo) -> Self {
        Self {
            file_path: Arc::from(serializable.file_path),
            upload_timestamp: serializable.upload_timestamp,
            file_size: serializable.file_size,
        }
    }
}

impl From<DomainFileInfo> for SerializableFileInfo {
    fn from(domain: DomainFileInfo) -> Self {
        Self {
            file_path: domain.file_path.to_string(),
            upload_timestamp: domain.upload_timestamp,
            file_size: domain.file_size,
        }
    }
}
