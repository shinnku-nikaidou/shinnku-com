use crate::models::search::{SearchItem, SearchList};
use fuse_lib::config::Fuse;

/// Perform fuzzy search on a list of files
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

/// Perform combined search with two queries
pub fn combine_search(q1: &str, q2: &str, n: usize, files: &SearchList) -> SearchList {
    use std::collections::HashMap;

    let fuse = Fuse {
        threshold: 0.6,
        distance: 800,
        max_pattern_length: 32,
        ..Default::default()
    };

    let q1_res = fuse.search_text_in_fuse_list(q1, files.as_slice());
    let q2_res = fuse.search_text_in_fuse_list(q2, files.as_slice());

    let mut scores: HashMap<usize, f64> = HashMap::new();

    for res in q1_res {
        scores.insert(res.index, res.score);
    }

    for res in q2_res {
        scores
            .entry(res.index)
            .and_modify(|s| *s = (*s + res.score) / 2.0)
            .or_insert(res.score);
    }

    let mut items: Vec<(SearchItem, f64)> = scores
        .into_iter()
        .map(|(idx, score)| (files[idx].clone(), score))
        .collect();

    items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    items.into_iter().take(n).map(|(item, _)| item).collect()
}
