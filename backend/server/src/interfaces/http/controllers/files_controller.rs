use crate::domain::files::entities::tree_node::{NavigationResult, TreeNode};
use crate::dto::files::Inode;
use crate::error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Synchronous helpers used by the router closures

/// Get a file or directory node by path.
///
/// # Errors
///
/// Returns an error if the path is not found in the tree.
pub async fn get_node(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    get_node_impl(&path, &state.tree)
}

/// Get the root directory node.
///
/// # Errors
///
/// Returns an error if the root path cannot be accessed.
pub async fn get_node_root(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    get_node_impl("", &state.tree)
}

/// Implementation helper for getting nodes from the tree.
///
/// # Errors
///
/// Returns an error if:
/// - The path is not found in the tree
/// - Path navigation fails
pub fn get_node_impl(path: &str, tree: &TreeNode) -> Result<Response, AppError> {
    match tree.navigate_path(path) {
        NavigationResult::File { name, info } => {
            let resp = Inode::File { name, info };
            Ok((StatusCode::OK, Json(resp)).into_response())
        }
        NavigationResult::Folder(folder) => {
            let data = folder.to_node_list();
            let resp = Inode::Folder { data };
            Ok((StatusCode::OK, Json(resp)).into_response())
        }
        NavigationResult::NotFound => Err(AppError::NotFound(format!("path '{path}' not found"))),
    }
}
