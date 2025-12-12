use crate::domain::files::entities::tree_node::{NavigationResult, NodeType, TreeNode};
use crate::interfaces::http::dto::files_dto::{Inode, Node};

/// Mapper for converting between domain objects and DTOs
pub struct FileTreeMapper;

impl FileTreeMapper {
    /// Convert TreeNode to Node list for API response
    pub fn tree_to_node_list(tree: &TreeNode) -> Vec<Node> {
        tree.as_ref()
            .iter()
            .map(|(name, value)| match value {
                NodeType::File(info) => Node::File {
                    name: name.clone(),
                    info: info.clone(),
                },
                NodeType::Node(_) => Node::Folder { name: name.clone() },
            })
            .collect()
    }

    /// Convert NavigationResult to Inode DTO
    pub fn navigation_result_to_dto(result: NavigationResult) -> Option<Inode> {
        match result {
            NavigationResult::File { name, info } => Some(Inode::File { name, info }),
            NavigationResult::Folder(folder) => {
                let data = Self::tree_to_node_list(folder);
                Some(Inode::Folder { data })
            }
            NavigationResult::NotFound => None,
        }
    }
}
