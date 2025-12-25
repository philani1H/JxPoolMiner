pub mod detector;
pub mod cpu;

pub use detector::*;

use jxpoolminer_core::{Device, DeviceType, Algorithm};
use anyhow::Result;

pub async fn detect_all() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    // Detect CPUs (always available)
    devices.extend(cpu::detect().await?);
    
    // TODO: Detect ASICs
    // TODO: Detect GPUs
    
    Ok(devices)
}
