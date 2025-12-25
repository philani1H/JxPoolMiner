pub mod client;
pub mod stratum;

pub use client::Client;

use jxpoolminer_core::{Share, MiningJob};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub primary: String,
    pub fallback: Option<String>,
    pub wallet_address: String,
    pub worker_name: String,
    pub use_tls: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            primary: "stratum+tcp://pool.jxminer.com:3333".to_string(),
            fallback: None,
            wallet_address: "test_wallet".to_string(),
            worker_name: "worker1".to_string(),
            use_tls: false,
        }
    }
}
