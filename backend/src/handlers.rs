use crate::config::{FileInfo, NodeValue, TreeNode};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}

async fn proxy(path: &str, name: Option<String>) -> impl IntoResponse {
    let client = Client::new();
    let url = format!("http://127.0.0.1:2998{}", path);
    let req = if let Some(ref n) = name {
        client.get(&url).query(&[("name", n)])
    } else {
        client.get(&url)
    };
    match req.send().await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            match resp.bytes().await {
                Ok(body) => (status, body).into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => (StatusCode::BAD_GATEWAY, "Failed to proxy request").into_response(),
    }
}

pub async fn intro(Query(params): Query<NameQuery>) -> impl IntoResponse {
    proxy("/intro", params.name).await
}

pub async fn find_name(Query(params): Query<NameQuery>) -> impl IntoResponse {
    proxy("/findname", params.name).await
}

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

pub async fn inode(Path(path): Path<String>) -> Response {
    let tree = crate::alg::root::get_tree().await;
    let mut current = &tree.clone();
    println!("Path: {}", path);

    if !path.is_empty() {
        for segment in path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| percent_encoding::percent_decode_str(s).decode_utf8_lossy())
        {
            match current.get(segment.as_ref()) {
                Some(NodeValue::Node(node)) => {
                    current = node;
                }
                _ => return StatusCode::NOT_FOUND.into_response(),
            }
        }
    }

    let inode = node2list(current);
    (StatusCode::OK, Json(inode)).into_response()
}
