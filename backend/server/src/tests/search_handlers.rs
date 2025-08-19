use crate::{
    application::shared::services::application_bootstrap_service::ApplicationBootstrapService,
    domain::search::repositories::fuzzy_search_repository::FuzzySearchRepository,
    infrastructure::adapters::search::fuse_search_adapter::FuseSearchAdapter,
};

#[tokio::test]
async fn test_search() {
    let q = "サノバウィッチ";
    let bootstrap_service = ApplicationBootstrapService::new();
    let root = bootstrap_service.initialize().await.unwrap();
    let search_index = &root.search_index;
    let n = 20;

    let adapter = FuseSearchAdapter::with_default_config();
    let results = adapter.search(q, search_index);
    let sliced: Vec<_> = results.into_iter().take(n).collect();
    tracing::info!("Search results for '{q}': {sliced:?}");
}
