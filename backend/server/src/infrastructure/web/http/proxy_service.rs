use crate::error::AppError;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use reqwest::Client;
use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;

#[derive(Clone)]
pub struct ProxyService {
    client: Client,
    base_url: String,
}

impl ProxyService {
    pub fn new(base_url: impl Into<String>) -> Self {
        // Disable the idle keep-alive pool: uvicorn closes idle HTTP/1
        // connections aggressively, which makes pooled requests race and fail
        // with "connection closed before message completed".
        let client = Client::builder()
            .pool_max_idle_per_host(0)
            .build()
            .expect("failed to build reqwest client");
        Self {
            client,
            base_url: base_url.into(),
        }
    }
}

impl Service<Request<Body>> for ProxyService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let client = self.client.clone();
        let base_url = self.base_url.clone();

        Box::pin(async move {
            match Self::proxy_request(client, base_url, req).await {
                Ok(response) => Ok(response),
                Err(error) => Ok(error.into_response()),
            }
        })
    }
}

impl ProxyService {
    /// Forward only path+query as a clean GET; inbound headers and body are dropped.
    async fn proxy_request(
        client: Client,
        base_url: String,
        req: Request<Body>,
    ) -> Result<Response<Body>, AppError> {
        let path_and_query = req
            .uri()
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or(req.uri().path());
        let target_url = format!("{base_url}{path_and_query}");

        let response = match client.get(&target_url).send().await {
            Ok(r) => r,
            Err(e) => {
                let mut chain = format!("{e}");
                let mut src: Option<&dyn std::error::Error> = std::error::Error::source(&e);
                while let Some(s) = src {
                    chain.push_str(" | caused by: ");
                    chain.push_str(&s.to_string());
                    src = s.source();
                }
                tracing::error!("proxy GET {target_url} failed: {chain}");
                return Err(AppError::Internal(chain));
            }
        };

        let status = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .cloned();

        let body_bytes = response.bytes().await?;

        let mut builder = Response::builder().status(status);
        if let Some(ct) = content_type {
            builder = builder.header(reqwest::header::CONTENT_TYPE, ct);
        }
        builder
            .body(Body::from(body_bytes))
            .map_err(|e| AppError::Internal(format!("Failed to build response body: {e}")))
    }
}
