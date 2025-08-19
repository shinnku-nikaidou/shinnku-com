use crate::{
    application::shared::services::application_bootstrap_service::ApplicationData,
    domain::files::entities::tree_node::TreeNode,
};
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub root: ApplicationData,
    pub tree: TreeNode,
}
