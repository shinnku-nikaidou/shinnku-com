mod alg;
mod config;
mod fuse;
mod handlers;

use alg::root::get_root;
use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{find_name, inode, intro};

#[tokio::main]
async fn main() -> Result<()> {
    config::get_redis().await;
    get_root().await;
    color_eyre::install().expect("Failed to install error reporting");

    let app = Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name))
        .route("/files/*path", get(inode));

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2999)).await?;
    let addr = listener.local_addr()?;
    println!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
