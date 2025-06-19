use crate::config::{BucketFiles, FileInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchItem {
    pub id: String,
    pub info: FileInfo,
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
}
