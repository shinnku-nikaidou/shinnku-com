use crate::domain::search::entities::search_item::SearchList;
use fuse_lib::config::Fuse;

/// Perform fuzzy search on a list of files using Fuse.js algorithm
pub fn runsearch(query: &str, files: &SearchList) -> SearchList {
    let fuse = Fuse {
        threshold: 0.6,
        distance: 800,
        max_pattern_length: 32,
        ..Default::default()
    };
    fuse.search_text_in_fuse_list(query, files.as_slice())
        .into_iter()
        .map(|r| files[r.index].clone())
        .collect()
}
