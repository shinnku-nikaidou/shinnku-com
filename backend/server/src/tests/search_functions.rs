use crate::application::search::services::search_index_service::SearchIndexService;
use crate::domain::files::entities::file_info::FileInfo;
use crate::domain::search::entities::search_item::SearchItem;
use crate::domain::search::services::combined_search_service::combine_search;
use crate::domain::search::services::fuzzy_search_service::runsearch;

#[test]
fn test_search_index_builder() {
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
    let service = SearchIndexService::new();
    let list = service.build_index(&[b1.clone(), b2.clone()]);
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

#[test]
fn test_char_boundary_panic() {
    // Test to reproduce the exact panic from the error message:
    // "byte index 63 is not a char boundary; it is inside '！' (bytes 62..65)"
    let files = vec![SearchItem {
        id: "zd/1001-1500/[181026][hulotte] 出会って5分は俺のもの！時間停止と不可避な運命.rar".into(),
        info: FileInfo {
            file_path: "合集系列/zd/1001-1500/[181026][hulotte] 出会って5分は俺のもの！時間停止と不可避な運命.rar".into(),
            upload_timestamp: 0,
            file_size: 1,
        },
    }];

    // This is the URL-decoded query from the error:
    // %E5%87%BA%E4%BC%9A%E3%81%A3%E3%81%A65%E5%88%86%E3%81%AF%E4%BF%BA%E3%81%AE%E3%82%82%E3%81%AE%EF%BC%81%E6%99%82%E9%96%93%E5%81%9C%E6%AD%A2%E3%81%A8%E4%B8%8D%E5%8F%AF%E9%81%BF%E3%81%AA%E9%81%8B%E5%91%BD
    let problematic_query = "出会って5分は俺のもの！時間停止と不可避な運命";

    // This should reproduce the panic about char boundary at byte index 63
    let res = runsearch(problematic_query, &files);

    // If we get here without panicking, the bug is fixed
    println!("Search completed successfully. Result count: {}", res.len());
}
