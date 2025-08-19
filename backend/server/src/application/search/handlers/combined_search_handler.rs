use crate::application::search::queries::combined_search_query::CombinedSearchQuery;
use crate::domain::search::entities::search_item::SearchList;
use crate::domain::search::repositories::fuzzy_search_repository::FuzzySearchRepository;

/// Handler for combined search operations
pub struct CombinedSearchHandler<R: FuzzySearchRepository> {
    repository: R,
}

impl<R: FuzzySearchRepository> CombinedSearchHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Execute the combined search query
    pub fn handle(&self, query: &CombinedSearchQuery, search_index: &SearchList) -> SearchList {
        self.repository
            .combined_search(&query.query1, &query.query2, query.limit, search_index)
    }
}
