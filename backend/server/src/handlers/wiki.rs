use crate::services::wiki::get_wiki_background;
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

pub async fn wiki_search_picture(
    State(state): State<AppState>,
    Query(params): Query<WikiPictureQuery>,
) -> impl IntoResponse {
    let name = match params.name {
        Some(n) => n,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut con: ConnectionManager = state.redis.clone();

    let bg: Option<String> = get_wiki_background(&mut con, &name)
        .await
        .unwrap_or_default();

    (StatusCode::OK, Json(WikiPictureResponse { bg })).into_response()
}
