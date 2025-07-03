use crate::error::AppError;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use reqwest::Client;
use std::convert::Infallible;
use std::{
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
        Self {
            client: Client::new(),
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
            let result: Result<Response<Body>, AppError> = async {
                let method = req.method().clone();
                let path_and_query = req
                    .uri()
                    .path_and_query()
                    .map(|pq| pq.as_str())
                    .unwrap_or(req.uri().path());
                let url = format!("{base_url}{path_and_query}");
                let mut request_builder = client.request(method, url);
                for (k, v) in req.headers() {
                    request_builder = request_builder.header(k, v);
                }
                let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                let resp = request_builder.body(body_bytes).send().await?;
                let status = StatusCode::from_u16(resp.status().as_u16())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                let mut builder = Response::builder().status(status);
                for (k, v) in resp.headers() {
                    if k != "content-length" {
                        builder = builder.header(k, v);
                    }
                }
                let bytes = resp.bytes().await?;
                Ok(builder.body(Body::from(bytes)).unwrap())
            }
            .await;
            Ok(result.unwrap_or_else(|e| e.into_response()))
        })
    }
}
