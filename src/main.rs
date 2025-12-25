//! JxPoolMiner - Professional Cross-Platform Mining Software
//! 
//! A modern, high-performance mining application with:
//! - Automatic device detection (ASIC, GPU, CPU)
//! - Multi-algorithm support (SHA-256, Ethash, GXHash)
//! - Modern GUI with real-time statistics
//! - Pool connection with Stratum V1/V2
//! - Auto-update system

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("🚀 JxPoolMiner v{} starting...", env!("CARGO_PKG_VERSION"));
    
    // Load configuration
    info!("📝 Loading configuration...");
    let config = jxpoolminer_config::load_config()?;
    
    // Detect devices
    info!("🔍 Detecting mining devices...");
    let devices = jxpoolminer_devices::detect_all().await?;
    info!("✅ Found {} device(s)", devices.len());
    
    for device in &devices {
        info!("  - {} ({:?})", device.name, device.device_type);
    }
    
    // Initialize mining engine
    info!("⚙️  Initializing mining engine...");
    let mining_engine = jxpoolminer_mining::Engine::new(devices.clone())?;
    
    // Connect to pool
    info!("🌐 Connecting to pool: {}", config.pool.primary);
    let pool_client = jxpoolminer_pool::Client::connect(&config.pool).await?;
    
    // Initialize statistics collector
    info!("📊 Starting statistics collector...");
    let stats_collector = jxpoolminer_stats::Collector::new();
    
    // Start GUI
    info!("🖥️  Launching GUI...");
    jxpoolminer_gui::run(
        config,
        devices,
        mining_engine,
        pool_client,
        stats_collector,
    ).await?;
    
    info!("👋 JxPoolMiner shutting down...");
    Ok(())
}
