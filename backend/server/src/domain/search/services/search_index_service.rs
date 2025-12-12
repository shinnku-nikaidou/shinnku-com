use crate::domain::files::entities::file_info::FileInfo;
use crate::domain::search::entities::search_item::{SearchItem, SearchList};
use crate::domain::search::value_objects::search_path::SearchPath;

/// Domain service for building search indexes
///
/// This service contains pure business logic for converting file information
/// into searchable items, without external dependencies.
#[derive(Default)]
pub struct SearchIndexService;

impl SearchIndexService {
    pub fn new() -> Self {
        Self
    }

    /// Build search index from multiple bucket files
    ///
    /// This is a domain service operation that applies business rules
    /// for creating search items from file information.
    pub fn build_index(&self, buckets: &[Vec<FileInfo>]) -> SearchList {
        let mut search_list = Vec::new();

        for bucket in buckets {
            for file_info in bucket {
                let search_path = SearchPath::new(&file_info.file_path);
                search_list.push(SearchItem {
                    id: search_path.to_string(),
                    info: file_info.clone(),
                });
            }
        }

        search_list
    }
}
