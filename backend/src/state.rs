use crate::{alg::root::Root, config::TreeNode};
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub root: Root,
    pub tree: TreeNode,
}
