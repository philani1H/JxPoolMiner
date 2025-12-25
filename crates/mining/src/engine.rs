use jxpoolminer_core::{Device, MiningJob, Share};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct Engine {
    devices: Vec<Device>,
    active_jobs: Arc<RwLock<HashMap<String, MiningJob>>>,
}

impl Engine {
    pub fn new(devices: Vec<Device>) -> Result<Self> {
        Ok(Self {
            devices,
            active_jobs: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn start_mining(&self, device_id: &str, job: MiningJob) -> Result<()> {
        let mut jobs = self.active_jobs.write().await;
        jobs.insert(device_id.to_string(), job);
        Ok(())
    }
    
    pub async fn stop_mining(&self, device_id: &str) -> Result<()> {
        let mut jobs = self.active_jobs.write().await;
        jobs.remove(device_id);
        Ok(())
    }
    
    pub fn devices(&self) -> &[Device] {
        &self.devices
    }
}
