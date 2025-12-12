use crate::domain::files::entities::tree_node::{NodeType, TreeNode};

/// Application service for file tree operations
pub struct FileTreeService;

impl FileTreeService {
    /// Construct the combined tree used by the frontend
    ///
    /// This mirrors the following TypeScript snippet:
    /// ```ts
    /// export const tree = {
    ///   shinnku: shinnku_tree,
    ///   galgame0: (galgame0_tree['合集系列'] as TreeNode)[
    ///     '浮士德galgame游戏合集'
    ///   ] as TreeNode,
    /// }
    /// ```
    pub fn build_combined_frontend_tree(
        shinnku_tree: &TreeNode,
        galgame0_tree: &TreeNode,
    ) -> TreeNode {
        let mut tree = TreeNode::new();
        tree.as_mut()
            .insert("shinnku".into(), NodeType::Node(shinnku_tree.clone()));

        let galgame0_sub = galgame0_tree
            .as_ref()
            .get("合集系列")
            .and_then(|v| match v {
                NodeType::Node(node) => node.as_ref().get("浮士德galgame游戏合集"),
                _ => None,
            })
            .and_then(|v| match v {
                NodeType::Node(node) => Some(node.clone()),
                _ => None,
            });

        let galgame0_sub = match galgame0_sub {
            Some(sub) => sub,
            None => {
                tracing::error!("Expected galgame0 subtree not found");
                TreeNode::new() // Return empty tree if subtree not found
            }
        };

        tree.as_mut()
            .insert("galgame0".into(), NodeType::Node(galgame0_sub));
        tree
    }
}
