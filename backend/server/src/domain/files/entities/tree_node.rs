use crate::domain::files::entities::file_info::FileInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeType {
    File(FileInfo),
    Node(TreeNode),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeNode(HashMap<String, NodeType>);

/// Result of navigating through a tree structure
pub enum NavigationResult<'a> {
    Folder(&'a TreeNode),
    File { name: String, info: FileInfo },
    NotFound,
}

impl TreeNode {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: NodeType) -> Option<NodeType> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&NodeType> {
        self.0.get(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, NodeType> {
        self.0.iter()
    }

    pub fn entry(
        &mut self,
        key: String,
    ) -> std::collections::hash_map::Entry<'_, String, NodeType> {
        self.0.entry(key)
    }

    pub fn to_node_list(&self) -> Vec<crate::dto::files::Node> {
        self.iter()
            .map(|(name, value)| match value {
                NodeType::File(info) => crate::dto::files::Node::File {
                    name: name.clone(),
                    info: info.clone(),
                },
                NodeType::Node(_) => crate::dto::files::Node::Folder { name: name.clone() },
            })
            .collect()
    }

    pub fn navigate<'a>(&'a self, path_segments: &[String]) -> NavigationResult<'a> {
        let mut current = self;

        for (idx, segment) in path_segments.iter().enumerate() {
            match current.get(segment) {
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
