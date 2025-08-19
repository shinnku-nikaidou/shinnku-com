use crate::application::search::queries::search_files_query::SearchFilesQuery;
use crate::domain::search::entities::search_item::SearchList;
use crate::domain::search::services::fuzzy_search_service::runsearch;

/// Handler for file search operations
#[derive(Default)]
pub struct SearchFilesHandler;

impl SearchFilesHandler {
    pub fn new() -> Self {
        Self
    }

    /// Execute the search files query
    ///
    /// # Errors
    ///
    /// Returns an error if the search operation fails
    pub fn handle(&self, query: &SearchFilesQuery, search_index: &SearchList) -> SearchList {
        let results = runsearch(&query.query, search_index);

        if let Some(limit) = query.limit {
            results.into_iter().take(limit).collect()
        } else {
            results
        }
    }
}
