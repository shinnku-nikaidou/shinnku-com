use axum::{Router, routing::get};

mod data;
mod handlers;

use handlers::{find_name, intro};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name));

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2998))
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
