use crate::domain::files::entities::file_info::FileInfoRef;
use crate::domain::files::entities::tree_node::{NodeType, TreeNode};

/// Factory for creating and combining file trees
pub struct TreeFactory;

impl TreeFactory {
    /// Generate a hierarchical tree from a flat list of files
    pub fn from_file_list(file_list: &[FileInfoRef]) -> TreeNode {
        let mut root = TreeNode::new();

        for file in file_list {
            let path_parts: Vec<&str> = file.file_path.split('/').collect();
            let mut pointer = &mut root;
            let last_idx = path_parts.len() - 1;

            for part in &path_parts[..last_idx] {
                pointer = match pointer
                    .entry(part.to_string())
                    .or_insert_with(|| NodeType::Node(TreeNode::new()))
                {
                    NodeType::Node(node) => node,
                    NodeType::File(_) => {
                        tracing::error!("Expected folder but found file at path: {}", part);
                        return TreeNode::new(); // Return empty tree on error
                    }
                };
            }

            pointer.insert(
                path_parts[last_idx].to_string(),
                NodeType::File(file.clone()),
            );
        }

        root
    }
}
