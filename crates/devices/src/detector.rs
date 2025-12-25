use jxpoolminer_core::Device;
use anyhow::Result;

pub async fn detect_devices() -> Result<Vec<Device>> {
    crate::detect_all().await
}
