pub mod gxhash;
pub mod sha256;
pub mod ethash;

use jxpoolminer_core::{Device, MiningJob, Share, Algorithm};
use anyhow::Result;
use tokio::sync::mpsc;

pub async fn mine(device: &Device, job: MiningJob, cancel_rx: &mut mpsc::Receiver<()>) -> Result<Share> {
    match job.algorithm {
        Algorithm::SHA256 => sha256::mine(device, job, cancel_rx).await,
        Algorithm::Ethash => ethash::mine(device, job, cancel_rx).await,
        Algorithm::GXHash => gxhash::mine(device, job, cancel_rx).await,
    }
}
