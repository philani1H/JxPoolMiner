use jxpoolminer_core::{Device, DeviceType, DeviceCapabilities, Algorithm};
use anyhow::Result;

pub async fn detect() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    #[cfg(target_os = "linux")]
    {
        devices.extend(detect_linux().await?);
    }
    
    Ok(devices)
}

#[cfg(target_os = "linux")]
async fn detect_linux() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    if let Ok(output) = tokio::process::Command::new("lsusb")
        .output()
        .await
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for (idx, line) in output_str.lines().enumerate() {
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("bitmain") || 
               line_lower.contains("antminer") ||
               line_lower.contains("whatsminer") ||
               line_lower.contains("avalon") ||
               line_lower.contains("asic") {
                
                let name = extract_asic_name(line);
                let hashrate = estimate_asic_hashrate(&name);
                
                devices.push(Device {
                    id: format!("asic-{}", idx),
                    name,
                    device_type: DeviceType::ASIC,
                    capabilities: DeviceCapabilities {
                        max_hashrate: hashrate,
                        memory: 0,
                        supported_algorithms: vec![Algorithm::SHA256],
                    },
                    status: jxpoolminer_core::DeviceStatus::Idle,
                });
            }
        }
    }
    
    if let Ok(entries) = std::fs::read_dir("/dev") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            if name_str.starts_with("ttyUSB") || name_str.starts_with("ttyACM") {
                if let Ok(device) = detect_cgminer_device(&entry.path()).await {
                    devices.push(device);
                }
            }
        }
    }
    
    Ok(devices)
}

async fn detect_cgminer_device(path: &std::path::Path) -> Result<Device> {
    let name = format!("ASIC Miner ({})", path.display());
    
    Ok(Device {
        id: format!("asic-{}", path.display()),
        name,
        device_type: DeviceType::ASIC,
        capabilities: DeviceCapabilities {
            max_hashrate: 100_000_000_000_000.0,
            memory: 0,
            supported_algorithms: vec![Algorithm::SHA256],
        },
        status: jxpoolminer_core::DeviceStatus::Idle,
    })
}

fn extract_asic_name(line: &str) -> String {
    if let Some(pos) = line.find("ID") {
        if let Some(desc_start) = line[pos..].find(char::is_alphabetic) {
            return line[pos + desc_start..].trim().to_string();
        }
    }
    "ASIC Miner".to_string()
}

fn estimate_asic_hashrate(name: &str) -> f64 {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("s19") {
        110_000_000_000_000.0
    } else if name_lower.contains("s17") {
        56_000_000_000_000.0
    } else if name_lower.contains("s9") {
        14_000_000_000_000.0
    } else if name_lower.contains("whatsminer") {
        100_000_000_000_000.0
    } else if name_lower.contains("avalon") {
        50_000_000_000_000.0
    } else {
        50_000_000_000_000.0
    }
}
