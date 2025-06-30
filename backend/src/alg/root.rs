use crate::alg::search::{SearchList, aggregate_builder};
use crate::config::{BucketFiles, NodeValue, TreeNode};
use anyhow::Result;
use tokio::fs;

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
                .or_insert_with(|| NodeValue::Node(TreeNode::new()))
            {
                NodeValue::Node(node) => node,
                NodeValue::File(_) => panic!("expected folder, found file"),
            };
        }

        pointer.insert(
            path_parts[last_idx].to_string(),
            NodeValue::File(file.clone()),
        );
    }

    root
}

#[derive(Clone)]
pub struct Root {
    pub shinnku_tree: TreeNode,
    pub galgame0_tree: TreeNode,
    pub search_index: SearchList,
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
    tree.insert("shinnku".into(), NodeValue::Node(shinnku_tree.clone()));

    let galgame0_sub = galgame0_tree
        .get("合集系列")
        .and_then(|v| match v {
            NodeValue::Node(node) => node.get("浮士德galgame游戏合集"),
            _ => None,
        })
        .and_then(|v| match v {
            NodeValue::Node(node) => Some(node.clone()),
            _ => None,
        })
        .expect("expected galgame0 subtree");

    tree.insert("galgame0".into(), NodeValue::Node(galgame0_sub));
    tree
}

/// Load bucket files and build trees and search index.
pub async fn load_root() -> Result<Root> {
    let shinnku_raw = fs::read_to_string("data/shinnku_bucket_files.json").await?;
    let galgame0_raw = fs::read_to_string("data/galgame0_bucket_files.json").await?;

    let shinnku_bucket_files: BucketFiles = serde_json::from_str(&shinnku_raw)?;
    let galgame0_bucket_files: BucketFiles = serde_json::from_str(&galgame0_raw)?;

    let shinnku_tree = generate_tree(&shinnku_bucket_files);
    let galgame0_tree = generate_tree(&galgame0_bucket_files);

    let galgame0_filtered: BucketFiles = galgame0_bucket_files
        .iter()
        .filter(|v| v.file_path.starts_with("合集系列/浮士德galgame游戏合集"))
        .cloned()
        .collect();

    let search_index = aggregate_builder(&[shinnku_bucket_files.clone(), galgame0_filtered]);

    Ok(Root {
        shinnku_tree,
        galgame0_tree,
        search_index,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_root() -> Result<()> {
        let root = load_root().await?;
        println!("Shinnku tree: {:?}", root.shinnku_tree);
        println!("Galgame0 tree: {:?}", root.galgame0_tree);
        Ok(())
    }
}
