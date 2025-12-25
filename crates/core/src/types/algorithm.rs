use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Algorithm {
    SHA256,
    Ethash,
    GXHash,
}

impl Algorithm {
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::SHA256 => "SHA-256",
            Algorithm::Ethash => "Ethash",
            Algorithm::GXHash => "GXHash",
        }
    }
    
    pub fn for_device(device_type: &super::DeviceType) -> Self {
        match device_type {
            super::DeviceType::ASIC => Algorithm::SHA256,
            super::DeviceType::GPU { .. } => Algorithm::Ethash,
            super::DeviceType::CPU { .. } => Algorithm::GXHash,
        }
    }
}
