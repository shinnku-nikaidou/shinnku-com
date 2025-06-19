mod alg;
mod config;
mod fuse;
mod handlers;

use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{find_name, intro};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install error reporting");

    let app = Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name));

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2999)).await?;
    let addr = listener.local_addr()?;
    println!("Listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
