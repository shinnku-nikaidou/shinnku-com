use crate::domain::files::entities::file_info::BucketFiles;
use crate::domain::search::entities::search_item::{SearchItem, SearchList};
use crate::domain::search::value_objects::search_path::SearchPath;

/// Application service for building search indexes
#[derive(Default)]
pub struct SearchIndexService;

impl SearchIndexService {
    pub fn new() -> Self {
        Self
    }

    /// Build search index from multiple bucket files
    pub fn build_index(&self, buckets: &[BucketFiles]) -> SearchList {
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
