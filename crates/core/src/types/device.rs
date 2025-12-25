use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    ASIC,
    GPU { vendor: GPUVendor },
    CPU { cores: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GPUVendor {
    NVIDIA,
    AMD,
    Intel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub capabilities: DeviceCapabilities,
    pub status: DeviceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub max_hashrate: f64,
    pub memory: u64,
    pub supported_algorithms: Vec<crate::Algorithm>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Idle,
    Mining,
    Error(String),
}

impl Device {
    pub fn new(id: String, name: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            capabilities: DeviceCapabilities {
                max_hashrate: 0.0,
                memory: 0,
                supported_algorithms: vec![],
            },
            status: DeviceStatus::Idle,
        }
    }
}
