use jxpoolminer_core::{Device, MiningJob, Share};
use anyhow::Result;
use sha2::{Sha256, Digest};
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
                for _ in 0..10000 {
                    let mut hasher = Sha256::new();
                    hasher.update(&job.header);
                    hasher.update(&nonce.to_le_bytes());
                    let hash = hasher.finalize();
                    
                    let hash_bytes: [u8; 32] = hash.into();
                    
                    if check_target(&hash_bytes, &target) {
                        return Some(Share {
                            job_id: job.id.clone(),
                            nonce,
                            hash: hash_bytes.to_vec(),
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

fn check_target(hash: &[u8; 32], target: &[u8]) -> bool {
    if target.is_empty() {
        return hash[0] == 0 && hash[1] == 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_target() {
        let hash = [0u8; 32];
        let target = vec![0xFF; 32];
        assert!(check_target(&hash, &target));
        
        let hash = [0xFF; 32];
        let target = vec![0x00; 32];
        assert!(!check_target(&hash, &target));
    }
}
