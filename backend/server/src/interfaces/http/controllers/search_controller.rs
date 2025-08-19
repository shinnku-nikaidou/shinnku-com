use crate::application::search::handlers::combined_search_handler::CombinedSearchHandler;
use crate::application::search::handlers::search_files_handler::SearchFilesHandler;
use crate::application::search::queries::combined_search_query::CombinedSearchQuery;
use crate::application::search::queries::search_files_query::SearchFilesQuery;
use crate::dto::search::{CombineSearchQuery, SearchQuery};
use crate::error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokio::task::spawn_blocking;

/// Search for files using a single query string.
///
/// # Errors
///
/// Returns an error if:
/// - The query parameter `q` is missing
/// - Task spawning fails
/// - Search execution fails
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let q = params
        .q
        .ok_or_else(|| AppError::BadRequest("missing `q` query param".into()))?;

    let search_index = state.root.search_index.clone();
    let limit = params.n;
    let query = SearchFilesQuery::new(q, limit);
    let handler = SearchFilesHandler::new();

    let results = spawn_blocking(move || handler.handle(&query, &search_index))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(results)).into_response())
}

/// Search for files using two combined query strings.
///
/// # Errors
///
/// Returns an error if:
/// - Either query parameter `q1` or `q2` is missing
/// - Task spawning fails
/// - Combined search execution fails
pub async fn search_combined(
    State(state): State<AppState>,
    Query(params): Query<CombineSearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let (q1, q2) = match (params.q1, params.q2) {
        (Some(q1), Some(q2)) => (q1, q2),
        _ => {
            return Err(AppError::BadRequest(
                "missing `q1` and/or `q2` query param".into(),
            ));
        }
    };

    let search_index = state.root.search_index.clone();
    let limit = params.n.unwrap_or(100);
    let query = CombinedSearchQuery::new(q1, q2, limit);
    let handler = CombinedSearchHandler::new();

    let results = spawn_blocking(move || handler.handle(&query, &search_index))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(results)).into_response())
}
