use crate::infrastructure::persistence::redis::connection::connect_redis;

// This test requires a running Redis instance and a valid config.toml.
// It checks that the REDIS connection manager can be initialized and used.
#[tokio::test]
async fn test_redis_get_set() {
    // Skip if no config.toml is present
    if !std::path::Path::new("config.toml").exists() {
        tracing::warn!("Skipping redis test: config.toml not found");
        return;
    }
    let mut con = connect_redis().await.unwrap();
    let key = "img:wiki:zh:5406655";
    let res: String = ::redis::cmd("GET")
        .arg(key)
        .query_async(&mut con)
        .await
        .unwrap();
    tracing::info!("Value for {}: {}", key, res);
}
