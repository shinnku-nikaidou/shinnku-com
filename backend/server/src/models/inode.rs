use super::{FileInfo, NodeType, TreeNode};
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "file")]
    File { name: String, info: FileInfo },
    #[serde(rename = "folder")]
    Folder { name: String },
}

pub enum NavigationResult<'a> {
    Folder(&'a TreeNode),
    File { name: String, info: FileInfo },
    NotFound,
}

pub trait TreeNodeExt {
    fn to_node_list(&self) -> Vec<Node>;
    fn navigate<'a>(&'a self, path_segments: &[String]) -> NavigationResult<'a>;
}

impl TreeNodeExt for TreeNode {
    fn to_node_list(&self) -> Vec<Node> {
        self.iter()
            .map(|(name, value)| match value {
                NodeType::File(info) => Node::File {
                    name: name.clone(),
                    info: info.clone(),
                },
                NodeType::Node(_) => Node::Folder { name: name.clone() },
            })
            .collect()
    }

    fn navigate<'a>(&'a self, path_segments: &[String]) -> NavigationResult<'a> {
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
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Inode {
    #[serde(rename = "folder")]
    Folder { data: Vec<Node> },
    #[serde(rename = "file")]
    File { name: String, info: FileInfo },
}
