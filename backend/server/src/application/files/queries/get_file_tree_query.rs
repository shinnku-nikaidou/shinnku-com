use serde::{Deserialize, Serialize};

/// Query for getting a file or directory node by path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileTreeQuery {
    /// File path to navigate to. Empty string means root.
    pub path: String,
}

impl GetFileTreeQuery {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn root() -> Self {
        Self {
            path: String::new(),
        }
    }
}
