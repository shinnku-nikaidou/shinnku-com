use crate::application::files::queries::get_file_tree_query::GetFileTreeQuery;
use crate::domain::files::entities::tree_node::{NavigationResult, TreeNode};
use crate::dto::files::Inode;
use crate::error::AppError;

/// Handler for getting file tree nodes
#[derive(Default)]
pub struct GetFileTreeHandler;

impl GetFileTreeHandler {
    pub fn new() -> Self {
        Self
    }

    /// Execute the get file tree query
    ///
    /// # Errors
    ///
    /// Returns an error if the requested path is not found in the tree
    pub fn handle(&self, query: &GetFileTreeQuery, tree: &TreeNode) -> Result<Inode, AppError> {
        match tree.navigate_path(&query.path) {
            NavigationResult::File { name, info } => Ok(Inode::File { name, info }),
            NavigationResult::Folder(folder) => {
                let data = folder.to_node_list();
                Ok(Inode::Folder { data })
            }
            NavigationResult::NotFound => Err(AppError::NotFound(format!(
                "path '{}' not found",
                query.path
            ))),
        }
    }
}
