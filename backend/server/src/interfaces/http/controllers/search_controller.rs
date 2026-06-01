use crate::application::search::handlers::combined_search_handler::CombinedSearchHandler;
use crate::application::search::handlers::search_files_handler::SearchFilesHandler;
use crate::application::search::queries::combined_search_query::CombinedSearchQuery;
use crate::application::search::queries::search_files_query::SearchFilesQuery;
use crate::error::AppError;
use crate::infrastructure::adapters::search::fuse_search_adapter::FuseSearchAdapter;
use crate::interfaces::http::dto::search_dto::{AiSearchQuery, CombineSearchQuery, SearchQuery};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::time::Duration;
use tokio::task::spawn_blocking;

const AI_SERVICE_URL: &str = "http://127.0.0.1:2998";

lazy_static! {
    static ref AI_CLIENT: reqwest::Client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build AI service HTTP client");
}

#[derive(Deserialize)]
struct FindNameResponse {
    ans: Vec<String>,
}

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

    // Create adapter and handler
    let adapter = FuseSearchAdapter::with_default_config();
    let handler = SearchFilesHandler::new(adapter);

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

    // Create adapter and handler
    let adapter = FuseSearchAdapter::with_default_config();
    let handler = CombinedSearchHandler::new(adapter);

    let results = spawn_blocking(move || handler.handle(&query, &search_index))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(results)).into_response())
}

/// One-shot AI search: hits the Python `/findname` to canonicalize the query
/// name, then runs a combined fuse search using (canonical_name, raw_query).
///
/// # Errors
///
/// Returns an error if:
/// - The query parameter `q` is missing
/// - Task spawning fails
pub async fn ai_search(
    State(state): State<AppState>,
    Query(params): Query<AiSearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let q = params
        .q
        .ok_or_else(|| AppError::BadRequest("missing `q` query param".into()))?;
    let limit = params.n.unwrap_or(200);

    // Best-effort name canonicalization via the AI service. On any failure
    // (network / timeout / parse) we degrade to an empty q1 so the fuse
    // search still runs on the raw user query.
    let q1 = match AI_CLIENT
        .get(format!("{AI_SERVICE_URL}/findname"))
        .query(&[("name", q.as_str())])
        .send()
        .await
    {
        Ok(resp) => match resp.json::<FindNameResponse>().await {
            Ok(body) => body.ans.into_iter().next().unwrap_or_default(),
            Err(e) => {
                tracing::warn!("/findname parse error: {e}");
                String::new()
            }
        },
        Err(e) => {
            tracing::warn!("/findname request error: {e}");
            String::new()
        }
    };

    let search_index = state.root.search_index.clone();
    let query = CombinedSearchQuery::new(q1, q.clone(), limit);

    let adapter = FuseSearchAdapter::with_default_config();
    let handler = CombinedSearchHandler::new(adapter);

    let results = spawn_blocking(move || handler.handle(&query, &search_index))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(results)).into_response())
}
