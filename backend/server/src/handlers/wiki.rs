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

pub async fn wikisearchpicture(
    State(state): State<AppState>,
    Query(params): Query<WikiPictureQuery>,
) -> impl IntoResponse {
    let name = match params.name {
        Some(n) => n,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut con: ConnectionManager = state.redis.clone();

    let key_search = format!("cache:search:wiki:zh:{}", name);
    let pageid: Option<String> = match redis::cmd("GET")
        .arg(&key_search)
        .query_async::<Option<String>>(&mut con)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Redis GET {} error: {}", key_search, e);
            None
        }
    };

    if let Some(pageid) = pageid {
        let key_img = format!("img:wiki:zh:{}", pageid);
        let bg = match redis::cmd("GET")
            .arg(&key_img)
            .query_async::<Option<String>>(&mut con)
            .await
        {
            Ok(bg) => bg,
            Err(e) => {
                eprintln!("Redis GET {} error: {}", key_img, e);
                None
            }
        };
        return (StatusCode::OK, Json(WikiPictureResponse { bg })).into_response();
    }

    (StatusCode::OK, Json(WikiPictureResponse { bg: None })).into_response()
}
