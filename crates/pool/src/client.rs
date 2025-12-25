use crate::{PoolConfig, stratum::StratumClient};
use jxpoolminer_core::{Share, MiningJob, Algorithm};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;

pub struct Client {
    config: PoolConfig,
    stratum: Arc<StratumClient>,
    connected: Arc<RwLock<bool>>,
    current_job: Arc<RwLock<Option<MiningJob>>>,
}

impl Client {
    pub async fn connect(config: &PoolConfig) -> Result<Self> {
        tracing::info!("Connecting to pool: {}", config.primary);
        
        let stratum = Arc::new(StratumClient::new());
        stratum.connect(&config.primary).await?;
        
        let worker = format!("{}:{}", config.wallet_address, config.worker_name);
        stratum.subscribe("JxPoolMiner/1.0.0").await?;
        stratum.authorize(&worker, "x").await?;
        
        let client = Self {
            config: config.clone(),
            stratum: stratum.clone(),
            connected: Arc::new(RwLock::new(true)),
            current_job: Arc::new(RwLock::new(None)),
        };
        
        client.start_job_listener().await;
        
        Ok(client)
    }
    
    async fn start_job_listener(&self) {
        let stratum = self.stratum.clone();
        let current_job = self.current_job.clone();
        
        tokio::spawn(async move {
            loop {
                if let Some(response) = stratum.receive().await {
                    if let Some(method) = response.method {
                        if method == "mining.notify" {
                            if let Some(params) = response.params {
                                if let Some(job) = Self::parse_job(params) {
                                    *current_job.write().await = Some(job);
                                    tracing::info!("Received new mining job");
                                }
                            }
                        }
                    }
                }
            }
        });
    }
    
    fn parse_job(params: Vec<Value>) -> Option<MiningJob> {
        if params.len() < 2 {
            return None;
        }
        
        let job_id = params[0].as_str()?.to_string();
        
        Some(MiningJob::new(
            job_id,
            Algorithm::SHA256,
            1.0,
        ))
    }
    
    pub async fn submit_share(&self, share: Share) -> Result<bool> {
        tracing::info!("Submitting share: nonce={}", share.nonce);
        
        let nonce_hex = format!("{:08x}", share.nonce);
        let result_hex = hex::encode(&share.hash);
        
        self.stratum.submit(
            &self.config.worker_name,
            &share.job_id,
            &nonce_hex,
            &result_hex,
        ).await?;
        
        Ok(true)
    }
    
    pub async fn receive_job(&self) -> Result<MiningJob> {
        loop {
            let job_guard = self.current_job.read().await;
            if let Some(job) = job_guard.as_ref() {
                return Ok(job.clone());
            }
            drop(job_guard);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
    
    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }
}
