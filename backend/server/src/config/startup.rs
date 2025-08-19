use crate::config::search::aggregate_builder;
use crate::config::tree::generate_tree;
use crate::domain::files::entities::tree_node::TreeNode;
use crate::models::search::SearchList;
use crate::repositories::bucket_files::BucketFilesRepository;
use anyhow::Result;
use tokio::task::spawn_blocking;

#[derive(Clone)]
pub struct Root {
    pub shinnku_tree: TreeNode,
    pub galgame0_tree: TreeNode,
    pub search_index: SearchList,
}

/// Load bucket files and build trees and search index.
///
/// # Errors
///
/// Returns an error if:
/// - JSON parsing fails for bucket files
/// - Task spawning fails
pub async fn load_root() -> Result<Root> {
    spawn_blocking(|| {
        let repo = BucketFilesRepository::new();

        let shinnku_bucket_files = repo.load_shinnku_files()?;
        let galgame0_bucket_files = repo.load_galgame0_files()?;

        let shinnku_tree = generate_tree(&shinnku_bucket_files);
        let galgame0_tree = generate_tree(&galgame0_bucket_files);

        let galgame0_filtered =
            repo.filter_galgame0_files(&galgame0_bucket_files, "合集系列/浮士德galgame游戏合集");

        let search_index = aggregate_builder(&[shinnku_bucket_files, galgame0_filtered]);

        Ok(Root {
            shinnku_tree,
            galgame0_tree,
            search_index,
        })
    })
    .await?
}
