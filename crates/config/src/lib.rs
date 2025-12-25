use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::fs;

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
    pub use_tls: bool,
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
                fallback: Some("stratum+tcp://backup.jxminer.com:3333".to_string()),
                wallet_address: "test_wallet".to_string(),
                worker_name: "worker1".to_string(),
                use_tls: false,
            },
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("jxpoolminer");
    path.push("config.toml");
    path
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    
    if config_path.exists() {
        let contents = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;
        Ok(config)
    } else {
        let config = Config::default();
        let _ = save_config(&config);
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create config directory")?;
    }
    
    let contents = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;
    fs::write(&config_path, contents)
        .context("Failed to write config file")?;
    
    Ok(())
}
