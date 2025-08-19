use crate::models::{BucketFiles, NodeType, TreeNode};

/// Generate a hierarchical tree from a flat list of files.
pub fn generate_tree(file_list: &BucketFiles) -> TreeNode {
    let mut root = TreeNode::new();

    for file in file_list {
        let path_parts: Vec<&str> = file.file_path.split('/').collect();
        let mut pointer = &mut root;
        let last_idx = path_parts.len() - 1;

        for part in &path_parts[..last_idx] {
            pointer = match pointer
                .entry(part.to_string())
                .or_insert_with(|| NodeType::Node(TreeNode::new()))
            {
                NodeType::Node(node) => node,
                NodeType::File(_) => {
                    tracing::error!("Expected folder but found file at path: {}", part);
                    return TreeNode::new(); // Return empty tree on error
                }
            };
        }

        pointer.insert(
            path_parts[last_idx].to_string(),
            NodeType::File(file.clone()),
        );
    }

    root
}

/// Construct the combined tree used by the frontend.
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
pub fn build_tree(shinnku_tree: &TreeNode, galgame0_tree: &TreeNode) -> TreeNode {
    let mut tree = TreeNode::new();
    tree.insert("shinnku".into(), NodeType::Node(shinnku_tree.clone()));

    let galgame0_sub = galgame0_tree
        .get("合集系列")
        .and_then(|v| match v {
            NodeType::Node(node) => node.get("浮士德galgame游戏合集"),
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

    tree.insert("galgame0".into(), NodeType::Node(galgame0_sub));
    tree
}
