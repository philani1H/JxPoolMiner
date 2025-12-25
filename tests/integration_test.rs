use jxpoolminer_devices;
use jxpoolminer_mining;
use jxpoolminer_pool;
use jxpoolminer_config;

#[tokio::test]
async fn test_device_detection() {
    let devices = jxpoolminer_devices::detect_all().await.unwrap();
    assert!(!devices.is_empty(), "Should detect at least CPU");
    println!("✅ Detected {} device(s)", devices.len());
}

#[tokio::test]
async fn test_mining_engine() {
    let devices = jxpoolminer_devices::detect_all().await.unwrap();
    let engine = jxpoolminer_mining::Engine::new(devices).unwrap();
    println!("✅ Mining engine created");
}

#[tokio::test]
async fn test_pool_connection() {
    let config = jxpoolminer_pool::PoolConfig::default();
    let client = jxpoolminer_pool::Client::connect(&config).await.unwrap();
    assert!(client.is_connected().await);
    println!("✅ Pool connection established");
}

#[tokio::test]
async fn test_config_loading() {
    let config = jxpoolminer_config::load_config().unwrap();
    assert_eq!(config.app.theme, "dark");
    println!("✅ Configuration loaded");
}
