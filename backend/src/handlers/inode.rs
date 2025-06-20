use crate::config::{FileInfo, NodeValue, TreeNode};
use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use percent_encoding::percent_decode_str;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "file")]
    File { name: String, info: FileInfo },
    #[serde(rename = "folder")]
    Folder { name: String },
}

fn node2list(node: &TreeNode) -> Vec<Node> {
    node.iter()
        .map(|(name, value)| match value {
            NodeValue::File(info) => Node::File {
                name: name.clone(),
                info: info.clone(),
            },
            NodeValue::Node(_) => Node::Folder { name: name.clone() },
        })
        .collect()
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Inode {
    #[serde(rename = "folder")]
    Folder { data: Vec<Node> },
    #[serde(rename = "file")]
    File { name: String, info: FileInfo },
}

pub async fn inode(Path(path): Path<String>) -> impl IntoResponse {
    inode_impl(path).await
}

pub async fn inode_root() -> impl IntoResponse {
    inode_impl(String::new()).await
}

pub async fn inode_impl(path: String) -> Response {
    let tree = crate::alg::root::get_tree().await;
    let mut current = tree;
    let segments: Vec<String> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| percent_decode_str(s).decode_utf8_lossy().to_string())
        .collect();

    for (idx, segment) in segments.iter().enumerate() {
        match current.get(segment) {
            Some(NodeValue::Node(node)) => {
                current = node;
            }
            Some(NodeValue::File(info)) => {
                if idx == segments.len() - 1 {
                    let resp = Inode::File {
                        name: segment.clone(),
                        info: info.clone(),
                    };
                    return (StatusCode::OK, Json(resp)).into_response();
                } else {
                    return StatusCode::NOT_FOUND.into_response();
                }
            }
            None => return StatusCode::NOT_FOUND.into_response(),
        }
    }

    let data = node2list(current);
    let resp = Inode::Folder { data };
    (StatusCode::OK, Json(resp)).into_response()
}
