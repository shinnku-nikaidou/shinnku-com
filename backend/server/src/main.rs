mod alg;
mod config;
mod handlers;
mod state;

use anyhow::Result;
use axum::{Router, routing::get};
use handlers::*;
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install error reporting");

    let redis = config::connect_redis().await?;
    let root = alg::root::load_root()?;
    let tree = alg::root::build_tree(&root.shinnku_tree, &root.galgame0_tree);
    let state = AppState { redis, root, tree };

    let app = Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name))
        .route("/search", get(search))
        .route("/conbinesearch", get(combine_search_query))
        .route("/wikisearchpicture", get(wikisearchpicture))
        .route("/files", get(inode_root))
        .route("/files/{*path}", get(inode))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2999)).await?;
    let addr = listener.local_addr()?;
    println!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
