use crate::alg::{
    root,
    search::{combine_search, runsearch},
};
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

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub n: Option<usize>,
}

#[derive(Deserialize)]
pub struct CombineSearchQuery {
    pub q1: Option<String>,
    pub q2: Option<String>,
    pub n: Option<usize>,
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
        .map(|s| {
            percent_encoding::percent_decode_str(s)
                .decode_utf8_lossy()
                .to_string()
        })
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

pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    let q = match params.q {
        Some(q) => q,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let search_index = &root::get_root().await.search_index;
    let n = params.n.unwrap_or(100);
    let results = runsearch(&q, search_index);
    let sliced: Vec<_> = results.into_iter().take(n).collect();
    (StatusCode::OK, Json(sliced)).into_response()
}

pub async fn conbine_search(Query(params): Query<CombineSearchQuery>) -> impl IntoResponse {
    let (q1, q2) = match (params.q1, params.q2) {
        (Some(q1), Some(q2)) => (q1, q2),
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    let search_index = &root::get_root().await.search_index;
    let n = params.n.unwrap_or(100);
    let results = combine_search(&q1, &q2, n, search_index);
    (StatusCode::OK, Json(results)).into_response()
}
