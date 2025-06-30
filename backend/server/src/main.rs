mod algorithm;
mod config;
mod handlers;
mod services;
mod state;

use anyhow::Result;
use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use handlers::{
    find_name, get_node, get_node_root, intro, search, search_combined, wiki_search_picture,
};
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install error reporting");

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
        .route(
            "/files",
            get(|state: State<AppState>| async move { get_node_root(state) }),
        )
        .route(
            "/files/{*path}",
            get(|path: Path<String>, state: State<AppState>| async move { get_node(path, state) }),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2999)).await?;
    let addr = listener.local_addr()?;
    println!("Listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
