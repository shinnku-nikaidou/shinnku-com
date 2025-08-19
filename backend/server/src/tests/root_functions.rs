use crate::application::shared::services::application_bootstrap_service::ApplicationBootstrapService;
use anyhow::Result;

#[tokio::test]
async fn test_load_root() -> Result<()> {
    let bootstrap_service = ApplicationBootstrapService::new();
    let root = bootstrap_service.initialize().await?;
    tracing::info!("root tree: {:?}", root.combined_tree);
    Ok(())
}
