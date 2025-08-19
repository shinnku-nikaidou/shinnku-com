use crate::application::search::queries::search_files_query::SearchFilesQuery;
use crate::domain::search::entities::search_item::SearchList;
use crate::domain::search::repositories::fuzzy_search_repository::FuzzySearchRepository;

/// Handler for file search operations
pub struct SearchFilesHandler<R: FuzzySearchRepository> {
    repository: R,
}

impl<R: FuzzySearchRepository> SearchFilesHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Execute the search files query
    ///
    /// # Errors
    ///
    /// Returns an error if the search operation fails
    pub fn handle(&self, query: &SearchFilesQuery, search_index: &SearchList) -> SearchList {
        let results = self.repository.search(&query.query, search_index);

        if let Some(limit) = query.limit {
            results.into_iter().take(limit).collect()
        } else {
            results
        }
    }
}
