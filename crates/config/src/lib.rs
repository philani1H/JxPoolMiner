use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub mining: MiningConfig,
    pub pool: PoolConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    pub auto_detect_devices: bool,
    pub auto_assign_algorithms: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub primary: String,
    pub fallback: Option<String>,
    pub wallet_address: String,
    pub worker_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig {
                theme: "dark".to_string(),
                language: "en".to_string(),
            },
            mining: MiningConfig {
                auto_detect_devices: true,
                auto_assign_algorithms: true,
            },
            pool: PoolConfig {
                primary: "stratum+tcp://pool.jxminer.com:3333".to_string(),
                fallback: None,
                wallet_address: "test_wallet".to_string(),
                worker_name: "worker1".to_string(),
            },
        }
    }
}

pub fn load_config() -> Result<Config> {
    // Try to load from file, otherwise use default
    Ok(Config::default())
}

pub fn save_config(config: &Config) -> Result<()> {
    // Save config to file
    Ok(())
}
