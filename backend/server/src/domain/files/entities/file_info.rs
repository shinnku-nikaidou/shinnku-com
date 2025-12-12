use std::sync::Arc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Domain entity for file information
/// Note: Serialization is kept for practical reasons but domain logic should not depend on it
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    #[serde(
        serialize_with = "serialize_arc_str",
        deserialize_with = "deserialize_arc_str"
    )]
    pub file_path: Arc<str>,
    pub upload_timestamp: u64,
    pub file_size: u64,
}

fn serialize_arc_str<S>(value: &Arc<str>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value)
}

fn deserialize_arc_str<'de, D>(deserializer: D) -> Result<Arc<str>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Arc::from(s))
}
