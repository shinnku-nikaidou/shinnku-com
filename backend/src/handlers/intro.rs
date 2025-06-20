use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}

async fn proxy(path: &str, name: Option<String>) -> impl IntoResponse {
    let client = Client::new();
    let url = format!("http://127.0.0.1:2998{}", path);
    let req = if let Some(ref n) = name {
        client.get(&url).query(&[("name", n)])
    } else {
        client.get(&url)
    };
    match req.send().await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            match resp.json::<serde_json::Value>().await {
                Ok(json) => (status, Json(json)).into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => (StatusCode::BAD_GATEWAY, "Failed to proxy request").into_response(),
    }
}

pub async fn intro(Query(params): Query<NameQuery>) -> impl IntoResponse {
    proxy("/intro", params.name).await
}

pub async fn find_name(Query(params): Query<NameQuery>) -> impl IntoResponse {
    proxy("/findname", params.name).await
}
