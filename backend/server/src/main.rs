mod application;
mod configuration;
mod domain;
mod error;
mod infrastructure;
mod interfaces;
mod shared;
mod state;

#[cfg(test)]
mod tests;

use crate::application::shared::services::application_bootstrap_service::ApplicationBootstrapService;
use crate::domain::files::factories::tree_factory::TreeFactory;
use crate::interfaces::http::routes::app_router::app_router;
use state::AppState;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    fmt::init();

    let redis = infrastructure::persistence::redis::connection::connect_redis().await?;
    let bootstrap_service = ApplicationBootstrapService::new();
    let root = bootstrap_service.initialize().await?;
    let tree = TreeFactory::combine_frontend_trees(&root.shinnku_tree, &root.galgame0_tree);
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
