use crate::{models::TreeNode, services::root::Root};
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub root: Root,
    pub tree: TreeNode,
}
