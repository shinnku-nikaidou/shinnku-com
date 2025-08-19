use crate::application::search::queries::combined_search_query::CombinedSearchQuery;
use crate::domain::search::entities::search_item::SearchList;
use crate::domain::search::services::combined_search_service::combine_search;

/// Handler for combined search operations
#[derive(Default)]
pub struct CombinedSearchHandler;

impl CombinedSearchHandler {
    pub fn new() -> Self {
        Self
    }

    /// Execute the combined search query
    pub fn handle(&self, query: &CombinedSearchQuery, search_index: &SearchList) -> SearchList {
        combine_search(&query.query1, &query.query2, query.limit, search_index)
    }
}
