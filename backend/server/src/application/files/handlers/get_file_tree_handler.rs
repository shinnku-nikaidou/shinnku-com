use crate::application::files::queries::get_file_tree_query::GetFileTreeQuery;
use crate::domain::files::entities::tree_node::{NavigationResult, TreeNode};
use crate::error::AppError;

/// Handler for getting file tree nodes
#[derive(Default)]
pub struct GetFileTreeHandler;

impl GetFileTreeHandler {
    pub fn new() -> Self {
        Self
    }

    /// Execute the get file tree query, returning domain navigation result
    ///
    /// # Errors
    ///
    /// Returns an error if the requested path is not found in the tree
    pub fn handle<'a>(
        &self,
        query: &GetFileTreeQuery,
        tree: &'a TreeNode,
    ) -> Result<NavigationResult<'a>, AppError> {
        match tree.navigate_path(&query.path) {
            NavigationResult::NotFound => Err(AppError::NotFound(format!(
                "path '{}' not found",
                query.path
            ))),
            result => Ok(result),
        }
    }
}
