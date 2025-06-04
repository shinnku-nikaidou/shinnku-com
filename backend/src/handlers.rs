use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;

use crate::data::{FINDNAME_DB, INTRO_DB};

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}

pub async fn intro(Query(params): Query<NameQuery>) -> impl IntoResponse {
    let ramdonshit = "Two things awe me most, the starry sky above me and the moral law within me.\n    ~ Immanuel Kant\n\n";
    if let Some(name) = params.name {
        match fetch_intro(&name).await {
            Ok(Some(content)) => (StatusCode::OK, content).into_response(),
            Ok(None) => (StatusCode::OK, "No results found.".to_string()).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An error occurred: {}", e),
            )
                .into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, ramdonshit.to_string()).into_response()
    }
}

pub async fn find_name(Query(params): Query<NameQuery>) -> impl IntoResponse {
    let ramdonshit = "Two things awe me most, the starry sky above me and the moral law within me.\n    ~ Immanuel Kant\n\n";
    if let Some(name) = params.name {
        match fetch_findname(&name).await {
            Ok(results) => {
                if results.is_empty() {
                    (StatusCode::NOT_FOUND, Json(json!({"ans": []}))).into_response()
                } else {
                    (StatusCode::OK, Json(json!({"ans": results}))).into_response()
                }
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"ans": [], "error": format!("An error occurred: {}", e)})),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": ramdonshit})),
        )
            .into_response()
    }
}

async fn fetch_intro(
    name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(INTRO_DB
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, intro)| intro.to_string()))
}

async fn fetch_findname(
    name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let name_lower = name.to_lowercase();
    Ok(FINDNAME_DB
        .iter()
        .filter(|(n, _)| n.to_lowercase().contains(&name_lower))
        .map(|(_, prompt)| prompt.to_string())
        .collect())
}
