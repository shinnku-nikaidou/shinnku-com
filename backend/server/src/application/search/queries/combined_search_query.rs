use serde::{Deserialize, Serialize};

/// Query for searching files using two combined query strings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedSearchQuery {
    /// First search query string
    pub query1: String,
    /// Second search query string
    pub query2: String,
    /// Maximum number of results to return
    pub limit: usize,
}

impl CombinedSearchQuery {
    pub fn new(query1: String, query2: String, limit: usize) -> Self {
        Self {
            query1,
            query2,
            limit,
        }
    }
}
