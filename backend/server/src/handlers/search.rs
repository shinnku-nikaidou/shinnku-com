use crate::error::AppError;
#[cfg(test)]
use crate::functions::root;
use crate::functions::search::{combine_search, runsearch};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

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

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let q = match params.q {
        Some(q) => q,
        None => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };

    let search_index = &state.root.search_index;
    let n = params.n.unwrap_or(100);
    let results = runsearch(&q, search_index);
    let sliced: Vec<_> = results.into_iter().take(n).collect();
    Ok((StatusCode::OK, Json(sliced)).into_response())
}

pub async fn search_combined(
    State(state): State<AppState>,
    Query(params): Query<CombineSearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let (q1, q2) = match (params.q1, params.q2) {
        (Some(q1), Some(q2)) => (q1, q2),
        _ => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };

    let search_index = &state.root.search_index;
    let n = params.n.unwrap_or(100);
    let results = combine_search(&q1, &q2, n, search_index);
    Ok((StatusCode::OK, Json(results)).into_response())
}

#[cfg(test)]
mod tests {
    use crate::functions::{root, search::runsearch};

    #[tokio::test]
    async fn test_search() {
        let q = "サノバウィッチ";
        let root = root::load_root().unwrap();
        let search_index = &root.search_index;
        let n = 20;
        let results = runsearch(q, search_index);
        let sliced: Vec<_> = results.into_iter().take(n).collect();
        tracing::info!("Search results for '{q}': {sliced:?}");
    }
}
