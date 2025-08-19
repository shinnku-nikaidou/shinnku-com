use crate::domain::files::entities::file_info::FileInfo;
use serde::{Deserialize, Serialize};

/// Search item for indexing and searching
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchItem {
    pub id: String,
    pub info: FileInfo,
}

impl fuse_lib::fuseable::Fuseable for SearchItem {
    fn properties(&self) -> Vec<fuse_lib::types::FuseProperty> {
        vec![fuse_lib::types::FuseProperty::init("id")]
    }

    fn lookup(&self, key: &str) -> Option<&str> {
        if key == "id" { Some(&self.id) } else { None }
    }
}

pub type SearchList = Vec<SearchItem>;
