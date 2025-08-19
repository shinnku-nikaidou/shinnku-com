use crate::application::search::services::search_index_service::SearchIndexService;
use crate::domain::files::entities::tree_node::TreeNode;
use crate::domain::files::factories::tree_factory::TreeFactory;
use crate::domain::files::repositories::file_repository::BucketFilesRepository as BucketFilesRepositoryTrait;
use crate::domain::search::entities::search_item::SearchList;
use crate::infrastructure::persistence::json::bucket_files_repository::JsonBucketFilesRepository;
use anyhow::Result;
use tokio::task::spawn_blocking;

/// Application state data structure
#[derive(Clone)]
pub struct ApplicationData {
    pub shinnku_tree: TreeNode,
    pub galgame0_tree: TreeNode,
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
            let repo = JsonBucketFilesRepository::new();

            let shinnku_bucket_files = repo.load_shinnku_files()?;
            let galgame0_bucket_files = repo.load_galgame0_files()?;

            let shinnku_tree = TreeFactory::from_file_list(&shinnku_bucket_files);
            let galgame0_tree = TreeFactory::from_file_list(&galgame0_bucket_files);

            let galgame0_filtered = repo
                .filter_galgame0_files(&galgame0_bucket_files, "合集系列/浮士德galgame游戏合集");

            let search_index_service = SearchIndexService::new();
            let search_index =
                search_index_service.build_index(&[shinnku_bucket_files, galgame0_filtered]);

            Ok(ApplicationData {
                shinnku_tree,
                galgame0_tree,
                search_index,
            })
        })
        .await?
    }
}
