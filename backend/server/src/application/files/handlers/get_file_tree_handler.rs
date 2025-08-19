use crate::application::files::queries::get_file_tree_query::GetFileTreeQuery;
use crate::domain::files::entities::tree_node::{NavigationResult, NodeType, TreeNode};
use crate::error::AppError;
use crate::interfaces::http::dto::files_dto::{Inode, Node};

/// Handler for getting file tree nodes
#[derive(Default)]
pub struct GetFileTreeHandler;

impl GetFileTreeHandler {
    pub fn new() -> Self {
        Self
    }

    /// Convert TreeNode to Node list (replacing the removed Domain method)
    fn tree_to_node_list(&self, tree: &TreeNode) -> Vec<Node> {
        tree.iter()
            .map(|(name, value)| match value {
                NodeType::File(info) => Node::File {
                    name: name.clone(),
                    info: info.clone(),
                },
                NodeType::Node(_) => Node::Folder { name: name.clone() },
            })
            .collect()
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
                let data = self.tree_to_node_list(folder);
                Ok(Inode::Folder { data })
            }
            NavigationResult::NotFound => Err(AppError::NotFound(format!(
                "path '{}' not found",
                query.path
            ))),
        }
    }
}
