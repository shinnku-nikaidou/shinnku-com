use crate::application::files::services::file_tree_service::FileTreeService;
use crate::domain::files::entities::tree_node::TreeNode;
use crate::domain::files::factories::tree_factory::TreeFactory;
use crate::domain::search::entities::search_item::SearchList;
use crate::domain::search::services::search_index_service::SearchIndexService;
use crate::infrastructure::persistence::json::bucket_files_repository::{
    GALGAME0_FILES, SHINNKU_FILES, filter_galgame0_files,
};
use anyhow::Result;
use tokio::task::spawn_blocking;

/// Application state data structure
#[derive(Clone)]
pub struct ApplicationData {
    pub combined_tree: TreeNode,
    pub search_index: SearchList,
}

/// Application bootstrap service for initializing application state
#[derive(Default)]
pub struct ApplicationBootstrapService;

impl ApplicationBootstrapService {
    pub fn new() -> Self {
        Self
    }

    /// Initialize application data by loading and processing all required resources
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - JSON parsing fails for bucket files
    /// - Task spawning fails
    pub async fn initialize(&self) -> Result<ApplicationData> {
        spawn_blocking(|| {
            let shinnku_bucket_files = &*SHINNKU_FILES;
            let galgame0_bucket_files = &*GALGAME0_FILES;

            let shinnku_tree = TreeFactory::from_file_list(&shinnku_bucket_files);
            let galgame0_tree = TreeFactory::from_file_list(&galgame0_bucket_files);

            let galgame0_filtered =
                filter_galgame0_files(&galgame0_bucket_files, "合集系列/浮士德galgame游戏合集");

            let search_index_service = SearchIndexService::new();
            let search_index = search_index_service
                .build_index(&[shinnku_bucket_files.clone(), galgame0_filtered]);

            let combined_tree =
                FileTreeService::build_combined_frontend_tree(&shinnku_tree, &galgame0_tree);

            Ok(ApplicationData {
                combined_tree,
                search_index,
            })
        })
        .await?
    }
}
