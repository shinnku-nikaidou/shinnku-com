use crate::domain::wiki::services::wiki_service::get_wiki_background;
use crate::error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use redis::aio::ConnectionManager;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WikiPictureQuery {
    pub name: Option<String>,
}

#[derive(serde::Serialize)]
pub struct WikiPictureResponse {
    pub bg: Option<String>,
}

/// Search for a Wikipedia picture background by name.
///
/// # Errors
///
/// Returns an error if:
/// - The `name` query parameter is missing
/// - Redis connection fails
/// - Wiki service call fails
pub async fn wiki_search_picture(
    State(state): State<AppState>,
    Query(params): Query<WikiPictureQuery>,
) -> Result<impl IntoResponse, AppError> {
    let name = params
        .name
        .ok_or_else(|| AppError::BadRequest("missing `name` query param".into()))?;

    let mut con: ConnectionManager = state.redis.clone();

    let bg: Option<String> = get_wiki_background(&mut con, &name)
        .await
        .unwrap_or_default();

    Ok((StatusCode::OK, Json(WikiPictureResponse { bg })).into_response())
}
