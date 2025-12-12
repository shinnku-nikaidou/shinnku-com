use crate::domain::files::entities::file_info::FileInfoRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeType {
    File(FileInfoRef),
    Node(TreeNode),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeNode(HashMap<String, NodeType>);

impl AsRef<HashMap<String, NodeType>> for TreeNode {
    fn as_ref(&self) -> &HashMap<String, NodeType> {
        &self.0
    }
}

impl AsMut<HashMap<String, NodeType>> for TreeNode {
    fn as_mut(&mut self) -> &mut HashMap<String, NodeType> {
        &mut self.0
    }
}

/// Result of navigating through a tree structure
pub enum NavigationResult<'a> {
    Folder(&'a TreeNode),
    File { name: String, info: FileInfoRef },
    NotFound,
}

impl TreeNode {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn navigate<'a>(&'a self, path_segments: &[String]) -> NavigationResult<'a> {
        let mut current = self;

        for (idx, segment) in path_segments.iter().enumerate() {
            match current.as_ref().get(segment) {
                Some(NodeType::Node(node)) => {
                    current = node;
                }
                Some(NodeType::File(info)) => {
                    if idx == path_segments.len() - 1 {
                        return NavigationResult::File {
                            name: segment.clone(),
                            info: info.clone(),
                        };
                    } else {
                        return NavigationResult::NotFound;
                    }
                }
                None => return NavigationResult::NotFound,
            }
        }

        NavigationResult::Folder(current)
    }

    /// Navigate to a node by path string with URL decoding support
    pub fn navigate_path<'a>(&'a self, path: &str) -> NavigationResult<'a> {
        use percent_encoding::percent_decode_str;

        let segments: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| percent_decode_str(s).decode_utf8_lossy().to_string())
            .collect();

        self.navigate(&segments)
    }
}

impl From<&[FileInfoRef]> for TreeNode {
    fn from(file_list: &[FileInfoRef]) -> Self {
        let mut root = TreeNode::new();

        for file in file_list {
            let path_parts: Vec<&str> = file.file_path.split('/').collect();
            let mut pointer = &mut root;
            let last_idx = path_parts.len() - 1;

            for part in &path_parts[..last_idx] {
                pointer = match pointer
                    .as_mut()
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

            pointer.as_mut().insert(
                path_parts[last_idx].to_string(),
                NodeType::File(file.clone()),
            );
        }

        root
    }
}
