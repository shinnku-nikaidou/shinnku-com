use axum::{
    Json,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::Client;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}

async fn proxy(path: &str, name: Option<String>) -> Result<impl IntoResponse, ProxyError> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:2998{path}");
    let req = if let Some(ref n) = name {
        client.get(&url).query(&[("name", n)])
    } else {
        client.get(&url)
    };

    let resp = req.send().await.map_err(ProxyError::Request)?;
    let status =
        StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let json = resp
        .json::<serde_json::Value>()
        .await
        .map_err(ProxyError::Json)?;
    Ok((status, Json(json)))
}

#[derive(Debug)]
pub enum ProxyError {
    Request(reqwest::Error),
    Json(reqwest::Error),
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::Request(err) => write!(f, "request error: {err}"),
            ProxyError::Json(err) => write!(f, "json error: {err}"),
        }
    }
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        match self {
            ProxyError::Request(_) => {
                (StatusCode::BAD_GATEWAY, "Failed to proxy request").into_response()
            }
            ProxyError::Json(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub async fn intro(Query(params): Query<NameQuery>) -> Result<impl IntoResponse, ProxyError> {
    proxy("/intro", params.name).await
}

pub async fn find_name(Query(params): Query<NameQuery>) -> Result<impl IntoResponse, ProxyError> {
    proxy("/findname", params.name).await
}
