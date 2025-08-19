use crate::domain::files::entities::file_info::BucketFiles;
use crate::models::search::{SearchItem, SearchList};

/// Trim the common prefix from file paths for search indexing
pub fn trim_file_path(file_path: &str) -> String {
    const PREFIX: &str = "合集系列/";
    file_path
        .strip_prefix(PREFIX)
        .unwrap_or(file_path)
        .to_string()
}

/// Build search index from bucket files
pub fn aggregate_builder(buckets: &[BucketFiles]) -> SearchList {
    let mut res = Vec::new();
    for bucket in buckets {
        for item in bucket {
            res.push(SearchItem {
                id: trim_file_path(&item.file_path),
                info: item.clone(),
            });
        }
    }
    res
}
