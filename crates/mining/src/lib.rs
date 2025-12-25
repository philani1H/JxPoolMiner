pub mod engine;
pub mod algorithms;

pub use engine::Engine;

use jxpoolminer_core::{Device, MiningJob, Share};
use anyhow::Result;

pub async fn start_mining(device: &Device, job: MiningJob) -> Result<Share> {
    algorithms::mine(device, job).await
}
