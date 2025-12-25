pub mod detector;
pub mod cpu;
pub mod gpu;
pub mod asic;

pub use detector::*;

use jxpoolminer_core::{Device, DeviceType, Algorithm};
use anyhow::Result;

pub async fn detect_all() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    tracing::info!("Detecting ASIC miners...");
    match asic::detect().await {
        Ok(asics) => {
            tracing::info!("Found {} ASIC device(s)", asics.len());
            devices.extend(asics);
        }
        Err(e) => tracing::warn!("ASIC detection failed: {}", e),
    }
    
    tracing::info!("Detecting GPU devices...");
    match gpu::detect().await {
        Ok(gpus) => {
            tracing::info!("Found {} GPU device(s)", gpus.len());
            devices.extend(gpus);
        }
        Err(e) => tracing::warn!("GPU detection failed: {}", e),
    }
    
    tracing::info!("Detecting CPU devices...");
    match cpu::detect().await {
        Ok(cpus) => {
            tracing::info!("Found {} CPU device(s)", cpus.len());
            devices.extend(cpus);
        }
        Err(e) => tracing::warn!("CPU detection failed: {}", e),
    }
    
    Ok(devices)
}
