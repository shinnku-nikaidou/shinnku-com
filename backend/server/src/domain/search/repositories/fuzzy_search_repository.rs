use crate::domain::search::entities::search_item::SearchList;

/// Repository trait for performing fuzzy search operations on search items.
///
/// This trait abstracts the fuzzy search functionality, allowing different
/// implementations (like Fuse, Elasticsearch, etc.) to be used without
/// affecting the domain logic.
pub trait FuzzySearchRepository {
    /// Performs a single-query fuzzy search on the provided search items.
    ///
    /// # Arguments
    /// * `query` - The search query string
    /// * `items` - The collection of items to search through
    ///
    /// # Returns
    /// A filtered and sorted collection of search items that match the query
    fn search(&self, query: &str, items: &SearchList) -> SearchList;

    /// Performs a combined fuzzy search using two queries.
    ///
    /// This method searches for items that match either query and combines
    /// the results with appropriate scoring.
    ///
    /// # Arguments  
    /// * `q1` - The first search query
    /// * `q2` - The second search query
    /// * `limit` - Maximum number of results to return
    /// * `items` - The collection of items to search through
    ///
    /// # Returns
    /// A filtered, scored, and limited collection of search items
    fn combined_search(&self, q1: &str, q2: &str, limit: usize, items: &SearchList) -> SearchList;
}
