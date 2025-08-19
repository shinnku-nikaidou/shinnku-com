use crate::{config::startup::Root, domain::files::entities::tree_node::TreeNode};
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub root: Root,
    pub tree: TreeNode,
}
