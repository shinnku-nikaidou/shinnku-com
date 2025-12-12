use crate::domain::files::entities::file_info::FileInfoRef;
use serde::{Deserialize, Serialize};

/// Search item for indexing and searching
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchItem {
    pub id: String,
    pub info: FileInfoRef,
}

pub type SearchList = Vec<SearchItem>;
