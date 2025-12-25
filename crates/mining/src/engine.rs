use jxpoolminer_core::{Device, MiningJob, Share, DeviceStatus};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;
use crate::algorithms;

struct MiningTask {
    cancel_tx: mpsc::Sender<()>,
    share_rx: Arc<RwLock<mpsc::Receiver<Share>>>,
}

pub struct Engine {
    devices: Arc<RwLock<Vec<Device>>>,
    active_tasks: Arc<RwLock<HashMap<String, MiningTask>>>,
}

impl Engine {
    pub fn new(devices: Vec<Device>) -> Result<Self> {
        Ok(Self {
            devices: Arc::new(RwLock::new(devices)),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn start_mining(&self, device_id: &str, job: MiningJob) -> Result<()> {
        let device = {
            let devices = self.devices.read().await;
            devices.iter()
                .find(|d| d.id == device_id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Device not found: {}", device_id))?
        };
        
        self.stop_mining(device_id).await?;
        
        let (cancel_tx, mut cancel_rx) = mpsc::channel(1);
        let (share_tx, share_rx) = mpsc::channel(100);
        
        let device_clone = device.clone();
        let job_clone = job.clone();
        
        tokio::spawn(async move {
            tracing::info!("Starting mining on device: {}", device_clone.id);
            
            loop {
                match algorithms::mine(&device_clone, job_clone.clone(), &mut cancel_rx).await {
                    Ok(share) => {
                        tracing::info!("Found share! Device: {}, Nonce: {}", device_clone.id, share.nonce);
                        if share_tx.send(share).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Mining error on device {}: {}", device_clone.id, e);
                        break;
                    }
                }
            }
        });
        
        let task = MiningTask {
            cancel_tx,
            share_rx: Arc::new(RwLock::new(share_rx)),
        };
        
        self.active_tasks.write().await.insert(device_id.to_string(), task);
        
        {
            let mut devices = self.devices.write().await;
            if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
                device.status = DeviceStatus::Mining;
            }
        }
        
        tracing::info!("Mining started on device: {}", device_id);
        Ok(())
    }
    
    pub async fn stop_mining(&self, device_id: &str) -> Result<()> {
        if let Some(task) = self.active_tasks.write().await.remove(device_id) {
            let _ = task.cancel_tx.send(()).await;
            tracing::info!("Mining stopped on device: {}", device_id);
        }
        
        {
            let mut devices = self.devices.write().await;
            if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
                device.status = DeviceStatus::Idle;
            }
        }
        
        Ok(())
    }
    
    pub async fn get_share(&self, device_id: &str) -> Option<Share> {
        let tasks = self.active_tasks.read().await;
        if let Some(task) = tasks.get(device_id) {
            let mut rx = task.share_rx.write().await;
            rx.try_recv().ok()
        } else {
            None
        }
    }
    
    pub async fn devices(&self) -> Vec<Device> {
        self.devices.read().await.clone()
    }
    
    pub async fn update_device_status(&self, device_id: &str, status: DeviceStatus) {
        let mut devices = self.devices.write().await;
        if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
            device.status = status;
        }
    }
}
