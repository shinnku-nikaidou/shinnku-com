mod application;
mod config;
mod configuration;
mod domain;
mod dto;
mod handlers;
mod infrastructure;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use routes::app_router;
use state::AppState;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    fmt::init();

    let redis = config::redis::connect_redis().await?;
    let root = config::startup::load_root().await?;
    let tree = config::tree::build_tree(&root.shinnku_tree, &root.galgame0_tree);
    let state = AppState { redis, root, tree };

    let app = app_router()
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2999)).await?;
    let addr = listener.local_addr()?;
    info!("Listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
