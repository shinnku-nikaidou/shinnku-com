use crate::models::{BucketFiles, FileInfo};
use fuse_lib::{config::Fuse, fuseable::Fuseable, types::FuseProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchItem {
    pub id: String,
    pub info: FileInfo,
}

impl Fuseable for SearchItem {
    fn properties(&self) -> Vec<FuseProperty> {
        vec![FuseProperty::init("id")]
    }

    fn lookup(&self, key: &str) -> Option<&str> {
        if key == "id" { Some(&self.id) } else { None }
    }
}

pub type SearchList = Vec<SearchItem>;

pub fn trim_file_path(file_path: &str) -> String {
    const PREFIX: &str = "合集系列/";
    file_path
        .strip_prefix(PREFIX)
        .unwrap_or(file_path)
        .to_string()
}

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

    items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    items.into_iter().take(n).map(|(item, _)| item).collect()
}
