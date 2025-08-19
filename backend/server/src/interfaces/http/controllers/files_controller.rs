use crate::application::files::handlers::get_file_tree_handler::GetFileTreeHandler;
use crate::application::files::queries::get_file_tree_query::GetFileTreeQuery;
use crate::error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

/// Get a file or directory node by path.
///
/// # Errors
///
/// Returns an error if the path is not found in the tree.
pub async fn get_node(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let query = GetFileTreeQuery::new(path);
    let handler = GetFileTreeHandler::new();
    let result = handler.handle(&query, &state.tree)?;

    Ok((StatusCode::OK, Json(result)).into_response())
}

/// Get the root directory node.
///
/// # Errors
///
/// Returns an error if the root path cannot be accessed.
pub async fn get_node_root(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let query = GetFileTreeQuery::root();
    let handler = GetFileTreeHandler::new();
    let result = handler.handle(&query, &state.tree)?;

    Ok((StatusCode::OK, Json(result)).into_response())
}
