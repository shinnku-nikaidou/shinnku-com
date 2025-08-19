pub mod files;

use crate::infrastructure::web::http::proxy_service::ProxyService;
use crate::interfaces::http::controllers::{
    search_controller::{search, search_combined},
    wiki_controller::wiki_search_picture,
};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn app_router() -> Router<AppState> {
    let proxy = ProxyService::new("http://127.0.0.1:2998");
    Router::new()
        .route_service("/intro", proxy.clone())
        .route_service("/findname", proxy)
        .route("/search", get(search))
        .route("/combinesearch", get(search_combined))
        .route("/wikisearchpicture", get(wiki_search_picture))
        .nest("/files", files::router())
}
