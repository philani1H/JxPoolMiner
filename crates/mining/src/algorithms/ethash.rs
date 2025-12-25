use jxpoolminer_core::{Device, MiningJob, Share};
use anyhow::Result;
use sha3::{Keccak256, Digest};
use tokio::sync::mpsc;

pub async fn mine(
    device: &Device,
    job: MiningJob,
    cancel_rx: &mut mpsc::Receiver<()>,
) -> Result<Share> {
    let mut nonce = 0u64;
    let target = job.target.clone();
    
    loop {
        tokio::select! {
            _ = cancel_rx.recv() => {
                tracing::info!("Mining cancelled for device: {}", device.id);
                anyhow::bail!("Mining cancelled");
            }
            result = async {
                for _ in 0..5000 {
                    let hash = ethash_hash(&job.header, nonce);
                    
                    if check_target(&hash, &target) {
                        return Some(Share {
                            job_id: job.id.clone(),
                            nonce,
                            hash: hash.to_vec(),
                            device_id: device.id.clone(),
                            timestamp: chrono::Utc::now(),
                        });
                    }
                    
                    nonce = nonce.wrapping_add(1);
                }
                None::<Share>
            } => {
                if let Some(share) = result {
                    return Ok(share);
                }
            }
        }
        
        tokio::task::yield_now().await;
    }
}

fn ethash_hash(header: &[u8], nonce: u64) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(header);
    hasher.update(&nonce.to_le_bytes());
    
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    
    let mut hasher2 = Keccak256::new();
    hasher2.update(&hash);
    let result2 = hasher2.finalize();
    hash.copy_from_slice(&result2);
    
    hash
}

fn check_target(hash: &[u8; 32], target: &[u8]) -> bool {
    if target.is_empty() {
        return hash[0] == 0 && hash[1] == 0 && hash[2] == 0;
    }
    
    for i in 0..hash.len().min(target.len()) {
        if hash[i] > target[i] {
            return false;
        } else if hash[i] < target[i] {
            return true;
        }
    }
    true
}
