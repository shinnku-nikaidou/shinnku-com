use serde::{Deserialize, Serialize};

/// Query for searching files using fuzzy search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilesQuery {
    /// Search query string
    pub query: String,
    /// Maximum number of results to return
    pub limit: Option<usize>,
}

impl SearchFilesQuery {
    pub fn new(query: String, limit: Option<usize>) -> Self {
        Self { query, limit }
    }
}
