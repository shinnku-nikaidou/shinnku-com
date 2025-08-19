use serde::{Deserialize, Serialize};

/// Query for getting wiki picture background by name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiContentQuery {
    /// Name of the wiki page to get background for
    pub name: String,
}

impl GetWikiContentQuery {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
