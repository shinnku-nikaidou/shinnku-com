use crate::error::AppError;
use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}

async fn proxy(path: &str, name: Option<String>) -> Result<impl IntoResponse, AppError> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:2998{path}");
    let req = if let Some(ref n) = name {
        client.get(&url).query(&[("name", n)])
    } else {
        client.get(&url)
    };

    let resp = req.send().await?;
    let status =
        StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let json = resp.json::<serde_json::Value>().await?;
    Ok((status, Json(json)))
}

pub async fn intro(Query(params): Query<NameQuery>) -> Result<impl IntoResponse, AppError> {
    proxy("/intro", params.name).await
}

pub async fn find_name(Query(params): Query<NameQuery>) -> Result<impl IntoResponse, AppError> {
    proxy("/findname", params.name).await
}
