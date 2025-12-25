use jxpoolminer_core::{Device, DeviceType, DeviceCapabilities, Algorithm, GPUVendor};
use anyhow::Result;
use sysinfo::System;

pub async fn detect() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    #[cfg(target_os = "linux")]
    {
        devices.extend(detect_linux().await?);
    }
    
    #[cfg(target_os = "windows")]
    {
        devices.extend(detect_windows().await?);
    }
    
    #[cfg(target_os = "macos")]
    {
        devices.extend(detect_macos().await?);
    }
    
    Ok(devices)
}

#[cfg(target_os = "linux")]
async fn detect_linux() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    if let Ok(output) = tokio::process::Command::new("lspci")
        .output()
        .await
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for (idx, line) in output_str.lines().enumerate() {
            let line_lower = line.to_lowercase();
            
            if line_lower.contains("vga") || line_lower.contains("3d") || line_lower.contains("display") {
                let (vendor, name) = if line_lower.contains("nvidia") {
                    (GPUVendor::NVIDIA, extract_gpu_name(line, "NVIDIA"))
                } else if line_lower.contains("amd") || line_lower.contains("radeon") {
                    (GPUVendor::AMD, extract_gpu_name(line, "AMD"))
                } else if line_lower.contains("intel") {
                    (GPUVendor::Intel, extract_gpu_name(line, "Intel"))
                } else {
                    continue;
                };
                
                let memory = estimate_gpu_memory(&vendor);
                let hashrate = estimate_gpu_hashrate(&vendor);
                
                devices.push(Device {
                    id: format!("gpu-{}", idx),
                    name,
                    device_type: DeviceType::GPU { vendor: vendor.clone() },
                    capabilities: DeviceCapabilities {
                        max_hashrate: hashrate,
                        memory,
                        supported_algorithms: vec![Algorithm::Ethash],
                    },
                    status: jxpoolminer_core::DeviceStatus::Idle,
                });
            }
        }
    }
    
    Ok(devices)
}

#[cfg(target_os = "windows")]
async fn detect_windows() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    if let Ok(output) = tokio::process::Command::new("wmic")
        .args(&["path", "win32_VideoController", "get", "name"])
        .output()
        .await
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for (idx, line) in output_str.lines().skip(1).enumerate() {
            let line = line.trim();
            if line.is_empty() || line == "Name" {
                continue;
            }
            
            let line_lower = line.to_lowercase();
            let (vendor, name) = if line_lower.contains("nvidia") {
                (GPUVendor::NVIDIA, line.to_string())
            } else if line_lower.contains("amd") || line_lower.contains("radeon") {
                (GPUVendor::AMD, line.to_string())
            } else if line_lower.contains("intel") {
                (GPUVendor::Intel, line.to_string())
            } else {
                continue;
            };
            
            let memory = estimate_gpu_memory(&vendor);
            let hashrate = estimate_gpu_hashrate(&vendor);
            
            devices.push(Device {
                id: format!("gpu-{}", idx),
                name,
                device_type: DeviceType::GPU { vendor: vendor.clone() },
                capabilities: DeviceCapabilities {
                    max_hashrate: hashrate,
                    memory,
                    supported_algorithms: vec![Algorithm::Ethash],
                },
                status: jxpoolminer_core::DeviceStatus::Idle,
            });
        }
    }
    
    Ok(devices)
}

#[cfg(target_os = "macos")]
async fn detect_macos() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    
    if let Ok(output) = tokio::process::Command::new("system_profiler")
        .args(&["SPDisplaysDataType"])
        .output()
        .await
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for (idx, line) in output_str.lines().enumerate() {
            if line.trim().starts_with("Chipset Model:") {
                let name = line.split(':').nth(1)
                    .unwrap_or("Unknown GPU")
                    .trim()
                    .to_string();
                
                let line_lower = name.to_lowercase();
                let vendor = if line_lower.contains("nvidia") {
                    GPUVendor::NVIDIA
                } else if line_lower.contains("amd") || line_lower.contains("radeon") {
                    GPUVendor::AMD
                } else {
                    GPUVendor::Intel
                };
                
                let memory = estimate_gpu_memory(&vendor);
                let hashrate = estimate_gpu_hashrate(&vendor);
                
                devices.push(Device {
                    id: format!("gpu-{}", idx),
                    name,
                    device_type: DeviceType::GPU { vendor: vendor.clone() },
                    capabilities: DeviceCapabilities {
                        max_hashrate: hashrate,
                        memory,
                        supported_algorithms: vec![Algorithm::Ethash],
                    },
                    status: jxpoolminer_core::DeviceStatus::Idle,
                });
            }
        }
    }
    
    Ok(devices)
}

fn extract_gpu_name(line: &str, vendor: &str) -> String {
    if let Some(pos) = line.find(vendor) {
        line[pos..].split('[').next()
            .unwrap_or(vendor)
            .trim()
            .to_string()
    } else {
        format!("{} GPU", vendor)
    }
}

fn estimate_gpu_memory(vendor: &GPUVendor) -> u64 {
    match vendor {
        GPUVendor::NVIDIA => 8 * 1024 * 1024 * 1024,
        GPUVendor::AMD => 8 * 1024 * 1024 * 1024,
        GPUVendor::Intel => 4 * 1024 * 1024 * 1024,
    }
}

fn estimate_gpu_hashrate(vendor: &GPUVendor) -> f64 {
    match vendor {
        GPUVendor::NVIDIA => 100_000_000.0,
        GPUVendor::AMD => 80_000_000.0,
        GPUVendor::Intel => 20_000_000.0,
    }
}
