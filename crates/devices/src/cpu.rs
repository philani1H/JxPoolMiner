use jxpoolminer_core::{Device, DeviceType, DeviceCapabilities, Algorithm};
use anyhow::Result;
use sysinfo::System;

pub async fn detect() -> Result<Vec<Device>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let cpu_count = sys.cpus().len();
    let cpu_name = sys.cpus().first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    
    let device = Device {
        id: "cpu-0".to_string(),
        name: cpu_name,
        device_type: DeviceType::CPU { cores: cpu_count },
        capabilities: DeviceCapabilities {
            max_hashrate: (cpu_count as f64) * 1000.0, // Estimate
            memory: sys.total_memory(),
            supported_algorithms: vec![Algorithm::GXHash],
        },
        status: jxpoolminer_core::DeviceStatus::Idle,
    };
    
    Ok(vec![device])
}
