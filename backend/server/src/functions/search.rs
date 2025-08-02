use crate::models::{BucketFiles, FileInfo};
use fuse_lib::lib::{Fuse, FuseProperty, Fuseable};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_builder() {
        let b1 = vec![FileInfo {
            file_path: "合集系列/foo/bar.txt".into(),
            upload_timestamp: 0,
            file_size: 1,
        }];
        let b2 = vec![FileInfo {
            file_path: "other/baz.txt".into(),
            upload_timestamp: 1,
            file_size: 2,
        }];
        let list = aggregate_builder(&[b1.clone(), b2.clone()]);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].id, "foo/bar.txt");
        assert_eq!(list[1].id, "other/baz.txt");
        assert_eq!(list[0].info, b1[0]);
        assert_eq!(list[1].info, b2[0]);
    }

    #[test]
    fn test_runsearch() {
        let files = vec![
            SearchItem {
                id: "foo.txt".into(),
                info: FileInfo {
                    file_path: "foo.txt".into(),
                    upload_timestamp: 0,
                    file_size: 1,
                },
            },
            SearchItem {
                id: "bar.txt".into(),
                info: FileInfo {
                    file_path: "bar.txt".into(),
                    upload_timestamp: 0,
                    file_size: 1,
                },
            },
        ];
        let res = runsearch("foo", &files);
        assert!(!res.is_empty());
        assert_eq!(res[0].id, "foo.txt");
    }

    #[test]
    fn test_combine_search() {
        let files = vec![
            SearchItem {
                id: "foo.txt".into(),
                info: FileInfo {
                    file_path: "foo.txt".into(),
                    upload_timestamp: 0,
                    file_size: 1,
                },
            },
            SearchItem {
                id: "bar.txt".into(),
                info: FileInfo {
                    file_path: "bar.txt".into(),
                    upload_timestamp: 0,
                    file_size: 1,
                },
            },
        ];

        let res = combine_search("foo", "bar", 10, &files);
        assert_eq!(res.len(), 2);
        assert!(res.iter().any(|i| i.id == "foo.txt"));
        assert!(res.iter().any(|i| i.id == "bar.txt"));

        let res2 = combine_search("foo", "foo", 10, &files);
        assert_eq!(res2.len(), 1);
        assert_eq!(res2[0].id, "foo.txt");
    }

    #[test]
    fn test_long_search_query() {
        // Test with a long Japanese query that previously caused overflow
        let files = vec![SearchItem {
            id: "出会った5分は俺のもの！.txt".into(),
            info: FileInfo {
                file_path: "出会った5分は俺のもの！.txt".into(),
                upload_timestamp: 0,
                file_size: 1,
            },
        }];

        // Test with original problematic query - should not panic
        let long_query = "出会った5分は俺のもの！時間停止と不可避な運命";
        let res = runsearch(long_query, &files);
        // The main goal is that this doesn't panic due to shift overflow
        // The result might be empty due to pattern truncation, which is acceptable
        println!(
            "Search completed without panic. Result count: {}",
            res.len()
        );

        // Test with a shorter query that should match
        let short_query = "出会った";
        let res2 = runsearch(short_query, &files);
        assert!(!res2.is_empty(), "Short query should find matches");
    }
}
