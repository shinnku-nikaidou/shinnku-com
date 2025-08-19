use crate::interfaces::http::controllers::files_controller::{get_node, get_node_root};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_node_root))
        .route("/{*path}", get(get_node))
}
