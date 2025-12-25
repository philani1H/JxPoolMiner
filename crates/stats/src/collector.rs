use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeviceStats {
    pub hashrate: f64,
    pub shares_accepted: u64,
    pub shares_rejected: u64,
    pub temperature: f32,
    pub power_usage: f32,
    pub uptime: u64,
}

impl Default for DeviceStats {
    fn default() -> Self {
        Self {
            hashrate: 0.0,
            shares_accepted: 0,
            shares_rejected: 0,
            temperature: 0.0,
            power_usage: 0.0,
            uptime: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HashratePoint {
    pub timestamp: DateTime<Utc>,
    pub hashrate: f64,
}

pub struct Collector {
    device_hashrate_history: Arc<RwLock<HashMap<String, VecDeque<HashratePoint>>>>,
    device_stats: Arc<RwLock<HashMap<String, DeviceStats>>>,
    global_stats: Arc<RwLock<GlobalStats>>,
}

#[derive(Debug, Clone)]
pub struct GlobalStats {
    pub total_shares: u64,
    pub accepted_shares: u64,
    pub rejected_shares: u64,
    pub start_time: DateTime<Utc>,
    pub pending_rewards: f64,
}

impl Default for GlobalStats {
    fn default() -> Self {
        Self {
            total_shares: 0,
            accepted_shares: 0,
            rejected_shares: 0,
            start_time: Utc::now(),
            pending_rewards: 0.0,
        }
    }
}

impl Collector {
    pub fn new() -> Self {
        Self {
            device_hashrate_history: Arc::new(RwLock::new(HashMap::new())),
            device_stats: Arc::new(RwLock::new(HashMap::new())),
            global_stats: Arc::new(RwLock::new(GlobalStats::default())),
        }
    }
    
    pub async fn record_hashrate(&self, device_id: &str, hashrate: f64) {
        let point = HashratePoint {
            timestamp: Utc::now(),
            hashrate,
        };
        
        let mut history = self.device_hashrate_history.write().await;
        let device_history = history.entry(device_id.to_string())
            .or_insert_with(VecDeque::new);
        
        device_history.push_back(point);
        
        if device_history.len() > 1440 {
            device_history.pop_front();
        }
        
        let mut stats = self.device_stats.write().await;
        stats.entry(device_id.to_string())
            .or_insert_with(DeviceStats::default)
            .hashrate = hashrate;
    }
    
    pub async fn record_share(&self, device_id: &str, accepted: bool) {
        let mut stats = self.device_stats.write().await;
        let device_stats = stats.entry(device_id.to_string())
            .or_insert_with(DeviceStats::default);
        
        if accepted {
            device_stats.shares_accepted += 1;
        } else {
            device_stats.shares_rejected += 1;
        }
        
        let mut global = self.global_stats.write().await;
        global.total_shares += 1;
        if accepted {
            global.accepted_shares += 1;
        } else {
            global.rejected_shares += 1;
        }
    }
    
    pub async fn update_temperature(&self, device_id: &str, temperature: f32) {
        let mut stats = self.device_stats.write().await;
        stats.entry(device_id.to_string())
            .or_insert_with(DeviceStats::default)
            .temperature = temperature;
    }
    
    pub async fn update_power(&self, device_id: &str, power: f32) {
        let mut stats = self.device_stats.write().await;
        stats.entry(device_id.to_string())
            .or_insert_with(DeviceStats::default)
            .power_usage = power;
    }
    
    pub async fn total_hashrate(&self) -> f64 {
        let stats = self.device_stats.read().await;
        stats.values().map(|s| s.hashrate).sum()
    }
    
    pub async fn device_stats(&self, device_id: &str) -> Option<DeviceStats> {
        let stats = self.device_stats.read().await;
        stats.get(device_id).cloned()
    }
    
    pub async fn all_device_stats(&self) -> HashMap<String, DeviceStats> {
        self.device_stats.read().await.clone()
    }
    
    pub async fn global_stats(&self) -> GlobalStats {
        self.global_stats.read().await.clone()
    }
    
    pub async fn pending_rewards(&self) -> f64 {
        self.global_stats.read().await.pending_rewards
    }
    
    pub async fn hashrate_history(&self, device_id: &str) -> Vec<HashratePoint> {
        let history = self.device_hashrate_history.read().await;
        history.get(device_id)
            .map(|h| h.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    pub async fn acceptance_rate(&self) -> f64 {
        let global = self.global_stats.read().await;
        if global.total_shares > 0 {
            (global.accepted_shares as f64 / global.total_shares as f64) * 100.0
        } else {
            0.0
        }
    }
}
