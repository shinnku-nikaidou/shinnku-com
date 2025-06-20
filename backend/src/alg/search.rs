use crate::config::{BucketFiles, FileInfo};
use crate::fuse::lib::{Fuse, FuseProperty, Fuseable};
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
    const PREFIX: &str = "合集系列/"; // "合集系列/"
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
    let mut fuse = Fuse::default();
    fuse.threshold = 0.78;
    fuse.search_text_in_fuse_list(query, files.as_slice())
        .into_iter()
        .map(|r| files[r.index].clone())
        .collect()
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
}
