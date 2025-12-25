use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct DeviceStats {
    pub hashrate: f64,
    pub shares_accepted: u64,
    pub shares_rejected: u64,
    pub temperature: f32,
}

impl Default for DeviceStats {
    fn default() -> Self {
        Self {
            hashrate: 0.0,
            shares_accepted: 0,
            shares_rejected: 0,
            temperature: 0.0,
        }
    }
}

pub struct Collector {
    hashrate_history: Arc<RwLock<VecDeque<f64>>>,
    device_stats: Arc<RwLock<HashMap<String, DeviceStats>>>,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            hashrate_history: Arc::new(RwLock::new(VecDeque::new())),
            device_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn record_hashrate(&self, device_id: &str, hashrate: f64) {
        let mut history = self.hashrate_history.write().await;
        history.push_back(hashrate);
        
        if history.len() > 1000 {
            history.pop_front();
        }
        
        let mut stats = self.device_stats.write().await;
        stats.entry(device_id.to_string())
            .or_insert_with(DeviceStats::default)
            .hashrate = hashrate;
    }
    
    pub async fn total_hashrate(&self) -> f64 {
        let stats = self.device_stats.read().await;
        stats.values().map(|s| s.hashrate).sum()
    }
    
    pub async fn pending_rewards(&self) -> f64 {
        // Placeholder
        0.0
    }
    
    pub async fn hashrate_history(&self) -> Vec<f64> {
        self.hashrate_history.read().await.iter().copied().collect()
    }
}
