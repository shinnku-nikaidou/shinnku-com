mod config;
mod dto;
mod handlers;
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

    let redis = config::connect_redis().await?;
    let root = services::root::load_root().await?;
    let tree = services::root::build_tree(&root.shinnku_tree, &root.galgame0_tree);
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
