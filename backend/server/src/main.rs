mod algorithm;
mod config;
mod handlers;
mod services;
mod state;

use axum::{Router, routing::get};
use handlers::*;
use state::AppState;
use tracing::info;
use tracing_subscriber::fmt;
mod error;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    color_eyre::install().expect("Failed to install error reporting");
    fmt::init();

    let redis = config::connect_redis().await?;
    let root = algorithm::root::load_root()?;
    let tree = algorithm::root::build_tree(&root.shinnku_tree, &root.galgame0_tree);
    let state = AppState { redis, root, tree };

    let app = Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name))
        .route("/search", get(search))
        .route("/combinesearch", get(search_combined))
        .route("/wikisearchpicture", get(wiki_search_picture))
        .route("/files", get(get_node_root))
        .route("/files/{*path}", get(get_node))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2999)).await?;
    let addr = listener.local_addr()?;
    info!("Listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
