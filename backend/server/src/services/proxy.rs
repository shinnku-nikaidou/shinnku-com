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
            match Self::proxy_request(client, base_url, req).await {
                Ok(response) => Ok(response),
                Err(error) => Ok(error.into_response()),
            }
        })
    }
}

impl ProxyService {
    /// Forwards the incoming request to the target server and returns the response
    async fn proxy_request(
        client: Client,
        base_url: String,
        req: Request<Body>,
    ) -> Result<Response<Body>, AppError> {
        // Extract request components
        let method = req.method().clone();
        let path_and_query = req
            .uri()
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or(req.uri().path());
        let target_url = format!("{base_url}{path_and_query}");

        // Build the proxied request
        let mut request_builder = client.request(method, target_url);

        // Copy headers from original request
        for (name, value) in req.headers() {
            request_builder = request_builder.header(name, value);
        }

        // Extract and forward the request body
        let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to read request body: {e}")))?;

        // Send the proxied request
        let response = request_builder.body(body_bytes).send().await?;

        // Build the response to return
        let status = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let mut response_builder = Response::builder().status(status);

        // Copy response headers (excluding content-length as it may change)
        for (name, value) in response.headers() {
            if name != "content-length" {
                response_builder = response_builder.header(name, value);
            }
        }

        // Get response body and build final response
        let response_bytes = response.bytes().await?;
        Ok(response_builder.body(Body::from(response_bytes)).unwrap())
    }
}
