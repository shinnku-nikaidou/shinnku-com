mod config;
mod functions;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

use routes::app_router;
use state::AppState;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt;
mod error;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    color_eyre::install().expect("Failed to install error reporting");
    fmt::init();

    let redis = config::connect_redis().await?;
    let root = functions::root::load_root().await?;
    let tree = functions::root::build_tree(&root.shinnku_tree, &root.galgame0_tree);
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
