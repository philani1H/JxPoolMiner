pub mod gxhash;

use jxpoolminer_core::{Device, MiningJob, Share, Algorithm};
use anyhow::Result;

pub async fn mine(device: &Device, job: MiningJob) -> Result<Share> {
    match job.algorithm {
        Algorithm::GXHash => gxhash::mine(device, job).await,
        _ => Err(anyhow::anyhow!("Algorithm not implemented")),
    }
}
