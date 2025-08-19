use anyhow::Result;
use shinnku_com_backend::functions::root::*;

#[tokio::test]
async fn test_load_root() -> Result<()> {
    let root = load_root().await?;
    tracing::info!("Shinnku tree: {:?}", root.shinnku_tree);
    tracing::info!("Galgame0 tree: {:?}", root.galgame0_tree);
    Ok(())
}
