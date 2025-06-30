use crate::error::AppError;
use crate::models::{
    NodeType, TreeNode,
    inode::{Inode, node2list},
};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use percent_encoding::percent_decode_str;

// Synchronous helpers used by the router closures
pub async fn get_node(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    Ok(get_node_impl(&path, &state.tree))
}

pub async fn get_node_root(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(get_node_impl("", &state.tree))
}

pub fn get_node_impl(path: &str, tree: &TreeNode) -> Response {
    let mut current = tree;
    let segments: Vec<String> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| percent_decode_str(s).decode_utf8_lossy().to_string())
        .collect();

    for (idx, segment) in segments.iter().enumerate() {
        match current.get(segment) {
            Some(NodeType::Node(node)) => {
                current = node;
            }
            Some(NodeType::File(info)) => {
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
