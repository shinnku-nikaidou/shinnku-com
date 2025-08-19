use shinnku_com_backend::config::startup::load_root;
use shinnku_com_backend::domain::search::services::fuzzy_search_service::runsearch;

#[tokio::test]
async fn test_search() {
    let q = "サノバウィッチ";
    let root = load_root().await.unwrap();
    let search_index = &root.search_index;
    let n = 20;
    let results = runsearch(q, search_index);
    let sliced: Vec<_> = results.into_iter().take(n).collect();
    tracing::info!("Search results for '{q}': {sliced:?}");
}
