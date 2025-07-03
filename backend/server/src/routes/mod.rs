pub mod files;

use crate::handlers::{
    intro::{find_name, intro},
    search::{search, search_combined},
    wiki::wiki_search_picture,
};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn app_router() -> Router<AppState> {
    Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name))
        .route("/search", get(search))
        .route("/combinesearch", get(search_combined))
        .route("/wikisearchpicture", get(wiki_search_picture))
        .nest("/files", files::router())
}
