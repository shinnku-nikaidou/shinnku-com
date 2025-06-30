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

pub fn node2list(node: &TreeNode) -> Vec<Node> {
    node.iter()
        .map(|(name, value)| match value {
            NodeType::File(info) => Node::File {
                name: name.clone(),
                info: info.clone(),
            },
            NodeType::Node(_) => Node::Folder { name: name.clone() },
        })
        .collect()
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Inode {
    #[serde(rename = "folder")]
    Folder { data: Vec<Node> },
    #[serde(rename = "file")]
    File { name: String, info: FileInfo },
}
